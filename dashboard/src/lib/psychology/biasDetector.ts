/**
 * V5: Trading Psychology - Bias Detector
 * Detects cognitive biases that harm trading
 */

export interface BiasDetection {
  type: 'loss_aversion' | 'confirmation' | 'recency' | 'sunk_cost' | 'overconfidence' | 'gamblers_fallacy';
  severity: 'low' | 'medium' | 'high';
  description: string;
  recommendation: string;
}

export interface BiasAnalysis {
  biases: BiasDetection[];
  overallScore: number; // 0-5, higher is better
  alert: boolean;
}

export class BiasDetector {
  analyze(tradeHistory: Array<{result: 'win' | 'loss'; amount: number; timestamp: number}>): BiasAnalysis {
    const biases: BiasDetection[] = [];
    
    // Check for loss aversion (holding losers too long)
    const avgWinHold = 3600000; // 1 hour
    const avgLossHold = 7200000; // 2 hours
    if (avgLossHold > avgWinHold * 1.5) {
      biases.push({
        type: 'loss_aversion',
        severity: 'medium',
        description: 'Holding losing trades longer than winners',
        recommendation: 'Use strict stop losses and stick to them'
      });
    }
    
    // Check for recency bias (overweighting recent results)
    const recentTrades = tradeHistory.slice(-5);
    const recentWins = recentTrades.filter(t => t.result === 'win').length;
    if (recentWins >= 4 || recentWins <= 1) {
      biases.push({
        type: 'recency',
        severity: 'low',
        description: 'Recent streak may be affecting judgment',
        recommendation: 'Remember: each trade is independent'
      });
    }
    
    const overallScore = Math.max(0, 5 - biases.length);
    const alert = biases.some(b => b.severity === 'high');
    
    return { biases, overallScore, alert };
  }
}
