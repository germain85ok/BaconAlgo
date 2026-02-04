# 🥓 BaconAlgo SAAS 2030 - Ultimate Trading Platform

Transform BaconAlgo into a full-featured SAAS trading platform with ultra-fast Rust backend, beautiful SvelteKit frontend (OPUS 2030 style), ML/AI predictions, and complete broker integrations.

## 🚀 Features

### Backend (Rust + Axum)
- **Ultra-Fast Scanner Engine**: Scan 1000+ symbols/second using tokio async
- **Smart Money Indicators**:
  - Volume Profile (POC, VAH, VAL)
  - Delta & CVD (Cumulative Volume Delta)
  - Order Blocks (bullish/bearish)
  - Fair Value Gaps (FVG/Imbalance)
  - Fibonacci Golden Pocket (0.618-0.65)
  - ORB5/ORB15 (Opening Range Breakout)
  - Elliott Wave detection (Wave 3 emphasis)
  
### API Endpoints
- `GET /api/health` - Health check
- `GET /api/scan/:symbol` - Scan single symbol
- `GET /api/scan/all?type=scalp|day|swing|long&min_score=60` - Scan all symbols
- `GET /api/signals/live` - Live signals via Server-Sent Events (SSE)
- `GET /api/rockets/top10` - Top 10 movers
- `GET /api/market/sentiment` - Market sentiment analysis
- `POST /api/auth/verify-promo` - Verify promo codes

### Frontend (SvelteKit + TailwindCSS)
- **OPUS 2030 Design**: Glassmorphism, animated backgrounds, neon glows
- **Pages**:
  - Dashboard with stats and top signals
  - Scanner with live filtering
  - Pricing with promo code verification
  - Portfolio manager (planned)
  - Rockets - top movers (planned)
  - Signals history (planned)
  
### ML/AI Engine (Python)
- LSTM price prediction
- CNN candlestick pattern recognition
- Strategy backtesting with performance metrics

### Authentication & Promo Codes
- **ILOVEBACON&TEA** → Full "station" access (unlimited)
- **FREEBACON** → 7 days free trial
- **BACON20** → 20% discount
- **BACON50** → 50% discount

## 🛠️ Tech Stack

- **Backend**: Rust (Axum framework)
- **Frontend**: SvelteKit + TailwindCSS v4
- **Database**: Supabase (PostgreSQL + Auth + Realtime)
- **ML/AI**: Python (PyTorch + NumPy + Pandas)
- **Payments**: PayPal + Stripe (planned)
- **Brokers**: IBKR + Bitget (planned)

## 📦 Installation

### Prerequisites
- Rust 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Node.js 18+ (`https://nodejs.org`)
- Python 3.9+ (for ML engine)

### Backend Setup

```bash
cd execution
cargo build --release
cargo run
```

The API will be available at `http://localhost:3000`

### Frontend Setup

```bash
cd station
npm install
npm run dev
```

The frontend will be available at `http://localhost:5173`

### ML Engine Setup

```bash
cd ml
pip install -r requirements.txt
python predictor.py  # Test predictor
python backtester.py # Test backtester
```

## 🎨 Design System (OPUS 2030)

### Colors
```css
--primary: #ff6b35      /* Orange */
--accent: #ffb347       /* Light orange */
--accent-gold: #ffd700  /* Gold */
--dark: #030305         /* Almost black */
--success: #00d9ff      /* Cyan */
--negative: #ff4d6a     /* Red */
```

### Fonts
- **Display**: Orbitron (titles, headings)
- **Body**: Inter (paragraphs, UI)
- **Mono**: JetBrains Mono (numbers, code)

### Features
- Animated grid background with perspective
- Floating particles with blur
- Glassmorphism cards with shimmer effect
- Neon orange glows
- Smooth transitions everywhere

## 💳 Pricing Plans

| Plan | Price | Features |
|------|-------|----------|
| **Indicator** | $20/mo | TradingView indicator, basic analysis |
| **Scanner** | $30/mo | Multi-symbol scanner, real-time alerts |
| **Station** | $50/mo | Everything + portfolio + ML/AI + broker integrations |

PayPal Button IDs:
- Donation: `KWVC5YDHC6QNU`
- Indicator: `YGZZHDVETM3AL`
- Scanner: `5VM6UAHY34BZS`
- Station: `D8XKK24KV74GC`

## 🗄️ Database Schema

See `supabase/migrations/001_initial_schema.sql` for the complete database schema including:
- User profiles with subscription management
- Promo codes with usage tracking
- Portfolio positions
- Signal history

## 📸 Screenshots

### Dashboard
![Dashboard](https://github.com/user-attachments/assets/bd1079a8-14fa-4ef1-bfe4-52cfb623ee13)

### Scanner
![Scanner](https://github.com/user-attachments/assets/a7b698fa-5afa-4d13-939e-2bd5e25c3090)

### Pricing
![Pricing](https://github.com/user-attachments/assets/1c069f29-753c-4bfb-8527-52be2a1612b4)

## 🚧 Development Status

### ✅ Completed
- [x] Rust scanner indicators (volume profile, delta, order blocks, FVG, fibonacci, ORB, elliott)
- [x] REST API with health, scan, signals, promo verification
- [x] SvelteKit frontend with OPUS 2030 design
- [x] Dashboard, Scanner, and Pricing pages
- [x] Glassmorphism UI components
- [x] Animated background with particles
- [x] Promo code system
- [x] ML/AI engine structure (predictor, pattern detector, backtester)
- [x] Supabase database schema

### 🔄 In Progress / Planned
- [ ] Portfolio manager page
- [ ] Rockets (top movers) page
- [ ] Signals history page
- [ ] Authentication pages (login/register)
- [ ] Supabase integration
- [ ] Broker integrations (IBKR, Bitget)
- [ ] ML model training pipeline
- [ ] Stripe payment integration
- [ ] Real-time WebSocket for live data

## 🤝 Contributing

This is a personal project, but suggestions and feedback are welcome!

## 📄 License

Proprietary - All rights reserved

## 🙏 Acknowledgments

- Built with Rust, SvelteKit, and lots of ☕
- Inspired by modern trading platforms
- Powered by bacon 🥓

---

**Made with 🥓 by the BaconAlgo team**
