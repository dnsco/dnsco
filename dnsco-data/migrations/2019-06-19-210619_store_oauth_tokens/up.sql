-- Your SQL goes here

CREATE TABLE oauth_tokens
(
    id                SERIAL PRIMARY KEY,
    token             VARCHAR NOT NULL,
    refresh           VARCHAR NOT NULL,
    remote_athlete_id INT     NOT NULL UNIQUE
);
