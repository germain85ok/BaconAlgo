use crate::scanner::timeframe::Timeframe;

#[derive(Clone)]
pub struct Candle {
    pub symbol: String,
    pub tf: Timeframe,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}
