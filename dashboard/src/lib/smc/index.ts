export * from './fvg';
export * from './orderBlock';
export * from './bos';
export * from './liquidity';

import type { FVGAnalysis } from './fvg';
import type { OrderBlockAnalysis } from './orderBlock';
import type { BOSAnalysis } from './bos';
import type { LiquidityAnalysis } from './liquidity';

export interface SMCAnalysis {
  fvg_score: number;
  order_block_score: number;
  bos_score: number;
  liquidity_score: number;
  overall_score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
}

export function aggregateSMC(
  fvg: FVGAnalysis,
  ob: OrderBlockAnalysis,
  bos: BOSAnalysis,
  liq: LiquidityAnalysis
): SMCAnalysis {
  const fvg_score = fvg.score;
  const order_block_score = ob.score;
  const bos_score = bos.score;
  const liquidity_score = liq.score;
  
  const overall_score = (fvg_score * 0.25 + order_block_score * 0.3 + bos_score * 0.25 + liquidity_score * 0.2);
  
  const signals = [fvg.signal, ob.signal, bos.signal, liq.signal];
  const bullish = signals.filter(s => s === 'bullish').length;
  const bearish = signals.filter(s => s === 'bearish').length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullish > bearish) signal = 'bullish';
  else if (bearish > bullish) signal = 'bearish';
  
  return {
    fvg_score,
    order_block_score,
    bos_score,
    liquidity_score,
    overall_score,
    signal
  };
}
