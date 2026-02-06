use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize, Deserialize)]
pub struct HistoricalBar {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Deserialize)]
pub struct HistoricalQuery {
    pub timeframe: Option<String>,
    pub limit: Option<usize>,
}

pub async fn get_historical(
    Path(symbol): Path<String>,
    Query(params): Query<HistoricalQuery>,
) -> Result<Json<Vec<HistoricalBar>>, StatusCode> {
    let timeframe = params.timeframe.unwrap_or_else(|| "1h".to_string());
    let limit = params.limit.unwrap_or(100).min(1000);

    let mut bars = Vec::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let interval = match timeframe.as_str() {
        "1m" => 60,
        "5m" => 300,
        "15m" => 900,
        "1h" => 3600,
        "4h" => 14400,
        "1D" => 86400,
        "1W" => 604800,
        _ => 3600,
    };

    for i in 0..limit {
        let timestamp = now - (interval * i as i64);
        let base_price = 45000.0 + (i as f64 * 10.0);
        
        bars.push(HistoricalBar {
            timestamp,
            open: base_price,
            high: base_price + 100.0,
            low: base_price - 100.0,
            close: base_price + 50.0,
            volume: 1000000.0 + (i as f64 * 10000.0),
        });
    }

    bars.reverse();
    Ok(Json(bars))
}
