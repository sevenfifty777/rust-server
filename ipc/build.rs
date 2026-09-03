use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // On Windows the workspace links `mlua` in `module` mode against the `lua.lib` import library,
    // so every executable that references an `mlua` symbol imports `lua.dll` at process start.
    // The unit tests of this crate never actually call into Lua, but the test binary would still
    // fail to launch (STATUS_DLL_NOT_FOUND) without a DCS installation on PATH. Delay-loading the
    // DLL defers that lookup to the first real Lua call, which the tests never make.
    //
    // `rustc-link-arg` only applies to this package's own linked targets (here: the lib unit-test
    // executable); it is not propagated to dependents such as the `dcs_grpc` cdylib.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if target_os == "windows" && target_env == "msvc" {
        println!("cargo:rustc-link-arg=/DELAYLOAD:lua.dll");
        println!("cargo:rustc-link-arg=delayimp.lib");
    }
}
