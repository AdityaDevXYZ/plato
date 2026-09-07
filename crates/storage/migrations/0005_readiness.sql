CREATE TABLE readiness_reports (
    release_id UUID PRIMARY KEY REFERENCES releases(id),
    data JSONB NOT NULL
);
