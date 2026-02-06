use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// Position information
#[derive(Clone, Debug)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub avg_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
}

impl Position {
    pub fn market_value(&self) -> f64 {
        self.quantity * self.current_price
    }

    pub fn update_pnl(&mut self) {
        self.unrealized_pnl = (self.current_price - self.avg_price) * self.quantity;
    }
}

/// Portfolio metrics
#[derive(Clone, Debug)]
pub struct PortfolioMetrics {
    pub total_value: f64,
    pub total_pnl: f64,
    pub total_return: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub calmar_ratio: f64,
    pub max_drawdown: f64,
    pub current_drawdown: f64,
    pub var_95: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
}

impl Default for PortfolioMetrics {
    fn default() -> Self {
        Self {
            total_value: 0.0,
            total_pnl: 0.0,
            total_return: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            calmar_ratio: 0.0,
            max_drawdown: 0.0,
            current_drawdown: 0.0,
            var_95: 0.0,
            win_rate: 0.0,
            profit_factor: 0.0,
        }
    }
}

/// Risk limits
#[derive(Clone, Debug)]
pub struct RiskLimits {
    pub max_position_size: f64,
    pub max_portfolio_leverage: f64,
    pub max_drawdown_pct: f64,
    pub max_daily_loss: f64,
    pub max_concentration: f64, // Max % in single position
    pub min_sharpe_ratio: f64,
}

impl Default for RiskLimits {
    fn default() -> Self {
        Self {
            max_position_size: 100000.0,
            max_portfolio_leverage: 2.0,
            max_drawdown_pct: 20.0,
            max_daily_loss: 5000.0,
            max_concentration: 25.0,
            min_sharpe_ratio: 1.0,
        }
    }
}

/// Circuit breaker state
#[derive(Clone, Debug, PartialEq)]
pub enum CircuitBreakerState {
    Normal,
    Warning,
    Triggered,
}

/// Risk manager with Kelly Criterion and real-time risk metrics
pub struct RiskManager {
    /// Current positions
    positions: Arc<RwLock<HashMap<String, Position>>>,
    /// Risk limits
    limits: Arc<RwLock<RiskLimits>>,
    /// Circuit breaker state
    circuit_breaker: Arc<RwLock<CircuitBreakerState>>,
    /// Portfolio equity curve for metrics calculation
    equity_curve: Arc<RwLock<Vec<f64>>>,
    /// Daily returns for risk calculations
    daily_returns: Arc<RwLock<Vec<f64>>>,
    /// Trade history
    trade_history: Arc<RwLock<Vec<TradeRecord>>>,
    /// Initial capital
    initial_capital: f64,
}

#[derive(Clone, Debug)]
pub struct TradeRecord {
    pub symbol: String,
    pub pnl: f64,
    pub return_pct: f64,
    pub timestamp: u64,
}

impl RiskManager {
    pub fn new(initial_capital: f64) -> Self {
        Self {
            positions: Arc::new(RwLock::new(HashMap::new())),
            limits: Arc::new(RwLock::new(RiskLimits::default())),
            circuit_breaker: Arc::new(RwLock::new(CircuitBreakerState::Normal)),
            equity_curve: Arc::new(RwLock::new(vec![initial_capital])),
            daily_returns: Arc::new(RwLock::new(Vec::new())),
            trade_history: Arc::new(RwLock::new(Vec::new())),
            initial_capital,
        }
    }

    /// Calculate position size using Kelly Criterion
    pub fn calculate_kelly_position_size(
        &self,
        win_rate: f64,
        avg_win: f64,
        avg_loss: f64,
        capital: f64,
    ) -> f64 {
        if avg_loss == 0.0 {
            return 0.0;
        }

        let win_loss_ratio = avg_win / avg_loss.abs();
        let kelly_fraction = (win_rate * win_loss_ratio - (1.0 - win_rate)) / win_loss_ratio;

        // Use half-Kelly for safety
        let safe_kelly = (kelly_fraction * 0.5).max(0.0).min(0.25);
        
        capital * safe_kelly
    }

    /// Check if order is allowed based on risk limits
    pub fn check_order_risk(&self, _symbol: &str, quantity: f64, price: f64) -> Result<(), RiskError> {
        // Check circuit breaker
        let breaker_state = self.circuit_breaker.read();
        if *breaker_state == CircuitBreakerState::Triggered {
            return Err(RiskError::CircuitBreakerTriggered);
        }

        let limits = self.limits.read();
        let order_value = quantity.abs() * price;

        // Check position size limit
        if order_value > limits.max_position_size {
            return Err(RiskError::PositionSizeLimitExceeded);
        }

        // Check concentration
        let metrics = self.calculate_metrics();
        if metrics.total_value > 0.0 {
            let position_pct = (order_value / metrics.total_value) * 100.0;
            if position_pct > limits.max_concentration {
                return Err(RiskError::ConcentrationLimitExceeded);
            }
        }

        Ok(())
    }

    /// Update position
    pub fn update_position(&self, symbol: String, quantity: f64, price: f64) {
        let mut positions = self.positions.write();
        
        if let Some(pos) = positions.get_mut(&symbol) {
            let total_cost = pos.avg_price * pos.quantity + price * quantity;
            pos.quantity += quantity;
            if pos.quantity != 0.0 {
                pos.avg_price = total_cost / pos.quantity;
            }
            pos.current_price = price;
            pos.update_pnl();
        } else {
            positions.insert(
                symbol.clone(),
                Position {
                    symbol,
                    quantity,
                    avg_price: price,
                    current_price: price,
                    unrealized_pnl: 0.0,
                    realized_pnl: 0.0,
                },
            );
        }
    }

    /// Calculate real-time VaR (95% confidence)
    pub fn calculate_var(&self, confidence: f64) -> f64 {
        let returns = self.daily_returns.read();
        if returns.len() < 2 {
            return 0.0;
        }

        let mut sorted_returns = returns.clone();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = ((1.0 - confidence) * sorted_returns.len() as f64) as usize;
        sorted_returns.get(index).copied().unwrap_or(0.0)
    }

    /// Calculate Sharpe ratio
    pub fn calculate_sharpe(&self, risk_free_rate: f64) -> f64 {
        let returns = self.daily_returns.read();
        if returns.len() < 2 {
            return 0.0;
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns
            .iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>()
            / returns.len() as f64;
        
        let std_dev = variance.sqrt();
        if std_dev == 0.0 {
            return 0.0;
        }

        (mean_return - risk_free_rate) / std_dev * (252.0_f64).sqrt()
    }

    /// Calculate Sortino ratio (downside deviation)
    pub fn calculate_sortino(&self, risk_free_rate: f64) -> f64 {
        let returns = self.daily_returns.read();
        if returns.len() < 2 {
            return 0.0;
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        
        let downside_returns: Vec<f64> = returns
            .iter()
            .filter(|&&r| r < 0.0)
            .copied()
            .collect();

        if downside_returns.is_empty() {
            return f64::INFINITY;
        }

        let downside_variance = downside_returns
            .iter()
            .map(|r| r.powi(2))
            .sum::<f64>()
            / downside_returns.len() as f64;
        
        let downside_dev = downside_variance.sqrt();
        if downside_dev == 0.0 {
            return 0.0;
        }

        (mean_return - risk_free_rate) / downside_dev * (252.0_f64).sqrt()
    }

    /// Calculate Calmar ratio (return / max drawdown)
    pub fn calculate_calmar(&self) -> f64 {
        let equity = self.equity_curve.read();
        if equity.len() < 2 {
            return 0.0;
        }

        let returns = self.daily_returns.read();
        let annual_return = returns.iter().sum::<f64>() * 252.0 / returns.len() as f64;
        
        let max_dd = self.calculate_max_drawdown();
        if max_dd == 0.0 {
            return 0.0;
        }

        annual_return / max_dd.abs()
    }

    /// Calculate maximum drawdown
    pub fn calculate_max_drawdown(&self) -> f64 {
        let equity = self.equity_curve.read();
        if equity.is_empty() {
            return 0.0;
        }

        let mut max_value = equity[0];
        let mut max_dd = 0.0;

        for &value in equity.iter() {
            if value > max_value {
                max_value = value;
            }
            let dd = (value - max_value) / max_value * 100.0;
            if dd < max_dd {
                max_dd = dd;
            }
        }

        max_dd
    }

    /// Update equity and check circuit breaker
    pub fn update_equity(&self, current_equity: f64) {
        let mut equity_curve = self.equity_curve.write();
        
        if let Some(&last_equity) = equity_curve.last() {
            let daily_return = (current_equity - last_equity) / last_equity;
            self.daily_returns.write().push(daily_return);
        }
        
        equity_curve.push(current_equity);

        // Check drawdown and trigger circuit breaker if needed
        let current_dd = (current_equity - self.initial_capital) / self.initial_capital * 100.0;
        let limits = self.limits.read();
        
        let mut breaker = self.circuit_breaker.write();
        if current_dd.abs() >= limits.max_drawdown_pct {
            *breaker = CircuitBreakerState::Triggered;
            tracing::error!("Circuit breaker TRIGGERED! Drawdown: {:.2}%", current_dd);
        } else if current_dd.abs() >= limits.max_drawdown_pct * 0.75 {
            *breaker = CircuitBreakerState::Warning;
            tracing::warn!("Circuit breaker WARNING! Drawdown: {:.2}%", current_dd);
        } else {
            *breaker = CircuitBreakerState::Normal;
        }
    }

    /// Calculate comprehensive portfolio metrics
    pub fn calculate_metrics(&self) -> PortfolioMetrics {
        let positions = self.positions.read();
        let equity = self.equity_curve.read();
        
        let total_value = positions.values().map(|p| p.market_value()).sum::<f64>();
        let total_pnl = positions.values().map(|p| p.unrealized_pnl + p.realized_pnl).sum::<f64>();
        
        let total_return = if self.initial_capital > 0.0 {
            (total_pnl / self.initial_capital) * 100.0
        } else {
            0.0
        };

        let sharpe_ratio = self.calculate_sharpe(0.02); // 2% risk-free rate
        let sortino_ratio = self.calculate_sortino(0.02);
        let calmar_ratio = self.calculate_calmar();
        let max_drawdown = self.calculate_max_drawdown();
        
        let current_drawdown = if let Some(&last_equity) = equity.last() {
            (last_equity - self.initial_capital) / self.initial_capital * 100.0
        } else {
            0.0
        };

        let var_95 = self.calculate_var(0.95);

        // Calculate win rate and profit factor
        let trades = self.trade_history.read();
        let winning_trades = trades.iter().filter(|t| t.pnl > 0.0).count();
        let win_rate = if !trades.is_empty() {
            (winning_trades as f64 / trades.len() as f64) * 100.0
        } else {
            0.0
        };

        let gross_profit: f64 = trades.iter().filter(|t| t.pnl > 0.0).map(|t| t.pnl).sum();
        let gross_loss: f64 = trades.iter().filter(|t| t.pnl < 0.0).map(|t| t.pnl.abs()).sum();
        let profit_factor = if gross_loss > 0.0 {
            gross_profit / gross_loss
        } else {
            0.0
        };

        PortfolioMetrics {
            total_value,
            total_pnl,
            total_return,
            sharpe_ratio,
            sortino_ratio,
            calmar_ratio,
            max_drawdown,
            current_drawdown,
            var_95,
            win_rate,
            profit_factor,
        }
    }

    /// Reset circuit breaker (manual intervention)
    pub fn reset_circuit_breaker(&self) {
        let mut breaker = self.circuit_breaker.write();
        *breaker = CircuitBreakerState::Normal;
        tracing::info!("Circuit breaker manually reset");
    }

    /// Get circuit breaker state
    pub fn get_circuit_breaker_state(&self) -> CircuitBreakerState {
        self.circuit_breaker.read().clone()
    }
}

#[derive(Debug, Clone)]
pub enum RiskError {
    CircuitBreakerTriggered,
    PositionSizeLimitExceeded,
    ConcentrationLimitExceeded,
    DrawdownLimitExceeded,
    DailyLossLimitExceeded,
}

impl std::fmt::Display for RiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskError::CircuitBreakerTriggered => write!(f, "Circuit breaker is triggered"),
            RiskError::PositionSizeLimitExceeded => write!(f, "Position size limit exceeded"),
            RiskError::ConcentrationLimitExceeded => write!(f, "Concentration limit exceeded"),
            RiskError::DrawdownLimitExceeded => write!(f, "Drawdown limit exceeded"),
            RiskError::DailyLossLimitExceeded => write!(f, "Daily loss limit exceeded"),
        }
    }
}

impl std::error::Error for RiskError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kelly_criterion() {
        let rm = RiskManager::new(100000.0);
        let size = rm.calculate_kelly_position_size(0.6, 1000.0, 500.0, 100000.0);
        assert!(size > 0.0);
        assert!(size <= 25000.0); // Max 25% of capital with half-Kelly
    }

    #[test]
    fn test_circuit_breaker() {
        let rm = RiskManager::new(100000.0);
        assert_eq!(rm.get_circuit_breaker_state(), CircuitBreakerState::Normal);
        
        // Simulate large loss
        rm.update_equity(75000.0); // 25% drawdown
        assert_eq!(rm.get_circuit_breaker_state(), CircuitBreakerState::Triggered);
    }
}
