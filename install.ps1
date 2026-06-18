#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$Repo       = "Walon-Foundation/code-lang"
$Bin        = "code-lang"
$InstallDir = Join-Path $env:LOCALAPPDATA "code-lang\bin"

# ── arch detection ────────────────────────────────────────────────────────────
if (-not [System.Environment]::Is64BitOperatingSystem) {
    Write-Error "error: 32-bit Windows is not supported"
    exit 1
}
$Target = "x86_64-pc-windows-msvc"

# ── latest version from GitHub ────────────────────────────────────────────────
try {
    $Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
    $Version = $Release.tag_name
} catch {
    Write-Error "error: could not determine latest release version`n$_"
    exit 1
}

$Archive = "$Bin-$Target.zip"
$Url     = "https://github.com/$Repo/releases/download/$Version/$Archive"

Write-Host "installing code-lang $Version for $Target"
Write-Host "from: $Url"
Write-Host ""

# ── temp directory ────────────────────────────────────────────────────────────
$TmpDir = Join-Path ([System.IO.Path]::GetTempPath()) "code-lang-install-$([System.IO.Path]::GetRandomFileName())"
New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null

try {
    # ── download ──────────────────────────────────────────────────────────────
    $ZipPath = Join-Path $TmpDir $Archive
    Invoke-WebRequest -Uri $Url -OutFile $ZipPath -UseBasicParsing

    # ── extract ───────────────────────────────────────────────────────────────
    Expand-Archive -Path $ZipPath -DestinationPath $TmpDir -Force

    # find the binary (cargo-dist may nest it in a subdirectory)
    $BinaryPath = Get-ChildItem -Path $TmpDir -Filter "$Bin.exe" -Recurse | Select-Object -First 1
    if (-not $BinaryPath) {
        Write-Error "error: could not find '$Bin.exe' in the archive"
        exit 1
    }

    # ── install ───────────────────────────────────────────────────────────────
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Copy-Item $BinaryPath.FullName (Join-Path $InstallDir "$Bin.exe") -Force

    Write-Host "installed: $InstallDir\$Bin.exe"

    # ── PATH ──────────────────────────────────────────────────────────────────
    $UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($UserPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$InstallDir;$UserPath", "User")
        Write-Host ""
        Write-Host "note: added $InstallDir to your PATH"
        Write-Host "restart your terminal for the change to take effect"
        Write-Host ""
    }

    Write-Host "done - run: code-lang --version"

} finally {
    Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue
}
