$ErrorActionPreference = "Stop"

# Define paths
$RepoRoot = $PSScriptRoot
$ReleasesDir = Join-Path $RepoRoot "Releases"

# 1. Get the version from Cargo.toml
Write-Host "Reading version from Cargo.toml..."
$CargoTomlPath = Join-Path $RepoRoot "Cargo.toml"
$VersionLine = Select-String -Path $CargoTomlPath -Pattern '^version\s*=\s*"([^"]+)"' | Select-Object -First 1
if (-not $VersionLine) {
    Write-Error "Could not find workspace version in Cargo.toml!"
    exit 1
}
$Version = $VersionLine.Matches.Groups[1].Value
Write-Host "Building release for version: $Version"

# 2. Build the project in release mode
Write-Host "Compiling Rust project in release mode..."
Set-Location $RepoRoot
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed!"
    exit $LASTEXITCODE
}

# 3. Create Release Directory Structure
$ReleaseName = "DCS-gRPC-$Version"
$ReleaseFolder = Join-Path $ReleasesDir $ReleaseName
Write-Host "Creating release folder structure at: $ReleaseFolder"

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
Copy-Item (Join-Path $RepoRoot "target\release\dcs_grpc.dll") (Join-Path $ReleaseFolder "Mods\tech\DCS-gRPC\") -Force

# Lua Bridge
Copy-Item (Join-Path $RepoRoot "lua\Hooks\*") (Join-Path $ReleaseFolder "Scripts\Hooks\") -Recurse -Force
Copy-Item (Join-Path $RepoRoot "lua\DCS-gRPC\*") (Join-Path $ReleaseFolder "Scripts\DCS-gRPC\") -Recurse -Force

# Tools
if (Test-Path (Join-Path $RepoRoot "target\release\repl.exe")) {
    Copy-Item (Join-Path $RepoRoot "target\release\repl.exe") (Join-Path $ReleaseFolder "Tools\DCS-gRPC\") -Force
}

# Protos
Copy-Item (Join-Path $RepoRoot "protos\dcs\*") (Join-Path $ReleaseFolder "Tools\DCS-gRPC\protos\dcs\") -Recurse -Force

# Docs
$DocsToCopy = @("CHANGELOG.md", "README.md", "STATUS.md")
foreach ($Doc in $DocsToCopy) {
    if (Test-Path (Join-Path $RepoRoot $Doc)) {
        Copy-Item (Join-Path $RepoRoot $Doc) (Join-Path $ReleaseFolder "Docs\DCS-gRPC\") -Force
    }
}

# Generate api.html if protoc-gen-doc is available
Write-Host "Attempting to generate api.html using protoc-gen-doc..."
try {
    # Ensure Go's bin directory is in the PATH just in case protoc-gen-doc was installed via go install
    $GoBin = Join-Path $HOME "go\bin"
    if (($env:PATH -split ';') -notcontains $GoBin) {
        $env:PATH += ";$GoBin"
    }

    # Check if protoc is available
    $protocCheck = Get-Command protoc -ErrorAction SilentlyContinue
    if ($protocCheck) {
        $protocArgs = @("--doc_out=$(Join-Path $ReleaseFolder 'Docs\DCS-gRPC')", "--doc_opt=html,api.html", "-I", "$(Join-Path $RepoRoot 'protos')")
        $protoFiles = Get-ChildItem -Path (Join-Path $RepoRoot "protos\dcs") -Filter *.proto -Recurse | Select-Object -ExpandProperty FullName
        $protocArgs += $protoFiles
        
        & protoc $protocArgs
        if ($LASTEXITCODE -eq 0) {
            Write-Host "api.html generated successfully."
        } else {
            Write-Host "Failed to generate api.html (protoc returned an error). Skipping." -ForegroundColor Yellow
        }
    } else {
        Write-Host "protoc not found in PATH. Skipping api.html generation." -ForegroundColor Yellow
    }
} catch {
    Write-Host "Error generating api.html: $_. Skipping." -ForegroundColor Yellow
}

# 5. Package as ZIP
$ZipPath = Join-Path $ReleasesDir "$ReleaseName.zip"
if (Test-Path $ZipPath) {
    Remove-Item -Force $ZipPath
}

Write-Host "Compressing release into ZIP at: $ZipPath"
Compress-Archive -Path (Join-Path $ReleaseFolder "*") -DestinationPath $ZipPath

Write-Host ""
Write-Host "=========================================" -ForegroundColor Green
Write-Host "Release created successfully!" -ForegroundColor Green
Write-Host "Folder: $ReleaseFolder" -ForegroundColor Cyan
Write-Host "Zip:    $ZipPath" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Green
