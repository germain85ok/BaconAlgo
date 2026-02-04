mod bus;
mod api;
mod scanner;
mod market;
mod news;
mod smc;
mod signal;

use axum::{routing::get, Router};
use std::{net::SocketAddr, time::Duration};

use api::{routes::sse_signals, LiveSignal};
use bus::SignalBus;
use market::{MarketAdapter, MarketEvent, binance::BinanceAdapter, candle::Candle};
use signal::{Signal, Horizon, Direction, SignalScorer};

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
        let mut scorer = SignalScorer::new();
        let mut candle_history: Vec<Candle> = Vec::new();
        let mut last_signal_score: Option<u8> = None;

        loop {
            if let Some(MarketEvent::Candle(c)) = adapter.next_event().await {
                // Add to history
                candle_history.push(c.clone());
                if candle_history.len() > 100 {
                    candle_history.remove(0);
                }

                // Generate signal every few candles
                if candle_history.len() >= 20 {
                    let mut signal = Signal::new(
                        c.symbol.clone(),
                        Horizon::Day,
                        if c.close > c.open { Direction::Long } else { Direction::Short }
                    );

                    // Set basic entry/exit levels
                    signal.entry = c.close;
                    signal.stop_loss = if signal.direction == Direction::Long {
                        c.close * 0.98
                    } else {
                        c.close * 1.02
                    };
                    
                    // Score the signal using SMC analysis
                    scorer.score_signal(&mut signal, &candle_history, c.close);
                    
                    // Generate targets
                    signal.targets = scorer.generate_targets(
                        signal.entry,
                        signal.stop_loss,
                        &signal.direction
                    );
                    signal.calculate_risk_reward();
                    
                    // Add whale score
                    let (whale_score, whale_bars) = scorer.calculate_whale_score(&candle_history);
                    signal.whale_score = whale_score;
                    signal.whale_bars = whale_bars;

                    // Only publish if score changed significantly or is high
                    if signal.score >= 50 {
                        if last_signal_score.map(|prev| (signal.score as i16 - prev as i16).abs() >= 10).unwrap_or(true) {
                            let score = signal.score;
                            bus_clone.publish(signal.into());
                            last_signal_score = Some(score);
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    let app = Router::new()
        .route("/signals/live", get(sse_signals))
        .with_state(bus);

    let addr = SocketAddr::from(([0,0,0,0], 3000));
    println!("🥓 BaconAlgo 2030 LIVE @ http://{addr}/signals/live");

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
