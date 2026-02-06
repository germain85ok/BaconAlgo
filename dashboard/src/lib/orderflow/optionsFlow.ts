export interface OptionsFlow {
  timestamp: number;
  calls: number;
  puts: number;
  callVolume: number;
  putVolume: number;
  pcRatio: number;
  sentiment: 'bullish' | 'bearish' | 'neutral';
}

export interface OptionsAnalysis {
  flows: OptionsFlow[];
  score: number;
  signal: 'bullish' | 'bearish' | 'neutral';
  avgPCRatio: number;
  currentPCRatio: number;
}

export function analyzeOptions(
  data: Array<{ timestamp: number; calls: number; puts: number; callVolume: number; putVolume: number }>
): OptionsAnalysis {
  const flows: OptionsFlow[] = data.map(d => {
    const pcRatio = d.putVolume > 0 ? d.callVolume / d.putVolume : 0;
    
    let sentiment: 'bullish' | 'bearish' | 'neutral' = 'neutral';
    if (pcRatio > 1.5) sentiment = 'bullish';
    else if (pcRatio < 0.7) sentiment = 'bearish';
    
    return {
      timestamp: d.timestamp,
      calls: d.calls,
      puts: d.puts,
      callVolume: d.callVolume,
      putVolume: d.putVolume,
      pcRatio,
      sentiment
    };
  });
  
  const avgPCRatio = flows.reduce((sum, f) => sum + f.pcRatio, 0) / flows.length;
  const currentPCRatio = flows[flows.length - 1]?.pcRatio || 1;
  
  const bullishFlows = flows.filter(f => f.sentiment === 'bullish').length;
  const bearishFlows = flows.filter(f => f.sentiment === 'bearish').length;
  
  let signal: 'bullish' | 'bearish' | 'neutral' = 'neutral';
  if (bullishFlows > bearishFlows * 1.5) signal = 'bullish';
  else if (bearishFlows > bullishFlows * 1.5) signal = 'bearish';
  
  const deviation = Math.abs(currentPCRatio - avgPCRatio);
  const score = Math.min(100, 50 + deviation * 20 + (bullishFlows + bearishFlows) * 2);
  
  return {
    flows,
    score,
    signal,
    avgPCRatio,
    currentPCRatio
  };
}
