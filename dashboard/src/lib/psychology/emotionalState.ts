export interface EmotionalState {
  emotion: 'fear' | 'greed' | 'neutral' | 'panic' | 'euphoria';
  intensity: number; // 0-1
  timestamp: number;
}

export interface EmotionalAnalysis {
  states: EmotionalState[];
  score: number;
  signal: 'stable' | 'volatile' | 'extreme';
  currentEmotion: string;
  fearGreedIndex: number; // 0-100
}

export function trackEmotionalState(
  marketData: Array<{ volatility: number; volume: number; priceChange: number; timestamp: number }>
): EmotionalAnalysis {
  const states: EmotionalState[] = [];
  
  marketData.forEach(data => {
    let emotion: 'fear' | 'greed' | 'neutral' | 'panic' | 'euphoria' = 'neutral';
    let intensity = 0.5;
    
    const volChange = data.volatility;
    const priceChange = data.priceChange;
    
    // Extreme fear/panic: high volatility + big drop
    if (volChange > 30 && priceChange < -3) {
      emotion = 'panic';
      intensity = Math.min(1, (Math.abs(priceChange) + volChange) / 50);
    }
    // Fear: moderate volatility + drop
    else if (volChange > 20 && priceChange < -1) {
      emotion = 'fear';
      intensity = Math.min(1, (Math.abs(priceChange) + volChange) / 40);
    }
    // Euphoria: low volatility + big gain
    else if (volChange < 15 && priceChange > 3) {
      emotion = 'euphoria';
      intensity = Math.min(1, priceChange / 10);
    }
    // Greed: moderate volatility + gain
    else if (priceChange > 1) {
      emotion = 'greed';
      intensity = Math.min(1, priceChange / 5);
    }
    // Fear: negative but controlled
    else if (priceChange < -0.5) {
      emotion = 'fear';
      intensity = Math.min(1, Math.abs(priceChange) / 3);
    }
    
    states.push({
      emotion,
      intensity,
      timestamp: data.timestamp
    });
  });
  
  const fearStates = states.filter(s => s.emotion === 'fear' || s.emotion === 'panic').length;
  const greedStates = states.filter(s => s.emotion === 'greed' || s.emotion === 'euphoria').length;
  
  const fearGreedIndex = ((greedStates - fearStates) / states.length) * 50 + 50;
  
  const currentEmotion = states[states.length - 1]?.emotion || 'neutral';
  const avgIntensity = states.reduce((s, st) => s + st.intensity, 0) / states.length;
  
  let signal: 'stable' | 'volatile' | 'extreme' = 'stable';
  if (avgIntensity > 0.8) signal = 'extreme';
  else if (avgIntensity > 0.5) signal = 'volatile';
  
  // Score is higher when emotions are neutral and stable
  const score = Math.max(0, 100 - avgIntensity * 60 - Math.abs(fearGreedIndex - 50) * 0.5);
  
  return {
    states,
    score,
    signal,
    currentEmotion,
    fearGreedIndex
  };
}
