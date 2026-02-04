use async_trait::async_trait;
use crate::market::{adapter::MarketAdapter, candle::Candle};
use crate::scanner::timeframe::Timeframe;

pub struct CryptoAdapter {
    pub symbol: String,
}

#[async_trait]
impl MarketAdapter for CryptoAdapter {
    async fn next_candle(&mut self) -> Option<Candle> {
        // MOCK — remplacé par Binance / Coinbase / Polygon
        Some(Candle {
            symbol: self.symbol.clone(),
            tf: Timeframe::M15,
            ts_open: chrono::Utc::now().timestamp(),
            open: 100.0,
            high: 105.0,
            low: 98.0,
            close: 102.0,
            volume: 1234.0,
        })
    }
}
