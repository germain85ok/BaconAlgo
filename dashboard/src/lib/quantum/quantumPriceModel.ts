/**
 * V4: Quantum Price Model
 * Wave function, probability distribution, quantum barriers
 */

export interface QuantumPriceAnalysis {
  symbol: string;
  waveFunction: number[];
  probabilityDistribution: Array<{price: number; probability: number}>;
  expectedValue: number;
  barriers: Array<{level: number; strength: number}>;
  skew: 'bullish' | 'bearish' | 'neutral';
  score: number; // 0-5
}

export class QuantumPriceModel {
  analyze(symbol: string, prices: number[]): QuantumPriceAnalysis {
    const current = prices[prices.length - 1];
    const volatility = this.calculateVolatility(prices);
    
    // Create probability distribution
    const distribution: Array<{price: number; probability: number}> = [];
    for (let i = -10; i <= 10; i++) {
      const price = current * (1 + i * 0.01);
      const probability = this.gaussianProbability(price, current, volatility);
      distribution.push({ price, probability });
    }
    
    // Calculate expected value
    const expectedValue = distribution.reduce((sum, d) => sum + d.price * d.probability, 0);
    
    // Identify barriers (support/resistance)
    const barriers = this.identifyBarriers(prices);
    
    // Determine skew
    const skew = expectedValue > current ? 'bullish' : expectedValue < current ? 'bearish' : 'neutral';
    
    return {
      symbol,
      waveFunction: prices.slice(-20),
      probabilityDistribution: distribution,
      expectedValue,
      barriers,
      skew,
      score: 4
    };
  }

  private calculateVolatility(prices: number[]): number {
    const returns = prices.slice(1).map((p, i) => Math.log(p / prices[i]));
    const mean = returns.reduce((sum, r) => sum + r, 0) / returns.length;
    const variance = returns.reduce((sum, r) => sum + Math.pow(r - mean, 2), 0) / returns.length;
    return Math.sqrt(variance);
  }

  private gaussianProbability(x: number, mean: number, stdDev: number): number {
    return Math.exp(-Math.pow(x - mean, 2) / (2 * Math.pow(stdDev, 2))) / (stdDev * Math.sqrt(2 * Math.PI));
  }

  private identifyBarriers(prices: number[]): Array<{level: number; strength: number}> {
    // Find price levels that act as barriers
    const levels = new Map<number, number>();
    
    for (const price of prices) {
      const rounded = Math.round(price * 100) / 100;
      levels.set(rounded, (levels.get(rounded) || 0) + 1);
    }
    
    return Array.from(levels.entries())
      .filter(([, count]) => count >= 3)
      .map(([level, count]) => ({ level, strength: count }))
      .slice(0, 5);
  }
}
