-- Add migration script here
CREATE TABLE media_asset_searches (
    media_id TEXT NOT NULL,
    asset_id INTEGER NOT NULL,
    searched_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (media_id, asset_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (asset_id) REFERENCES assets(id) ON DELETE CASCADE
);

CREATE INDEX idx_media_asset_searches_media_id
ON media_asset_searches(media_id);

INSERT OR IGNORE INTO media_asset_searches (media_id, asset_id)
SELECT DISTINCT media_id, asset_id FROM media_assets;
