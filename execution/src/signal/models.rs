use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Horizon {
    Scalp,
    Day,
    Swing,
    Long,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    Long,
    Short,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Grade {
    Legendary, // 90+
    Epic,      // 80-89
    Rare,      // 70-79
    Common,    // < 70
}

impl Grade {
    pub fn from_score(score: u8) -> Self {
        match score {
            90..=100 => Grade::Legendary,
            80..=89 => Grade::Epic,
            70..=79 => Grade::Rare,
            _ => Grade::Common,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub id: Uuid,
    pub symbol: String,
    pub horizon: Horizon,
    pub direction: Direction,
    pub score: u8, // 0-100
    pub grade: Grade,
    
    // Entry/Exit
    pub entry: f64,
    pub stop_loss: f64,
    pub targets: Vec<f64>, // TP1, TP2, TP3
    pub risk_reward: f64,
    
    // SMC Tags
    pub near_fvg: bool,
    pub near_ob: bool,
    pub near_golden_pocket: bool,
    pub near_npoc: bool,
    pub near_avwap: bool,
    pub bos_confirmed: bool,
    pub choch_detected: bool,
    
    // Volume Profile (simplified for now)
    pub poc: Option<f64>,
    pub vah: Option<f64>,
    pub val: Option<f64>,
    
    // Elliott Wave (simplified)
    pub wave_phase: Option<String>,
    
    // Confluence Score
    pub confluence_count: u8,
    pub power_score: u16,
    
    // Whale Activity
    pub whale_score: u8,
    pub whale_bars: u8,
    pub absorption_count: u8,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl Signal {
    pub fn new(symbol: String, horizon: Horizon, direction: Direction) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            symbol,
            horizon,
            direction,
            score: 0,
            grade: Grade::Common,
            entry: 0.0,
            stop_loss: 0.0,
            targets: vec![],
            risk_reward: 0.0,
            near_fvg: false,
            near_ob: false,
            near_golden_pocket: false,
            near_npoc: false,
            near_avwap: false,
            bos_confirmed: false,
            choch_detected: false,
            poc: None,
            vah: None,
            val: None,
            wave_phase: None,
            confluence_count: 0,
            power_score: 0,
            whale_score: 0,
            whale_bars: 0,
            absorption_count: 0,
            created_at: Utc::now(),
            expires_at: None,
        }
    }

    pub fn calculate_risk_reward(&mut self) {
        let risk = (self.entry - self.stop_loss).abs();
        if risk > 0.0 && !self.targets.is_empty() {
            let reward = (self.targets[0] - self.entry).abs();
            self.risk_reward = reward / risk;
        }
    }

    pub fn update_grade(&mut self) {
        self.grade = Grade::from_score(self.score);
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenPocket {
    pub fib_618: f64,
    pub fib_65: f64,
    pub fib_786: f64,
    pub swing_low: f64,
    pub swing_high: f64,
    pub is_bullish: bool,
}

impl GoldenPocket {
    pub fn new(swing_high: f64, swing_low: f64, is_bullish: bool) -> Self {
        let range = swing_high - swing_low;
        Self {
            fib_618: swing_low + range * 0.618,
            fib_65: swing_low + range * 0.65,
            fib_786: swing_low + range * 0.786,
            swing_low,
            swing_high,
            is_bullish,
        }
    }

    pub fn is_in_zone(&self, price: f64) -> bool {
        price >= self.fib_618 && price <= self.fib_786
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_creation() {
        let signal = Signal::new("BTCUSDT".to_string(), Horizon::Day, Direction::Long);
        assert_eq!(signal.symbol, "BTCUSDT");
        assert_eq!(signal.horizon, Horizon::Day);
        assert_eq!(signal.direction, Direction::Long);
    }

    #[test]
    fn test_grade_from_score() {
        assert_eq!(Grade::from_score(95), Grade::Legendary);
        assert_eq!(Grade::from_score(85), Grade::Epic);
        assert_eq!(Grade::from_score(75), Grade::Rare);
        assert_eq!(Grade::from_score(65), Grade::Common);
    }

    #[test]
    fn test_risk_reward_calculation() {
        let mut signal = Signal::new("TEST".to_string(), Horizon::Scalp, Direction::Long);
        signal.entry = 100.0;
        signal.stop_loss = 95.0;
        signal.targets = vec![110.0];
        signal.calculate_risk_reward();
        assert_eq!(signal.risk_reward, 2.0); // 10/5 = 2
    }

    #[test]
    fn test_golden_pocket() {
        let gp = GoldenPocket::new(100.0, 80.0, true);
        assert_eq!(gp.fib_618, 92.36);
        assert!(gp.is_in_zone(93.0));
        assert!(!gp.is_in_zone(100.0));
    }
}
