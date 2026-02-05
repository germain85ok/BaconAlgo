# BaconAlgo Admin Panel

Complete admin panel for BaconAlgo platform management.

## Access

**Route**: `/admin`

**Requirements**: 
- User must be authenticated
- User profile must have `is_admin = true`
- Non-admin users are automatically redirected to `/dashboard`

## Features

### 1. Dashboard Overview (`/admin`)
Real-time platform statistics and quick actions.

**Stats Cards**:
- Total Users
- Active Subscriptions
- Monthly Recurring Revenue (MRR)
- Churn Rate
- Today's Signups
- Revenue Today

**Charts**:
- Signups Over Time (7 days, line chart)
- Revenue Over Time (30 days, bar chart)

**Quick Actions**:
- Create Promo Code
- Send Discord Alert
- Push Manual Signal

**Recent Activity**:
- Last 10 registered users
- Recent promo code usage

---

### 2. User Management (`/admin/users`)
Complete user administration and management.

**Features**:
- **Search**: By email or username
- **Filters**:
  - Plan (Free, Indicator, Scanner, Station)
  - Status (Active, Banned)
  - Admin Flag (Yes, No, All)
- **Actions per User**:
  - View full profile details
  - Change subscription plan (dropdown)
  - Ban/Unban toggle
  - Delete user (with confirmation)
- **Pagination**: 20 users per page
- **Export**: CSV export of filtered results

**User Detail Modal**:
- Full profile information
- Bacon Points balance
- Login streak
- Earned badges
- Join date and last active

---

### 3. Promo Codes Manager (`/admin/promos`)
Create and manage promotional codes.

**Promo Types**:
1. **Full Access**: Grant specific plan (Indicator/Scanner/Station)
2. **Trial Extension**: Add X days to trial
3. **Discount**: Percentage off (5-100%)

**Configuration**:
- Code name (auto-uppercase)
- Max uses (optional, unlimited if not set)
- Expiry date (optional)
- Active/Inactive toggle

**Table View**:
- Code name
- Type and value
- Current uses / Max uses
- Expiration date
- Active status
- Actions: Edit, Enable/Disable, Delete

---

### 4. Discord Alert Sender (`/admin/discord`)
Send alerts to Discord channels with rich embeds.

**Channel Selection**:
- General
- Signals
- Announcements
- VIP

**Message Types**:
- Signal Alert
- Announcement
- Custom

**Embed Builder**:
- Title
- Description
- Color picker (with hex input)
- Custom fields (add/remove dynamically)

**Features**:
- Live preview panel (Discord-style)
- Schedule send (date/time picker)
- Send immediately
- Sent messages history

---

### 5. Manual Signal Push (`/admin/signals`)
Manually create and push trading signals.

**Signal Configuration**:
- **Symbol**: Ticker (BTC, ETH, AAPL, etc.)
- **Direction**: LONG/SHORT (radio buttons)
- **Horizon**: Scalp/Day/Swing/Long (dropdown)
- **Score**: 0-100 slider
- **Grade**: S/A/B/C (radio buttons)

**Price Levels**:
- Entry Price
- Stop Loss
- Take Profit 1, 2, 3

**Smart Money Concepts (SMC) Tags**:
- FVG (Fair Value Gap)
- OB (Order Block)
- BOS (Break of Structure)
- CHoCH (Change of Character)
- Liquidity Sweep
- Premium/Discount
- Imbalance

**Additional**:
- Notes textarea
- "Push to Discord" checkbox
- Recent manual signals table (last 50)

---

### 6. Analytics Dashboard (`/admin/analytics`)
Comprehensive platform analytics and metrics.

**Date Range Filters**:
- Last 7 Days
- Last 30 Days
- Last 90 Days
- Custom Range

**Charts** (Chart.js):
1. **Revenue Over Time**: Line chart with area fill
2. **Signups Over Time**: Bar chart
3. **Active Users Over Time**: Line chart
4. **Churn Rate Over Time**: Line chart with percentage
5. **Tier Breakdown**: Pie chart (Free/Indicator/Scanner/Station)

**Top Referrers Table**:
- Source name
- Number of signups
- Revenue generated

**Export**:
- CSV export of all analytics data

---

## Design

### Visual Style
- **Glassmorphism**: Frosted glass effect with backdrop blur
- **Dark Theme**: Consistent with platform design
- **Gradient Accents**: Purple/Pink gradient for actions
- **Color Coding**:
  - Admin badge: Gold
  - Active status: Green
  - Banned status: Red
  - Plans: Purple (Station), Orange (Scanner), Blue (Indicator), Gray (Free)

### Mobile Responsive
- Collapsible sidebar (hamburger menu)
- Stacked layouts on small screens
- Touch-friendly buttons and inputs
- Horizontal scroll for tables

### Components
- Reusable glassmorphic cards
- Consistent form styling
- Table pagination
- Modal dialogs
- Toast notifications

---

## Security

### Authentication Guard
```typescript
// +layout.ts
if (!profile.is_admin) {
  throw redirect(303, '/dashboard');
}
```

### Database Operations
All operations use Supabase RLS (Row Level Security):
- Only admins can modify user data
- Audit logs for sensitive operations
- Confirmation dialogs for destructive actions

### Input Validation
- Client-side validation for all forms
- Server-side validation via Supabase
- SQL injection protection (parameterized queries)
- XSS protection (Svelte auto-escaping)

---

## Technical Stack

- **Framework**: SvelteKit 2.x + Svelte 5
- **Language**: TypeScript
- **Database**: Supabase (PostgreSQL)
- **Charts**: Chart.js 4.x
- **Styling**: Tailwind CSS 4.x + Custom CSS
- **State Management**: Svelte 5 Runes ($state, $derived, $effect)

---

## Code Structure

```
station/src/routes/admin/
├── +layout.ts              # Admin guard
├── +layout.svelte          # Admin layout & navigation
├── +page.svelte            # Dashboard overview
├── users/
│   └── +page.svelte        # User management
├── promos/
│   └── +page.svelte        # Promo codes
├── discord/
│   └── +page.svelte        # Discord alerts
├── signals/
│   └── +page.svelte        # Manual signals
└── analytics/
    └── +page.svelte        # Analytics dashboard
```

**Total**: 2,748 lines of code

---

## Usage Examples

### Granting Admin Access
```sql
UPDATE profiles 
SET is_admin = true 
WHERE email = 'admin@baconalgo.com';
```

### Creating a Promo Code
1. Navigate to `/admin/promos`
2. Click "Create New Promo"
3. Enter code name (e.g., "LAUNCH50")
4. Select type: "Discount"
5. Set discount: 50%
6. Set max uses: 100
7. Set expiry: 30 days from now
8. Click "Create Promo Code"

### Pushing a Manual Signal
1. Navigate to `/admin/signals`
2. Enter symbol: BTC
3. Select direction: LONG
4. Set score: 85
5. Set grade: A
6. Enter price levels
7. Check SMC tags
8. Enable "Push to Discord"
9. Click "Push Signal to Scanner"

### Viewing Analytics
1. Navigate to `/admin/analytics`
2. Select date range: "Last 30 Days"
3. Review charts and metrics
4. Click "Export CSV" for detailed data

---

## Future Enhancements

- [ ] A/B Testing Manager
- [ ] Email Campaign Sender
- [ ] Subscription Management
- [ ] Revenue Projections
- [ ] User Activity Heatmaps
- [ ] Real-time Platform Health Monitor
- [ ] API Rate Limiting Controls
- [ ] Feature Flag Management
- [ ] Automated Report Generation
- [ ] Advanced User Segmentation

---

## Support

For issues or questions about the admin panel:
- Check console for errors
- Verify admin permissions
- Review Supabase logs
- Contact development team

---

**Version**: 1.0.0  
**Last Updated**: 2024-02-05  
**Maintained By**: BaconAlgo Team
