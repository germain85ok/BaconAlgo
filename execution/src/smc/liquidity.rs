/// Liquidity Detection
/// 
/// Liquidity zones are areas where stop losses and pending orders cluster,
/// often at equal highs/lows. Smart money hunts these liquidity pools.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LiquidityType {
    BuySideLiquidity,   // Above resistance (equal highs)
    SellSideLiquidity,  // Below support (equal lows)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityZone {
    pub liquidity_type: LiquidityType,
    pub price_level: f64,
    pub timestamp: i64,
    pub strength: f64,       // Based on number of touches
    pub swept: bool,         // Whether liquidity has been taken
    pub sweep_timestamp: Option<i64>,
}

impl LiquidityZone {
    pub fn new(liquidity_type: LiquidityType, price_level: f64, timestamp: i64, strength: f64) -> Self {
        Self {
            liquidity_type,
            price_level,
            timestamp,
            strength: strength.min(1.0).max(0.0),
            swept: false,
            sweep_timestamp: None,
        }
    }

    /// Check if this liquidity zone has been swept
    pub fn check_sweep(&mut self, high: f64, low: f64, timestamp: i64) -> bool {
        if self.swept {
            return true;
        }

        let is_swept = match self.liquidity_type {
            LiquidityType::BuySideLiquidity => high > self.price_level,
            LiquidityType::SellSideLiquidity => low < self.price_level,
        };

        if is_swept {
            self.swept = true;
            self.sweep_timestamp = Some(timestamp);
        }

        is_swept
    }

    /// Check if price is near this liquidity zone (within 0.3%)
    pub fn is_near(&self, price: f64) -> bool {
        let distance_pct = ((price - self.price_level).abs() / self.price_level) * 100.0;
        distance_pct < 0.3
    }
}

/// Detect liquidity zones from price data
pub struct LiquidityDetector {
    pub equal_threshold_pct: f64, // How close prices need to be to be "equal"
    pub min_touches: usize,        // Minimum touches to qualify as liquidity zone
}

impl LiquidityDetector {
    pub fn new(equal_threshold_pct: f64, min_touches: usize) -> Self {
        Self {
            equal_threshold_pct,
            min_touches,
        }
    }

    /// Detect equal highs (buy-side liquidity)
    pub fn find_equal_highs(
        &self,
        candles: &[(f64, f64, f64, f64, i64)], // (open, high, low, close, timestamp)
    ) -> Vec<LiquidityZone> {
        if candles.len() < self.min_touches {
            return vec![];
        }

        let mut liquidity_zones = Vec::new();
        let highs: Vec<(f64, i64)> = candles.iter().map(|(_, h, _, _, ts)| (*h, *ts)).collect();

        // Group similar highs
        let mut i = 0;
        while i < highs.len() {
            let (base_high, base_ts) = highs[i];
            let mut equal_count = 1;
            let mut sum = base_high;

            for j in (i + 1)..highs.len() {
                let (high, _) = highs[j];
                let diff_pct = ((high - base_high).abs() / base_high) * 100.0;

                if diff_pct <= self.equal_threshold_pct {
                    equal_count += 1;
                    sum += high;
                }
            }

            if equal_count >= self.min_touches {
                let avg_level = sum / equal_count as f64;
                let strength = (equal_count as f64 / (self.min_touches as f64 * 2.0)).min(1.0);

                liquidity_zones.push(LiquidityZone::new(
                    LiquidityType::BuySideLiquidity,
                    avg_level,
                    base_ts,
                    strength,
                ));
            }

            i += equal_count;
        }

        liquidity_zones
    }

    /// Detect equal lows (sell-side liquidity)
    pub fn find_equal_lows(
        &self,
        candles: &[(f64, f64, f64, f64, i64)], // (open, high, low, close, timestamp)
    ) -> Vec<LiquidityZone> {
        if candles.len() < self.min_touches {
            return vec![];
        }

        let mut liquidity_zones = Vec::new();
        let lows: Vec<(f64, i64)> = candles.iter().map(|(_, _, l, _, ts)| (*l, *ts)).collect();

        // Group similar lows
        let mut i = 0;
        while i < lows.len() {
            let (base_low, base_ts) = lows[i];
            let mut equal_count = 1;
            let mut sum = base_low;

            for j in (i + 1)..lows.len() {
                let (low, _) = lows[j];
                let diff_pct = ((low - base_low).abs() / base_low) * 100.0;

                if diff_pct <= self.equal_threshold_pct {
                    equal_count += 1;
                    sum += low;
                }
            }

            if equal_count >= self.min_touches {
                let avg_level = sum / equal_count as f64;
                let strength = (equal_count as f64 / (self.min_touches as f64 * 2.0)).min(1.0);

                liquidity_zones.push(LiquidityZone::new(
                    LiquidityType::SellSideLiquidity,
                    avg_level,
                    base_ts,
                    strength,
                ));
            }

            i += equal_count;
        }

        liquidity_zones
    }

    /// Detect all liquidity zones
    pub fn detect(
        &self,
        candles: &[(f64, f64, f64, f64, i64)],
    ) -> Vec<LiquidityZone> {
        let mut zones = Vec::new();
        zones.extend(self.find_equal_highs(candles));
        zones.extend(self.find_equal_lows(candles));
        zones
    }

    /// Check for recent liquidity sweeps
    pub fn detect_sweeps(
        &self,
        zones: &mut [LiquidityZone],
        recent_candles: &[(f64, f64, f64, f64, i64)],
    ) -> Vec<LiquidityZone> {
        let mut swept_zones = Vec::new();

        for candle in recent_candles {
            let (_, high, low, _, ts) = candle;

            for zone in zones.iter_mut() {
                if zone.check_sweep(*high, *low, *ts) {
                    swept_zones.push(zone.clone());
                }
            }
        }

        swept_zones
    }
}

impl Default for LiquidityDetector {
    fn default() -> Self {
        Self::new(0.2, 2) // 0.2% threshold, minimum 2 touches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_highs_detection() {
        let detector = LiquidityDetector::default();

        // Create candles with equal highs around 105
        let candles = vec![
            (100.0, 105.0, 98.0, 102.0, 1000),
            (102.0, 105.1, 100.0, 103.0, 2000), // Equal high
            (103.0, 104.9, 101.0, 102.0, 3000), // Equal high
            (102.0, 103.0, 100.0, 101.0, 4000),
        ];

        let zones = detector.find_equal_highs(&candles);
        assert!(!zones.is_empty());
        assert_eq!(zones[0].liquidity_type, LiquidityType::BuySideLiquidity);
    }

    #[test]
    fn test_equal_lows_detection() {
        let detector = LiquidityDetector::default();

        // Create candles with equal lows around 95
        let candles = vec![
            (100.0, 105.0, 95.0, 102.0, 1000),
            (102.0, 104.0, 95.1, 103.0, 2000), // Equal low
            (103.0, 106.0, 94.9, 105.0, 3000), // Equal low
            (105.0, 107.0, 100.0, 106.0, 4000),
        ];

        let zones = detector.find_equal_lows(&candles);
        assert!(!zones.is_empty());
        assert_eq!(zones[0].liquidity_type, LiquidityType::SellSideLiquidity);
    }

    #[test]
    fn test_liquidity_sweep() {
        let mut zone = LiquidityZone::new(
            LiquidityType::BuySideLiquidity,
            105.0,
            1000,
            0.8,
        );

        assert!(!zone.swept);
        assert!(!zone.check_sweep(104.0, 100.0, 2000)); // Didn't reach level
        assert!(zone.check_sweep(106.0, 100.0, 3000));  // Swept above
        assert!(zone.swept);
    }

    #[test]
    fn test_is_near() {
        let zone = LiquidityZone::new(
            LiquidityType::BuySideLiquidity,
            105.0,
            1000,
            0.8,
        );

        assert!(zone.is_near(105.1));  // Very close
        assert!(zone.is_near(104.9));  // Very close
        assert!(!zone.is_near(110.0)); // Far away
    }
}
