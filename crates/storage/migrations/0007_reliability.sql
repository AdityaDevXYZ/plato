
ALTER TABLE releases ADD COLUMN IF NOT EXISTS version_num BIGINT NOT NULL DEFAULT 1;

CREATE TABLE IF NOT EXISTS outbox_messages (
    id UUID PRIMARY KEY,
    event_type VARCHAR NOT NULL,
    payload JSONB NOT NULL,
    status VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retry_count INT NOT NULL DEFAULT 0,
    last_error TEXT
);

CREATE TABLE IF NOT EXISTS dlq_messages (
    id UUID PRIMARY KEY,
    payload JSONB NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS idempotency_keys (
    key VARCHAR PRIMARY KEY,
    response JSONB,
    created_at TIMESTAMPTZ NOT NULL
);
