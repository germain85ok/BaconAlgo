/**
 * V5: Emotional State Tracker
 * Pre-trade emotional check-in
 */

export interface EmotionalState {
  timestamp: number;
  stress: number; // 0-10
  fomo: number; // 0-10
  fear: number; // 0-10
  confidence: number; // 0-10
  fatigue: number; // 0-10
  overall: 'optimal' | 'good' | 'caution' | 'stop_trading';
  score: number; // 0-5
}

export class EmotionalStateTracker {
  assess(stress: number, fomo: number, fear: number, confidence: number, fatigue: number): EmotionalState {
    // Calculate overall state
    let overall: 'optimal' | 'good' | 'caution' | 'stop_trading' = 'optimal';
    
    if (stress > 7 || fomo > 7 || fear > 7 || fatigue > 7) {
      overall = 'stop_trading';
    } else if (stress > 5 || fomo > 5 || fear > 5 || fatigue > 5) {
      overall = 'caution';
    } else if (stress > 3 || fomo > 3 || fear > 3 || fatigue > 3) {
      overall = 'good';
    }
    
    // Score (lower negative emotions = higher score)
    const negativeScore = (stress + fomo + fear + fatigue) / 4;
    const score = Math.max(0, Math.min(5, 5 - (negativeScore / 2)));
    
    return {
      timestamp: Date.now(),
      stress,
      fomo,
      fear,
      confidence,
      fatigue,
      overall,
      score
    };
  }
}
