# BaconAlgo Dashboard Components

Trading signal and badge components for the BaconAlgo SAAS platform.

## Components

### 1. SignalCard.svelte

Trading signal display card with comprehensive signal information.

**Props:**
```typescript
interface Signal {
  ticker: string;
  direction: 'LONG' | 'SHORT';
  score: number;
  grade: 'S' | 'A' | 'B' | 'C';
  entry: number;
  stop_loss: number;
  take_profit_1: number;
  take_profit_2: number;
  take_profit_3: number;
  risk_reward_ratio: number;
  smc_tags: string[];
  horizon: 'Scalp' | 'Day' | 'Swing' | 'Long';
  timestamp?: string;
}

interface Props {
  signal: Signal;
  onAddToWatchlist?: (signal: Signal) => void;
  onSetAlert?: (signal: Signal) => void;
}
```

**Usage:**
```svelte
<script>
  import { SignalCard } from '$lib/components/dashboard';

  const signal = {
    ticker: 'AAPL',
    direction: 'LONG',  // Use 'LONG' or 'SHORT' (displayed as BUY/SELL)
    score: 85,
    grade: 'A',  // Use 'S', 'A', 'B', or 'C' (auto-maps to Legendary/Epic/Rare/Common)
    entry: 175.50,
    stop_loss: 172.00,
    take_profit_1: 178.00,
    take_profit_2: 180.50,
    take_profit_3: 183.00,
    risk_reward_ratio: 2.5,
    smc_tags: ['FVG', 'OB', 'BOS'],
    horizon: 'Day',
    timestamp: new Date().toISOString()
  };
</script>

<SignalCard 
  {signal}
  onAddToWatchlist={(s) => console.log('Watchlist:', s)}
  onSetAlert={(s) => console.log('Alert:', s)}
/>
```

**Features:**
- Glass morphism card with hover effects
- Direction badge: 'LONG' displays as BUY (green), 'SHORT' displays as SELL (red)
- Integrated score gauge and grade badge (letter grades auto-map to word grades)
- Price levels with color coding (Entry=blue, SL=red, TP=green)
- Risk:Reward ratio display
- SMC tags as pills
- Horizon indicator
- Timestamp with relative time
- Action buttons (Watchlist, Alert)

---

### 2. ScoreGauge.svelte

Circular score visualization with animated progress.

**Props:**
```typescript
interface Props {
  score: number;        // 0-100
  size?: 'sm' | 'md' | 'lg';  // Default: 'md'
}
```

**Usage:**
```svelte
<script>
  import { ScoreGauge } from '$lib/components/dashboard';
</script>

<ScoreGauge score={85} size="md" />
```

**Features:**
- SVG circular progress bar
- Color gradient based on score:
  - 0-25: gray
  - 26-50: yellow
  - 51-75: orange
  - 76-100: bacon-orange to gold gradient
- Smooth animation on mount
- Three sizes (sm: 60px, md: 80px, lg: 120px)
- Pulsing glow effect

---

### 3. GradeBadge.svelte

Signal grade badge with shimmer effects.

**Props:**
```typescript
interface Props {
  grade: 'S' | 'A' | 'B' | 'C';
}
```

**Grade Mapping:**
- S → Legendary (👑 gold gradient)
- A → Epic (💎 purple-pink gradient)
- B → Rare (💠 blue-cyan gradient)
- C → Common (⚪ gray gradient)

**Usage:**
```svelte
<script>
  import { GradeBadge } from '$lib/components/dashboard';
</script>

<GradeBadge grade="S" />  <!-- Legendary -->
<GradeBadge grade="A" />  <!-- Epic -->
<GradeBadge grade="B" />  <!-- Rare -->
<GradeBadge grade="C" />  <!-- Common -->
```

**Features:**
- Grade mapping:
  - S → Legendary (👑 gold gradient with glow)
  - A → Epic (💎 purple-pink gradient with glow)
  - B → Rare (💠 blue-cyan gradient with glow)
  - C → Common (⚪ gray gradient)
- Shimmer animation for S (Legendary) and A (Epic)
- Hover scale effect

---

### 4. FilterSidebar.svelte

Comprehensive signal filter panel.

**Props:**
```typescript
interface Filters {
  horizon: 'all' | 'Scalp' | 'Day' | 'Swing' | 'Long';
  direction: 'all' | 'LONG' | 'SHORT';
  minScore: number;
  minRR: number;
  assetTypes: {
    stocks: boolean;
    crypto: boolean;
    forex: boolean;
    futures: boolean;
  };
  smcFilters: {
    nearFVG: boolean;
    nearOB: boolean;
    bosConfirmed: boolean;
    choch: boolean;
    liquidity: boolean;
    imbalance: boolean;
  };
}

interface Props {
  filters: Filters;  // Use with $bindable()
  onFilterChange?: (filters: Filters) => void;
}
```

**Usage:**
```svelte
<script>
  import { FilterSidebar } from '$lib/components/dashboard';

  let filters = $state({
    horizon: 'all',
    direction: 'all',
    minScore: 0,
    minRR: 0,
    assetTypes: {
      stocks: true,
      crypto: true,
      forex: true,
      futures: true
    },
    smcFilters: {
      nearFVG: false,
      nearOB: false,
      bosConfirmed: false,
      choch: false,
      liquidity: false,
      imbalance: false
    }
  });

  const handleFilterChange = (newFilters) => {
    console.log('Filters updated:', newFilters);
    // Apply filters to signal list
  };
</script>

<FilterSidebar bind:filters onFilterChange={handleFilterChange} />
```

**Features:**
- Sticky sidebar (desktop)
- Glass morphism card
- Filter sections:
  - Horizon: Radio buttons
  - Direction: Radio buttons
  - Min Score: Slider (0-100)
  - Min R:R: Slider (0-10)
  - Asset Type: Checkboxes
  - SMC Filters: Checkboxes
- Reset filters button
- Apply filters button (bacon-orange gradient)
- Custom scrollbar styling
- Fully responsive

---

## Styling

All components use the **bacon color scheme**:
- Primary: `#ff6b35` (Bacon Orange)
- Secondary: `#fbbf24` (Gold)
- Background: Dark glass morphism
- Gradients: Orange to gold

## Responsiveness

All components are mobile-responsive:
- **SignalCard**: Grid adjusts to single column on mobile
- **ScoreGauge**: Proportional sizing
- **GradeBadge**: Smaller text on mobile
- **FilterSidebar**: Becomes static on tablets/mobile

## Animations

- Smooth transitions on all hover states
- Score gauge: 1s ease-out cubic animation
- Grade badge: Shimmer effect for Legendary/Epic
- All buttons: Scale and shadow effects

## Browser Support

- Modern browsers with CSS Grid support
- SVG support required for ScoreGauge
- Backdrop-filter support for glass morphism

## Example Integration

See `Demo.svelte` for complete usage examples.

```svelte
<script>
  import { 
    SignalCard, 
    ScoreGauge, 
    GradeBadge, 
    FilterSidebar 
  } from '$lib/components/dashboard';
</script>
```
