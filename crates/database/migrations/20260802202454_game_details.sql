-- Add migration script here

ALTER TABLE media_game_details
ADD COLUMN playtime_minutes INTEGER;

ALTER TABLE media_game_details
ADD COLUMN last_played_at INTEGER;
