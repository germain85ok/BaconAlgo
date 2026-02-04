use async_trait::async_trait;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::market::{adapter::{MarketAdapter, MarketEvent}, candle::Candle};
use crate::scanner::timeframe::Timeframe;

pub struct BinanceAdapter {
    symbol: String,
}

impl BinanceAdapter {
    pub fn new(symbol: &str) -> Self {
        Self { symbol: symbol.to_string() }
    }
}

#[async_trait]
impl MarketAdapter for BinanceAdapter {
    fn name(&self) -> &'static str { "binance" }

    async fn next_event(&mut self) -> Option<MarketEvent> {
        let url = format!(
            "wss://stream.binance.com:9443/ws/{}@kline_15m",
            self.symbol.to_lowercase()
        );

        let (ws, _) = connect_async(Url::parse(&url).ok()?).await.ok()?;
        let (_, mut read) = ws.split();

        while let Some(msg) = read.next().await {
            let txt = msg.ok()?.into_text().ok()?;
            let v: serde_json::Value = serde_json::from_str(&txt).ok()?;
            let k = &v["k"];

            if !k["x"].as_bool()? { continue; }

            return Some(MarketEvent::Candle(Candle {
                symbol: self.symbol.clone(),
                tf: Timeframe::M15,
                open: k["o"].as_str()?.parse().ok()?,
                high: k["h"].as_str()?.parse().ok()?,
                low:  k["l"].as_str()?.parse().ok()?,
                close:k["c"].as_str()?.parse().ok()?,
            }));
        }
        None
    }
}
