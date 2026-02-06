export * from './probability';
export * from './barrier';

import type { ProbabilityAnalysis } from './probability';
import type { BarrierAnalysis } from './barrier';

export interface QuantumAnalysis {
  probability_score: number;
  barrier_score: number;
  overall_score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
}

export function aggregateQuantum(
  probability: ProbabilityAnalysis,
  barrier: BarrierAnalysis
): QuantumAnalysis {
  const probability_score = probability.score;
  const barrier_score = barrier.score;
  
  const overall_score = (probability_score * 0.5 + barrier_score * 0.5);
  
  const signals = [probability.signal, barrier.signal];
  const bullish = signals.filter(s => s === 'bullish').length;
  const bearish = signals.filter(s => s === 'bearish').length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullish > bearish) signal = 'bullish';
  else if (bearish > bullish) signal = 'bearish';
  
  return {
    probability_score,
    barrier_score,
    overall_score,
    signal
  };
}
