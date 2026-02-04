# ============================
# BaconAlgo — One Click Launch
# ============================

$root = "C:\Users\germa\Desktop\COPILOT\BaconAlgo"

# --- Backend (Rust execution) ---
Start-Process powershell -ArgumentList `
  "-NoExit", `
  "-Command", `
  "cd '$root\execution'; cargo run -p execution"

# --- Frontend (STATION) ---
Start-Sleep -Seconds 2

Start-Process powershell -ArgumentList `
  "-NoExit", `
  "-Command", `
  "cd '$root\station'; pnpm run dev --open"
