-- =============================================================================
-- BaconAlgo SAAS Platform Schema
-- =============================================================================

-- User profiles (extends auth.users)
CREATE TABLE IF NOT EXISTS profiles (
  id UUID REFERENCES auth.users PRIMARY KEY,
  email TEXT,
  username TEXT UNIQUE,
  avatar_url TEXT,
  plan TEXT DEFAULT 'free', -- 'free', 'indicator', 'scanner', 'station'
  plan_expires_at TIMESTAMP WITH TIME ZONE,
  trial_ends_at TIMESTAMP WITH TIME ZONE,
  promo_code_used TEXT,
  referral_code TEXT UNIQUE, -- Their personal referral code
  referred_by TEXT, -- Who referred them
  bacon_points INTEGER DEFAULT 0,
  badges JSONB DEFAULT '[]',
  streak_days INTEGER DEFAULT 0,
  last_active_at TIMESTAMP WITH TIME ZONE,
  is_admin BOOLEAN DEFAULT false,
  is_banned BOOLEAN DEFAULT false,
  two_factor_enabled BOOLEAN DEFAULT false,
  two_factor_secret TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Promo codes
CREATE TABLE IF NOT EXISTS promo_codes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code TEXT UNIQUE NOT NULL,
  type TEXT NOT NULL, -- 'full_access', 'trial', 'discount'
  discount_percent INTEGER,
  trial_days INTEGER,
  plan_grant TEXT, -- Which plan to grant
  max_uses INTEGER,
  current_uses INTEGER DEFAULT 0,
  expires_at TIMESTAMP WITH TIME ZONE,
  is_active BOOLEAN DEFAULT true,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Insert default promo codes
INSERT INTO promo_codes (code, type, plan_grant, max_uses) VALUES
  ('ILOVEBACON&TEA', 'full_access', 'station', NULL),
  ('FREEBACON', 'trial', NULL, NULL),
  ('BACON20', 'discount', NULL, 20),
  ('BACON50', 'discount', NULL, 50)
ON CONFLICT (code) DO NOTHING;

-- Referrals tracking
CREATE TABLE IF NOT EXISTS referrals (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  referrer_id UUID REFERENCES profiles(id),
  referred_id UUID REFERENCES profiles(id),
  status TEXT DEFAULT 'pending', -- 'pending', 'converted', 'paid'
  reward_given BOOLEAN DEFAULT false,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User badges/achievements
CREATE TABLE IF NOT EXISTS achievements (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL UNIQUE,
  description TEXT,
  icon TEXT,
  points INTEGER DEFAULT 0,
  criteria JSONB -- How to earn it
);

-- Insert default achievements
INSERT INTO achievements (name, description, icon, points, criteria) VALUES
  ('First Bacon', 'Complete registration', '🥓', 10, '{"action": "register"}'),
  ('Signal Watcher', 'View 100 signals', '👀', 25, '{"action": "view_signals", "count": 100}'),
  ('Sharpshooter', '10 winning paper trades', '🎯', 50, '{"action": "winning_trades", "count": 10}'),
  ('On Fire', '7-day login streak', '🔥', 30, '{"action": "streak", "days": 7}'),
  ('Bacon Master', 'Complete all Academy courses', '🏆', 100, '{"action": "complete_academy"}'),
  ('Diamond Hands', 'Hold a position for 30+ days', '💎', 75, '{"action": "hold_position", "days": 30}'),
  ('To The Moon', '100%+ gain on paper trade', '🚀', 100, '{"action": "gain_percent", "value": 100}'),
  ('Bacon King', 'Refer 50 users', '👑', 500, '{"action": "referrals", "count": 50}')
ON CONFLICT (name) DO NOTHING;

-- User watchlist
CREATE TABLE IF NOT EXISTS watchlist (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  symbol TEXT NOT NULL,
  notes TEXT,
  added_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  UNIQUE(user_id, symbol)
);

-- Paper trading positions
CREATE TABLE IF NOT EXISTS paper_positions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  symbol TEXT NOT NULL,
  side TEXT NOT NULL, -- 'long', 'short'
  quantity FLOAT NOT NULL,
  entry_price FLOAT NOT NULL,
  current_price FLOAT,
  stop_loss FLOAT,
  take_profit FLOAT,
  pnl FLOAT,
  pnl_percent FLOAT,
  status TEXT DEFAULT 'open', -- 'open', 'closed'
  opened_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  closed_at TIMESTAMP WITH TIME ZONE
);

-- Discord messages log (for admin)
CREATE TABLE IF NOT EXISTS discord_messages (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  admin_id UUID REFERENCES profiles(id),
  channel TEXT,
  message_type TEXT,
  content JSONB,
  sent_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- A/B Tests
CREATE TABLE IF NOT EXISTS ab_tests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  page TEXT NOT NULL,
  variants JSONB NOT NULL,
  traffic_split JSONB,
  status TEXT DEFAULT 'active', -- 'active', 'paused', 'completed'
  winner TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- A/B Test Results
CREATE TABLE IF NOT EXISTS ab_test_results (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  test_id UUID REFERENCES ab_tests(id) ON DELETE CASCADE,
  variant TEXT NOT NULL,
  user_id UUID,
  converted BOOLEAN DEFAULT false,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Academy progress
CREATE TABLE IF NOT EXISTS academy_progress (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  course_id TEXT NOT NULL,
  lesson_id TEXT NOT NULL,
  completed BOOLEAN DEFAULT false,
  quiz_score INTEGER,
  completed_at TIMESTAMP WITH TIME ZONE,
  UNIQUE(user_id, course_id, lesson_id)
);

-- User achievements earned
CREATE TABLE IF NOT EXISTS user_achievements (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
  achievement_id UUID REFERENCES achievements(id),
  earned_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  UNIQUE(user_id, achievement_id)
);

-- =============================================================================
-- INDEXES
-- =============================================================================

CREATE INDEX IF NOT EXISTS idx_profiles_username ON profiles(username);
CREATE INDEX IF NOT EXISTS idx_profiles_plan ON profiles(plan);
CREATE INDEX IF NOT EXISTS idx_profiles_referral_code ON profiles(referral_code);
CREATE INDEX IF NOT EXISTS idx_profiles_is_admin ON profiles(is_admin);

CREATE INDEX IF NOT EXISTS idx_promo_codes_code ON promo_codes(code);
CREATE INDEX IF NOT EXISTS idx_promo_codes_is_active ON promo_codes(is_active);

CREATE INDEX IF NOT EXISTS idx_referrals_referrer_id ON referrals(referrer_id);
CREATE INDEX IF NOT EXISTS idx_referrals_referred_id ON referrals(referred_id);

CREATE INDEX IF NOT EXISTS idx_watchlist_user_id ON watchlist(user_id);
CREATE INDEX IF NOT EXISTS idx_watchlist_symbol ON watchlist(symbol);

CREATE INDEX IF NOT EXISTS idx_paper_positions_user_id ON paper_positions(user_id);
CREATE INDEX IF NOT EXISTS idx_paper_positions_status ON paper_positions(status);

CREATE INDEX IF NOT EXISTS idx_academy_progress_user_id ON academy_progress(user_id);
CREATE INDEX IF NOT EXISTS idx_user_achievements_user_id ON user_achievements(user_id);

-- =============================================================================
-- FUNCTIONS
-- =============================================================================

-- Function to generate unique referral code
CREATE OR REPLACE FUNCTION generate_referral_code()
RETURNS TEXT AS $$
DECLARE
  chars TEXT := 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789';
  result TEXT := '';
  i INTEGER;
BEGIN
  FOR i IN 1..8 LOOP
    result := result || substr(chars, floor(random() * length(chars) + 1)::int, 1);
  END LOOP;
  RETURN result;
END;
$$ LANGUAGE plpgsql;

-- Function to update profile updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Trigger to update profiles.updated_at
CREATE TRIGGER update_profiles_updated_at
  BEFORE UPDATE ON profiles
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

-- Trigger to create profile on user signup
CREATE OR REPLACE FUNCTION handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO profiles (id, email, referral_code, trial_ends_at)
  VALUES (
    NEW.id,
    NEW.email,
    generate_referral_code(),
    NOW() + INTERVAL '7 days'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE TRIGGER on_auth_user_created
  AFTER INSERT ON auth.users
  FOR EACH ROW
  EXECUTE FUNCTION handle_new_user();

-- =============================================================================
-- ROW LEVEL SECURITY (RLS)
-- =============================================================================

ALTER TABLE profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE promo_codes ENABLE ROW LEVEL SECURITY;
ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
ALTER TABLE watchlist ENABLE ROW LEVEL SECURITY;
ALTER TABLE paper_positions ENABLE ROW LEVEL SECURITY;
ALTER TABLE discord_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE ab_tests ENABLE ROW LEVEL SECURITY;
ALTER TABLE ab_test_results ENABLE ROW LEVEL SECURITY;
ALTER TABLE academy_progress ENABLE ROW LEVEL SECURITY;
ALTER TABLE achievements ENABLE ROW LEVEL SECURITY;
ALTER TABLE user_achievements ENABLE ROW LEVEL SECURITY;

-- Profiles policies
CREATE POLICY "Users can view their own profile"
  ON profiles FOR SELECT
  USING (auth.uid() = id);

CREATE POLICY "Users can update their own profile"
  ON profiles FOR UPDATE
  USING (auth.uid() = id);

CREATE POLICY "Admins can view all profiles"
  ON profiles FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

CREATE POLICY "Admins can update all profiles"
  ON profiles FOR UPDATE
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Promo codes policies
CREATE POLICY "Anyone can view active promo codes"
  ON promo_codes FOR SELECT
  USING (is_active = true);

CREATE POLICY "Admins can manage promo codes"
  ON promo_codes FOR ALL
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Watchlist policies
CREATE POLICY "Users can manage their own watchlist"
  ON watchlist FOR ALL
  USING (auth.uid() = user_id);

-- Paper positions policies
CREATE POLICY "Users can manage their own paper positions"
  ON paper_positions FOR ALL
  USING (auth.uid() = user_id);

-- Academy progress policies
CREATE POLICY "Users can manage their own academy progress"
  ON academy_progress FOR ALL
  USING (auth.uid() = user_id);

-- Achievements policies
CREATE POLICY "Anyone can view achievements"
  ON achievements FOR SELECT
  USING (true);

-- User achievements policies
CREATE POLICY "Users can view their own achievements"
  ON user_achievements FOR SELECT
  USING (auth.uid() = user_id);

CREATE POLICY "Users can earn achievements"
  ON user_achievements FOR INSERT
  WITH CHECK (auth.uid() = user_id);

-- Discord messages policies (admin only)
CREATE POLICY "Admins can manage discord messages"
  ON discord_messages FOR ALL
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- A/B tests policies (admin only)
CREATE POLICY "Admins can manage A/B tests"
  ON ab_tests FOR ALL
  USING (
    EXISTS (
      SELECT 1 FROM profiles
      WHERE id = auth.uid() AND is_admin = true
    )
  );

-- Referrals policies
CREATE POLICY "Users can view their referrals"
  ON referrals FOR SELECT
  USING (auth.uid() = referrer_id OR auth.uid() = referred_id);

CREATE POLICY "System can create referrals"
  ON referrals FOR INSERT
  WITH CHECK (true);
