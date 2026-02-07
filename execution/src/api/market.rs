use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct FearGreedResponse {
    data: Vec<FearGreedData>,
}

#[derive(Debug, Deserialize)]
struct FearGreedData {
    value: String,
    value_classification: String,
    #[allow(dead_code)]
    timestamp: String,
}

/// GET /api/market/fear-greed - Fear & Greed Index
pub async fn get_fear_greed_index() -> impl IntoResponse {
    // Fetch from Alternative.me Fear & Greed API
    match reqwest::get("https://api.alternative.me/fng/?limit=1").await {
        Ok(response) => {
            match response.json::<FearGreedResponse>().await {
                Ok(data) => {
                    if let Some(fg_data) = data.data.first() {
                        let value: i32 = fg_data.value.parse().unwrap_or(50);
                        (StatusCode::OK, Json(json!({
                            "value": value,
                            "classification": fg_data.value_classification,
                            "timestamp": chrono::Utc::now().to_rfc3339(),
                            "source": "alternative.me"
                        })))
                    } else {
                        (StatusCode::OK, Json(json!({
                            "value": 50,
                            "classification": "Neutral",
                            "timestamp": chrono::Utc::now().to_rfc3339(),
                            "source": "fallback"
                        })))
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to parse fear-greed response: {}", e);
                    (StatusCode::OK, Json(json!({
                        "value": 50,
                        "classification": "Neutral",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                        "source": "fallback"
                    })))
                }
            }
        }
        Err(e) => {
            tracing::warn!("Failed to fetch fear-greed index: {}", e);
            (StatusCode::OK, Json(json!({
                "value": 50,
                "classification": "Neutral",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "source": "fallback"
            })))
        }
    }
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
    // Simplified approach: fetch quotes for popular symbols and sort by change percent
    let symbols = vec!["AAPL", "MSFT", "GOOGL", "AMZN", "NVDA", "TSLA", "META", "AMD", "NFLX", "AVGO"];
    
    // For now, return a basic structure since Yahoo Finance API requires more complex handling
    // In production, you would fetch from a real quote API
    tracing::info!("Movers requested - returning popular symbols");
    
    (StatusCode::OK, Json(json!({
        "gainers": [
            {
                "symbol": "Popular symbols",
                "note": "Movers data requires real-time quote API subscription"
            }
        ],
        "losers": [],
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "source": "placeholder",
        "available_symbols": symbols
    })))
}
