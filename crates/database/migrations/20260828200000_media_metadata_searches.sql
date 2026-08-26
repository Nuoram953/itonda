-- Add migration script here
CREATE TABLE media_metadata_searches (
    media_id TEXT PRIMARY KEY NOT NULL,
    searched_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_media_metadata_searches_media_id
ON media_metadata_searches(media_id);
