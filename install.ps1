#Requires -Version 5.1
param(
    [string]$Version,
    [string]$Prefix,
    [string]$Variant,
    [switch]$Force,
    [switch]$Uninstall,
    [switch]$Help
)
$ErrorActionPreference = "Stop"

$Repo = "rl-lang/rl-lang"
$InstallDir = if ($Prefix) { $Prefix } elseif ($env:RL_INSTALL_DIR) { $env:RL_INSTALL_DIR } else { "$env:LOCALAPPDATA\rl-lang\bin" }

# --- Variant definitions ---

$Bases = @("rl", "rl_vm", "rl_debug", "rl_vm_debug")
$Suffixes = @("", "_no_docs", "_no_repl", "_no_docs_repl")

$Sections = @{
    1  = "Standard (vm)"
    5  = "VM-only"
    9  = "Debug builds"
    17 = "Language server"
}

$ActualName = @{
    "rl" = "rl"
    "rl_no_docs" = "rl_nd"
    "rl_no_repl" = "rl_nr"
    "rl_no_docs_repl" = "rl_ndr"
    "rl_debug" = "rld"
    "rl_debug_no_docs" = "rld_nd"
    "rl_debug_no_repl" = "rld_nr"
    "rl_debug_no_docs_repl" = "rld_ndr"
    "rl_vm" = "rlc"
    "rl_vm_no_docs" = "rlc_nd"
    "rl_vm_no_repl" = "rlc_nr"
    "rl_vm_no_docs_repl" = "rlc_ndr"
    "rl_vm_debug" = "rlcd"
    "rl_vm_debug_no_docs" = "rlcd_nd"
    "rl_vm_debug_no_repl" = "rlcd_nr"
    "rl_vm_debug_no_docs_repl" = "rlcd_ndr"
    "rl_lsp" = "rlsp"
}

# --- Output helpers ---

function Write-Info { param([string]$Text) Write-Host ("  :: " + $Text) -ForegroundColor DarkGray }
function Write-Ok   { param([string]$Text) Write-Host ("  [ OK ] " + $Text) -ForegroundColor Green }
function Write-Warn { param([string]$Text) Write-Host ("  [WARN] " + $Text) -ForegroundColor Yellow }
function Write-Err  { param([string]$Text) Write-Host ("  [FAIL] " + $Text) -ForegroundColor Red }

# --- Usage ---

function Write-Usage {
    $usage = @"

Usage: install.ps1 [OPTIONS] [VERSION]

Install prebuilt rl-lang binaries from GitHub Releases.

Arguments:
  VERSION    Version to install (default: interactive picker)
             Use "latest", "nightly", or a specific version like "v2.0.0"

Options:
  -Help              Show this help message
  -Prefix DIR        Install directory (default: %LOCALAPPDATA%\rl-lang\bin)
  -Force             Overwrite existing binaries without prompting
  -Variant VARIANTS  Comma-separated list of variants to install
                     Use "all" to install all variants
  -Uninstall         Remove installed binaries

Environment variables:
  RL_INSTALL_DIR     Same as -Prefix
  RL_VERSION         Same as VERSION argument
  RL_VARIANT         Same as -Variant

Examples:
  .\install.ps1                          # interactive install
  .\install.ps1 latest                   # install latest stable
  .\install.ps1 nightly                  # install nightly build
  .\install.ps1 v2.0.0                   # install specific version
  .\install.ps1 -Variant rl,rl_vm latest # install specific variants
  .\install.ps1 -Prefix C:\rl -Force v2.0.0
  .\install.ps1 -Uninstall               # remove all installed binaries
"@
    Write-Host $usage
}

# --- Variant helpers ---

function Get-GroupedVariants {
    $variants = @()
    foreach ($b in $Bases) {
        foreach ($s in $Suffixes) {
            $variants += "$b$s"
        }
    }
    $variants += "rl_lsp"
    return $variants
}

function Print-Menu {
    $variants = Get-GroupedVariants
    Write-Host "  Select a build to install:"
    $i = 1
    foreach ($v in $variants) {
        if ($Sections.ContainsKey($i)) {
            Write-Host ""
            Write-Host ("  " + $Sections[$i]) -ForegroundColor Cyan
        }
        $actual = $ActualName[$v]
        Write-Host ("    {0,2}) {1,-24} ({2})" -f $i, $v, $actual)
        $i++
    }
    Write-Host ""
    Write-Host "  Enter number(s), comma-separated (e.g. 1,3,9), or 'all'." -ForegroundColor DarkGray
}

function Select-Variants {
    if ($Variant) {
        if ($Variant.Trim().ToLower() -eq "all") { return Get-GroupedVariants }
        return $Variant -split "," | ForEach-Object { $_.Trim() }
    }

    if ($env:RL_VARIANT) {
        if ($env:RL_VARIANT.Trim().ToLower() -eq "all") { return Get-GroupedVariants }
        return $env:RL_VARIANT -split "," | ForEach-Object { $_.Trim() }
    }

    if (-not [Environment]::UserInteractive) {
        Write-Err "No interactive terminal detected and RL_VARIANT is not set."
        Write-Err "Non-interactive use requires: `$env:RL_VARIANT = 'rl,rl_vm'; .\install.ps1 [version]"
        exit 1
    }

    Print-Menu
    $choices = Read-Host "  Enter number(s), comma-separated (e.g. 1,3,9), or 'all'"

    $variants = Get-GroupedVariants

    if ($choices.Trim().ToLower() -eq "all") {
        return $variants
    }

    $selected = @()
    foreach ($part in ($choices -split ",")) {
        $trimmed = $part.Trim()
        if (-not $trimmed) { continue }

        $index = 0
        if (-not [int]::TryParse($trimmed, [ref]$index) -or $index -lt 1 -or $index -gt $variants.Count) {
            Write-Err "Invalid selection: $trimmed"
            exit 1
        }

        $selected += $variants[$index - 1]
    }

    return $selected
}

# --- Platform detection ---

function Get-Arch {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x86_64" }
        "ARM64" { return "aarch64" }
        default {
            Write-Err "Unsupported arch: $arch"
            exit 1
        }
    }
}

# --- Version resolution ---

function Test-Release {
    param([string]$Tag)
    try {
        $null = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/tags/$Tag"
        return $true
    } catch {
        return $false
    }
}

function Get-Version {
    param([string]$Requested)

    switch -Regex ($Requested) {
        "^latest$" {
            try {
                $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
                return $release.tag_name
            } catch {
                Write-Err "Could not resolve the latest release from GitHub."
                exit 1
            }
        }
        "^nightly$" {
            if (-not (Test-Release "nightly")) {
                Write-Err "Release 'nightly' not found on GitHub."
                exit 1
            }
            return "nightly"
        }
        "^v[0-9]" {
            if (-not (Test-Release $Requested)) {
                Write-Err "Release '$Requested' not found on GitHub."
                Write-Err "Check the tag (e.g. 'v1.0.0') or use 'latest'/'nightly'."
                exit 1
            }
            return $Requested
        }
        "^[0-9]" {
            $normalized = "v$Requested"
            if (-not (Test-Release $normalized)) {
                Write-Err "Release '$normalized' not found on GitHub."
                Write-Err "Check the tag (e.g. 'v1.0.0') or use 'latest'/'nightly'."
                exit 1
            }
            return $normalized
        }
        default {
            Write-Err "Unknown version '$Requested'."
            Write-Err "Use 'latest', 'nightly', or a specific version like 'v1.0.0'."
            exit 1
        }
    }
}

function Select-VersionPicker {
    Write-Host ""
    Write-Host "  Select a version to install:"
    Write-Host ""
    Write-Host "    1) latest   - newest stable release" -ForegroundColor Cyan
    Write-Host "    2) nightly  - latest build from the dev branch" -ForegroundColor Cyan
    Write-Host "    3) custom   - pin a specific version (e.g. v2.0.0)" -ForegroundColor Cyan
    Write-Host ""
    $choice = Read-Host "  Choose [1]"
    if ([string]::IsNullOrWhiteSpace($choice)) { $choice = "1" }

    switch ($choice.Trim()) {
        "2" { return "nightly" }
        "3" {
            $custom = Read-Host "  Enter version (e.g. v2.0.0)"
            if ([string]::IsNullOrWhiteSpace($custom)) { return "latest" }
            return $custom.Trim()
        }
        default { return "latest" }
    }
}

# --- Checksum verification ---

function Test-Checksum {
    param([string]$FilePath)

    $shaPath = "$FilePath.sha256"
    if (-not (Test-Path $shaPath)) {
        Write-Warn "No checksum file found for $(Split-Path $FilePath -Leaf). Skipping verification."
        return $true
    }

    $expected = (Get-Content $shaPath -Raw).Trim().Split(" ")[0]
    $hash = (Get-FileHash -Path $FilePath -Algorithm SHA256).Hash.ToLower()

    if ($hash -eq $expected) {
        return $true
    } else {
        Write-Err "Checksum mismatch for $(Split-Path $FilePath -Leaf)!"
        Write-Err "  Expected: $expected"
        Write-Err "  Got:      $hash"
        return $false
    }
}

# --- Install ---

function Install-One {
    param($Variant, $Arch, $Version, [switch]$ForceInstall)

    $actual = $ActualName[$Variant]
    if (-not $actual) {
        Write-Err "No actual-name mapping for '$Variant'. Skipping."
        return $false
    }

    # Check if already installed
    $exePath = Join-Path $InstallDir "$actual.exe"
    if ((Test-Path $exePath) -and -not $ForceInstall) {
        Write-Warn "$actual already exists at $exePath. Use -Force to overwrite."
        return $true
    }

    Write-Info "Installing $Variant ($actual) $Version (windows-$Arch)..."

    $asset = "$actual-windows-$Arch.zip"
    $url = "https://github.com/$Repo/releases/download/$Version/$asset"

    $tmpDir = Join-Path $env:TEMP "rl-install-$(Get-Random)"
    New-Item -ItemType Directory -Path $tmpDir | Out-Null

    try {
        $zipPath = Join-Path $tmpDir $asset
        try {
            Invoke-WebRequest -Uri $url -OutFile $zipPath
        } catch {
            Write-Err "Failed to download $url"
            Write-Err "Check that this variant/version combination was published."
            return $false
        }

        # Download and verify checksum
        $shaUrl = "$url.sha256"
        $shaPath = Join-Path $tmpDir "$asset.sha256"
        try {
            Invoke-WebRequest -Uri $shaUrl -OutFile $shaPath -ErrorAction SilentlyContinue
        } catch {
            # Checksum file may not exist for older releases
        }

        if (Test-Path $shaPath) {
            $expected = (Get-Content $shaPath -Raw).Trim().Split(" ")[0]
            $hash = (Get-FileHash -Path $zipPath -Algorithm SHA256).Hash.ToLower()
            if ($hash -ne $expected) {
                Write-Err "Checksum mismatch for $asset!"
                Write-Err "  Expected: $expected"
                Write-Err "  Got:      $hash"
                return $false
            }
        } else {
            Write-Warn "No checksum file found. Skipping verification."
        }

        Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        $exeName = "$actual.exe"
        Copy-Item -Path (Join-Path $tmpDir $exeName) -Destination (Join-Path $InstallDir $exeName) -Force

        Write-Ok "Installed: $InstallDir\$exeName"
        return $true
    } finally {
        Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# --- Uninstall ---

function Uninstall-All {
    $removed = 0
    Write-Host ""
    Write-Host "  Uninstalling rl-lang binaries from $InstallDir..." -ForegroundColor White
    Write-Host ""

    foreach ($name in $ActualName.Values) {
        $path = Join-Path $InstallDir "$name.exe"
        if (Test-Path $path) {
            Remove-Item -Path $path -Force
            Write-Ok "Removed: $path"
            $removed++
        }
    }

    Write-Host ""
    if ($removed -gt 0) {
        Write-Host "  Removed $removed binary(ies)." -ForegroundColor Green
    } else {
        Write-Host "  No rl-lang binaries found in $InstallDir." -ForegroundColor DarkGray
    }
}

# --- Main ---

function Main {
    if ($Help) {
        Write-Usage
        return
    }

    if ($Uninstall) {
        Uninstall-All
        return
    }

    $arch = Get-Arch

    $requested = $null
    if (-not [string]::IsNullOrWhiteSpace($Version)) {
        $requested = $Version.Trim()
    } elseif ($env:RL_VERSION) {
        $requested = $env:RL_VERSION.Trim()
    } elseif ([Environment]::UserInteractive) {
        $requested = Select-VersionPicker
    } else {
        $requested = "latest"
    }

    $resolvedVersion = Get-Version $requested

    Write-Host ""
    Write-Host "  rl-lang installer"
    Write-Info "repo:    $Repo"
    Write-Info "arch:    $arch"
    Write-Info "version: $resolvedVersion"
    Write-Info "install: $InstallDir"
    Write-Info "----------------------------------------"
    Write-Host ""

    $variants = Select-Variants
    $installed = 0
    $total = 0

    foreach ($variant in $variants) {
        $total++
        $params = @{
            Variant = $variant
            Arch = $arch
            Version = $resolvedVersion
        }
        if ($Force) { $params.ForceInstall = $true }

        if (Install-One @params) {
            $installed++
        }
    }

    Write-Host ""
    $failed = $total - $installed
    if ($failed -gt 0) {
        Write-Host ("  Summary: {0}/{1} installed, some failed." -f $installed, $total) -ForegroundColor Yellow
    } else {
        Write-Host ("  Summary: {0}/{1} installed." -f $installed, $total) -ForegroundColor Green
    }

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
        Write-Host "  Added $InstallDir to your user PATH. Restart your terminal for it to take effect."
    }

    if ($failed -gt 0) {
        exit 1
    }
}

Main
