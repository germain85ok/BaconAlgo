/**
 * V1: SMC Signal Generation
 * 
 * Generates trading signals with:
 * - Entry/Exit calculations
 * - Stop Loss placement
 * - Take Profit levels (TP1, TP2, TP3)
 * - Risk-Reward calculation
 * - Signal scoring (0-100)
 */

import type { Candle, FairValueGap, OrderBlock, StructureBreak, LiquidityLevel, PremiumDiscount } from './detector';

export interface SMCSignal {
  symbol: string;
  timeframe: string;
  timestamp: number;
  
  // Signal type
  direction: 'long' | 'short';
  confidence: number; // 0-100
  
  // Entry details
  entry: number;
  stopLoss: number;
  takeProfit1: number;
  takeProfit2: number;
  takeProfit3: number;
  
  // Risk metrics
  riskReward: number;
  riskAmount: number;
  
  // SMC components (scoring 0-25 total)
  scores: {
    fvg: number;        // 0-7
    orderBlock: number; // 0-7
    bos: number;        // 0-6
    liquidity: number;  // 0-5
  };
  
  totalScore: number; // Sum of all scores
  
  // Supporting data
  context: {
    fvg?: FairValueGap;
    orderBlock?: OrderBlock;
    structureBreak?: StructureBreak;
    liquidityLevel?: LiquidityLevel;
    premiumDiscount?: PremiumDiscount;
  };
  
  // Reason for signal
  reason: string;
}

export class SMCSignalGenerator {
  /**
   * Generate trading signal from SMC analysis
   */
  generateSignal(
    symbol: string,
    timeframe: string,
    candles: Candle[],
    fvgs: FairValueGap[],
    orderBlocks: OrderBlock[],
    structureBreaks: StructureBreak[],
    liquidityLevels: LiquidityLevel[],
    premiumDiscount: PremiumDiscount
  ): SMCSignal | null {
    const current = candles[candles.length - 1];
    
    // Find most relevant FVG
    const activeFVG = this.findNearestFVG(fvgs, current.close);
    
    // Find most relevant order block
    const activeOB = this.findNearestOrderBlock(orderBlocks, current.close);
    
    // Find most recent structure break
    const recentBreak = structureBreaks[structureBreaks.length - 1];
    
    // Find nearest liquidity level
    const nearestLiquidity = this.findNearestLiquidity(liquidityLevels, current.close);
    
    // Score each component
    const scores = {
      fvg: activeFVG ? activeFVG.strength : 0,
      orderBlock: activeOB ? activeOB.strength : 0,
      bos: recentBreak ? recentBreak.strength : 0,
      liquidity: nearestLiquidity ? nearestLiquidity.strength : 0
    };
    
    const totalScore = scores.fvg + scores.orderBlock + scores.bos + scores.liquidity;
    
    // Require minimum score to generate signal
    if (totalScore < 10) return null;
    
    // Determine direction based on confluence
    const direction = this.determineDirection(
      activeFVG,
      activeOB,
      recentBreak,
      premiumDiscount
    );
    
    if (!direction) return null;
    
    // Calculate entry, stop loss, and take profits
    const { entry, stopLoss, tp1, tp2, tp3 } = this.calculateLevels(
      direction,
      current,
      activeFVG,
      activeOB,
      nearestLiquidity
    );
    
    const riskAmount = Math.abs(entry - stopLoss);
    const riskReward = Math.abs(tp1 - entry) / riskAmount;
    
    // Build reason string
    const reason = this.buildReason(activeFVG, activeOB, recentBreak, premiumDiscount);
    
    return {
      symbol,
      timeframe,
      timestamp: current.time,
      direction,
      confidence: totalScore * 4, // Scale to 0-100
      entry,
      stopLoss,
      takeProfit1: tp1,
      takeProfit2: tp2,
      takeProfit3: tp3,
      riskReward,
      riskAmount,
      scores,
      totalScore,
      context: {
        fvg: activeFVG,
        orderBlock: activeOB,
        structureBreak: recentBreak,
        liquidityLevel: nearestLiquidity,
        premiumDiscount
      },
      reason
    };
  }

  /**
   * Calculate entry and exit levels
   */
  private calculateLevels(
    direction: 'long' | 'short',
    current: Candle,
    fvg?: FairValueGap,
    orderBlock?: OrderBlock,
    liquidity?: LiquidityLevel
  ): {
    entry: number;
    stopLoss: number;
    tp1: number;
    tp2: number;
    tp3: number;
  } {
    let entry = current.close;
    let stopLoss: number;
    let tp1: number;
    let tp2: number;
    let tp3: number;
    
    if (direction === 'long') {
      // Entry at FVG bottom or current price
      if (fvg && fvg.type === 'bullish') {
        entry = fvg.bottom;
      }
      
      // Stop loss below order block or structure
      if (orderBlock && orderBlock.type === 'bullish') {
        stopLoss = orderBlock.low * 0.998;
      } else {
        stopLoss = entry * 0.99;
      }
      
      const risk = entry - stopLoss;
      
      // Take profits at 2R, 3R, 5R
      tp1 = entry + (risk * 2);
      tp2 = entry + (risk * 3);
      tp3 = entry + (risk * 5);
      
    } else {
      // Entry at FVG top or current price
      if (fvg && fvg.type === 'bearish') {
        entry = fvg.top;
      }
      
      // Stop loss above order block or structure
      if (orderBlock && orderBlock.type === 'bearish') {
        stopLoss = orderBlock.high * 1.002;
      } else {
        stopLoss = entry * 1.01;
      }
      
      const risk = stopLoss - entry;
      
      // Take profits at 2R, 3R, 5R
      tp1 = entry - (risk * 2);
      tp2 = entry - (risk * 3);
      tp3 = entry - (risk * 5);
    }
    
    return { entry, stopLoss, tp1, tp2, tp3 };
  }

  /**
   * Determine signal direction based on confluence
   */
  private determineDirection(
    fvg?: FairValueGap,
    orderBlock?: OrderBlock,
    structureBreak?: StructureBreak,
    premiumDiscount?: PremiumDiscount
  ): 'long' | 'short' | null {
    let bullishSignals = 0;
    let bearishSignals = 0;
    
    // FVG direction
    if (fvg) {
      if (fvg.type === 'bullish') bullishSignals++;
      if (fvg.type === 'bearish') bearishSignals++;
    }
    
    // Order block direction
    if (orderBlock) {
      if (orderBlock.type === 'bullish') bullishSignals++;
      if (orderBlock.type === 'bearish') bearishSignals++;
    }
    
    // Structure break direction
    if (structureBreak) {
      if (structureBreak.direction === 'bullish') bullishSignals++;
      if (structureBreak.direction === 'bearish') bearishSignals++;
    }
    
    // Premium/Discount zone preference
    if (premiumDiscount) {
      if (premiumDiscount.currentZone === 'discount') bullishSignals++;
      if (premiumDiscount.currentZone === 'premium') bearishSignals++;
    }
    
    // Need at least 2 confirming signals
    if (bullishSignals >= 2 && bullishSignals > bearishSignals) return 'long';
    if (bearishSignals >= 2 && bearishSignals > bullishSignals) return 'short';
    
    return null;
  }

  /**
   * Find nearest Fair Value Gap to price
   */
  private findNearestFVG(fvgs: FairValueGap[], price: number): FairValueGap | undefined {
    let nearest: FairValueGap | undefined;
    let minDistance = Infinity;
    
    for (const fvg of fvgs) {
      if (fvg.mitigated) continue;
      
      const distance = Math.min(
        Math.abs(price - fvg.top),
        Math.abs(price - fvg.bottom)
      );
      
      if (distance < minDistance) {
        minDistance = distance;
        nearest = fvg;
      }
    }
    
    return nearest;
  }

  /**
   * Find nearest Order Block to price
   */
  private findNearestOrderBlock(orderBlocks: OrderBlock[], price: number): OrderBlock | undefined {
    let nearest: OrderBlock | undefined;
    let minDistance = Infinity;
    
    for (const ob of orderBlocks) {
      if (ob.tested) continue;
      
      const distance = Math.min(
        Math.abs(price - ob.high),
        Math.abs(price - ob.low)
      );
      
      if (distance < minDistance) {
        minDistance = distance;
        nearest = ob;
      }
    }
    
    return nearest;
  }

  /**
   * Find nearest Liquidity level to price
   */
  private findNearestLiquidity(levels: LiquidityLevel[], price: number): LiquidityLevel | undefined {
    let nearest: LiquidityLevel | undefined;
    let minDistance = Infinity;
    
    for (const level of levels) {
      if (level.swept) continue;
      
      const distance = Math.abs(price - level.price);
      
      if (distance < minDistance) {
        minDistance = distance;
        nearest = level;
      }
    }
    
    return nearest;
  }

  /**
   * Build human-readable reason for signal
   */
  private buildReason(
    fvg?: FairValueGap,
    orderBlock?: OrderBlock,
    structureBreak?: StructureBreak,
    premiumDiscount?: PremiumDiscount
  ): string {
    const reasons: string[] = [];
    
    if (fvg) {
      reasons.push(`${fvg.type.toUpperCase()} FVG detected (strength: ${fvg.strength}/7)`);
    }
    
    if (orderBlock) {
      reasons.push(`${orderBlock.type.toUpperCase()} Order Block (strength: ${orderBlock.strength}/7)`);
    }
    
    if (structureBreak) {
      reasons.push(`${structureBreak.type} - ${structureBreak.direction} (strength: ${structureBreak.strength}/6)`);
    }
    
    if (premiumDiscount) {
      reasons.push(`In ${premiumDiscount.currentZone.toUpperCase()} zone`);
    }
    
    return reasons.join(' + ');
  }

  /**
   * Batch analyze multiple symbols
   */
  async analyzeMultiple(
    symbols: string[],
    timeframe: string,
    candlesMap: Map<string, Candle[]>
  ): Promise<SMCSignal[]> {
    const signals: SMCSignal[] = [];
    
    // This would normally import SMCDetector, but for simplicity we'll create instance
    const { SMCDetector } = await import('./detector');
    const detector = new SMCDetector();
    
    for (const symbol of symbols) {
      const candles = candlesMap.get(symbol);
      if (!candles || candles.length < 50) continue;
      
      // Run all detections
      const fvgs = detector.detectFVG(candles);
      const orderBlocks = detector.detectOrderBlocks(candles);
      const structureBreaks = detector.detectStructureBreaks(candles);
      const liquidityLevels = detector.detectLiquidity(candles);
      const premiumDiscount = detector.calculatePremiumDiscount(candles);
      
      // Generate signal
      const signal = this.generateSignal(
        symbol,
        timeframe,
        candles,
        fvgs,
        orderBlocks,
        structureBreaks,
        liquidityLevels,
        premiumDiscount
      );
      
      if (signal) {
        signals.push(signal);
      }
    }
    
    // Sort by confidence
    return signals.sort((a, b) => b.confidence - a.confidence);
  }
}
