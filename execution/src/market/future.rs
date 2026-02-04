use async_trait::async_trait;
use crate::market::{adapter::MarketAdapter, candle::Candle};
use crate::scanner::timeframe::Timeframe;

pub struct FutureAdapter {
    pub symbol: String,
}

#[async_trait]
impl MarketAdapter for FutureAdapter {
    async fn next_candle(&mut self) -> Option<Candle> {
        Some(Candle {
            symbol: self.symbol.clone(),
            tf: Timeframe::M15,
            ts_open: chrono::Utc::now().timestamp(),
            open: 4800.0,
            high: 4820.0,
            low: 4785.0,
            close: 4810.0,
            volume: 9000.0,
        })
    }
}
