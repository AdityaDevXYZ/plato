CREATE TABLE satellite_orbits (
    id UUID PRIMARY KEY,
    data JSONB NOT NULL
);

CREATE TABLE orbital_predictions (
    id UUID NOT NULL,
    target_time TIMESTAMPTZ NOT NULL,
    data JSONB NOT NULL
);

CREATE TABLE comm_windows (
    sat_id UUID NOT NULL,
    ground_id UUID NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    data JSONB NOT NULL
);
