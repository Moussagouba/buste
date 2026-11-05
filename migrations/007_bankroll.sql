-- 007_bankroll.sql: Virtual bankroll snapshots, balance and transactions
CREATE TABLE IF NOT EXISTS bankroll_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL DEFAULT 'Main Simulation Portfolio',
    currency VARCHAR(10) NOT NULL DEFAULT 'EUR',
    initial_balance DOUBLE PRECISION NOT NULL DEFAULT 1000.0,
    current_balance DOUBLE PRECISION NOT NULL DEFAULT 1000.0,
    peak_balance DOUBLE PRECISION NOT NULL DEFAULT 1000.0,
    total_staked DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    total_profit DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    max_drawdown_pct DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS bankroll_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES bankroll_accounts(id) ON DELETE CASCADE,
    paper_bet_id UUID REFERENCES paper_bets(id) ON DELETE SET NULL,
    transaction_type VARCHAR(50) NOT NULL, -- INITIAL, BET_PLACED, BET_WON, BET_LOST, BET_VOID, ADJUSTMENT
    amount DOUBLE PRECISION NOT NULL,      -- Negative for stake placed, positive for winnings/refund
    balance_after DOUBLE PRECISION NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS bankroll_snapshots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES bankroll_accounts(id) ON DELETE CASCADE,
    balance DOUBLE PRECISION NOT NULL,
    equity DOUBLE PRECISION NOT NULL,      -- balance + pending stakes
    total_staked DOUBLE PRECISION NOT NULL,
    net_profit DOUBLE PRECISION NOT NULL,
    roi_pct DOUBLE PRECISION NOT NULL,
    win_rate_pct DOUBLE PRECISION NOT NULL,
    drawdown_pct DOUBLE PRECISION NOT NULL,
    active_bets_count INT NOT NULL DEFAULT 0,
    settled_bets_count INT NOT NULL DEFAULT 0,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed initial account
INSERT INTO bankroll_accounts (id, name, currency, initial_balance, current_balance, peak_balance)
VALUES ('00000000-0000-0000-0000-000000000001', 'Default Paper Bankroll', 'EUR', 1000.0, 1000.0, 1000.0)
ON CONFLICT (id) DO NOTHING;

INSERT INTO bankroll_transactions (account_id, transaction_type, amount, balance_after, description)
VALUES ('00000000-0000-0000-0000-000000000001', 'INITIAL', 1000.0, 1000.0, 'Initial paper bankroll funding')
ON CONFLICT DO NOTHING;

