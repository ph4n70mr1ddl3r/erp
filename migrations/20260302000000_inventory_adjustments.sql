CREATE TABLE inventory_adjustments (
    id TEXT PRIMARY KEY,
    adjustment_number TEXT NOT NULL UNIQUE,
    warehouse_id TEXT NOT NULL,
    adjustment_type TEXT NOT NULL,
    reason TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    total_value_change INTEGER NOT NULL DEFAULT 0,
    approved_by TEXT,
    approved_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT
);

CREATE TABLE inventory_adjustment_lines (
    id TEXT PRIMARY KEY,
    adjustment_id TEXT NOT NULL REFERENCES inventory_adjustments(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL,
    location_id TEXT NOT NULL,
    system_quantity INTEGER NOT NULL,
    counted_quantity INTEGER NOT NULL,
    adjustment_quantity INTEGER NOT NULL,
    unit_cost INTEGER NOT NULL DEFAULT 0,
    total_value_change INTEGER NOT NULL DEFAULT 0,
    lot_number TEXT,
    serial_number TEXT,
    reason_code TEXT,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_inventory_adjustments_warehouse ON inventory_adjustments(warehouse_id);
CREATE INDEX idx_inventory_adjustments_status ON inventory_adjustments(status);
CREATE INDEX idx_inventory_adjustment_lines_adjustment ON inventory_adjustment_lines(adjustment_id);
CREATE INDEX idx_inventory_adjustment_lines_product ON inventory_adjustment_lines(product_id);
