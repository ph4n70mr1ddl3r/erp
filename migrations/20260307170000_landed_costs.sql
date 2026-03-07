-- Landed Cost Tables
CREATE TABLE IF NOT EXISTS landed_cost_categories (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    allocation_method TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS landed_cost_vouchers (
    id TEXT PRIMARY KEY,
    voucher_number TEXT NOT NULL UNIQUE,
    voucher_date TEXT NOT NULL,
    reference_type TEXT NOT NULL,
    reference_id TEXT NOT NULL,
    total_landed_cost INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS landed_cost_lines (
    id TEXT PRIMARY KEY,
    voucher_id TEXT NOT NULL REFERENCES landed_cost_vouchers(id) ON DELETE CASCADE,
    category_id TEXT NOT NULL REFERENCES landed_cost_categories(id),
    description TEXT NOT NULL,
    amount INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS landed_cost_allocations (
    id TEXT PRIMARY KEY,
    voucher_id TEXT NOT NULL REFERENCES landed_cost_vouchers(id) ON DELETE CASCADE,
    item_id TEXT NOT NULL,
    allocated_amount INTEGER NOT NULL,
    allocation_factor REAL NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_landed_cost_vouchers_ref ON landed_cost_vouchers(reference_id);
CREATE INDEX IF NOT EXISTS idx_landed_cost_allocations_voucher ON landed_cost_allocations(voucher_id);
CREATE INDEX IF NOT EXISTS idx_landed_cost_allocations_item ON landed_cost_allocations(item_id);
