<script lang="ts">
  import { onMount } from 'svelte';
  import { calculateUltimateScore, getGradeColor, getSignalColor } from '$lib/scoring';
  
  interface ScanResult {
    symbol: string;
    price: number;
    ultimate: any;
    v1_smc: number;
    v2_orderflow: number;
    v2_sentiment: number;
    v3_auction: number;
    v4_quantum: number;
    v5_psychology: number;
  }
  
  let scanResults: ScanResult[] = [];
  let filters = {
    minScore: 0,
    maxScore: 100,
    signal: 'all',
    minGrade: 'F',
    showDetails: true
  };
  
  let isScanning = false;
  
  function generateMockData() {
    const symbols = ['AAPL', 'TSLA', 'NVDA', 'MSFT', 'GOOGL', 'AMZN', 'META', 'SPY', 'QQQ', 'AMD', 
                     'NFLX', 'COIN', 'PLTR', 'SOFI', 'RIVN', 'LCID', 'NIO', 'BABA', 'JD', 'PDD'];
    
    return symbols.map(symbol => {
      const v1 = 50 + Math.random() * 50;
      const v2 = 50 + Math.random() * 50;
      const v3 = 50 + Math.random() * 50;
      const v4 = 50 + Math.random() * 50;
      const v5 = 50 + Math.random() * 50;
      const v6 = 50 + Math.random() * 50;
      
      const signals = ['bullish', 'bearish', 'neutral'];
      const psychSignals = ['healthy', 'caution', 'danger'];
      
      const ultimate = calculateUltimateScore(
        v1, v2, v3, v4, v5, v6,
        signals[Math.floor(Math.random() * signals.length)] as any,
        signals[Math.floor(Math.random() * signals.length)] as any,
        signals[Math.floor(Math.random() * signals.length)] as any,
        signals[Math.floor(Math.random() * signals.length)] as any,
        signals[Math.floor(Math.random() * signals.length)] as any,
        psychSignals[Math.floor(Math.random() * psychSignals.length)] as any
      );
      
      return {
        symbol,
        price: 100 + Math.random() * 400,
        ultimate,
        v1_smc: v1,
        v2_orderflow: v2,
        v2_sentiment: v3,
        v3_auction: v4,
        v4_quantum: v5,
        v5_psychology: v6
      };
    }).sort((a, b) => b.ultimate.score - a.ultimate.score);
  }
  
  async function runScan() {
    isScanning = true;
    await new Promise(resolve => setTimeout(resolve, 1500));
    scanResults = generateMockData();
    isScanning = false;
  }
  
  $: filteredResults = scanResults.filter(result => {
    if (result.ultimate.score < filters.minScore || result.ultimate.score > filters.maxScore) return false;
    if (filters.signal !== 'all' && result.ultimate.signal !== filters.signal) return false;
    
    const gradeOrder = ['F', 'D', 'C', 'B', 'A', 'S'];
    const minGradeIndex = gradeOrder.indexOf(filters.minGrade);
    const resultGradeIndex = gradeOrder.indexOf(result.ultimate.grade);
    if (resultGradeIndex < minGradeIndex) return false;
    
    return true;
  });
  
  onMount(() => {
    runScan();
  });
</script>

<div class="scanner">
  <header class="page-header">
    <h1>🔍 Scanner V1-V5</h1>
    <p class="subtitle">Analyse multi-dimensionnelle des marchés</p>
  </header>
  
  <div class="controls">
    <div class="filter-group">
      <label>
        Score Min:
        <input type="number" bind:value={filters.minScore} min="0" max="100" />
      </label>
      
      <label>
        Score Max:
        <input type="number" bind:value={filters.maxScore} min="0" max="100" />
      </label>
      
      <label>
        Signal:
        <select bind:value={filters.signal}>
          <option value="all">Tous</option>
          <option value="bullish">Haussier</option>
          <option value="bearish">Baissier</option>
          <option value="neutral">Neutre</option>
        </select>
      </label>
      
      <label>
        Grade Min:
        <select bind:value={filters.minGrade}>
          <option value="F">F</option>
          <option value="D">D</option>
          <option value="C">C</option>
          <option value="B">B</option>
          <option value="A">A</option>
          <option value="S">S</option>
        </select>
      </label>
      
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={filters.showDetails} />
        Afficher détails
      </label>
    </div>
    
    <button class="scan-btn" on:click={runScan} disabled={isScanning}>
      {isScanning ? '⏳ Scan en cours...' : '🔄 Lancer Scan'}
    </button>
  </div>
  
  <div class="results-count">
    {filteredResults.length} résultat{filteredResults.length > 1 ? 's' : ''} trouvé{filteredResults.length > 1 ? 's' : ''}
  </div>
  
  <div class="results">
    {#each filteredResults as result}
      <div class="result-card">
        <div class="result-header">
          <div class="symbol-section">
            <h3 class="symbol">{result.symbol}</h3>
            <div class="price">${result.price.toFixed(2)}</div>
          </div>
          
          <div class="score-section">
            <div class="ultimate-score">
              <div class="score-value" style="color: {getGradeColor(result.ultimate.grade)}">
                {result.ultimate.score.toFixed(1)}
              </div>
              <div class="grade" style="color: {getGradeColor(result.ultimate.grade)}">
                Grade {result.ultimate.grade}
              </div>
            </div>
            
            <div class="signal-badge" style="background: {getSignalColor(result.ultimate.signal)}20; color: {getSignalColor(result.ultimate.signal)}; border: 1px solid {getSignalColor(result.ultimate.signal)}">
              {result.ultimate.signal === 'bullish' ? '↗️ Haussier' : result.ultimate.signal === 'bearish' ? '↘️ Baissier' : '➡️ Neutre'}
            </div>
          </div>
          
          <div class="prob-section">
            <div class="win-prob">
              Win Prob: <strong>{result.ultimate.win_probability}%</strong>
            </div>
            <div class="confidence">
              CI: [{result.ultimate.confidence_interval[0].toFixed(0)}, {result.ultimate.confidence_interval[1].toFixed(0)}]
            </div>
          </div>
        </div>
        
        {#if filters.showDetails}
          <div class="result-details">
            <div class="detail-grid">
              <div class="detail-item">
                <span class="detail-label">V1 SMC</span>
                <div class="detail-bar">
                  <div class="bar-fill" style="width: {result.v1_smc}%; background: #ff6b35"></div>
                  <span class="bar-value">{result.v1_smc.toFixed(0)}</span>
                </div>
              </div>
              
              <div class="detail-item">
                <span class="detail-label">V2 Order Flow</span>
                <div class="detail-bar">
                  <div class="bar-fill" style="width: {result.v2_orderflow}%; background: #4ecdc4"></div>
                  <span class="bar-value">{result.v2_orderflow.toFixed(0)}</span>
                </div>
              </div>
              
              <div class="detail-item">
                <span class="detail-label">V2 Sentiment</span>
                <div class="detail-bar">
                  <div class="bar-fill" style="width: {result.v2_sentiment}%; background: #95e1d3"></div>
                  <span class="bar-value">{result.v2_sentiment.toFixed(0)}</span>
                </div>
              </div>
              
              <div class="detail-item">
                <span class="detail-label">V3 Auction</span>
                <div class="detail-bar">
                  <div class="bar-fill" style="width: {result.v3_auction}%; background: #f9ca24"></div>
                  <span class="bar-value">{result.v3_auction.toFixed(0)}</span>
                </div>
              </div>
              
              <div class="detail-item">
                <span class="detail-label">V4 Quantum</span>
                <div class="detail-bar">
                  <div class="bar-fill" style="width: {result.v4_quantum}%; background: #a29bfe"></div>
                  <span class="bar-value">{result.v4_quantum.toFixed(0)}</span>
                </div>
              </div>
              
              <div class="detail-item">
                <span class="detail-label">V5 Psychologie</span>
                <div class="detail-bar">
                  <div class="bar-fill" style="width: {result.v5_psychology}%; background: #fd79a8"></div>
                  <span class="bar-value">{result.v5_psychology.toFixed(0)}</span>
                </div>
              </div>
            </div>
            
            <div class="recommendation">
              💡 {result.ultimate.recommendation}
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .scanner {
    max-width: 1400px;
    margin: 0 auto;
  }
  
  .page-header h1 {
    font-size: 2.5rem;
    margin: 0 0 0.5rem 0;
    color: #ff6b35;
  }
  
  .subtitle {
    color: #95a5a6;
    margin: 0 0 2rem 0;
  }
  
  .controls {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }
  
  .filter-group {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }
  
  .filter-group label {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-size: 0.875rem;
    color: #95a5a6;
  }
  
  .filter-group input,
  .filter-group select {
    padding: 0.5rem;
    background: rgba(15, 20, 25, 0.8);
    border: 1px solid rgba(255, 107, 53, 0.3);
    border-radius: 6px;
    color: #d1d4dc;
    font-size: 0.875rem;
  }
  
  .checkbox-label {
    flex-direction: row !important;
    align-items: center;
  }
  
  .scan-btn {
    padding: 0.75rem 1.5rem;
    background: #ff6b35;
    border: none;
    border-radius: 8px;
    color: white;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }
  
  .scan-btn:hover:not(:disabled) {
    background: #ff8555;
    box-shadow: 0 4px 12px rgba(255, 107, 53, 0.4);
  }
  
  .scan-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .results-count {
    color: #95a5a6;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }
  
  .results {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .result-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
    transition: all 0.2s;
  }
  
  .result-card:hover {
    border-color: #ff6b35;
    box-shadow: 0 8px 24px rgba(255, 107, 53, 0.15);
  }
  
  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 2rem;
    margin-bottom: 1rem;
    flex-wrap: wrap;
  }
  
  .symbol {
    font-size: 1.5rem;
    margin: 0;
    color: #ff6b35;
  }
  
  .price {
    color: #95a5a6;
    font-size: 0.875rem;
  }
  
  .score-section {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .ultimate-score {
    text-align: center;
  }
  
  .score-value {
    font-size: 2rem;
    font-weight: 700;
  }
  
  .grade {
    font-size: 0.875rem;
    font-weight: 600;
  }
  
  .signal-badge {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-weight: 600;
    font-size: 0.875rem;
  }
  
  .prob-section {
    text-align: right;
  }
  
  .win-prob {
    font-size: 1.1rem;
    margin-bottom: 0.25rem;
  }
  
  .confidence {
    font-size: 0.75rem;
    color: #95a5a6;
  }
  
  .result-details {
    border-top: 1px solid rgba(255, 107, 53, 0.1);
    padding-top: 1rem;
  }
  
  .detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
    margin-bottom: 1rem;
  }
  
  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .detail-label {
    font-size: 0.875rem;
    color: #95a5a6;
    font-weight: 600;
  }
  
  .detail-bar {
    position: relative;
    height: 24px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 4px;
    overflow: hidden;
  }
  
  .bar-fill {
    height: 100%;
    transition: width 0.3s;
    border-radius: 4px;
  }
  
  .bar-value {
    position: absolute;
    right: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    font-size: 0.75rem;
    font-weight: 600;
    color: white;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
  }
  
  .recommendation {
    background: rgba(255, 107, 53, 0.1);
    border-left: 3px solid #ff6b35;
    padding: 0.75rem 1rem;
    border-radius: 4px;
    font-size: 0.875rem;
    line-height: 1.5;
  }
</style>
