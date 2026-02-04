/// Fibonacci levels - Golden Pocket (0.618-0.65)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FibonacciLevels {
    pub high: f64,
    pub low: f64,
    pub fib_0: f64,        // 0%
    pub fib_236: f64,      // 23.6%
    pub fib_382: f64,      // 38.2%
    pub fib_50: f64,       // 50%
    pub fib_618: f64,      // 61.8% (Golden Ratio)
    pub fib_65: f64,       // 65%
    pub fib_786: f64,      // 78.6%
    pub fib_100: f64,      // 100%
}

impl FibonacciLevels {
    /// Calculate Fibonacci retracement levels from swing high to swing low
    pub fn calculate(high: f64, low: f64) -> Self {
        let range = high - low;
        
        FibonacciLevels {
            high,
            low,
            fib_0: high,
            fib_236: high - range * 0.236,
            fib_382: high - range * 0.382,
            fib_50: high - range * 0.50,
            fib_618: high - range * 0.618,
            fib_65: high - range * 0.65,
            fib_786: high - range * 0.786,
            fib_100: low,
        }
    }
    
    /// Check if price is in Golden Pocket (between 61.8% and 65%)
    pub fn is_in_golden_pocket(&self, price: f64) -> bool {
        price >= self.fib_65 && price <= self.fib_618
    }
    
    /// Get the nearest Fibonacci level to current price
    pub fn nearest_level(&self, price: f64) -> (f64, &str) {
        let levels = [
            (self.fib_0, "0%"),
            (self.fib_236, "23.6%"),
            (self.fib_382, "38.2%"),
            (self.fib_50, "50%"),
            (self.fib_618, "61.8%"),
            (self.fib_65, "65%"),
            (self.fib_786, "78.6%"),
            (self.fib_100, "100%"),
        ];
        
        levels.iter()
            .min_by(|(a, _), (b, _)| {
                let dist_a = (price - a).abs();
                let dist_b = (price - b).abs();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(level, name)| (*level, *name))
            .unwrap_or((price, "unknown"))
    }
}

/// Golden Pocket detector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenPocket {
    pub in_pocket: bool,
    pub upper_bound: f64,  // 61.8%
    pub lower_bound: f64,  // 65%
}

impl GoldenPocket {
    pub fn detect(fib: &FibonacciLevels, price: f64) -> Self {
        GoldenPocket {
            in_pocket: fib.is_in_golden_pocket(price),
            upper_bound: fib.fib_618,
            lower_bound: fib.fib_65,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_calculation() {
        let fib = FibonacciLevels::calculate(200.0, 100.0);
        
        assert_eq!(fib.fib_0, 200.0);
        assert_eq!(fib.fib_100, 100.0);
        assert_eq!(fib.fib_50, 150.0);
        assert_eq!(fib.fib_618, 138.2);
    }

    #[test]
    fn test_golden_pocket() {
        let fib = FibonacciLevels::calculate(200.0, 100.0);
        
        assert!(fib.is_in_golden_pocket(137.0));
        assert!(!fib.is_in_golden_pocket(150.0));
        assert!(!fib.is_in_golden_pocket(130.0));
    }

    #[test]
    fn test_nearest_level() {
        let fib = FibonacciLevels::calculate(200.0, 100.0);
        let (level, name) = fib.nearest_level(151.0);
        
        assert_eq!(name, "50%");
        assert_eq!(level, 150.0);
    }
}
