-- Add toggleable options (e.g. Fullscreen -> -f) to emulators
ALTER TABLE emulators ADD COLUMN options TEXT NOT NULL DEFAULT '[]';
