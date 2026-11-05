-- 001_users.sql: User & Auth management
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'analyst',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default admin / analyst user
INSERT INTO users (id, username, email, password_hash, role)
VALUES ('00000000-0000-0000-0000-000000000001', 'trader', 'trader@melbet-analytics.local', 'argon2_dummy_hash', 'admin')
ON CONFLICT (username) DO NOTHING;

