/// Fair Value Gap (FVG) Detection
/// 
/// A Fair Value Gap occurs when there's a gap between candles that represents
/// inefficient price discovery. These gaps often act as magnets for price to return.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FvgType {
    Bullish,
    Bearish,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairValueGap {
    pub fvg_type: FvgType,
    pub top: f64,
    pub bottom: f64,
    pub timestamp: i64,
    pub mitigated: bool,
    pub mitigation_timestamp: Option<i64>,
}

impl FairValueGap {
    /// Create a new FVG
    pub fn new(fvg_type: FvgType, top: f64, bottom: f64, timestamp: i64) -> Self {
        Self {
            fvg_type,
            top,
            bottom,
            timestamp,
            mitigated: false,
            mitigation_timestamp: None,
        }
    }

    /// Check if price has mitigated (filled) this FVG
    pub fn check_mitigation(&mut self, price: f64, timestamp: i64) -> bool {
        if self.mitigated {
            return true;
        }

        let is_mitigated = match self.fvg_type {
            FvgType::Bullish => price <= self.bottom,
            FvgType::Bearish => price >= self.top,
        };

        if is_mitigated {
            self.mitigated = true;
            self.mitigation_timestamp = Some(timestamp);
        }

        is_mitigated
    }

    /// Check if price is near this FVG (within 0.5% distance)
    pub fn is_near(&self, price: f64) -> bool {
        let mid = (self.top + self.bottom) / 2.0;
        let distance_pct = ((price - mid).abs() / mid) * 100.0;
        distance_pct < 0.5
    }

    /// Get the size of the gap
    pub fn size(&self) -> f64 {
        self.top - self.bottom
    }
}

/// Detect Fair Value Gaps from candle data
pub struct FvgDetector {
    pub min_gap_pct: f64, // Minimum gap size as percentage
}

impl FvgDetector {
    pub fn new(min_gap_pct: f64) -> Self {
        Self { min_gap_pct }
    }

    /// Detect FVG from three consecutive candles
    /// candles: [oldest, middle, newest]
    pub fn detect(
        &self,
        candles: &[(f64, f64, f64, f64, i64)], // (open, high, low, close, timestamp)
    ) -> Vec<FairValueGap> {
        if candles.len() < 3 {
            return vec![];
        }

        let mut fvgs = Vec::new();

        for i in 0..candles.len() - 2 {
            let (_, high1, low1, _, _) = candles[i];
            let (_, _, _, _, _) = candles[i + 1]; // middle candle
            let (_, high3, low3, _, ts3) = candles[i + 2];

            // Bullish FVG: gap between candle 1's high and candle 3's low
            if low3 > high1 {
                let gap_size = low3 - high1;
                let gap_pct = (gap_size / high1) * 100.0;

                if gap_pct >= self.min_gap_pct {
                    fvgs.push(FairValueGap::new(FvgType::Bullish, low3, high1, ts3));
                }
            }
            // Bearish FVG: gap between candle 1's low and candle 3's high
            else if high3 < low1 {
                let gap_size = low1 - high3;
                let gap_pct = (gap_size / low1) * 100.0;

                if gap_pct >= self.min_gap_pct {
                    fvgs.push(FairValueGap::new(FvgType::Bearish, low1, high3, ts3));
                }
            }
        }

        fvgs
    }
}

impl Default for FvgDetector {
    fn default() -> Self {
        Self::new(0.1) // 0.1% minimum gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullish_fvg_detection() {
        let detector = FvgDetector::default();
        
        // Create candles with a bullish FVG
        let candles = vec![
            (100.0, 105.0, 95.0, 102.0, 1000),  // Candle 1
            (102.0, 108.0, 101.0, 107.0, 2000), // Candle 2
            (110.0, 115.0, 108.0, 113.0, 3000), // Candle 3 - gap from 105 to 108
        ];

        let fvgs = detector.detect(&candles);
        assert_eq!(fvgs.len(), 1);
        assert_eq!(fvgs[0].fvg_type, FvgType::Bullish);
        assert_eq!(fvgs[0].top, 108.0);
        assert_eq!(fvgs[0].bottom, 105.0);
    }

    #[test]
    fn test_bearish_fvg_detection() {
        let detector = FvgDetector::default();
        
        let candles = vec![
            (100.0, 105.0, 95.0, 98.0, 1000),   // Candle 1
            (98.0, 99.0, 92.0, 93.0, 2000),     // Candle 2
            (90.0, 92.0, 85.0, 88.0, 3000),     // Candle 3 - gap from 92 to 95
        ];

        let fvgs = detector.detect(&candles);
        assert_eq!(fvgs.len(), 1);
        assert_eq!(fvgs[0].fvg_type, FvgType::Bearish);
    }

    #[test]
    fn test_fvg_mitigation() {
        let mut fvg = FairValueGap::new(FvgType::Bullish, 108.0, 105.0, 3000);
        
        assert!(!fvg.mitigated);
        assert!(!fvg.check_mitigation(109.0, 4000)); // Above gap
        assert!(fvg.check_mitigation(104.0, 5000));  // Below gap - mitigated
        assert!(fvg.mitigated);
    }

    #[test]
    fn test_is_near() {
        let fvg = FairValueGap::new(FvgType::Bullish, 108.0, 105.0, 3000);
        
        assert!(fvg.is_near(106.5));  // Mid point
        assert!(fvg.is_near(107.0));  // Close to mid
        assert!(!fvg.is_near(120.0)); // Far away
    }
}
