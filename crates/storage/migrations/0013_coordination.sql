CREATE TABLE coordination_sessions (
    id UUID PRIMARY KEY,
    plan_id UUID NOT NULL,
    data JSONB NOT NULL
);
