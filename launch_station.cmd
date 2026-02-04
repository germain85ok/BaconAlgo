@echo off
set ROOT=C:\Users\germa\Desktop\COPILOT\BaconAlgo

echo === Starting BaconAlgo Backend ===
start cmd /k "cd /d %ROOT%\execution && cargo run -p execution"

timeout /t 2 >nul

echo === Starting STATION Frontend ===
start cmd /k "cd /d %ROOT%\station && pnpm run dev --open"
