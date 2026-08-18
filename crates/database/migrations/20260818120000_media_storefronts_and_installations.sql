-- Add migration script here
INSERT OR IGNORE INTO storefronts (id, name) VALUES ('0', 'Steam');

CREATE TABLE IF NOT EXISTS media_storefronts (
    media_id TEXT NOT NULL,
    storefront_id TEXT NOT NULL,
    external_id TEXT NOT NULL,
    playtime_minutes INTEGER,
    last_played_at INTEGER,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (media_id, storefront_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (storefront_id) REFERENCES storefronts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_media_storefronts_storefront
ON media_storefronts(storefront_id);

CREATE TABLE IF NOT EXISTS media_installations (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    storefront_id TEXT,
    external_id TEXT,
    path TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (media_id, agent_id, storefront_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE CASCADE,
    FOREIGN KEY (storefront_id) REFERENCES storefronts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_media_installations_media_agent
ON media_installations(media_id, agent_id);
