# 🥓 BaconAlgo — Guide de Déploiement Production

## Prérequis
- Docker Desktop installé
- Compte Cloudflare (gratuit)
- Domaine baconalgo.com configuré sur Cloudflare DNS

## Installation rapide

### 1. Cloner le repo
```bash
git clone https://github.com/germain85ok/BaconAlgo.git
cd BaconAlgo
```

### 2. Configurer les variables d'environnement
```bash
cp .env.example .env
# Éditer .env avec tes vraies clés API
```

### 3. Lancer la production
```bash
chmod +x deploy/*.sh
./deploy/setup-production.sh
```

### 4. Configurer Cloudflare Tunnel
```bash
./deploy/cloudflare-tunnel-setup.sh
```

### 5. C'est en ligne!
Visite **https://baconalgo.com** 🥓

---

## Architecture

### Serveur Production
- **CPU**: AMD Ryzen 7 7800X3D (8 cores / 16 threads, 96MB V-Cache)
- **RAM**: 32 GB DDR5
- **GPU**: NVIDIA RTX 4070 (12 GB VRAM, 5888 CUDA cores)
- **SSD**: WD BLACK SN850X 2TB NVMe
- **OS**: Windows 11 Pro (WSL2 disponible)
- **Internet**: 447 Mbps down / 56 Mbps up / 12ms ping

### Services Docker

#### 🦀 Backend (Rust Axum)
- Port: 8080
- CPU: 4 cores max
- RAM: 4GB max
- Rust high-performance API
- Scanner Rayon multithreadé

#### 🎨 Station (SvelteKit Node SSR)
- Port: 5173
- CPU: 2 cores max
- RAM: 2GB max
- SSR pour SEO optimal

#### 🗄️ PostgreSQL 16
- Port: 5432
- Optimisé pour 32GB RAM:
  - Shared buffers: 4GB
  - Effective cache size: 16GB
  - Work mem: 256MB
- Persistent storage: `./data/postgres`

#### ⚡ Redis 7
- Port: 6379
- Max memory: 2GB (allkeys-lru)
- Cache pour market data
- Persistent storage: `./data/redis`

#### 🌐 Caddy Reverse Proxy
- Ports: 80, 443, 8443
- HTTPS automatique (Let's Encrypt)
- HTTP/3 support
- Compression gzip/zstd
- Routes:
  - `/api/*` → Backend
  - `/ws/*` → Backend (WebSocket)
  - `/sse/*` → Backend (Server-Sent Events)
  - `/*` → Station (SvelteKit)

---

## Commandes utiles

### Démarrage
```bash
# Production complète
./deploy/setup-production.sh

# Cloudflare Tunnel
cloudflared tunnel run baconalgo

# Windows (auto-start au boot)
deploy\windows-startup.bat
```

### Monitoring
```bash
# Voir les logs en temps réel
docker compose -f docker-compose.production.yml logs -f

# Logs d'un service spécifique
docker compose -f docker-compose.production.yml logs -f backend
docker compose -f docker-compose.production.yml logs -f station

# Statut des services
docker compose -f docker-compose.production.yml ps

# Health check manuel
./deploy/monitor.sh
```

### Maintenance
```bash
# Backup base de données
./deploy/backup.sh

# Redémarrer un service
docker compose -f docker-compose.production.yml restart backend
docker compose -f docker-compose.production.yml restart station

# Redémarrer tout
docker compose -f docker-compose.production.yml restart

# Arrêter
docker compose -f docker-compose.production.yml down

# Arrêter et supprimer les volumes (⚠️ DANGER)
docker compose -f docker-compose.production.yml down -v
```

### Mise à jour
```bash
# Pull les derniers changements
git pull origin main

# Rebuild et redéployer
docker compose -f docker-compose.production.yml build
docker compose -f docker-compose.production.yml up -d

# Ou avec script
./deploy/setup-production.sh
```

---

## Monitoring & Alertes

### Health Check Automatique
```bash
# Ajouter au crontab (vérifie toutes les 5 minutes)
*/5 * * * * /chemin/vers/BaconAlgo/deploy/monitor.sh
```

### Discord Alerts
Configure `DISCORD_WEBHOOK_URL` dans `.env` pour recevoir des alertes automatiques si un service tombe.

### Logs
Les logs sont stockés dans `./logs/` et peuvent être consultés via:
```bash
docker compose -f docker-compose.production.yml logs -f
```

---

## Capacité & Performance

### Charge supportée
- **10,000-20,000 abonnés total**
- **1,500-3,000 utilisateurs simultanés**
- Scanner 10K+ instruments en parallèle
- ML/LSTM sur RTX 4070 (12GB VRAM)

### Optimisations
- Cloudflare CDN devant (cache global)
- Cloudflare Tunnel (pas besoin d'ouvrir ports)
- Compression Brotli/gzip automatique
- HTTP/3 support
- Static assets cache 1 an
- PostgreSQL tuné pour 7800X3D
- Redis LRU pour market data
- Rayon parallel scanner (16 threads)

---

## Cloudflare Tunnel

### Setup initial
```bash
./deploy/cloudflare-tunnel-setup.sh
```

### Démarrer le tunnel
```bash
cloudflared tunnel run baconalgo
```

### Installer comme service (Linux/WSL2)
```bash
sudo cloudflared service install
sudo systemctl start cloudflared
sudo systemctl enable cloudflared
```

### Installer comme service (Windows)
```powershell
# PowerShell en Admin
cloudflared service install
sc start cloudflared
```

---

## Backup & Restore

### Backup automatique
```bash
./deploy/backup.sh
```

Backups stockés dans `./data/backups/`
- Nom: `baconalgo_YYYYMMDD_HHMMSS.sql.gz`
- Rétention: 30 derniers backups

### Restore manuel
```bash
# Arrêter les services
docker compose -f docker-compose.production.yml down

# Restore depuis backup
gunzip -c ./data/backups/baconalgo_20240206_120000.sql.gz | \
  docker compose -f docker-compose.production.yml exec -T postgres \
  psql -U baconalgo baconalgo

# Redémarrer
docker compose -f docker-compose.production.yml up -d
```

---

## Sécurité

### Firewall
- Cloudflare Tunnel = Pas besoin d'ouvrir ports
- Tous les services isolés dans Docker network
- HTTPS automatique via Caddy

### Variables sensibles
- **JAMAIS** commit `.env` dans git
- Utilise `.env.example` comme template
- Change tous les mots de passe par défaut

### Headers de sécurité
Caddy ajoute automatiquement:
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Strict-Transport-Security: max-age=31536000`

---

## Troubleshooting

### Services ne démarrent pas
```bash
# Voir les logs détaillés
docker compose -f docker-compose.production.yml logs

# Vérifier l'espace disque
df -h

# Vérifier la RAM
free -h
```

### Backend ne répond pas
```bash
# Voir les logs backend
docker compose -f docker-compose.production.yml logs backend

# Redémarrer backend
docker compose -f docker-compose.production.yml restart backend

# Vérifier health
curl http://localhost:8080/health
```

### Station ne charge pas
```bash
# Voir les logs station
docker compose -f docker-compose.production.yml logs station

# Rebuild station
docker compose -f docker-compose.production.yml build station
docker compose -f docker-compose.production.yml up -d station
```

### PostgreSQL slow
```bash
# Vacuum database
docker compose -f docker-compose.production.yml exec postgres \
  vacuumdb -U baconalgo -d baconalgo -z -v

# Voir les queries lentes
docker compose -f docker-compose.production.yml exec postgres \
  psql -U baconalgo -d baconalgo -c "SELECT * FROM pg_stat_statements ORDER BY total_exec_time DESC LIMIT 10;"
```

### Redis mémoire pleine
```bash
# Vérifier mémoire
docker compose -f docker-compose.production.yml exec redis redis-cli INFO memory

# Flush cache (⚠️ perte de données)
docker compose -f docker-compose.production.yml exec redis redis-cli FLUSHALL
```

---

## Support

### Contact
- **Email**: germain85@hotmail.com
- **Discord**: Voir `PUBLIC_DISCORD_URL` dans .env
- **YouTube**: Voir `PUBLIC_YOUTUBE_URL` dans .env

### Issues
Ouvre une issue sur GitHub: https://github.com/germain85ok/BaconAlgo/issues

---

## 🥓 Bon déploiement!

**BaconAlgo** est maintenant prêt à servir des milliers d'utilisateurs depuis ton PC Windows 11 avec Cloudflare Tunnel!

Pour toute question, consulte la documentation ou contacte le support.

**Made with 🥓 in Montréal**
