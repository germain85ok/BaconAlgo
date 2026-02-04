# BaconAlgo Execution Engine - Quick Start

## 🚀 Getting Started in 60 Seconds

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Configure Environment
```bash
cd execution
cp .env.example .env
# Edit .env with your API keys (optional for development)
```

### 3. Run the Server
```bash
cargo run
```

The server will start on `http://0.0.0.0:8080`

## 📡 Test Endpoints

### Health Check
```bash
curl http://localhost:8080/health
```

### Get Signals
```bash
curl http://localhost:8080/api/signals
```

### Stream Signals (SSE)
```bash
curl -N http://localhost:8080/api/signals/stream
```

### Market Data
```bash
# Fear & Greed Index
curl http://localhost:8080/api/market/fear-greed

# VIX Volatility
curl http://localhost:8080/api/market/vix

# Top Movers
curl http://localhost:8080/api/market/movers
```

### News
```bash
curl http://localhost:8080/api/news
```

### Trigger Scan
```bash
curl -X POST http://localhost:8080/api/scan
```

## 🔧 Development

### Build for Release
```bash
cargo build --release
./target/release/execution
```

### Run Tests
```bash
cargo test
```

### Check Code
```bash
cargo check
```

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

## 📝 Configuration

All configuration is in `.env`. Key settings:

```env
# Server
SERVER_PORT=8080

# Scanner
SCAN_INTERVAL_SECS=300  # 5 minutes
SCAN_SYMBOLS_LIMIT=100

# Logging
RUST_LOG=execution=debug
```

## 🔌 Frontend Integration

### Connect from JavaScript
```javascript
// HTTP requests
const response = await fetch('http://localhost:8080/api/signals');
const data = await response.json();

// SSE streaming
const eventSource = new EventSource('http://localhost:8080/api/signals/stream');
eventSource.onmessage = (event) => {
  const signal = JSON.parse(event.data);
  console.log('New signal:', signal);
};
```

## 📚 Learn More

- Full documentation: [README.md](./README.md)
- API endpoints: See README.md § API Endpoints
- Architecture: See README.md § Architecture
- Adding indicators: See README.md § Adding Indicators

## 🆘 Troubleshooting

### Port already in use
```bash
# Change port in .env
SERVER_PORT=8081
```

### Missing dependencies
```bash
cargo clean
cargo build
```

### CORS errors
```bash
# Add your frontend URL to .env
CORS_ORIGINS=http://localhost:5173,http://localhost:3000
```

---

**Happy coding! 🦀**
