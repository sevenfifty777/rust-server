#![allow(dead_code)]
#![recursion_limit = "256"]

mod authentication;
mod config;
mod fps;
#[cfg(feature = "hot-reload")]
mod hot_reload;
mod integrity;
pub mod rpc;
mod server;
mod shutdown;
mod srs;
mod stats;
mod stream;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Instant;

use config::Config;
use mlua::prelude::*;
use mlua::{Function, LuaSerdeExt, Value};
use once_cell::sync::Lazy;
use server::{Server, TtsOptions};
use stubs::mission::v0::StreamEventsResponse;
use thiserror::Error;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static SERVER: Lazy<RwLock<Option<Server>>> = Lazy::new(|| RwLock::new(None));
/// Epoch of the monotonic clock exposed to Lua as `grpc.monotonicMs()`.
static MONOTONIC_EPOCH: Lazy<Instant> = Lazy::new(Instant::now);

/// Initialise file logging once per process. Returns a human-readable error (and leaves the
/// process un-initialised so a later `start` can retry) instead of panicking, since a panic here
/// would unwind across the Lua FFI boundary.
pub fn init(config: &Config) -> Result<(), String> {
    if INITIALIZED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .unwrap_or(true)
    {
        return Ok(());
    }

    match init_logging(config) {
        Ok(()) => Ok(()),
        Err(err) => {
            INITIALIZED.store(false, Ordering::Release);
            Err(err)
        }
    }
}

fn init_logging(config: &Config) -> Result<(), String> {
    use log::LevelFilter;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let mut log_file = PathBuf::from(&config.write_dir);
    log_file.push("Logs/gRPC.log");

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} {l:<7} {t}: {m}{n}",
        )))
        .append(false)
        .build(&log_file)
        .map_err(|err| format!("failed to open log file {}: {err}", log_file.display()))?;

    let level = if config.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let log_config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(requests)))
        .logger(Logger::builder().build("dcs_grpc", level))
        .logger(Logger::builder().build("dcs_grpc_srs", level))
        .logger(Logger::builder().build("dcs_grpc_tts", level))
        .logger(Logger::builder().build("tokio", level))
        .logger(Logger::builder().build("tonic", level))
        .build(Root::builder().appender("file").build(LevelFilter::Off))
        .map_err(|err| format!("invalid log configuration: {err}"))?;

    log4rs::init_config(log_config).map_err(|err| format!("failed to install logger: {err}"))?;
    Ok(())
}

/// Convert a poisoned `SERVER` lock into a logged Lua error instead of panicking.
fn server_lock_error(op: &str, err: &dyn std::fmt::Display) -> mlua::Error {
    let message = format!("dcs_grpc: server state lock poisoned while acquiring {op} lock: {err}");
    log::error!("{message}");
    mlua::Error::RuntimeError(message)
}

fn server_read() -> LuaResult<RwLockReadGuard<'static, Option<Server>>> {
    SERVER.read().map_err(|err| server_lock_error("read", &err))
}

fn server_write() -> LuaResult<RwLockWriteGuard<'static, Option<Server>>> {
    SERVER
        .write()
        .map_err(|err| server_lock_error("write", &err))
}

#[unsafe(no_mangle)]
pub fn start(_: &Lua, config: Config) -> LuaResult<(bool, Option<String>)> {
    {
        if server_read()?.is_some() {
            return Ok((true, None));
        }
    }

    if let Err(err) = init(&config) {
        // Logging is not available yet, so the message is handed back to Lua where `grpc.lua`
        // raises it through `assert(grpc.start(...))` into the DCS log.
        return Ok((false, Some(format!("dcs_grpc failed to initialise: {err}"))));
    }

    log::debug!("Config: {:#?}", config);

    if !config.integrity_check_disabled {
        if env!("CARGO_PKG_VERSION") != config.version {
            return Ok((false, Some("dcs_grpc.dll version does not match version of DCS-gRPC Lua files; please check your installation!".to_string())));
        }

        if let Err(err) = integrity::check(&config) {
            return Ok((false, Some(err.to_string())));
        }
        log::info!("integrity check successful");
    }

    log::info!("Starting ...");

    let mut server =
        Server::new(&config).map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?;
    server.run_in_background();
    *server_write()? = Some(server);

    log::info!("Started");

    Ok((true, None))
}

#[unsafe(no_mangle)]
pub fn stop(_: &Lua, _: ()) -> LuaResult<()> {
    log::info!("Stopping ...");

    if let Some(server) = server_write()?.take() {
        server.stop_blocking();
    }

    log::info!("Stopped");

    Ok(())
}

#[unsafe(no_mangle)]
pub fn next(lua: &Lua, (env, callback): (i32, Function)) -> LuaResult<bool> {
    let start = Instant::now();

    if let Some(server) = &*server_read()? {
        let _guard = server.stats().track_block_time(start);

        let (next, discarded_cancelled) = match env {
            1 => {
                let ipc = server.ipc_mission();
                (ipc.try_next(), ipc.take_discarded_cancelled())
            }
            2 => {
                let ipc = server.ipc_hook();
                (ipc.try_next(), ipc.take_discarded_cancelled())
            }
            _ => return Ok(false),
        };
        server
            .stats()
            .track_cancelled_ipc_requests(discarded_cancelled);

        if let Some(mut next) = next {
            server.stats().track_call();

            let request_id = next.id();
            let queue_wait = next.queue_wait();
            let queue_depth_at_enqueue = next.queue_depth_at_enqueue();
            let queue_depth_at_dequeue = next.queue_depth_at_dequeue();
            let method = next.method().to_string();
            #[allow(clippy::arc_with_non_send_sync)]
            let params = next
                .params(lua)
                .map_err(|err| mlua::Error::ExternalError(Arc::new(Error::SerializeParams(err))))?;

            if let Some(params) = &params {
                log::debug!(
                    "Sending request `{}`: {}",
                    method,
                    pretty_print_value(params.clone(), 0)?
                );
            } else {
                log::debug!("Sending request `{}`", method,);
            }

            // Per-request IPC metadata handed to the Lua request handler as a third argument, so
            // individual methods (e.g. `getRecoverySnapshot`) can report queue diagnostics.
            let meta = lua.create_table()?;
            meta.set("requestId", request_id)?;
            meta.set("queueWaitMs", queue_wait.as_secs_f64() * 1_000.0)?;
            meta.set("queueDepthAtEnqueue", queue_depth_at_enqueue)?;
            meta.set("queueDepthAtDequeue", queue_depth_at_dequeue)?;

            let execution_started_at = Instant::now();
            let result: LuaTable = callback.call((method.as_str(), params, meta))?;
            let error: Option<LuaTable> = result.get("error")?;

            if let Some(error) = error {
                let message: String = error.get("message")?;
                let kind: Option<String> = error.get("type")?;

                next.error(message, kind);
                server
                    .stats()
                    .track_ipc_request(crate::stats::IpcRequestMeasurement {
                        request_id,
                        method: &method,
                        queue_wait,
                        execution_time: execution_started_at.elapsed(),
                        queue_depth_at_enqueue,
                        queue_depth_at_dequeue,
                        outcome: "script_error",
                    });
                return Ok(true);
            }

            let res: Value = result.get("result")?;
            log::debug!("Receiving: {}", pretty_print_value(res.clone(), 0)?);

            next.success(lua, &res).map_err(|err| {
                #[allow(clippy::arc_with_non_send_sync)]
                mlua::Error::ExternalError(Arc::new(Error::DeserializeResult {
                    err,
                    method: method.clone(),
                    result: pretty_print_value(res, 0)
                        .unwrap_or_else(|err| format!("failed to pretty print result: {err}")),
                }))
            })?;
            server
                .stats()
                .track_ipc_request(crate::stats::IpcRequestMeasurement {
                    request_id,
                    method: &method,
                    queue_wait,
                    execution_time: execution_started_at.elapsed(),
                    queue_depth_at_enqueue,
                    queue_depth_at_dequeue,
                    outcome: "success",
                });

            return Ok(true);
        }
    }

    Ok(false)
}

#[unsafe(no_mangle)]
pub fn tts(_lua: &Lua, (ssml, freq, opts): (String, u64, Option<TtsOptions>)) -> LuaResult<()> {
    let start = Instant::now();
    if let Some(server) = &*server_read()? {
        let _guard = server.stats().track_block_time(start);
        server.tts(ssml, freq, opts);
    }

    Ok(())
}

#[unsafe(no_mangle)]
pub fn event(lua: &Lua, event: Value) -> LuaResult<()> {
    let start = Instant::now();

    let event: StreamEventsResponse = match lua.from_value(event) {
        Ok(event) => event,
        Err(err) => {
            log::error!("failed to deserialize event: {}", err);
            // In certain cases DCS crashes when we return an error back to Lua here (see
            // https://github.com/DCS-gRPC/rust-server/issues/19), which we are working around
            // by intercepting and logging the error instead.
            return Ok(());
        }
    };

    // A poisoned lock is already logged by `server_read`; swallow it here for the same
    // crash-avoidance reason as the deserialization error above.
    let Ok(server) = server_read() else {
        return Ok(());
    };
    if let Some(server) = &*server {
        let _guard = server.stats().track_block_time(start);
        server.stats().track_event();

        log::debug!("Received event: {:#?}", event);
        server.block_on(server.ipc_mission().event(event));
    }

    Ok(())
}

// This method is called on each simulation frame, so make sure to do as few as possible (avoid
// even getting a lock on [SERVER]).
#[unsafe(no_mangle)]
pub fn simulation_frame(_lua: &Lua, time: f64) -> LuaResult<()> {
    crate::fps::frame(time);

    Ok(())
}

/// Milliseconds elapsed on a monotonic clock since the first call into this function. Exposed to
/// Lua as `grpc.monotonicMs()` so callbacks can measure their own execution time even when the
/// mission scripting environment has `os` sanitized.
#[unsafe(no_mangle)]
pub fn monotonic_ms(_: &Lua, _: ()) -> LuaResult<f64> {
    Ok(MONOTONIC_EPOCH.elapsed().as_secs_f64() * 1_000.0)
}

#[unsafe(no_mangle)]
pub fn log_error(_: &Lua, err: String) -> LuaResult<()> {
    log::error!("{}", err);
    Ok(())
}

#[unsafe(no_mangle)]
pub fn log_warning(_: &Lua, err: String) -> LuaResult<()> {
    log::warn!("{}", err);
    Ok(())
}

#[unsafe(no_mangle)]
pub fn log_info(_: &Lua, err: String) -> LuaResult<()> {
    log::info!("{}", err);
    Ok(())
}

#[unsafe(no_mangle)]
pub fn log_debug(_: &Lua, err: String) -> LuaResult<()> {
    log::debug!("{}", err);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to deserialize params: {0}")]
    DeserializeParams(#[source] mlua::Error),
    #[error("Failed to deserialize result for method {method}: {err}\n{result}")]
    DeserializeResult {
        #[source]
        err: mlua::Error,
        method: String,
        result: String,
    },
    #[error("Failed to serialize params: {0}")]
    SerializeParams(#[source] mlua::Error),
}

#[cfg(feature = "hot-reload")]
#[mlua::lua_module]
pub fn dcs_grpc_hot_reload(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", lua.create_function(hot_reload::start)?)?;
    exports.set("stop", lua.create_function(hot_reload::stop)?)?;
    exports.set("next", lua.create_function(hot_reload::next)?)?;
    exports.set("event", lua.create_function(hot_reload::event)?)?;
    exports.set(
        "simulationFrame",
        lua.create_function(hot_reload::simulation_frame)?,
    )?;
    exports.set("tts", lua.create_function(hot_reload::tts)?)?;
    exports.set(
        "monotonicMs",
        lua.create_function(hot_reload::monotonic_ms)?,
    )?;
    exports.set("logError", lua.create_function(hot_reload::log_error)?)?;
    exports.set("logWarning", lua.create_function(hot_reload::log_warning)?)?;
    exports.set("logInfo", lua.create_function(hot_reload::log_info)?)?;
    exports.set("logDebug", lua.create_function(hot_reload::log_debug)?)?;
    Ok(exports)
}

#[cfg(not(feature = "hot-reload"))]
#[mlua::lua_module]
pub fn dcs_grpc(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", lua.create_function(start)?)?;
    exports.set("stop", lua.create_function(stop)?)?;
    exports.set("next", lua.create_function(next)?)?;
    exports.set("event", lua.create_function(event)?)?;
    exports.set("simulationFrame", lua.create_function(simulation_frame)?)?;
    exports.set("tts", lua.create_function(tts)?)?;
    exports.set("monotonicMs", lua.create_function(monotonic_ms)?)?;
    exports.set("logError", lua.create_function(log_error)?)?;
    exports.set("logWarning", lua.create_function(log_warning)?)?;
    exports.set("logInfo", lua.create_function(log_info)?)?;
    exports.set("logDebug", lua.create_function(log_debug)?)?;
    Ok(exports)
}

fn pretty_print_value(val: Value, indent: usize) -> LuaResult<String> {
    use std::fmt::Write;

    Ok(match val {
        Value::Nil => "nil".to_string(),
        Value::Boolean(v) => v.to_string(),
        Value::LightUserData(_) => String::new(),
        Value::Integer(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => format!("\"{}\"", v.to_str()?),
        Value::Table(t) => {
            let mut s = "{\n".to_string();
            for pair in t.pairs::<Value, Value>() {
                let (key, value) = pair?;
                let _ = writeln!(
                    s,
                    "{}{} = {},",
                    "  ".repeat(indent + 1),
                    pretty_print_value(key, indent + 1)?,
                    pretty_print_value(value, indent + 1)?
                );
            }
            let _ = write!(s, "{}}}", "  ".repeat(indent));
            s
        }
        Value::Function(_) => "[function]".to_string(),
        Value::Thread(_) => String::new(),
        Value::UserData(_) => String::new(),
        Value::Error(err) => err.to_string(),
        Value::Other(_) => "(unknown type)".to_string(),
    })
}
