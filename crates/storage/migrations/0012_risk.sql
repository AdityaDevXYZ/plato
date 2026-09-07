CREATE TABLE risk_reports (
    id UUID PRIMARY KEY,
    plan_id UUID NOT NULL,
    data JSONB NOT NULL
);
