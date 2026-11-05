-- 002_events.sql: Sports events collected from bookmaker feed
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    external_id VARCHAR(100) NOT NULL UNIQUE, -- Melbet event ID
    sport_name VARCHAR(100) NOT NULL,        -- Football, Basketball, Tennis, etc.
    league_name VARCHAR(255) NOT NULL,       -- Premier League, Champions League, etc.
    home_team VARCHAR(255) NOT NULL,
    away_team VARCHAR(255) NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'SCHEDULED', -- SCHEDULED, LIVE, FINISHED, CANCELLED
    home_score INT DEFAULT NULL,
    away_score INT DEFAULT NULL,
    period_scores JSONB DEFAULT NULL,
    raw_metadata JSONB DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_events_start_time ON events(start_time);
CREATE INDEX IF NOT EXISTS idx_events_sport ON events(sport_name);
CREATE INDEX IF NOT EXISTS idx_events_status ON events(status);

