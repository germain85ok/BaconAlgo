export interface UltimateScore {
  score: number;  // 0-100
  grade: 'S' | 'A' | 'B' | 'C' | 'D' | 'F';
  win_probability: number;
  v1_smc: number;
  v2_orderflow: number;
  v2_sentiment: number;
  v3_auction: number;
  v4_quantum: number;
  v5_psychology: number;
  confidence_interval: [number, number];
  signal: 'bullish' | 'bearish' | 'neutral';
  recommendation: string;
}

export interface ScoreWeights {
  smc: number;
  orderflow: number;
  sentiment: number;
  auction: number;
  quantum: number;
  psychology: number;
}

const DEFAULT_WEIGHTS: ScoreWeights = {
  smc: 0.25,
  orderflow: 0.20,
  sentiment: 0.10,
  auction: 0.20,
  quantum: 0.15,
  psychology: 0.10
};

export function calculateUltimateScore(
  v1_smc: number,
  v2_orderflow: number,
  v2_sentiment: number,
  v3_auction: number,
  v4_quantum: number,
  v5_psychology: number,
  smcSignal: 'bullish' | 'bearish' | 'neutral',
  orderflowSignal: 'bullish' | 'bearish' | 'neutral',
  sentimentSignal: 'bullish' | 'bearish' | 'neutral',
  auctionSignal: 'bullish' | 'bearish' | 'neutral',
  quantumSignal: 'bullish' | 'bearish' | 'neutral',
  psychologySignal: 'healthy' | 'caution' | 'danger',
  weights: ScoreWeights = DEFAULT_WEIGHTS
): UltimateScore {
  // Calculate weighted score
  const score = 
    v1_smc * weights.smc +
    v2_orderflow * weights.orderflow +
    v2_sentiment * weights.sentiment +
    v3_auction * weights.auction +
    v4_quantum * weights.quantum +
    v5_psychology * weights.psychology;
  
  // Determine grade
  let grade: 'S' | 'A' | 'B' | 'C' | 'D' | 'F';
  if (score >= 95) grade = 'S';
  else if (score >= 85) grade = 'A';
  else if (score >= 75) grade = 'B';
  else if (score >= 65) grade = 'C';
  else if (score >= 55) grade = 'D';
  else grade = 'F';
  
  // Calculate win probability
  const base_probability = score / 100;
  
  // Adjust based on signal consensus
  const signals = [
    smcSignal === 'bullish' ? 1 : smcSignal === 'bearish' ? -1 : 0,
    orderflowSignal === 'bullish' ? 1 : orderflowSignal === 'bearish' ? -1 : 0,
    sentimentSignal === 'bullish' ? 1 : sentimentSignal === 'bearish' ? -1 : 0,
    auctionSignal === 'bullish' ? 1 : auctionSignal === 'bearish' ? -1 : 0,
    quantumSignal === 'bullish' ? 1 : quantumSignal === 'bearish' ? -1 : 0
  ];
  
  const signalSum = signals.reduce((sum, s) => sum + s, 0);
  const signalConsensus = signalSum / signals.length;
  
  let overall_signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (signalConsensus > 0.3) overall_signal = 'bullish';
  else if (signalConsensus < -0.3) overall_signal = 'bearish';
  
  // Adjust probability based on consensus
  const consensus_boost = Math.abs(signalConsensus) * 0.15;
  const win_probability = Math.min(0.95, Math.max(0.05, base_probability + consensus_boost));
  
  // Calculate confidence interval
  const scores = [v1_smc, v2_orderflow, v2_sentiment, v3_auction, v4_quantum, v5_psychology];
  const mean = scores.reduce((sum, s) => sum + s, 0) / scores.length;
  const variance = scores.reduce((sum, s) => sum + Math.pow(s - mean, 2), 0) / scores.length;
  const stdDev = Math.sqrt(variance);
  
  const confidence_interval: [number, number] = [
    Math.max(0, score - stdDev * 1.96),
    Math.min(100, score + stdDev * 1.96)
  ];
  
  // Generate recommendation
  let recommendation = '';
  
  if (grade === 'S' || grade === 'A') {
    if (overall_signal === 'bullish') {
      recommendation = 'Signal EXCELLENT - Position longue fortement recommandée';
    } else if (overall_signal === 'bearish') {
      recommendation = 'Signal EXCELLENT - Position courte fortement recommandée';
    } else {
      recommendation = 'Excellente configuration - Attendre confirmation directionnelle';
    }
  } else if (grade === 'B' || grade === 'C') {
    if (overall_signal === 'bullish') {
      recommendation = 'Signal MODÉRÉ - Position longue avec précaution';
    } else if (overall_signal === 'bearish') {
      recommendation = 'Signal MODÉRÉ - Position courte avec précaution';
    } else {
      recommendation = 'Configuration mitigée - Réduire la taille de position';
    }
  } else {
    recommendation = 'Signal FAIBLE - Éviter le trade ou attendre amélioration';
  }
  
  // Psychology check
  if (psychologySignal === 'danger') {
    recommendation += ' ⚠️ ATTENTION: Biais psychologiques détectés';
  } else if (psychologySignal === 'caution') {
    recommendation += ' ⚡ État émotionnel à surveiller';
  }
  
  return {
    score: Math.round(score * 100) / 100,
    grade,
    win_probability: Math.round(win_probability * 10000) / 100,
    v1_smc,
    v2_orderflow,
    v2_sentiment,
    v3_auction,
    v4_quantum,
    v5_psychology,
    confidence_interval,
    signal: overall_signal,
    recommendation
  };
}

export function getGradeColor(grade: string): string {
  switch (grade) {
    case 'S': return '#ff6b35'; // Orange neon
    case 'A': return '#4ecdc4';
    case 'B': return '#95e1d3';
    case 'C': return '#f9ca24';
    case 'D': return '#f38181';
    case 'F': return '#e74c3c';
    default: return '#95a5a6';
  }
}

export function getSignalColor(signal: string): string {
  switch (signal) {
    case 'bullish': return '#26a69a';
    case 'bearish': return '#ef5350';
    case 'neutral': return '#95a5a6';
    default: return '#95a5a6';
  }
}
