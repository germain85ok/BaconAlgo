#!/bin/bash
echo "🥓 BaconAlgo Production Setup"
echo "=============================="

# Create data directories
mkdir -p data/postgres
mkdir -p data/redis
mkdir -p data/backups
mkdir -p logs

# Copy .env.example to .env if not exists
if [ ! -f .env ]; then
    cp .env.example .env
    echo "⚠️  Created .env from .env.example — EDIT IT with your real keys!"
fi

# Build and start
echo "🐳 Building Docker containers..."
docker compose -f docker-compose.production.yml build

echo "🚀 Starting BaconAlgo..."
docker compose -f docker-compose.production.yml up -d

echo ""
echo "✅ BaconAlgo is running!"
echo ""
echo "Services:"
echo "  🦀 Backend:   http://localhost:8080"
echo "  🎨 Station:   http://localhost:5173"
echo "  🗄️  Postgres:  localhost:5432"
echo "  ⚡ Redis:     localhost:6379"
echo "  🌐 Caddy:     http://localhost:80 / https://localhost:443"
echo ""
echo "Next steps:"
echo "  1. Run: ./deploy/cloudflare-tunnel-setup.sh"
echo "  2. Start tunnel: cloudflared tunnel run baconalgo"
echo "  3. Visit: https://baconalgo.com 🥓"
