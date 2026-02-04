/// Opening Range Breakout (ORB5/ORB15)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ORB {
    pub high: f64,
    pub low: f64,
    pub range: f64,
    pub breakout_type: Option<ORBBreakoutType>,
    pub minutes: u32,  // 5 or 15
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ORBBreakoutType {
    Bullish,   // Price broke above ORB high
    Bearish,   // Price broke below ORB low
}

#[derive(Debug, Clone)]
pub struct Candle {
    pub high: f64,
    pub low: f64,
    pub timestamp: i64,
}

impl ORB {
    /// Calculate Opening Range from first N minutes of trading
    pub fn calculate(candles: &[Candle], minutes: u32) -> Option<Self> {
        if candles.is_empty() {
            return None;
        }
        
        let high = candles.iter()
            .map(|c| c.high)
            .fold(f64::NEG_INFINITY, f64::max);
        
        let low = candles.iter()
            .map(|c| c.low)
            .fold(f64::INFINITY, f64::min);
        
        let range = high - low;
        
        Some(ORB {
            high,
            low,
            range,
            breakout_type: None,
            minutes,
        })
    }
    
    /// Check for breakout
    pub fn check_breakout(&mut self, current_price: f64) -> Option<ORBBreakoutType> {
        if current_price > self.high {
            self.breakout_type = Some(ORBBreakoutType::Bullish);
            return Some(ORBBreakoutType::Bullish);
        }
        
        if current_price < self.low {
            self.breakout_type = Some(ORBBreakoutType::Bearish);
            return Some(ORBBreakoutType::Bearish);
        }
        
        None
    }
    
    /// Check if price is within the opening range
    pub fn is_in_range(&self, price: f64) -> bool {
        price >= self.low && price <= self.high
    }
}

/// ORB Strategy helper
#[derive(Debug, Clone)]
pub struct ORBStrategy {
    pub orb5: Option<ORB>,
    pub orb15: Option<ORB>,
}

impl ORBStrategy {
    pub fn new() -> Self {
        ORBStrategy {
            orb5: None,
            orb15: None,
        }
    }
    
    pub fn set_orb5(&mut self, candles: &[Candle]) {
        self.orb5 = ORB::calculate(candles, 5);
    }
    
    pub fn set_orb15(&mut self, candles: &[Candle]) {
        self.orb15 = ORB::calculate(candles, 15);
    }
    
    /// Check if both ORB5 and ORB15 confirm the same breakout
    pub fn is_confirmed_breakout(&self) -> Option<ORBBreakoutType> {
        match (&self.orb5, &self.orb15) {
            (Some(orb5), Some(orb15)) => {
                if orb5.breakout_type == orb15.breakout_type {
                    orb5.breakout_type.clone()
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orb_calculation() {
        let candles = vec![
            Candle { high: 102.0, low: 98.0, timestamp: 1 },
            Candle { high: 103.0, low: 99.0, timestamp: 2 },
            Candle { high: 101.0, low: 97.0, timestamp: 3 },
        ];
        
        let orb = ORB::calculate(&candles, 5).unwrap();
        assert_eq!(orb.high, 103.0);
        assert_eq!(orb.low, 97.0);
        assert_eq!(orb.range, 6.0);
    }

    #[test]
    fn test_orb_breakout() {
        let candles = vec![
            Candle { high: 102.0, low: 98.0, timestamp: 1 },
        ];
        
        let mut orb = ORB::calculate(&candles, 5).unwrap();
        
        let breakout = orb.check_breakout(105.0);
        assert_eq!(breakout, Some(ORBBreakoutType::Bullish));
        
        let breakout2 = orb.check_breakout(95.0);
        assert_eq!(breakout2, Some(ORBBreakoutType::Bearish));
    }
}
