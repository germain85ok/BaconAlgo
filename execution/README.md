# BaconAlgo Execution Engine

Rust-based backend for the BaconAlgo trading platform. Provides real-time market scanning, indicator evaluation, and signal generation.

## 🏗️ Architecture

### Core Components

1. **Config Module** (`src/config.rs`)
   - Environment-based configuration with `.env` support
   - API key management for multiple providers
   - Server and scanner settings

2. **API Server** (`src/main.rs` + `src/api/`)
   - Axum-based HTTP server
   - RESTful endpoints for market data and signals
   - Server-Sent Events (SSE) for real-time streaming
   - CORS and logging middleware

3. **Scanner Engine** (`src/scanner/`)
   - Continuous market scanning
   - Rate-limited API calls
   - Signal broadcasting via tokio channels

4. **Indicator System** (`src/families/`)
   - Trait-based indicator architecture
   - Technical indicators (RSI, MACD, etc.)
   - Fundamental indicators (P/E, EPS, etc.)

## 🚀 API Endpoints

### Health & Status
- `GET /health` - Health check and configuration status

### Signals
- `GET /api/signals` - Get latest trading signals
- `GET /api/signals/stream` - SSE stream of real-time signals
- `GET /signals/live` - Legacy SSE endpoint

### Market Data
- `GET /api/market/fear-greed` - Fear & Greed Index
- `GET /api/market/vix` - VIX volatility index
- `GET /api/market/movers` - Top gainers and losers

### News
- `GET /api/news` - Latest market news

### Control
- `POST /api/scan` - Trigger manual scan

## ⚙️ Configuration

Create a `.env` file in the project root:

```env
# Market Data Providers
ALPHA_VANTAGE_KEY=your_key_here
FINNHUB_KEY=your_key_here
TWELVE_DATA_KEY=your_key_here
POLYGON_KEY=your_key_here

# Crypto APIs
BINANCE_KEY=your_key_here
BINANCE_SECRET=your_secret_here
COINGECKO_KEY=your_key_here

# Broker APIs
ALPACA_KEY=your_key_here
ALPACA_SECRET=your_secret_here
ALPACA_BASE_URL=https://paper-api.alpaca.markets

# Database
SUPABASE_URL=your_supabase_url
SUPABASE_KEY=your_supabase_key

# Redis
REDIS_URL=redis://127.0.0.1:6379

# Server
SERVER_PORT=8080
CORS_ORIGINS=http://localhost:5173,http://localhost:3000

# Scanner
SCAN_INTERVAL_SECS=300
SCAN_SYMBOLS_LIMIT=100
RATE_LIMIT_PER_MIN=60
```

## 🔧 Development

### Build
```bash
cd execution
cargo build
```

### Run
```bash
cargo run
```

### Release Build
```bash
cargo build --release
./target/release/execution
```

### Test
```bash
cargo test
```

## 📦 Dependencies

- **axum** - Web framework
- **tokio** - Async runtime
- **tower/tower-http** - Middleware
- **serde/serde_json** - Serialization
- **dotenvy** - Environment configuration
- **tracing** - Logging
- **reqwest** - HTTP client
- **chrono** - Time handling

## 🔄 Signal Flow

```
Market Data → Scanner → Indicators → Signals → SSE Stream → Frontend
```

1. Scanner fetches market data from configured providers
2. Each indicator evaluates the data
3. Signals are generated based on indicator results
4. Signals are broadcast via tokio channels
5. SSE streams push signals to connected clients

## 🛠️ Adding Indicators

To add a new indicator:

1. Create a new module in `src/families/technical/` or `src/families/fundamental/`
2. Implement the `Indicator` trait:

```rust
use crate::families::{Indicator, MarketData, Signal};

pub struct MyIndicator {
    // Configuration
}

impl Indicator for MyIndicator {
    fn evaluate(&self, data: &MarketData) -> Option<Signal> {
        // Your logic here
    }
    
    fn name(&self) -> &str {
        "MyIndicator"
    }
    
    fn category(&self) -> &str {
        "technical" // or "fundamental"
    }
}
```

3. Register it in the scanner (in `main.rs`):

```rust
let scanner = Scanner::new(scanner_bus.tx.clone());
scanner.add_indicator(Arc::new(MyIndicator::new()));
```

## 🔍 Monitoring

The execution engine includes structured logging with tracing. Set the log level:

```bash
RUST_LOG=execution=debug cargo run
```

Log levels:
- `error` - Critical errors only
- `warn` - Warnings and errors
- `info` - General information (default)
- `debug` - Detailed debugging
- `trace` - Very verbose

## 📊 Performance

- Scanner runs every 5 minutes by default (configurable)
- Rate limiting prevents API quota exhaustion
- Broadcast channels buffer up to 256 signals
- SSE keeps connections alive with heartbeats

## 🚧 Next Steps

- [ ] Implement real market data provider integrations
- [ ] Add actual technical indicators (RSI, MACD, etc.)
- [ ] Add fundamental data fetching
- [ ] Implement database persistence
- [ ] Add Redis caching
- [ ] Create backtest mode
- [ ] Add authentication/authorization
- [ ] Implement WebSocket support
- [ ] Add metrics and monitoring

## 📝 License

Part of the BaconAlgo project.
