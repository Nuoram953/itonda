-- Add migration script here
--
CREATE TABLE media (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    media_type TEXT NOT NULL,
    year INTEGER
);
