-- Add migration script here
DROP TABLE media_game_installations;
DROP TABLE media_game_storefront;
DROP TABLE media_variants;

CREATE TABLE media_game_storefront (
    media_id TEXT NOT NULL,
    storefront_id TEXT NOT NULL,
    internal_id TEXT NOT NULL,
    PRIMARY KEY(media_id, storefront_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (storefront_id) REFERENCES storefronts(id) ON DELETE CASCADE
);

CREATE TABLE media_game_installations (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    path TEXT NOT NULL,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE
);


CREATE TABLE media_statuses (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    status_id TEXT NOT NULL,
    UNIQUE(media_id, status_id)
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (status_id) REFERENCES statuses(id)
);


CREATE TABLE media_relations (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    child_media_id TEXT NOT NULL,
    relation TEXT,
    UNIQUE(media_id, child_media_id, relation),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (child_media_id) REFERENCES media(id) ON DELETE CASCADE
);
