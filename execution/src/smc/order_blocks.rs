use crate::market::candle::Candle;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Order Block - last opposing candle before strong impulse move
#[derive(Debug, Clone, Serialize)]
pub struct OrderBlock {
    pub is_bullish: bool,
    pub top: f64,
    pub bottom: f64,
    pub created_at: DateTime<Utc>,
    pub is_mitigated: bool,
    pub is_premium: bool, // Above 50% of range = premium, below = discount
}

impl OrderBlock {
    pub fn new(is_bullish: bool, top: f64, bottom: f64) -> Self {
        Self {
            is_bullish,
            top,
            bottom,
            created_at: Utc::now(),
            is_mitigated: false,
            is_premium: false,
        }
    }

    pub fn check_mitigation(&mut self, current_price: f64) {
        if self.is_bullish {
            // Bullish OB mitigated when price returns to it
            if current_price <= self.top && current_price >= self.bottom {
                self.is_mitigated = true;
            }
        } else {
            // Bearish OB mitigated when price returns to it
            if current_price >= self.bottom && current_price <= self.top {
                self.is_mitigated = true;
            }
        }
    }

    pub fn set_premium_discount(&mut self, range_high: f64, range_low: f64) {
        let mid = (range_high + range_low) / 2.0;
        self.is_premium = self.bottom > mid;
    }
}

pub struct OrderBlockDetector;

impl OrderBlockDetector {
    /// Detect Order Blocks from candle data
    /// Bullish OB: Last down candle before impulsive up move
    /// Bearish OB: Last up candle before impulsive down move
    pub fn detect(candles: &[Candle], min_impulse: f64) -> Vec<OrderBlock> {
        let mut obs = Vec::new();
        
        if candles.len() < 3 {
            return obs;
        }

        for i in 1..candles.len() - 1 {
            let _prev = &candles[i - 1];
            let curr = &candles[i];
            let next = &candles[i + 1];

            // Bullish OB: down candle followed by strong up move
            if curr.close < curr.open {
                let impulse = next.close - curr.low;
                let body = curr.open - curr.close;
                if impulse > min_impulse && impulse > body * 2.0 {
                    obs.push(OrderBlock::new(true, curr.open, curr.low));
                }
            }
            // Bearish OB: up candle followed by strong down move
            else if curr.close > curr.open {
                let impulse = curr.high - next.close;
                let body = curr.close - curr.open;
                if impulse > min_impulse && impulse > body * 2.0 {
                    obs.push(OrderBlock::new(false, curr.high, curr.open));
                }
            }
        }

        obs
    }

    /// Check if current price is near an unmitigated OB
    pub fn is_near_ob(obs: &[OrderBlock], price: f64, tolerance: f64) -> bool {
        obs.iter().any(|ob| {
            !ob.is_mitigated && 
            price >= ob.bottom - tolerance &&
            price <= ob.top + tolerance
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::timeframe::Timeframe;

    #[test]
    fn test_bullish_ob_detection() {
        let candles = vec![
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 102.0, low: 99.0, close: 100.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 101.0, low: 95.0, close: 96.0 }, // Down candle
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 96.0, high: 110.0, low: 95.0, close: 109.0 }, // Strong impulse up
        ];

        let obs = OrderBlockDetector::detect(&candles, 5.0);
        assert_eq!(obs.len(), 1);
        assert!(obs[0].is_bullish);
        assert_eq!(obs[0].top, 100.0);
        assert_eq!(obs[0].bottom, 95.0);
    }

    #[test]
    fn test_bearish_ob_detection() {
        let candles = vec![
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 102.0, low: 99.0, close: 100.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 105.0, low: 99.0, close: 104.0 }, // Up candle
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 104.0, high: 105.0, low: 90.0, close: 91.0 }, // Strong impulse down
        ];

        let obs = OrderBlockDetector::detect(&candles, 5.0);
        assert_eq!(obs.len(), 1);
        assert!(!obs[0].is_bullish);
        assert_eq!(obs[0].top, 105.0);
        assert_eq!(obs[0].bottom, 100.0);
    }
}
