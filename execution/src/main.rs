mod bus;
mod api;
mod scanner;
mod market;
mod news;

use axum::{routing::{get, post}, Router};
use std::{net::SocketAddr, time::Duration};

use api::routes::*;
use api::LiveSignal;
use bus::SignalBus;
use market::{MarketAdapter, MarketEvent, binance::BinanceAdapter};
use scanner::{context::Context, engine::ScannerEngine, timeframe::Timeframe};

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}

#[tokio::main]
async fn main() {
    let bus = SignalBus::<LiveSignal>::new(256);

    let bus_clone = bus.clone();
    tokio::spawn(async move {
        let mut adapter = BinanceAdapter::new("btcusdt");
        let mut engine = ScannerEngine::new();
        let mut last_ready: Option<bool> = None;

        loop {
            if let Some(MarketEvent::Candle(c)) = adapter.next_event().await {
                engine.update(Context {
                    tf: Timeframe::M15,
                    near_npoc: c.close >= c.open,
                    in_golden_pocket: true,
                    structure_ok: true,
                });

                let ready = engine.ready();
                if last_ready.map(|p| p != ready).unwrap_or(true) {
                    bus_clone.publish(LiveSignal {
                        symbol: c.symbol,
                        horizon: "M15".into(),
                        ready,
                        tags: serde_json::json!({ "close": c.close }),
                        reason: if ready { "READY".into() } else { "NOT READY".into() },
                        ts_unix_ms: now_ms(),
                    });
                    last_ready = Some(ready);
                }
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    });

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/scan/:symbol", get(scan_symbol))
        .route("/api/scan/all", get(scan_all))
        .route("/api/signals/live", get(sse_signals))
        .route("/api/rockets/top10", get(rockets_top10))
        .route("/api/market/sentiment", get(market_sentiment))
        .route("/api/auth/verify-promo", post(verify_promo))
        .with_state(bus);

    let addr = SocketAddr::from(([0,0,0,0], 3000));
    println!("🔥 BaconAlgo SAAS API @ http://{addr}");
    println!("   /api/health");
    println!("   /api/scan/:symbol");
    println!("   /api/scan/all?type=scalp|day|swing|long&min_score=60");
    println!("   /api/signals/live (SSE)");
    println!("   /api/rockets/top10");
    println!("   /api/market/sentiment");
    println!("   /api/auth/verify-promo (POST)");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
