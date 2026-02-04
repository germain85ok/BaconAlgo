# BaconAlgo ML/AI Engine

Machine learning and AI components for price prediction and pattern detection.

## Features

- **predictor.py**: LSTM-based neural network for price prediction
- **pattern_detector.py**: CNN-based candlestick pattern recognition
- **backtester.py**: Strategy backtesting engine with performance metrics

## Installation

```bash
pip install -r requirements.txt
```

**Note**: Some dependencies (like ta-lib) may require system-level packages:

```bash
# Ubuntu/Debian
sudo apt-get install build-essential python3-dev

# macOS
brew install ta-lib
```

## Usage

### Price Prediction

```python
from ml.predictor import predict_price_movement

prices = [100, 102, 101, 105, 103, ...]
result = predict_price_movement("BTCUSDT", prices)
print(result)
# {'symbol': 'BTCUSDT', 'prediction': 'bullish', 'confidence': 75.5, ...}
```

### Pattern Detection

```python
from ml.pattern_detector import detect_patterns

candles = [
    {"open": 100, "high": 105, "low": 98, "close": 103},
    {"open": 103, "high": 104, "low": 95, "close": 96},
]
patterns = detect_patterns(candles)
```

### Backtesting

```python
import pandas as pd
from ml.backtester import Backtester, example_strategy

# Load your data
data = pd.read_csv('historical_data.csv')

# Run backtest
backtester = Backtester(initial_balance=10000)
results = backtester.run(data, example_strategy)
print(results)
```

## GPU Support

For CUDA acceleration (recommended for training):

```bash
# Install PyTorch with CUDA support
pip install torch torchvision --index-url https://download.pytorch.org/whl/cu118
```

## Future Enhancements

- [ ] Train models on historical data
- [ ] Implement attention mechanisms
- [ ] Add reinforcement learning for strategy optimization
- [ ] Multi-symbol correlation analysis
- [ ] Sentiment analysis from news/social media
