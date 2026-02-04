/// Elliott Wave detection (simplified Wave 3 detection)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElliottWave {
    pub wave_type: WaveType,
    pub start_price: f64,
    pub end_price: f64,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WaveType {
    Wave1,
    Wave2,
    Wave3,  // Usually the strongest - target for trades
    Wave4,
    Wave5,
}

#[derive(Debug, Clone)]
pub struct SwingPoint {
    pub price: f64,
    pub timestamp: i64,
    pub is_high: bool,
}

impl ElliottWave {
    /// Simplified Wave 3 detection
    /// Wave 3 characteristics:
    /// - Typically the longest wave
    /// - Should be at least 1.618x of Wave 1
    /// - Never the shortest of waves 1, 3, 5
    pub fn detect_wave3(swings: &[SwingPoint]) -> Option<ElliottWave> {
        if swings.len() < 5 {
            return None;
        }
        
        // Looking for pattern: Low -> High (W1) -> Low (W2) -> High (W3)
        let wave1_start = swings[0].price;
        let wave1_end = swings[1].price;
        let wave2_end = swings[2].price;
        let wave3_end = swings[3].price;
        
        let wave1_size = (wave1_end - wave1_start).abs();
        let wave3_size = (wave3_end - wave2_end).abs();
        
        // Wave 3 should be at least 1.618x of Wave 1 (Fibonacci extension)
        if wave3_size >= wave1_size * 1.618 {
            return Some(ElliottWave {
                wave_type: WaveType::Wave3,
                start_price: wave2_end,
                end_price: wave3_end,
                start_time: swings[2].timestamp,
                end_time: swings[3].timestamp,
            });
        }
        
        None
    }
    
    /// Check if currently in potential Wave 3
    pub fn is_wave3_active(&self, current_price: f64) -> bool {
        match self.wave_type {
            WaveType::Wave3 => {
                let is_uptrend = self.end_price > self.start_price;
                if is_uptrend {
                    current_price >= self.start_price && current_price <= self.end_price * 1.1
                } else {
                    current_price <= self.start_price && current_price >= self.end_price * 0.9
                }
            },
            _ => false,
        }
    }
}

/// Wave pattern analyzer
#[derive(Debug, Clone)]
pub struct WaveAnalyzer {
    swings: Vec<SwingPoint>,
}

impl WaveAnalyzer {
    pub fn new() -> Self {
        WaveAnalyzer {
            swings: Vec::new(),
        }
    }
    
    pub fn add_swing(&mut self, swing: SwingPoint) {
        self.swings.push(swing);
    }
    
    pub fn detect_wave3(&self) -> Option<ElliottWave> {
        ElliottWave::detect_wave3(&self.swings)
    }
    
    /// Find swing points from price data
    pub fn find_swings(prices: &[(f64, i64)], window: usize) -> Vec<SwingPoint> {
        let mut swings = Vec::new();
        
        for i in window..prices.len() - window {
            let current_price = prices[i].0;
            
            // Check if local high
            let is_high = prices[i - window..i].iter().all(|(p, _)| *p < current_price)
                && prices[i + 1..=i + window].iter().all(|(p, _)| *p < current_price);
            
            // Check if local low
            let is_low = prices[i - window..i].iter().all(|(p, _)| *p > current_price)
                && prices[i + 1..=i + window].iter().all(|(p, _)| *p > current_price);
            
            if is_high {
                swings.push(SwingPoint {
                    price: current_price,
                    timestamp: prices[i].1,
                    is_high: true,
                });
            } else if is_low {
                swings.push(SwingPoint {
                    price: current_price,
                    timestamp: prices[i].1,
                    is_high: false,
                });
            }
        }
        
        swings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave3_detection() {
        let swings = vec![
            SwingPoint { price: 100.0, timestamp: 1, is_high: false },
            SwingPoint { price: 110.0, timestamp: 2, is_high: true },
            SwingPoint { price: 105.0, timestamp: 3, is_high: false },
            SwingPoint { price: 125.0, timestamp: 4, is_high: true },
        ];
        
        let wave3 = ElliottWave::detect_wave3(&swings);
        assert!(wave3.is_some());
        let wave = wave3.unwrap();
        assert_eq!(wave.wave_type, WaveType::Wave3);
    }

    #[test]
    fn test_swing_detection() {
        let prices = vec![
            (100.0, 1), (105.0, 2), (110.0, 3), (108.0, 4), (106.0, 5),
            (104.0, 6), (108.0, 7), (112.0, 8),
        ];
        
        let swings = WaveAnalyzer::find_swings(&prices, 1);
        assert!(!swings.is_empty());
    }
}
