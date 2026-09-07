CREATE TABLE twin_satellites (
    id UUID PRIMARY KEY,
    mission_id UUID NOT NULL,
    data JSONB NOT NULL
);
