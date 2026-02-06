/**
 * V2: Options Flow Analysis
 * 
 * Analyzes:
 * - Put/Call ratio
 * - Unusual options activity
 * - Premium flow
 * - Gamma exposure (GEX)
 * - Max pain calculation
 * - Smart money options bets
 */

export interface OptionsData {
  timestamp: number;
  strike: number;
  expiration: string;
  type: 'call' | 'put';
  volume: number;
  openInterest: number;
  premium: number;
  iv: number; // Implied volatility
  delta: number;
  gamma: number;
}

export interface UnusualActivity {
  timestamp: number;
  strike: number;
  type: 'call' | 'put';
  volume: number;
  volumeRatio: number; // vs avg
  premium: number;
  sentiment: 'bullish' | 'bearish';
}

export interface OptionsFlowAnalysis {
  symbol: string;
  
  // Ratios
  putCallRatio: number;
  putCallVolumeRatio: number;
  
  // Premium flow
  callPremium: number;
  putPremium: number;
  netPremiumFlow: number; // Call - Put
  
  // Gamma exposure
  totalGEX: number;
  gexLevel: number; // Normalized
  gexSkew: 'call' | 'put' | 'neutral';
  
  // Max pain
  maxPain: number;
  distanceFromMaxPain: number; // %
  
  // Unusual activity
  unusualCalls: UnusualActivity[];
  unusualPuts: UnusualActivity[];
  
  // Smart money indicator
  smartMoneySignal: 'bullish' | 'bearish' | 'neutral';
  confidence: number; // 0-100
  
  // Scoring (0-5)
  score: number;
}

export class OptionsFlowAnalyzer {
  /**
   * Analyze options flow
   */
  analyzeFlow(
    symbol: string,
    options: OptionsData[],
    currentPrice: number
  ): OptionsFlowAnalysis {
    // Separate calls and puts
    const calls = options.filter(o => o.type === 'call');
    const puts = options.filter(o => o.type === 'put');
    
    // Calculate ratios
    const putCallRatio = this.calculatePutCallRatio(calls, puts);
    const putCallVolumeRatio = this.calculateVolumeRatio(calls, puts);
    
    // Calculate premium flow
    const callPremium = this.calculateTotalPremium(calls);
    const putPremium = this.calculateTotalPremium(puts);
    const netPremiumFlow = callPremium - putPremium;
    
    // Calculate GEX
    const gexData = this.calculateGEX(options, currentPrice);
    
    // Calculate max pain
    const maxPain = this.calculateMaxPain(options, currentPrice);
    const distanceFromMaxPain = ((currentPrice - maxPain) / maxPain) * 100;
    
    // Detect unusual activity
    const unusualCalls = this.detectUnusualActivity(calls);
    const unusualPuts = this.detectUnusualActivity(puts);
    
    // Determine smart money signal
    const { signal, confidence } = this.determineSmartMoneySignal(
      putCallRatio,
      netPremiumFlow,
      gexData,
      unusualCalls,
      unusualPuts
    );
    
    // Score options flow (0-5)
    const score = this.scoreOptionsFlow(
      putCallRatio,
      Math.abs(netPremiumFlow),
      unusualCalls.length + unusualPuts.length,
      confidence
    );
    
    return {
      symbol,
      putCallRatio,
      putCallVolumeRatio,
      callPremium,
      putPremium,
      netPremiumFlow,
      totalGEX: gexData.total,
      gexLevel: gexData.level,
      gexSkew: gexData.skew,
      maxPain,
      distanceFromMaxPain,
      unusualCalls,
      unusualPuts,
      smartMoneySignal: signal,
      confidence,
      score
    };
  }

  /**
   * Calculate Put/Call ratio (open interest)
   */
  private calculatePutCallRatio(calls: OptionsData[], puts: OptionsData[]): number {
    const callOI = calls.reduce((sum, c) => sum + c.openInterest, 0);
    const putOI = puts.reduce((sum, p) => sum + p.openInterest, 0);
    
    return callOI > 0 ? putOI / callOI : 0;
  }

  /**
   * Calculate Put/Call volume ratio
   */
  private calculateVolumeRatio(calls: OptionsData[], puts: OptionsData[]): number {
    const callVol = calls.reduce((sum, c) => sum + c.volume, 0);
    const putVol = puts.reduce((sum, p) => sum + p.volume, 0);
    
    return callVol > 0 ? putVol / callVol : 0;
  }

  /**
   * Calculate total premium
   */
  private calculateTotalPremium(options: OptionsData[]): number {
    return options.reduce((sum, o) => sum + (o.premium * o.volume), 0);
  }

  /**
   * Calculate Gamma Exposure (GEX)
   */
  private calculateGEX(
    options: OptionsData[],
    currentPrice: number
  ): {
    total: number;
    level: number;
    skew: 'call' | 'put' | 'neutral';
  } {
    let callGEX = 0;
    let putGEX = 0;
    
    for (const option of options) {
      const gex = option.gamma * option.openInterest * 100; // Per share
      
      if (option.type === 'call') {
        callGEX += gex;
      } else {
        putGEX += gex;
      }
    }
    
    const total = Math.abs(callGEX) + Math.abs(putGEX);
    const level = total / 1000000; // Normalize to millions
    
    const skew = callGEX > putGEX * 1.2 ? 'call' :
                 putGEX > callGEX * 1.2 ? 'put' : 'neutral';
    
    return { total, level, skew };
  }

  /**
   * Calculate Max Pain price
   * Price where option holders lose most money
   */
  private calculateMaxPain(options: OptionsData[], currentPrice: number): number {
    // Get unique strikes near current price
    const strikes = [...new Set(options.map(o => o.strike))]
      .filter(s => s > currentPrice * 0.8 && s < currentPrice * 1.2)
      .sort((a, b) => a - b);
    
    let minPain = Infinity;
    let maxPainStrike = currentPrice;
    
    for (const strike of strikes) {
      let pain = 0;
      
      for (const option of options) {
        if (option.type === 'call' && strike > option.strike) {
          pain += (strike - option.strike) * option.openInterest;
        } else if (option.type === 'put' && strike < option.strike) {
          pain += (option.strike - strike) * option.openInterest;
        }
      }
      
      if (pain < minPain) {
        minPain = pain;
        maxPainStrike = strike;
      }
    }
    
    return maxPainStrike;
  }

  /**
   * Detect unusual options activity
   */
  private detectUnusualActivity(options: OptionsData[]): UnusualActivity[] {
    const unusual: UnusualActivity[] = [];
    
    // Calculate average volume
    const avgVolume = options.reduce((sum, o) => sum + o.volume, 0) / options.length;
    
    for (const option of options) {
      const volumeRatio = option.volume / (avgVolume || 1);
      
      // Unusual if volume > 3x average
      if (volumeRatio > 3) {
        unusual.push({
          timestamp: option.timestamp,
          strike: option.strike,
          type: option.type,
          volume: option.volume,
          volumeRatio,
          premium: option.premium,
          sentiment: option.type === 'call' ? 'bullish' : 'bearish'
        });
      }
    }
    
    return unusual.sort((a, b) => b.volumeRatio - a.volumeRatio).slice(0, 5);
  }

  /**
   * Determine smart money signal
   */
  private determineSmartMoneySignal(
    putCallRatio: number,
    netPremiumFlow: number,
    gex: { total: number; level: number; skew: 'call' | 'put' | 'neutral' },
    unusualCalls: UnusualActivity[],
    unusualPuts: UnusualActivity[]
  ): { signal: 'bullish' | 'bearish' | 'neutral'; confidence: number } {
    let bullishSignals = 0;
    let bearishSignals = 0;
    let confidence = 0;
    
    // Put/Call ratio signal
    if (putCallRatio < 0.7) {
      bullishSignals++;
      confidence += 15;
    } else if (putCallRatio > 1.3) {
      bearishSignals++;
      confidence += 15;
    }
    
    // Premium flow signal
    if (netPremiumFlow > 1000000) {
      bullishSignals++;
      confidence += 20;
    } else if (netPremiumFlow < -1000000) {
      bearishSignals++;
      confidence += 20;
    }
    
    // GEX skew signal
    if (gex.skew === 'call') {
      bullishSignals++;
      confidence += 15;
    } else if (gex.skew === 'put') {
      bearishSignals++;
      confidence += 15;
    }
    
    // Unusual activity signal
    const callActivity = unusualCalls.reduce((sum, u) => sum + u.premium, 0);
    const putActivity = unusualPuts.reduce((sum, u) => sum + u.premium, 0);
    
    if (callActivity > putActivity * 1.5) {
      bullishSignals++;
      confidence += 25;
    } else if (putActivity > callActivity * 1.5) {
      bearishSignals++;
      confidence += 25;
    }
    
    const signal = bullishSignals > bearishSignals ? 'bullish' :
                   bearishSignals > bullishSignals ? 'bearish' : 'neutral';
    
    return { signal, confidence: Math.min(100, confidence) };
  }

  /**
   * Score options flow
   */
  private scoreOptionsFlow(
    putCallRatio: number,
    premiumMagnitude: number,
    unusualCount: number,
    confidence: number
  ): number {
    let score = 0;
    
    // Extreme put/call ratio (0-2)
    if (putCallRatio < 0.6 || putCallRatio > 1.4) score += 1;
    if (putCallRatio < 0.5 || putCallRatio > 1.6) score += 1;
    
    // Premium flow (0-2)
    if (premiumMagnitude > 5000000) score += 1;
    if (premiumMagnitude > 10000000) score += 1;
    
    // Unusual activity (0-1)
    if (unusualCount >= 3) score += 1;
    
    return Math.min(5, score);
  }
}
