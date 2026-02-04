<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    score: number;
    size?: 'sm' | 'md' | 'lg';
  }

  let { score = 0, size = 'md' }: Props = $props();

  let animatedScore = $state(0);
  let mounted = $state(false);

  const sizes = {
    sm: { width: 60, stroke: 4, fontSize: 14 },
    md: { width: 80, stroke: 5, fontSize: 18 },
    lg: { width: 120, stroke: 6, fontSize: 24 }
  };

  const config = $derived(sizes[size]);
  const radius = $derived(config.width / 2 - config.stroke);
  const circumference = $derived(2 * Math.PI * radius);
  const offset = $derived(circumference - (animatedScore / 100) * circumference);

  const getColor = (score: number): string => {
    if (score <= 25) return '#9ca3af';
    if (score <= 50) return '#fbbf24';
    if (score <= 75) return '#f97316';
    return 'url(#baconGradient)';
  };

  onMount(() => {
    mounted = true;
    const duration = 1000;
    const start = Date.now();
    
    const animate = () => {
      const now = Date.now();
      const progress = Math.min((now - start) / duration, 1);
      const easeOutCubic = 1 - Math.pow(1 - progress, 3);
      animatedScore = easeOutCubic * score;
      
      if (progress < 1) {
        requestAnimationFrame(animate);
      }
    };
    
    requestAnimationFrame(animate);
  });
</script>

<div class="score-gauge" class:mounted style="--size: {config.width}px">
  <svg width={config.width} height={config.width} class="gauge-svg">
    <defs>
      <linearGradient id="baconGradient" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" style="stop-color:#ff6b35;stop-opacity:1" />
        <stop offset="100%" style="stop-color:#fbbf24;stop-opacity:1" />
      </linearGradient>
    </defs>
    
    <!-- Background circle -->
    <circle
      cx={config.width / 2}
      cy={config.width / 2}
      r={radius}
      stroke="#1f2937"
      stroke-width={config.stroke}
      fill="none"
    />
    
    <!-- Progress circle -->
    <circle
      class="progress-circle"
      cx={config.width / 2}
      cy={config.width / 2}
      r={radius}
      stroke={getColor(score)}
      stroke-width={config.stroke}
      fill="none"
      stroke-dasharray={circumference}
      stroke-dashoffset={offset}
      stroke-linecap="round"
      transform="rotate(-90 {config.width / 2} {config.width / 2})"
    />
    
    <!-- Score text -->
    <text
      x="50%"
      y="50%"
      text-anchor="middle"
      dominant-baseline="middle"
      class="score-text"
      style="font-size: {config.fontSize}px"
    >
      {Math.round(animatedScore)}
    </text>
  </svg>
</div>

<style>
  .score-gauge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .gauge-svg {
    filter: drop-shadow(0 0 8px rgba(255, 107, 53, 0.3));
  }

  .progress-circle {
    transition: stroke-dashoffset 0.1s ease-out;
  }

  .score-text {
    fill: #f9fafb;
    font-weight: 700;
    font-family: 'Inter', system-ui, -apple-system, sans-serif;
  }

  .score-gauge.mounted .gauge-svg {
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% {
      filter: drop-shadow(0 0 8px rgba(255, 107, 53, 0.3));
    }
    50% {
      filter: drop-shadow(0 0 12px rgba(255, 107, 53, 0.5));
    }
  }
</style>
