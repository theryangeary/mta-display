-- Add migration script here
CREATE TABLE IF NOT EXISTS gallery_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message TEXT NOT NULL,
    train TEXT NOT NULL,
    submitter_name TEXT NOT NULL,
    submitted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    approved_at DATETIME,
    description TEXT
);

CREATE INDEX IF NOT EXISTS idx_approved_at ON gallery_entries (approved_at);
