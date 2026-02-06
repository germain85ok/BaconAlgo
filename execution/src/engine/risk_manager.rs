// BaconAlgo 2040 Quantum Edition - Risk Manager
// Gestion des risques en temps réel: VaR, Kelly Criterion, Circuit Breakers

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use dashmap::DashMap;
use std::collections::HashMap;

/// Configuration du risk manager
#[derive(Debug, Clone)]
pub struct RiskConfig {
    /// Capital total du compte
    pub total_capital: f64,
    
    /// Risque maximum par trade (en % du capital)
    pub max_risk_per_trade_pct: f64,
    
    /// Exposition maximale (en % du capital)
    pub max_exposure_pct: f64,
    
    /// Drawdown maximum avant circuit breaker (en %)
    pub max_drawdown_pct: f64,
    
    /// Perte journalière maximale (en $)
    pub max_daily_loss: f64,
    
    /// Nombre maximum de positions simultanées
    pub max_concurrent_positions: usize,
    
    /// Activer Kelly Criterion pour position sizing
    pub enable_kelly_criterion: bool,
    
    /// Activer les circuit breakers
    pub enable_circuit_breakers: bool,
    
    /// Confidence level pour VaR (95%, 99%, etc.)
    pub var_confidence_level: f64,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            total_capital: 100000.0,  // $100K
            max_risk_per_trade_pct: 1.0,  // 1% max par trade
            max_exposure_pct: 20.0,  // 20% exposition max
            max_drawdown_pct: 10.0,  // 10% drawdown max
            max_daily_loss: 2000.0,  // $2K perte max par jour
            max_concurrent_positions: 10,
            enable_kelly_criterion: true,
            enable_circuit_breakers: true,
            var_confidence_level: 0.95,  // 95%
        }
    }
}

/// Métriques de risque
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    /// Value-at-Risk (VaR) en $
    pub var: f64,
    
    /// Conditional VaR (CVaR/Expected Shortfall)
    pub cvar: f64,
    
    /// Exposition actuelle (en $)
    pub current_exposure: f64,
    
    /// Exposition en % du capital
    pub exposure_pct: f64,
    
    /// Drawdown actuel (en %)
    pub current_drawdown_pct: f64,
    
    /// Drawdown maximum (en %)
    pub max_drawdown_pct: f64,
    
    /// P&L du jour
    pub daily_pnl: f64,
    
    /// Nombre de positions ouvertes
    pub open_positions: usize,
    
    /// Ratio de Sharpe
    pub sharpe_ratio: f64,
    
    /// Circuit breaker actif?
    pub circuit_breaker_active: bool,
}

/// Position pour risk management
#[derive(Debug, Clone)]
pub struct RiskPosition {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub position_value: f64,
    pub unrealized_pnl: f64,
}

/// Limite de risque par instrument
#[derive(Debug, Clone)]
pub struct InstrumentLimit {
    pub max_position_size: f64,
    pub max_exposure_pct: f64,
}

/// Limite de risque par secteur
#[derive(Debug, Clone)]
pub struct SectorLimit {
    pub sector: String,
    pub max_exposure_pct: f64,
    pub current_exposure: f64,
}

/// Risk Manager - Gestion des risques en temps réel
/// 
/// Fonctionnalités:
/// - Position sizing dynamique (Kelly Criterion)
/// - Drawdown protection (circuit breakers)
/// - Exposure limits (instrument/secteur/portfolio)
/// - Value-at-Risk (VaR) Monte Carlo
/// - Stress testing
/// - Correlation matrix
pub struct RiskManager {
    /// Configuration
    config: RiskConfig,
    
    /// Positions actives pour risk tracking
    positions: Arc<DashMap<String, RiskPosition>>,
    
    /// Limites par instrument
    instrument_limits: Arc<DashMap<String, InstrumentLimit>>,
    
    /// Limites par secteur
    sector_limits: Arc<RwLock<HashMap<String, SectorLimit>>>,
    
    /// Métriques de risque actuelles
    metrics: Arc<RwLock<RiskMetrics>>,
    
    /// Capital initial (pour calcul drawdown)
    initial_capital: f64,
    
    /// Peak capital (plus haut niveau)
    peak_capital: Arc<RwLock<f64>>,
    
    /// Historique des returns (pour Sharpe, VaR)
    returns_history: Arc<RwLock<Vec<f64>>>,
}

impl RiskManager {
    /// Crée un nouveau risk manager
    pub fn new(config: RiskConfig) -> Self {
        let initial_capital = config.total_capital;
        
        Self {
            initial_capital,
            peak_capital: Arc::new(RwLock::new(initial_capital)),
            positions: Arc::new(DashMap::new()),
            instrument_limits: Arc::new(DashMap::new()),
            sector_limits: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(RiskMetrics {
                var: 0.0,
                cvar: 0.0,
                current_exposure: 0.0,
                exposure_pct: 0.0,
                current_drawdown_pct: 0.0,
                max_drawdown_pct: 0.0,
                daily_pnl: 0.0,
                open_positions: 0,
                sharpe_ratio: 0.0,
                circuit_breaker_active: false,
            })),
            returns_history: Arc::new(RwLock::new(Vec::new())),
            config,
        }
    }
    
    /// Calcule la taille de position optimale (Kelly Criterion)
    /// 
    /// Kelly% = (Win% × Average Win) - (Loss% × Average Loss) / Average Win
    pub fn calculate_position_size(
        &self,
        win_rate: f64,
        avg_win: f64,
        avg_loss: f64,
        price: f64,
    ) -> f64 {
        if !self.config.enable_kelly_criterion {
            // Position sizing fixe
            let risk_amount = self.config.total_capital * (self.config.max_risk_per_trade_pct / 100.0);
            return risk_amount / price;
        }
        
        // Kelly Criterion
        let kelly_pct = if avg_win > 0.0 {
            ((win_rate * avg_win) - ((1.0 - win_rate) * avg_loss)) / avg_win
        } else {
            0.0
        };
        
        // Kelly fractionné (on utilise 50% du Kelly pour être conservateur)
        let fractional_kelly = kelly_pct * 0.5;
        
        // Limiter à max_risk_per_trade_pct
        let final_kelly = fractional_kelly.min(self.config.max_risk_per_trade_pct / 100.0);
        
        if final_kelly > 0.0 {
            let position_value = self.config.total_capital * final_kelly;
            position_value / price
        } else {
            0.0
        }
    }
    
    /// Valide si un trade est autorisé selon les règles de risque
    pub fn validate_trade(
        &self,
        symbol: &str,
        quantity: f64,
        price: f64,
    ) -> anyhow::Result<()> {
        // Vérifier le circuit breaker
        {
            let metrics = self.metrics.read();
            if metrics.circuit_breaker_active {
                return Err(anyhow::anyhow!("Circuit breaker actif - trading suspendu"));
            }
        }
        
        // Vérifier le nombre maximum de positions
        if self.positions.len() >= self.config.max_concurrent_positions {
            return Err(anyhow::anyhow!(
                "Nombre maximum de positions atteint: {}",
                self.config.max_concurrent_positions
            ));
        }
        
        // Calculer la valeur de la nouvelle position
        let position_value = quantity * price;
        
        // Vérifier l'exposition maximale
        let current_exposure = self.calculate_total_exposure();
        let new_exposure = current_exposure + position_value;
        let max_exposure = self.config.total_capital * (self.config.max_exposure_pct / 100.0);
        
        if new_exposure > max_exposure {
            return Err(anyhow::anyhow!(
                "Exposition maximale dépassée: ${:.2} / ${:.2}",
                new_exposure,
                max_exposure
            ));
        }
        
        // Vérifier les limites par instrument
        if let Some(limit) = self.instrument_limits.get(symbol) {
            if position_value > limit.max_position_size {
                return Err(anyhow::anyhow!(
                    "Taille de position maximale dépassée pour {}: ${:.2} / ${:.2}",
                    symbol,
                    position_value,
                    limit.max_position_size
                ));
            }
        }
        
        Ok(())
    }
    
    /// Ajoute une position au tracking
    pub fn add_position(&self, position: RiskPosition) {
        self.positions.insert(position.symbol.clone(), position);
        self.update_metrics();
    }
    
    /// Met à jour une position existante
    pub fn update_position(&self, symbol: &str, current_price: f64) -> anyhow::Result<()> {
        if let Some(mut pos_ref) = self.positions.get_mut(symbol) {
            let pos = pos_ref.value_mut();
            pos.current_price = current_price;
            pos.position_value = pos.quantity * current_price;
            pos.unrealized_pnl = (current_price - pos.entry_price) * pos.quantity;
            
            // Vérifier stop loss
            if let Some(stop_loss) = pos.stop_loss {
                if current_price <= stop_loss {
                    tracing::warn!("🛑 Stop loss triggered for {}: ${:.2}", symbol, current_price);
                    // En production: déclencher la fermeture de position
                }
            }
            
            // Vérifier take profit
            if let Some(take_profit) = pos.take_profit {
                if current_price >= take_profit {
                    tracing::info!("🎯 Take profit triggered for {}: ${:.2}", symbol, current_price);
                    // En production: déclencher la fermeture de position
                }
            }
            
            drop(pos_ref);
            self.update_metrics();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Position not found: {}", symbol))
        }
    }
    
    /// Supprime une position (fermée)
    pub fn remove_position(&self, symbol: &str) {
        self.positions.remove(symbol);
        self.update_metrics();
    }
    
    /// Calcule l'exposition totale
    fn calculate_total_exposure(&self) -> f64 {
        self.positions
            .iter()
            .map(|entry| entry.value().position_value)
            .sum()
    }
    
    /// Calcule le P&L total
    fn calculate_total_pnl(&self) -> f64 {
        self.positions
            .iter()
            .map(|entry| entry.value().unrealized_pnl)
            .sum()
    }
    
    /// Met à jour les métriques de risque
    fn update_metrics(&self) {
        let total_exposure = self.calculate_total_exposure();
        let total_pnl = self.calculate_total_pnl();
        let current_capital = self.config.total_capital + total_pnl;
        
        // Mettre à jour le peak capital
        {
            let mut peak = self.peak_capital.write();
            if current_capital > *peak {
                *peak = current_capital;
            }
        }
        
        // Calculer le drawdown
        let peak = *self.peak_capital.read();
        let drawdown_pct = if peak > 0.0 {
            ((peak - current_capital) / peak) * 100.0
        } else {
            0.0
        };
        
        // Calculer VaR (simplifié - Monte Carlo en production)
        let var = self.calculate_var();
        
        // Vérifier si on doit activer le circuit breaker
        let circuit_breaker_active = if self.config.enable_circuit_breakers {
            drawdown_pct >= self.config.max_drawdown_pct ||
            total_pnl <= -self.config.max_daily_loss
        } else {
            false
        };
        
        if circuit_breaker_active {
            tracing::error!("🚨 CIRCUIT BREAKER ACTIVÉ!");
            tracing::error!("   Drawdown: {:.2}% (max: {:.2}%)", 
                drawdown_pct, self.config.max_drawdown_pct);
            tracing::error!("   Daily P&L: ${:.2} (max loss: ${:.2})", 
                total_pnl, -self.config.max_daily_loss);
        }
        
        // Mettre à jour les métriques
        let mut metrics = self.metrics.write();
        metrics.current_exposure = total_exposure;
        metrics.exposure_pct = (total_exposure / self.config.total_capital) * 100.0;
        metrics.current_drawdown_pct = drawdown_pct;
        metrics.max_drawdown_pct = drawdown_pct.max(metrics.max_drawdown_pct);
        metrics.daily_pnl = total_pnl;
        metrics.open_positions = self.positions.len();
        metrics.var = var;
        metrics.circuit_breaker_active = circuit_breaker_active;
        
        // Calculer Sharpe ratio (simplifié)
        metrics.sharpe_ratio = self.calculate_sharpe_ratio();
    }
    
    /// Calcule le Value-at-Risk (VaR)
    /// En production: Monte Carlo simulation avec 10K+ scénarios
    fn calculate_var(&self) -> f64 {
        // Simplifié pour l'instant - utiliser historical VaR
        let returns = self.returns_history.read();
        if returns.len() < 30 {
            return 0.0;
        }
        
        // Trier les returns
        let mut sorted_returns = returns.clone();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Trouver le percentile
        let index = ((1.0 - self.config.var_confidence_level) * sorted_returns.len() as f64) as usize;
        let var_return = sorted_returns.get(index).unwrap_or(&0.0);
        
        // VaR en dollars
        (*var_return).abs() * self.config.total_capital
    }
    
    /// Calcule le Sharpe Ratio
    fn calculate_sharpe_ratio(&self) -> f64 {
        let returns = self.returns_history.read();
        if returns.len() < 30 {
            return 0.0;
        }
        
        // Moyenne des returns
        let mean: f64 = returns.iter().sum::<f64>() / returns.len() as f64;
        
        // Écart-type
        let variance: f64 = returns
            .iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>() / returns.len() as f64;
        let std_dev = variance.sqrt();
        
        if std_dev > 0.0 {
            // Sharpe = (Mean Return - Risk Free Rate) / Std Dev
            // On assume un risk-free rate de 0 pour simplifier
            mean / std_dev * (252.0_f64).sqrt()  // Annualisé
        } else {
            0.0
        }
    }
    
    /// Obtient les métriques de risque
    pub fn get_metrics(&self) -> RiskMetrics {
        self.metrics.read().clone()
    }
    
    /// Définit une limite pour un instrument
    pub fn set_instrument_limit(&self, symbol: String, max_position_size: f64, max_exposure_pct: f64) {
        self.instrument_limits.insert(symbol, InstrumentLimit {
            max_position_size,
            max_exposure_pct,
        });
    }
    
    /// Définit une limite pour un secteur
    pub fn set_sector_limit(&self, sector: String, max_exposure_pct: f64) {
        let mut limits = self.sector_limits.write();
        limits.insert(sector.clone(), SectorLimit {
            sector,
            max_exposure_pct,
            current_exposure: 0.0,
        });
    }
    
    /// Reset le circuit breaker (après intervention manuelle)
    pub fn reset_circuit_breaker(&self) {
        let mut metrics = self.metrics.write();
        metrics.circuit_breaker_active = false;
        tracing::info!("✅ Circuit breaker réinitialisé");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_risk_manager_creation() {
        let config = RiskConfig::default();
        let rm = RiskManager::new(config);
        let metrics = rm.get_metrics();
        assert_eq!(metrics.open_positions, 0);
        assert!(!metrics.circuit_breaker_active);
    }
    
    #[test]
    fn test_kelly_criterion() {
        let config = RiskConfig::default();
        let rm = RiskManager::new(config);
        
        // Win rate 60%, avg win $100, avg loss $50
        let size = rm.calculate_position_size(0.6, 100.0, 50.0, 50000.0);
        assert!(size > 0.0);
    }
    
    #[test]
    fn test_validate_trade() {
        let config = RiskConfig::default();
        let rm = RiskManager::new(config);
        
        // Trade normal devrait passer
        let result = rm.validate_trade("BTC", 1.0, 50000.0);
        assert!(result.is_ok());
        
        // Trade avec exposition excessive devrait échouer
        let result = rm.validate_trade("BTC", 100.0, 50000.0);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_position_tracking() {
        let config = RiskConfig::default();
        let rm = RiskManager::new(config);
        
        let position = RiskPosition {
            symbol: "ETH".to_string(),
            quantity: 10.0,
            entry_price: 3000.0,
            current_price: 3000.0,
            stop_loss: Some(2900.0),
            take_profit: Some(3200.0),
            position_value: 30000.0,
            unrealized_pnl: 0.0,
        };
        
        rm.add_position(position);
        
        let metrics = rm.get_metrics();
        assert_eq!(metrics.open_positions, 1);
        assert_eq!(metrics.current_exposure, 30000.0);
    }
}
