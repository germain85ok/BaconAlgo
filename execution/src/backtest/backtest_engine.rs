use ndarray::Array1;
use rayon::prelude::*;

/// Backtest configuration
#[derive(Clone, Debug)]
pub struct BacktestConfig {
    pub initial_capital: f64,
    pub commission_rate: f64,
    pub slippage_pct: f64,
    pub position_size_pct: f64,
    pub max_positions: usize,
}

impl Default for BacktestConfig {
    fn default() -> Self {
        Self {
            initial_capital: 100000.0,
            commission_rate: 0.001, // 0.1%
            slippage_pct: 0.001,    // 0.1%
            position_size_pct: 0.1, // 10% per position
            max_positions: 10,
        }
    }
}

/// Trade in backtest
#[derive(Clone, Debug)]
pub struct BacktestTrade {
    pub symbol: String,
    pub entry_time: u64,
    pub exit_time: u64,
    pub entry_price: f64,
    pub exit_price: f64,
    pub quantity: f64,
    pub side: TradeSide,
    pub pnl: f64,
    pub return_pct: f64,
    pub commission: f64,
    pub slippage: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TradeSide {
    Long,
    Short,
}

/// Backtest metrics
#[derive(Clone, Debug)]
pub struct BacktestMetrics {
    pub total_return: f64,
    pub annual_return: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub calmar_ratio: f64,
    pub max_drawdown: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub largest_win: f64,
    pub largest_loss: f64,
    pub avg_trade_duration_hours: f64,
    pub final_capital: f64,
}

impl Default for BacktestMetrics {
    fn default() -> Self {
        Self {
            total_return: 0.0,
            annual_return: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            calmar_ratio: 0.0,
            max_drawdown: 0.0,
            win_rate: 0.0,
            profit_factor: 0.0,
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            avg_win: 0.0,
            avg_loss: 0.0,
            largest_win: 0.0,
            largest_loss: 0.0,
            avg_trade_duration_hours: 0.0,
            final_capital: 0.0,
        }
    }
}

/// Monte Carlo simulation result
#[derive(Clone, Debug)]
pub struct MonteCarloResult {
    pub mean_return: f64,
    pub median_return: f64,
    pub std_return: f64,
    pub var_95: f64,
    pub best_case: f64,
    pub worst_case: f64,
    pub probability_profit: f64,
}

/// Backtest engine with vectorized calculations
pub struct BacktestEngine {
    config: BacktestConfig,
    trades: Vec<BacktestTrade>,
    equity_curve: Vec<f64>,
}

impl BacktestEngine {
    pub fn new(config: BacktestConfig) -> Self {
        let initial_capital = config.initial_capital;
        Self {
            config,
            trades: Vec::new(),
            equity_curve: vec![initial_capital],
        }
    }

    /// Execute a single trade with realistic fill simulation
    pub fn execute_trade(
        &mut self,
        symbol: String,
        entry_time: u64,
        exit_time: u64,
        entry_price: f64,
        exit_price: f64,
        side: TradeSide,
    ) {
        let current_capital = self.equity_curve.last().copied().unwrap_or(self.config.initial_capital);
        let position_value = current_capital * self.config.position_size_pct;
        
        // Apply slippage
        let slippage_entry = entry_price * self.config.slippage_pct;
        let slippage_exit = exit_price * self.config.slippage_pct;
        
        let actual_entry = match side {
            TradeSide::Long => entry_price + slippage_entry,
            TradeSide::Short => entry_price - slippage_entry,
        };
        
        let actual_exit = match side {
            TradeSide::Long => exit_price - slippage_exit,
            TradeSide::Short => exit_price + slippage_exit,
        };
        
        let quantity = position_value / actual_entry;
        
        // Calculate P&L
        let gross_pnl = match side {
            TradeSide::Long => (actual_exit - actual_entry) * quantity,
            TradeSide::Short => (actual_entry - actual_exit) * quantity,
        };
        
        let commission = position_value * self.config.commission_rate * 2.0; // Entry + exit
        let total_slippage = (slippage_entry + slippage_exit) * quantity;
        let net_pnl = gross_pnl - commission - total_slippage;
        let return_pct = (net_pnl / position_value) * 100.0;
        
        let trade = BacktestTrade {
            symbol,
            entry_time,
            exit_time,
            entry_price: actual_entry,
            exit_price: actual_exit,
            quantity,
            side,
            pnl: net_pnl,
            return_pct,
            commission,
            slippage: total_slippage,
        };
        
        self.trades.push(trade);
        
        // Update equity curve
        let new_capital = current_capital + net_pnl;
        self.equity_curve.push(new_capital);
    }

    /// Calculate comprehensive metrics
    pub fn calculate_metrics(&self) -> BacktestMetrics {
        if self.trades.is_empty() {
            return BacktestMetrics::default();
        }

        let final_capital = self.equity_curve.last().copied().unwrap_or(self.config.initial_capital);
        let total_return = ((final_capital - self.config.initial_capital) / self.config.initial_capital) * 100.0;
        
        // Calculate returns array for statistics
        let returns: Vec<f64> = self.trades.iter().map(|t| t.return_pct).collect();
        let returns_arr = Array1::from_vec(returns.clone());
        
        let mean_return = returns_arr.mean().unwrap_or(0.0);
        let std_return = returns_arr.std(0.0);
        
        // Sharpe ratio (assuming daily returns, annualized)
        let sharpe_ratio = if std_return > 0.0 {
            (mean_return / std_return) * (252.0_f64).sqrt()
        } else {
            0.0
        };
        
        // Sortino ratio (downside deviation)
        let downside_returns: Vec<f64> = returns.iter().filter(|&&r| r < 0.0).copied().collect();
        let sortino_ratio = if !downside_returns.is_empty() {
            let downside_arr = Array1::from_vec(downside_returns);
            let downside_std = downside_arr.std(0.0);
            if downside_std > 0.0 {
                (mean_return / downside_std) * (252.0_f64).sqrt()
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Max drawdown
        let max_drawdown = self.calculate_max_drawdown();
        
        // Calmar ratio
        let annual_return = total_return; // Simplified - would need time period for accurate calculation
        let calmar_ratio = if max_drawdown.abs() > 0.0 {
            annual_return / max_drawdown.abs()
        } else {
            0.0
        };
        
        // Win/loss statistics
        let winning_trades: Vec<&BacktestTrade> = self.trades.iter().filter(|t| t.pnl > 0.0).collect();
        let losing_trades: Vec<&BacktestTrade> = self.trades.iter().filter(|t| t.pnl <= 0.0).collect();
        
        let win_rate = (winning_trades.len() as f64 / self.trades.len() as f64) * 100.0;
        
        let gross_profit: f64 = winning_trades.iter().map(|t| t.pnl).sum();
        let gross_loss: f64 = losing_trades.iter().map(|t| t.pnl.abs()).sum();
        let profit_factor = if gross_loss > 0.0 {
            gross_profit / gross_loss
        } else {
            0.0
        };
        
        let avg_win = if !winning_trades.is_empty() {
            gross_profit / winning_trades.len() as f64
        } else {
            0.0
        };
        
        let avg_loss = if !losing_trades.is_empty() {
            -gross_loss / losing_trades.len() as f64
        } else {
            0.0
        };
        
        let largest_win = winning_trades.iter().map(|t| t.pnl).fold(0.0, f64::max);
        let largest_loss = losing_trades.iter().map(|t| t.pnl).fold(0.0, f64::min);
        
        let avg_duration = if !self.trades.is_empty() {
            let total_duration: u64 = self.trades.iter().map(|t| t.exit_time - t.entry_time).sum();
            (total_duration as f64 / self.trades.len() as f64) / 3600000.0 // Convert ms to hours
        } else {
            0.0
        };

        BacktestMetrics {
            total_return,
            annual_return,
            sharpe_ratio,
            sortino_ratio,
            calmar_ratio,
            max_drawdown,
            win_rate,
            profit_factor,
            total_trades: self.trades.len(),
            winning_trades: winning_trades.len(),
            losing_trades: losing_trades.len(),
            avg_win,
            avg_loss,
            largest_win,
            largest_loss,
            avg_trade_duration_hours: avg_duration,
            final_capital,
        }
    }

    /// Calculate maximum drawdown
    fn calculate_max_drawdown(&self) -> f64 {
        if self.equity_curve.is_empty() {
            return 0.0;
        }

        let mut max_value = self.equity_curve[0];
        let mut max_dd = 0.0;

        for &value in &self.equity_curve {
            if value > max_value {
                max_value = value;
            }
            let dd = ((value - max_value) / max_value) * 100.0;
            if dd < max_dd {
                max_dd = dd;
            }
        }

        max_dd
    }

    /// Run Monte Carlo simulation (parallel)
    pub fn monte_carlo_simulation(&self, num_simulations: usize) -> MonteCarloResult {
        if self.trades.is_empty() {
            return MonteCarloResult {
                mean_return: 0.0,
                median_return: 0.0,
                std_return: 0.0,
                var_95: 0.0,
                best_case: 0.0,
                worst_case: 0.0,
                probability_profit: 0.0,
            };
        }

        // Run simulations in parallel
        let results: Vec<f64> = (0..num_simulations)
            .into_par_iter()
            .map(|_| {
                self.run_simulation_iteration()
            })
            .collect();

        let results_arr = Array1::from_vec(results.clone());
        
        let mean_return = results_arr.mean().unwrap_or(0.0);
        let std_return = results_arr.std(0.0);
        
        let mut sorted_results = results.clone();
        sorted_results.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median_idx = sorted_results.len() / 2;
        let median_return = sorted_results[median_idx];
        
        let var_95_idx = (0.05 * sorted_results.len() as f64) as usize;
        let var_95 = sorted_results[var_95_idx];
        
        let best_case = *sorted_results.last().unwrap_or(&0.0);
        let worst_case = *sorted_results.first().unwrap_or(&0.0);
        
        let profitable = results.iter().filter(|&&r| r > 0.0).count();
        let probability_profit = (profitable as f64 / results.len() as f64) * 100.0;

        MonteCarloResult {
            mean_return,
            median_return,
            std_return,
            var_95,
            best_case,
            worst_case,
            probability_profit,
        }
    }

    /// Run single Monte Carlo iteration
    fn run_simulation_iteration(&self) -> f64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let mut capital = self.config.initial_capital;
        
        // Randomly sample trades with replacement
        for _ in 0..self.trades.len() {
            let idx = rng.gen_range(0..self.trades.len());
            let trade = &self.trades[idx];
            capital += trade.pnl;
        }
        
        ((capital - self.config.initial_capital) / self.config.initial_capital) * 100.0
    }

    /// Get equity curve
    pub fn get_equity_curve(&self) -> &[f64] {
        &self.equity_curve
    }

    /// Get all trades
    pub fn get_trades(&self) -> &[BacktestTrade] {
        &self.trades
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtest_engine() {
        let config = BacktestConfig::default();
        let mut engine = BacktestEngine::new(config);

        engine.execute_trade(
            "BTCUSDT".to_string(),
            0,
            3600000, // 1 hour
            50000.0,
            51000.0,
            TradeSide::Long,
        );

        let metrics = engine.calculate_metrics();
        assert_eq!(metrics.total_trades, 1);
        assert!(metrics.final_capital > 100000.0); // Profitable trade
    }

    #[test]
    fn test_monte_carlo() {
        let config = BacktestConfig::default();
        let mut engine = BacktestEngine::new(config);

        // Add some trades
        for i in 0..10 {
            engine.execute_trade(
                "TEST".to_string(),
                i * 1000,
                (i + 1) * 1000,
                100.0,
                if i % 2 == 0 { 102.0 } else { 99.0 },
                TradeSide::Long,
            );
        }

        let mc_result = engine.monte_carlo_simulation(100);
        assert!(mc_result.probability_profit >= 0.0);
        assert!(mc_result.probability_profit <= 100.0);
    }
}
