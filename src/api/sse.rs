use axum::{
    response::sse::{Event, Sse},
    response::IntoResponse,
};
use futures::stream::{self, Stream};
use serde_json::json;
use std::{convert::Infallible, time::Duration};
use tokio::time;

pub async fn sse_handler() -> impl IntoResponse {
    let stream = stream::unfold((), |_| async {
        time::sleep(Duration::from_secs(5)).await;

        let signal = json!({
            "type": "SIGNAL",
            "data": {
                "id": format!("SIG-{}", chrono::Utc::now().timestamp()),
                "symbol": "BTCUSD",
                "direction": "LONG",
                "confidence": 75.5,
                "entry_price": 45000.0,
                "stop_loss": 44500.0,
                "take_profit": 46000.0,
                "timeframe": "1h",
                "timestamp": chrono::Utc::now().timestamp(),
                "neural_score": 82.0,
                "risk_reward": 2.0,
                "status": "ACTIVE"
            },
            "timestamp": chrono::Utc::now().timestamp()
        });

        let event = Event::default().data(signal.to_string());
        Some((Ok::<_, Infallible>(event), ()))
    });

    Sse::new(stream)
}
