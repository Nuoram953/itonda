  CREATE TABLE IF NOT EXISTS media_storefronts (
      media_id TEXT NOT NULL,
      storefront_id TEXT NOT NULL,
      external_id TEXT NOT NULL,
      playtime_minutes INTEGER,
      last_played_at INTEGER,
      is_installed INTEGER NOT NULL DEFAULT 0,
      created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
      PRIMARY KEY (media_id, storefront_id, external_id),
      FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
      FOREIGN KEY (storefront_id) REFERENCES storefronts(id) ON DELETE CASCADE
  );

  CREATE INDEX IF NOT EXISTS idx_media_storefronts_storefront
  ON media_storefronts(storefront_id);

  CREATE INDEX IF NOT EXISTS idx_media_storefronts_installed
  ON media_storefronts(media_id, is_installed);

