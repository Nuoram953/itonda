-- Add migration script for media reviews and session thoughts
CREATE TABLE IF NOT EXISTS media_reviews (
    media_id TEXT PRIMARY KEY NOT NULL,
    verdict TEXT NOT NULL CHECK (verdict IN ('masterpiece', 'recommended', 'neutral', 'do_not_recommend')),
    summary TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS media_thoughts (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'general',
    playtime_minutes INTEGER,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_media_thoughts_media_id ON media_thoughts(media_id);
CREATE INDEX IF NOT EXISTS idx_media_thoughts_category ON media_thoughts(media_id, category);
CREATE INDEX IF NOT EXISTS idx_media_thoughts_created_at ON media_thoughts(media_id, created_at DESC);
