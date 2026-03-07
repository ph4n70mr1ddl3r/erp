CREATE TABLE IF NOT EXISTS favorites (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    favorite_type TEXT NOT NULL,
    entity_id TEXT,
    entity_name TEXT NOT NULL,
    entity_code TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE INDEX IF NOT EXISTS idx_favorites_user_id ON favorites(user_id);
CREATE INDEX IF NOT EXISTS idx_favorites_user_type ON favorites(user_id, favorite_type);
CREATE INDEX IF NOT EXISTS idx_favorites_entity ON favorites(user_id, favorite_type, entity_id);
