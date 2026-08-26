-- Add migration script here
ALTER TABLE media ADD COLUMN description TEXT;
ALTER TABLE media ADD COLUMN summary TEXT;
ALTER TABLE media ADD COLUMN release_date INTEGER;

ALTER TABLE media_game_details ADD COLUMN series TEXT;

INSERT OR IGNORE INTO roles (id, name) VALUES ('developer', 'developer');
INSERT OR IGNORE INTO roles (id, name) VALUES ('publisher', 'publisher');

CREATE INDEX IF NOT EXISTS idx_media_genres_media_id ON media_genres(media_id);
CREATE INDEX IF NOT EXISTS idx_media_tags_media_id ON media_tags(media_id);
CREATE INDEX IF NOT EXISTS idx_media_companies_media_id ON media_companies(media_id);
