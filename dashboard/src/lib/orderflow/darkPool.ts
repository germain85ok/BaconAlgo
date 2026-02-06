/**
 * V2: Dark Pool Analysis
 * 
 * Tracks:
 * - Dark pool print detection
 * - DIX (Dark Index) tracking
 * - Institutional accumulation/distribution
 * - Hidden liquidity analysis
 */

export interface DarkPoolPrint {
  timestamp: number;
  symbol: string;
  size: number;
  price: number;
  venue: string;
  type: 'accumulation' | 'distribution';
}

export interface DIXData {
  timestamp: number;
  dix: number; // Dark pool index (0-1)
  gex: number; // Gamma exposure
  trend: 'bullish' | 'bearish' | 'neutral';
}

export interface DarkPoolAnalysis {
  symbol: string;
  
  // Recent prints
  recentPrints: DarkPoolPrint[];
  totalDarkVolume: number;
  darkVolumeRatio: number; // vs lit exchanges
  
  // DIX metrics
  dix: number;
  dixTrend: 'bullish' | 'bearish' | 'neutral';
  
  // Accumulation/Distribution
  accumulationScore: number; // -100 to +100
  institutionalBias: 'buying' | 'selling' | 'neutral';
  
  // Hidden liquidity
  estimatedHiddenSize: number;
  
  // Scoring (0-7)
  score: number;
}

export class DarkPoolAnalyzer {
  private dixHistory: number[] = [];
  
  /**
   * Analyze dark pool activity
   */
  analyzeDarkPool(
    symbol: string,
    darkPrints: DarkPoolPrint[],
    litVolume: number,
    currentPrice: number
  ): DarkPoolAnalysis {
    // Calculate dark volume metrics
    const totalDarkVolume = darkPrints.reduce((sum, p) => sum + p.size, 0);
    const darkVolumeRatio = litVolume > 0 ? totalDarkVolume / litVolume : 0;
    
    // Calculate accumulation/distribution
    const accumulationScore = this.calculateAccumulationScore(darkPrints, currentPrice);
    const institutionalBias = this.determineInstitutionalBias(accumulationScore);
    
    // Calculate DIX (simplified version)
    const dix = this.calculateDIX(darkPrints, litVolume);
    this.dixHistory.push(dix);
    if (this.dixHistory.length > 20) this.dixHistory.shift();
    
    const dixTrend = this.calculateDIXTrend();
    
    // Estimate hidden liquidity
    const estimatedHiddenSize = this.estimateHiddenLiquidity(darkPrints);
    
    // Score dark pool activity (0-7)
    const score = this.scoreDarkPool(
      darkVolumeRatio,
      Math.abs(accumulationScore),
      dix
    );
    
    return {
      symbol,
      recentPrints: darkPrints.slice(-10),
      totalDarkVolume,
      darkVolumeRatio,
      dix,
      dixTrend,
      accumulationScore,
      institutionalBias,
      estimatedHiddenSize,
      score
    };
  }

  /**
   * Detect significant dark pool prints
   */
  detectSignificantPrints(
    prints: DarkPoolPrint[],
    avgPrintSize: number
  ): DarkPoolPrint[] {
    const threshold = avgPrintSize * 5;
    return prints.filter(p => p.size > threshold);
  }

  /**
   * Calculate Dark Index (DIX)
   * Higher DIX = institutions buying on dips
   */
  private calculateDIX(prints: DarkPoolPrint[], litVolume: number): number {
    if (prints.length === 0) return 0.5;
    
    const darkVolume = prints.reduce((sum, p) => sum + p.size, 0);
    const totalVolume = darkVolume + litVolume;
    
    if (totalVolume === 0) return 0.5;
    
    // DIX is ratio of dark volume to total, adjusted for accumulation
    const baseRatio = darkVolume / totalVolume;
    const accumulationPrints = prints.filter(p => p.type === 'accumulation').length;
    const distributionPrints = prints.filter(p => p.type === 'distribution').length;
    
    const accumulationBias = accumulationPrints > distributionPrints ? 1.2 : 
                             distributionPrints > accumulationPrints ? 0.8 : 1.0;
    
    return Math.min(1, Math.max(0, baseRatio * accumulationBias));
  }

  /**
   * Calculate accumulation/distribution score
   */
  private calculateAccumulationScore(
    prints: DarkPoolPrint[],
    currentPrice: number
  ): number {
    if (prints.length === 0) return 0;
    
    let score = 0;
    
    for (const print of prints) {
      const priceDistance = ((currentPrice - print.price) / print.price) * 100;
      const volumeWeight = print.size / 1000000;
      
      if (print.type === 'accumulation') {
        // Buying below current price is more bullish
        score += priceDistance > 0 ? volumeWeight * 2 : volumeWeight;
      } else {
        // Selling above current price is more bearish
        score -= priceDistance < 0 ? volumeWeight * 2 : volumeWeight;
      }
    }
    
    return Math.max(-100, Math.min(100, score));
  }

  /**
   * Determine institutional bias
   */
  private determineInstitutionalBias(
    accumulationScore: number
  ): 'buying' | 'selling' | 'neutral' {
    if (accumulationScore > 20) return 'buying';
    if (accumulationScore < -20) return 'selling';
    return 'neutral';
  }

  /**
   * Calculate DIX trend
   */
  private calculateDIXTrend(): 'bullish' | 'bearish' | 'neutral' {
    if (this.dixHistory.length < 5) return 'neutral';
    
    const recent = this.dixHistory.slice(-5);
    const avg = recent.reduce((sum, d) => sum + d, 0) / recent.length;
    const previous = this.dixHistory.slice(-10, -5);
    const prevAvg = previous.length > 0 ? 
      previous.reduce((sum, d) => sum + d, 0) / previous.length : avg;
    
    if (avg > prevAvg * 1.1 && avg > 0.55) return 'bullish';
    if (avg < prevAvg * 0.9 && avg < 0.45) return 'bearish';
    return 'neutral';
  }

  /**
   * Estimate hidden liquidity
   */
  private estimateHiddenLiquidity(prints: DarkPoolPrint[]): number {
    // Iceberg orders typically 5-10x visible size
    const totalDarkVolume = prints.reduce((sum, p) => sum + p.size, 0);
    return totalDarkVolume * 7; // Conservative estimate
  }

  /**
   * Score dark pool activity
   */
  private scoreDarkPool(
    darkVolumeRatio: number,
    accumulationStrength: number,
    dix: number
  ): number {
    let score = 0;
    
    // Dark volume ratio (0-3)
    if (darkVolumeRatio > 0.3) score += 1;
    if (darkVolumeRatio > 0.5) score += 1;
    if (darkVolumeRatio > 0.7) score += 1;
    
    // Accumulation strength (0-2)
    if (accumulationStrength > 30) score += 1;
    if (accumulationStrength > 60) score += 1;
    
    // DIX level (0-2)
    if (dix > 0.6 || dix < 0.4) score += 1;
    if (dix > 0.7 || dix < 0.3) score += 1;
    
    return Math.min(7, score);
  }

  reset() {
    this.dixHistory = [];
  }
}
