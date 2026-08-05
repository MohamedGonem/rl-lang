#Requires -Version 5.1
$ErrorActionPreference = "Stop"

$Repo = "rl-lang/rl-lang"
$InstallDir = if ($env:RL_INSTALL_DIR) { $env:RL_INSTALL_DIR } else { "$env:LOCALAPPDATA\rl-lang\bin" }

$Bases = @("rl", "rl_debug", "rl_vm", "rl_vm_debug", "rl_treewalker", "rl_treewalker_debug")
$Suffixes = @("", "_no_docs", "_no_repl", "_no_docs_repl")

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
    "rl_treewalker" = "rlp"
    "rl_treewalker_no_docs" = "rlp_nd"
    "rl_treewalker_no_repl" = "rlp_nr"
    "rl_treewalker_no_docs_repl" = "rlp_ndr"
    "rl_treewalker_debug" = "rlpd"
    "rl_treewalker_debug_no_docs" = "rlpd_nd"
    "rl_treewalker_debug_no_repl" = "rlpd_nr"
    "rl_treewalker_debug_no_docs_repl" = "rlpd_ndr"
    "rl_lsp" = "rlsp"
}

function Get-VariantList {
    $variants = @()
    foreach ($b in $Bases) {
        foreach ($s in $Suffixes) {
            $variants += "$b$s"
        }
    }
    $variants += "rl_lsp"
    return $variants
}

function Select-Variants {
    if ($env:RL_VARIANT) {
        return $env:RL_VARIANT -split "," | ForEach-Object { $_.Trim() }
    }

    $variants = Get-VariantList
    Write-Host "Select build(s) to install:"
    for ($i = 0; $i -lt $variants.Count; $i++) {
        Write-Host ("  {0,2}) {1}" -f ($i + 1), $variants[$i])
    }

    $choices = Read-Host "Enter number(s), comma-separated (e.g. 1,3,9), or 'all'"

    if ($choices.Trim().ToLower() -eq "all") {
        return $variants
    }

    $selected = @()
    foreach ($part in ($choices -split ",")) {
        $trimmed = $part.Trim()
        if (-not $trimmed) { continue }

        $index = 0
        if (-not [int]::TryParse($trimmed, [ref]$index) -or $index -lt 1 -or $index -gt $variants.Count) {
            Write-Error "Invalid selection: $trimmed"
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
            Write-Error "Unsupported arch: $arch"
            exit 1
        }
    }
}

function Install-One {
    param($Variant, $Arch, $Version)

    $actual = $ActualName[$Variant]
    if (-not $actual) {
        Write-Warning "No actual-name mapping for '$Variant' — add it to `$ActualName. Skipping."
        return $false
    }

    Write-Host "Installing $Variant ($actual) $Version (windows-$Arch)..."

    $asset = "$actual-windows-$Arch.zip"
    $url = "https://github.com/$Repo/releases/download/$Version/$asset"

    $tmpDir = Join-Path $env:TEMP "rl-install-$(Get-Random)"
    New-Item -ItemType Directory -Path $tmpDir | Out-Null

    try {
        $zipPath = Join-Path $tmpDir $asset
        try {
            Invoke-WebRequest -Uri $url -OutFile $zipPath
        } catch {
            Write-Warning "Failed to download $url. Check that this variant/version combination was published."
            return $false
        }

        Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        $exeName = "$actual.exe"
        Copy-Item -Path (Join-Path $tmpDir $exeName) -Destination (Join-Path $InstallDir $exeName) -Force

        Write-Host "Installed: $InstallDir\$exeName"
        return $true
    } finally {
        Remove-Item -Path $tmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Main {
    $arch = Get-Arch
    $version = if ($env:RL_VERSION) { $env:RL_VERSION } else { "latest" }

    if ($version -eq "latest") {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        $version = $release.tag_name
    }

    $variants = Select-Variants
    $anyFailed = $false

    foreach ($variant in $variants) {
        if (-not (Install-One -Variant $variant -Arch $arch -Version $version)) {
            $anyFailed = $true
        }
    }

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
        Write-Host "Added $InstallDir to your user PATH. Restart your terminal for it to take effect."
    }

    if ($anyFailed) {
        exit 1
    }
}

Main
