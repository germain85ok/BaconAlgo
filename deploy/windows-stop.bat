@echo off
echo 🛑 Stopping BaconAlgo...
docker compose -f docker-compose.production.yml down
taskkill /f /im cloudflared.exe 2>nul
echo ✅ BaconAlgo stopped.
