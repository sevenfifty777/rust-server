# DCS-gRPC Release Process

This document describes how to bump the version, build the Windows artifacts, create the DCS directory layout, validate the package, and publish a DCS-gRPC release.

## Prerequisites

Install or provide the following tools before starting:

- Rust toolchain `1.98.0` and Cargo.
- `protoc`, either on `PATH`, supplied with `-ProtocPath`, or available from the repository's `protoc-bundled` build dependency.
- `protoc-gen-doc`, either on `PATH`, installed in `%USERPROFILE%\go\bin`, or supplied with `-ProtocGenDocPath`.
- PowerShell 5.1 or PowerShell 7.

The packaging script treats the server DLL, REPL, API documentation, sample mission, third-party tools, licences, and other documented artifacts as required. It stops before replacing an existing release if a prerequisite cannot be found.

## 1. Update the version

Update the workspace version in the root `Cargo.toml`:

```toml
[workspace.package]
version = "X.Y.Z"
```

The workspace members use `version.workspace = true`, so this updates the server, REPL, stubs, and other internal crates together.

## 2. Update the changelog

In `CHANGELOG.md`, move the current changes under a versioned heading with the release date and add a new empty `Unreleased` section:

```markdown
## [Unreleased]

## [X.Y.Z] - YYYY-MM-DD

### Added

- Describe the new functionality.
```

## 3. Build and package the release

From a PowerShell prompt at the repository root, run:

```powershell
.\build_release.ps1
```

The script runs the following locked release build:

```powershell
cargo +1.98.0 build --release --locked -p dcs-grpc -p dcs-grpc-repl
```

The build produces:

- `target/release/dcs_grpc.dll`
- `target/release/dcs_grpc.pdb` when debug symbols are available
- `target/release/dcs-grpc-repl.exe`, packaged as `repl.exe`

If automatic tool discovery is unavailable, provide explicit executable paths:

```powershell
.\build_release.ps1 `
    -ProtocPath "C:\path\to\protoc.exe" `
    -ProtocGenDocPath "C:\path\to\protoc-gen-doc.exe"
```

To test packaging without replacing files under the repository's `Releases` directory, use an isolated output directory:

```powershell
.\build_release.ps1 -ReleasesDirectory "$env:TEMP\dcs-grpc-release-check"
```

## 4. Verify the package layout

The script creates `Releases/DCS-gRPC-X.Y.Z/` with this layout:

```text
DCS-gRPC-X.Y.Z/
├── Docs/
│   └── DCS-gRPC/
│       ├── protos/
│       │   └── dcs/
│       ├── api.html
│       ├── CHANGELOG.md
│       ├── README.md
│       └── STATUS.md
├── Missions/
│   └── DCS-gRPC-Example.miz
├── Mods/
│   └── tech/
│       └── DCS-gRPC/
│           ├── dcs_grpc.dll
│           └── dcs_grpc.pdb (when available)
├── Scripts/
│   ├── DCS-gRPC/
│   └── Hooks/
│       └── DCS-gRPC.lua
└── Tools/
    └── DCS-gRPC/
        ├── protos/
        │   └── dcs/
        ├── grpcui.exe
        ├── grpcui-LICENSE.txt
        ├── grpcurl.exe
        ├── grpcurl-LICENSE.txt
        └── repl.exe
```

The ZIP contains the contents of `DCS-gRPC-X.Y.Z/` at its root. It intentionally does not add an extra enclosing `DCS-gRPC-X.Y.Z` directory, so users can extract the ZIP directly into their DCS Saved Games directory.

## 5. Manual API documentation generation

The release script generates and validates `api.html` automatically. If you need to reproduce that step manually from the repository root, first create the destination directory and collect the protobuf paths explicitly because PowerShell does not expand `**` glob patterns for native commands:

```powershell
$apiDocsDirectory = ".\Releases\DCS-gRPC-X.Y.Z\Docs\DCS-gRPC"
$protoRoot = (Resolve-Path -LiteralPath ".\protos").Path
$protoFiles = Get-ChildItem -LiteralPath (Join-Path $protoRoot "dcs") -Filter "*.proto" -File -Recurse |
    Select-Object -ExpandProperty FullName

New-Item -ItemType Directory -Force -Path $apiDocsDirectory | Out-Null
protoc `
    "--doc_out=$apiDocsDirectory" `
    "--doc_opt=html,api.html" `
    -I $protoRoot `
    $protoFiles
```

When `protoc-gen-doc` is not on `PATH`, pass its executable explicitly with `--plugin=protoc-gen-doc=C:\path\to\protoc-gen-doc.exe`.

## 6. Inspect the result

After a successful run, confirm that both outputs exist:

```text
Releases/DCS-gRPC-X.Y.Z/
Releases/DCS-gRPC-X.Y.Z.zip
```

The script already verifies required files in the release directory, compares the packaged protobuf counts with the sources, generates `api.html`, and reopens the ZIP to verify its required entries.

Extracting `DCS-gRPC-X.Y.Z.zip` directly into a DCS Saved Games directory places `Mods`, `Scripts`, `Tools`, `Docs`, and `Missions` at the correct level.

## 7. Commit, tag, and publish

Review the changes before committing:

```powershell
git status --short
git diff --check
git diff
```

Commit the version, changelog, release script, and documentation changes:

```powershell
git add Cargo.toml Cargo.lock CHANGELOG.md build_release.ps1 README.md docs/release_process.md
git commit -m "chore: release vX.Y.Z"
```

Create and push the release tag:

```powershell
git tag vX.Y.Z
git push
git push origin vX.Y.Z
```

Finally, create a GitHub release for `vX.Y.Z` and upload `Releases/DCS-gRPC-X.Y.Z.zip`.
