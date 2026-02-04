use async_trait::async_trait;
use crate::market::{adapter::MarketAdapter, candle::Candle};
use crate::scanner::timeframe::Timeframe;

pub struct StockAdapter {
    pub symbol: String,
}

#[async_trait]
impl MarketAdapter for StockAdapter {
    async fn next_candle(&mut self) -> Option<Candle> {
        Some(Candle {
            symbol: self.symbol.clone(),
            tf: Timeframe::M15,
            ts_open: chrono::Utc::now().timestamp(),
            open: 190.0,
            high: 192.0,
            low: 189.5,
            close: 191.2,
            volume: 1_200_000.0,
        })
    }
}
