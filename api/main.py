from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import List, Optional
import uvicorn

app = FastAPI(title="BaconAlgo API", version="2.0")

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Models
class Signal(BaseModel):
    id: str
    symbol: str
    timeframe: str
    direction: str
    score: int
    entry: float
    stopLoss: float
    takeProfit: float
    targets: List[float]
    riskRewardRatio: float
    confidence: float
    reasons: List[str]
    createdAt: str

class MarketSummary(BaseModel):
    indices: List[dict]
    crypto: List[dict]
    commodities: List[dict]
    gainers: List[dict]
    losers: List[dict]

class Order(BaseModel):
    symbol: str
    quantity: float
    side: str
    type: str
    limitPrice: Optional[float] = None
    stopPrice: Optional[float] = None

class BacktestRequest(BaseModel):
    symbol: str
    startDate: str
    endDate: str
    strategy: str

# Endpoints
@app.get("/")
async def root():
    return {"message": "BaconAlgo API v2.0", "status": "running"}

@app.get("/api/signals")
async def get_signals(
    timeframe: Optional[str] = None,
    minScore: Optional[int] = 0
):
    """Get trading signals"""
    # Mock signals for now
    signals = [
        {
            "id": "NVDA-15m-1234567890",
            "symbol": "NVDA",
            "timeframe": "15m",
            "direction": "LONG",
            "score": 85,
            "entry": 870.00,
            "stopLoss": 852.60,
            "takeProfit": 922.20,
            "targets": [887.40, 904.80, 922.20],
            "riskRewardRatio": 3.0,
            "confidence": 0.85,
            "reasons": ["Bullish Order Block", "FVG detected", "Break of Structure"],
            "createdAt": "2026-02-06T05:30:00Z"
        },
        {
            "id": "TSLA-1h-1234567891",
            "symbol": "TSLA",
            "timeframe": "1h",
            "direction": "LONG",
            "score": 72,
            "entry": 245.00,
            "stopLoss": 240.10,
            "takeProfit": 264.70,
            "targets": [251.45, 257.90, 264.70],
            "riskRewardRatio": 2.5,
            "confidence": 0.72,
            "reasons": ["Bullish FVG", "Support zone"],
            "createdAt": "2026-02-06T05:15:00Z"
        }
    ]
    
    # Filter by score
    if minScore:
        signals = [s for s in signals if s["score"] >= minScore]
    
    return {"signals": signals, "count": len(signals)}

@app.post("/api/signals/scan")
async def scan_signals(symbols: List[str]):
    """Scan for new signals"""
    return {
        "message": f"Scanning {len(symbols)} symbols",
        "symbols": symbols,
        "newSignals": 0
    }

@app.get("/api/market/summary")
async def market_summary():
    """Get market overview"""
    return {
        "indices": [
            {"symbol": "SPY", "name": "S&P 500 ETF", "price": 450.25, "change": 5.32, "changePercent": 1.19},
            {"symbol": "QQQ", "name": "Nasdaq 100 ETF", "price": 380.15, "change": 4.18, "changePercent": 1.11},
        ],
        "crypto": [
            {"symbol": "BTC", "price": 52000.00, "changePercent24h": 2.5},
            {"symbol": "ETH", "price": 2800.00, "changePercent24h": 3.2},
        ],
        "commodities": [
            {"symbol": "GC=F", "name": "Gold", "price": 2050.30, "changePercent": 0.61},
        ]
    }

@app.get("/api/market/movers")
async def top_movers():
    """Get top gainers and losers"""
    return {
        "gainers": [
            {"symbol": "NVDA", "name": "NVIDIA", "price": 875.50, "changePercent": 8.5},
            {"symbol": "AMD", "name": "AMD", "price": 165.30, "changePercent": 7.2},
        ],
        "losers": [
            {"symbol": "META", "name": "Meta", "price": 380.20, "changePercent": -5.3},
        ]
    }

@app.post("/api/backtest")
async def run_backtest(request: BacktestRequest):
    """Run backtest"""
    return {
        "symbol": request.symbol,
        "startDate": request.startDate,
        "endDate": request.endDate,
        "totalTrades": 42,
        "winRate": 68.5,
        "totalReturn": 24.8,
        "maxDrawdown": -8.2
    }

@app.post("/api/orders")
async def place_order(order: Order):
    """Place an order"""
    return {
        "id": "order-123456",
        "symbol": order.symbol,
        "quantity": order.quantity,
        "side": order.side,
        "type": order.type,
        "status": "pending",
        "message": "Order placed successfully"
    }

@app.get("/api/positions")
async def get_positions():
    """Get current positions"""
    return {
        "positions": [
            {
                "symbol": "NVDA",
                "quantity": 100,
                "avgPrice": 850.00,
                "currentPrice": 875.50,
                "unrealizedPnL": 2550.00,
                "unrealizedPnLPercent": 3.0
            }
        ]
    }

@app.get("/api/account")
async def get_account():
    """Get account information"""
    return {
        "cash": 50000.00,
        "equity": 125430.50,
        "buyingPower": 100000.00,
        "portfolioValue": 125430.50,
        "dayTradeCount": 0
    }

@app.post("/api/orders/cancel-all")
async def cancel_all_orders():
    """Cancel all open orders"""
    return {"message": "All orders cancelled", "count": 0}

@app.post("/api/positions/close-all")
async def close_all_positions():
    """Close all positions (Kill Switch)"""
    return {"message": "All positions closed", "count": 3}

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
