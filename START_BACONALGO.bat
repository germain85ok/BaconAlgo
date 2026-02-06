@echo off
REM BaconAlgo Startup Script for Windows
REM Launches the complete trading platform

echo.
echo ===============================================
echo     BACONALGO ULTIMATE TRADING PLATFORM
echo     V1-V5 Complete Implementation
echo ===============================================
echo.

REM Check if Node.js is installed
where node >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Node.js is not installed!
    echo Please install Node.js 18+ from https://nodejs.org
    pause
    exit /b 1
)

echo [OK] Node.js found: 
node --version
echo.

REM Navigate to dashboard directory
cd /d "%~dp0dashboard"

REM Check if node_modules exists
if not exist "node_modules\" (
    echo Installing dependencies...
    call npm install
    echo.
)

echo.
echo ===============================================
echo     STARTING BACONALGO DASHBOARD
echo ===============================================
echo.
echo Dashboard URL: http://localhost:5173
echo.
echo Press Ctrl+C to stop the server
echo.

REM Start the development server
call npm run dev

pause
