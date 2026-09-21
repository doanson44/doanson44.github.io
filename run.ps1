#!/usr/bin/env pwsh
# doanson44.github.io - Development server launcher (PowerShell port of run.bat)

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

# Native command failures are reported via $LASTEXITCODE, not exceptions.
if (Test-Path Variable:PSNativeCommandUseErrorActionPreference) {
    $PSNativeCommandUseErrorActionPreference = $false
}

Write-Host '========================================'
Write-Host 'Starting development server'
Write-Host '========================================'
Write-Host ''

if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
    Write-Host '========================================'
    Write-Host 'APPLICATION STOPPED WITH ERROR'
    Write-Host '========================================'
    Write-Host "Trunk was not found on PATH. Install it with 'cargo install --locked trunk' (or download a release binary)." -ForegroundColor Red
    exit 1
}

& trunk serve --open
$exitCode = $LASTEXITCODE

if ($exitCode -ne 0) {
    Write-Host ''
    Write-Host '========================================'
    Write-Host 'APPLICATION STOPPED WITH ERROR'
    Write-Host '========================================'
    Write-Host "trunk serve exited with code $exitCode." -ForegroundColor Red
    Write-Host ''
    Read-Host 'Press Enter to exit'
    exit $exitCode
}

exit 0
