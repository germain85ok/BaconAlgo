"""
BaconAlgo Backtester
Strategy backtesting engine with performance metrics
"""

import numpy as np
import pandas as pd
from typing import List, Dict, Callable
from datetime import datetime


class Trade:
    """Represents a single trade"""
    
    def __init__(self, symbol: str, entry_price: float, entry_time: datetime,
                 direction: str, size: float = 1.0):
        self.symbol = symbol
        self.entry_price = entry_price
        self.entry_time = entry_time
        self.direction = direction  # 'long' or 'short'
        self.size = size
        
        self.exit_price = None
        self.exit_time = None
        self.profit = None
        self.profit_pct = None
    
    def close(self, exit_price: float, exit_time: datetime):
        """Close the trade"""
        self.exit_price = exit_price
        self.exit_time = exit_time
        
        if self.direction == 'long':
            self.profit = (exit_price - self.entry_price) * self.size
            self.profit_pct = ((exit_price - self.entry_price) / self.entry_price) * 100
        else:  # short
            self.profit = (self.entry_price - exit_price) * self.size
            self.profit_pct = ((self.entry_price - exit_price) / self.entry_price) * 100
    
    def to_dict(self) -> dict:
        """Convert trade to dictionary"""
        return {
            'symbol': self.symbol,
            'direction': self.direction,
            'entry_price': self.entry_price,
            'exit_price': self.exit_price,
            'entry_time': self.entry_time,
            'exit_time': self.exit_time,
            'profit': self.profit,
            'profit_pct': self.profit_pct,
            'size': self.size
        }


class Backtester:
    """Backtesting engine"""
    
    def __init__(self, initial_balance: float = 10000.0):
        self.initial_balance = initial_balance
        self.balance = initial_balance
        self.trades: List[Trade] = []
        self.equity_curve: List[float] = [initial_balance]
    
    def run(self, data: pd.DataFrame, strategy: Callable) -> Dict:
        """
        Run backtest on historical data
        
        Args:
            data: DataFrame with OHLCV data
            strategy: Function that returns 'long', 'short', or None for each row
        
        Returns:
            Performance metrics dictionary
        """
        open_trade = None
        
        for idx, row in data.iterrows():
            signal = strategy(data.loc[:idx])
            
            # Close existing trade if signal changes
            if open_trade and signal != open_trade.direction:
                open_trade.close(row['close'], row.get('timestamp', idx))
                self.balance += open_trade.profit
                self.trades.append(open_trade)
                self.equity_curve.append(self.balance)
                open_trade = None
            
            # Open new trade
            if signal and not open_trade:
                trade_size = self.balance * 0.1  # Risk 10% per trade
                open_trade = Trade(
                    symbol=data.attrs.get('symbol', 'UNKNOWN'),
                    entry_price=row['close'],
                    entry_time=row.get('timestamp', idx),
                    direction=signal,
                    size=trade_size / row['close']
                )
        
        # Close any remaining open trade
        if open_trade:
            last_row = data.iloc[-1]
            open_trade.close(last_row['close'], last_row.get('timestamp', len(data)-1))
            self.balance += open_trade.profit
            self.trades.append(open_trade)
            self.equity_curve.append(self.balance)
        
        return self.calculate_metrics()
    
    def calculate_metrics(self) -> Dict:
        """Calculate performance metrics"""
        if not self.trades:
            return {
                'total_trades': 0,
                'win_rate': 0.0,
                'total_profit': 0.0,
                'total_profit_pct': 0.0,
                'max_drawdown': 0.0,
                'sharpe_ratio': 0.0
            }
        
        winning_trades = [t for t in self.trades if t.profit > 0]
        losing_trades = [t for t in self.trades if t.profit <= 0]
        
        total_profit = sum(t.profit for t in self.trades)
        total_profit_pct = ((self.balance - self.initial_balance) / self.initial_balance) * 100
        
        # Calculate max drawdown
        peak = self.equity_curve[0]
        max_dd = 0
        for equity in self.equity_curve:
            if equity > peak:
                peak = equity
            dd = ((peak - equity) / peak) * 100
            max_dd = max(max_dd, dd)
        
        # Calculate Sharpe ratio (simplified)
        returns = np.diff(self.equity_curve) / self.equity_curve[:-1]
        sharpe = (np.mean(returns) / np.std(returns)) * np.sqrt(252) if len(returns) > 0 else 0
        
        return {
            'total_trades': len(self.trades),
            'winning_trades': len(winning_trades),
            'losing_trades': len(losing_trades),
            'win_rate': (len(winning_trades) / len(self.trades)) * 100,
            'total_profit': round(total_profit, 2),
            'total_profit_pct': round(total_profit_pct, 2),
            'avg_profit_per_trade': round(total_profit / len(self.trades), 2),
            'max_drawdown': round(max_dd, 2),
            'sharpe_ratio': round(sharpe, 2),
            'final_balance': round(self.balance, 2)
        }


def example_strategy(data: pd.DataFrame) -> str:
    """Example moving average crossover strategy"""
    if len(data) < 20:
        return None
    
    # Simple moving average crossover
    fast_ma = data['close'].rolling(window=5).mean().iloc[-1]
    slow_ma = data['close'].rolling(window=20).mean().iloc[-1]
    
    if fast_ma > slow_ma:
        return 'long'
    elif fast_ma < slow_ma:
        return 'short'
    
    return None


if __name__ == "__main__":
    # Example usage
    dates = pd.date_range(start='2023-01-01', periods=100, freq='D')
    prices = 100 + np.cumsum(np.random.randn(100))
    
    data = pd.DataFrame({
        'timestamp': dates,
        'open': prices,
        'high': prices + np.random.rand(100),
        'low': prices - np.random.rand(100),
        'close': prices,
        'volume': np.random.randint(1000, 10000, 100)
    })
    data.attrs['symbol'] = 'BTCUSDT'
    
    backtester = Backtester(initial_balance=10000)
    results = backtester.run(data, example_strategy)
    
    print("Backtest Results:")
    for key, value in results.items():
        print(f"  {key}: {value}")
