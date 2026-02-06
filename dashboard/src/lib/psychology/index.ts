export * from './biasDetector';
export * from './emotionalState';

import type { BiasDetection } from './biasDetector';
import type { EmotionalAnalysis } from './emotionalState';

export interface PsychologyAnalysis {
  bias_score: number;
  emotional_score: number;
  overall_score: number;
  signal: 'healthy' | 'caution' | 'danger';
}

export function aggregatePsychology(
  bias: BiasDetection,
  emotional: EmotionalAnalysis
): PsychologyAnalysis {
  const bias_score = bias.score;
  const emotional_score = emotional.score;
  
  const overall_score = (bias_score * 0.5 + emotional_score * 0.5);
  
  let signal: 'healthy' | 'caution' | 'danger' = 'healthy';
  if (overall_score < 40 || bias.signal === 'warning' || emotional.signal === 'extreme') {
    signal = 'danger';
  } else if (overall_score < 60 || bias.signal === 'caution' || emotional.signal === 'volatile') {
    signal = 'caution';
  }
  
  return {
    bias_score,
    emotional_score,
    overall_score,
    signal
  };
}
