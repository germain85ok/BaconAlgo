<script lang="ts">
  let quantumData = {
    probability: { upProb: 65.5, downProb: 34.5, confidence: [60, 72] },
    barriers: { support: 445.20, resistance: 458.30 },
    delta: 0.68,
    gamma: 0.042
  };
</script>

<div class="quantum">
  <h1>⚛️ Quantum Analysis</h1>
  <p class="subtitle">Analyse probabiliste et Greeks options</p>
  
  <div class="metrics-grid">
    <div class="metric-card">
      <h3>🎲 Probabilité</h3>
      <div class="prob-bars">
        <div class="prob-item">
          <span class="prob-label">Hausse</span>
          <div class="prob-bar">
            <div class="bar-fill" style="width: {quantumData.probability.upProb}%; background: #26a69a"></div>
          </div>
          <span class="prob-value">{quantumData.probability.upProb}%</span>
        </div>
        <div class="prob-item">
          <span class="prob-label">Baisse</span>
          <div class="prob-bar">
            <div class="bar-fill" style="width: {quantumData.probability.downProb}%; background: #ef5350"></div>
          </div>
          <span class="prob-value">{quantumData.probability.downProb}%</span>
        </div>
      </div>
      <div class="confidence">
        IC 95%: [{quantumData.probability.confidence[0]}, {quantumData.probability.confidence[1]}]
      </div>
    </div>
    
    <div class="metric-card">
      <h3>🚧 Barrières</h3>
      <div class="barrier-item">
        <span class="barrier-label">Support</span>
        <span class="barrier-value">${quantumData.barriers.support.toFixed(2)}</span>
      </div>
      <div class="barrier-item">
        <span class="barrier-label">Résistance</span>
        <span class="barrier-value">${quantumData.barriers.resistance.toFixed(2)}</span>
      </div>
    </div>
    
    <div class="metric-card">
      <h3>📐 Greeks</h3>
      <div class="greek-item">
        <span class="greek-label">Delta</span>
        <span class="greek-value">{quantumData.delta}</span>
      </div>
      <div class="greek-item">
        <span class="greek-label">Gamma</span>
        <span class="greek-value">{quantumData.gamma}</span>
      </div>
    </div>
  </div>
</div>

<style>
  .quantum {
    max-width: 1400px;
    margin: 0 auto;
  }
  
  h1 {
    font-size: 2.5rem;
    margin: 0 0 0.5rem 0;
    color: #ff6b35;
  }
  
  .subtitle {
    color: #95a5a6;
    margin: 0 0 2rem 0;
  }
  
  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
  }
  
  .metric-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 2rem;
  }
  
  .metric-card h3 {
    margin: 0 0 1.5rem 0;
    color: #ff6b35;
  }
  
  .prob-bars {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 1rem;
  }
  
  .prob-item {
    display: grid;
    grid-template-columns: 80px 1fr 60px;
    align-items: center;
    gap: 0.75rem;
  }
  
  .prob-label {
    font-size: 0.875rem;
    color: #95a5a6;
  }
  
  .prob-bar {
    height: 20px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 4px;
    overflow: hidden;
  }
  
  .bar-fill {
    height: 100%;
  }
  
  .prob-value {
    font-weight: 600;
    text-align: right;
  }
  
  .confidence {
    color: #95a5a6;
    font-size: 0.875rem;
    text-align: center;
  }
  
  .barrier-item,
  .greek-item {
    display: flex;
    justify-content: space-between;
    padding: 0.75rem 0;
    border-bottom: 1px solid rgba(255, 107, 53, 0.1);
  }
  
  .barrier-item:last-child,
  .greek-item:last-child {
    border-bottom: none;
  }
  
  .barrier-label,
  .greek-label {
    color: #95a5a6;
  }
  
  .barrier-value,
  .greek-value {
    font-weight: 600;
    color: #ff6b35;
  }
</style>
