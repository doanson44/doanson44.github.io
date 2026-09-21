#!/usr/bin/env pwsh
# doanson44.github.io - Validation (PowerShell port of validate.bat)

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

function Invoke-Check {
    param(
        [Parameter(Mandatory)][string]$Label,
        [Parameter(Mandatory)][string]$Command,
        [Parameter(Mandatory)][string[]]$Arguments
    )

    Write-Host $Label
    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "'$Command $($Arguments -join ' ')' failed with exit code $LASTEXITCODE."
    }
    Write-Host ''
}

Write-Host '========================================'
Write-Host 'doanson44.github.io - Validation'
Write-Host '========================================'
Write-Host ''

try {
    Invoke-Check -Label '[1/4] Checking formatting...' -Command cargo -Arguments @('fmt', '--check')
    Invoke-Check -Label '[2/4] Checking WASM compilation...' -Command cargo -Arguments @('check', '--target', 'wasm32-unknown-unknown')
    Invoke-Check -Label '[3/4] Running tests...' -Command cargo -Arguments @('test')
    Invoke-Check -Label '[4/4] Running Clippy...' -Command cargo -Arguments @('clippy', '--target', 'wasm32-unknown-unknown', '--', '-D', 'warnings')
}
catch {
    Write-Host ''
    Write-Host '========================================'
    Write-Host 'VALIDATION FAILED'
    Write-Host '========================================'
    Write-Host $_.Exception.Message -ForegroundColor Red
    exit 1
}

Write-Host '========================================'
Write-Host 'ALL CHECKS PASSED'
Write-Host '========================================'
exit 0
