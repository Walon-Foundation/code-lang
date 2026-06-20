#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$Repo       = "Walon-Foundation/code-lang"
$InstallDir = Join-Path $env:USERPROFILE ".code-lang\bin"
$Target     = "x86_64-pc-windows-msvc"

# ── 32-bit guard ──────────────────────────────────────────────────────────────
if (-not [System.Environment]::Is64BitOperatingSystem) {
    Write-Error "error: 32-bit Windows is not supported"
    exit 1
}

# ── latest release info ───────────────────────────────────────────────────────
try {
    $Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
    $Version = $Release.tag_name
} catch {
    Write-Error "error: could not determine latest release version`n$_"
    exit 1
}

Write-Host "installing code-lang $Version for $Target"
Write-Host ""

# ── helper: download one archive and extract a named binary ──────────────────
function Install-Bin {
    param([string]$BinName)

    $Asset = $Release.assets |
        Where-Object { $_.name -like "*$BinName*" -and $_.name -like "*$Target*" -and $_.name -notlike "*.sha256" } |
        Select-Object -First 1

    if (-not $Asset) {
        Write-Host "warning: no release asset found for '$BinName', skipping"
        return
    }

    $Url     = $Asset.browser_download_url
    $Archive = $Asset.name

    $TmpDir  = Join-Path ([System.IO.Path]::GetTempPath()) "code-lang-install-$([System.IO.Path]::GetRandomFileName())"
    New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null

    try {
        Write-Host "downloading $BinName..."
        $ZipPath = Join-Path $TmpDir $Archive
        Invoke-WebRequest -Uri $Url -OutFile $ZipPath -UseBasicParsing

        Expand-Archive -Path $ZipPath -DestinationPath $TmpDir -Force

        $BinaryPath = Get-ChildItem -Path $TmpDir -Filter "$BinName.exe" -Recurse | Select-Object -First 1
        if (-not $BinaryPath) {
            Write-Error "error: could not find '$BinName.exe' in the archive"
            exit 1
        }

        Copy-Item $BinaryPath.FullName (Join-Path $InstallDir "$BinName.exe") -Force
        Write-Host "installed: $InstallDir\$BinName.exe"

    } finally {
        Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue
    }
}

# ── install ───────────────────────────────────────────────────────────────────
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Install-Bin "code-lang"
Install-Bin "code-lang-fmt"

Write-Host ""

# ── PATH ──────────────────────────────────────────────────────────────────────
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$InstallDir;$UserPath", "User")
    Write-Host "note: added $InstallDir to your PATH"
    Write-Host "restart your terminal for the change to take effect"
    Write-Host ""
}

Write-Host "done - run: code-lang --version"
