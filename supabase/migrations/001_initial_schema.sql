-- =============================================================================
-- BaconAlgo Initial Database Schema
-- =============================================================================

-- Enable necessary extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================================================
-- ENUMS
-- =============================================================================

-- Signal direction
CREATE TYPE signal_direction AS ENUM ('BUY', 'SELL');

-- Signal grade
CREATE TYPE signal_grade AS ENUM ('S', 'A', 'B', 'C');

-- Donation status
CREATE TYPE donation_status AS ENUM ('PENDING', 'COMPLETED', 'FAILED');

-- Donation tier
CREATE TYPE donation_tier AS ENUM ('BACON_BIT', 'CRISPY', 'SIZZLING', 'LEGENDARY');

-- =============================================================================
-- TABLES
-- =============================================================================

-- Trading signals table
CREATE TABLE signals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ticker TEXT NOT NULL,
    direction signal_direction NOT NULL,
    grade signal_grade NOT NULL,
    score NUMERIC(10, 2) NOT NULL,
    indicators JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Donations table
CREATE TABLE donations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    donor_name TEXT NOT NULL,
    amount NUMERIC(10, 2) NOT NULL CHECK (amount > 0),
    message TEXT,
    tier donation_tier NOT NULL,
    status donation_status NOT NULL DEFAULT 'PENDING',
    payment_method TEXT NOT NULL,
    transaction_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Donors aggregation table
CREATE TABLE donors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL,
    total_donated NUMERIC(10, 2) NOT NULL DEFAULT 0,
    donation_count INTEGER NOT NULL DEFAULT 0,
    first_donation_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_donation_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Sponsors table
CREATE TABLE sponsors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    logo_url TEXT NOT NULL,
    website_url TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Positions table (for tracking signal executions)
CREATE TABLE positions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    signal_id UUID REFERENCES signals(id) ON DELETE SET NULL,
    ticker TEXT NOT NULL,
    entry_price NUMERIC(10, 4) NOT NULL,
    quantity NUMERIC(10, 4) NOT NULL,
    status TEXT NOT NULL DEFAULT 'OPEN',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMPTZ
);

-- Daily stats aggregation table
CREATE TABLE daily_stats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    date DATE UNIQUE NOT NULL,
    total_donations NUMERIC(10, 2) NOT NULL DEFAULT 0,
    donation_count INTEGER NOT NULL DEFAULT 0,
    unique_donors INTEGER NOT NULL DEFAULT 0,
    signals_generated INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Song requests table
CREATE TABLE song_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    requester_name TEXT NOT NULL,
    song_title TEXT NOT NULL,
    artist TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    played_at TIMESTAMPTZ
);

-- Stream configuration table
CREATE TABLE stream_config (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key TEXT UNIQUE NOT NULL,
    value JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- INDEXES
-- =============================================================================

-- Signals indexes
CREATE INDEX idx_signals_ticker ON signals(ticker);
CREATE INDEX idx_signals_direction ON signals(direction);
CREATE INDEX idx_signals_grade ON signals(grade);
CREATE INDEX idx_signals_created_at ON signals(created_at DESC);
CREATE INDEX idx_signals_score ON signals(score DESC);

-- Donations indexes
CREATE INDEX idx_donations_donor_name ON donations(donor_name);
CREATE INDEX idx_donations_status ON donations(status);
CREATE INDEX idx_donations_created_at ON donations(created_at DESC);
CREATE INDEX idx_donations_tier ON donations(tier);
CREATE INDEX idx_donations_amount ON donations(amount DESC);

-- Donors indexes
CREATE INDEX idx_donors_total_donated ON donors(total_donated DESC);
CREATE INDEX idx_donors_donation_count ON donors(donation_count DESC);
CREATE INDEX idx_donors_name ON donors(name);

-- Sponsors indexes
CREATE INDEX idx_sponsors_active ON sponsors(active);
CREATE INDEX idx_sponsors_display_order ON sponsors(display_order);

-- Positions indexes
CREATE INDEX idx_positions_ticker ON positions(ticker);
CREATE INDEX idx_positions_status ON positions(status);
CREATE INDEX idx_positions_signal_id ON positions(signal_id);
CREATE INDEX idx_positions_created_at ON positions(created_at DESC);

-- Daily stats indexes
CREATE INDEX idx_daily_stats_date ON daily_stats(date DESC);

-- Song requests indexes
CREATE INDEX idx_song_requests_status ON song_requests(status);
CREATE INDEX idx_song_requests_created_at ON song_requests(created_at DESC);

-- Stream config indexes
CREATE INDEX idx_stream_config_key ON stream_config(key);

-- =============================================================================
-- FUNCTIONS
-- =============================================================================

-- Function to update donor statistics
CREATE OR REPLACE FUNCTION update_donor_stats()
RETURNS TRIGGER AS $$
BEGIN
    -- Only process COMPLETED donations
    IF NEW.status = 'COMPLETED' THEN
        -- Insert or update donor record
        INSERT INTO donors (name, total_donated, donation_count, first_donation_at, last_donation_at)
        VALUES (NEW.donor_name, NEW.amount, 1, NEW.created_at, NEW.created_at)
        ON CONFLICT (name) DO UPDATE
        SET
            total_donated = donors.total_donated + NEW.amount,
            donation_count = donors.donation_count + 1,
            last_donation_at = NEW.created_at;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update daily donation statistics
CREATE OR REPLACE FUNCTION update_daily_donation_stats()
RETURNS TRIGGER AS $$
DECLARE
    donation_date DATE;
BEGIN
    IF NEW.status = 'COMPLETED' THEN
        donation_date := DATE(NEW.created_at);
        
        -- Insert or update daily stats
        INSERT INTO daily_stats (date, total_donations, donation_count, unique_donors)
        VALUES (
            donation_date,
            NEW.amount,
            1,
            (SELECT COUNT(DISTINCT donor_name) FROM donations WHERE DATE(created_at) = donation_date AND status = 'COMPLETED')
        )
        ON CONFLICT (date) DO UPDATE
        SET
            total_donations = daily_stats.total_donations + NEW.amount,
            donation_count = daily_stats.donation_count + 1,
            unique_donors = (SELECT COUNT(DISTINCT donor_name) FROM donations WHERE DATE(created_at) = donation_date AND status = 'COMPLETED'),
            updated_at = NOW();
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update daily signal statistics
CREATE OR REPLACE FUNCTION update_daily_signal_stats()
RETURNS TRIGGER AS $$
DECLARE
    signal_date DATE;
BEGIN
    signal_date := DATE(NEW.created_at);
    
    -- Insert or update daily stats
    INSERT INTO daily_stats (date, signals_generated)
    VALUES (signal_date, 1)
    ON CONFLICT (date) DO UPDATE
    SET
        signals_generated = daily_stats.signals_generated + 1,
        updated_at = NOW();
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to get top donors
CREATE OR REPLACE FUNCTION get_top_donors(limit_count INTEGER DEFAULT 10)
RETURNS TABLE (
    id UUID,
    name TEXT,
    total_donated NUMERIC,
    donation_count INTEGER,
    first_donation_at TIMESTAMPTZ,
    last_donation_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        donors.id,
        donors.name,
        donors.total_donated,
        donors.donation_count,
        donors.first_donation_at,
        donors.last_donation_at
    FROM donors
    ORDER BY donors.total_donated DESC
    LIMIT limit_count;
END;
$$ LANGUAGE plpgsql;

-- Function to get today's donations
CREATE OR REPLACE FUNCTION get_todays_donations()
RETURNS TABLE (
    id UUID,
    donor_name TEXT,
    amount NUMERIC,
    message TEXT,
    tier donation_tier,
    status donation_status,
    payment_method TEXT,
    transaction_id TEXT,
    created_at TIMESTAMPTZ
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        donations.id,
        donations.donor_name,
        donations.amount,
        donations.message,
        donations.tier,
        donations.status,
        donations.payment_method,
        donations.transaction_id,
        donations.created_at
    FROM donations
    WHERE DATE(donations.created_at) = CURRENT_DATE
    ORDER BY donations.created_at DESC;
END;
$$ LANGUAGE plpgsql;

-- Function to get donation goal progress
CREATE OR REPLACE FUNCTION get_donation_goal_progress(goal_amount NUMERIC)
RETURNS TABLE (
    total NUMERIC,
    goal NUMERIC,
    progress_percentage NUMERIC,
    remaining NUMERIC
) AS $$
DECLARE
    total_today NUMERIC;
BEGIN
    -- Get total donations for today
    SELECT COALESCE(SUM(amount), 0)
    INTO total_today
    FROM donations
    WHERE DATE(created_at) = CURRENT_DATE
    AND status = 'COMPLETED';
    
    -- Return progress data
    RETURN QUERY
    SELECT
        total_today,
        goal_amount,
        CASE
            WHEN goal_amount > 0 THEN ROUND((total_today / goal_amount * 100), 2)
            ELSE 0
        END,
        GREATEST(goal_amount - total_today, 0);
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Trigger to update donor stats when donation is completed
CREATE TRIGGER trigger_update_donor_stats
AFTER INSERT OR UPDATE ON donations
FOR EACH ROW
EXECUTE FUNCTION update_donor_stats();

-- Trigger to update daily donation stats
CREATE TRIGGER trigger_update_daily_donation_stats
AFTER INSERT OR UPDATE ON donations
FOR EACH ROW
EXECUTE FUNCTION update_daily_donation_stats();

-- Trigger to update daily signal stats
CREATE TRIGGER trigger_update_daily_signal_stats
AFTER INSERT ON signals
FOR EACH ROW
EXECUTE FUNCTION update_daily_signal_stats();

-- =============================================================================
-- ROW LEVEL SECURITY (RLS)
-- =============================================================================

-- Enable RLS on all tables
ALTER TABLE signals ENABLE ROW LEVEL SECURITY;
ALTER TABLE donations ENABLE ROW LEVEL SECURITY;
ALTER TABLE donors ENABLE ROW LEVEL SECURITY;
ALTER TABLE sponsors ENABLE ROW LEVEL SECURITY;
ALTER TABLE positions ENABLE ROW LEVEL SECURITY;
ALTER TABLE daily_stats ENABLE ROW LEVEL SECURITY;
ALTER TABLE song_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE stream_config ENABLE ROW LEVEL SECURITY;

-- Public read access for signals
CREATE POLICY "Public read access for signals"
ON signals FOR SELECT
USING (true);

-- Public read access for completed donations
CREATE POLICY "Public read access for donations"
ON donations FOR SELECT
USING (true);

-- Public read access for donors
CREATE POLICY "Public read access for donors"
ON donors FOR SELECT
USING (true);

-- Public read access for active sponsors
CREATE POLICY "Public read access for sponsors"
ON sponsors FOR SELECT
USING (active = true);

-- Public read access for positions
CREATE POLICY "Public read access for positions"
ON positions FOR SELECT
USING (true);

-- Public read access for daily stats
CREATE POLICY "Public read access for daily_stats"
ON daily_stats FOR SELECT
USING (true);

-- Public read access for song requests
CREATE POLICY "Public read access for song_requests"
ON song_requests FOR SELECT
USING (true);

-- Public insert for donations (for donation submissions)
CREATE POLICY "Public insert for donations"
ON donations FOR INSERT
WITH CHECK (true);

-- Public insert for song requests
CREATE POLICY "Public insert for song_requests"
ON song_requests FOR INSERT
WITH CHECK (true);

-- Public read access for stream config
CREATE POLICY "Public read access for stream_config"
ON stream_config FOR SELECT
USING (true);

-- =============================================================================
-- REALTIME
-- =============================================================================

-- Enable realtime for donations
ALTER PUBLICATION supabase_realtime ADD TABLE donations;

-- Enable realtime for signals
ALTER PUBLICATION supabase_realtime ADD TABLE signals;

-- Enable realtime for song requests
ALTER PUBLICATION supabase_realtime ADD TABLE song_requests;

-- =============================================================================
-- INITIAL DATA
-- =============================================================================

-- Insert default stream configuration
INSERT INTO stream_config (key, value) VALUES
('donation_goal', '{"daily": 100, "weekly": 500, "monthly": 2000}'),
('donation_tiers', '{"BACON_BIT": 2, "CRISPY": 5, "SIZZLING": 10, "LEGENDARY": 20}'),
('alert_settings', '{"min_amount": 2, "duration": 10, "show_animation": true}'),
('feature_flags', '{"donations": true, "sponsors": true, "song_requests": true, "chat_overlay": true}')
ON CONFLICT (key) DO NOTHING;

-- Insert sample sponsors (optional - comment out if not needed)
-- Note: Uncomment this if you want sample data
-- INSERT INTO sponsors (name, logo_url, website_url, active, display_order) VALUES
-- ('BaconAlgo', 'https://placeholder.com/logo1.png', 'https://baconalgo.com', true, 1);

-- =============================================================================
-- COMMENTS
-- =============================================================================

COMMENT ON TABLE signals IS 'Trading signals generated by BaconAlgo scanner';
COMMENT ON TABLE donations IS 'Donations received during streams';
COMMENT ON TABLE donors IS 'Aggregated donor statistics';
COMMENT ON TABLE sponsors IS 'Stream sponsors and partners';
COMMENT ON TABLE positions IS 'Trading positions opened from signals';
COMMENT ON TABLE daily_stats IS 'Daily aggregated statistics';
COMMENT ON TABLE song_requests IS 'Song requests from viewers';
COMMENT ON TABLE stream_config IS 'Stream configuration and settings';

COMMENT ON FUNCTION update_donor_stats() IS 'Automatically updates donor totals when donations are completed';
COMMENT ON FUNCTION update_daily_donation_stats() IS 'Updates daily donation statistics';
COMMENT ON FUNCTION update_daily_signal_stats() IS 'Updates daily signal statistics';
COMMENT ON FUNCTION get_top_donors(INTEGER) IS 'Returns top donors by total donation amount';
COMMENT ON FUNCTION get_todays_donations() IS 'Returns all donations for current day';
COMMENT ON FUNCTION get_donation_goal_progress(NUMERIC) IS 'Calculates progress towards donation goal';
