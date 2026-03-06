CREATE TABLE notes (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    note_type TEXT NOT NULL DEFAULT 'General',
    title TEXT,
    content TEXT NOT NULL,
    is_private INTEGER NOT NULL DEFAULT 0,
    is_pinned INTEGER NOT NULL DEFAULT 0,
    reminder_at TEXT,
    reminded_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE INDEX idx_notes_entity ON notes(entity_type, entity_id);
CREATE INDEX idx_notes_created_by ON notes(created_by);
CREATE INDEX idx_notes_reminder ON notes(reminder_at) WHERE reminder_at IS NOT NULL AND reminded_at IS NULL;
