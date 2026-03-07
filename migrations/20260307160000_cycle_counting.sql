CREATE TABLE IF NOT EXISTS cycle_counts (
    id TEXT PRIMARY KEY,
    count_number TEXT NOT NULL UNIQUE,
    warehouse_id TEXT NOT NULL REFERENCES warehouses(id),
    name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    planned_date TEXT NOT NULL,
    completed_at TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cycle_count_lines (
    id TEXT PRIMARY KEY,
    cycle_count_id TEXT NOT NULL REFERENCES cycle_counts(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    location_id TEXT NOT NULL REFERENCES stock_locations(id),
    expected_quantity INTEGER NOT NULL,
    actual_quantity INTEGER,
    adjustment_qty INTEGER,
    status TEXT NOT NULL DEFAULT 'Pending',
    notes TEXT,
    UNIQUE(cycle_count_id, product_id, location_id)
);

CREATE INDEX IF NOT EXISTS idx_cycle_counts_warehouse ON cycle_counts(warehouse_id);
CREATE INDEX IF NOT EXISTS idx_cycle_count_lines_count ON cycle_count_lines(cycle_count_id);
