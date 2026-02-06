export interface DarkPoolPrint {
  id: string;
  timestamp: number;
  price: number;
  size: number;
  type: 'block' | 'sweep' | 'split';
  sentiment: 'bullish' | 'bearish';
}

export interface DarkPoolAnalysis {
  prints: DarkPoolPrint[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  totalVolume: number;
  largestPrint: number;
}

export function detectDarkPool(
  candles: Array<{ high: number; low: number; close: number; volume: number; timestamp: number }>,
  volumeThreshold: number = 1000000
): DarkPoolAnalysis {
  const prints: DarkPoolPrint[] = [];
  
  candles.forEach((candle, i) => {
    if (candle.volume > volumeThreshold) {
      const avgVolume = candles.slice(Math.max(0, i - 20), i)
        .reduce((sum, c) => sum + c.volume, 0) / Math.min(20, i);
      
      if (candle.volume > avgVolume * 3) {
        const price = (candle.high + candle.low + candle.close) / 3;
        const sentiment = candle.close > (candle.high + candle.low) / 2 ? 'bullish' : 'bearish';
        const type = candle.volume > avgVolume * 5 ? 'block' : 'sweep';
        
        prints.push({
          id: `dp-${i}`,
          timestamp: candle.timestamp,
          price,
          size: candle.volume,
          type,
          sentiment
        });
      }
    }
  });
  
  const totalVolume = prints.reduce((sum, p) => sum + p.size, 0);
  const largestPrint = Math.max(...prints.map(p => p.size), 0);
  
  const bullishPrints = prints.filter(p => p.sentiment === 'bullish').length;
  const bearishPrints = prints.filter(p => p.sentiment === 'bearish').length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullishPrints > bearishPrints) signal = 'bullish';
  else if (bearishPrints > bullishPrints) signal = 'bearish';
  
  const score = Math.min(100, prints.length * 15 + (totalVolume / volumeThreshold) * 2);
  
  return {
    prints,
    score,
    signal,
    totalVolume,
    largestPrint
  };
}
