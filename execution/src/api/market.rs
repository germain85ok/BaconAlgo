use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

/// GET /api/market/fear-greed - Fear & Greed Index
pub async fn get_fear_greed_index() -> impl IntoResponse {
    // TODO: Fetch from real API (e.g., alternative.me)
    // For now, return mock data
    (StatusCode::OK, Json(json!({
        "value": 65,
        "classification": "Greed",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "source": "mock"
    })))
}

/// GET /api/market/vix - VIX volatility index
pub async fn get_vix() -> impl IntoResponse {
    // TODO: Fetch from real market data provider
    // For now, return mock data
    (StatusCode::OK, Json(json!({
        "value": 18.42,
        "change": -0.23,
        "change_percent": -1.23,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "source": "mock"
    })))
}

/// GET /api/market/movers - Top movers (gainers/losers)
pub async fn get_movers() -> impl IntoResponse {
    // TODO: Fetch from real market data provider
    // For now, return mock data
    (StatusCode::OK, Json(json!({
        "gainers": [
            {
                "symbol": "AAPL",
                "price": 175.43,
                "change_percent": 5.23,
                "volume": 65234100
            },
            {
                "symbol": "TSLA",
                "price": 242.84,
                "change_percent": 4.87,
                "volume": 98543200
            }
        ],
        "losers": [
            {
                "symbol": "META",
                "price": 312.15,
                "change_percent": -3.45,
                "volume": 45123400
            },
            {
                "symbol": "NVDA",
                "price": 495.22,
                "change_percent": -2.91,
                "volume": 78234500
            }
        ],
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "source": "mock"
    })))
}
