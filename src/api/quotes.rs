use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: f64,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HistoricalBar {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub side: String,
    pub opened_at: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub total_value: f64,
    pub cash_balance: f64,
    pub positions: Vec<Position>,
    pub daily_pnl: f64,
    pub total_pnl: f64,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub sound_enabled: bool,
    pub default_timeframe: String,
    pub favorite_symbols: Vec<String>,
    pub risk_tolerance: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub side: String,
    pub quantity: f64,
    pub order_type: String,
    pub limit_price: Option<f64>,
    pub stop_price: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub status: String,
    pub filled_quantity: f64,
    pub average_price: f64,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
