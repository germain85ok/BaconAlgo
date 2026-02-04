use crate::market::candle::Candle;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Liquidity Zone - areas where stop losses are likely clustered
#[derive(Debug, Clone, Serialize)]
pub struct LiquidityZone {
    pub is_buy_side: bool, // true = buy-side liquidity (above highs), false = sell-side (below lows)
    pub level: f64,
    pub created_at: DateTime<Utc>,
    pub is_swept: bool,
    pub strength: u8, // 0-100, based on number of times tested
}

impl LiquidityZone {
    pub fn new(is_buy_side: bool, level: f64, strength: u8) -> Self {
        Self {
            is_buy_side,
            level,
            created_at: Utc::now(),
            is_swept: false,
            strength,
        }
    }

    pub fn check_sweep(&mut self, candle: &Candle) {
        if self.is_buy_side {
            // Buy-side liquidity swept when price breaks above and retraces
            if candle.high > self.level && candle.close < self.level {
                self.is_swept = true;
            }
        } else {
            // Sell-side liquidity swept when price breaks below and retraces
            if candle.low < self.level && candle.close > self.level {
                self.is_swept = true;
            }
        }
    }
}

pub struct LiquidityDetector {
    zones: Vec<LiquidityZone>,
}

impl LiquidityDetector {
    pub fn new() -> Self {
        Self {
            zones: Vec::new(),
        }
    }

    /// Detect liquidity zones from swing highs and lows
    pub fn detect(&mut self, candles: &[Candle]) -> Vec<LiquidityZone> {
        let mut new_zones = Vec::new();
        
        if candles.len() < 5 {
            return new_zones;
        }

        for i in 2..candles.len() - 2 {
            let curr = &candles[i];
            let prev1 = &candles[i - 1];
            let prev2 = &candles[i - 2];
            let next1 = &candles[i + 1];
            let next2 = &candles[i + 2];

            // Buy-side liquidity at swing highs
            if curr.high > prev1.high && curr.high > prev2.high &&
               curr.high > next1.high && curr.high > next2.high {
                let strength = self.calculate_strength(candles, curr.high, true);
                new_zones.push(LiquidityZone::new(true, curr.high, strength));
            }

            // Sell-side liquidity at swing lows
            if curr.low < prev1.low && curr.low < prev2.low &&
               curr.low < next1.low && curr.low < next2.low {
                let strength = self.calculate_strength(candles, curr.low, false);
                new_zones.push(LiquidityZone::new(false, curr.low, strength));
            }
        }

        // Update existing zones
        if let Some(last_candle) = candles.last() {
            for zone in &mut self.zones {
                zone.check_sweep(last_candle);
            }
        }

        // Add new zones
        self.zones.extend(new_zones.clone());
        
        // Keep only unswept zones and recent swept zones
        self.zones.retain(|z| !z.is_swept || {
            let age = Utc::now().signed_duration_since(z.created_at);
            age.num_hours() < 24
        });

        new_zones
    }

    fn calculate_strength(&self, candles: &[Candle], level: f64, is_high: bool) -> u8 {
        let mut touches = 0;
        let tolerance = level * 0.001; // 0.1% tolerance

        for candle in candles.iter().rev().take(50) {
            if is_high {
                if (candle.high - level).abs() < tolerance {
                    touches += 1;
                }
            } else {
                if (candle.low - level).abs() < tolerance {
                    touches += 1;
                }
            }
        }

        (touches * 20).min(100) as u8
    }

    pub fn get_unswept_zones(&self) -> Vec<&LiquidityZone> {
        self.zones.iter().filter(|z| !z.is_swept).collect()
    }

    pub fn has_recent_sweep(&self, lookback_hours: i64) -> bool {
        let cutoff = Utc::now() - chrono::Duration::hours(lookback_hours);
        self.zones.iter().any(|z| z.is_swept && z.created_at > cutoff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::timeframe::Timeframe;

    #[test]
    fn test_liquidity_zone_detection() {
        let mut detector = LiquidityDetector::new();
        let candles = vec![
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 102.0, low: 99.0, close: 101.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 101.0, high: 105.0, low: 100.0, close: 104.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 104.0, high: 108.0, low: 103.0, close: 106.0 }, // Swing high
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 106.0, high: 107.0, low: 105.0, close: 106.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 106.0, high: 107.0, low: 104.0, close: 105.0 },
        ];

        let zones = detector.detect(&candles);
        assert!(zones.len() > 0);
    }

    #[test]
    fn test_liquidity_sweep() {
        let mut zone = LiquidityZone::new(true, 110.0, 50);
        
        // Sweep: wick above, close below
        let sweep_candle = Candle {
            symbol: "TEST".into(),
            tf: Timeframe::M15,
            open: 108.0,
            high: 112.0,
            low: 107.0,
            close: 109.0,
        };
        
        zone.check_sweep(&sweep_candle);
        assert!(zone.is_swept);
    }
}
