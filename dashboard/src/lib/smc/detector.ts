/**
 * V1: Smart Money Concepts (SMC) Detector
 * 
 * Detects key SMC concepts:
 * - Fair Value Gaps (FVG)
 * - Order Blocks (OB)
 * - Break of Structure (BOS)
 * - Change of Character (CHoCH)
 * - Liquidity levels (BSL/SSL)
 * - Premium/Discount zones
 */

export interface Candle {
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export interface FairValueGap {
  type: 'bullish' | 'bearish';
  startIndex: number;
  endIndex: number;
  top: number;
  bottom: number;
  size: number;
  mitigated: boolean;
  strength: number; // 0-7
}

export interface OrderBlock {
  type: 'bullish' | 'bearish';
  index: number;
  high: number;
  low: number;
  volume: number;
  tested: boolean;
  strength: number; // 0-7
}

export interface StructureBreak {
  type: 'BOS' | 'CHoCH';
  direction: 'bullish' | 'bearish';
  index: number;
  level: number;
  strength: number; // 0-6
}

export interface LiquidityLevel {
  type: 'BSL' | 'SSL'; // Buy-Side Liquidity / Sell-Side Liquidity
  price: number;
  index: number;
  swept: boolean;
  strength: number; // 0-5
}

export interface PremiumDiscount {
  equilibrium: number;
  premium: { start: number; end: number };
  discount: { start: number; end: number };
  currentZone: 'premium' | 'discount' | 'equilibrium';
}

export class SMCDetector {
  /**
   * Detect Fair Value Gaps in price action
   */
  detectFVG(candles: Candle[], lookback: number = 50): FairValueGap[] {
    const fvgs: FairValueGap[] = [];
    
    for (let i = 2; i < Math.min(candles.length, lookback); i++) {
      const prev = candles[i - 2];
      const current = candles[i - 1];
      const next = candles[i];
      
      // Bullish FVG: gap between prev high and next low
      if (prev.high < next.low) {
        const size = next.low - prev.high;
        const strength = Math.min(7, Math.floor((size / prev.close) * 1000));
        
        fvgs.push({
          type: 'bullish',
          startIndex: i - 2,
          endIndex: i,
          top: next.low,
          bottom: prev.high,
          size,
          mitigated: false,
          strength
        });
      }
      
      // Bearish FVG: gap between prev low and next high
      if (prev.low > next.high) {
        const size = prev.low - next.high;
        const strength = Math.min(7, Math.floor((size / prev.close) * 1000));
        
        fvgs.push({
          type: 'bearish',
          startIndex: i - 2,
          endIndex: i,
          top: prev.low,
          bottom: next.high,
          size,
          mitigated: false,
          strength
        });
      }
    }
    
    return fvgs;
  }

  /**
   * Detect Order Blocks (high volume zones before strong moves)
   */
  detectOrderBlocks(candles: Candle[], lookback: number = 100): OrderBlock[] {
    const orderBlocks: OrderBlock[] = [];
    
    for (let i = 5; i < Math.min(candles.length, lookback); i++) {
      const current = candles[i];
      const prev = candles[i - 1];
      
      // Bullish OB: strong up move after consolidation
      const upMove = current.close > prev.close * 1.02;
      if (upMove) {
        const volumeStrength = Math.min(7, Math.floor(current.volume / 1000000));
        
        orderBlocks.push({
          type: 'bullish',
          index: i - 1,
          high: prev.high,
          low: prev.low,
          volume: prev.volume,
          tested: false,
          strength: volumeStrength
        });
      }
      
      // Bearish OB: strong down move after consolidation
      const downMove = current.close < prev.close * 0.98;
      if (downMove) {
        const volumeStrength = Math.min(7, Math.floor(current.volume / 1000000));
        
        orderBlocks.push({
          type: 'bearish',
          index: i - 1,
          high: prev.high,
          low: prev.low,
          volume: prev.volume,
          tested: false,
          strength: volumeStrength
        });
      }
    }
    
    return orderBlocks;
  }

  /**
   * Detect Break of Structure (BOS) and Change of Character (CHoCH)
   */
  detectStructureBreaks(candles: Candle[], lookback: number = 50): StructureBreak[] {
    const breaks: StructureBreak[] = [];
    
    // Find swing highs and lows
    const swings = this.findSwingPoints(candles, lookback);
    
    for (let i = 1; i < swings.length; i++) {
      const prev = swings[i - 1];
      const current = swings[i];
      
      // BOS: continuation of trend (higher high or lower low)
      if (prev.type === 'high' && current.type === 'high' && current.price > prev.price) {
        breaks.push({
          type: 'BOS',
          direction: 'bullish',
          index: current.index,
          level: current.price,
          strength: Math.min(6, Math.floor(((current.price - prev.price) / prev.price) * 100))
        });
      }
      
      if (prev.type === 'low' && current.type === 'low' && current.price < prev.price) {
        breaks.push({
          type: 'BOS',
          direction: 'bearish',
          index: current.index,
          level: current.price,
          strength: Math.min(6, Math.floor(((prev.price - current.price) / prev.price) * 100))
        });
      }
      
      // CHoCH: reversal (lower high or higher low)
      if (prev.type === 'high' && current.type === 'high' && current.price < prev.price) {
        breaks.push({
          type: 'CHoCH',
          direction: 'bearish',
          index: current.index,
          level: current.price,
          strength: Math.min(6, Math.floor(((prev.price - current.price) / prev.price) * 100))
        });
      }
      
      if (prev.type === 'low' && current.type === 'low' && current.price > prev.price) {
        breaks.push({
          type: 'CHoCH',
          direction: 'bullish',
          index: current.index,
          level: current.price,
          strength: Math.min(6, Math.floor(((current.price - prev.price) / prev.price) * 100))
        });
      }
    }
    
    return breaks;
  }

  /**
   * Detect liquidity levels (BSL/SSL)
   */
  detectLiquidity(candles: Candle[], lookback: number = 100): LiquidityLevel[] {
    const levels: LiquidityLevel[] = [];
    
    for (let i = 2; i < Math.min(candles.length, lookback); i++) {
      const prev = candles[i - 2];
      const current = candles[i - 1];
      const next = candles[i];
      
      // Buy-Side Liquidity (resistance - resting sell orders)
      if (current.high > prev.high && current.high > next.high) {
        const strength = Math.min(5, Math.floor(current.volume / 2000000));
        
        levels.push({
          type: 'BSL',
          price: current.high,
          index: i - 1,
          swept: false,
          strength
        });
      }
      
      // Sell-Side Liquidity (support - resting buy orders)
      if (current.low < prev.low && current.low < next.low) {
        const strength = Math.min(5, Math.floor(current.volume / 2000000));
        
        levels.push({
          type: 'SSL',
          price: current.low,
          index: i - 1,
          swept: false,
          strength
        });
      }
    }
    
    return levels;
  }

  /**
   * Calculate Premium/Discount zones based on range
   */
  calculatePremiumDiscount(candles: Candle[], lookback: number = 50): PremiumDiscount {
    const recentCandles = candles.slice(-lookback);
    const high = Math.max(...recentCandles.map(c => c.high));
    const low = Math.min(...recentCandles.map(c => c.low));
    const equilibrium = (high + low) / 2;
    
    const current = candles[candles.length - 1].close;
    
    let currentZone: 'premium' | 'discount' | 'equilibrium' = 'equilibrium';
    if (current > equilibrium * 1.01) currentZone = 'premium';
    if (current < equilibrium * 0.99) currentZone = 'discount';
    
    return {
      equilibrium,
      premium: { start: equilibrium, end: high },
      discount: { start: low, end: equilibrium },
      currentZone
    };
  }

  /**
   * Multi-timeframe analysis
   */
  analyzeMultiTimeframe(
    m1: Candle[],
    m5: Candle[],
    m15: Candle[],
    h1: Candle[]
  ): {
    alignment: number; // 0-100
    trend: 'bullish' | 'bearish' | 'neutral';
  } {
    const trends = [
      this.getTrend(m1),
      this.getTrend(m5),
      this.getTrend(m15),
      this.getTrend(h1)
    ];
    
    const bullishCount = trends.filter(t => t === 'bullish').length;
    const bearishCount = trends.filter(t => t === 'bearish').length;
    
    let trend: 'bullish' | 'bearish' | 'neutral' = 'neutral';
    let alignment = 0;
    
    if (bullishCount > bearishCount) {
      trend = 'bullish';
      alignment = (bullishCount / trends.length) * 100;
    } else if (bearishCount > bullishCount) {
      trend = 'bearish';
      alignment = (bearishCount / trends.length) * 100;
    }
    
    return { alignment, trend };
  }

  // Helper methods
  
  private findSwingPoints(candles: Candle[], lookback: number): Array<{type: 'high' | 'low', price: number, index: number}> {
    const swings: Array<{type: 'high' | 'low', price: number, index: number}> = [];
    
    for (let i = 2; i < Math.min(candles.length, lookback); i++) {
      const prev = candles[i - 2];
      const current = candles[i - 1];
      const next = candles[i];
      
      // Swing high
      if (current.high > prev.high && current.high > next.high) {
        swings.push({ type: 'high', price: current.high, index: i - 1 });
      }
      
      // Swing low
      if (current.low < prev.low && current.low < next.low) {
        swings.push({ type: 'low', price: current.low, index: i - 1 });
      }
    }
    
    return swings;
  }

  private getTrend(candles: Candle[]): 'bullish' | 'bearish' | 'neutral' {
    if (candles.length < 10) return 'neutral';
    
    const ema9 = this.calculateEMA(candles, 9);
    const ema21 = this.calculateEMA(candles, 21);
    
    if (ema9 > ema21 * 1.005) return 'bullish';
    if (ema9 < ema21 * 0.995) return 'bearish';
    return 'neutral';
  }

  private calculateEMA(candles: Candle[], period: number): number {
    if (candles.length < period) return candles[candles.length - 1].close;
    
    const k = 2 / (period + 1);
    let ema = candles.slice(-period).reduce((sum, c) => sum + c.close, 0) / period;
    
    for (let i = candles.length - period; i < candles.length; i++) {
      ema = candles[i].close * k + ema * (1 - k);
    }
    
    return ema;
  }
}
