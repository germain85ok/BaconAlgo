# BaconAlgo Platform - Deployment & Setup Guide

## 🚀 What's Been Implemented

### ✅ Complete Features

1. **Database Schema (Supabase)**
   - Coupon system with validation
   - Subscription management
   - Broker connections
   - Auto-trade settings
   - Trade execution tracking
   - Row Level Security (RLS) policies

2. **Pricing System**
   - 4 tiers: FREE, INDICATEUR ($20/mo), SCANNER ($30/mo), STATION ($50/mo)
   - Yearly discounts (20% off): $192, $288, $480
   - Live coupon validation
   - Dynamic price display with discounts

3. **Admin Panel**
   - `/admin/coupons` - Full coupon management
   - `/admin/subscriptions` - Subscription oversight with stats
   - Updated navigation

4. **Services**
   - `subscriptions.ts` - Coupon & subscription operations
   - `autoTrade.ts` - Auto-trade settings & broker management
   - `supabase.ts` - Database client

---

## 📋 Setup Instructions

### 1. Environment Variables

Create a `.env` file in the `station` directory:

```env
# Supabase Configuration
PUBLIC_SUPABASE_URL=https://your-project.supabase.co
PUBLIC_SUPABASE_ANON_KEY=your-anon-key-here
SUPABASE_SERVICE_ROLE_KEY=your-service-role-key-here

# Stripe (for payment processing)
PUBLIC_STRIPE_PUBLISHABLE_KEY=pk_test_...
STRIPE_SECRET_KEY=sk_test_...

# Data Providers (for live market data)
FINNHUB_API_KEY_1=your-key-1
FINNHUB_API_KEY_2=your-key-2
FINNHUB_API_KEY_3=your-key-3
TWELVEDATA_API_KEY_1=your-key-1
TWELVEDATA_API_KEY_2=your-key-2
ALPHAVANTAGE_API_KEY_1=your-key-1
ALPHAVANTAGE_API_KEY_2=your-key-2
POLYGON_API_KEY=your-key

# Broker APIs
ALPACA_API_KEY=your-key
ALPACA_SECRET_KEY=your-secret
ALPACA_BASE_URL=https://paper-api.alpaca.markets
```

### 2. Run Database Migration

In your Supabase project:

1. Navigate to SQL Editor
2. Run the migration file: `supabase/migrations/003_pricing_and_coupons.sql`
3. Verify tables created:
   - coupons
   - subscriptions
   - broker_connections
   - auto_trade_settings
   - trade_executions

### 3. Set Admin User

Update the profiles table to grant admin access:

```sql
-- Replace with your email
UPDATE profiles 
SET is_admin = true 
WHERE email = 'germain85@hotmail.com';
```

### 4. Insert Default Coupons

The migration already includes these coupons:
- `BACON20` - 20% off any plan (unlimited uses)
- `LAUNCH50` - 50% off (max 100 uses)
- `YEARLY30` - 30% off yearly plans

### 5. Install Dependencies

```bash
cd station
npm install
```

### 6. Run Development Server

```bash
cd station
npm run dev
```

---

## 🎯 How to Use

### For Users:

1. **Browse Pricing** - Visit `/pricing` to see plans
2. **Apply Coupon** - Enter code like `BACON20` to see discount
3. **Sign Up** - Register and select a plan
4. **Auto-Trade** - Configure at `/dashboard/auto-trade`

### For Admins:

1. **Manage Coupons** - `/admin/coupons`
   - Create new coupons
   - Set discount % or fixed amount
   - Set expiration dates
   - Limit max uses
   - Choose applicable plans

2. **View Subscriptions** - `/admin/subscriptions`
   - See all subscriptions
   - Filter by status/plan
   - View revenue stats
   - Track coupon usage

3. **Manage Users** - `/admin/users`
   - View all users
   - Edit user plans
   - Ban/unban users

---

## 🔐 Security Features

### Implemented:
- ✅ Row Level Security (RLS) on all tables
- ✅ Admin-only access to sensitive operations
- ✅ Unique constraint prevents duplicate active subscriptions
- ✅ Secure update types prevent unauthorized field modifications
- ✅ Encrypted broker credentials (schema ready)
- ✅ Kill switch for auto-trading
- ✅ CodeQL security scan passed

### Best Practices:
- Never expose service role key in client code
- Use environment variables for all secrets
- Validate all user input
- Rate limit API endpoints
- Monitor for suspicious activity

---

## 📊 Database Schema Overview

### Coupons Table
```
- code (unique)
- discount_percent OR discount_amount
- valid_from, valid_until
- max_uses, current_uses
- applicable_plans (array)
- is_active
```

### Subscriptions Table
```
- user_id (unique constraint on active subscriptions)
- plan (FREE, INDICATEUR, SCANNER, STATION)
- status (active, cancelled, expired, past_due)
- billing_cycle (monthly, yearly)
- amount
- coupon_used
- stripe_subscription_id
- current_period_start, current_period_end
```

### Auto-Trade Settings
```
- user_id (unique)
- enabled
- mode (paper, semi-auto, full-auto)
- broker_connection_id
- max_daily_loss
- max_position_size_percent
- max_concurrent_positions
- auto_execute_grades (array)
- symbol_blacklist (array)
- kill_switch_activated
```

---

## 🛠️ Next Steps

### High Priority:
1. **Stripe Integration**
   - Set up Stripe account
   - Create webhook endpoint for subscription events
   - Implement checkout flow

2. **Auto-Trade UI**
   - Complete the auto-trade settings page
   - Add broker connection interface
   - Implement kill switch button

3. **Broker Integrations**
   - Implement credential encryption
   - Test broker API connections
   - Add connection status indicators

### Medium Priority:
4. **API Key Rotation**
   - Implement round-robin rotation
   - Add automatic fallback on rate limits
   - Monitor usage across providers

5. **Charts & Visualizations**
   - TradingView integration
   - Real-time data feeds
   - Indicator overlays

### Low Priority:
6. **Svelte 5 Optimization**
   - Review all reactive statements
   - Optimize $effect usage
   - Improve performance

---

## 🐛 Known Issues & Limitations

1. **Stripe Not Integrated** - Payment processing needs Stripe setup
2. **Auto-Trade Incomplete** - UI needs to be updated to use new backend
3. **Broker Encryption** - Credentials storage needs encryption implementation
4. **API Rotation** - Live data integration pending
5. **Mobile Testing** - Responsive design needs thorough testing

---

## 💡 Testing Checklist

### Before Production:
- [ ] Test coupon validation with all edge cases
- [ ] Verify subscription creation and updates
- [ ] Test admin panel access controls
- [ ] Verify RLS policies work correctly
- [ ] Test auto-trade settings CRUD operations
- [ ] Verify unique subscription constraint
- [ ] Test pricing calculations with various coupons
- [ ] Load test API endpoints
- [ ] Security audit with penetration testing
- [ ] Mobile responsiveness check

---

## 📞 Support

For issues or questions:
- GitHub Issues: [Create an issue](https://github.com/germain85ok/BaconAlgo/issues)
- Email: germain85@hotmail.com

---

## 📄 License

[Your License Here]

---

**Built with ❤️ using Svelte 5, SvelteKit, Supabase, and TailwindCSS**
