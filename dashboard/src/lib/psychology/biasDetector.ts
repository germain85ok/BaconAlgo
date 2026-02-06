export interface CognitiveBias {
  name: string;
  type: 'confirmation' | 'anchoring' | 'recency' | 'overconfidence' | 'loss_aversion' | 'herd';
  severity: number; // 0-1
  description: string;
  detected: boolean;
}

export interface BiasDetection {
  biases: CognitiveBias[];
  score: number;
  signal: 'warning' | 'caution' | 'clear';
  totalBiases: number;
  riskLevel: 'low' | 'medium' | 'high';
}

export function detectBiases(
  tradingHistory: Array<{ profit: number; duration: number; conviction: number }>,
  marketCondition: 'trending' | 'ranging' | 'volatile'
): BiasDetection {
  const biases: CognitiveBias[] = [];
  
  // Confirmation bias: only seeing what confirms your view
  const recentWins = tradingHistory.slice(-10).filter(t => t.profit > 0).length;
  if (recentWins >= 7) {
    biases.push({
      name: 'Biais de confirmation',
      type: 'confirmation',
      severity: Math.min(1, recentWins / 10),
      description: 'Tendance à chercher uniquement des informations confirmant votre position',
      detected: true
    });
  }
  
  // Recency bias: overweighting recent events
  const recentAvgProfit = tradingHistory.slice(-5).reduce((s, t) => s + t.profit, 0) / 5;
  const overallAvgProfit = tradingHistory.reduce((s, t) => s + t.profit, 0) / tradingHistory.length;
  if (Math.abs(recentAvgProfit - overallAvgProfit) > overallAvgProfit * 0.5) {
    biases.push({
      name: 'Biais de récence',
      type: 'recency',
      severity: Math.min(1, Math.abs(recentAvgProfit - overallAvgProfit) / overallAvgProfit),
      description: 'Surpondération des événements récents',
      detected: true
    });
  }
  
  // Overconfidence: high conviction after wins
  const highConvictionTrades = tradingHistory.filter(t => t.conviction > 0.8).length;
  if (highConvictionTrades > tradingHistory.length * 0.6) {
    biases.push({
      name: 'Surconfiance',
      type: 'overconfidence',
      severity: highConvictionTrades / tradingHistory.length,
      description: 'Confiance excessive dans vos analyses',
      detected: true
    });
  }
  
  // Loss aversion: holding losers too long
  const avgWinDuration = tradingHistory.filter(t => t.profit > 0)
    .reduce((s, t) => s + t.duration, 0) / tradingHistory.filter(t => t.profit > 0).length;
  const avgLossDuration = tradingHistory.filter(t => t.profit < 0)
    .reduce((s, t) => s + t.duration, 0) / tradingHistory.filter(t => t.profit < 0).length;
  
  if (avgLossDuration > avgWinDuration * 1.5) {
    biases.push({
      name: 'Aversion à la perte',
      type: 'loss_aversion',
      severity: Math.min(1, avgLossDuration / avgWinDuration - 1),
      description: 'Tendance à garder les positions perdantes trop longtemps',
      detected: true
    });
  }
  
  // Herd mentality in trending markets
  if (marketCondition === 'trending') {
    const trendFollowingTrades = tradingHistory.filter(t => t.conviction > 0.7).length;
    if (trendFollowingTrades > tradingHistory.length * 0.7) {
      biases.push({
        name: 'Mentalité de troupeau',
        type: 'herd',
        severity: trendFollowingTrades / tradingHistory.length,
        description: 'Suivre la tendance sans analyse critique',
        detected: true
      });
    }
  }
  
  const totalBiases = biases.length;
  const avgSeverity = biases.reduce((s, b) => s + b.severity, 0) / (biases.length || 1);
  
  let signal: 'warning' | 'caution' | 'clear' = 'clear';
  if (totalBiases >= 3) signal = 'warning';
  else if (totalBiases >= 1) signal = 'caution';
  
  let riskLevel: 'low' | 'medium' | 'high' = 'low';
  if (avgSeverity > 0.7) riskLevel = 'high';
  else if (avgSeverity > 0.4) riskLevel = 'medium';
  
  const score = Math.max(0, 100 - totalBiases * 20 - avgSeverity * 30);
  
  return {
    biases,
    score,
    signal,
    totalBiases,
    riskLevel
  };
}
