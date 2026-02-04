# BaconAlgo Stream Components

Production-ready stream overlay components for BaconAlgo trading platform.

## 📦 Store Modules (`/station/src/lib/stores/`)

### signals.ts
Trading signals store with real-time Supabase integration.

**Stores:**
- `signals` - All trading signals (max 100)
- `hotSignals` - Derived: Top 5 S/A grade signals
- `buySignals` - Derived: All BUY signals
- `sellSignals` - Derived: All SELL signals

**Functions:**
- `initSignalsStore()` - Initialize with data and real-time subscription
- `addSignal(signal)` - Add new signal to store
- `clearOldSignals(hours)` - Remove signals older than X hours
- `cleanupSignalsStore()` - Unsubscribe from real-time updates

### donations.ts
Donations management with alerts and goal tracking.

**Stores:**
- `donations` - All donations (max 100)
- `recentDonations` - Derived: Last 10 donations
- `topDonors` - Top donors list
- `todaysTotal` - Today's total amount
- `goalProgress` - Progress toward daily goal (0-1)
- `donationAlert` - Currently shown alert

**Functions:**
- `initDonationsStore()` - Initialize with data and subscriptions
- `addDonation(donation)` - Add and potentially show alert
- `showDonationAlert(donation)` - Trigger alert display
- `clearDonationAlert()` - Dismiss current alert
- `cleanupDonationsStore()` - Cleanup subscriptions

### market.ts
Market data and indicators.

**Stores:**
- `fearGreedIndex` - Fear & Greed Index (0-100)
- `vixValue` - Current VIX value
- `topMovers` - Top market movers array
- `marketStatus` - Market open/close status

**Functions:**
- `initMarketStore()` - Initialize with mock data
- `updateFearGreed(value)` - Update Fear/Greed
- `updateVix(value)` - Update VIX
- `updateTopMovers(movers)` - Update movers list
- `isMarketOpen()` - Check if market is currently open
- `getNextMarketOpen()` - Get next market open time

### settings.ts
User preferences with localStorage persistence.

**Store:**
- `settings` - All user settings

**Settings:**
- `theme` - 'dark' | 'light' | 'bacon'
- `volume` - Sound volume (0-1)
- `showChat` - Show chat overlay
- `showDonations` - Show donation widgets
- `showSignals` - Show signal widgets
- `showMarketData` - Show market widgets
- `streamLayout` - 'default' | 'minimal' | 'full'
- `enableSounds` - Enable sound effects
- `enableNotifications` - Enable notifications
- `autoHideAlerts` - Auto-hide alerts

**Functions:**
- `updateSetting(key, value)` - Update specific setting
- `resetSettings()` - Reset to defaults
- `toggleSetting(key)` - Toggle boolean setting
- `setVolume(volume)` - Set volume (0-1)
- `setTheme(theme)` - Set theme
- `setLayout(layout)` - Set layout

## 🎨 Stream Components (`/station/src/lib/stream/`)

### StreamDashboard.svelte
Main 1920x1080 stream overlay layout.

**Features:**
- Glass morphism design
- Automatic store initialization
- Responsive widget positioning
- Error handling

**Layout:**
- Top bar: Market countdown + Fear/Greed Index
- Left panel: Hot signals display
- Right panel: Donation widgets
- Bottom bar: Social links
- Floating: Donation alerts

### widgets/MarketCountdown.svelte
Countdown to 9:30 AM market open.

**Features:**
- Real-time clock
- Market status indicator (open/closed)
- Countdown timer with hours/minutes/seconds
- Pulsing "Market Open" indicator
- Auto-updates every second

### widgets/FearGreedIndex.svelte
Fear & Greed Index gauge (0-100).

**Features:**
- Circular gauge with gradient
- Color-coded sentiment (red to green)
- Animated needle
- Text labels: Extreme Fear, Fear, Neutral, Greed, Extreme Greed
- Smooth transitions

### widgets/RocketSignals.svelte
Hot signals display (S/A grades only).

**Features:**
- Shows top 5 hot signals
- Grade badges with colors (S=gold, A=orange)
- BUY/SELL indicators
- Signal scores
- Timestamps
- Auto-scroll
- Empty state with loading message
- Smooth entry/exit animations

### donations/DonationAlert.svelte
Epic donation popup with animations.

**Features:**
- Full-screen overlay
- Confetti effects (10+ for Sizzling, 50 for Legendary)
- Tier-based styling and colors
- Donor name, amount, and message
- Auto-dismiss after ALERT_DURATION seconds
- Legendary donations get special pulse animation
- Smooth fly-in/out transitions

### donations/DonationTiers.svelte
$2/$5/$10/$20 tier cards.

**Features:**
- 4 tier cards in 2x2 grid
- Tier icons: 🥓 ✨ 🔥 🏆
- Color-coded borders
- Hover animations (scale + lift)
- Glass card styling

**Tiers:**
- Bacon Bit ($2) - Orange
- Crispy ($5) - Orange/Yellow
- Sizzling ($10) - Red/Orange
- Legendary ($20) - Gold

### donations/DonationGoalTracker.svelte
Daily goal progress tracker.

**Features:**
- Animated progress bar
- Milestone markers (25%, 50%, 75%, 100%)
- Current/goal amounts display
- Shimmer effect on progress bar
- Percentage display
- Color changes when milestones reached

### social/SocialLinks.svelte
YouTube + Discord buttons.

**Features:**
- Icon + text buttons
- Glass button styling
- Color overlays on hover
- Opens links in new tab
- Responsive layout

## 🎯 Usage Example

```svelte
<script lang="ts">
  import { StreamDashboard } from '$lib/stream';
</script>

<StreamDashboard />
```

Individual components:

```svelte
<script lang="ts">
  import { 
    MarketCountdown, 
    FearGreedIndex, 
    RocketSignals,
    DonationAlert,
    DonationTiers,
    DonationGoalTracker,
    SocialLinks 
  } from '$lib/stream';
</script>

<MarketCountdown />
<FearGreedIndex />
<RocketSignals />
<DonationAlert />
<DonationTiers />
<DonationGoalTracker />
<SocialLinks />
```

## 🎨 Styling

All components use:
- Glass morphism design (`.glass-card`, `.glass-button`)
- CSS variables for brand colors
- Smooth transitions and animations
- Responsive layouts
- Drop shadows and glows

**CSS Variables:**
```css
--brand-primary: #ff6b35
--brand-accent: #f7931e
--grade-s: #ffd700
--grade-a: #ff6b35
--grade-b: #4a90e2
--grade-c: #95a5a6
--buy-color: #2ecc71
--sell-color: #e74c3c
```

## 🔌 Integration

All components automatically:
- Connect to Supabase real-time subscriptions
- Sync with localStorage (settings)
- Handle errors gracefully
- Clean up on unmount
- Update reactively

## 📱 Browser Compatibility

- Chrome/Edge: ✅ Full support
- Firefox: ✅ Full support
- Safari: ✅ Full support
- Modern browsers with ES2020+ support

## 🚀 Performance

- Efficient Svelte reactivity
- Auto-cleanup of old data
- Optimized real-time subscriptions
- Minimal re-renders
- CSS animations (GPU accelerated)

## 🔒 Security

- ✅ No CodeQL security alerts
- ✅ TypeScript strict mode
- ✅ Input validation
- ✅ Secure Supabase client
- ✅ No XSS vulnerabilities
