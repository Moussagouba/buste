-- 003_markets.sql: Betting markets linked to events
CREATE TABLE IF NOT EXISTS markets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    external_market_id VARCHAR(100),
    market_type VARCHAR(100) NOT NULL, -- 1X2, OVER_UNDER, BOTH_TEAMS_SCORE, HANDICAP
    name VARCHAR(255) NOT NULL,        -- Match Winner (1X2), Total Goals Over/Under 2.5
    parameter DOUBLE PRECISION DEFAULT NULL, -- 2.5 for Over/Under 2.5, -1.5 for Handicap
    status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE', -- ACTIVE, SUSPENDED, SETTLED
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_event_market UNIQUE (event_id, market_type, parameter)
);

CREATE INDEX IF NOT EXISTS idx_markets_event_id ON markets(event_id);
CREATE INDEX IF NOT EXISTS idx_markets_type ON markets(market_type);

