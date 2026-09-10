$ErrorActionPreference = "Stop"

$toolsDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$version = $env:ChocolateyPackageVersion

$url = "https://github.com/rl-lang/rl-lang/releases/download/v$version/rl-windows-x86_64.zip"

$packageArgs = @{
  packageName    = $env:ChocolateyPackageName
  unzipLocation  = $toolsDir
  url            = $url
  url64bit       = $url
  checksum       = ""
  checksumType   = "sha256"
  checksum64     = ""
  checksumType64 = "sha256"
}

Install-ChocolateyZipPackage @packageArgs

$installDir = Join-Path $toolsDir "bin"
New-Item -ItemType Directory -Path $installDir -Force | Out-Null

Copy-Item (Join-Path $toolsDir "rl.exe") $installDir -Force
Copy-Item (Join-Path $toolsDir "rlc.exe") $installDir -Force -ErrorAction SilentlyContinue
Copy-Item (Join-Path $toolsDir "rld.exe") $installDir -Force -ErrorAction SilentlyContinue

$machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if ($machinePath -notlike "*$installDir*") {
  [Environment]::SetEnvironmentVariable("Path", "$machinePath;$installDir", "Machine")
  Write-Host "Added $installDir to system PATH." -ForegroundColor Green
}
