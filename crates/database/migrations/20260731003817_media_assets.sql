CREATE TABLE assets (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT INTO assets (id, name)
VALUES
    (1, 'poster'),
    (2, 'backdrop'),
    (3, 'logo'),
    (4, 'banner'),
    (5, 'thumbnail'),
    (6, 'icon'),
    (7, 'trailer'),
    (8, 'screenshot');

CREATE TABLE media_assets (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    asset_id INTEGER NOT NULL,
    path TEXT NOT NULL,

    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (asset_id) REFERENCES assets(id)
);

CREATE INDEX idx_media_assets_media_id
ON media_assets(media_id);

CREATE INDEX idx_media_assets_asset_type_id
ON media_assets(asset_id);
