-- =============================================================================
-- BaconAlgo Pricing, Coupons, and Subscriptions Schema
-- =============================================================================

-- =============================================================================
-- COUPONS TABLE
-- =============================================================================
CREATE TABLE IF NOT EXISTS coupons (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code VARCHAR(50) UNIQUE NOT NULL,
  discount_percent INTEGER CHECK (discount_percent >= 0 AND discount_percent <= 100),
  discount_amount DECIMAL(10,2) CHECK (discount_amount >= 0),
  valid_from TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  valid_until TIMESTAMP WITH TIME ZONE,
  max_uses INTEGER,
  current_uses INTEGER DEFAULT 0,
  applicable_plans TEXT[] DEFAULT ARRAY['INDICATEUR', 'SCANNER', 'STATION'], -- Plans this coupon applies to
  is_active BOOLEAN DEFAULT true,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  CONSTRAINT check_discount_type CHECK (
    (discount_percent IS NOT NULL AND discount_amount IS NULL) OR
    (discount_percent IS NULL AND discount_amount IS NOT NULL)
  )
);

-- Insert example coupons
INSERT INTO coupons (code, discount_percent, applicable_plans, max_uses) VALUES
  ('BACON20', 20, ARRAY['INDICATEUR', 'SCANNER', 'STATION'], NULL),
  ('LAUNCH50', 50, ARRAY['INDICATEUR', 'SCANNER', 'STATION'], 100),
  ('YEARLY30', 30, ARRAY['INDICATEUR', 'SCANNER', 'STATION'], NULL)
ON CONFLICT (code) DO NOTHING;

-- =============================================================================
-- SUBSCRIPTIONS TABLE
-- =============================================================================
CREATE TABLE IF NOT EXISTS subscriptions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  plan VARCHAR(50) NOT NULL CHECK (plan IN ('FREE', 'INDICATEUR', 'SCANNER', 'STATION')),
  status VARCHAR(50) DEFAULT 'active' CHECK (status IN ('active', 'cancelled', 'expired', 'past_due')),
  billing_cycle VARCHAR(20) CHECK (billing_cycle IN ('monthly', 'yearly')),
  amount DECIMAL(10,2) NOT NULL,
  coupon_used VARCHAR(50),
  stripe_subscription_id VARCHAR(255),
  stripe_customer_id VARCHAR(255),
  current_period_start TIMESTAMP WITH TIME ZONE,
  current_period_end TIMESTAMP WITH TIME ZONE,
  cancelled_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =============================================================================
-- BROKER CONNECTIONS TABLE
-- =============================================================================
CREATE TABLE IF NOT EXISTS broker_connections (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  broker_name VARCHAR(50) NOT NULL CHECK (broker_name IN ('alpaca', 'ib', 'questrade', 'bitget')),
  api_key_encrypted TEXT,
  api_secret_encrypted TEXT,
  additional_config JSONB DEFAULT '{}', -- For broker-specific settings
  is_paper BOOLEAN DEFAULT true,
  is_active BOOLEAN DEFAULT true,
  last_connected_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =============================================================================
-- UPDATE SIGNALS TABLE
-- =============================================================================
-- Add new fields to existing signals table
ALTER TABLE signals ADD COLUMN IF NOT EXISTS symbol TEXT;
ALTER TABLE signals ADD COLUMN IF NOT EXISTS entry_price DECIMAL(10,2);
ALTER TABLE signals ADD COLUMN IF NOT EXISTS target1 DECIMAL(10,2);
ALTER TABLE signals ADD COLUMN IF NOT EXISTS target2 DECIMAL(10,2);
ALTER TABLE signals ADD COLUMN IF NOT EXISTS target3 DECIMAL(10,2);
ALTER TABLE signals ADD COLUMN IF NOT EXISTS stop_loss DECIMAL(10,2);
ALTER TABLE signals ADD COLUMN IF NOT EXISTS timeframe VARCHAR(10);
ALTER TABLE signals ADD COLUMN IF NOT EXISTS status VARCHAR(20) DEFAULT 'ACTIVE';
ALTER TABLE signals ADD COLUMN IF NOT EXISTS created_by UUID REFERENCES profiles(id);

-- Update existing ticker column to symbol if it exists
UPDATE signals SET symbol = ticker WHERE symbol IS NULL AND ticker IS NOT NULL;

-- =============================================================================
-- AUTO-TRADE SETTINGS TABLE
-- =============================================================================
CREATE TABLE IF NOT EXISTS auto_trade_settings (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE UNIQUE,
  enabled BOOLEAN DEFAULT false,
  mode VARCHAR(20) DEFAULT 'paper' CHECK (mode IN ('paper', 'semi-auto', 'full-auto')),
  broker_connection_id UUID REFERENCES broker_connections(id) ON DELETE SET NULL,
  
  -- Risk Management
  max_daily_loss DECIMAL(10,2) DEFAULT 500.00,
  max_position_size_percent DECIMAL(5,2) DEFAULT 10.00,
  max_concurrent_positions INTEGER DEFAULT 5,
  
  -- Trading Rules
  auto_execute_grades TEXT[] DEFAULT ARRAY['S', 'A'], -- Which signal grades to auto-execute
  symbol_blacklist TEXT[] DEFAULT '{}',
  trading_hours_start TIME,
  trading_hours_end TIME,
  
  -- Notification Settings
  discord_webhook_url TEXT,
  telegram_chat_id TEXT,
  email_notifications BOOLEAN DEFAULT true,
  
  -- Kill Switch
  kill_switch_activated BOOLEAN DEFAULT false,
  kill_switch_activated_at TIMESTAMP WITH TIME ZONE,
  
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =============================================================================
-- TRADE EXECUTIONS TABLE
-- =============================================================================
CREATE TABLE IF NOT EXISTS trade_executions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  signal_id UUID REFERENCES signals(id) ON DELETE SET NULL,
  broker_connection_id UUID REFERENCES broker_connections(id) ON DELETE SET NULL,
  
  symbol TEXT NOT NULL,
  side VARCHAR(10) NOT NULL CHECK (side IN ('BUY', 'SELL', 'LONG', 'SHORT')),
  quantity DECIMAL(20,8) NOT NULL,
  entry_price DECIMAL(10,2),
  filled_price DECIMAL(10,2),
  
  stop_loss DECIMAL(10,2),
  take_profit DECIMAL(10,2),
  
  status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'filled', 'partial', 'cancelled', 'failed')),
  broker_order_id TEXT,
  
  mode VARCHAR(20) CHECK (mode IN ('paper', 'semi-auto', 'full-auto')),
  
  pnl DECIMAL(10,2),
  pnl_percent DECIMAL(5,2),
  
  error_message TEXT,
  
  executed_at TIMESTAMP WITH TIME ZONE,
  closed_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- =============================================================================
-- INDEXES
-- =============================================================================
CREATE INDEX IF NOT EXISTS idx_coupons_code ON coupons(code);
CREATE INDEX IF NOT EXISTS idx_coupons_is_active ON coupons(is_active);
CREATE INDEX IF NOT EXISTS idx_coupons_valid_until ON coupons(valid_until);

CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);
CREATE INDEX IF NOT EXISTS idx_subscriptions_plan ON subscriptions(plan);
CREATE INDEX IF NOT EXISTS idx_subscriptions_stripe_subscription_id ON subscriptions(stripe_subscription_id);

CREATE INDEX IF NOT EXISTS idx_broker_connections_user_id ON broker_connections(user_id);
CREATE INDEX IF NOT EXISTS idx_broker_connections_broker_name ON broker_connections(broker_name);
CREATE INDEX IF NOT EXISTS idx_broker_connections_is_active ON broker_connections(is_active);

CREATE INDEX IF NOT EXISTS idx_signals_symbol ON signals(symbol);
CREATE INDEX IF NOT EXISTS idx_signals_status ON signals(status);
CREATE INDEX IF NOT EXISTS idx_signals_timeframe ON signals(timeframe);
CREATE INDEX IF NOT EXISTS idx_signals_created_by ON signals(created_by);

CREATE INDEX IF NOT EXISTS idx_auto_trade_settings_user_id ON auto_trade_settings(user_id);
CREATE INDEX IF NOT EXISTS idx_auto_trade_settings_enabled ON auto_trade_settings(enabled);

CREATE INDEX IF NOT EXISTS idx_trade_executions_user_id ON trade_executions(user_id);
CREATE INDEX IF NOT EXISTS idx_trade_executions_signal_id ON trade_executions(signal_id);
CREATE INDEX IF NOT EXISTS idx_trade_executions_status ON trade_executions(status);
CREATE INDEX IF NOT EXISTS idx_trade_executions_symbol ON trade_executions(symbol);

-- =============================================================================
-- FUNCTIONS
-- =============================================================================

-- Function to validate coupon
CREATE OR REPLACE FUNCTION validate_coupon(
  coupon_code TEXT,
  plan_name TEXT
)
RETURNS TABLE (
  valid BOOLEAN,
  discount_percent INTEGER,
  discount_amount DECIMAL,
  message TEXT
) AS $$
DECLARE
  coupon_record RECORD;
BEGIN
  -- Find the coupon
  SELECT * INTO coupon_record
  FROM coupons
  WHERE code = coupon_code;
  
  -- Check if coupon exists
  IF NOT FOUND THEN
    RETURN QUERY SELECT false, NULL::INTEGER, NULL::DECIMAL, 'Invalid coupon code';
    RETURN;
  END IF;
  
  -- Check if active
  IF NOT coupon_record.is_active THEN
    RETURN QUERY SELECT false, NULL::INTEGER, NULL::DECIMAL, 'Coupon is not active';
    RETURN;
  END IF;
  
  -- Check expiration
  IF coupon_record.valid_until IS NOT NULL AND coupon_record.valid_until < NOW() THEN
    RETURN QUERY SELECT false, NULL::INTEGER, NULL::DECIMAL, 'Coupon has expired';
    RETURN;
  END IF;
  
  -- Check max uses
  IF coupon_record.max_uses IS NOT NULL AND coupon_record.current_uses >= coupon_record.max_uses THEN
    RETURN QUERY SELECT false, NULL::INTEGER, NULL::DECIMAL, 'Coupon has reached maximum uses';
    RETURN;
  END IF;
  
  -- Check if applicable to plan
  IF NOT (plan_name = ANY(coupon_record.applicable_plans)) THEN
    RETURN QUERY SELECT false, NULL::INTEGER, NULL::DECIMAL, 'Coupon not applicable to this plan';
    RETURN;
  END IF;
  
  -- Valid coupon
  RETURN QUERY SELECT 
    true,
    coupon_record.discount_percent,
    coupon_record.discount_amount,
    'Coupon valid'::TEXT;
END;
$$ LANGUAGE plpgsql;

-- Function to increment coupon usage
CREATE OR REPLACE FUNCTION increment_coupon_usage(coupon_code TEXT)
RETURNS VOID AS $$
BEGIN
  UPDATE coupons
  SET current_uses = current_uses + 1
  WHERE code = coupon_code;
END;
$$ LANGUAGE plpgsql;

-- Function to update subscription updated_at
CREATE OR REPLACE FUNCTION update_subscription_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update broker connection updated_at
CREATE OR REPLACE FUNCTION update_broker_connection_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update auto_trade_settings updated_at
CREATE OR REPLACE FUNCTION update_auto_trade_settings_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- TRIGGERS
-- =============================================================================

CREATE TRIGGER update_subscriptions_updated_at
  BEFORE UPDATE ON subscriptions
  FOR EACH ROW
  EXECUTE FUNCTION update_subscription_updated_at();

CREATE TRIGGER update_broker_connections_updated_at
  BEFORE UPDATE ON broker_connections
  FOR EACH ROW
  EXECUTE FUNCTION update_broker_connection_updated_at();

CREATE TRIGGER update_auto_trade_settings_updated_at
  BEFORE UPDATE ON auto_trade_settings
  FOR EACH ROW
  EXECUTE FUNCTION update_auto_trade_settings_updated_at();

-- =============================================================================
-- ROW LEVEL SECURITY (RLS)
-- =============================================================================

ALTER TABLE coupons ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE broker_connections ENABLE ROW LEVEL SECURITY;
ALTER TABLE auto_trade_settings ENABLE ROW LEVEL SECURITY;
ALTER TABLE trade_executions ENABLE ROW LEVEL SECURITY;

-- Coupons policies (public can validate, admin can manage)
CREATE POLICY "Anyone can view active coupons"
  ON coupons FOR SELECT
  USING (is_active = true);

CREATE POLICY "Admins can manage coupons"
  ON coupons FOR ALL
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Subscriptions policies
CREATE POLICY "Users can view their own subscriptions"
  ON subscriptions FOR SELECT
  USING (auth.uid() = user_id);

CREATE POLICY "Users can insert their own subscriptions"
  ON subscriptions FOR INSERT
  WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Admins can view all subscriptions"
  ON subscriptions FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

CREATE POLICY "Admins can manage all subscriptions"
  ON subscriptions FOR ALL
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Broker connections policies
CREATE POLICY "Users can manage their own broker connections"
  ON broker_connections FOR ALL
  USING (auth.uid() = user_id);

-- Auto-trade settings policies
CREATE POLICY "Users can manage their own auto-trade settings"
  ON auto_trade_settings FOR ALL
  USING (auth.uid() = user_id);

-- Trade executions policies
CREATE POLICY "Users can view their own trade executions"
  ON trade_executions FOR SELECT
  USING (auth.uid() = user_id);

CREATE POLICY "Users can insert their own trade executions"
  ON trade_executions FOR INSERT
  WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Admins can view all trade executions"
  ON trade_executions FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE coupons IS 'Promotional coupon codes for discounts';
COMMENT ON TABLE subscriptions IS 'User subscription records for paid plans';
COMMENT ON TABLE broker_connections IS 'User broker API connections for auto-trading';
COMMENT ON TABLE auto_trade_settings IS 'Auto-trade configuration and risk management settings';
COMMENT ON TABLE trade_executions IS 'Record of all trade executions (paper and live)';

COMMENT ON FUNCTION validate_coupon(TEXT, TEXT) IS 'Validates if a coupon code is valid for a given plan';
COMMENT ON FUNCTION increment_coupon_usage(TEXT) IS 'Increments the usage count for a coupon code';
