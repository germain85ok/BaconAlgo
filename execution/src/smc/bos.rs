/// Break of Structure (BOS) Detection
/// 
/// BOS occurs when price breaks above a previous swing high (bullish BOS)
/// or below a previous swing low (bearish BOS), indicating trend continuation
/// or reversal.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BosType {
    Bullish,  // Break above previous high
    Bearish,  // Break below previous low
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakOfStructure {
    pub bos_type: BosType,
    pub break_level: f64,
    pub timestamp: i64,
    pub strength: f64, // How far beyond the level (as percentage)
}

impl BreakOfStructure {
    pub fn new(bos_type: BosType, break_level: f64, timestamp: i64, strength: f64) -> Self {
        Self {
            bos_type,
            break_level,
            timestamp,
            strength: strength.min(1.0).max(0.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwingPoint {
    pub price: f64,
    pub timestamp: i64,
    pub is_high: bool,
}

/// Detect Break of Structure from price data
pub struct BosDetector {
    pub lookback_period: usize,
    pub min_break_pct: f64, // Minimum percentage break to qualify
}

impl BosDetector {
    pub fn new(lookback_period: usize, min_break_pct: f64) -> Self {
        Self {
            lookback_period,
            min_break_pct,
        }
    }

    /// Identify swing highs and lows
    pub fn find_swing_points(
        &self,
        candles: &[(f64, f64, f64, f64, i64)], // (open, high, low, close, timestamp)
    ) -> Vec<SwingPoint> {
        if candles.len() < 3 {
            return vec![];
        }

        let mut swings = Vec::new();

        for i in 1..candles.len() - 1 {
            let (_, high_prev, low_prev, _, _) = candles[i - 1];
            let (_, high, low, _, ts) = candles[i];
            let (_, high_next, low_next, _, _) = candles[i + 1];

            // Swing high: higher than neighbors
            if high > high_prev && high > high_next {
                swings.push(SwingPoint {
                    price: high,
                    timestamp: ts,
                    is_high: true,
                });
            }

            // Swing low: lower than neighbors
            if low < low_prev && low < low_next {
                swings.push(SwingPoint {
                    price: low,
                    timestamp: ts,
                    is_high: false,
                });
            }
        }

        swings
    }

    /// Detect break of structure events
    pub fn detect(
        &self,
        candles: &[(f64, f64, f64, f64, i64)], // (open, high, low, close, timestamp)
    ) -> Vec<BreakOfStructure> {
        let swings = self.find_swing_points(candles);
        if swings.is_empty() || candles.is_empty() {
            return vec![];
        }

        let mut bos_events = Vec::new();
        let (_, current_high, current_low, current_close, current_ts) = candles[candles.len() - 1];

        // Look for recent swing highs and lows
        let recent_highs: Vec<f64> = swings
            .iter()
            .filter(|s| s.is_high)
            .rev()
            .take(self.lookback_period)
            .map(|s| s.price)
            .collect();

        let recent_lows: Vec<f64> = swings
            .iter()
            .filter(|s| !s.is_high)
            .rev()
            .take(self.lookback_period)
            .map(|s| s.price)
            .collect();

        // Check for bullish BOS (break above previous high)
        if let Some(&prev_high) = recent_highs.first() {
            if current_high > prev_high {
                let break_pct = ((current_high - prev_high) / prev_high) * 100.0;
                if break_pct >= self.min_break_pct {
                    let strength = (break_pct / (self.min_break_pct * 3.0)).min(1.0);
                    bos_events.push(BreakOfStructure::new(
                        BosType::Bullish,
                        prev_high,
                        current_ts,
                        strength,
                    ));
                }
            }
        }

        // Check for bearish BOS (break below previous low)
        if let Some(&prev_low) = recent_lows.first() {
            if current_low < prev_low {
                let break_pct = ((prev_low - current_low) / prev_low) * 100.0;
                if break_pct >= self.min_break_pct {
                    let strength = (break_pct / (self.min_break_pct * 3.0)).min(1.0);
                    bos_events.push(BreakOfStructure::new(
                        BosType::Bearish,
                        prev_low,
                        current_ts,
                        strength,
                    ));
                }
            }
        }

        bos_events
    }

    /// Check if current price shows a BOS
    pub fn is_bos_confirmed(&self, candles: &[(f64, f64, f64, f64, i64)]) -> bool {
        !self.detect(candles).is_empty()
    }
}

impl Default for BosDetector {
    fn default() -> Self {
        Self::new(5, 0.2) // Look back 5 swings, 0.2% minimum break
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swing_point_detection() {
        let detector = BosDetector::default();

        let candles = vec![
            (100.0, 102.0, 98.0, 101.0, 1000),
            (101.0, 105.0, 100.0, 104.0, 2000), // Swing high at 105
            (104.0, 103.0, 99.0, 100.0, 3000),
            (100.0, 102.0, 96.0, 97.0, 4000),   // Swing low at 96
            (97.0, 101.0, 96.5, 100.0, 5000),
        ];

        let swings = detector.find_swing_points(&candles);
        assert!(!swings.is_empty());

        let highs: Vec<_> = swings.iter().filter(|s| s.is_high).collect();
        let lows: Vec<_> = swings.iter().filter(|s| !s.is_high).collect();

        assert!(!highs.is_empty());
        assert!(!lows.is_empty());
    }

    #[test]
    fn test_bullish_bos() {
        let detector = BosDetector::default();

        let candles = vec![
            (100.0, 102.0, 98.0, 101.0, 1000),
            (101.0, 105.0, 100.0, 104.0, 2000), // Swing high at 105
            (104.0, 103.0, 99.0, 100.0, 3000),
            (100.0, 102.0, 99.0, 101.0, 4000),
            (101.0, 107.0, 100.0, 106.0, 5000), // Breaks above 105
        ];

        let bos_events = detector.detect(&candles);
        assert!(!bos_events.is_empty());

        let bullish_bos: Vec<_> = bos_events.iter().filter(|b| b.bos_type == BosType::Bullish).collect();
        assert!(!bullish_bos.is_empty());
    }

    #[test]
    fn test_bearish_bos() {
        let detector = BosDetector::default();

        let candles = vec![
            (100.0, 105.0, 98.0, 101.0, 1000),
            (101.0, 103.0, 96.0, 97.0, 2000),   // Swing low at 96
            (97.0, 100.0, 96.5, 99.0, 3000),
            (99.0, 101.0, 98.0, 100.0, 4000),
            (100.0, 99.0, 94.0, 95.0, 5000),    // Breaks below 96
        ];

        let bos_events = detector.detect(&candles);
        assert!(!bos_events.is_empty());

        let bearish_bos: Vec<_> = bos_events.iter().filter(|b| b.bos_type == BosType::Bearish).collect();
        assert!(!bearish_bos.is_empty());
    }
}
