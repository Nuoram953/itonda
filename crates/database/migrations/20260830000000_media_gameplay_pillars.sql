-- Add migration script here
CREATE TABLE IF NOT EXISTS media_gameplay_pillars (
    id TEXT PRIMARY KEY,
    media_id TEXT NOT NULL,
    position INTEGER NOT NULL,
    pillar_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    icon TEXT NOT NULL,
    asset_id TEXT,
    source TEXT NOT NULL DEFAULT 'wikipedia',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (asset_id) REFERENCES media_assets(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_media_gameplay_pillars_media_id 
ON media_gameplay_pillars(media_id);
