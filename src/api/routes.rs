use axum::{
    extract::{State, Query, Path, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::{sse::{Event, Sse}, IntoResponse, Json},
    http::StatusCode,
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use std::convert::Infallible;
use serde::Deserialize;
use futures_util::{sink::SinkExt, stream::StreamExt as FuturesStreamExt};

use crate::bus::SignalBus;
use crate::api::models::*;

// ============================================================================
// SSE Endpoints (Server-Sent Events)
// ============================================================================

pub async fn sse_signals(
    State(bus): State<SignalBus<LiveSignal>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = bus.subscribe();
    let stream = BroadcastStream::new(rx)
        .map(|msg| {
            match msg {
                Ok(signal) => {
                    serde_json::to_string(&signal)
                        .ok()
                        .map(|json| Ok(Event::default().data(json)))
                }
                Err(_) => None,
            }
        })
        .filter_map(|x| x);

    Sse::new(stream)
}

// ============================================================================
// WebSocket Endpoints
// ============================================================================

pub async fn ws_signals(
    ws: WebSocketUpgrade,
    State(bus): State<SignalBus<LiveSignal>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_signal_socket(socket, bus))
}

async fn handle_signal_socket(socket: WebSocket, bus: SignalBus<LiveSignal>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = bus.subscribe();

    // Spawn task to send signals to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(signal) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&signal) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Spawn task to receive messages from client (for ping/pong)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

// ============================================================================
// REST API Endpoints
// ============================================================================

#[derive(Deserialize)]
pub struct QuoteQuery {
    symbols: Option<String>, // Comma-separated symbols
}

pub async fn get_market_quotes(
    Query(params): Query<QuoteQuery>,
) -> Result<Json<ApiResponse<Vec<MarketQuote>>>, StatusCode> {
    // Mock implementation - replace with real market data fetching
    let symbols = params.symbols.unwrap_or_else(|| "BTC,ETH,SPY".to_string());
    let quotes: Vec<MarketQuote> = symbols
        .split(',')
        .map(|symbol| MarketQuote {
            symbol: symbol.trim().to_string(),
            price: 50000.0,
            change: 500.0,
            change_percent: 1.01,
            volume: Some(1000000),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(quotes),
        error: None,
    }))
}

#[derive(Deserialize)]
pub struct HistoricalQuery {
    interval: Option<String>, // 1m, 5m, 15m, 1h, 4h, 1d
    limit: Option<usize>,
}

pub async fn get_historical_data(
    Path(symbol): Path<String>,
    Query(params): Query<HistoricalQuery>,
) -> Result<Json<ApiResponse<HistoricalData>>, StatusCode> {
    // Mock implementation
    let interval = params.interval.unwrap_or_else(|| "1h".to_string());
    let limit = params.limit.unwrap_or(100);
    
    let data: Vec<CandleData> = (0..limit)
        .map(|i| CandleData {
            timestamp: chrono::Utc::now().to_rfc3339(),
            open: 50000.0 + i as f64,
            high: 50100.0 + i as f64,
            low: 49900.0 + i as f64,
            close: 50050.0 + i as f64,
            volume: 1000000,
        })
        .collect();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(HistoricalData {
            symbol,
            interval,
            data,
        }),
        error: None,
    }))
}

pub async fn get_portfolio() -> Result<Json<ApiResponse<Portfolio>>, StatusCode> {
    // Mock implementation
    let positions = vec![
        Position {
            id: "pos_1".to_string(),
            symbol: "BTC".to_string(),
            quantity: 0.5,
            entry_price: 48000.0,
            current_price: 50000.0,
            pnl: 1000.0,
            pnl_percent: 4.17,
            opened_at: chrono::Utc::now().to_rfc3339(),
        },
    ];

    Ok(Json(ApiResponse {
        success: true,
        data: Some(Portfolio {
            positions,
            total_value: 25000.0,
            cash_balance: 10000.0,
            total_pnl: 1000.0,
            total_pnl_percent: 4.17,
        }),
        error: None,
    }))
}

pub async fn get_user_preferences() -> Result<Json<ApiResponse<UserPreferences>>, StatusCode> {
    // Mock implementation
    Ok(Json(ApiResponse {
        success: true,
        data: Some(UserPreferences {
            user_id: "user_1".to_string(),
            theme: "dark".to_string(),
            notifications_enabled: true,
            sound_alerts: true,
            default_timeframe: "1h".to_string(),
        }),
        error: None,
    }))
}

pub async fn update_user_preferences(
    Json(prefs): Json<UserPreferences>,
) -> Result<Json<ApiResponse<UserPreferences>>, StatusCode> {
    // Mock implementation - would save to database
    Ok(Json(ApiResponse {
        success: true,
        data: Some(prefs),
        error: None,
    }))
}

pub async fn get_signal_details(
    Path(signal_id): Path<String>,
) -> Result<Json<ApiResponse<SignalWithMetrics>>, StatusCode> {
    // Mock implementation
    let signal = LiveSignal {
        id: signal_id,
        symbol: "BTC".to_string(),
        horizon: "DAY".to_string(),
        ready: true,
        tags: serde_json::json!({
            "near_npoc": true,
            "in_golden_pocket": true,
            "structure": "W2"
        }),
        confidence: Some(95.0),
        direction: Some("BUY".to_string()),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let timeframes = vec![
        TimeframeAnalysis {
            timeframe: "1m".to_string(),
            trend: "BULLISH".to_string(),
            strength: 85.0,
        },
        TimeframeAnalysis {
            timeframe: "5m".to_string(),
            trend: "BULLISH".to_string(),
            strength: 90.0,
        },
        TimeframeAnalysis {
            timeframe: "1h".to_string(),
            trend: "BULLISH".to_string(),
            strength: 95.0,
        },
    ];

    let indicators = IndicatorMetrics {
        leading: LeadingIndicators {
            rsi: Some(65.0),
            stochastic: Some(75.0),
            macd_signal: Some("BUY".to_string()),
        },
        lagging: LaggingIndicators {
            ma_50: Some(49500.0),
            ma_200: Some(48000.0),
            ema_21: Some(49800.0),
        },
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(SignalWithMetrics {
            signal,
            timeframes,
            indicators,
        }),
        error: None,
    }))
}
