export interface VolumeProfileLevel {
  price: number;
  volume: number;
  percentage: number;
}

export interface VolumeProfile {
  levels: VolumeProfileLevel[];
  poc: number; // Point of Control
  vah: number; // Value Area High
  val: number; // Value Area Low
  valueArea: number; // Percentage of volume in value area
}

export interface VolumeProfileAnalysis {
  profile: VolumeProfile;
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  currentPricePosition: 'above_vah' | 'in_value' | 'below_val';
}

export function calculateVolumeProfile(
  candles: Array<{ high: number; low: number; close: number; volume: number }>,
  bins: number = 50
): VolumeProfileAnalysis {
  const prices = candles.flatMap(c => [c.high, c.low, c.close]);
  const minPrice = Math.min(...prices);
  const maxPrice = Math.max(...prices);
  const binSize = (maxPrice - minPrice) / bins;
  
  const volumeByPrice: Map<number, number> = new Map();
  
  candles.forEach(candle => {
    const avgPrice = (candle.high + candle.low + candle.close) / 3;
    const bin = Math.floor((avgPrice - minPrice) / binSize) * binSize + minPrice;
    volumeByPrice.set(bin, (volumeByPrice.get(bin) || 0) + candle.volume);
  });
  
  const totalVolume = Array.from(volumeByPrice.values()).reduce((sum, v) => sum + v, 0);
  
  const levels: VolumeProfileLevel[] = Array.from(volumeByPrice.entries())
    .map(([price, volume]) => ({
      price,
      volume,
      percentage: (volume / totalVolume) * 100
    }))
    .sort((a, b) => b.volume - a.volume);
  
  const poc = levels[0]?.price || 0;
  
  const sortedLevels = [...levels].sort((a, b) => b.volume - a.volume);
  let vaVolume = 0;
  let vaLevels: VolumeProfileLevel[] = [];
  const targetVolume = totalVolume * 0.7;
  
  for (const level of sortedLevels) {
    vaVolume += level.volume;
    vaLevels.push(level);
    if (vaVolume >= targetVolume) break;
  }
  
  const vaPrices = vaLevels.map(l => l.price);
  const vah = Math.max(...vaPrices);
  const val = Math.min(...vaPrices);
  const valueArea = (vaVolume / totalVolume) * 100;
  
  const currentPrice = candles[candles.length - 1]?.close || 0;
  let currentPricePosition: 'above_vah' | 'in_value' | 'below_val';
  if (currentPrice > vah) currentPricePosition = 'above_vah';
  else if (currentPrice < val) currentPricePosition = 'below_val';
  else currentPricePosition = 'in_value';
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (currentPricePosition === 'above_vah') signal = 'bullish';
  else if (currentPricePosition === 'below_val') signal = 'bearish';
  
  const distanceFromPOC = Math.abs(currentPrice - poc) / poc;
  const score = Math.min(100, 70 - distanceFromPOC * 100 + valueArea * 0.3);
  
  return {
    profile: { levels, poc, vah, val, valueArea },
    score,
    signal,
    currentPricePosition
  };
}
