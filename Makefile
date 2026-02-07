# =============================================================================
# 🥓 BACONALGO — MAKEFILE
# =============================================================================
# Easy commands for development and production deployment
# =============================================================================

.PHONY: dev prod stop logs backup clean help

# Default target
help:
	@echo "🥓 BaconAlgo — Available Commands:"
	@echo ""
	@echo "  make dev     - Start dev services (Postgres + Redis only)"
	@echo "  make prod    - Deploy full production stack"
	@echo "  make stop    - Stop all services"
	@echo "  make logs    - View production logs"
	@echo "  make backup  - Backup database"
	@echo "  make clean   - Clean temp files and artifacts"
	@echo ""

# Development: just DB + Redis
dev:
	@echo "🥓 Starting development services..."
	docker compose -f docker-compose.dev.yml up -d
	@echo ""
	@echo "✅ Dev services running (Postgres on :5432 + Redis on :6379)"
	@echo ""
	@echo "📋 Next steps:"
	@echo "  1. Run 'cd station && pnpm dev' for frontend (:5173)"
	@echo "  2. Run 'cd execution && cargo run' for backend (:8080)"
	@echo ""

# Production: full stack
prod:
	@echo "🥓 Deploying production stack..."
	@if [ -f ./deploy/setup-production.sh ]; then \
		./deploy/setup-production.sh; \
	else \
		docker compose -f docker-compose.production.yml up -d; \
	fi
	@echo "✅ Production deployment complete!"

# Stop everything
stop:
	@echo "🛑 Stopping all services..."
	docker compose -f docker-compose.production.yml down 2>/dev/null || true
	docker compose -f docker-compose.dev.yml down 2>/dev/null || true
	@echo "✅ All services stopped"

# View logs
logs:
	docker compose -f docker-compose.production.yml logs -f

# Backup database
backup:
	@echo "💾 Creating database backup..."
	@if [ -f ./deploy/backup.sh ]; then \
		./deploy/backup.sh; \
	else \
		@echo "⚠️  No backup script found at ./deploy/backup.sh"; \
	fi

# Clean up temp files and build artifacts
clean:
	@echo "🧹 Cleaning temp files..."
	@find . -maxdepth 1 -name "*.save" -delete 2>/dev/null || true
	@find . -maxdepth 1 -name "*.bak" -delete 2>/dev/null || true
	@echo "✅ Clean complete!"
