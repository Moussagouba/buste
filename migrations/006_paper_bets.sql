-- 006_paper_bets.sql: Virtual paper betting orders and tracking
CREATE TABLE IF NOT EXISTS paper_bets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    prediction_id UUID REFERENCES predictions(id) ON DELETE SET NULL,
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    market_id UUID NOT NULL REFERENCES markets(id) ON DELETE CASCADE,
    outcome_name VARCHAR(100) NOT NULL,
    odds_placed DOUBLE PRECISION NOT NULL,
    stake_amount DOUBLE PRECISION NOT NULL,
    potential_return DOUBLE PRECISION NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING', -- PENDING, WON, LOST, VOID, CANCELLED
    profit_loss DOUBLE PRECISION DEFAULT NULL,
    closing_odds DOUBLE PRECISION DEFAULT NULL,    -- To measure CLV (Closing Line Value)
    closing_line_value DOUBLE PRECISION DEFAULT NULL, -- (odds_placed / closing_odds) - 1.0
    strategy_tag VARCHAR(100) DEFAULT 'kelly_value_standard',
    placed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    settled_at TIMESTAMPTZ DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_paper_bets_status ON paper_bets(status);
CREATE INDEX IF NOT EXISTS idx_paper_bets_placed_at ON paper_bets(placed_at);
CREATE INDEX IF NOT EXISTS idx_paper_bets_event_id ON paper_bets(event_id);

