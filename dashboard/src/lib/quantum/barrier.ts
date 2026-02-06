export interface BarrierLevel {
  price: number;
  type: 'resistance' | 'support';
  strength: number;
  touches: number;
}

export interface BarrierAnalysis {
  barriers: BarrierLevel[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  nearestSupport: number;
  nearestResistance: number;
  delta: number; // Option Greeks concept
  gamma: number;
}

export function analyzeBarriers(
  candles: Array<{ high: number; low: number; close: number }>,
  tolerance: number = 0.01
): BarrierAnalysis {
  const barriers: BarrierLevel[] = [];
  const prices = candles.map(c => c.close);
  const currentPrice = prices[prices.length - 1] || 0;
  
  // Find resistance levels (highs)
  const highs = candles.map(c => c.high);
  const resistanceClusters = findClusters(highs, tolerance);
  
  resistanceClusters.forEach((cluster, i) => {
    barriers.push({
      price: cluster.level,
      type: 'resistance',
      strength: cluster.strength,
      touches: cluster.count
    });
  });
  
  // Find support levels (lows)
  const lows = candles.map(c => c.low);
  const supportClusters = findClusters(lows, tolerance);
  
  supportClusters.forEach((cluster, i) => {
    barriers.push({
      price: cluster.level,
      type: 'support',
      strength: cluster.strength,
      touches: cluster.count
    });
  });
  
  const supports = barriers.filter(b => b.type === 'support' && b.price < currentPrice);
  const resistances = barriers.filter(b => b.type === 'resistance' && b.price > currentPrice);
  
  const nearestSupport = supports.length > 0 
    ? Math.max(...supports.map(s => s.price)) 
    : currentPrice * 0.95;
  
  const nearestResistance = resistances.length > 0 
    ? Math.min(...resistances.map(r => r.price)) 
    : currentPrice * 1.05;
  
  // Delta: rate of change (like option delta)
  const priceChange = prices[prices.length - 1] - prices[prices.length - 10];
  const delta = priceChange / prices[prices.length - 10];
  
  // Gamma: acceleration (like option gamma)
  const recentChange = prices[prices.length - 1] - prices[prices.length - 5];
  const olderChange = prices[prices.length - 5] - prices[prices.length - 10];
  const gamma = (recentChange - olderChange) / prices[prices.length - 10];
  
  const distanceToResistance = (nearestResistance - currentPrice) / currentPrice;
  const distanceToSupport = (currentPrice - nearestSupport) / currentPrice;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (distanceToResistance > distanceToSupport * 2) signal = 'bullish';
  else if (distanceToSupport > distanceToResistance * 2) signal = 'bearish';
  
  const avgStrength = barriers.reduce((sum, b) => sum + b.strength, 0) / barriers.length;
  const score = Math.min(100, avgStrength * 50 + Math.abs(delta) * 100);
  
  return {
    barriers,
    score,
    signal,
    nearestSupport,
    nearestResistance,
    delta,
    gamma
  };
}

function findClusters(prices: number[], tolerance: number): Array<{ level: number; count: number; strength: number }> {
  const clusters: Map<number, number> = new Map();
  
  prices.forEach(price => {
    let found = false;
    for (const [level, count] of clusters.entries()) {
      if (Math.abs(price - level) / level <= tolerance) {
        clusters.set(level, count + 1);
        found = true;
        break;
      }
    }
    if (!found) {
      clusters.set(price, 1);
    }
  });
  
  return Array.from(clusters.entries())
    .map(([level, count]) => ({
      level,
      count,
      strength: Math.min(1, count / 10)
    }))
    .filter(c => c.count >= 2)
    .sort((a, b) => b.count - a.count)
    .slice(0, 10);
}
