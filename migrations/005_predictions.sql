-- 005_predictions.sql: Model probability and expected value predictions
CREATE TABLE IF NOT EXISTS predictions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    market_id UUID NOT NULL REFERENCES markets(id) ON DELETE CASCADE,
    outcome_name VARCHAR(100) NOT NULL,
    model_name VARCHAR(100) NOT NULL DEFAULT 'poisson_ensemble_v1',
    model_probability DOUBLE PRECISION NOT NULL,  -- Estimated true probability
    bookmaker_odds DOUBLE PRECISION NOT NULL,     -- Available market odds
    fair_odds DOUBLE PRECISION NOT NULL,          -- 1 / model_probability
    expected_value DOUBLE PRECISION NOT NULL,     -- (model_prob * odds) - 1.0
    edge DOUBLE PRECISION NOT NULL,               -- model_prob - implied_prob
    confidence_score DOUBLE PRECISION NOT NULL,   -- 0.0 to 1.0 based on data depth
    kelly_stake_pct DOUBLE PRECISION NOT NULL,    -- Recommended stake % from Kelly
    is_value_bet BOOLEAN NOT NULL DEFAULT FALSE,
    features_used JSONB DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_predictions_event_id ON predictions(event_id);
CREATE INDEX IF NOT EXISTS idx_predictions_value_bet ON predictions(is_value_bet);
CREATE INDEX IF NOT EXISTS idx_predictions_ev ON predictions(expected_value);

