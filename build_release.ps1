[CmdletBinding()]
param(
    [ValidateNotNullOrEmpty()]
    [string]$RustToolchain = "1.98.0",

    [string]$ReleasesDirectory,

    [string]$ProtocPath,

    [string]$ProtocGenDocPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Assert-RequiredPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $true)]
        [string]$Description,

        [ValidateSet("Any", "Leaf", "Container")]
        [string]$PathType = "Any"
    )

    $TestPathParameters = @{ LiteralPath = $Path }
    if ($PathType -ne "Any") {
        $TestPathParameters.PathType = $PathType
    }

    if (-not (Test-Path @TestPathParameters)) {
        throw "Required $Description was not found: $Path"
    }
}

function Resolve-RequiredExecutable {
    param(
        [string]$ExplicitPath,

        [Parameter(Mandatory = $true)]
        [string]$CommandName,

        [string[]]$FallbackPaths = @()
    )

    if ($ExplicitPath) {
        $ExplicitCommand = Get-Command $ExplicitPath -CommandType Application -ErrorAction SilentlyContinue
        if ($ExplicitCommand) {
            return $ExplicitCommand.Source
        }

        throw "The configured $CommandName executable was not found: $ExplicitPath"
    }

    $Command = Get-Command $CommandName -CommandType Application -ErrorAction SilentlyContinue
    if ($Command) {
        return $Command.Source
    }

    foreach ($FallbackPath in $FallbackPaths) {
        if (Test-Path -LiteralPath $FallbackPath -PathType Leaf) {
            return (Resolve-Path -LiteralPath $FallbackPath).Path
        }
    }

    throw "$CommandName was not found. Add it to PATH or pass its explicit path to this script."
}

# Define paths
$RepoRoot = $PSScriptRoot
$ReleasesDir = if ($ReleasesDirectory) {
    [System.IO.Path]::GetFullPath($ReleasesDirectory)
} else {
    Join-Path $RepoRoot "Releases"
}

# 1. Get the version from Cargo.toml
Write-Host "Reading version from Cargo.toml..."
$CargoTomlPath = Join-Path $RepoRoot "Cargo.toml"
$VersionLine = Select-String -Path $CargoTomlPath -Pattern '^version\s*=\s*"([^"]+)"' | Select-Object -First 1
if (-not $VersionLine) {
    throw "Could not find the workspace version in Cargo.toml."
}
$Version = $VersionLine.Matches.Groups[1].Value
Write-Host "Building release for version: $Version"

# 2. Build the project in release mode
Write-Host "Compiling the server and REPL with Rust $RustToolchain..."
$CargoCommand = Resolve-RequiredExecutable -CommandName "cargo"
Push-Location $RepoRoot
try {
    & $CargoCommand "+$RustToolchain" build --release --locked -p dcs-grpc -p dcs-grpc-repl
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo release build failed with exit code $LASTEXITCODE."
    }
} finally {
    Pop-Location
}

$ServerDllPath = Join-Path $RepoRoot "target\release\dcs_grpc.dll"
$ServerPdbPath = Join-Path $RepoRoot "target\release\dcs_grpc.pdb"
$ReplExecutablePath = Join-Path $RepoRoot "target\release\dcs-grpc-repl.exe"
$HooksSourceDir = Join-Path $RepoRoot "lua\Hooks"
$LuaBridgeSourceDir = Join-Path $RepoRoot "lua\DCS-gRPC"
$ProtosSourceDir = Join-Path $RepoRoot "protos\dcs"
$SampleMission = Join-Path $RepoRoot "sample_release\DCS-gRPC-0.8.1\Missions\DCS-gRPC-Example.miz"
$SampleTools = Join-Path $RepoRoot "sample_release\DCS-gRPC-0.8.1\Tools\DCS-gRPC"
$DocsToCopy = @("CHANGELOG.md", "README.md", "STATUS.md")
$ToolFiles = @("grpcui.exe", "grpcui-LICENSE.txt", "grpcurl.exe", "grpcurl-LICENSE.txt")

Assert-RequiredPath -Path $ServerDllPath -Description "server DLL" -PathType Leaf
Assert-RequiredPath -Path $ReplExecutablePath -Description "REPL executable" -PathType Leaf
Assert-RequiredPath -Path $HooksSourceDir -Description "Lua hooks directory" -PathType Container
Assert-RequiredPath -Path $LuaBridgeSourceDir -Description "Lua bridge directory" -PathType Container
Assert-RequiredPath -Path $ProtosSourceDir -Description "protobuf source directory" -PathType Container
Assert-RequiredPath -Path $SampleMission -Description "sample mission" -PathType Leaf

$ProtoFiles = @(Get-ChildItem -LiteralPath $ProtosSourceDir -Filter "*.proto" -File -Recurse)
if ($ProtoFiles.Count -eq 0) {
    throw "No protobuf files were found under: $ProtosSourceDir"
}

foreach ($Doc in $DocsToCopy) {
    Assert-RequiredPath -Path (Join-Path $RepoRoot $Doc) -Description "documentation file $Doc" -PathType Leaf
}

foreach ($Tool in $ToolFiles) {
    Assert-RequiredPath -Path (Join-Path $SampleTools $Tool) -Description "bundled tool $Tool" -PathType Leaf
}

# Resolve the documentation generators before replacing an existing release.
$CargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $HOME ".cargo" }
$BundledProtocPaths = @(
    Get-ChildItem -LiteralPath (Join-Path $CargoHome "git\checkouts") -Filter "protoc-*-win64.exe" -File -Recurse -ErrorAction SilentlyContinue |
        Sort-Object LastWriteTime -Descending |
        Select-Object -ExpandProperty FullName
)
$ProtocCommand = Resolve-RequiredExecutable -ExplicitPath $ProtocPath -CommandName "protoc" -FallbackPaths $BundledProtocPaths
$GoBinProtocGenDoc = Join-Path $HOME "go\bin\protoc-gen-doc.exe"
$ProtocGenDocCommand = Resolve-RequiredExecutable -ExplicitPath $ProtocGenDocPath -CommandName "protoc-gen-doc" -FallbackPaths @($GoBinProtocGenDoc)

# 3. Create Release Directory Structure
$ReleaseName = "DCS-gRPC-$Version"
$ReleaseFolder = Join-Path $ReleasesDir $ReleaseName
Write-Host "Creating release folder structure at: $ReleaseFolder"

New-Item -ItemType Directory -Force -Path $ReleasesDir | Out-Null

if (Test-Path $ReleaseFolder) {
    Remove-Item -Recurse -Force $ReleaseFolder
}

$DirsToCreate = @(
    "Docs\DCS-gRPC",
    "Missions",
    "Mods\tech\DCS-gRPC",
    "Scripts\DCS-gRPC",
    "Scripts\Hooks",
    "Tools\DCS-gRPC\protos\dcs"
)

foreach ($Dir in $DirsToCreate) {
    New-Item -ItemType Directory -Force -Path (Join-Path $ReleaseFolder $Dir) | Out-Null
}

# 4. Copy Files
Write-Host "Copying files to release folder..."

# Server DLL
Copy-Item $ServerDllPath (Join-Path $ReleaseFolder "Mods\tech\DCS-gRPC\") -Force
# Debug symbols (PDB)
if (Test-Path -LiteralPath $ServerPdbPath -PathType Leaf) {
    Copy-Item $ServerPdbPath (Join-Path $ReleaseFolder "Mods\tech\DCS-gRPC\") -Force
}

# Lua Bridge
Copy-Item (Join-Path $RepoRoot "lua\Hooks\*") (Join-Path $ReleaseFolder "Scripts\Hooks\") -Recurse -Force
Copy-Item (Join-Path $RepoRoot "lua\DCS-gRPC\*") (Join-Path $ReleaseFolder "Scripts\DCS-gRPC\") -Recurse -Force

# The Cargo binary is named dcs-grpc-repl.exe; retain the documented release name.
Copy-Item $ReplExecutablePath (Join-Path $ReleaseFolder "Tools\DCS-gRPC\repl.exe") -Force

# Protos (Tools)
Copy-Item (Join-Path $RepoRoot "protos\dcs\*") (Join-Path $ReleaseFolder "Tools\DCS-gRPC\protos\dcs\") -Recurse -Force

# Protos (Docs - for reference)
$DocsProtosDir = Join-Path $ReleaseFolder "Docs\DCS-gRPC\protos\dcs"
New-Item -ItemType Directory -Force -Path $DocsProtosDir | Out-Null
Copy-Item (Join-Path $RepoRoot "protos\dcs\*") $DocsProtosDir -Recurse -Force

# Docs
foreach ($Doc in $DocsToCopy) {
    Copy-Item (Join-Path $RepoRoot $Doc) (Join-Path $ReleaseFolder "Docs\DCS-gRPC\") -Force
}

# Example Mission
Copy-Item $SampleMission (Join-Path $ReleaseFolder "Missions\") -Force

# Third-party tools (grpcui and grpcurl)
foreach ($Tool in $ToolFiles) {
    $ToolPath = Join-Path $SampleTools $Tool
    Copy-Item $ToolPath (Join-Path $ReleaseFolder "Tools\DCS-gRPC\") -Force
}

# Generate the required API reference.
Write-Host "Generating api.html using protoc-gen-doc..."
$ApiDocsDir = Join-Path $ReleaseFolder "Docs\DCS-gRPC"
$ApiHtmlPath = Join-Path $ApiDocsDir "api.html"
$ProtocArgs = @(
    "--plugin=protoc-gen-doc=$ProtocGenDocCommand",
    "--doc_out=$ApiDocsDir",
    "--doc_opt=html,api.html",
    "-I",
    (Join-Path $RepoRoot "protos")
)
$ProtocArgs += $ProtoFiles.FullName

& $ProtocCommand @ProtocArgs
if ($LASTEXITCODE -ne 0) {
    throw "API documentation generation failed with exit code $LASTEXITCODE."
}
Assert-RequiredPath -Path $ApiHtmlPath -Description "generated API documentation" -PathType Leaf
Write-Host "api.html generated successfully."

$RequiredReleaseFiles = @(
    "Mods\tech\DCS-gRPC\dcs_grpc.dll",
    "Scripts\Hooks\DCS-gRPC.lua",
    "Tools\DCS-gRPC\repl.exe",
    "Docs\DCS-gRPC\CHANGELOG.md",
    "Docs\DCS-gRPC\README.md",
    "Docs\DCS-gRPC\STATUS.md",
    "Docs\DCS-gRPC\api.html",
    "Missions\DCS-gRPC-Example.miz",
    "Tools\DCS-gRPC\grpcui.exe",
    "Tools\DCS-gRPC\grpcui-LICENSE.txt",
    "Tools\DCS-gRPC\grpcurl.exe",
    "Tools\DCS-gRPC\grpcurl-LICENSE.txt"
)

foreach ($RelativePath in $RequiredReleaseFiles) {
    Assert-RequiredPath -Path (Join-Path $ReleaseFolder $RelativePath) -Description "release artifact $RelativePath" -PathType Leaf
}

$ToolsProtoCount = @(Get-ChildItem -LiteralPath (Join-Path $ReleaseFolder "Tools\DCS-gRPC\protos\dcs") -Filter "*.proto" -File -Recurse).Count
$DocsProtoCount = @(Get-ChildItem -LiteralPath (Join-Path $ReleaseFolder "Docs\DCS-gRPC\protos\dcs") -Filter "*.proto" -File -Recurse).Count
if (($ToolsProtoCount -ne $ProtoFiles.Count) -or ($DocsProtoCount -ne $ProtoFiles.Count)) {
    throw "The packaged protobuf file count does not match the source file count ($($ProtoFiles.Count))."
}

# 5. Package as ZIP
$ZipPath = Join-Path $ReleasesDir "$ReleaseName.zip"
if (Test-Path $ZipPath) {
    Remove-Item -Force $ZipPath
}

Write-Host "Compressing release into ZIP at: $ZipPath"
Compress-Archive -Path (Join-Path $ReleaseFolder "*") -DestinationPath $ZipPath -CompressionLevel Optimal
Assert-RequiredPath -Path $ZipPath -Description "release ZIP" -PathType Leaf

Add-Type -AssemblyName System.IO.Compression.FileSystem
$Archive = [System.IO.Compression.ZipFile]::OpenRead($ZipPath)
try {
    $ArchiveEntries = @($Archive.Entries | ForEach-Object { $_.FullName.Replace("\", "/") })
    foreach ($RelativePath in $RequiredReleaseFiles) {
        $RequiredEntry = $RelativePath.Replace("\", "/")
        if ($ArchiveEntries -notcontains $RequiredEntry) {
            throw "Required release artifact is missing from the ZIP: $RequiredEntry"
        }
    }
} finally {
    $Archive.Dispose()
}

Write-Host ""
Write-Host "=========================================" -ForegroundColor Green
Write-Host "Release created successfully!" -ForegroundColor Green
Write-Host "Folder: $ReleaseFolder" -ForegroundColor Cyan
Write-Host "Zip:    $ZipPath" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Green
