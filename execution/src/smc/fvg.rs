use crate::market::candle::Candle;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Fair Value Gap - represents an imbalance in price
#[derive(Debug, Clone, Serialize)]
pub struct FVG {
    pub is_bullish: bool,
    pub top: f64,
    pub bottom: f64,
    pub midpoint: f64,
    pub created_at: DateTime<Utc>,
    pub is_filled: bool,
}

impl FVG {
    pub fn new(is_bullish: bool, top: f64, bottom: f64) -> Self {
        Self {
            is_bullish,
            top,
            bottom,
            midpoint: (top + bottom) / 2.0,
            created_at: Utc::now(),
            is_filled: false,
        }
    }

    pub fn check_filled(&mut self, current_price: f64) {
        if self.is_bullish {
            // Bullish FVG filled when price comes back down through midpoint
            if current_price <= self.midpoint {
                self.is_filled = true;
            }
        } else {
            // Bearish FVG filled when price comes back up through midpoint
            if current_price >= self.midpoint {
                self.is_filled = true;
            }
        }
    }
}

pub struct FVGDetector;

impl FVGDetector {
    /// Detect FVG from three consecutive candles
    /// Bullish FVG: candle[i-1].high < candle[i+1].low
    /// Bearish FVG: candle[i-1].low > candle[i+1].high
    pub fn detect(candles: &[Candle]) -> Vec<FVG> {
        let mut fvgs = Vec::new();
        
        if candles.len() < 3 {
            return fvgs;
        }

        for i in 1..candles.len() - 1 {
            let prev = &candles[i - 1];
            let _curr = &candles[i];
            let next = &candles[i + 1];

            // Bullish FVG: gap between prev high and next low
            if prev.high < next.low {
                fvgs.push(FVG::new(true, next.low, prev.high));
            }
            // Bearish FVG: gap between prev low and next high
            else if prev.low > next.high {
                fvgs.push(FVG::new(false, prev.low, next.high));
            }
        }

        fvgs
    }

    /// Check if current price is near an unfilled FVG
    pub fn is_near_fvg(fvgs: &[FVG], price: f64, tolerance: f64) -> bool {
        fvgs.iter().any(|fvg| {
            !fvg.is_filled && 
            price >= fvg.bottom - tolerance &&
            price <= fvg.top + tolerance
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::timeframe::Timeframe;

    #[test]
    fn test_bullish_fvg_detection() {
        let candles = vec![
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 105.0, low: 99.0, close: 104.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 104.0, high: 110.0, low: 103.0, close: 109.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 109.0, high: 115.0, low: 108.0, close: 114.0 },
        ];

        let fvgs = FVGDetector::detect(&candles);
        assert_eq!(fvgs.len(), 1);
        assert!(fvgs[0].is_bullish);
        assert_eq!(fvgs[0].bottom, 105.0);
        assert_eq!(fvgs[0].top, 108.0);
    }

    #[test]
    fn test_bearish_fvg_detection() {
        let candles = vec![
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 105.0, low: 99.0, close: 100.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 99.0, low: 90.0, close: 91.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 91.0, high: 92.0, low: 85.0, close: 86.0 },
        ];

        let fvgs = FVGDetector::detect(&candles);
        assert_eq!(fvgs.len(), 1);
        assert!(!fvgs[0].is_bullish);
        assert_eq!(fvgs[0].top, 99.0);
        assert_eq!(fvgs[0].bottom, 92.0);
    }
}
