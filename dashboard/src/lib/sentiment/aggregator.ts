export interface SentimentSource {
  name: string;
  weight: number;
  sentiment: number; // -1 to 1
  confidence: number; // 0 to 1
}

export interface AggregatedSentiment {
  sources: SentimentSource[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  weightedSentiment: number;
  confidence: number;
}

export function aggregateSentiment(
  sources: SentimentSource[]
): AggregatedSentiment {
  const totalWeight = sources.reduce((sum, s) => sum + s.weight, 0);
  const weightedSentiment = sources.reduce(
    (sum, s) => sum + (s.sentiment * s.weight * s.confidence), 
    0
  ) / totalWeight;
  
  const confidence = sources.reduce((sum, s) => sum + s.confidence, 0) / sources.length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (weightedSentiment > 0.2) signal = 'bullish';
  else if (weightedSentiment < -0.2) signal = 'bearish';
  
  const score = ((weightedSentiment + 1) / 2) * 100 * confidence;
  
  return {
    sources,
    score,
    signal,
    weightedSentiment,
    confidence
  };
}
