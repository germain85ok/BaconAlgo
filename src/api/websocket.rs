use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::time::Duration;
use tokio::time;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            
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

            if sender.send(Message::Text(signal.to_string())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                println!("[WS] Received: {}", text);
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
