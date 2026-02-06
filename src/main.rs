mod bus;
mod api;

use axum::{Router, routing::get};
use axum::routing::post;
use tower_http::cors::{CorsLayer, Any};
use bus::SignalBus;
use api::models::LiveSignal;
use api::routes::*;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let bus = SignalBus::<LiveSignal>::new(128);

    // 🔥 SIMULATION LIVE (remplacée par le scanner réel)
    let bus_clone = bus.clone();
    tokio::spawn(async move {
        loop {
            let signal = LiveSignal {
                id: Uuid::new_v4().to_string(),
                symbol: "BTC".into(),
                horizon: "DAY".into(),
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
            bus_clone.publish(signal);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    // Configure CORS for frontend communication
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // WebSocket routes
        .route("/ws/signals", get(ws_signals))
        // SSE routes
        .route("/signals/live", get(sse_signals))
        // REST API routes
        .route("/api/quotes", get(get_market_quotes))
        .route("/api/historical/:symbol", get(get_historical_data))
        .route("/api/portfolio", get(get_portfolio))
        .route("/api/preferences", get(get_user_preferences))
        .route("/api/preferences", post(update_user_preferences))
        .route("/api/signals/:id", get(get_signal_details))
        .layer(cors)
        .with_state(bus);

    let addr = "0.0.0.0:3000";
    println!("🥓 BaconAlgo 2040 Backend Server");
    println!("================================");
    println!("🔥 SSE Signals:      http://localhost:3000/signals/live");
    println!("🌐 WebSocket:        ws://localhost:3000/ws/signals");
    println!("📊 Market Quotes:    http://localhost:3000/api/quotes");
    println!("📈 Historical Data:  http://localhost:3000/api/historical/:symbol");
    println!("💼 Portfolio:        http://localhost:3000/api/portfolio");
    println!("⚙️  Preferences:      http://localhost:3000/api/preferences");
    println!("🎯 Signal Details:   http://localhost:3000/api/signals/:id");
    println!("================================");
    println!("Server running on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
