use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

fn main() {
    write_version_to_lua();
    embed_lua_file_hashes();
}

/// Write the current version into `lua/DCS-gRPC/version.lua` to be picked up by the Lua side of the
/// server.
///
/// The file is generated and ignored by git. The version is read from the environment at run time
/// (not baked in with `env!` at compile time) so that a build script binary compiled for one
/// version can never write a stale version after a branch switch. The file is only rewritten when
/// its content changes, so unrelated builds leave its timestamp alone.
fn write_version_to_lua() {
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

    let version = std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION is set by cargo");
    let content =
        format!("-- this file is auto-generated on `cargo build`\nGRPC.version = \"{version}\"\n");

    let path = PathBuf::from("./lua/DCS-gRPC/version.lua");
    if std::fs::read_to_string(&path).ok().as_deref() == Some(content.as_str()) {
        return;
    }

    let mut out = File::create(path).unwrap();
    out.write_all(content.as_bytes()).unwrap();
}

/// Embed the hash of each Lua file into the binary to allow a runtime integrity check.
fn embed_lua_file_hashes() {
    println!("cargo:rerun-if-changed=lua/DCS-gRPC");
    println!("cargo:rerun-if-changed=lua/Hooks");

    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("lua_files.rs");
    let mut out = File::create(path).unwrap();

    for (ident, base_path) in [("DCS_GRPC", "./lua/DCS-gRPC"), ("HOOKS", "./lua/Hooks")] {
        writeln!(out, "#[allow(clippy::needless_raw_string_hashes)]").unwrap();
        writeln!(out, "const {ident}: &[(&str, u64)] = &[").unwrap();

        for entry in WalkDir::new(base_path) {
            let entry = entry.unwrap();
            if !entry.metadata().unwrap().is_file() {
                continue;
            }

            let path = entry
                .path()
                .strip_prefix(base_path)
                .unwrap()
                .to_str()
                .expect("non-utf8 path");
            let hash = file_hash(entry.path());
            writeln!(out, r##"    (r#"{path}"#, {hash}),"##).unwrap();
            eprintln!("{}", entry.path().display());
        }

        writeln!(out, "];").unwrap();
    }
}

fn file_hash(path: &Path) -> u64 {
    // Not a cryptographic hasher, but good enough for our use-case.
    let mut hasher = DefaultHasher::new();
    let mut buffer = [0; 1024];
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }

    hasher.finish()
}
