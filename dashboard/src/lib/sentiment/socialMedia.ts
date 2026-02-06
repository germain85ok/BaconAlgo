export interface SocialMediaData {
  source: string;
  timestamp: number;
  mentions: number;
  sentiment: number; // -1 to 1
  engagement: number;
}

export interface SocialAnalysis {
  data: SocialMediaData[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  avgSentiment: number;
  totalMentions: number;
}

export function analyzeSocialMedia(
  data: SocialMediaData[]
): SocialAnalysis {
  const totalMentions = data.reduce((sum, d) => sum + d.mentions, 0);
  const avgSentiment = data.reduce((sum, d) => sum + d.sentiment, 0) / data.length;
  const weightedSentiment = data.reduce((sum, d) => sum + (d.sentiment * d.mentions), 0) / totalMentions;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (weightedSentiment > 0.3) signal = 'bullish';
  else if (weightedSentiment < -0.3) signal = 'bearish';
  
  const mentionScore = Math.min(50, (totalMentions / 1000) * 10);
  const sentimentScore = (weightedSentiment + 1) * 25;
  const score = mentionScore + sentimentScore;
  
  return {
    data,
    score,
    signal,
    avgSentiment: weightedSentiment,
    totalMentions
  };
}
