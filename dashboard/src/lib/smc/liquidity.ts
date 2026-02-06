export interface LiquidityZone {
  id: string;
  type: 'buy' | 'sell';
  level: number;
  timestamp: number;
  swept: boolean;
  strength: number;
}

export interface LiquidityAnalysis {
  zones: LiquidityZone[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  sweeps: number;
}

export function detectLiquidity(
  candles: Array<{ high: number; low: number; close: number; volume: number; timestamp: number }>,
  lookback: number = 50
): LiquidityAnalysis {
  const zones: LiquidityZone[] = [];
  
  for (let i = lookback; i < candles.length; i++) {
    const curr = candles[i];
    const previous = candles.slice(i - lookback, i);
    
    // Find liquidity pools at highs and lows
    const recentHigh = Math.max(...previous.map(c => c.high));
    const recentLow = Math.min(...previous.map(c => c.low));
    
    // Sell-side liquidity sweep (price goes below recent low then reverses)
    if (curr.low < recentLow) {
      const strength = ((recentLow - curr.low) / recentLow) * 100;
      zones.push({
        id: `liq-sell-${i}`,
        type: 'sell',
        level: recentLow,
        timestamp: curr.timestamp,
        swept: true,
        strength
      });
    }
    
    // Buy-side liquidity sweep (price goes above recent high then reverses)
    if (curr.high > recentHigh) {
      const strength = ((curr.high - recentHigh) / recentHigh) * 100;
      zones.push({
        id: `liq-buy-${i}`,
        type: 'buy',
        level: recentHigh,
        timestamp: curr.timestamp,
        swept: true,
        strength
      });
    }
  }
  
  const sweeps = zones.filter(z => z.swept).length;
  const buySweeps = zones.filter(z => z.type === 'buy' && z.swept).length;
  const sellSweeps = zones.filter(z => z.type === 'sell' && z.swept).length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (sellSweeps > buySweeps) signal = 'bullish'; // Sell-side sweep = bullish reversal
  else if (buySweeps > sellSweeps) signal = 'bearish'; // Buy-side sweep = bearish reversal
  
  const avgStrength = zones.length > 0
    ? zones.reduce((sum, z) => sum + z.strength, 0) / zones.length
    : 0;
  
  const score = Math.min(100, sweeps * 8 + avgStrength * 5);
  
  return { zones, score, signal, sweeps };
}
