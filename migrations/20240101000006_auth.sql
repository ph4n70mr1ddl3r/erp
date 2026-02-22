CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    full_name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'User',
    status TEXT NOT NULL DEFAULT 'Active',
    last_login TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE roles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    permissions TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL
);

CREATE TABLE user_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL
);

INSERT INTO roles (id, name, description, permissions, created_at) VALUES 
('role-admin', 'Admin', 'Full system access', '["*"]', datetime('now')),
('role-finance', 'Finance', 'Finance module access', '["finance:*","sales:read"]', datetime('now')),
('role-warehouse', 'Warehouse', 'Inventory module access', '["inventory:*","purchasing:*"]', datetime('now')),
('role-sales', 'Sales', 'Sales module access', '["sales:*","inventory:read"]', datetime('now')),
('role-hr', 'HR', 'HR module access', '["hr:*"]', datetime('now')),
('role-user', 'User', 'Basic read access', '["*:read"]', datetime('now'));
