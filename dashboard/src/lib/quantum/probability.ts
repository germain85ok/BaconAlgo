export interface ProbabilityDistribution {
  mean: number;
  stdDev: number;
  confidence68: [number, number];
  confidence95: [number, number];
  confidence99: [number, number];
}

export interface ProbabilityAnalysis {
  distribution: ProbabilityDistribution;
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  upProbability: number;
  downProbability: number;
}

export function analyzeProbability(
  returns: number[]
): ProbabilityAnalysis {
  const mean = returns.reduce((sum, r) => sum + r, 0) / returns.length;
  
  const variance = returns.reduce((sum, r) => {
    return sum + Math.pow(r - mean, 2);
  }, 0) / returns.length;
  
  const stdDev = Math.sqrt(variance);
  
  const confidence68: [number, number] = [mean - stdDev, mean + stdDev];
  const confidence95: [number, number] = [mean - 2 * stdDev, mean + 2 * stdDev];
  const confidence99: [number, number] = [mean - 3 * stdDev, mean + 3 * stdDev];
  
  const distribution: ProbabilityDistribution = {
    mean,
    stdDev,
    confidence68,
    confidence95,
    confidence99
  };
  
  const upReturns = returns.filter(r => r > 0).length;
  const downReturns = returns.filter(r => r < 0).length;
  const total = returns.length;
  
  const upProbability = (upReturns / total) * 100;
  const downProbability = (downReturns / total) * 100;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (mean > stdDev * 0.5) signal = 'bullish';
  else if (mean < -stdDev * 0.5) signal = 'bearish';
  
  const sharpe = stdDev !== 0 ? mean / stdDev : 0;
  const score = Math.min(100, 50 + sharpe * 20 + upProbability * 0.3);
  
  return {
    distribution,
    score,
    signal,
    upProbability,
    downProbability
  };
}

export function calculateReturns(prices: number[]): number[] {
  const returns: number[] = [];
  for (let i = 1; i < prices.length; i++) {
    const ret = (prices[i] - prices[i - 1]) / prices[i - 1];
    returns.push(ret);
  }
  return returns;
}
