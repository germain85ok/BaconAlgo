/// Order Block Detection
/// 
/// An Order Block is the last opposing candle before a strong move.
/// It represents where smart money likely placed their orders.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderBlockType {
    Bullish,  // Last red candle before strong bullish move
    Bearish,  // Last green candle before strong bearish move
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBlock {
    pub ob_type: OrderBlockType,
    pub top: f64,
    pub bottom: f64,
    pub timestamp: i64,
    pub mitigated: bool,
    pub mitigation_timestamp: Option<i64>,
    pub strength: f64, // 0.0 to 1.0 based on subsequent move size
}

impl OrderBlock {
    pub fn new(ob_type: OrderBlockType, top: f64, bottom: f64, timestamp: i64, strength: f64) -> Self {
        Self {
            ob_type,
            top,
            bottom,
            timestamp,
            mitigated: false,
            mitigation_timestamp: None,
            strength: strength.min(1.0).max(0.0),
        }
    }

    /// Check if price has mitigated (revisited) this order block
    pub fn check_mitigation(&mut self, price: f64, timestamp: i64) -> bool {
        if self.mitigated {
            return true;
        }

        let is_mitigated = match self.ob_type {
            OrderBlockType::Bullish => price >= self.bottom && price <= self.top,
            OrderBlockType::Bearish => price >= self.bottom && price <= self.top,
        };

        if is_mitigated {
            self.mitigated = true;
            self.mitigation_timestamp = Some(timestamp);
        }

        is_mitigated
    }

    /// Check if price is near this order block (within 1% distance)
    pub fn is_near(&self, price: f64) -> bool {
        let mid = (self.top + self.bottom) / 2.0;
        let distance_pct = ((price - mid).abs() / mid) * 100.0;
        distance_pct < 1.0
    }

    /// Get the size of the order block
    pub fn size(&self) -> f64 {
        self.top - self.bottom
    }
}

/// Detect Order Blocks from candle data
pub struct OrderBlockDetector {
    pub min_move_pct: f64, // Minimum subsequent move to qualify as OB
}

impl OrderBlockDetector {
    pub fn new(min_move_pct: f64) -> Self {
        Self { min_move_pct }
    }

    /// Detect order blocks from candle data
    /// Looks for last opposing candle before a strong directional move
    pub fn detect(
        &self,
        candles: &[(f64, f64, f64, f64, i64)], // (open, high, low, close, timestamp)
    ) -> Vec<OrderBlock> {
        if candles.len() < 5 {
            return vec![];
        }

        let mut order_blocks = Vec::new();

        for i in 1..candles.len() - 3 {
            let (open_prev, high_prev, low_prev, close_prev, _) = candles[i - 1];
            let (open, high, low, close, ts) = candles[i];

            // Check for bullish order block: last bearish candle before bullish move
            if close < open {
                // Check if followed by strong bullish move
                let mut max_high = high;
                for j in (i + 1)..=(i + 3).min(candles.len() - 1) {
                    max_high = max_high.max(candles[j].1);
                }

                let move_pct = ((max_high - close) / close) * 100.0;
                if move_pct >= self.min_move_pct {
                    let strength = (move_pct / (self.min_move_pct * 3.0)).min(1.0);
                    order_blocks.push(OrderBlock::new(
                        OrderBlockType::Bullish,
                        high,
                        low,
                        ts,
                        strength,
                    ));
                }
            }
            // Check for bearish order block: last bullish candle before bearish move
            else if close > open {
                let mut min_low = low;
                for j in (i + 1)..=(i + 3).min(candles.len() - 1) {
                    min_low = min_low.min(candles[j].2);
                }

                let move_pct = ((close - min_low) / close) * 100.0;
                if move_pct >= self.min_move_pct {
                    let strength = (move_pct / (self.min_move_pct * 3.0)).min(1.0);
                    order_blocks.push(OrderBlock::new(
                        OrderBlockType::Bearish,
                        high,
                        low,
                        ts,
                        strength,
                    ));
                }
            }
        }

        order_blocks
    }

    /// Filter to get only unmitigated order blocks
    pub fn get_unmitigated(&self, order_blocks: &[OrderBlock]) -> Vec<OrderBlock> {
        order_blocks
            .iter()
            .filter(|ob| !ob.mitigated)
            .cloned()
            .collect()
    }
}

impl Default for OrderBlockDetector {
    fn default() -> Self {
        Self::new(1.0) // 1% minimum move
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullish_ob_detection() {
        let detector = OrderBlockDetector::default();

        // Last bearish candle followed by strong bullish move
        let candles = vec![
            (100.0, 102.0, 98.0, 101.0, 1000),
            (101.0, 103.0, 100.0, 99.0, 2000),   // Bearish candle
            (99.0, 105.0, 99.0, 104.0, 3000),    // Strong bullish
            (104.0, 108.0, 103.0, 107.0, 4000),  // Continuation
            (107.0, 110.0, 106.0, 109.0, 5000),
        ];

        let obs = detector.detect(&candles);
        assert!(!obs.is_empty());
        
        let bullish_obs: Vec<_> = obs.iter().filter(|ob| ob.ob_type == OrderBlockType::Bullish).collect();
        assert!(!bullish_obs.is_empty());
    }

    #[test]
    fn test_ob_mitigation() {
        let mut ob = OrderBlock::new(OrderBlockType::Bullish, 103.0, 100.0, 2000, 0.8);

        assert!(!ob.mitigated);
        assert!(!ob.check_mitigation(99.0, 3000));  // Below OB
        assert!(ob.check_mitigation(101.0, 4000));  // Inside OB - mitigated
        assert!(ob.mitigated);
    }

    #[test]
    fn test_is_near() {
        let ob = OrderBlock::new(OrderBlockType::Bullish, 103.0, 100.0, 2000, 0.8);

        assert!(ob.is_near(101.5));  // Mid point
        assert!(ob.is_near(102.0));  // Inside
        assert!(!ob.is_near(110.0)); // Far away
    }
}
