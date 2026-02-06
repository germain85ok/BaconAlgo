/**
 * Ultimate Scoring System
 * 
 * Combines V1-V5 scores into ultimate signal rating
 * 
 * Scoring breakdown:
 * - V1 SMC: 25 points (FVG 7, OB 7, BOS 6, Liquidity 5)
 * - V2 Order Flow: 25 points (Delta 8, Dark Pool 7, Options 5, Sentiment 5)
 * - V3 Auction: 20 points (Volume Profile 7, Footprint 7, VWAP 6)
 * - V4 Quantum: 15 points (Probability 5, Entanglement 5, Tunneling 5)
 * - V5 Psychology: 15 points (Emotional 5, Bias 5, Flow State 5)
 * 
 * Total: 100 points
 */

export interface UltimateSignal {
  symbol: string;
  timeframe: string;
  timestamp: number;
  direction: 'long' | 'short';
  
  // V1: SMC (25 points)
  v1_smc: {
    fvg: number;        // 0-7
    orderBlock: number; // 0-7
    bos: number;        // 0-6
    liquidity: number;  // 0-5
    total: number;      // Sum
  };
  
  // V2: Order Flow + Sentiment (25 points)
  v2_flow: {
    delta: number;      // 0-8
    darkPool: number;   // 0-7
    options: number;    // 0-5
    sentiment: number;  // 0-5
    total: number;      // Sum
  };
  
  // V3: Auction Theory (20 points)
  v3_auction: {
    volumeProfile: number;  // 0-7
    footprint: number;      // 0-7
    vwap: number;           // 0-6
    total: number;          // Sum
  };
  
  // V4: Quantum (15 points)
  v4_quantum: {
    probability: number;    // 0-5
    entanglement: number;   // 0-5
    tunneling: number;      // 0-5
    total: number;          // Sum
  };
  
  // V5: Psychology (15 points)
  v5_psychology: {
    emotionalState: number; // 0-5
    biasCheck: number;      // 0-5
    flowState: number;      // 0-5
    total: number;          // Sum
  };
  
  // Total score
  totalScore: number; // 0-100
  
  // Grade
  grade: 'S' | 'A' | 'B' | 'C' | 'D' | 'F';
  
  // Win probability
  winProbability: number; // 0-100
  
  // Recommended action
  action: 'STRONG_ENTRY' | 'ENTRY' | 'WATCH' | 'AVOID' | 'STAY_OUT';
  
  // Entry details
  entry: number;
  stopLoss: number;
  takeProfit1: number;
  takeProfit2: number;
  takeProfit3: number;
  riskReward: number;
  
  // Reasoning
  reasons: string[];
}

export class UltimateScorer {
  /**
   * Calculate ultimate score from all components
   */
  calculateUltimateScore(
    symbol: string,
    timeframe: string,
    v1: { fvg: number; orderBlock: number; bos: number; liquidity: number },
    v2: { delta: number; darkPool: number; options: number; sentiment: number },
    v3: { volumeProfile: number; footprint: number; vwap: number },
    v4: { probability: number; entanglement: number; tunneling: number },
    v5: { emotionalState: number; biasCheck: number; flowState: number },
    direction: 'long' | 'short',
    levels: { entry: number; stopLoss: number; tp1: number; tp2: number; tp3: number }
  ): UltimateSignal {
    // Calculate totals for each version
    const v1_total = v1.fvg + v1.orderBlock + v1.bos + v1.liquidity;
    const v2_total = v2.delta + v2.darkPool + v2.options + v2.sentiment;
    const v3_total = v3.volumeProfile + v3.footprint + v3.vwap;
    const v4_total = v4.probability + v4.entanglement + v4.tunneling;
    const v5_total = v5.emotionalState + v5.biasCheck + v5.flowState;
    
    const totalScore = v1_total + v2_total + v3_total + v4_total + v5_total;
    
    // Calculate grade
    const grade = this.calculateGrade(totalScore);
    
    // Calculate win probability
    const winProbability = this.calculateWinProbability(totalScore, grade);
    
    // Determine action
    const action = this.determineAction(totalScore, grade);
    
    // Calculate risk/reward
    const riskReward = Math.abs(levels.tp1 - levels.entry) / Math.abs(levels.entry - levels.stopLoss);
    
    // Build reasons
    const reasons = this.buildReasons(v1, v2, v3, v4, v5);
    
    return {
      symbol,
      timeframe,
      timestamp: Date.now(),
      direction,
      v1_smc: { ...v1, total: v1_total },
      v2_flow: { ...v2, total: v2_total },
      v3_auction: { ...v3, total: v3_total },
      v4_quantum: { ...v4, total: v4_total },
      v5_psychology: { ...v5, total: v5_total },
      totalScore,
      grade,
      winProbability,
      action,
      entry: levels.entry,
      stopLoss: levels.stopLoss,
      takeProfit1: levels.tp1,
      takeProfit2: levels.tp2,
      takeProfit3: levels.tp3,
      riskReward,
      reasons
    };
  }

  /**
   * Calculate grade from total score
   */
  private calculateGrade(score: number): 'S' | 'A' | 'B' | 'C' | 'D' | 'F' {
    if (score >= 90) return 'S'; // GOD TIER
    if (score >= 80) return 'A'; // EXCELLENT
    if (score >= 70) return 'B'; // GOOD
    if (score >= 60) return 'C'; // OKAY
    if (score >= 50) return 'D'; // WEAK
    return 'F'; // AVOID
  }

  /**
   * Calculate win probability from score and grade
   */
  private calculateWinProbability(score: number, grade: string): number {
    switch (grade) {
      case 'S': return 85 + ((score - 90) / 10) * 10; // 85-95%
      case 'A': return 75 + ((score - 80) / 10) * 10; // 75-85%
      case 'B': return 65 + ((score - 70) / 10) * 10; // 65-75%
      case 'C': return 55 + ((score - 60) / 10) * 10; // 55-65%
      case 'D': return 50 + ((score - 50) / 10) * 5;  // 50-55%
      default: return 50 - ((50 - score) / 50) * 20;  // <50%
    }
  }

  /**
   * Determine recommended action
   */
  private determineAction(
    score: number,
    grade: string
  ): 'STRONG_ENTRY' | 'ENTRY' | 'WATCH' | 'AVOID' | 'STAY_OUT' {
    if (score >= 85 && grade === 'S') return 'STRONG_ENTRY';
    if (score >= 75 && (grade === 'S' || grade === 'A')) return 'ENTRY';
    if (score >= 60) return 'WATCH';
    if (score >= 50) return 'AVOID';
    return 'STAY_OUT';
  }

  /**
   * Build human-readable reasons
   */
  private buildReasons(
    v1: { fvg: number; orderBlock: number; bos: number; liquidity: number },
    v2: { delta: number; darkPool: number; options: number; sentiment: number },
    v3: { volumeProfile: number; footprint: number; vwap: number },
    v4: { probability: number; entanglement: number; tunneling: number },
    v5: { emotionalState: number; biasCheck: number; flowState: number }
  ): string[] {
    const reasons: string[] = [];
    
    // V1 reasons
    if (v1.fvg >= 5) reasons.push(`Strong FVG signal (${v1.fvg}/7)`);
    if (v1.orderBlock >= 5) reasons.push(`Quality Order Block (${v1.orderBlock}/7)`);
    if (v1.bos >= 4) reasons.push(`Clear BOS/CHoCH (${v1.bos}/6)`);
    if (v1.liquidity >= 4) reasons.push(`Liquidity sweep detected (${v1.liquidity}/5)`);
    
    // V2 reasons
    if (v2.delta >= 6) reasons.push(`Strong delta flow (${v2.delta}/8)`);
    if (v2.darkPool >= 5) reasons.push(`Significant dark pool activity (${v2.darkPool}/7)`);
    if (v2.options >= 4) reasons.push(`Smart money options flow (${v2.options}/5)`);
    if (v2.sentiment >= 4) reasons.push(`Favorable sentiment (${v2.sentiment}/5)`);
    
    // V3 reasons
    if (v3.volumeProfile >= 5) reasons.push(`Volume Profile confluence (${v3.volumeProfile}/7)`);
    if (v3.footprint >= 5) reasons.push(`Footprint absorption detected (${v3.footprint}/7)`);
    if (v3.vwap >= 4) reasons.push(`VWAP alignment (${v3.vwap}/6)`);
    
    // V4 reasons
    if (v4.probability >= 4) reasons.push(`High quantum probability (${v4.probability}/5)`);
    if (v4.entanglement >= 4) reasons.push(`Asset correlation detected (${v4.entanglement}/5)`);
    if (v4.tunneling >= 4) reasons.push(`Barrier break probability high (${v4.tunneling}/5)`);
    
    // V5 reasons
    if (v5.emotionalState >= 4) reasons.push(`Optimal emotional state (${v5.emotionalState}/5)`);
    if (v5.biasCheck >= 4) reasons.push(`No significant biases detected (${v5.biasCheck}/5)`);
    if (v5.flowState >= 4) reasons.push(`In flow state (${v5.flowState}/5)`);
    
    if (reasons.length === 0) {
      reasons.push('Low confidence signal - multiple weak indicators');
    }
    
    return reasons;
  }

  /**
   * Batch score multiple signals
   */
  scoreMultiple(signals: any[]): UltimateSignal[] {
    // Implementation would score each signal
    // For now, return empty array
    return [];
  }

  /**
   * Filter signals by minimum score
   */
  filterByScore(signals: UltimateSignal[], minScore: number): UltimateSignal[] {
    return signals.filter(s => s.totalScore >= minScore);
  }

  /**
   * Filter signals by grade
   */
  filterByGrade(
    signals: UltimateSignal[],
    minGrade: 'S' | 'A' | 'B' | 'C' | 'D' | 'F'
  ): UltimateSignal[] {
    const gradeValues = { S: 6, A: 5, B: 4, C: 3, D: 2, F: 1 };
    const minValue = gradeValues[minGrade];
    
    return signals.filter(s => gradeValues[s.grade] >= minValue);
  }

  /**
   * Sort signals by score
   */
  sortByScore(signals: UltimateSignal[]): UltimateSignal[] {
    return [...signals].sort((a, b) => b.totalScore - a.totalScore);
  }

  /**
   * Get top N signals
   */
  getTopSignals(signals: UltimateSignal[], n: number): UltimateSignal[] {
    return this.sortByScore(signals).slice(0, n);
  }
}

/**
 * Quick helper to create mock scores for testing
 */
export function createMockScores(level: 'high' | 'medium' | 'low'): {
  v1: { fvg: number; orderBlock: number; bos: number; liquidity: number };
  v2: { delta: number; darkPool: number; options: number; sentiment: number };
  v3: { volumeProfile: number; footprint: number; vwap: number };
  v4: { probability: number; entanglement: number; tunneling: number };
  v5: { emotionalState: number; biasCheck: number; flowState: number };
} {
  if (level === 'high') {
    return {
      v1: { fvg: 6, orderBlock: 6, bos: 5, liquidity: 4 },
      v2: { delta: 7, darkPool: 6, options: 4, sentiment: 4 },
      v3: { volumeProfile: 6, footprint: 6, vwap: 5 },
      v4: { probability: 4, entanglement: 4, tunneling: 4 },
      v5: { emotionalState: 4, biasCheck: 4, flowState: 4 }
    };
  } else if (level === 'medium') {
    return {
      v1: { fvg: 4, orderBlock: 4, bos: 3, liquidity: 3 },
      v2: { delta: 5, darkPool: 4, options: 3, sentiment: 3 },
      v3: { volumeProfile: 4, footprint: 4, vwap: 3 },
      v4: { probability: 3, entanglement: 3, tunneling: 3 },
      v5: { emotionalState: 3, biasCheck: 3, flowState: 3 }
    };
  } else {
    return {
      v1: { fvg: 2, orderBlock: 2, bos: 1, liquidity: 1 },
      v2: { delta: 2, darkPool: 2, options: 1, sentiment: 1 },
      v3: { volumeProfile: 2, footprint: 2, vwap: 1 },
      v4: { probability: 1, entanglement: 1, tunneling: 1 },
      v5: { emotionalState: 1, biasCheck: 1, flowState: 1 }
    };
  }
}
