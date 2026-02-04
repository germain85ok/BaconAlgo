/// Delta & CVD (Cumulative Volume Delta) calculation
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    pub buy_volume: f64,
    pub sell_volume: f64,
    pub delta: f64,           // buy_volume - sell_volume
    pub cumulative_delta: f64, // CVD
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub price: f64,
    pub volume: f64,
    pub is_buy: bool,  // true for aggressive buy, false for aggressive sell
}

impl Delta {
    /// Calculate delta from trades
    pub fn from_trades(trades: &[Trade]) -> Self {
        let buy_volume: f64 = trades.iter()
            .filter(|t| t.is_buy)
            .map(|t| t.volume)
            .sum();
        
        let sell_volume: f64 = trades.iter()
            .filter(|t| !t.is_buy)
            .map(|t| t.volume)
            .sum();
        
        let delta = buy_volume - sell_volume;
        
        Delta {
            buy_volume,
            sell_volume,
            delta,
            cumulative_delta: delta,
        }
    }
    
    /// Update CVD with new delta
    pub fn add_to_cvd(&mut self, new_delta: f64) {
        self.cumulative_delta += new_delta;
    }
}

/// Track cumulative volume delta over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CVDTracker {
    pub cvd: f64,
    pub deltas: Vec<f64>,
}

impl CVDTracker {
    pub fn new() -> Self {
        CVDTracker {
            cvd: 0.0,
            deltas: Vec::new(),
        }
    }
    
    pub fn update(&mut self, delta: f64) {
        self.cvd += delta;
        self.deltas.push(delta);
    }
    
    pub fn get_cvd(&self) -> f64 {
        self.cvd
    }
    
    /// Check if CVD is diverging from price (bullish/bearish signal)
    pub fn is_divergence(&self, price_trend_up: bool) -> bool {
        if self.deltas.len() < 2 {
            return false;
        }
        
        let recent_cvd_trend = self.cvd > *self.deltas.last().unwrap_or(&0.0);
        
        // Bullish divergence: price down but CVD up
        // Bearish divergence: price up but CVD down
        (!price_trend_up && recent_cvd_trend) || (price_trend_up && !recent_cvd_trend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_calculation() {
        let trades = vec![
            Trade { price: 100.0, volume: 10.0, is_buy: true },
            Trade { price: 100.5, volume: 5.0, is_buy: false },
            Trade { price: 101.0, volume: 15.0, is_buy: true },
        ];
        
        let delta = Delta::from_trades(&trades);
        assert_eq!(delta.buy_volume, 25.0);
        assert_eq!(delta.sell_volume, 5.0);
        assert_eq!(delta.delta, 20.0);
    }

    #[test]
    fn test_cvd_tracker() {
        let mut tracker = CVDTracker::new();
        tracker.update(10.0);
        tracker.update(-5.0);
        tracker.update(15.0);
        
        assert_eq!(tracker.get_cvd(), 20.0);
    }
}
