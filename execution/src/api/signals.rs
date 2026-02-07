use axum::{
    extract::State,
    response::{sse::Event, IntoResponse, Json, Sse},
    http::StatusCode,
};
use serde_json::json;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::bus::SignalBus;
use crate::api::models::LiveSignal;

/// GET /api/signals - Get latest signals
pub async fn get_signals(
    State(signal_buffer): State<Arc<RwLock<Vec<LiveSignal>>>>,
) -> impl IntoResponse {
    // Read from the shared signal buffer
    let signals = signal_buffer.read().await;
    let signal_list: Vec<LiveSignal> = signals.clone();
    
    (StatusCode::OK, Json(json!({
        "signals": signal_list,
        "count": signal_list.len(),
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
