/**
 * V2: Social Sentiment Analyzer
 * Twitter/X, Reddit, StockTwits
 */

export interface SocialPost {
  platform: 'twitter' | 'reddit' | 'stocktwits';
  timestamp: number;
  content: string;
  sentiment: number; // -1 to 1
  engagement: number; // Likes, upvotes, etc.
  author: string;
}

export interface SocialSentimentAnalysis {
  symbol: string;
  overallSentiment: number; // -100 to 100
  twitterSentiment: number;
  redditSentiment: number;
  stocktwitsSentiment: number;
  volume: number; // Posts per hour
  trending: boolean;
  influencerBuzz: boolean;
  score: number; // 0-5 (part of sentiment total)
}

export class SocialSentimentAnalyzer {
  analyze(symbol: string, posts: SocialPost[]): SocialSentimentAnalysis {
    const twitter = posts.filter(p => p.platform === 'twitter');
    const reddit = posts.filter(p => p.platform === 'reddit');
    const stocktwits = posts.filter(p => p.platform === 'stocktwits');
    
    const calcSentiment = (posts: SocialPost[]) => {
      if (posts.length === 0) return 0;
      const avg = posts.reduce((sum, p) => sum + p.sentiment, 0) / posts.length;
      return avg * 100;
    };
    
    const twitterSentiment = calcSentiment(twitter);
    const redditSentiment = calcSentiment(reddit);
    const stocktwitsSentiment = calcSentiment(stocktwits);
    
    const overallSentiment = (twitterSentiment + redditSentiment + stocktwitsSentiment) / 3;
    
    // Calculate volume
    const timeRange = posts.length > 1 ? 
      (posts[posts.length - 1].timestamp - posts[0].timestamp) / 3600000 : 1;
    const volume = posts.length / timeRange;
    
    const trending = volume > 100;
    const influencerBuzz = posts.some(p => p.engagement > 1000);
    
    // Score
    let score = 0;
    if (Math.abs(overallSentiment) > 50) score += 2;
    else if (Math.abs(overallSentiment) > 30) score += 1;
    if (trending) score += 2;
    if (influencerBuzz) score += 1;
    
    return {
      symbol,
      overallSentiment,
      twitterSentiment,
      redditSentiment,
      stocktwitsSentiment,
      volume,
      trending,
      influencerBuzz,
      score: Math.min(5, score)
    };
  }
}
