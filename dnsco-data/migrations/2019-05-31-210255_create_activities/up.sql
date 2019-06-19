-- Your SQL goes here
CREATE TABLE activities
(
    id                SERIAL PRIMARY KEY,
    description       TEXT,
    distance          DOUBLE PRECISION,
    name              VARCHAR NOT NULL,
    remote_athlete_id INT     NOT NULL,
    remote_id         INT     NOT NULL UNIQUE
);

CREATE INDEX ON activities (remote_athlete_id);