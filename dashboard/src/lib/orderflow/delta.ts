export interface DeltaData {
  timestamp: number;
  buyVolume: number;
  sellVolume: number;
  delta: number;
  cumulativeDelta: number;
}

export interface DeltaAnalysis {
  deltas: DeltaData[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  currentDelta: number;
  cumulativeDelta: number;
}

export function analyzeDelta(
  candles: Array<{ close: number; open: number; volume: number; timestamp: number }>
): DeltaAnalysis {
  const deltas: DeltaData[] = [];
  let cumulativeDelta = 0;
  
  candles.forEach((candle, i) => {
    const isBullish = candle.close > candle.open;
    const buyVolume = isBullish ? candle.volume * 0.65 : candle.volume * 0.35;
    const sellVolume = candle.volume - buyVolume;
    const delta = buyVolume - sellVolume;
    
    cumulativeDelta += delta;
    
    deltas.push({
      timestamp: candle.timestamp,
      buyVolume,
      sellVolume,
      delta,
      cumulativeDelta
    });
  });
  
  const currentDelta = deltas[deltas.length - 1]?.delta || 0;
  const avgDelta = deltas.reduce((sum, d) => sum + Math.abs(d.delta), 0) / deltas.length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (cumulativeDelta > avgDelta * 2) signal = 'bullish';
  else if (cumulativeDelta < -avgDelta * 2) signal = 'bearish';
  
  const score = Math.min(100, Math.abs(cumulativeDelta / avgDelta) * 10);
  
  return {
    deltas,
    score,
    signal,
    currentDelta,
    cumulativeDelta
  };
}
