# 🥓 BaconAlgo 2030 - Complete Institutional Trading Platform

**Professional-grade trading platform with Smart Money Concepts, automated trading, and institutional-level risk management.**

---

## 🌟 Features

### Frontend (SvelteKit + TypeScript)
- **Dashboard** - Portfolio overview, P&L tracking, active positions
- **Markets Overview** - Real-time data for indices, crypto, commodities, and top movers
- **Signal Scanner** - Smart Money Concepts (SMC) pattern detection with scoring
- **Risk Management** - VaR calculations, stress testing, drawdown monitoring, kill switch
- **Order Flow Analysis** - Volume delta, dark pool data, options flow, institutional tracking
- **Auto-Trading** - Automated signal execution with configurable rules
- **Broker Integration** - Connect Alpaca, Interactive Brokers, Questrade, Bitget
- **Stream Overlay** - Full 1920x1080 layout for OBS streaming
- **PWA Support** - Install as mobile/desktop app with offline support

### Backend (FastAPI)
- **RESTful API** - Fast, async endpoints for all platform features
- **Market Data** - Real-time indices, crypto, and commodity data
- **Signal Generation** - SMC-based signal scanning and scoring
- **Order Management** - Place, cancel, and track orders across brokers
- **Backtesting** - Test strategies on historical data

### Discord Bot
- **Auto-Posting** - Automatically post high-quality signals to Discord
- **Commands** - `!bacon signal`, `!bacon market`, `!bacon stats`
- **Scheduled Alerts** - Market open (9:25 AM) and close (4:30 PM) summaries
- **Market Updates** - Real-time market data in Discord

---

## 🚀 Quick Start

### Prerequisites
- Node.js 18+ and npm/pnpm
- Python 3.10+
- Supabase account (for authentication and database)
- Discord bot token (optional, for Discord integration)

### 1. Clone Repository
```bash
git clone https://github.com/germain85ok/BaconAlgo.git
cd BaconAlgo
```

### 2. Setup Frontend (Station)
```bash
cd station
npm install
cp .env.example .env
# Edit .env with your Supabase credentials
npm run dev
```

Frontend will be running at `http://localhost:5173`

### 3. Setup Backend API
```bash
cd ../api
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
python main.py
```

API will be running at `http://localhost:8000`

### 4. Setup Discord Bot (Optional)
```bash
cd ../discord-bot
pip install -r requirements.txt
cp .env.example .env
# Edit .env with your Discord bot token and channel ID
python bot.py
```

---

## 📁 Project Structure

```
BaconAlgo/
├── station/                 # SvelteKit frontend
│   ├── src/
│   │   ├── routes/         # Pages
│   │   │   ├── dashboard/  # Dashboard pages
│   │   │   └── stream/     # Stream overlay
│   │   ├── lib/
│   │   │   ├── stores/     # Svelte stores
│   │   │   ├── services/   # Market data, audio
│   │   │   ├── smc/        # SMC detector & signal generator
│   │   │   ├── brokers/    # Broker integrations
│   │   │   ├── risk/       # Risk management
│   │   │   ├── institutional/ # Order flow analysis
│   │   │   └── security/   # Encryption & security
│   │   └── app.html
│   └── static/
│       ├── manifest.json   # PWA manifest
│       ├── sw.js           # Service worker
│       └── offline.html    # Offline fallback
├── api/                    # FastAPI backend
│   ├── main.py            # Main API server
│   └── requirements.txt
├── discord-bot/           # Discord bot
│   ├── bot.py            # Bot implementation
│   ├── requirements.txt
│   └── .env.example
└── README.md
```

---

## 🎨 Tech Stack

### Frontend
- **Framework**: SvelteKit 5 (TypeScript)
- **Styling**: TailwindCSS
- **Charts**: Lightweight Charts
- **Auth**: Supabase Auth
- **Database**: Supabase (PostgreSQL)

### Backend
- **Framework**: FastAPI (Python)
- **Data**: Pandas, NumPy
- **Brokers**: Alpaca API, IB TWS API

### Services
- **Discord**: Discord.py
- **PWA**: Service Workers, Web App Manifest

---

## 🔧 Configuration

### Environment Variables

#### Station (.env)
```env
PUBLIC_SUPABASE_URL=your_supabase_url
PUBLIC_SUPABASE_ANON_KEY=your_supabase_anon_key
```

#### Discord Bot (.env)
```env
DISCORD_TOKEN=your_discord_bot_token
SIGNALS_CHANNEL_ID=your_channel_id
API_URL=http://localhost:8000
```

---

## 📊 Features Deep Dive

### Smart Money Concepts (SMC)
- **Fair Value Gaps (FVG)** - Detect bullish and bearish gaps
- **Order Blocks** - Identify institutional order zones
- **Break of Structure (BOS)** - Trend change detection
- **Change of Character (CHoCH)** - Market structure shifts

### Risk Management
- **Value at Risk (VaR)** - 95% and 99% confidence levels
- **Stress Testing** - 6 market scenarios
- **Exposure Limits** - Position and leverage controls
- **Kill Switch** - Emergency close all positions
- **Drawdown Monitoring** - Track from high water mark

### Order Flow Analysis
- **Volume Delta** - Buy vs sell pressure
- **Cumulative Delta** - Net volume flow
- **Dark Pool Data** - Institutional block trades
- **Options Flow** - Put/Call ratio, unusual activity
- **Smart Money Index** - Composite institutional indicator

---

## 🌐 Deployment

### Frontend (Vercel)
```bash
cd station
npm run build
# Deploy to Vercel
```

### Backend (Railway/Render)
```bash
cd api
# Deploy Python app to Railway or Render
```

### Discord Bot (Server)
```bash
cd discord-bot
# Run on a server or use pm2
pm2 start bot.py --name baconalgo-bot
```

---

## 📱 PWA Installation

1. Open the app in a browser
2. Click the install icon in the address bar
3. App will be installed as a native app
4. Works offline with cached data

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## 🙏 Acknowledgments

- Smart Money Concepts methodology
- TradingView for charting inspiration
- Alpaca for market data API
- Supabase for auth and database

---

## 📞 Support

- **Email**: support@baconalgo.com
- **Discord**: [Join our community](https://discord.gg/baconalgo)
- **Documentation**: [docs.baconalgo.com](https://docs.baconalgo.com)

---

**Built with 🥓 by BaconAlgo Team**

*Professional trading platform for the modern trader.*
