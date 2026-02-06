// BaconAlgo 2040 Quantum Edition - Signal Engine
// Moteur de signaux multi-timeframe avec fusion d'indicateurs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use smallvec::SmallVec;

/// Timeframe pour l'analyse
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    Tick,
    Second1,
    Minute1,
    Minute5,
    Minute15,
    Hour1,
    Hour4,
    Day1,
    Week1,
}

impl Timeframe {
    pub fn as_secs(&self) -> u64 {
        match self {
            Timeframe::Tick => 0,
            Timeframe::Second1 => 1,
            Timeframe::Minute1 => 60,
            Timeframe::Minute5 => 300,
            Timeframe::Minute15 => 900,
            Timeframe::Hour1 => 3600,
            Timeframe::Hour4 => 14400,
            Timeframe::Day1 => 86400,
            Timeframe::Week1 => 604800,
        }
    }
}

/// Indicateur technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Indicator {
    // Leading indicators
    RSI { period: usize, value: f64 },
    Stochastic { k: f64, d: f64 },
    WilliamsR { period: usize, value: f64 },
    CCI { period: usize, value: f64 },
    MFI { period: usize, value: f64 },
    
    // Lagging indicators
    EMA { period: usize, value: f64 },
    SMA { period: usize, value: f64 },
    MACD { fast: f64, slow: f64, signal: f64 },
    Bollinger { upper: f64, middle: f64, lower: f64 },
    Keltner { upper: f64, middle: f64, lower: f64 },
    Ichimoku { 
        tenkan: f64, 
        kijun: f64, 
        senkou_a: f64, 
        senkou_b: f64 
    },
    
    // Volume indicators
    OBV { value: f64 },
    VWAP { value: f64 },
    VolumeProfile { poc: f64, vah: f64, val: f64 },
    CVD { value: f64 },  // Cumulative Volume Delta
}

/// Signal de trading généré
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSignal {
    /// Symbole
    pub symbol: String,
    /// Action recommandée
    pub action: SignalAction,
    /// Timeframe du signal
    pub timeframe: Timeframe,
    /// Score de confiance (0-100)
    pub confidence: f64,
    /// Indicateurs supportant le signal
    pub supporting_indicators: Vec<Indicator>,
    /// Prix d'entrée suggéré
    pub entry_price: f64,
    /// Stop loss suggéré
    pub stop_loss: Option<f64>,
    /// Take profit suggéré
    pub take_profit: Option<f64>,
    /// Raison du signal
    pub reason: String,
    /// Timestamp
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalAction {
    BuyLong,
    SellShort,
    CloseLong,
    CloseShort,
    Hold,
}

/// Données OHLCV
#[derive(Debug, Clone)]
pub struct OHLCV {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub timestamp: u64,
}

/// Signal Engine - Analyse multi-timeframe
pub struct SignalEngine {
    /// Cache des données par symbole et timeframe
    data_cache: HashMap<(String, Timeframe), Vec<OHLCV>>,
    
    /// Signaux actifs
    active_signals: HashMap<String, Vec<TradingSignal>>,
}

impl SignalEngine {
    /// Crée un nouveau signal engine
    pub fn new() -> Self {
        Self {
            data_cache: HashMap::new(),
            active_signals: HashMap::new(),
        }
    }
    
    /// Analyse multi-timeframe pour un symbole
    pub fn analyze_multi_timeframe(&mut self, symbol: &str, price: f64, volume: f64) -> Vec<TradingSignal> {
        let mut signals = Vec::new();
        
        // Analyser chaque timeframe
        let timeframes = vec![
            Timeframe::Minute1,
            Timeframe::Minute5,
            Timeframe::Minute15,
            Timeframe::Hour1,
            Timeframe::Hour4,
            Timeframe::Day1,
        ];
        
        for timeframe in timeframes {
            if let Some(signal) = self.analyze_timeframe(symbol, timeframe, price, volume) {
                signals.push(signal);
            }
        }
        
        // Fusion des signaux (consensus entre timeframes)
        let fused_signal = self.fuse_signals(&signals);
        
        // Stocker dans le cache
        if !signals.is_empty() {
            self.active_signals.insert(symbol.to_string(), signals.clone());
        }
        
        if let Some(signal) = fused_signal {
            vec![signal]
        } else {
            signals
        }
    }
    
    /// Analyse un timeframe spécifique
    fn analyze_timeframe(
        &self, 
        symbol: &str, 
        timeframe: Timeframe, 
        price: f64, 
        volume: f64
    ) -> Option<TradingSignal> {
        // Calculer les indicateurs (simplifié)
        let indicators = self.calculate_indicators(symbol, timeframe, price, volume);
        
        // Générer un signal basé sur les indicateurs
        let (action, confidence, reason) = self.generate_signal_from_indicators(&indicators, price);
        
        if confidence < 60.0 {
            return None;  // Pas assez confiant
        }
        
        // Calculer stop loss et take profit
        let stop_loss = match action {
            SignalAction::BuyLong => Some(price * 0.98),  // 2% stop
            SignalAction::SellShort => Some(price * 1.02),
            _ => None,
        };
        
        let take_profit = match action {
            SignalAction::BuyLong => Some(price * 1.05),  // 5% target
            SignalAction::SellShort => Some(price * 0.95),
            _ => None,
        };
        
        Some(TradingSignal {
            symbol: symbol.to_string(),
            action,
            timeframe,
            confidence,
            supporting_indicators: indicators,
            entry_price: price,
            stop_loss,
            take_profit,
            reason,
            timestamp: Self::timestamp_micros(),
        })
    }
    
    /// Calcule les indicateurs techniques
    fn calculate_indicators(
        &self,
        _symbol: &str,
        _timeframe: Timeframe,
        price: f64,
        volume: f64,
    ) -> Vec<Indicator> {
        let mut indicators = SmallVec::<[Indicator; 10]>::new();
        
        // RSI (simplifié - en production: calculer avec historique réel)
        let rsi = 50.0 + ((price % 100.0) - 50.0);
        indicators.push(Indicator::RSI { 
            period: 14, 
            value: rsi 
        });
        
        // MACD (simplifié)
        indicators.push(Indicator::MACD {
            fast: price * 0.99,
            slow: price * 0.98,
            signal: price * 0.985,
        });
        
        // VWAP (simplifié)
        indicators.push(Indicator::VWAP {
            value: price,
        });
        
        // EMA (simplifié)
        indicators.push(Indicator::EMA {
            period: 20,
            value: price * 0.995,
        });
        
        // Volume (OBV)
        indicators.push(Indicator::OBV {
            value: volume,
        });
        
        indicators.into_vec()
    }
    
    /// Génère un signal à partir des indicateurs
    fn generate_signal_from_indicators(
        &self,
        indicators: &[Indicator],
        price: f64,
    ) -> (SignalAction, f64, String) {
        let mut bullish_score: f64 = 0.0;
        let mut bearish_score: f64 = 0.0;
        let mut reasons = Vec::new();
        
        for indicator in indicators {
            match indicator {
                Indicator::RSI { value, .. } => {
                    if *value < 30.0 {
                        bullish_score += 20.0;
                        reasons.push("RSI oversold".to_string());
                    } else if *value > 70.0 {
                        bearish_score += 20.0;
                        reasons.push("RSI overbought".to_string());
                    }
                }
                Indicator::MACD { fast, slow, .. } => {
                    if fast > slow {
                        bullish_score += 15.0;
                        reasons.push("MACD bullish crossover".to_string());
                    } else {
                        bearish_score += 15.0;
                        reasons.push("MACD bearish crossover".to_string());
                    }
                }
                Indicator::EMA { value, .. } => {
                    if price > *value {
                        bullish_score += 10.0;
                        reasons.push("Price above EMA".to_string());
                    } else {
                        bearish_score += 10.0;
                        reasons.push("Price below EMA".to_string());
                    }
                }
                _ => {}
            }
        }
        
        // Déterminer l'action et la confiance
        let action;
        let confidence;
        let reason;
        
        if bullish_score > bearish_score && bullish_score > 30.0 {
            action = SignalAction::BuyLong;
            confidence = bullish_score.min(100.0);
            reason = reasons.join(", ");
        } else if bearish_score > bullish_score && bearish_score > 30.0 {
            action = SignalAction::SellShort;
            confidence = bearish_score.min(100.0);
            reason = reasons.join(", ");
        } else {
            action = SignalAction::Hold;
            confidence = 50.0;
            reason = "No clear signal".to_string();
        }
        
        (action, confidence, reason)
    }
    
    /// Fusionne plusieurs signaux de différents timeframes
    fn fuse_signals(&self, signals: &[TradingSignal]) -> Option<TradingSignal> {
        if signals.is_empty() {
            return None;
        }
        
        // Calculer le consensus
        let mut buy_count = 0;
        let mut sell_count = 0;
        let mut total_confidence = 0.0;
        
        for signal in signals {
            match signal.action {
                SignalAction::BuyLong => buy_count += 1,
                SignalAction::SellShort => sell_count += 1,
                _ => {}
            }
            total_confidence += signal.confidence;
        }
        
        let avg_confidence = total_confidence / signals.len() as f64;
        
        // Consensus: au moins 60% des signaux doivent être d'accord
        let total_signals = signals.len();
        let consensus_threshold = (total_signals as f64 * 0.6) as usize;
        
        if buy_count >= consensus_threshold {
            let first_signal = &signals[0];
            Some(TradingSignal {
                symbol: first_signal.symbol.clone(),
                action: SignalAction::BuyLong,
                timeframe: Timeframe::Day1,  // Signal fusionné = higher timeframe
                confidence: avg_confidence,
                supporting_indicators: Vec::new(),
                entry_price: first_signal.entry_price,
                stop_loss: first_signal.stop_loss,
                take_profit: first_signal.take_profit,
                reason: format!("Multi-timeframe consensus: {} buy signals", buy_count),
                timestamp: Self::timestamp_micros(),
            })
        } else if sell_count >= consensus_threshold {
            let first_signal = &signals[0];
            Some(TradingSignal {
                symbol: first_signal.symbol.clone(),
                action: SignalAction::SellShort,
                timeframe: Timeframe::Day1,
                confidence: avg_confidence,
                supporting_indicators: Vec::new(),
                entry_price: first_signal.entry_price,
                stop_loss: first_signal.stop_loss,
                take_profit: first_signal.take_profit,
                reason: format!("Multi-timeframe consensus: {} sell signals", sell_count),
                timestamp: Self::timestamp_micros(),
            })
        } else {
            None  // Pas de consensus
        }
    }
    
    /// Obtient les signaux actifs pour un symbole
    pub fn get_active_signals(&self, symbol: &str) -> Option<&Vec<TradingSignal>> {
        self.active_signals.get(symbol)
    }
    
    /// Timestamp en microsecondes
    #[inline(always)]
    fn timestamp_micros() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64
    }
}

impl Default for SignalEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signal_engine() {
        let mut engine = SignalEngine::new();
        let signals = engine.analyze_multi_timeframe("BTC", 50000.0, 1000000.0);
        
        // Devrait générer au moins quelques signaux
        assert!(!signals.is_empty());
    }
    
    #[test]
    fn test_indicator_calculation() {
        let engine = SignalEngine::new();
        let indicators = engine.calculate_indicators("ETH", Timeframe::Hour1, 3000.0, 500000.0);
        
        assert!(!indicators.is_empty());
        
        // Vérifier qu'on a différents types d'indicateurs
        let has_rsi = indicators.iter().any(|i| matches!(i, Indicator::RSI { .. }));
        let has_macd = indicators.iter().any(|i| matches!(i, Indicator::MACD { .. }));
        
        assert!(has_rsi);
        assert!(has_macd);
    }
}
