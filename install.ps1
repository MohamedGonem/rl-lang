#Requires -Version 5.1
param([string]$Version)
$ErrorActionPreference = "Stop"

$Repo = "rl-lang/rl-lang"
$InstallDir = if ($env:RL_INSTALL_DIR) { $env:RL_INSTALL_DIR } else { "$env:LOCALAPPDATA\rl-lang\bin" }

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

function Write-Help {
    param([string]$Text)
    Write-Host ("  " + $Text) -ForegroundColor DarkGray
}

function Write-Err {
    param([string]$Text)
    Write-Host ("  [FAIL] " + $Text) -ForegroundColor Red
}

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
    Write-Help "Enter number(s), comma-separated (e.g. 1,3,9), or 'all'."
}

function Select-Variants {
    if ($env:RL_VARIANT) {
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
            Write-Err "Use 'latest', 'nightly', or a specific version like 'v1.0.0'. Select builds interactively or via `$env:RL_VARIANT."
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
    Write-Host "    3) custom   - pin a specific version (e.g. v1.0.0)" -ForegroundColor Cyan
    Write-Host ""
    $choice = Read-Host "  Choose [1]"
    if ([string]::IsNullOrWhiteSpace($choice)) { $choice = "1" }

    switch ($choice.Trim()) {
        "2" { return "nightly" }
        "3" {
            $custom = Read-Host "  Enter version (e.g. v1.0.0)"
            if ([string]::IsNullOrWhiteSpace($custom)) { return "latest" }
            return $custom.Trim()
        }
        default { return "latest" }
    }
}

function Install-One {
    param($Variant, $Arch, $Version)

    $actual = $ActualName[$Variant]
    if (-not $actual) {
        Write-Warning "No actual-name mapping for '$Variant' - add it to `$ActualName. Skipping."
        return $false
    }

    Write-Host ("  :: Installing {0} ({1}) {2} (windows-{3})..." -f $Variant, $actual, $Version, $Arch) -ForegroundColor DarkGray

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

        Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        $exeName = "$actual.exe"
        Copy-Item -Path (Join-Path $tmpDir $exeName) -Destination (Join-Path $InstallDir $exeName) -Force

        Write-Host ("  [ OK ] Installed: {0}\{1}" -f $InstallDir, $exeName) -ForegroundColor Green
        return $true
    } finally {
        Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Main {
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
    Write-Help "repo:    $Repo"
    Write-Help "arch:    $arch"
    Write-Help "version: $resolvedVersion"
    Write-Help "install: $InstallDir"
    Write-Help "----------------------------------------"
    Write-Host ""

    $variants = Select-Variants
    $installed = 0
    $total = 0

    foreach ($variant in $variants) {
        $total++
        if (Install-One -Variant $variant -Arch $arch -Version $resolvedVersion) {
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
