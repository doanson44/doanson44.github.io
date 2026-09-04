@echo off
setlocal

echo ========================================
echo Starting development server
echo ========================================
echo.

trunk serve --open

if errorlevel 1 (
    echo.
    echo ========================================
    echo APPLICATION STOPPED WITH ERROR
    echo ========================================
    pause
    exit /b 1
)

endlocal