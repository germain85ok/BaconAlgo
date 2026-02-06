export interface BreakOfStructure {
  id: string;
  type: 'bullish' | 'bearish';
  level: number;
  timestamp: number;
  strength: number;
  confirmed: boolean;
}

export interface BOSAnalysis {
  breaks: BreakOfStructure[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  recentBreaks: number;
}

export function detectBOS(
  candles: Array<{ high: number; low: number; close: number; timestamp: number }>,
  lookback: number = 20
): BOSAnalysis {
  const breaks: BreakOfStructure[] = [];
  
  for (let i = lookback; i < candles.length; i++) {
    const curr = candles[i];
    const previous = candles.slice(i - lookback, i);
    
    const highestHigh = Math.max(...previous.map(c => c.high));
    const lowestLow = Math.min(...previous.map(c => c.low));
    
    // Bullish BOS: break above previous high
    if (curr.close > highestHigh) {
      const strength = ((curr.close - highestHigh) / highestHigh) * 100;
      breaks.push({
        id: `bos-bull-${i}`,
        type: 'bullish',
        level: highestHigh,
        timestamp: curr.timestamp,
        strength,
        confirmed: true
      });
    }
    
    // Bearish BOS: break below previous low
    if (curr.close < lowestLow) {
      const strength = ((lowestLow - curr.close) / lowestLow) * 100;
      breaks.push({
        id: `bos-bear-${i}`,
        type: 'bearish',
        level: lowestLow,
        timestamp: curr.timestamp,
        strength,
        confirmed: true
      });
    }
  }
  
  const recentBreaks = breaks.slice(-5).length;
  const bullishBreaks = breaks.filter(b => b.type === 'bullish').length;
  const bearishBreaks = breaks.filter(b => b.type === 'bearish').length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullishBreaks > bearishBreaks) signal = 'bullish';
  else if (bearishBreaks > bullishBreaks) signal = 'bearish';
  
  const avgStrength = breaks.length > 0
    ? breaks.reduce((sum, b) => sum + b.strength, 0) / breaks.length
    : 0;
  
  const score = Math.min(100, recentBreaks * 12 + avgStrength * 4);
  
  return { breaks, score, signal, recentBreaks };
}
