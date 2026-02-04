use async_trait::async_trait;
use crate::market::{Candle, MarketAdapter, MarketEvent, MarketKind};
use crate::scanner::timeframe::Timeframe;

pub struct StockMockAdapter {
    pub symbol: String,
    pub tf: Timeframe,
    px: f64,
}

impl StockMockAdapter {
    pub fn new(symbol: impl Into<String>, tf: Timeframe, start_px: f64) -> Self {
        Self { symbol: symbol.into(), tf, px: start_px }
    }
}

fn now_unix_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}

#[async_trait]
impl MarketAdapter for StockMockAdapter {
    fn name(&self) -> &'static str { "stock-mock" }

    async fn next_event(&mut self) -> Option<MarketEvent> {
        self.px = self.px + ((self.px * 3.0).sin() * 0.05);

        let o = self.px;
        let h = o * 1.001;
        let l = o * 0.999;
        let c = o * 1.0002;

        Some(MarketEvent::Candle(Candle {
            market: MarketKind::Stock,
            symbol: self.symbol.clone(),
            tf: self.tf,
            ts_open_unix_ms: now_unix_ms(),
            open: o,
            high: h,
            low: l,
            close: c,
            volume: 1_200_000.0,
        }))
    }
}
