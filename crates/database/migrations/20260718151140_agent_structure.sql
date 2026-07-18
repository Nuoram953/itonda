CREATE TABLE agents (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    hostname TEXT,
    platform TEXT,
    agent_version TEXT,
    last_seen_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);


CREATE TABLE agent_connections (
    id TEXT PRIMARY KEY NOT NULL,
    agent_id TEXT NOT NULL,
    connected_at INTEGER NOT NULL DEFAULT (unixepoch()),
    disconnected_at INTEGER,
    ip_address TEXT,

    FOREIGN KEY(agent_id) REFERENCES agents(id)
);
