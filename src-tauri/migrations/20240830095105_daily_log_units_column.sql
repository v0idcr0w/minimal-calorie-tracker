-- Add migration script here
ALTER TABLE daily_logs ADD COLUMN units INTEGER NOT NULL DEFAULT 0; 