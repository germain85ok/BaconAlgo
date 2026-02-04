use async_trait::async_trait;
use crate::market::{Candle, MarketAdapter, MarketEvent, MarketKind};
use crate::scanner::timeframe::Timeframe;

pub struct FutureMockAdapter {
    pub symbol: String,
    pub tf: Timeframe,
    px: f64,
}

impl FutureMockAdapter {
    pub fn new(symbol: impl Into<String>, tf: Timeframe, start_px: f64) -> Self {
        Self { symbol: symbol.into(), tf, px: start_px }
    }
}

fn now_unix_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}

#[async_trait]
impl MarketAdapter for FutureMockAdapter {
    fn name(&self) -> &'static str { "future-mock" }

    async fn next_event(&mut self) -> Option<MarketEvent> {
        self.px = self.px + ((self.px * 10.0).cos() * 0.25);

        let o = self.px;
        let h = o + 1.25;
        let l = o - 1.00;
        let c = o + 0.50;

        Some(MarketEvent::Candle(Candle {
            market: MarketKind::Future,
            symbol: self.symbol.clone(),
            tf: self.tf,
            ts_open_unix_ms: now_unix_ms(),
            open: o,
            high: h,
            low: l,
            close: c,
            volume: 9000.0,
        }))
    }
}
