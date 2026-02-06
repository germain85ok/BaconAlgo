@echo off
echo ========================================
echo   BaconAlgo 2030 - Startup Script
echo ========================================
echo.

REM Check if services should start
set /p START_FRONTEND=Start Frontend? (Y/N): 
set /p START_API=Start API? (Y/N): 
set /p START_DISCORD=Start Discord Bot? (Y/N): 

echo.
echo Starting services...
echo.

REM Start Frontend
if /I "%START_FRONTEND%"=="Y" (
    echo [1/3] Starting Frontend...
    start "BaconAlgo Frontend" cmd /k "cd station && npm run dev"
    timeout /t 2 /nobreak > nul
)

REM Start API
if /I "%START_API%"=="Y" (
    echo [2/3] Starting API...
    start "BaconAlgo API" cmd /k "cd api && python main.py"
    timeout /t 2 /nobreak > nul
)

REM Start Discord Bot
if /I "%START_DISCORD%"=="Y" (
    echo [3/3] Starting Discord Bot...
    start "BaconAlgo Discord Bot" cmd /k "cd discord-bot && python bot.py"
    timeout /t 2 /nobreak > nul
)

echo.
echo ========================================
echo   All services started!
echo ========================================
echo.
echo Frontend: http://localhost:5173
echo API: http://localhost:8000
echo API Docs: http://localhost:8000/docs
echo.
pause
