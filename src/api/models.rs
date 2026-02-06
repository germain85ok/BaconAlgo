use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LiveSignal {
    pub id: String,
    pub symbol: String,
    pub horizon: String,
    pub ready: bool,
    pub tags: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>, // BUY or SELL
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MarketQuote {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: Option<i64>,
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HistoricalData {
    pub symbol: String,
    pub interval: String,
    pub data: Vec<CandleData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CandleData {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Position {
    pub id: String,
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub opened_at: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Portfolio {
    pub positions: Vec<Position>,
    pub total_value: f64,
    pub cash_balance: f64,
    pub total_pnl: f64,
    pub total_pnl_percent: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserPreferences {
    pub user_id: String,
    pub theme: String,
    pub notifications_enabled: bool,
    pub sound_alerts: bool,
    pub default_timeframe: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignalWithMetrics {
    pub signal: LiveSignal,
    pub timeframes: Vec<TimeframeAnalysis>,
    pub indicators: IndicatorMetrics,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TimeframeAnalysis {
    pub timeframe: String, // 1m, 5m, 15m, 1h, 4h, 1D
    pub trend: String, // BULLISH, BEARISH, NEUTRAL
    pub strength: f64, // 0-100
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IndicatorMetrics {
    pub leading: LeadingIndicators,
    pub lagging: LaggingIndicators,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LeadingIndicators {
    pub rsi: Option<f64>,
    pub stochastic: Option<f64>,
    pub macd_signal: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LaggingIndicators {
    pub ma_50: Option<f64>,
    pub ma_200: Option<f64>,
    pub ema_21: Option<f64>,
}
