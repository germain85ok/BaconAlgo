#!/bin/bash
# BaconAlgo — Health Monitor
# Run with cron: */5 * * * * /path/to/monitor.sh

DISCORD_WEBHOOK="${DISCORD_WEBHOOK_URL}"
BACKEND_URL="http://localhost:8080"
STATION_URL="http://localhost:5173"

check_service() {
    local name=$1
    local url=$2
    local status=$(curl -s -o /dev/null -w "%{http_code}" --max-time 5 "$url")
    
    if [ "$status" != "200" ]; then
        echo "❌ $name is DOWN (HTTP $status)"
        if [ -n "$DISCORD_WEBHOOK" ]; then
            curl -H "Content-Type: application/json" \
                -d "{\"content\": \"🚨 **BaconAlgo Alert**: $name is DOWN (HTTP $status)\"}" \
                "$DISCORD_WEBHOOK"
        fi
        return 1
    else
        echo "✅ $name is UP"
        return 0
    fi
}

echo "🥓 BaconAlgo Health Check — $(date)"
echo "=================================="
check_service "Backend API" "$BACKEND_URL"
check_service "Station" "$STATION_URL"

# Check Docker containers
echo ""
echo "🐳 Docker Status:"
docker compose -f docker-compose.production.yml ps --format "table {{.Name}}\t{{.Status}}"

# Check system resources
echo ""
echo "💻 System Resources:"
echo "  CPU: $(top -bn1 | grep 'Cpu(s)' | awk '{print $2}')% used"
echo "  RAM: $(free -h | awk '/Mem:/ {print $3 "/" $2}')"
echo "  Disk: $(df -h / | awk 'NR==2 {print $3 "/" $2 " (" $5 " used)"}')"
