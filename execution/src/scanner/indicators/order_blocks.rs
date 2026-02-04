/// Order Blocks detection (Bullish & Bearish)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBlock {
    pub block_type: OrderBlockType,
    pub high: f64,
    pub low: f64,
    pub timestamp: i64,
    pub strength: f64,  // 0-100 based on volume and price movement
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderBlockType {
    Bullish,
    Bearish,
}

#[derive(Debug, Clone)]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: i64,
}

impl OrderBlock {
    /// Detect order blocks from candle data
    /// Order block = last down candle before strong up move (bullish) or vice versa
    pub fn detect(candles: &[Candle], min_move_percent: f64) -> Vec<OrderBlock> {
        if candles.len() < 3 {
            return Vec::new();
        }
        
        let mut blocks = Vec::new();
        
        for i in 1..candles.len() - 1 {
            let prev = &candles[i - 1];
            let current = &candles[i];
            let next = &candles[i + 1];
            
            // Bullish Order Block: bearish candle followed by strong bullish move
            let is_bearish_candle = current.close < current.open;
            let next_move_up = (next.close - current.low) / current.low * 100.0;
            
            if is_bearish_candle && next_move_up >= min_move_percent {
                let strength = (next_move_up / min_move_percent * 50.0).min(100.0);
                blocks.push(OrderBlock {
                    block_type: OrderBlockType::Bullish,
                    high: current.high,
                    low: current.low,
                    timestamp: current.timestamp,
                    strength,
                });
            }
            
            // Bearish Order Block: bullish candle followed by strong bearish move
            let is_bullish_candle = current.close > current.open;
            let next_move_down = (current.high - next.close) / current.high * 100.0;
            
            if is_bullish_candle && next_move_down >= min_move_percent {
                let strength = (next_move_down / min_move_percent * 50.0).min(100.0);
                blocks.push(OrderBlock {
                    block_type: OrderBlockType::Bearish,
                    high: current.high,
                    low: current.low,
                    timestamp: current.timestamp,
                    strength,
                });
            }
        }
        
        blocks
    }
    
    /// Check if price is near an order block
    pub fn is_price_near(&self, price: f64, tolerance_percent: f64) -> bool {
        let range = self.high - self.low;
        let tolerance = range * tolerance_percent / 100.0;
        
        price >= (self.low - tolerance) && price <= (self.high + tolerance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_block_detection() {
        let candles = vec![
            Candle { open: 100.0, high: 102.0, low: 99.0, close: 101.0, volume: 1000.0, timestamp: 1 },
            Candle { open: 101.0, high: 102.0, low: 98.0, close: 99.0, volume: 1500.0, timestamp: 2 },
            Candle { open: 99.0, high: 105.0, low: 99.0, close: 104.0, volume: 2000.0, timestamp: 3 },
        ];
        
        let blocks = OrderBlock::detect(&candles, 3.0);
        assert!(!blocks.is_empty());
        assert_eq!(blocks[0].block_type, OrderBlockType::Bullish);
    }

    #[test]
    fn test_price_near_block() {
        let block = OrderBlock {
            block_type: OrderBlockType::Bullish,
            high: 100.0,
            low: 95.0,
            timestamp: 1,
            strength: 75.0,
        };
        
        assert!(block.is_price_near(97.0, 5.0));
        assert!(!block.is_price_near(90.0, 5.0));
    }
}
