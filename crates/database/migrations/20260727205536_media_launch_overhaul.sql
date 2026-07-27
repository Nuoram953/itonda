-- Add migration script here
DROP TABLE media_game_installations;
DROP TABLE media_game_storefront;

CREATE TABLE media_launches (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    name TEXT NOT NULL,
    launch_type TEXT NOT NULL,
    program TEXT NOT NULL,
    arguments TEXT NOT NULL,
    working_directory TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(media_id, name, launch_type),

    FOREIGN KEY (media_id)
        REFERENCES media(id)
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_media_default_launch
ON media_launches(media_id)
WHERE is_default = 1;
