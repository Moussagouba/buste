-- 004_odds.sql: Real-time and historical odds snapshots
CREATE TABLE IF NOT EXISTS odds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    market_id UUID NOT NULL REFERENCES markets(id) ON DELETE CASCADE,
    outcome_name VARCHAR(100) NOT NULL, -- "1", "X", "2", "Over", "Under", "Yes", "No"
    price DOUBLE PRECISION NOT NULL,    -- Decimal odds, e.g. 2.15
    implied_probability DOUBLE PRECISION NOT NULL, -- 1 / price
    bookmaker_margin DOUBLE PRECISION DEFAULT NULL, -- Overround percentage
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_odds_market_id ON odds(market_id);
CREATE INDEX IF NOT EXISTS idx_odds_recorded_at ON odds(recorded_at);

