@echo off
setlocal

echo ========================================
echo doanson44.github.io - Validation
echo ========================================
echo.

echo [1/4] Checking formatting...
cargo fmt --check
if errorlevel 1 goto :failed

echo.
echo [2/4] Checking WASM compilation...
cargo check --target wasm32-unknown-unknown
if errorlevel 1 goto :failed

echo.
echo [3/4] Running tests...
cargo test
if errorlevel 1 goto :failed

echo.
echo [4/4] Running Clippy...
cargo clippy --target wasm32-unknown-unknown -- -D warnings
if errorlevel 1 goto :failed

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
exit /b 1