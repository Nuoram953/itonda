DROP TABLE media;

CREATE TABLE media (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    media_type TEXT NOT NULL
);

CREATE TABLE peoples (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE companies (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE roles (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE genres (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE tags (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE storefronts (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE statuses (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE media_variants (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    title TEXT,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE
);

CREATE TABLE media_statuses (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    variant_id TEXT,
    status_id TEXT NOT NULL,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (variant_id) REFERENCES media_variants(id) ON DELETE CASCADE,
    FOREIGN KEY (status_id) REFERENCES statuses(id)
);


CREATE TABLE media_game_details (
    media_id TEXT PRIMARY KEY NOT NULL,
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE
);


CREATE TABLE media_game_storefront (
    variant_id TEXT NOT NULL,
    storefront_id TEXT PRIMARY KEY NOT NULL,
    internal_id TEXT NOT NULL,
    FOREIGN KEY (variant_id) REFERENCES media_variants(id) ON DELETE CASCADE,
    FOREIGN KEY (storefront_id) REFERENCES storefronts(id) ON DELETE CASCADE
);

CREATE TABLE media_game_installations (
    id TEXT PRIMARY KEY NOT NULL,
    variant_id TEXT NOT NULL,
    path TEXT NOT NULL,
    FOREIGN KEY (variant_id) REFERENCES media_variants(id) ON DELETE CASCADE
);

CREATE TABLE media_peoples (
    media_id TEXT NOT NULL,
    person_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    PRIMARY KEY (media_id, person_id, role_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (person_id) REFERENCES peoples(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

CREATE TABLE media_companies (
    media_id TEXT NOT NULL,
    company_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    PRIMARY KEY (media_id, company_id, role_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

CREATE TABLE media_genres (
    media_id TEXT NOT NULL,
    genre_id TEXT NOT NULL,
    PRIMARY KEY(media_id, genre_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (genre_id) REFERENCES genres(id) ON DELETE CASCADE
);


CREATE TABLE media_tags (
    media_id TEXT NOT NULL,
    tags_id TEXT NOT NULL,
    PRIMARY KEY(media_id, tags_id),
    FOREIGN KEY (media_id) REFERENCES media(id) ON DELETE CASCADE,
    FOREIGN KEY (tags_id) REFERENCES tags(id) ON DELETE CASCADE
);
