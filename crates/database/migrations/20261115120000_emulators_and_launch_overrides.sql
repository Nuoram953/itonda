-- Create emulators table and extend media_launches for emulator-backed launch configurations
CREATE TABLE emulators (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    program TEXT NOT NULL,
    arguments TEXT NOT NULL DEFAULT '[]',
    working_directory TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE media_launches ADD COLUMN emulator_id TEXT REFERENCES emulators(id) ON DELETE CASCADE;
ALTER TABLE media_launches ADD COLUMN game_location TEXT;
ALTER TABLE media_launches ADD COLUMN argument_mode TEXT NOT NULL DEFAULT 'append';
