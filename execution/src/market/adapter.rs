use async_trait::async_trait;
use crate::market::candle::Candle;

pub enum MarketEvent {
    Candle(Candle),
}

#[async_trait]
pub trait MarketAdapter: Send {
    fn name(&self) -> &'static str;
    async fn next_event(&mut self) -> Option<MarketEvent>;
}
