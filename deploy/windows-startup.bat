@echo off
echo 🥓 Starting BaconAlgo Production...

REM Start Docker containers
cd /d C:\Users\germa\Desktop\COPILOT\BaconAlgo
docker compose -f docker-compose.production.yml up -d

REM Wait for services to start
timeout /t 10 /nobreak

REM Start Cloudflare Tunnel
start /min "" cloudflared tunnel run baconalgo

echo ✅ BaconAlgo is LIVE!
echo 🌐 https://baconalgo.com
