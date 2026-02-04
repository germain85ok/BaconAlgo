-- BaconAlgo SAAS 2030 Database Schema
-- Users table (extends Supabase auth.users)
CREATE TABLE IF NOT EXISTS profiles (
  id UUID REFERENCES auth.users PRIMARY KEY,
  email TEXT,
  plan TEXT DEFAULT 'free', -- 'free', 'indicator', 'scanner', 'station'
  promo_code_used TEXT,
  trial_ends_at TIMESTAMP,
  subscription_ends_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Promo codes
CREATE TABLE IF NOT EXISTS promo_codes (
  code TEXT PRIMARY KEY,
  type TEXT NOT NULL, -- 'full_access', 'trial', 'discount'
  discount_percent INTEGER,
  trial_days INTEGER,
  max_uses INTEGER,
  uses INTEGER DEFAULT 0,
  active BOOLEAN DEFAULT true,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Insert default promo codes
INSERT INTO promo_codes (code, type, discount_percent, trial_days, max_uses, active)
VALUES 
  ('ILOVEBACON&TEA', 'full_access', NULL, NULL, NULL, true),
  ('FREEBACON', 'trial', NULL, 7, NULL, true),
  ('BACON20', 'discount', 20, NULL, NULL, true),
  ('BACON50', 'discount', 50, NULL, NULL, true)
ON CONFLICT (code) DO NOTHING;

-- Portfolio positions
CREATE TABLE IF NOT EXISTS positions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  symbol TEXT NOT NULL,
  side TEXT NOT NULL, -- 'long', 'short'
  qty FLOAT NOT NULL,
  entry_price FLOAT NOT NULL,
  stop_loss FLOAT,
  take_profit_1 FLOAT,
  take_profit_2 FLOAT,
  take_profit_3 FLOAT,
  notes TEXT,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- Signals history
CREATE TABLE IF NOT EXISTS signals (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  symbol TEXT NOT NULL,
  timeframe TEXT NOT NULL,
  signal_type TEXT NOT NULL, -- 'scalp', 'day', 'swing', 'long'
  direction TEXT NOT NULL, -- 'bullish', 'bearish'
  score INTEGER NOT NULL,
  entry_price FLOAT NOT NULL,
  stop_loss FLOAT,
  target_price FLOAT,
  tags JSONB,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_positions_user_id ON positions(user_id);
CREATE INDEX IF NOT EXISTS idx_positions_symbol ON positions(symbol);
CREATE INDEX IF NOT EXISTS idx_signals_symbol ON signals(symbol);
CREATE INDEX IF NOT EXISTS idx_signals_created_at ON signals(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_signals_score ON signals(score DESC);

-- Enable Row Level Security (RLS)
ALTER TABLE profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE positions ENABLE ROW LEVEL SECURITY;
ALTER TABLE signals ENABLE ROW LEVEL SECURITY;

-- RLS Policies for profiles
CREATE POLICY "Users can view own profile" 
  ON profiles FOR SELECT 
  USING (auth.uid() = id);

CREATE POLICY "Users can update own profile" 
  ON profiles FOR UPDATE 
  USING (auth.uid() = id);

-- RLS Policies for positions
CREATE POLICY "Users can view own positions" 
  ON positions FOR SELECT 
  USING (auth.uid() = user_id);

CREATE POLICY "Users can insert own positions" 
  ON positions FOR INSERT 
  WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own positions" 
  ON positions FOR UPDATE 
  USING (auth.uid() = user_id);

CREATE POLICY "Users can delete own positions" 
  ON positions FOR DELETE 
  USING (auth.uid() = user_id);

-- RLS Policies for signals (public read)
CREATE POLICY "Anyone can view signals" 
  ON signals FOR SELECT 
  TO authenticated, anon
  USING (true);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for positions updated_at
CREATE TRIGGER update_positions_updated_at 
  BEFORE UPDATE ON positions
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
