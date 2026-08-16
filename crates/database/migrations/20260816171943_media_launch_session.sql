-- Add migration script here

CREATE TABLE media_launch_sessions (
    id TEXT PRIMARY KEY NOT NULL,
    launch_id TEXT NOT NULL,
    duration_seconds TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    started_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (launch_id) REFERENCES media_launches(id) ON DELETE CASCADE
);
