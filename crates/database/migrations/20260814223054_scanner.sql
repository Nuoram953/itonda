-- Add migration script here

CREATE TABLE media_launches_new (
    id TEXT PRIMARY KEY NOT NULL,
    media_id TEXT NOT NULL,
    agent_id TEXT,
    name TEXT NOT NULL,
    launch_type TEXT NOT NULL,
    program TEXT NOT NULL,
    arguments TEXT NOT NULL,
    working_directory TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(media_id, name, launch_type, agent_id),

    FOREIGN KEY (media_id)
        REFERENCES media(id)
        ON DELETE CASCADE,
    FOREIGN KEY (agent_id)
        REFERENCES agents(id)
        ON DELETE CASCADE
);

INSERT INTO media_launches_new (id, media_id, agent_id, name, launch_type, program, arguments, working_directory, is_default, enabled, created_at, updated_at)
SELECT id, media_id, NULL, name, launch_type, program, arguments, working_directory, is_default, enabled, created_at, updated_at
FROM media_launches;

DROP TABLE media_launches;
ALTER TABLE media_launches_new RENAME TO media_launches;

CREATE INDEX idx_media_launches_agent ON media_launches(agent_id);
