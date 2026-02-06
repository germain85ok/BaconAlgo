# 🥓 BaconAlgo 2040 - Plateforme de Trading Institutionnelle

**Plateforme de trading professionnelle avec Smart Money Concepts, signaux IA en temps réel et analyse multi-timeframes**

---

## 🌟 Fonctionnalités 2040

### Frontend (SvelteKit 5 + TypeScript)
- **Panneau de Contrôle Neural IA** - Signaux IA en temps réel avec scores de confiance
- **Analyse Multi-Timeframes** - 1m, 5m, 15m, 1h, 4h, 1D avec indicateurs leading/lagging
- **Dashboard** - Aperçu du portfolio, P&L, positions actives
- **Markets Overview** - Données en temps réel pour indices, crypto, commodités
- **Signal Scanner** - Détection de patterns Smart Money Concepts avec scoring
- **Gestion des Risques** - Calculs VaR, stress testing, monitoring des drawdowns, kill switch
- **Analyse Order Flow** - Volume delta, dark pools, flux d'options, suivi institutionnel
- **Auto-Trading** - Exécution automatisée des signaux avec règles configurables
- **Intégration Brokers** - Alpaca, Interactive Brokers, Questrade, Bitget
- **Stream Overlay** - Layout complet 1920x1080 pour streaming OBS
- **PWA** - Installation comme app mobile/desktop avec support hors-ligne
- **Design 2040** - Glassmorphism, néon orange, effets holographiques

### Backend (Rust + Axum)
- **API RESTful** - Endpoints async rapides pour toutes les fonctionnalités
- **WebSocket** - Streaming temps réel des signaux et données de marché
- **Données de Marché** - Indices, crypto et données de commodités en temps réel
- **Génération de Signaux** - Scanning et scoring basés sur SMC
- **Gestion des Ordres** - Placement, annulation et suivi des ordres multi-brokers
- **Backtesting** - Test de stratégies sur données historiques

### Bot Discord
- **Auto-Posting** - Publication automatique des signaux haute qualité sur Discord
- **Commandes** - `!bacon signal`, `!bacon market`, `!bacon stats`
- **Alertes Planifiées** - Résumés ouverture (9h25) et fermeture (16h30) du marché
- **Mises à Jour Marché** - Données de marché en temps réel dans Discord

---

## 🚀 Démarrage Rapide

### Prérequis
- Node.js 18+ et pnpm
- Rust 1.70+
- Compte Supabase (pour authentification et base de données)
- Token Discord bot (optionnel, pour intégration Discord)

### 1. Cloner le Repository
```bash
git clone https://github.com/germain85ok/BaconAlgo.git
cd BaconAlgo
```

### 2. Configuration Backend Rust
```bash
# Le backend utilise un workspace Cargo
cargo build --release

# Ou pour le dev
cd src
cargo run
```

Backend accessible sur `http://localhost:3000`

**Endpoints disponibles:**
- `/signals/live` - SSE pour signaux en temps réel
- `/ws/signals` - WebSocket pour signaux
- `/api/quotes` - Citations de marché
- `/api/historical/:symbol` - Données historiques
- `/api/portfolio` - Portfolio actuel
- `/api/preferences` - Préférences utilisateur
- `/api/signals/:id` - Détails d'un signal

### 3. Configuration Frontend Station
```bash
cd station
pnpm install
cp .env.example .env
# Éditer .env avec vos credentials Supabase
pnpm dev
```

Frontend accessible sur `http://localhost:5173`

### 4. Configuration Frontend Dashboard
```bash
cd dashboard
pnpm install
pnpm dev
```

Dashboard accessible sur `http://localhost:5174`

### 5. Configuration Bot Discord (Optionnel)
```bash
cd discord-bot
pip install -r requirements.txt
cp .env.example .env
# Éditer .env avec votre token Discord
python bot.py
```

---

## 📁 Structure du Projet

```
BaconAlgo/
├── src/                     # Backend Rust
│   ├── main.rs             # Point d'entrée serveur
│   ├── api/                # Routes et modèles API
│   │   ├── models.rs       # Types de données API
│   │   └── routes.rs       # Endpoints REST & WebSocket
│   ├── bus/                # Bus de signaux pub/sub
│   ├── signal/             # Logique génération signaux
│   └── execution/          # Moteur d'exécution
│
├── station/                # Frontend SvelteKit Station
│   ├── src/
│   │   ├── routes/        # Pages
│   │   │   ├── dashboard/ # Pages dashboard
│   │   │   ├── login/     # Authentification
│   │   │   ├── register/  # Inscription
│   │   │   └── stream/    # Overlay streaming
│   │   ├── lib/
│   │   │   ├── components/
│   │   │   │   ├── 2040/              # Composants design 2040
│   │   │   │   │   ├── GlassPanel.svelte
│   │   │   │   │   ├── NeonButton.svelte
│   │   │   │   │   └── AnimatedBackground.svelte
│   │   │   │   └── SignalControlPanel.svelte  # Panneau neural IA
│   │   │   ├── stores/    # Stores Svelte
│   │   │   │   └── auth.ts         # Store authentification
│   │   │   ├── services/  # Services
│   │   │   │   └── apiClient.ts    # Client API backend
│   │   │   ├── types/     # Types TypeScript
│   │   │   │   └── api.ts          # Types API partagés
│   │   │   ├── theme/     # Système de design
│   │   │   │   └── 2040.ts         # Theme 2040
│   │   │   ├── middleware/
│   │   │   │   └── auth.ts         # Middleware auth
│   │   │   ├── supabase/  # Client Supabase
│   │   │   ├── smc/       # Détecteur SMC
│   │   │   ├── brokers/   # Intégrations brokers
│   │   │   ├── risk/      # Gestion risques
│   │   │   └── institutional/ # Analyse order flow
│   │   └── static/
│   │       ├── manifest.json      # Manifest PWA
│   │       ├── sw.js              # Service Worker
│   │       └── offline.html       # Page hors-ligne
│   └── tailwind.config.js         # Config Tailwind 2040
│
├── dashboard/              # Frontend SvelteKit Dashboard
│
├── execution/              # Moteur d'exécution Rust
│   ├── Cargo.toml
│   └── src/
│
├── discord-bot/           # Bot Discord
│   ├── bot.py
│   └── requirements.txt
│
├── supabase/              # Configuration Supabase
│
├── docker-compose.yml     # Orchestration Docker
│
└── README.md
```

---

## 🎨 Stack Technique

### Frontend
- **Framework**: SvelteKit 5 (TypeScript)
- **Styling**: TailwindCSS avec système de design 2040
- **Charts**: Lightweight Charts
- **Auth**: Supabase Auth avec tiers d'abonnement
- **Database**: Supabase (PostgreSQL)
- **PWA**: Service Workers, Web App Manifest, Push Notifications

### Backend
- **Framework**: Rust + Axum
- **WebSocket**: tokio-tungstenite
- **Data**: Serde, serde_json
- **Async**: Tokio runtime
- **CORS**: tower-http

### Services
- **Discord**: Discord.py
- **PWA**: Service Workers, Web Push

---

## 🔧 Configuration

### Variables d'Environnement

#### Station (.env)
```env
# Supabase
PUBLIC_SUPABASE_URL=your_supabase_url
PUBLIC_SUPABASE_ANON_KEY=your_supabase_anon_key

# Backend API
VITE_API_URL=http://localhost:3000
VITE_WS_URL=ws://localhost:3000

# Social Links
PUBLIC_YOUTUBE_URL=https://youtube.com/@yourchannel
PUBLIC_DISCORD_URL=https://discord.gg/yourinvite
```

#### Bot Discord (.env)
```env
DISCORD_TOKEN=your_discord_bot_token
SIGNALS_CHANNEL_ID=your_channel_id
API_URL=http://localhost:3000
```

---

## 🔐 Authentification et Codes Promo

BaconAlgo 2040 utilise Supabase pour l'authentification avec support des tiers d'abonnement:

### Tiers Disponibles
- **FREE** - Accès basique
- **STATION** - Accès complet à la station de trading
- **PRO** - Fonctionnalités professionnelles avancées
- **INSTITUTIONAL** - Niveau institutionnel

### Codes Promo
- `ILOVEBACON-AND-TEA` → Accès STATION (illimité)
- `BACONALGO2040` → Accès STATION gratuit
- `PRO2040` → Accès PRO gratuit

Les codes promo peuvent être appliqués lors de l'inscription ou dans les paramètres du compte.

---

## 📊 Fonctionnalités Détaillées

### Smart Money Concepts (SMC)
- **Fair Value Gaps (FVG)** - Détection des gaps haussiers et baissiers
- **Order Blocks** - Identification des zones d'ordres institutionnels
- **Break of Structure (BOS)** - Détection des changements de tendance
- **Change of Character (CHoCH)** - Shifts de structure de marché

### Gestion des Risques
- **Value at Risk (VaR)** - Niveaux de confiance 95% et 99%
- **Stress Testing** - 6 scénarios de marché
- **Limites d'Exposition** - Contrôles position et leverage
- **Kill Switch** - Fermeture d'urgence de toutes les positions
- **Monitoring Drawdown** - Suivi depuis high water mark

### Analyse Order Flow
- **Volume Delta** - Pression achat vs vente
- **Delta Cumulatif** - Flux de volume net
- **Dark Pool Data** - Transactions bloc institutionnelles
- **Options Flow** - Ratio Put/Call, activité inhabituelle
- **Smart Money Index** - Indicateur composite institutionnel

---

## 📱 PWA (Progressive Web App)

### Installation
1. Ouvrir l'app dans un navigateur
2. Cliquer sur l'icône d'installation dans la barre d'adresse
3. L'app sera installée comme une app native
4. Fonctionne hors-ligne avec données en cache

### Fonctionnalités PWA
- ✅ Installation sur mobile/desktop
- ✅ Mode hors-ligne avec cache intelligent
- ✅ Notifications push pour signaux de trading
- ✅ Mises à jour en arrière-plan
- ✅ Contrôles optimisés tactiles
- ✅ Synchronisation en arrière-plan

---

## 🌐 Déploiement

### Frontend (Vercel/Netlify)
```bash
cd station
pnpm build
# Déployer sur Vercel ou Netlify
```

### Backend (Railway/Render/VPS)
```bash
cargo build --release
# Déployer l'exécutable sur Railway, Render ou votre VPS
```

### Docker (Production)
```bash
docker-compose up -d
```

### Bot Discord (Serveur/PM2)
```bash
cd discord-bot
pm2 start bot.py --name baconalgo-bot
```

---

## 🎨 Design System 2040

Le système de design BaconAlgo 2040 inclut:

### Palette de Couleurs
- **Primary Orange**: #ff6b35 avec effets de glow néon
- **Accent Colors**: Cyan (#00d9ff), Purple (#a855f7), Yellow (#ffd93d)
- **Trading Colors**: Green (bullish), Red (bearish)

### Composants Réutilisables
- **GlassPanel** - Panneaux glassmorphism avec backdrop blur
- **NeonButton** - Boutons avec bordures néon et glow
- **AnimatedBackground** - Fonds animés avec particules/grille

### Effets Visuels
- Glassmorphism avec backdrop-filter blur
- Glows néon orange RGB(255, 107, 53)
- Animations shimmer holographiques
- Gradients animés
- Custom scrollbars

---

## 🤝 Contribution

Les contributions sont bienvenues! Suivez ces étapes:
1. Fork le repository
2. Créer une branche feature (`git checkout -b feature/amazing-feature`)
3. Commit vos changements (`git commit -m 'Add amazing feature'`)
4. Push vers la branche (`git push origin feature/amazing-feature`)
5. Ouvrir une Pull Request

---

## 📄 Licence

Ce projet est sous licence MIT - voir le fichier LICENSE pour les détails.

---

## 🙏 Remerciements

- Méthodologie Smart Money Concepts
- TradingView pour l'inspiration des graphiques
- Alpaca pour l'API de données de marché
- Supabase pour auth et database
- La communauté Rust et Svelte

---

## 📞 Support

- **Email**: support@baconalgo.com
- **Discord**: [Rejoindre notre communauté](https://discord.gg/baconalgo)
- **Documentation**: [docs.baconalgo.com](https://docs.baconalgo.com)

---

**Construit avec 🥓 par BaconAlgo Team**

*Plateforme de trading professionnelle pour le trader moderne du futur 2040.*

