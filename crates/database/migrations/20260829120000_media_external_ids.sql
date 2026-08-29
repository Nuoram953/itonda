-- Add migration script here
CREATE TABLE IF NOT EXISTS media_external_ids (
    media_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    external_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (media_id, provider),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_media_external_ids_provider_external
ON media_external_ids(provider, external_id);

CREATE INDEX IF NOT EXISTS idx_media_external_ids_media_id
ON media_external_ids(media_id);

-- Backfill Steam external IDs from media_storefronts
INSERT OR IGNORE INTO media_external_ids (media_id, provider, external_id, created_at, updated_at)
SELECT media_id, 'steam', external_id, created_at, updated_at
FROM media_storefronts
WHERE storefront_id = '0';
