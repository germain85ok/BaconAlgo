/// Enhanced Signal Model with Scoring and SMC Integration
/// 
/// This module provides enhanced signal types with:
/// - Signal scoring (0-100)
/// - Confluence tracking
/// - SMC pattern tags
/// - Auto-generated targets

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SignalDirection {
    Buy,
    Sell,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SignalGrade {
    S,  // 90-100: Exceptional setup
    A,  // 80-89: Excellent setup
    B,  // 70-79: Good setup
    C,  // 60-69: Average setup
    D,  // 50-59: Weak setup
    F,  // <50: Failed/poor setup
}

impl SignalGrade {
    pub fn from_score(score: f64) -> Self {
        if score >= 90.0 {
            SignalGrade::S
        } else if score >= 80.0 {
            SignalGrade::A
        } else if score >= 70.0 {
            SignalGrade::B
        } else if score >= 60.0 {
            SignalGrade::C
        } else if score >= 50.0 {
            SignalGrade::D
        } else {
            SignalGrade::F
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalTargets {
    pub entry: f64,
    pub stop_loss: f64,
    pub take_profit_1: f64, // 2R
    pub take_profit_2: f64, // 3R
    pub take_profit_3: f64, // 5R
    pub risk_reward_ratio: f64,
}

impl SignalTargets {
    /// Generate targets based on entry and stop loss
    pub fn new(entry: f64, stop_loss: f64, direction: &SignalDirection) -> Self {
        let risk = (entry - stop_loss).abs();

        let (tp1, tp2, tp3) = match direction {
            SignalDirection::Buy => (
                entry + (risk * 2.0),  // 2R
                entry + (risk * 3.0),  // 3R
                entry + (risk * 5.0),  // 5R
            ),
            SignalDirection::Sell => (
                entry - (risk * 2.0),
                entry - (risk * 3.0),
                entry - (risk * 5.0),
            ),
            SignalDirection::Neutral => (entry, entry, entry),
        };

        Self {
            entry,
            stop_loss,
            take_profit_1: tp1,
            take_profit_2: tp2,
            take_profit_3: tp3,
            risk_reward_ratio: if direction == &SignalDirection::Neutral { 0.0 } else { 2.0 },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmcTags {
    pub near_fvg: bool,
    pub near_order_block: bool,
    pub bos_confirmed: bool,
    pub liquidity_sweep: bool,
    pub fvg_type: Option<String>,
    pub order_block_type: Option<String>,
    pub liquidity_type: Option<String>,
}

impl Default for SmcTags {
    fn default() -> Self {
        Self {
            near_fvg: false,
            near_order_block: false,
            bos_confirmed: false,
            liquidity_sweep: false,
            fvg_type: None,
            order_block_type: None,
            liquidity_type: None,
        }
    }
}

impl SmcTags {
    /// Calculate SMC bonus score
    pub fn bonus_score(&self) -> f64 {
        let mut score = 0.0;

        if self.near_fvg {
            score += 10.0;
        }
        if self.near_order_block {
            score += 15.0;
        }
        if self.bos_confirmed {
            score += 15.0;
        }
        if self.liquidity_sweep {
            score += 20.0;
        }

        score
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedSignal {
    pub symbol: String,
    pub timeframe: String,
    pub direction: SignalDirection,
    pub score: f64,            // 0-100 overall score
    pub grade: SignalGrade,    // S/A/B/C/D/F
    pub confluence_count: u32, // Number of aligned indicators
    pub power_score: f64,      // Strength of setup (0-100)
    pub whale_score: f64,      // Smart money involvement (0-100)
    pub targets: SignalTargets,
    pub smc_tags: SmcTags,
    pub indicators: Vec<String>,
    pub reason: String,
    pub timestamp: i64,
}

impl EnhancedSignal {
    /// Create a new enhanced signal
    pub fn new(
        symbol: String,
        timeframe: String,
        direction: SignalDirection,
        entry: f64,
        stop_loss: f64,
    ) -> Self {
        let targets = SignalTargets::new(entry, stop_loss, &direction);

        Self {
            symbol,
            timeframe,
            direction,
            score: 0.0,
            grade: SignalGrade::F,
            confluence_count: 0,
            power_score: 0.0,
            whale_score: 0.0,
            targets,
            smc_tags: SmcTags::default(),
            indicators: Vec::new(),
            reason: String::new(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }

    /// Add an indicator to the signal
    pub fn add_indicator(&mut self, indicator: String) {
        if !self.indicators.contains(&indicator) {
            self.indicators.push(indicator);
            self.confluence_count = self.indicators.len() as u32;
        }
    }

    /// Calculate confluence score based on number of aligned indicators
    fn calculate_confluence_score(&self) -> f64 {
        // More indicators = higher confidence
        match self.confluence_count {
            0..=1 => 20.0,
            2 => 40.0,
            3 => 60.0,
            4 => 75.0,
            5 => 85.0,
            _ => 95.0,
        }
    }

    /// Calculate overall signal score
    pub fn calculate_score(&mut self, indicator_strength: f64) {
        let confluence_score = self.calculate_confluence_score();
        let smc_bonus = self.smc_tags.bonus_score();

        // Weighted components:
        // - 40% confluence
        // - 30% indicator strength
        // - 20% SMC patterns
        // - 10% whale score
        self.score = (confluence_score * 0.4)
            + (indicator_strength * 100.0 * 0.3)
            + (smc_bonus * 0.2)
            + (self.whale_score * 0.1);

        self.score = self.score.min(100.0).max(0.0);
        self.grade = SignalGrade::from_score(self.score);
    }

    /// Set power score (setup strength)
    pub fn set_power_score(&mut self, power: f64) {
        self.power_score = power.min(100.0).max(0.0);
    }

    /// Set whale score (smart money involvement)
    pub fn set_whale_score(&mut self, whale: f64) {
        self.whale_score = whale.min(100.0).max(0.0);
    }

    /// Set SMC tags
    pub fn set_smc_tags(&mut self, tags: SmcTags) {
        self.smc_tags = tags;
    }

    /// Set reason for signal
    pub fn set_reason(&mut self, reason: String) {
        self.reason = reason;
    }

    /// Check if this is a high-quality signal (grade B or better)
    pub fn is_high_quality(&self) -> bool {
        matches!(self.grade, SignalGrade::S | SignalGrade::A | SignalGrade::B)
    }

    /// Check if this signal has SMC confirmation
    pub fn has_smc_confirmation(&self) -> bool {
        self.smc_tags.bos_confirmed
            || self.smc_tags.near_order_block
            || self.smc_tags.liquidity_sweep
    }
}

/// Builder for creating enhanced signals
pub struct SignalBuilder {
    signal: EnhancedSignal,
}

impl SignalBuilder {
    pub fn new(
        symbol: String,
        timeframe: String,
        direction: SignalDirection,
        entry: f64,
        stop_loss: f64,
    ) -> Self {
        Self {
            signal: EnhancedSignal::new(symbol, timeframe, direction, entry, stop_loss),
        }
    }

    pub fn add_indicator(mut self, indicator: String) -> Self {
        self.signal.add_indicator(indicator);
        self
    }

    pub fn power_score(mut self, score: f64) -> Self {
        self.signal.set_power_score(score);
        self
    }

    pub fn whale_score(mut self, score: f64) -> Self {
        self.signal.set_whale_score(score);
        self
    }

    pub fn smc_tags(mut self, tags: SmcTags) -> Self {
        self.signal.set_smc_tags(tags);
        self
    }

    pub fn reason(mut self, reason: String) -> Self {
        self.signal.set_reason(reason);
        self
    }

    pub fn build(mut self, indicator_strength: f64) -> EnhancedSignal {
        self.signal.calculate_score(indicator_strength);
        self.signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_targets() {
        let targets = SignalTargets::new(100.0, 98.0, &SignalDirection::Buy);
        
        assert_eq!(targets.entry, 100.0);
        assert_eq!(targets.stop_loss, 98.0);
        assert_eq!(targets.take_profit_1, 104.0); // 2R
        assert_eq!(targets.take_profit_2, 106.0); // 3R
        assert_eq!(targets.take_profit_3, 110.0); // 5R
    }

    #[test]
    fn test_signal_grade() {
        assert_eq!(SignalGrade::from_score(95.0), SignalGrade::S);
        assert_eq!(SignalGrade::from_score(85.0), SignalGrade::A);
        assert_eq!(SignalGrade::from_score(75.0), SignalGrade::B);
        assert_eq!(SignalGrade::from_score(65.0), SignalGrade::C);
        assert_eq!(SignalGrade::from_score(55.0), SignalGrade::D);
        assert_eq!(SignalGrade::from_score(45.0), SignalGrade::F);
    }

    #[test]
    fn test_signal_score_calculation() {
        let mut signal = EnhancedSignal::new(
            "BTCUSDT".to_string(),
            "H1".to_string(),
            SignalDirection::Buy,
            100.0,
            98.0,
        );

        signal.add_indicator("RSI".to_string());
        signal.add_indicator("MACD".to_string());
        signal.add_indicator("MA".to_string());

        signal.calculate_score(0.8);

        assert!(signal.score > 0.0);
        assert!(signal.score <= 100.0);
        assert_eq!(signal.confluence_count, 3);
    }

    #[test]
    fn test_smc_bonus() {
        let mut tags = SmcTags::default();
        tags.near_fvg = true;
        tags.bos_confirmed = true;
        tags.liquidity_sweep = true;

        let bonus = tags.bonus_score();
        assert_eq!(bonus, 45.0); // 10 + 15 + 20
    }

    #[test]
    fn test_signal_builder() {
        let signal = SignalBuilder::new(
            "ETHUSDT".to_string(),
            "M15".to_string(),
            SignalDirection::Buy,
            2000.0,
            1980.0,
        )
        .add_indicator("RSI".to_string())
        .add_indicator("MACD".to_string())
        .power_score(80.0)
        .whale_score(70.0)
        .reason("Golden pocket alignment".to_string())
        .build(0.85);

        assert_eq!(signal.symbol, "ETHUSDT");
        assert_eq!(signal.confluence_count, 2);
        assert!(signal.score > 0.0);
    }

    #[test]
    fn test_high_quality_check() {
        let mut signal = EnhancedSignal::new(
            "BTCUSDT".to_string(),
            "H1".to_string(),
            SignalDirection::Buy,
            100.0,
            98.0,
        );

        signal.score = 75.0;
        signal.grade = SignalGrade::from_score(75.0);
        assert!(signal.is_high_quality());

        signal.score = 55.0;
        signal.grade = SignalGrade::from_score(55.0);
        assert!(!signal.is_high_quality());
    }
}
