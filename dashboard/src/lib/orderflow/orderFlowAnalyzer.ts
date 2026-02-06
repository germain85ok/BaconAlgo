/**
 * V2: Order Flow Analyzer
 * 
 * Analyzes:
 * - Buy/Sell volume delta
 * - Cumulative delta tracking
 * - Large order detection (whale watching)
 * - Retail vs Institutional flow
 * - Bid/Ask imbalance
 */

export interface OrderFlowData {
  timestamp: number;
  buyVolume: number;
  sellVolume: number;
  delta: number;
  cumulativeDelta: number;
  largeOrders: LargeOrder[];
  bidAskImbalance: number;
  flowType: 'retail' | 'institutional' | 'mixed';
}

export interface LargeOrder {
  timestamp: number;
  side: 'buy' | 'sell';
  size: number;
  price: number;
  isWhale: boolean; // Size > 10x average
}

export interface OrderFlowAnalysis {
  symbol: string;
  timeframe: string;
  
  // Delta metrics
  currentDelta: number;
  cumulativeDelta: number;
  deltaMA: number;
  
  // Volume metrics
  buyVolume: number;
  sellVolume: number;
  totalVolume: number;
  buyPressure: number; // 0-100
  
  // Large order detection
  whaleActivity: {
    count: number;
    netBuy: number;
    netSell: number;
    dominance: number; // 0-100
  };
  
  // Flow classification
  flowType: 'retail' | 'institutional' | 'mixed';
  institutionalConfidence: number; // 0-100
  
  // Imbalance
  bidAskImbalance: number; // Positive = more bids, Negative = more asks
  
  // Scoring (0-8)
  score: number;
}

export class OrderFlowAnalyzer {
  private cumulativeDelta: number = 0;
  private deltaHistory: number[] = [];
  private largeOrderThreshold: number = 10000; // Configurable
  
  /**
   * Analyze order flow from trade data
   */
  analyzeFlow(
    symbol: string,
    timeframe: string,
    trades: Array<{ price: number; size: number; side: 'buy' | 'sell'; timestamp: number }>
  ): OrderFlowAnalysis {
    let buyVolume = 0;
    let sellVolume = 0;
    const largeOrders: LargeOrder[] = [];
    
    // Calculate average trade size
    const avgSize = trades.reduce((sum, t) => sum + t.size, 0) / trades.length;
    const whaleThreshold = avgSize * 10;
    
    // Process each trade
    for (const trade of trades) {
      if (trade.side === 'buy') {
        buyVolume += trade.size;
      } else {
        sellVolume += trade.size;
      }
      
      // Detect large orders
      if (trade.size > this.largeOrderThreshold) {
        largeOrders.push({
          timestamp: trade.timestamp,
          side: trade.side,
          size: trade.size,
          price: trade.price,
          isWhale: trade.size > whaleThreshold
        });
      }
    }
    
    const totalVolume = buyVolume + sellVolume;
    const currentDelta = buyVolume - sellVolume;
    this.cumulativeDelta += currentDelta;
    this.deltaHistory.push(currentDelta);
    
    // Keep only last 20 periods
    if (this.deltaHistory.length > 20) {
      this.deltaHistory.shift();
    }
    
    const deltaMA = this.deltaHistory.reduce((sum, d) => sum + d, 0) / this.deltaHistory.length;
    
    // Calculate buy pressure (0-100)
    const buyPressure = totalVolume > 0 ? (buyVolume / totalVolume) * 100 : 50;
    
    // Analyze whale activity
    const whales = largeOrders.filter(o => o.isWhale);
    const whaleBuyVolume = whales.filter(w => w.side === 'buy').reduce((sum, w) => sum + w.size, 0);
    const whaleSellVolume = whales.filter(w => w.side === 'sell').reduce((sum, w) => sum + w.size, 0);
    const whaleVolume = whaleBuyVolume + whaleSellVolume;
    const whaleDominance = totalVolume > 0 ? (whaleVolume / totalVolume) * 100 : 0;
    
    // Classify flow type
    const flowType = this.classifyFlow(buyPressure, whaleDominance);
    const institutionalConfidence = this.calculateInstitutionalConfidence(
      whaleDominance,
      currentDelta,
      buyPressure
    );
    
    // Calculate bid/ask imbalance (simplified)
    const bidAskImbalance = currentDelta / (totalVolume || 1);
    
    // Score the order flow (0-8)
    const score = this.scoreOrderFlow(
      Math.abs(currentDelta),
      buyPressure,
      whaleDominance,
      institutionalConfidence
    );
    
    return {
      symbol,
      timeframe,
      currentDelta,
      cumulativeDelta: this.cumulativeDelta,
      deltaMA,
      buyVolume,
      sellVolume,
      totalVolume,
      buyPressure,
      whaleActivity: {
        count: whales.length,
        netBuy: whaleBuyVolume,
        netSell: whaleSellVolume,
        dominance: whaleDominance
      },
      flowType,
      institutionalConfidence,
      bidAskImbalance,
      score
    };
  }

  /**
   * Detect divergences between price and delta
   */
  detectDivergence(
    prices: number[],
    deltas: number[]
  ): {
    type: 'bullish' | 'bearish' | 'none';
    strength: number;
  } {
    if (prices.length < 10 || deltas.length < 10) {
      return { type: 'none', strength: 0 };
    }
    
    const priceSlope = this.calculateSlope(prices.slice(-10));
    const deltaSlope = this.calculateSlope(deltas.slice(-10));
    
    // Bullish divergence: price down, delta up
    if (priceSlope < -0.001 && deltaSlope > 0.001) {
      return {
        type: 'bullish',
        strength: Math.min(10, Math.abs(deltaSlope - priceSlope) * 100)
      };
    }
    
    // Bearish divergence: price up, delta down
    if (priceSlope > 0.001 && deltaSlope < -0.001) {
      return {
        type: 'bearish',
        strength: Math.min(10, Math.abs(deltaSlope - priceSlope) * 100)
      };
    }
    
    return { type: 'none', strength: 0 };
  }

  /**
   * Calculate absorption (large volume but small price move)
   */
  detectAbsorption(
    volume: number,
    priceChange: number,
    avgVolume: number
  ): {
    detected: boolean;
    level: 'low' | 'medium' | 'high';
  } {
    const volumeRatio = volume / (avgVolume || 1);
    const priceImpact = Math.abs(priceChange);
    
    // High volume, low price movement = absorption
    if (volumeRatio > 2 && priceImpact < 0.001) {
      return {
        detected: true,
        level: volumeRatio > 5 ? 'high' : volumeRatio > 3 ? 'medium' : 'low'
      };
    }
    
    return { detected: false, level: 'low' };
  }

  // Helper methods
  
  private classifyFlow(
    buyPressure: number,
    whaleDominance: number
  ): 'retail' | 'institutional' | 'mixed' {
    if (whaleDominance > 60) return 'institutional';
    if (whaleDominance < 20) return 'retail';
    return 'mixed';
  }

  private calculateInstitutionalConfidence(
    whaleDominance: number,
    delta: number,
    buyPressure: number
  ): number {
    let confidence = 0;
    
    // Whale dominance component (0-40)
    confidence += Math.min(40, whaleDominance * 0.6);
    
    // Delta strength component (0-30)
    confidence += Math.min(30, Math.abs(delta) / 100);
    
    // Directional component (0-30)
    if (buyPressure > 60 || buyPressure < 40) {
      confidence += Math.min(30, Math.abs(buyPressure - 50) * 0.6);
    }
    
    return Math.min(100, confidence);
  }

  private scoreOrderFlow(
    deltaMagnitude: number,
    buyPressure: number,
    whaleDominance: number,
    institutionalConfidence: number
  ): number {
    let score = 0;
    
    // Delta strength (0-3)
    if (deltaMagnitude > 1000) score += 1;
    if (deltaMagnitude > 5000) score += 1;
    if (deltaMagnitude > 10000) score += 1;
    
    // Buy/Sell pressure extremes (0-2)
    if (buyPressure > 70 || buyPressure < 30) score += 1;
    if (buyPressure > 80 || buyPressure < 20) score += 1;
    
    // Whale activity (0-2)
    if (whaleDominance > 40) score += 1;
    if (whaleDominance > 60) score += 1;
    
    // Institutional confidence (0-1)
    if (institutionalConfidence > 70) score += 1;
    
    return Math.min(8, score);
  }

  private calculateSlope(values: number[]): number {
    if (values.length < 2) return 0;
    
    const n = values.length;
    let sumX = 0;
    let sumY = 0;
    let sumXY = 0;
    let sumX2 = 0;
    
    for (let i = 0; i < n; i++) {
      sumX += i;
      sumY += values[i];
      sumXY += i * values[i];
      sumX2 += i * i;
    }
    
    const slope = (n * sumXY - sumX * sumY) / (n * sumX2 - sumX * sumX);
    return slope;
  }

  reset() {
    this.cumulativeDelta = 0;
    this.deltaHistory = [];
  }
}
