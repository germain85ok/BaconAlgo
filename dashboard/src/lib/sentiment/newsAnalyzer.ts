/**
 * V2: News Sentiment Analyzer
 */

export interface NewsArticle {
  timestamp: number;
  headline: string;
  source: string;
  sentiment: number; // -1 to 1
  relevance: number; // 0-1
}

export interface NewsSentimentAnalysis {
  symbol: string;
  overallSentiment: number; // -100 to 100
  trend: 'bullish' | 'bearish' | 'neutral';
  velocity: number; // News per hour
  topHeadlines: NewsArticle[];
  score: number; // 0-5 (part of sentiment total)
}

export class NewsAnalyzer {
  analyze(symbol: string, articles: NewsArticle[]): NewsSentimentAnalysis {
    // Calculate weighted sentiment
    const totalWeight = articles.reduce((sum, a) => sum + a.relevance, 0);
    const weightedSentiment = articles.reduce((sum, a) => sum + (a.sentiment * a.relevance), 0);
    const overallSentiment = totalWeight > 0 ? (weightedSentiment / totalWeight) * 100 : 0;
    
    // Determine trend
    const trend = overallSentiment > 20 ? 'bullish' : overallSentiment < -20 ? 'bearish' : 'neutral';
    
    // Calculate velocity (news per hour)
    const timeRange = articles.length > 1 ? 
      (articles[articles.length - 1].timestamp - articles[0].timestamp) / 3600000 : 1;
    const velocity = articles.length / timeRange;
    
    // Get top headlines
    const topHeadlines = [...articles]
      .sort((a, b) => b.relevance - a.relevance)
      .slice(0, 5);
    
    // Score based on sentiment strength and velocity
    let score = 0;
    if (Math.abs(overallSentiment) > 40) score += 2;
    else if (Math.abs(overallSentiment) > 20) score += 1;
    if (velocity > 5) score += 2;
    else if (velocity > 2) score += 1;
    
    return {
      symbol,
      overallSentiment,
      trend,
      velocity,
      topHeadlines,
      score: Math.min(5, score)
    };
  }
}
