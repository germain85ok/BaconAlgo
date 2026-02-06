export interface FairValueGap {
  id: string;
  type: 'bullish' | 'bearish';
  top: number;
  bottom: number;
  timestamp: number;
  filled: boolean;
  strength: number;
}

export interface FVGAnalysis {
  gaps: FairValueGap[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  activeGaps: number;
}

export function detectFVG(candles: Array<{ high: number; low: number; close: number; timestamp: number }>): FVGAnalysis {
  const gaps: FairValueGap[] = [];
  
  for (let i = 2; i < candles.length; i++) {
    const prev = candles[i - 2];
    const curr = candles[i - 1];
    const next = candles[i];
    
    // Bullish FVG: gap between prev.high and next.low
    if (prev.high < next.low) {
      const strength = ((next.low - prev.high) / prev.high) * 100;
      gaps.push({
        id: `fvg-bull-${i}`,
        type: 'bullish',
        top: next.low,
        bottom: prev.high,
        timestamp: curr.timestamp,
        filled: false,
        strength
      });
    }
    
    // Bearish FVG: gap between prev.low and next.high
    if (prev.low > next.high) {
      const strength = ((prev.low - next.high) / prev.low) * 100;
      gaps.push({
        id: `fvg-bear-${i}`,
        type: 'bearish',
        top: prev.low,
        bottom: next.high,
        timestamp: curr.timestamp,
        filled: false,
        strength
      });
    }
  }
  
  const activeGaps = gaps.filter(g => !g.filled).length;
  const bullishGaps = gaps.filter(g => g.type === 'bullish' && !g.filled).length;
  const bearishGaps = gaps.filter(g => g.type === 'bearish' && !g.filled).length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullishGaps > bearishGaps) signal = 'bullish';
  else if (bearishGaps > bullishGaps) signal = 'bearish';
  
  const avgStrength = gaps.length > 0 
    ? gaps.reduce((sum, g) => sum + g.strength, 0) / gaps.length 
    : 0;
  
  const score = Math.min(100, activeGaps * 10 + avgStrength * 2);
  
  return { gaps, score, signal, activeGaps };
}
