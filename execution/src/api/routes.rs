use axum::{
    extract::{State, Path, Query},
    response::sse::{Event, Sse},
    response::Json,
    http::StatusCode,
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use std::convert::Infallible;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Deserialize;

use crate::bus::SignalBus;
use crate::api::models::*;

pub async fn sse_signals(
    State(bus): State<SignalBus<LiveSignal>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = bus.subscribe();

    let stream = BroadcastStream::new(rx)
        .map(|msg| {
            msg.ok().and_then(|signal| {
                serde_json::to_string(&signal)
                    .ok()
                    .map(|json| Ok(Event::default().data(json)))
            })
        })
        .filter_map(|x| x);

    Sse::new(stream)
}

// Health endpoint
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        uptime_seconds: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        version: "0.1.0".to_string(),
    })
}

// Scan single symbol
pub async fn scan_symbol(
    Path(symbol): Path<String>,
) -> Result<Json<ScanResult>, StatusCode> {
    // Mock implementation - replace with actual scanner logic
    Ok(Json(ScanResult {
        symbol: symbol.to_uppercase(),
        timeframe: "M15".to_string(),
        score: 75,
        signal_type: "day".to_string(),
        direction: "bullish".to_string(),
        entry_price: 100.0,
        stop_loss: 95.0,
        target_price: 110.0,
        tags: vec!["golden_pocket".to_string(), "order_block".to_string()],
    }))
}

#[derive(Deserialize)]
pub struct ScanAllQuery {
    #[serde(rename = "type")]
    pub signal_type: Option<String>,
    pub min_score: Option<u32>,
}

// Scan all symbols
pub async fn scan_all(
    Query(params): Query<ScanAllQuery>,
) -> Json<Vec<ScanResult>> {
    // Mock implementation - replace with actual scanner logic
    let min_score = params.min_score.unwrap_or(60);
    let signal_type = params.signal_type.unwrap_or_else(|| "day".to_string());
    
    let results = vec![
        ScanResult {
            symbol: "BTCUSDT".to_string(),
            timeframe: "M15".to_string(),
            score: 85,
            signal_type: signal_type.clone(),
            direction: "bullish".to_string(),
            entry_price: 45000.0,
            stop_loss: 44000.0,
            target_price: 47000.0,
            tags: vec!["fvg".to_string(), "wave3".to_string()],
        },
        ScanResult {
            symbol: "ETHUSDT".to_string(),
            timeframe: "M15".to_string(),
            score: 72,
            signal_type,
            direction: "bullish".to_string(),
            entry_price: 3000.0,
            stop_loss: 2900.0,
            target_price: 3200.0,
            tags: vec!["orb_breakout".to_string()],
        },
    ];
    
    Json(results.into_iter()
        .filter(|r| r.score >= min_score)
        .collect())
}

// Get top 10 movers (rockets)
pub async fn rockets_top10() -> Json<Vec<RocketMover>> {
    // Mock implementation - replace with actual market data
    Json(vec![
        RocketMover {
            symbol: "SOLUSDT".to_string(),
            price: 150.0,
            change_percent: 15.5,
            volume: 1000000.0,
            market_cap: Some(50000000000.0),
        },
        RocketMover {
            symbol: "AVAXUSDT".to_string(),
            price: 35.0,
            change_percent: 12.3,
            volume: 500000.0,
            market_cap: Some(12000000000.0),
        },
    ])
}

// Market sentiment
pub async fn market_sentiment() -> Json<MarketSentiment> {
    Json(MarketSentiment {
        overall: "bullish".to_string(),
        score: 65,
        fear_greed: 72,
        volume_trend: "increasing".to_string(),
    })
}

// Verify promo code
pub async fn verify_promo(
    Json(req): Json<PromoCodeRequest>,
) -> Json<PromoCodeResponse> {
    // Hardcoded promo codes as specified
    match req.code.to_uppercase().as_str() {
        "ILOVEBACON&TEA" => Json(PromoCodeResponse {
            valid: true,
            promo_type: Some("full_access".to_string()),
            discount_percent: None,
            trial_days: None,
            message: "Full 'station' access activated!".to_string(),
        }),
        "FREEBACON" => Json(PromoCodeResponse {
            valid: true,
            promo_type: Some("trial".to_string()),
            discount_percent: None,
            trial_days: Some(7),
            message: "7 days free trial activated!".to_string(),
        }),
        "BACON20" => Json(PromoCodeResponse {
            valid: true,
            promo_type: Some("discount".to_string()),
            discount_percent: Some(20),
            trial_days: None,
            message: "20% discount applied!".to_string(),
        }),
        "BACON50" => Json(PromoCodeResponse {
            valid: true,
            promo_type: Some("discount".to_string()),
            discount_percent: Some(50),
            trial_days: None,
            message: "50% discount applied!".to_string(),
        }),
        _ => Json(PromoCodeResponse {
            valid: false,
            promo_type: None,
            discount_percent: None,
            trial_days: None,
            message: "Invalid promo code".to_string(),
        }),
    }
}
