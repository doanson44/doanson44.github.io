#!/usr/bin/env pwsh
# doanson44.github.io - Validation (PowerShell port of validate.bat)

[CmdletBinding()]
param(
    [switch]$Clip,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$RemainingArgs
)

$ErrorActionPreference = 'Stop'
if (Test-Path Variable:\PSNativeCommandUseErrorActionPreference) {
    $PSNativeCommandUseErrorActionPreference = $false
}

$shouldClip = $Clip.IsPresent -or ($RemainingArgs -contains '--clip') -or ($RemainingArgs -contains '-clip')

function Copy-ToClipboard {
    param([string]$Text)

    try {
        if (Get-Command Set-Clipboard -ErrorAction SilentlyContinue) {
            Set-Clipboard -Value $Text
            return $true
        }
    }
    catch {
        # Fallback to clip.exe
    }

    try {
        if (Get-Command clip.exe -ErrorAction SilentlyContinue) {
            $Text | clip.exe
            return $true
        }
    }
    catch {
        # Ignore
    }

    return $false
}

$script:LastCheckFailure = $null

function Invoke-Check {
    param(
        [Parameter(Mandatory)][string]$Label,
        [Parameter(Mandatory)][string]$Command,
        [Parameter(Mandatory)][string[]]$Arguments,
        [bool]$Capture = $false
    )

    Write-Host $Label
    $capturedLines = [System.Collections.Generic.List[string]]::new()

    if ($Capture) {
        & $Command @Arguments 2>&1 | ForEach-Object {
            $line = "$_"
            Write-Host $line
            $capturedLines.Add($line)
        }
    }
    else {
        & $Command @Arguments
    }

    if ($LASTEXITCODE -ne 0) {
        $fullCmd = "$Command $($Arguments -join ' ')"
        $outputBody = if ($capturedLines.Count -gt 0) {
            $capturedLines -join [Environment]::NewLine
        }
        else {
            ''
        }

        $script:LastCheckFailure = [PSCustomObject]@{
            Label     = $Label
            Command   = $fullCmd
            ExitCode  = $LASTEXITCODE
            Output    = $outputBody
        }
        throw "'$fullCmd' failed with exit code $LASTEXITCODE."
    }
    Write-Host ''
}

function Invoke-BootstrapAudit {
    param([bool]$Capture = $false)

    $label = '[5/5] Auditing Bootstrap removal...'
    Write-Host $label

    $paths = @('src', 'styles', 'index.html', 'package.json', 'Trunk.toml', 'public')
    $existingPaths = $paths | Where-Object { Test-Path $_ }
    $pattern = 'bootstrap|data-bs-|--bs-|bi-[a-z0-9-]+'

    $matches = @(
        $existingPaths | ForEach-Object {
            if ((Get-Item $_).PSIsContainer) {
                Get-ChildItem $_ -Recurse -File
            }
            else {
                Get-Item $_
            }
        } | Select-String -Pattern $pattern
    )

    if ($matches.Count -gt 0) {
        $output = $matches | Out-String
        $script:LastCheckFailure = [PSCustomObject]@{
            Label     = $label
            Command   = 'Bootstrap reference audit'
            ExitCode  = 1
            Output    = $output.TrimEnd()
        }
        $matches | ForEach-Object { Write-Host $_ }
        throw 'Bootstrap-era references remain in production files.'
    }

    Write-Host ''
}

Write-Host '========================================'
Write-Host 'doanson44.github.io - Validation'
if ($shouldClip) {
    Write-Host '[clip] Clipboard copy on failure enabled'
}
Write-Host '========================================'
Write-Host ''

try {
    Invoke-Check -Label '[1/5] Checking formatting...' -Command cargo -Arguments @('fmt', '--check') -Capture $shouldClip
    Invoke-Check -Label '[2/5] Checking WASM compilation...' -Command cargo -Arguments @('check', '--target', 'wasm32-unknown-unknown') -Capture $shouldClip
    Invoke-Check -Label '[3/5] Running tests...' -Command cargo -Arguments @('test') -Capture $shouldClip
    Invoke-Check -Label '[4/5] Running Clippy...' -Command cargo -Arguments @('clippy', '--target', 'wasm32-unknown-unknown', '--', '-D', 'warnings') -Capture $shouldClip
    Invoke-BootstrapAudit -Capture $shouldClip
}
catch {
    Write-Host ''
    Write-Host '========================================'
    Write-Host 'VALIDATION FAILED'
    Write-Host '========================================'

    if ($script:LastCheckFailure) {
        $fail = $script:LastCheckFailure
        Write-Host "'$($fail.Command)' failed with exit code $($fail.ExitCode)." -ForegroundColor Red

        if ($shouldClip) {
            $clipText = @"
Validation failed at step: $($fail.Label)
Command: $($fail.Command) (exit code: $($fail.ExitCode))

$($fail.Output)
"@
            $copied = Copy-ToClipboard -Text $clipText
            if ($copied) {
                Write-Host '[validate] Error details copied to clipboard.' -ForegroundColor Green
            }
            else {
                Write-Host '[validate] Warning: Failed to copy error details to clipboard.' -ForegroundColor Yellow
            }
        }
    }
    else {
        Write-Host $_.Exception.Message -ForegroundColor Red
    }

    exit 1
}

Write-Host '========================================'
Write-Host 'ALL CHECKS PASSED'
Write-Host '========================================'
exit 0
