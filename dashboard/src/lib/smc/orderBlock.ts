export interface OrderBlock {
  id: string;
  type: 'bullish' | 'bearish';
  high: number;
  low: number;
  timestamp: number;
  volume: number;
  strength: number;
  touched: boolean;
}

export interface OrderBlockAnalysis {
  blocks: OrderBlock[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  activeBlocks: number;
}

export function detectOrderBlocks(
  candles: Array<{ open: number; high: number; low: number; close: number; volume: number; timestamp: number }>
): OrderBlockAnalysis {
  const blocks: OrderBlock[] = [];
  
  for (let i = 1; i < candles.length - 1; i++) {
    const prev = candles[i - 1];
    const curr = candles[i];
    const next = candles[i + 1];
    
    // Bullish OB: strong down candle followed by up move
    if (curr.close < curr.open && next.close > next.open && next.close > curr.high) {
      const strength = ((next.close - curr.low) / curr.low) * 100;
      blocks.push({
        id: `ob-bull-${i}`,
        type: 'bullish',
        high: curr.high,
        low: curr.low,
        timestamp: curr.timestamp,
        volume: curr.volume,
        strength,
        touched: false
      });
    }
    
    // Bearish OB: strong up candle followed by down move
    if (curr.close > curr.open && next.close < next.open && next.close < curr.low) {
      const strength = ((curr.high - next.close) / curr.high) * 100;
      blocks.push({
        id: `ob-bear-${i}`,
        type: 'bearish',
        high: curr.high,
        low: curr.low,
        timestamp: curr.timestamp,
        volume: curr.volume,
        strength,
        touched: false
      });
    }
  }
  
  const activeBlocks = blocks.filter(b => !b.touched).length;
  const bullishBlocks = blocks.filter(b => b.type === 'bullish' && !b.touched).length;
  const bearishBlocks = blocks.filter(b => b.type === 'bearish' && !b.touched).length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullishBlocks > bearishBlocks) signal = 'bullish';
  else if (bearishBlocks > bullishBlocks) signal = 'bearish';
  
  const avgStrength = blocks.length > 0
    ? blocks.reduce((sum, b) => sum + b.strength, 0) / blocks.length
    : 0;
  
  const score = Math.min(100, activeBlocks * 15 + avgStrength * 3);
  
  return { blocks, score, signal, activeBlocks };
}
