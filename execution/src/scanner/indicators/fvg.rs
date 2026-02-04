/// Fair Value Gap (FVG) / Imbalance detection
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairValueGap {
    pub gap_type: FVGType,
    pub top: f64,
    pub bottom: f64,
    pub timestamp: i64,
    pub filled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FVGType {
    Bullish,  // Gap up - potential support
    Bearish,  // Gap down - potential resistance
}

#[derive(Debug, Clone)]
pub struct Candle {
    pub high: f64,
    pub low: f64,
    pub timestamp: i64,
}

impl FairValueGap {
    /// Detect FVGs from three consecutive candles
    /// FVG exists when candle 1's high < candle 3's low (bullish)
    /// or candle 1's low > candle 3's high (bearish)
    pub fn detect(candles: &[Candle]) -> Vec<FairValueGap> {
        if candles.len() < 3 {
            return Vec::new();
        }
        
        let mut gaps = Vec::new();
        
        for i in 0..candles.len() - 2 {
            let c1 = &candles[i];
            let c2 = &candles[i + 1];
            let c3 = &candles[i + 2];
            
            // Bullish FVG: gap between c1.high and c3.low
            if c1.high < c3.low {
                gaps.push(FairValueGap {
                    gap_type: FVGType::Bullish,
                    top: c3.low,
                    bottom: c1.high,
                    timestamp: c2.timestamp,
                    filled: false,
                });
            }
            
            // Bearish FVG: gap between c3.high and c1.low
            if c1.low > c3.high {
                gaps.push(FairValueGap {
                    gap_type: FVGType::Bearish,
                    top: c1.low,
                    bottom: c3.high,
                    timestamp: c2.timestamp,
                    filled: false,
                });
            }
        }
        
        gaps
    }
    
    /// Check if FVG has been filled by price
    pub fn check_filled(&mut self, price: f64) {
        match self.gap_type {
            FVGType::Bullish => {
                if price <= self.bottom {
                    self.filled = true;
                }
            },
            FVGType::Bearish => {
                if price >= self.top {
                    self.filled = true;
                }
            }
        }
    }
    
    /// Check if price is within the gap
    pub fn is_price_in_gap(&self, price: f64) -> bool {
        price >= self.bottom && price <= self.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullish_fvg_detection() {
        let candles = vec![
            Candle { high: 100.0, low: 95.0, timestamp: 1 },
            Candle { high: 110.0, low: 105.0, timestamp: 2 },
            Candle { high: 115.0, low: 103.0, timestamp: 3 },
        ];
        
        let gaps = FairValueGap::detect(&candles);
        assert!(!gaps.is_empty());
        assert_eq!(gaps[0].gap_type, FVGType::Bullish);
        assert_eq!(gaps[0].top, 103.0);
        assert_eq!(gaps[0].bottom, 100.0);
    }

    #[test]
    fn test_bearish_fvg_detection() {
        let candles = vec![
            Candle { high: 100.0, low: 95.0, timestamp: 1 },
            Candle { high: 90.0, low: 85.0, timestamp: 2 },
            Candle { high: 93.0, low: 88.0, timestamp: 3 },
        ];
        
        let gaps = FairValueGap::detect(&candles);
        assert!(!gaps.is_empty());
        assert_eq!(gaps[0].gap_type, FVGType::Bearish);
    }

    #[test]
    fn test_fvg_filled() {
        let mut gap = FairValueGap {
            gap_type: FVGType::Bullish,
            top: 103.0,
            bottom: 100.0,
            timestamp: 1,
            filled: false,
        };
        
        gap.check_filled(99.0);
        assert!(gap.filled);
    }
}
