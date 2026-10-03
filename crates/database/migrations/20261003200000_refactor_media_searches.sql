-- Migration: 20261003200000_refactor_media_searches.sql

-- 1. Refactor media_metadata_searches
CREATE TABLE media_metadata_searches_new (
    media_id TEXT NOT NULL,
    store_id TEXT NOT NULL,
    metadata_type TEXT NOT NULL,
    searched_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    idempotency_key TEXT GENERATED ALWAYS AS (media_id || ':' || store_id || ':' || metadata_type) STORED,
    PRIMARY KEY (media_id, store_id, metadata_type),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE
);

INSERT OR IGNORE INTO media_metadata_searches_new (media_id, store_id, metadata_type, searched_at)
SELECT media_id, 'igdb', 'general', searched_at FROM media_metadata_searches;

DROP TABLE media_metadata_searches;
ALTER TABLE media_metadata_searches_new RENAME TO media_metadata_searches;

CREATE INDEX IF NOT EXISTS idx_media_metadata_searches_media_id
ON media_metadata_searches(media_id);

CREATE INDEX IF NOT EXISTS idx_media_metadata_searches_idempotency
ON media_metadata_searches(idempotency_key);

-- 2. Refactor media_asset_searches -> media_assets_searches
CREATE TABLE media_assets_searches (
    media_id TEXT NOT NULL,
    store_id TEXT NOT NULL,
    asset_id INTEGER NOT NULL,
    searched_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    idempotency_key TEXT GENERATED ALWAYS AS (media_id || ':' || store_id || ':' || asset_id) STORED,
    PRIMARY KEY (media_id, store_id, asset_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE
);

INSERT OR IGNORE INTO media_assets_searches (media_id, store_id, asset_id, searched_at)
SELECT media_id, 'steamgriddb', asset_id, searched_at FROM media_asset_searches;

DROP TABLE media_asset_searches;

CREATE INDEX IF NOT EXISTS idx_media_assets_searches_media_id
ON media_assets_searches(media_id);

CREATE INDEX IF NOT EXISTS idx_media_assets_searches_idempotency
ON media_assets_searches(idempotency_key);

-- Compatibility view for legacy table name
CREATE VIEW IF NOT EXISTS media_asset_searches AS SELECT * FROM media_assets_searches;
