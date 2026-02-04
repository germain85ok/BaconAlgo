use async_trait::async_trait;
use crate::market::{Candle, MarketAdapter, MarketEvent, MarketKind};
use crate::scanner::timeframe::Timeframe;

pub struct CryptoMockAdapter {
    pub symbol: String,
    pub tf: Timeframe,
    px: f64,
}

impl CryptoMockAdapter {
    pub fn new(symbol: impl Into<String>, tf: Timeframe, start_px: f64) -> Self {
        Self { symbol: symbol.into(), tf, px: start_px }
    }
}

fn now_unix_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}

#[async_trait]
impl MarketAdapter for CryptoMockAdapter {
    fn name(&self) -> &'static str { "crypto-mock" }

    async fn next_event(&mut self) -> Option<MarketEvent> {
        // Petit random-walk déterministe (sans rand crate)
        self.px = (self.px * 100.0 + 7.0).sin().abs() * 10.0 + self.px * 0.995;

        let o = self.px;
        let h = o * 1.002;
        let l = o * 0.998;
        let c = o * 1.0005;

        Some(MarketEvent::Candle(Candle {
            market: MarketKind::Crypto,
            symbol: self.symbol.clone(),
            tf: self.tf,
            ts_open_unix_ms: now_unix_ms(),
            open: o,
            high: h,
            low: l,
            close: c,
            volume: 1234.0,
        }))
    }
}
