use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use dcs_module_ipc::IPC;
use futures_util::Stream;
use stubs::mission::v0::StreamEventsResponse;
use tokio::sync::RwLock;
use tonic::{Request, Status};

pub use self::srs::Srs;
use crate::shutdown::ShutdownHandle;
use crate::stats::Stats;

mod atmosphere;
mod coalition;
mod controller;
mod custom;
mod group;
mod hook;
mod land;
mod metadata;
mod mission;
mod net;
mod recovery;
mod spot;
mod srs;
mod timer;
mod trigger;
mod unit;
mod warehouse;
mod weapon;
mod world;

#[derive(Clone)]
pub struct MissionRpc {
    ipc: IPC<StreamEventsResponse>,
    stats: Stats,
    eval_enabled: bool,
    shutdown_signal: ShutdownHandle,
    cache: Arc<RwLock<Cache>>,
    recovery_read_limiter: Option<RecoveryReadLimiter>,
}

#[derive(Clone)]
struct RecoveryReadLimiter {
    rate: f64,
    buckets: Arc<Mutex<HashMap<String, ReadBucket>>>,
}

struct ReadBucket {
    tokens: f64,
    updated_at: Instant,
}

impl RecoveryReadLimiter {
    fn new(rate: f64) -> Self {
        Self {
            rate,
            buckets: Default::default(),
        }
    }

    fn try_acquire(&self, owner: &str) -> bool {
        let now = Instant::now();
        let mut buckets = self
            .buckets
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(bucket) = buckets.get_mut(owner) {
            bucket.tokens = (bucket.tokens
                + now.duration_since(bucket.updated_at).as_secs_f64() * self.rate)
                .min(self.rate * 2.0);
            bucket.updated_at = now;
            if bucket.tokens < 1.0 {
                return false;
            }
            bucket.tokens -= 1.0;
            return true;
        }
        buckets.insert(
            owner.to_owned(),
            ReadBucket {
                tokens: self.rate * 2.0 - 1.0,
                updated_at: now,
            },
        );
        true
    }
}

#[derive(Default)]
struct Cache {
    scenario_start_time: Option<time::OffsetDateTime>,
}

#[derive(Clone)]
pub struct HookRpc {
    ipc: IPC<()>,
    stats: Stats,
    eval_enabled: bool,
    shutdown_signal: ShutdownHandle,
}

impl MissionRpc {
    pub fn new(
        ipc: IPC<StreamEventsResponse>,
        stats: Stats,
        shutdown_signal: ShutdownHandle,
        recovery_reads_per_second: Option<f64>,
    ) -> Self {
        MissionRpc {
            ipc,
            stats,
            eval_enabled: false,
            shutdown_signal,
            cache: Default::default(),
            recovery_read_limiter: recovery_reads_per_second.map(RecoveryReadLimiter::new),
        }
    }

    #[allow(clippy::result_large_err)] // Preserve tonic's established Status return type.
    pub(crate) fn check_recovery_read_quota(&self, owner: &str) -> Result<(), Status> {
        if self
            .recovery_read_limiter
            .as_ref()
            .is_some_and(|limiter| !limiter.try_acquire(owner))
        {
            return Err(Status::resource_exhausted(
                "recovery telemetry read quota exceeded",
            ));
        }
        Ok(())
    }

    pub fn enable_eval(&mut self) {
        self.eval_enabled = true;
    }

    #[allow(clippy::result_large_err)] // Preserve the established tonic Status return type.
    pub async fn request<I, O>(&self, method: &str, request: Request<I>) -> Result<O, Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
        for<'de> O: serde::Deserialize<'de> + Send + Sync + std::fmt::Debug + 'static,
    {
        let _guard = self.stats.track_queue_size();
        let started_at = Instant::now();
        let result = self
            .ipc
            .request(method, Some(request.into_inner()))
            .await
            .map_err(to_status);
        log::debug!(
            "IPC RPC completed: method={} total_ms={:.3} outcome={}",
            method,
            started_at.elapsed().as_secs_f64() * 1_000.0,
            if result.is_ok() { "success" } else { "error" }
        );
        result
    }

    pub async fn events(&self) -> impl Stream<Item = StreamEventsResponse> + use<> {
        let ipc = self.ipc.clone();
        ipc.events().await
    }

    pub async fn event(&self, event: StreamEventsResponse) {
        log::debug!("Received event: {:#?}", event);
        self.ipc.event(event).await
    }
}

impl HookRpc {
    pub fn new(ipc: IPC<()>, stats: Stats, shutdown_signal: ShutdownHandle) -> Self {
        HookRpc {
            ipc,
            stats,
            eval_enabled: false,
            shutdown_signal,
        }
    }

    pub fn enable_eval(&mut self) {
        self.eval_enabled = true;
    }

    #[allow(clippy::result_large_err)] // Preserve the established tonic Status return type.
    pub async fn request<I, O>(&self, method: &str, request: Request<I>) -> Result<O, Status>
    where
        I: serde::Serialize + Send + Sync + 'static,
        for<'de> O: serde::Deserialize<'de> + Send + Sync + std::fmt::Debug + 'static,
    {
        let _guard = self.stats.track_queue_size();
        let started_at = Instant::now();
        let result = self
            .ipc
            .request(method, Some(request.into_inner()))
            .await
            .map_err(to_status);
        log::debug!(
            "Hook IPC RPC completed: method={} total_ms={:.3} outcome={}",
            method,
            started_at.elapsed().as_secs_f64() * 1_000.0,
            if result.is_ok() { "success" } else { "error" }
        );
        result
    }
}

fn to_status(err: dcs_module_ipc::Error) -> Status {
    use dcs_module_ipc::Error;
    match err {
        Error::Script { kind, message } => match kind.as_deref() {
            Some("INVALID_ARGUMENT") => Status::invalid_argument(message),
            Some("NOT_FOUND") => Status::not_found(message),
            Some("ALREADY_EXISTS") => Status::already_exists(message),
            Some("PERMISSION_DENIED") => Status::permission_denied(message),
            Some("RESOURCE_EXHAUSTED") => Status::resource_exhausted(message),
            Some("UNAUTHENTICATED") => Status::unauthenticated(message),
            Some("UNIMPLEMENTED") => Status::unimplemented(message),
            // `GRPC.errorInternal`; any unknown kind also maps to INTERNAL (below).
            Some("INTERNAL") => Status::internal(message),
            _ => Status::internal(message),
        },
        queue_full @ Error::QueueFull { .. } => Status::resource_exhausted(queue_full.to_string()),
        closed @ Error::ResponseChannelClosed => Status::cancelled(closed.to_string()),
        err => Status::internal(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::RecoveryReadLimiter;

    #[test]
    fn recovery_read_limiter_is_owner_scoped_and_bounds_the_initial_burst() {
        let limiter = RecoveryReadLimiter::new(1.0);
        assert!(limiter.try_acquire("client-a"));
        assert!(limiter.try_acquire("client-a"));
        assert!(!limiter.try_acquire("client-a"));
        assert!(limiter.try_acquire("client-b"));
    }
}
