use crate::market::candle::Candle;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Break of Structure - indicates trend continuation
#[derive(Debug, Clone, Serialize)]
pub struct BOS {
    pub is_bullish: bool,
    pub level: f64,
    pub created_at: DateTime<Utc>,
    pub strength: u8, // 0-100
}

impl BOS {
    pub fn new(is_bullish: bool, level: f64, strength: u8) -> Self {
        Self {
            is_bullish,
            level,
            created_at: Utc::now(),
            strength,
        }
    }
}

pub struct BOSDetector {
    swing_highs: Vec<f64>,
    swing_lows: Vec<f64>,
}

impl BOSDetector {
    pub fn new() -> Self {
        Self {
            swing_highs: Vec::new(),
            swing_lows: Vec::new(),
        }
    }

    /// Detect Break of Structure from candle data
    pub fn detect(&mut self, candles: &[Candle]) -> Vec<BOS> {
        let mut bos_list = Vec::new();
        
        if candles.len() < 5 {
            return bos_list;
        }

        // Find swing highs and lows
        self.find_swing_points(candles);

        // Detect bullish BOS: break above previous swing high
        if let Some(&last_high) = self.swing_highs.last() {
            if let Some(current) = candles.last() {
                if current.high > last_high {
                    let strength = self.calculate_strength(current.high, last_high);
                    bos_list.push(BOS::new(true, last_high, strength));
                }
            }
        }

        // Detect bearish BOS: break below previous swing low
        if let Some(&last_low) = self.swing_lows.last() {
            if let Some(current) = candles.last() {
                if current.low < last_low {
                    let strength = self.calculate_strength(last_low, current.low);
                    bos_list.push(BOS::new(false, last_low, strength));
                }
            }
        }

        bos_list
    }

    fn find_swing_points(&mut self, candles: &[Candle]) {
        for i in 2..candles.len() - 2 {
            let curr = &candles[i];
            let prev1 = &candles[i - 1];
            let prev2 = &candles[i - 2];
            let next1 = &candles[i + 1];
            let next2 = &candles[i + 2];

            // Swing high: higher than 2 candles before and after
            if curr.high > prev1.high && curr.high > prev2.high &&
               curr.high > next1.high && curr.high > next2.high {
                self.swing_highs.push(curr.high);
            }

            // Swing low: lower than 2 candles before and after
            if curr.low < prev1.low && curr.low < prev2.low &&
               curr.low < next1.low && curr.low < next2.low {
                self.swing_lows.push(curr.low);
            }
        }

        // Keep only recent swing points
        if self.swing_highs.len() > 10 {
            self.swing_highs.drain(0..self.swing_highs.len() - 10);
        }
        if self.swing_lows.len() > 10 {
            self.swing_lows.drain(0..self.swing_lows.len() - 10);
        }
    }

    fn calculate_strength(&self, new_level: f64, old_level: f64) -> u8 {
        let pct_change = ((new_level - old_level).abs() / old_level) * 100.0;
        (pct_change.min(10.0) * 10.0) as u8
    }

    pub fn has_recent_bos(&mut self, candles: &[Candle], lookback: usize) -> bool {
        let recent = &candles[candles.len().saturating_sub(lookback)..];
        !self.detect(recent).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::timeframe::Timeframe;

    #[test]
    fn test_bullish_bos() {
        let mut detector = BOSDetector::new();
        let candles = vec![
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 102.0, low: 99.0, close: 101.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 101.0, high: 105.0, low: 100.0, close: 104.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 104.0, high: 108.0, low: 103.0, close: 106.0 }, // Swing high at 108
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 106.0, high: 107.0, low: 105.0, close: 106.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 106.0, high: 107.0, low: 104.0, close: 105.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 105.0, high: 110.0, low: 104.0, close: 109.0 }, // Break above 108
        ];

        let bos_list = detector.detect(&candles);
        assert!(bos_list.len() > 0);
        if let Some(bos) = bos_list.first() {
            assert!(bos.is_bullish);
        }
    }
}
