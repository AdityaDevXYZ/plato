CREATE TABLE deployment_plans (
    id UUID PRIMARY KEY,
    mission_id UUID NOT NULL,
    data JSONB NOT NULL
);
