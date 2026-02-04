use crate::market::candle::Candle;
use crate::smc::{FVGDetector, OrderBlockDetector, BOSDetector, CHoCHDetector, LiquidityDetector};
use super::models::{Signal, GoldenPocket};

pub struct SignalScorer {
    fvg_detector: FVGDetector,
    ob_detector: OrderBlockDetector,
    bos_detector: BOSDetector,
    choch_detector: CHoCHDetector,
    liquidity_detector: LiquidityDetector,
}

impl SignalScorer {
    pub fn new() -> Self {
        Self {
            fvg_detector: FVGDetector,
            ob_detector: OrderBlockDetector,
            bos_detector: BOSDetector::new(),
            choch_detector: CHoCHDetector::new(),
            liquidity_detector: LiquidityDetector::new(),
        }
    }

    /// Calculate comprehensive signal score based on multiple factors
    pub fn score_signal(&mut self, signal: &mut Signal, candles: &[Candle], current_price: f64) {
        let mut score = 0u8;
        let mut confluence = 0u8;
        let mut power = 0u16;

        // Detect FVG
        let fvgs = FVGDetector::detect(candles);
        if FVGDetector::is_near_fvg(&fvgs, current_price, current_price * 0.001) {
            signal.near_fvg = true;
            score += 15;
            confluence += 1;
            power += 50;
        }

        // Detect Order Blocks
        let obs = OrderBlockDetector::detect(candles, 1.0);
        if OrderBlockDetector::is_near_ob(&obs, current_price, current_price * 0.001) {
            signal.near_ob = true;
            score += 20;
            confluence += 1;
            power += 75;
        }

        // Detect BOS
        let bos_list = self.bos_detector.detect(candles);
        if !bos_list.is_empty() {
            signal.bos_confirmed = true;
            score += 15;
            confluence += 1;
            power += 60;
        }

        // Detect CHoCH
        let choch_list = self.choch_detector.detect(candles);
        if !choch_list.is_empty() {
            signal.choch_detected = true;
            score += 10;
            confluence += 1;
            power += 40;
        }

        // Detect Liquidity
        self.liquidity_detector.detect(candles);
        if self.liquidity_detector.has_recent_sweep(24) {
            score += 10;
            confluence += 1;
            power += 50;
        }

        // Check Golden Pocket
        if let Some((high, low)) = self.find_swing_range(candles) {
            let gp = GoldenPocket::new(high, low, true);
            if gp.is_in_zone(current_price) {
                signal.near_golden_pocket = true;
                score += 20;
                confluence += 1;
                power += 80;
            }
        }

        // Volume and momentum bonus
        if let Some(last_candle) = candles.last() {
            let body_size = (last_candle.close - last_candle.open).abs();
            let range = last_candle.high - last_candle.low;
            
            if body_size / range > 0.7 {
                // Strong directional candle
                score += 10;
                power += 30;
            }
        }

        // Update signal
        signal.confluence_count = confluence;
        signal.power_score = power;
        signal.score = score.min(100);
        signal.update_grade();
    }

    fn find_swing_range(&self, candles: &[Candle]) -> Option<(f64, f64)> {
        if candles.len() < 20 {
            return None;
        }

        let recent = &candles[candles.len() - 20..];
        let high = recent.iter().map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);
        let low = recent.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);

        Some((high, low))
    }

    /// Generate targets based on risk-reward ratios
    pub fn generate_targets(&self, entry: f64, stop_loss: f64, direction: &super::models::Direction) -> Vec<f64> {
        let risk = (entry - stop_loss).abs();
        let mut targets = Vec::new();

        match direction {
            super::models::Direction::Long => {
                targets.push(entry + risk * 2.0); // TP1: 2R
                targets.push(entry + risk * 3.0); // TP2: 3R
                targets.push(entry + risk * 5.0); // TP3: 5R
            },
            super::models::Direction::Short => {
                targets.push(entry - risk * 2.0); // TP1: 2R
                targets.push(entry - risk * 3.0); // TP2: 3R
                targets.push(entry - risk * 5.0); // TP3: 5R
            },
        }

        targets
    }

    /// Calculate whale activity score (simplified)
    pub fn calculate_whale_score(&self, candles: &[Candle]) -> (u8, u8) {
        let mut whale_score = 0u8;
        let mut whale_bars = 0u8;

        if candles.len() < 10 {
            return (0, 0);
        }

        // Calculate average volume proxy (using range as proxy for volume)
        let avg_range: f64 = candles.iter()
            .take(50)
            .map(|c| c.high - c.low)
            .sum::<f64>() / (50.0_f64).min(candles.len() as f64);

        // Check recent candles for whale activity
        for candle in candles.iter().rev().take(10) {
            let range = candle.high - candle.low;
            if range > avg_range * 2.0 {
                whale_bars += 1;
                whale_score += 10;
            }
        }

        (whale_score.min(100), whale_bars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::timeframe::Timeframe;
    use crate::signal::models::{Horizon, Direction};

    #[test]
    fn test_signal_scoring() {
        let mut scorer = SignalScorer::new();
        let mut signal = Signal::new("TEST".to_string(), Horizon::Day, Direction::Long);
        
        let candles = vec![
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 100.0, high: 105.0, low: 99.0, close: 104.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 104.0, high: 110.0, low: 103.0, close: 109.0 },
            Candle { symbol: "TEST".into(), tf: Timeframe::M15, open: 109.0, high: 115.0, low: 108.0, close: 114.0 },
        ];

        scorer.score_signal(&mut signal, &candles, 110.0);
        assert!(signal.score > 0);
    }

    #[test]
    fn test_target_generation() {
        let scorer = SignalScorer::new();
        let targets = scorer.generate_targets(100.0, 95.0, &Direction::Long);
        
        assert_eq!(targets.len(), 3);
        assert_eq!(targets[0], 110.0); // 2R
        assert_eq!(targets[1], 115.0); // 3R
        assert_eq!(targets[2], 125.0); // 5R
    }
}
