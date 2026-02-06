<script lang="ts">
  let psychologyData = {
    biases: [
      { name: 'Confirmation', severity: 0.65, detected: true },
      { name: 'Aversion à la perte', severity: 0.45, detected: true }
    ],
    emotional: {
      current: 'neutral',
      fearGreedIndex: 55,
      intensity: 0.4
    }
  };
</script>

<div class="psychology">
  <h1>🧠 Analyse Psychologique</h1>
  <p class="subtitle">Détection des biais cognitifs et état émotionnel</p>
  
  <div class="sections">
    <div class="section">
      <h2>⚠️ Biais Cognitifs Détectés</h2>
      <div class="biases-list">
        {#each psychologyData.biases as bias}
          <div class="bias-card">
            <div class="bias-header">
              <span class="bias-name">{bias.name}</span>
              <span class="bias-severity" style="color: {bias.severity > 0.6 ? '#ef5350' : '#f9ca24'}">
                Sévérité: {(bias.severity * 100).toFixed(0)}%
              </span>
            </div>
            <div class="severity-bar">
              <div class="bar-fill" style="width: {bias.severity * 100}%; background: {bias.severity > 0.6 ? '#ef5350' : '#f9ca24'}"></div>
            </div>
          </div>
        {/each}
      </div>
    </div>
    
    <div class="section">
      <h2>😊 État Émotionnel</h2>
      <div class="emotional-card">
        <div class="current-emotion">
          <span class="label">État Actuel:</span>
          <span class="value">{psychologyData.emotional.current === 'neutral' ? '😐 Neutre' : psychologyData.emotional.current}</span>
        </div>
        
        <div class="fear-greed">
          <span class="label">Indice Peur/Avidité</span>
          <div class="fg-bar">
            <div class="fg-fill" style="left: {psychologyData.emotional.fearGreedIndex}%"></div>
          </div>
          <div class="fg-labels">
            <span>Peur</span>
            <span>{psychologyData.emotional.fearGreedIndex}</span>
            <span>Avidité</span>
          </div>
        </div>
        
        <div class="intensity">
          <span class="label">Intensité:</span>
          <span class="value">{(psychologyData.emotional.intensity * 100).toFixed(0)}%</span>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .psychology {
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
  
  .sections {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  .section h2 {
    color: #ff6b35;
    margin-bottom: 1.5rem;
  }
  
  .biases-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .bias-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 1.5rem;
  }
  
  .bias-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }
  
  .bias-name {
    font-weight: 600;
  }
  
  .severity-bar {
    height: 20px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 4px;
    overflow: hidden;
  }
  
  .bar-fill {
    height: 100%;
  }
  
  .emotional-card {
    background: rgba(15, 20, 25, 0.6);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 107, 53, 0.2);
    border-radius: 12px;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .current-emotion,
  .intensity {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .label {
    color: #95a5a6;
    font-size: 0.875rem;
  }
  
  .value {
    font-weight: 600;
    font-size: 1.25rem;
  }
  
  .fear-greed {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .fg-bar {
    position: relative;
    height: 30px;
    background: linear-gradient(to right, #ef5350, #f9ca24, #26a69a);
    border-radius: 8px;
  }
  
  .fg-fill {
    position: absolute;
    top: -4px;
    width: 4px;
    height: 38px;
    background: white;
    border: 2px solid #ff6b35;
    border-radius: 2px;
    transform: translateX(-50%);
  }
  
  .fg-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    color: #95a5a6;
  }
</style>
