CREATE TABLE satellite_resources (
    id UUID PRIMARY KEY,
    data JSONB NOT NULL
);
CREATE TABLE ground_station_resources (
    id UUID PRIMARY KEY,
    data JSONB NOT NULL
);
