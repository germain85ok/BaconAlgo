# 🚀 BaconAlgo 2030 - Quick Start Guide

Get up and running in 5 minutes!

---

## ⚡ Fastest Setup (Development)

### 1. Install Dependencies

```bash
# Frontend
cd station
npm install

# Backend
cd ../api
pip install -r requirements.txt

# Discord Bot (Optional)
cd ../discord-bot
pip install -r requirements.txt
```

### 2. Configure Environment

```bash
# Station
cd station
cp .env.example .env
# Edit .env with your Supabase credentials

# Discord Bot
cd ../discord-bot
cp .env.example .env
# Edit .env with Discord credentials
```

### 3. Start Services

**Terminal 1 - Frontend:**
```bash
cd station
npm run dev
```
→ Open `http://localhost:5173`

**Terminal 2 - Backend API:**
```bash
cd api
python main.py
```
→ API at `http://localhost:8000`

**Terminal 3 - Discord Bot (Optional):**
```bash
cd discord-bot
python bot.py
```

---

## 🎯 First Steps

### 1. Create Account
1. Open `http://localhost:5173`
2. Click "Register"
3. Create account with email
4. Verify email (check inbox)

### 2. Connect a Broker
1. Go to Dashboard → My Brokers
2. Click "Add Broker"
3. Select Alpaca (easiest for testing)
4. Enter paper trading credentials
5. Test connection

### 3. View Market Data
1. Go to Dashboard → Markets
2. See real-time indices, crypto, commodities
3. Check top gainers/losers

### 4. Scan for Signals
1. Go to Dashboard → Scanner
2. View SMC-based signals
3. Filter by score, timeframe, direction

### 5. Monitor Risk
1. Go to Dashboard → Risk
2. View VaR calculations
3. Run stress tests
4. Set exposure limits

---

## 🔑 Getting API Keys

### Supabase
1. Go to [supabase.com](https://supabase.com)
2. Create new project
3. Copy URL and Anon Key
4. Paste in `station/.env`

### Alpaca (Paper Trading)
1. Go to [alpaca.markets](https://alpaca.markets)
2. Sign up for paper trading account
3. Generate API keys
4. Use in broker connection

### Discord Bot
1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create new application
3. Add bot, copy token
4. Paste in `discord-bot/.env`

---

## 🛠️ Troubleshooting

### Frontend won't start
```bash
cd station
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### API errors
```bash
cd api
pip install --upgrade -r requirements.txt
python main.py
```

### Port already in use
```bash
# Frontend (change port)
npm run dev -- --port 3000

# API (change port in main.py)
# Edit: uvicorn.run(app, port=8001)
```

---

## 📚 Next Steps

- Read full [README.md](./README.md)
- Check [Dashboard Routes](#dashboard-routes)
- Explore [API Endpoints](#api-endpoints)
- Join our Discord community

---

## 🎉 You're Ready!

Your BaconAlgo 2030 platform is now running.

Happy trading! 🥓
