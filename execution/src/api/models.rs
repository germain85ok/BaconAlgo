use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Clone, Serialize)]
pub struct LiveSignal {
    pub symbol: String,
    pub horizon: String,
    pub ready: bool,
    pub tags: Value,
    pub reason: String,
    pub ts_unix_ms: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub symbol: String,
    pub timeframe: String,
    pub score: u32,
    pub signal_type: String,  // "scalp", "day", "swing", "long"
    pub direction: String,    // "bullish", "bearish"
    pub entry_price: f64,
    pub stop_loss: f64,
    pub target_price: f64,
    pub tags: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RocketMover {
    pub symbol: String,
    pub price: f64,
    pub change_percent: f64,
    pub volume: f64,
    pub market_cap: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PortfolioPosition {
    pub id: String,
    pub user_id: String,
    pub symbol: String,
    pub side: String,  // "long", "short"
    pub qty: f64,
    pub entry_price: f64,
    pub stop_loss: f64,
    pub take_profit_1: Option<f64>,
    pub take_profit_2: Option<f64>,
    pub take_profit_3: Option<f64>,
    pub notes: Option<String>,
    pub created_at: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarketSentiment {
    pub overall: String,  // "bullish", "bearish", "neutral"
    pub score: i32,       // -100 to 100
    pub fear_greed: u32,  // 0-100
    pub volume_trend: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub uptime_seconds: u64,
    pub version: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PromoCodeRequest {
    pub code: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PromoCodeResponse {
    pub valid: bool,
    pub promo_type: Option<String>,
    pub discount_percent: Option<u32>,
    pub trial_days: Option<u32>,
    pub message: String,
}
