export * from './delta';
export * from './darkPool';
export * from './optionsFlow';

import type { DeltaAnalysis } from './delta';
import type { DarkPoolAnalysis } from './darkPool';
import type { OptionsAnalysis } from './optionsFlow';

export interface OrderFlowAnalysis {
  delta_score: number;
  darkpool_score: number;
  options_score: number;
  overall_score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
}

export function aggregateOrderFlow(
  delta: DeltaAnalysis,
  darkPool: DarkPoolAnalysis,
  options: OptionsAnalysis
): OrderFlowAnalysis {
  const delta_score = delta.score;
  const darkpool_score = darkPool.score;
  const options_score = options.score;
  
  const overall_score = (delta_score * 0.4 + darkpool_score * 0.35 + options_score * 0.25);
  
  const signals = [delta.signal, darkPool.signal, options.signal];
  const bullish = signals.filter(s => s === 'bullish').length;
  const bearish = signals.filter(s => s === 'bearish').length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullish > bearish) signal = 'bullish';
  else if (bearish > bullish) signal = 'bearish';
  
  return {
    delta_score,
    darkpool_score,
    options_score,
    overall_score,
    signal
  };
}
