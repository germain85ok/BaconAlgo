use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Debug, Deserialize)]
struct FinnhubNews {
    category: String,
    datetime: i64,
    headline: String,
    id: i64,
    image: String,
    related: String,
    source: String,
    summary: String,
    url: String,
}

/// GET /api/news - Latest market news
pub async fn get_news() -> impl IntoResponse {
    // Try to get Finnhub API key from environment
    if let Ok(api_key) = env::var("FINNHUB_KEY") {
        if !api_key.is_empty() {
            let url = format!("https://finnhub.io/api/v1/news?category=general&token={}", api_key);
            
            match reqwest::get(&url).await {
                Ok(response) => {
                    match response.json::<Vec<FinnhubNews>>().await {
                        Ok(news_items) => {
                            let formatted_news: Vec<_> = news_items.iter().take(10).map(|item| {
                                json!({
                                    "id": item.id.to_string(),
                                    "headline": item.headline,
                                    "summary": item.summary,
                                    "source": item.source,
                                    "url": item.url,
                                    "timestamp": chrono::DateTime::from_timestamp(item.datetime, 0)
                                        .map(|dt| dt.to_rfc3339())
                                        .unwrap_or_else(|| {
                                            tracing::warn!("Invalid timestamp {} for news item {}, using current time", item.datetime, item.id);
                                            chrono::Utc::now().to_rfc3339()
                                        }),
                                    "image": item.image,
                                    "symbols": item.related.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>()
                                })
                            }).collect();
                            
                            return (StatusCode::OK, Json(json!({
                                "news": formatted_news,
                                "count": formatted_news.len(),
                                "timestamp": chrono::Utc::now().to_rfc3339(),
                                "source": "finnhub"
                            })));
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse Finnhub news: {}", e);
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch from Finnhub: {}", e);
                }
            }
        }
    }
    
    // Fallback if no API key or fetch failed
    (StatusCode::OK, Json(json!({
        "news": [],
        "count": 0,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "source": "none",
        "message": "No Finnhub API key configured. Set FINNHUB_KEY environment variable to enable news."
    })))
}
