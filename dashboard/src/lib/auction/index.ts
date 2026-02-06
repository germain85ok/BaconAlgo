export * from './volumeProfile';
export * from './vwap';

import type { VolumeProfileAnalysis } from './volumeProfile';
import type { VWAPAnalysis } from './vwap';

export interface AuctionAnalysis {
  volume_profile_score: number;
  vwap_score: number;
  overall_score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
}

export function aggregateAuction(
  volumeProfile: VolumeProfileAnalysis,
  vwap: VWAPAnalysis
): AuctionAnalysis {
  const volume_profile_score = volumeProfile.score;
  const vwap_score = vwap.score;
  
  const overall_score = (volume_profile_score * 0.5 + vwap_score * 0.5);
  
  const signals = [volumeProfile.signal, vwap.signal];
  const bullish = signals.filter(s => s === 'bullish').length;
  const bearish = signals.filter(s => s === 'bearish').length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullish > bearish) signal = 'bullish';
  else if (bearish > bullish) signal = 'bearish';
  
  return {
    volume_profile_score,
    vwap_score,
    overall_score,
    signal
  };
}
