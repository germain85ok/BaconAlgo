use axum::{
    extract::State,
    response::{sse::Event, IntoResponse, Json, Sse},
    http::StatusCode,
};
use serde_json::json;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use std::convert::Infallible;

use crate::bus::SignalBus;
use crate::api::models::LiveSignal;

/// GET /api/signals - Get latest signals
pub async fn get_signals(
    State(_bus): State<SignalBus<LiveSignal>>,
) -> impl IntoResponse {
    // For now, return a placeholder - in production this would query from database
    let signals = vec![
        LiveSignal {
            symbol: "BTCUSDT".to_string(),
            horizon: "M15".to_string(),
            ready: true,
            tags: json!({"close": 45000.0}),
            reason: "Golden pocket alignment".to_string(),
            ts_unix_ms: chrono::Utc::now().timestamp_millis(),
        },
    ];
    
    (StatusCode::OK, Json(json!({
        "signals": signals,
        "count": signals.len(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
}

/// GET /api/signals/stream - SSE stream of signals
pub async fn stream_signals(
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
