mod bus;
mod api;

use axum::{Router, routing::get};
use bus::SignalBus;
use api::models::LiveSignal;
use api::routes::sse_signals;
use uuid::Uuid;

#[tokio::main]
async fn main() {
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
            };
            bus_clone.publish(signal);
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    let app = Router::new()
        .route("/signals/live", get(sse_signals))
        .with_state(bus);

    println!("🔥 LIVE SIGNAL SERVER @ http://localhost:3000/signals/live");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
