use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

/// GET /api/news - Latest market news
pub async fn get_news() -> impl IntoResponse {
    // TODO: Integrate with real news API (Finnhub, Alpha Vantage, etc.)
    // For now, return mock data
    (StatusCode::OK, Json(json!({
        "news": [
            {
                "id": "1",
                "headline": "Fed signals potential rate cuts in 2024",
                "summary": "Federal Reserve indicates possible interest rate reductions...",
                "source": "Reuters",
                "url": "https://example.com/news/1",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "sentiment": "neutral",
                "symbols": ["SPY", "QQQ"]
            },
            {
                "id": "2",
                "headline": "Tech stocks rally on strong earnings",
                "summary": "Major technology companies beat Q4 expectations...",
                "source": "Bloomberg",
                "url": "https://example.com/news/2",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "sentiment": "positive",
                "symbols": ["AAPL", "MSFT", "GOOGL"]
            },
            {
                "id": "3",
                "headline": "Bitcoin reaches new yearly high",
                "summary": "Cryptocurrency market sees renewed investor interest...",
                "source": "CoinDesk",
                "url": "https://example.com/news/3",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "sentiment": "positive",
                "symbols": ["BTCUSD"]
            }
        ],
        "count": 3,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "source": "mock"
    })))
}
