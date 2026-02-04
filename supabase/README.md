# BaconAlgo Supabase Integration

Complete Supabase database integration for BaconAlgo streaming platform.

## Files

- **`station/src/lib/supabase/client.ts`** - TypeScript client with API functions
- **`supabase/migrations/001_initial_schema.sql`** - Database schema migration

## Setup

### 1. Install Dependencies

```bash
cd station
npm install @supabase/supabase-js
```

### 2. Configure Environment Variables

Add to your `.env` file:

```env
PUBLIC_SUPABASE_URL=https://your-project.supabase.co
PUBLIC_SUPABASE_ANON_KEY=your-anon-key
```

### 3. Run Migration

Using Supabase CLI:

```bash
supabase db push
```

Or manually run the SQL file in your Supabase dashboard.

## Usage

### Initialize Client

```typescript
import { initSupabase } from '$lib/supabase/client';

// Initialize on app startup
initSupabase();
```

### Donations API

```typescript
import { 
  submitDonation, 
  getDonations, 
  getTodaysDonations,
  getTopDonors,
  getDonationGoalProgress,
  subscribeToDonations
} from '$lib/supabase/client';

// Submit a donation
const donation = await submitDonation({
  donor_name: 'John Doe',
  amount: 10.00,
  message: 'Great stream!',
  payment_method: 'paypal',
  transaction_id: 'txn_123'
});

// Get today's donations
const todayDonations = await getTodaysDonations();

// Get top donors
const topDonors = await getTopDonors(10);

// Get goal progress
const progress = await getDonationGoalProgress(100);
console.log(`Progress: ${progress.progress_percentage}%`);

// Subscribe to real-time donations
const channel = subscribeToDonations((donation) => {
  console.log('New donation:', donation);
});
```

### Signals API

```typescript
import { 
  getSignals, 
  subscribeToSignals 
} from '$lib/supabase/client';

// Get recent signals
const signals = await getSignals({ limit: 20 });

// Get only BUY signals with grade S
const buySignals = await getSignals({ 
  direction: 'BUY', 
  grade: 'S',
  limit: 10 
});

// Subscribe to real-time signals
const channel = subscribeToSignals(
  (signal) => {
    console.log('New signal:', signal);
  },
  { grade: 'S' } // Optional filter
);
```

### Sponsors API

```typescript
import { getSponsors } from '$lib/supabase/client';

// Get active sponsors
const sponsors = await getSponsors();

// Get all sponsors (including inactive)
const allSponsors = await getSponsors(false);
```

### Song Requests API

```typescript
import { 
  getSongRequests, 
  submitSongRequest 
} from '$lib/supabase/client';

// Submit song request
const request = await submitSongRequest({
  requester_name: 'Jane Doe',
  song_title: 'Bacon Sizzle',
  artist: 'The Crispers'
});

// Get pending requests
const pending = await getSongRequests({ status: 'pending' });
```

### Cleanup

```typescript
import { unsubscribe, unsubscribeAll } from '$lib/supabase/client';

// Unsubscribe from specific channel
await unsubscribe(channel);

// Unsubscribe from all channels
await unsubscribeAll();
```

## Database Schema

### Tables

- **`signals`** - Trading signals (BUY/SELL with grades S/A/B/C)
- **`donations`** - Donation transactions with tiers
- **`donors`** - Aggregated donor statistics
- **`sponsors`** - Stream sponsors and partners
- **`positions`** - Trading positions from signals
- **`daily_stats`** - Daily aggregated metrics
- **`song_requests`** - Viewer song requests
- **`stream_config`** - Configuration key-value store

### Functions

- **`get_top_donors(limit)`** - Get top donors by amount
- **`get_todays_donations()`** - Get all donations today
- **`get_donation_goal_progress(goal)`** - Calculate goal progress

### Triggers

- Auto-update donor statistics on completed donations
- Auto-update daily stats for donations and signals

### Real-time

Enabled on:
- `donations` - Get instant donation alerts
- `signals` - Get instant trading signals
- `song_requests` - Get instant song requests

## TypeScript Types

All types are exported from `client.ts`:

```typescript
import type { 
  Signal, 
  Donation, 
  Donor, 
  Sponsor,
  SongRequest,
  SignalDirection,
  SignalGrade,
  DonationStatus,
  DonationTier
} from '$lib/supabase/client';
```

## Security

- Row Level Security (RLS) enabled on all tables
- Public read access for most data
- Public insert for donations and song requests
- Admin access required for updates/deletes (configure in Supabase dashboard)

## Performance

- Indexes on all frequently queried columns
- Aggregated tables (`donors`, `daily_stats`) for fast queries
- Triggers for automatic statistics updates
