#!/bin/bash
# BaconAlgo — Database Backup
BACKUP_DIR="./data/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/baconalgo_$TIMESTAMP.sql.gz"

echo "🥓 BaconAlgo — Backing up database..."

docker compose -f docker-compose.production.yml exec -T postgres \
    pg_dump -U baconalgo baconalgo | gzip > "$BACKUP_FILE"

echo "✅ Backup saved: $BACKUP_FILE"

# Keep only last 30 backups
ls -t $BACKUP_DIR/baconalgo_*.sql.gz | tail -n +31 | xargs -r rm
echo "🧹 Cleaned old backups (keeping last 30)"
