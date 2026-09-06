# Release v0.9.2 - step by step

Runbook for publishing DCS-gRPC `v0.9.2` from the `justice-version-analysis` branch of
`sevenfifty777/rust-server`. Every command is PowerShell 7, run from the repository root.

## 0. What is already prepared on the branch (2026-09-06)

| Item | State |
| --- | --- |
| Workspace version (`Cargo.toml`) | `0.9.2` |
| Lua bridge version (`lua/DCS-gRPC/version.lua`) | `0.9.2` |
| `dcs-module-ipc` dependency | git pin `https://github.com/sevenfifty777/dcs-module-ipc` @ `55f0bf53764380845f507e0036dde57de5ce8580` (vendored `ipc/` crate removed, content was byte-identical) |
| `protoc-bundled` build dependency | git pin `https://github.com/sevenfifty777/protoc-bundled.git` @ `d8e8f513cfe007a6243a7eef571677fd366d2cb9` |
| `Cargo.lock` | regenerated, `dcs-module-ipc 0.9.1` resolved from the git source |
| `CHANGELOG.md` | `Unreleased` + former `0.10.0` + `0.9.2` entries consolidated under `[0.9.2] - 2026-09-06`, new empty `Unreleased` section |
| `README.md` | Releases link points to `sevenfifty777/rust-server/releases` |
| Local verification | `cargo fmt --check`, `cargo clippy --workspace -D warnings`, `cargo test --workspace`, `cargo +1.98.0 check --locked`, `cargo check --features hot-reload` all pass |

Not verified locally (no Lua toolchain installed): `luacheck ./lua` and the two Lua engine tests. CI runs them on the PR.

## 1. Review the pending changes

```powershell
git switch justice-version-analysis
git status --short
git diff --check
git diff -- Cargo.toml Cargo.lock CHANGELOG.md README.md AGENTS.md
```

Expected: modified `AGENTS.md`, `CHANGELOG.md`, `Cargo.lock`, `Cargo.toml`, `README.md`, new
`docs/release_v0.9.2_steps.md`, and the whole `ipc/` directory staged as deleted.

## 2. Re-run local verification (optional, already green)

```powershell
cargo fmt -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo check --features hot-reload
cargo +1.98.0 check --locked -p dcs-grpc -p dcs-grpc-repl
```

## 3. Commit the release preparation

```powershell
git add Cargo.toml Cargo.lock CHANGELOG.md README.md AGENTS.md docs/release_v0.9.2_steps.md
git add -u ipc
git commit -m "chore: release v0.9.2"
```

## 4. Push the branch and open the pull request to `main`

```powershell
git push origin justice-version-analysis

# The clone has two remotes (origin = fork, upstream = DCS-gRPC). Tell gh which one to target once:
gh repo set-default sevenfifty777/rust-server

$prBody = @'
## Release v0.9.2

- Workspace and Lua bridge version `0.9.2`; `CHANGELOG.md` consolidated under `[0.9.2] - 2026-09-06`.
- `dcs-module-ipc` is a git dependency on `sevenfifty777/dcs-module-ipc@55f0bf5` again; the vendored `ipc/` crate is removed (content was byte-identical).
- `protoc-bundled` stays pinned to `sevenfifty777/protoc-bundled@d8e8f51`.
- README Releases link points to this fork.
- Adds `docs/release_v0.9.2_steps.md` (release runbook).

Verified locally: `cargo fmt --check`, `cargo clippy --workspace -D warnings`, `cargo test --workspace`, `cargo +1.98.0 check --locked`, `cargo check --features hot-reload`.
'@

gh pr create `
    --base main `
    --head justice-version-analysis `
    --title "Release v0.9.2" `
    --body $prBody
```

## 5. Wait for CI and merge

```powershell
gh pr checks --watch
gh pr view --web
```

All five jobs must be green: Rust (Windows), Lua, Proto, Linux, Cargo audit (advisory only).

Merge from the CLI once green (merge commit keeps the branch history reachable for the tag):

```powershell
gh pr merge --merge
```

Use `gh pr merge --squash` instead if you want a single commit on `main`.

## 6. Update local `main`

```powershell
git switch main
git pull --ff-only origin main
git log --oneline -3
```

Confirm the top commit contains the release preparation (or the squashed PR commit).

## 7. Build and package the release from `main`

Prerequisites already present on this machine: Rust `1.98.0` toolchain, `protoc` (winget) on `PATH`,
`protoc-gen-doc` in `%USERPROFILE%\go\bin`.

```powershell
.\build_release.ps1
```

Expected outputs:

```text
Releases\DCS-gRPC-0.9.2\
Releases\DCS-gRPC-0.9.2.zip
```

The script runs `cargo +1.98.0 build --release --locked -p dcs-grpc -p dcs-grpc-repl`, verifies every packaged
file by SHA-256, generates `Docs\DCS-gRPC\api.html`, and re-opens the ZIP to verify its entries. If it stops on tool
discovery, pass the paths explicitly:

```powershell
.\build_release.ps1 `
    -ProtocPath "C:\Users\thierry\AppData\Local\Microsoft\WinGet\Packages\Google.Protobuf_Microsoft.Winget.Source_8wekyb3d8bbwe\bin\protoc.exe" `
    -ProtocGenDocPath "$env:USERPROFILE\go\bin\protoc-gen-doc.exe"
```

Quick sanity check of the archive:

```powershell
Get-ChildItem Releases\DCS-gRPC-0.9.2 -Recurse -File | Select-Object -ExpandProperty FullName
Get-Item Releases\DCS-gRPC-0.9.2.zip | Select-Object Name, Length
```

Optional smoke test: extract the ZIP into a DCS Saved Games directory, start a mission, and call
`MetadataService/GetVersion` (expect `0.9.2`) with `grpcurl` or `repl.exe`.

## 8. Tag `v0.9.2` on `main` and push the tag

```powershell
git switch main
git tag -a v0.9.2 -m "DCS-gRPC v0.9.2"
git push origin v0.9.2
git tag --sort=-v:refname | Select-Object -First 3
```

## 9. Create the GitHub release and upload the ZIP

Extract the `0.9.2` changelog section into a notes file, then publish:

```powershell
$changelog = Get-Content CHANGELOG.md -Raw
$section = [regex]::Match($changelog, '(?s)## \[0\.9\.2\].*?(?=\r?\n## \[)').Value
Set-Content -Path "$env:TEMP\release-notes-v0.9.2.md" -Value $section

gh release create v0.9.2 `
    "Releases\DCS-gRPC-0.9.2.zip" `
    --title "DCS-gRPC v0.9.2" `
    --notes-file "$env:TEMP\release-notes-v0.9.2.md" `
    --verify-tag
```

Verify:

```powershell
gh release view v0.9.2
gh release view v0.9.2 --json assets --jq '.assets[].name'
```

## 10. After the release

- Delete or keep the `justice-version-analysis` branch as you prefer (`git push origin --delete justice-version-analysis`).
- The LSO client repository still uses a local path dependency on `../DCS-gRPC/stubs` (see `AGENTS.md`). Repin it to
  `git = "https://github.com/sevenfifty777/rust-server", tag = "v0.9.2"` when you next touch it.
- Start the next cycle by adding entries under `## [Unreleased]` in `CHANGELOG.md`.
