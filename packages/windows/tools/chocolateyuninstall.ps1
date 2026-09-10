$ErrorActionPreference = "Stop"

$toolsDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$installDir = Join-Path $toolsDir "bin"

Remove-Item (Join-Path $installDir "rl.exe") -Force -ErrorAction SilentlyContinue
Remove-Item (Join-Path $installDir "rlc.exe") -Force -ErrorAction SilentlyContinue
Remove-Item (Join-Path $installDir "rld.exe") -Force -ErrorAction SilentlyContinue
Remove-Item $installDir -Force -ErrorAction SilentlyContinue
