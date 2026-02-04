"""
BaconAlgo Pattern Detector
CNN-based candlestick pattern recognition
"""

import numpy as np
from typing import List, Dict

try:
    import torch
    import torch.nn as nn
    TORCH_AVAILABLE = True
except ImportError:
    TORCH_AVAILABLE = False


class CandlestickPattern:
    """Candlestick pattern definitions"""
    
    PATTERNS = {
        "hammer": "Bullish reversal - small body, long lower wick",
        "shooting_star": "Bearish reversal - small body, long upper wick",
        "engulfing_bullish": "Bullish - current candle engulfs previous bearish",
        "engulfing_bearish": "Bearish - current candle engulfs previous bullish",
        "doji": "Indecision - open equals close",
        "marubozu_bullish": "Strong bullish - no wicks",
        "marubozu_bearish": "Strong bearish - no wicks",
    }


def detect_hammer(open_price: float, high: float, low: float, close: float) -> bool:
    """Detect hammer pattern"""
    body = abs(close - open_price)
    lower_wick = min(open_price, close) - low
    upper_wick = high - max(open_price, close)
    
    return (lower_wick > body * 2) and (upper_wick < body * 0.5)


def detect_doji(open_price: float, high: float, low: float, close: float) -> bool:
    """Detect doji pattern"""
    body = abs(close - open_price)
    total_range = high - low
    
    return body < total_range * 0.1


def detect_engulfing(prev_candle: dict, curr_candle: dict) -> str:
    """Detect engulfing patterns"""
    prev_bullish = prev_candle['close'] > prev_candle['open']
    curr_bullish = curr_candle['close'] > curr_candle['open']
    
    # Bullish engulfing
    if not prev_bullish and curr_bullish:
        if (curr_candle['close'] > prev_candle['open'] and 
            curr_candle['open'] < prev_candle['close']):
            return "engulfing_bullish"
    
    # Bearish engulfing
    if prev_bullish and not curr_bullish:
        if (curr_candle['open'] > prev_candle['close'] and 
            curr_candle['close'] < prev_candle['open']):
            return "engulfing_bearish"
    
    return None


def detect_patterns(candles: List[dict]) -> List[dict]:
    """
    Detect candlestick patterns in a series of candles
    
    Args:
        candles: List of candle dicts with 'open', 'high', 'low', 'close'
    
    Returns:
        List of detected patterns with metadata
    """
    patterns = []
    
    for i in range(len(candles)):
        candle = candles[i]
        detected = []
        
        # Single candle patterns
        if detect_hammer(candle['open'], candle['high'], candle['low'], candle['close']):
            detected.append({
                "pattern": "hammer",
                "type": "bullish",
                "confidence": 70,
                "description": CandlestickPattern.PATTERNS["hammer"]
            })
        
        if detect_doji(candle['open'], candle['high'], candle['low'], candle['close']):
            detected.append({
                "pattern": "doji",
                "type": "neutral",
                "confidence": 60,
                "description": CandlestickPattern.PATTERNS["doji"]
            })
        
        # Multi-candle patterns
        if i > 0:
            engulfing = detect_engulfing(candles[i-1], candle)
            if engulfing:
                pattern_type = "bullish" if "bullish" in engulfing else "bearish"
                detected.append({
                    "pattern": engulfing,
                    "type": pattern_type,
                    "confidence": 80,
                    "description": CandlestickPattern.PATTERNS[engulfing]
                })
        
        if detected:
            patterns.append({
                "index": i,
                "timestamp": candle.get('timestamp', i),
                "patterns": detected
            })
    
    return patterns


class CNNPatternDetector:
    """CNN-based pattern detector (placeholder for future ML implementation)"""
    
    def __init__(self):
        self.model = None
    
    def detect(self, candles: List[dict]) -> List[dict]:
        """Use traditional pattern detection for now"""
        return detect_patterns(candles)


if __name__ == "__main__":
    # Example usage
    sample_candles = [
        {"open": 100, "high": 105, "low": 98, "close": 103, "timestamp": 1},
        {"open": 103, "high": 104, "low": 95, "close": 96, "timestamp": 2},
        {"open": 96, "high": 102, "low": 95, "close": 101, "timestamp": 3},
    ]
    
    patterns = detect_patterns(sample_candles)
    print("Detected patterns:", patterns)
