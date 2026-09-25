@echo off
setlocal enabledelayedexpansion

set "CLIP_MODE=0"
for %%A in (%*) do (
    if /i "%%A"=="--clip" set "CLIP_MODE=1"
    if /i "%%A"=="-clip" set "CLIP_MODE=1"
)

set "TEMP_LOG=%TEMP%\validate_%RANDOM%_%RANDOM%.log"
set "TEMP_CLIP=%TEMP%\validate_clip_%RANDOM%_%RANDOM%.txt"

echo ========================================
echo doanson44.github.io - Validation
if "!CLIP_MODE!"=="1" echo [clip] Clipboard copy on failure enabled
echo ========================================
echo.

set "FAILED_LABEL="
set "FAILED_CMD="

echo [1/4] Checking formatting...
set "FAILED_LABEL=[1/4] Checking formatting..."
set "FAILED_CMD=cargo fmt --check"
if "!CLIP_MODE!"=="1" (
    cargo fmt --check > "!TEMP_LOG!" 2>&1
    set "STEP_EXIT=!ERRORLEVEL!"
    if exist "!TEMP_LOG!" type "!TEMP_LOG!"
    if not "!STEP_EXIT!"=="0" goto :failed
) else (
    cargo fmt --check
    if errorlevel 1 goto :failed
)

echo.
echo [2/4] Checking WASM compilation...
set "FAILED_LABEL=[2/4] Checking WASM compilation..."
set "FAILED_CMD=cargo check --target wasm32-unknown-unknown"
if "!CLIP_MODE!"=="1" (
    cargo check --target wasm32-unknown-unknown > "!TEMP_LOG!" 2>&1
    set "STEP_EXIT=!ERRORLEVEL!"
    if exist "!TEMP_LOG!" type "!TEMP_LOG!"
    if not "!STEP_EXIT!"=="0" goto :failed
) else (
    cargo check --target wasm32-unknown-unknown
    if errorlevel 1 goto :failed
)

echo.
echo [3/4] Running tests...
set "FAILED_LABEL=[3/4] Running tests..."
set "FAILED_CMD=cargo test"
if "!CLIP_MODE!"=="1" (
    cargo test > "!TEMP_LOG!" 2>&1
    set "STEP_EXIT=!ERRORLEVEL!"
    if exist "!TEMP_LOG!" type "!TEMP_LOG!"
    if not "!STEP_EXIT!"=="0" goto :failed
) else (
    cargo test
    if errorlevel 1 goto :failed
)

echo.
echo [4/4] Running Clippy...
set "FAILED_LABEL=[4/4] Running Clippy..."
set "FAILED_CMD=cargo clippy --target wasm32-unknown-unknown -- -D warnings"
if "!CLIP_MODE!"=="1" (
    cargo clippy --target wasm32-unknown-unknown -- -D warnings > "!TEMP_LOG!" 2>&1
    set "STEP_EXIT=!ERRORLEVEL!"
    if exist "!TEMP_LOG!" type "!TEMP_LOG!"
    if not "!STEP_EXIT!"=="0" goto :failed
) else (
    cargo clippy --target wasm32-unknown-unknown -- -D warnings
    if errorlevel 1 goto :failed
)

if exist "!TEMP_LOG!" del /f /q "!TEMP_LOG!" >nul 2>&1
if exist "!TEMP_CLIP!" del /f /q "!TEMP_CLIP!" >nul 2>&1

echo.
echo ========================================
echo ALL CHECKS PASSED
echo ========================================
exit /b 0

:failed
echo.
echo ========================================
echo VALIDATION FAILED
echo ========================================
if defined FAILED_CMD (
    echo '!FAILED_CMD!' failed.
)

if "!CLIP_MODE!"=="1" (
    > "!TEMP_CLIP!" echo Validation failed at step: !FAILED_LABEL!
    >> "!TEMP_CLIP!" echo Command: !FAILED_CMD!
    >> "!TEMP_CLIP!" echo.
    if exist "!TEMP_LOG!" type "!TEMP_LOG!" >> "!TEMP_CLIP!"
    clip < "!TEMP_CLIP!" >nul 2>&1
    if !ERRORLEVEL! EQU 0 (
        echo [validate] Error details copied to clipboard.
    ) else (
        echo [validate] Warning: Failed to copy error details to clipboard.
    )
    if exist "!TEMP_CLIP!" del /f /q "!TEMP_CLIP!" >nul 2>&1
)

if exist "!TEMP_LOG!" del /f /q "!TEMP_LOG!" >nul 2>&1
exit /b 1