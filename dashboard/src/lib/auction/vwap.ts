export interface VWAPData {
  timestamp: number;
  vwap: number;
  upperBand1: number;
  lowerBand1: number;
  upperBand2: number;
  lowerBand2: number;
}

export interface VWAPAnalysis {
  data: VWAPData[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  currentVWAP: number;
  priceVsVWAP: number;
}

export function calculateVWAP(
  candles: Array<{ high: number; low: number; close: number; volume: number; timestamp: number }>,
  stdDevMultiplier: number = 2
): VWAPAnalysis {
  const data: VWAPData[] = [];
  let cumulativeTPV = 0; // Typical Price * Volume
  let cumulativeVolume = 0;
  const typicalPrices: number[] = [];
  
  candles.forEach((candle, i) => {
    const typicalPrice = (candle.high + candle.low + candle.close) / 3;
    typicalPrices.push(typicalPrice);
    
    cumulativeTPV += typicalPrice * candle.volume;
    cumulativeVolume += candle.volume;
    
    const vwap = cumulativeTPV / cumulativeVolume;
    
    const variance = typicalPrices.reduce((sum, tp) => {
      return sum + Math.pow(tp - vwap, 2);
    }, 0) / typicalPrices.length;
    
    const stdDev = Math.sqrt(variance);
    
    data.push({
      timestamp: candle.timestamp,
      vwap,
      upperBand1: vwap + stdDev,
      lowerBand1: vwap - stdDev,
      upperBand2: vwap + stdDev * stdDevMultiplier,
      lowerBand2: vwap - stdDev * stdDevMultiplier
    });
  });
  
  const currentVWAP = data[data.length - 1]?.vwap || 0;
  const currentPrice = candles[candles.length - 1]?.close || 0;
  const priceVsVWAP = ((currentPrice - currentVWAP) / currentVWAP) * 100;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  const lastData = data[data.length - 1];
  
  if (lastData) {
    if (currentPrice > lastData.upperBand1) signal = 'bullish';
    else if (currentPrice < lastData.lowerBand1) signal = 'bearish';
  }
  
  const score = Math.min(100, 50 + Math.abs(priceVsVWAP) * 10);
  
  return {
    data,
    score,
    signal,
    currentVWAP,
    priceVsVWAP
  };
}
