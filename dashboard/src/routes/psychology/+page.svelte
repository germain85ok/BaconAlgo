<script lang="ts">
  import { BiasDetector } from '$lib/psychology/biasDetector';
  import { EmotionalStateTracker } from '$lib/psychology/emotionalState';
  
  let stress = 3;
  let fomo = 2;
  let fear = 2;
  let confidence = 7;
  let fatigue = 3;
  
  let emotionalState: any = null;
  let biases: any[] = [];
  
  function checkEmotionalState() {
    const tracker = new EmotionalStateTracker();
    emotionalState = tracker.assess(stress, fomo, fear, confidence, fatigue);
  }
  
  $: {
    checkEmotionalState();
  }
</script>

<style>
  .psychology {
    background: #0b0e11;
    color: #d1d4dc;
    min-height: 100vh;
    padding: 24px;
  }

  .title {
    font-size: 32px;
    font-weight: 700;
    margin: 0 0 24px 0;
  }

  .card {
    background: #141b22;
    border: 1px solid #1f2933;
    border-radius: 12px;
    padding: 24px;
    margin-bottom: 20px;
  }

  .card-title {
    font-size: 20px;
    font-weight: 600;
    margin: 0 0 20px 0;
  }

  .slider-group {
    margin: 20px 0;
  }

  .slider-label {
    display: flex;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .slider {
    width: 100%;
    height: 8px;
    border-radius: 4px;
    outline: none;
    -webkit-appearance: none;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #667eea;
    cursor: pointer;
  }

  .state-box {
    padding: 20px;
    border-radius: 8px;
    text-align: center;
    margin-top: 20px;
  }

  .state-optimal {
    background: rgba(38, 166, 154, 0.2);
    border: 2px solid #26a69a;
  }

  .state-good {
    background: rgba(0, 188, 212, 0.2);
    border: 2px solid #00BCD4;
  }

  .state-caution {
    background: rgba(255, 167, 38, 0.2);
    border: 2px solid #FFA726;
  }

  .state-stop {
    background: rgba(239, 83, 80, 0.2);
    border: 2px solid #ef5350;
  }
</style>

<div class="psychology">
  <h1 class="title">🧠 Trading Psychology</h1>

  <div class="card">
    <h3 class="card-title">😌 Emotional State Check-In</h3>
    
    <div class="slider-group">
      <div class="slider-label">
        <span>Stress Level</span>
        <span style="font-weight: 700;">{stress}/10</span>
      </div>
      <input type="range" min="0" max="10" bind:value={stress} class="slider" style="background: linear-gradient(to right, #26a69a 0%, #ef5350 100%);" />
    </div>
    
    <div class="slider-group">
      <div class="slider-label">
        <span>FOMO (Fear of Missing Out)</span>
        <span style="font-weight: 700;">{fomo}/10</span>
      </div>
      <input type="range" min="0" max="10" bind:value={fomo} class="slider" style="background: linear-gradient(to right, #26a69a 0%, #ef5350 100%);" />
    </div>
    
    <div class="slider-group">
      <div class="slider-label">
        <span>Fear Level</span>
        <span style="font-weight: 700;">{fear}/10</span>
      </div>
      <input type="range" min="0" max="10" bind:value={fear} class="slider" style="background: linear-gradient(to right, #26a69a 0%, #ef5350 100%);" />
    </div>
    
    <div class="slider-group">
      <div class="slider-label">
        <span>Confidence</span>
        <span style="font-weight: 700;">{confidence}/10</span>
      </div>
      <input type="range" min="0" max="10" bind:value={confidence} class="slider" style="background: linear-gradient(to right, #ef5350 0%, #26a69a 100%);" />
    </div>
    
    <div class="slider-group">
      <div class="slider-label">
        <span>Fatigue</span>
        <span style="font-weight: 700;">{fatigue}/10</span>
      </div>
      <input type="range" min="0" max="10" bind:value={fatigue} class="slider" style="background: linear-gradient(to right, #26a69a 0%, #ef5350 100%);" />
    </div>
    
    {#if emotionalState}
      <div class="state-box state-{emotionalState.overall === 'optimal' ? 'optimal' : emotionalState.overall === 'good' ? 'good' : emotionalState.overall === 'caution' ? 'caution' : 'stop'}">
        <div style="font-size: 16px; font-weight: 600; margin-bottom: 8px;">
          Overall State
        </div>
        <div style="font-size: 28px; font-weight: 700; text-transform: uppercase;">
          {emotionalState.overall.replace('_', ' ')}
        </div>
        <div style="margin-top: 12px; opacity: 0.9;">
          {#if emotionalState.overall === 'optimal'}
            ✅ Perfect state for trading!
          {:else if emotionalState.overall === 'good'}
            👍 Good to trade, stay focused
          {:else if emotionalState.overall === 'caution'}
            ⚠️ Proceed with caution
          {:else}
            🛑 Consider taking a break
          {/if}
        </div>
      </div>
    {/if}
  </div>

  <div class="card">
    <h3 class="card-title">📝 Trading Journal</h3>
    <div style="padding: 20px; text-align: center; opacity: 0.6;">
      Journal entries will appear here...
    </div>
  </div>

  <div class="card">
    <h3 class="card-title">🎯 Daily Rituals</h3>
    <div style="padding: 12px;">
      <div style="margin: 12px 0; display: flex; align-items: center; gap: 12px;">
        <input type="checkbox" id="ritual1" style="width: 20px; height: 20px;">
        <label for="ritual1">Morning meditation (10 min)</label>
      </div>
      <div style="margin: 12px 0; display: flex; align-items: center; gap: 12px;">
        <input type="checkbox" id="ritual2" style="width: 20px; height: 20px;">
        <label for="ritual2">Review yesterday's trades</label>
      </div>
      <div style="margin: 12px 0; display: flex; align-items: center; gap: 12px;">
        <input type="checkbox" id="ritual3" style="width: 20px; height: 20px;">
        <label for="ritual3">Set daily goals</label>
      </div>
      <div style="margin: 12px 0; display: flex; align-items: center; gap: 12px;">
        <input type="checkbox" id="ritual4" style="width: 20px; height: 20px;">
        <label for="ritual4">Post-market review</label>
      </div>
    </div>
  </div>
</div>
