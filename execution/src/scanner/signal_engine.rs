use std::collections::HashMap;

/// Timeframe for analysis
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Timeframe {
    Tick,
    M1,
    M5,
    M15,
    M30,
    H1,
    H4,
    D1,
    W1,
}

impl Timeframe {
    pub fn all() -> Vec<Timeframe> {
        vec![
            Timeframe::Tick,
            Timeframe::M1,
            Timeframe::M5,
            Timeframe::M15,
            Timeframe::M30,
            Timeframe::H1,
            Timeframe::H4,
            Timeframe::D1,
            Timeframe::W1,
        ]
    }

    pub fn as_str(&self) -> &str {
        match self {
            Timeframe::Tick => "tick",
            Timeframe::M1 => "1m",
            Timeframe::M5 => "5m",
            Timeframe::M15 => "15m",
            Timeframe::M30 => "30m",
            Timeframe::H1 => "1h",
            Timeframe::H4 => "4h",
            Timeframe::D1 => "1d",
            Timeframe::W1 => "1w",
        }
    }
}

/// Signal type
#[derive(Clone, Debug, PartialEq)]
pub enum SignalType {
    Buy,
    Sell,
    Neutral,
}

/// Technical indicator signal
#[derive(Clone, Debug)]
pub struct IndicatorSignal {
    pub name: String,
    pub timeframe: Timeframe,
    pub signal: SignalType,
    pub strength: f64, // 0.0 to 1.0
    pub metadata: HashMap<String, f64>,
}

/// Fused signal across multiple indicators
#[derive(Clone, Debug)]
pub struct FusedSignal {
    pub symbol: String,
    pub signal: SignalType,
    pub confidence: f64,
    pub consensus_score: f64,
    pub timeframes: Vec<Timeframe>,
    pub indicators: Vec<String>,
    pub timestamp: u64,
}

/// Signal engine with multi-timeframe analysis
pub struct SignalEngine {
    /// Minimum confidence threshold
    min_confidence: f64,
    /// Minimum consensus percentage
    min_consensus: f64,
}

impl SignalEngine {
    pub fn new(min_confidence: f64, min_consensus: f64) -> Self {
        Self {
            min_confidence,
            min_consensus,
        }
    }

    /// Fuse multiple indicator signals into a single signal
    pub fn fuse_signals(&self, symbol: &str, signals: Vec<IndicatorSignal>) -> Option<FusedSignal> {
        if signals.is_empty() {
            return None;
        }

        // Count buy/sell/neutral signals
        let mut buy_count = 0.0;
        let mut sell_count = 0.0;
        let mut neutral_count = 0.0;
        let mut total_strength = 0.0;

        for signal in &signals {
            match signal.signal {
                SignalType::Buy => buy_count += signal.strength,
                SignalType::Sell => sell_count += signal.strength,
                SignalType::Neutral => neutral_count += signal.strength,
            }
            total_strength += signal.strength;
        }

        // Determine overall signal
        let (signal_type, signal_strength) = if buy_count > sell_count && buy_count > neutral_count {
            (SignalType::Buy, buy_count)
        } else if sell_count > buy_count && sell_count > neutral_count {
            (SignalType::Sell, sell_count)
        } else {
            (SignalType::Neutral, neutral_count)
        };

        // Calculate consensus score (percentage of signals agreeing)
        let consensus = if total_strength > 0.0 {
            signal_strength / total_strength
        } else {
            0.0
        };

        // Calculate confidence (weighted average strength)
        let confidence = signal_strength / signals.len() as f64;

        // Filter by thresholds
        if confidence < self.min_confidence || consensus < self.min_consensus {
            return None;
        }

        let timeframes: Vec<Timeframe> = signals
            .iter()
            .map(|s| s.timeframe.clone())
            .collect();

        let indicators: Vec<String> = signals
            .iter()
            .map(|s| s.name.clone())
            .collect();

        Some(FusedSignal {
            symbol: symbol.to_string(),
            signal: signal_type,
            confidence,
            consensus_score: consensus,
            timeframes,
            indicators,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }

    /// Calculate RSI indicator signal
    pub fn calculate_rsi_signal(
        &self,
        timeframe: Timeframe,
        rsi_value: f64,
    ) -> IndicatorSignal {
        let (signal, strength) = if rsi_value < 30.0 {
            (SignalType::Buy, (30.0 - rsi_value) / 30.0)
        } else if rsi_value > 70.0 {
            (SignalType::Sell, (rsi_value - 70.0) / 30.0)
        } else {
            (SignalType::Neutral, 0.5)
        };

        let mut metadata = HashMap::new();
        metadata.insert("rsi".to_string(), rsi_value);

        IndicatorSignal {
            name: "RSI".to_string(),
            timeframe,
            signal,
            strength: strength.min(1.0),
            metadata,
        }
    }

    /// Calculate MACD indicator signal
    pub fn calculate_macd_signal(
        &self,
        timeframe: Timeframe,
        macd: f64,
        signal: f64,
    ) -> IndicatorSignal {
        let diff = macd - signal;
        let (signal_type, strength) = if diff > 0.0 {
            (SignalType::Buy, diff.abs().min(1.0))
        } else if diff < 0.0 {
            (SignalType::Sell, diff.abs().min(1.0))
        } else {
            (SignalType::Neutral, 0.5)
        };

        let mut metadata = HashMap::new();
        metadata.insert("macd".to_string(), macd);
        metadata.insert("signal".to_string(), signal);
        metadata.insert("histogram".to_string(), diff);

        IndicatorSignal {
            name: "MACD".to_string(),
            timeframe,
            signal: signal_type,
            strength,
            metadata,
        }
    }

    /// Calculate Moving Average crossover signal
    pub fn calculate_ma_crossover_signal(
        &self,
        timeframe: Timeframe,
        fast_ma: f64,
        slow_ma: f64,
    ) -> IndicatorSignal {
        let (signal, strength) = if fast_ma > slow_ma {
            let diff_pct = (fast_ma - slow_ma) / slow_ma;
            (SignalType::Buy, diff_pct.min(1.0))
        } else if fast_ma < slow_ma {
            let diff_pct = (slow_ma - fast_ma) / slow_ma;
            (SignalType::Sell, diff_pct.min(1.0))
        } else {
            (SignalType::Neutral, 0.5)
        };

        let mut metadata = HashMap::new();
        metadata.insert("fast_ma".to_string(), fast_ma);
        metadata.insert("slow_ma".to_string(), slow_ma);

        IndicatorSignal {
            name: "MA_Crossover".to_string(),
            timeframe,
            signal,
            strength,
            metadata,
        }
    }

    /// Calculate Bollinger Bands signal
    pub fn calculate_bollinger_signal(
        &self,
        timeframe: Timeframe,
        price: f64,
        upper_band: f64,
        lower_band: f64,
        middle_band: f64,
    ) -> IndicatorSignal {
        let (signal, strength) = if price <= lower_band {
            (SignalType::Buy, ((lower_band - price) / middle_band).min(1.0))
        } else if price >= upper_band {
            (SignalType::Sell, ((price - upper_band) / middle_band).min(1.0))
        } else {
            (SignalType::Neutral, 0.5)
        };

        let mut metadata = HashMap::new();
        metadata.insert("price".to_string(), price);
        metadata.insert("upper".to_string(), upper_band);
        metadata.insert("lower".to_string(), lower_band);
        metadata.insert("middle".to_string(), middle_band);

        IndicatorSignal {
            name: "BollingerBands".to_string(),
            timeframe,
            signal,
            strength,
            metadata,
        }
    }

    /// Calculate Stochastic signal
    pub fn calculate_stochastic_signal(
        &self,
        timeframe: Timeframe,
        k_value: f64,
        d_value: f64,
    ) -> IndicatorSignal {
        let (signal, strength) = if k_value < 20.0 && k_value > d_value {
            (SignalType::Buy, (20.0 - k_value) / 20.0)
        } else if k_value > 80.0 && k_value < d_value {
            (SignalType::Sell, (k_value - 80.0) / 20.0)
        } else {
            (SignalType::Neutral, 0.5)
        };

        let mut metadata = HashMap::new();
        metadata.insert("k".to_string(), k_value);
        metadata.insert("d".to_string(), d_value);

        IndicatorSignal {
            name: "Stochastic".to_string(),
            timeframe,
            signal,
            strength: strength.min(1.0),
            metadata,
        }
    }
}

impl Default for SignalEngine {
    fn default() -> Self {
        Self::new(0.6, 0.6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_signal() {
        let engine = SignalEngine::default();
        
        let signal = engine.calculate_rsi_signal(Timeframe::H1, 25.0);
        assert_eq!(signal.signal, SignalType::Buy);
        assert!(signal.strength > 0.0);
        
        let signal = engine.calculate_rsi_signal(Timeframe::H1, 75.0);
        assert_eq!(signal.signal, SignalType::Sell);
    }

    #[test]
    fn test_signal_fusion() {
        let engine = SignalEngine::new(0.5, 0.5);
        
        let signals = vec![
            IndicatorSignal {
                name: "RSI".to_string(),
                timeframe: Timeframe::H1,
                signal: SignalType::Buy,
                strength: 0.8,
                metadata: HashMap::new(),
            },
            IndicatorSignal {
                name: "MACD".to_string(),
                timeframe: Timeframe::H1,
                signal: SignalType::Buy,
                strength: 0.7,
                metadata: HashMap::new(),
            },
            IndicatorSignal {
                name: "MA".to_string(),
                timeframe: Timeframe::H1,
                signal: SignalType::Buy,
                strength: 0.9,
                metadata: HashMap::new(),
            },
        ];

        let fused = engine.fuse_signals("BTCUSDT", signals);
        assert!(fused.is_some());
        
        let signal = fused.unwrap();
        assert_eq!(signal.signal, SignalType::Buy);
        assert!(signal.confidence > 0.5);
        assert!(signal.consensus_score > 0.5);
    }

    #[test]
    fn test_low_consensus_filtered() {
        let engine = SignalEngine::new(0.6, 0.8);
        
        let signals = vec![
            IndicatorSignal {
                name: "RSI".to_string(),
                timeframe: Timeframe::H1,
                signal: SignalType::Buy,
                strength: 0.5,
                metadata: HashMap::new(),
            },
            IndicatorSignal {
                name: "MACD".to_string(),
                timeframe: Timeframe::H1,
                signal: SignalType::Sell,
                strength: 0.5,
                metadata: HashMap::new(),
            },
        ];

        let fused = engine.fuse_signals("BTCUSDT", signals);
        assert!(fused.is_none()); // Should be filtered out due to low consensus
    }
}
