CREATE TABLE assurance_reports (
    release_id UUID PRIMARY KEY REFERENCES releases(id),
    data JSONB NOT NULL
);
