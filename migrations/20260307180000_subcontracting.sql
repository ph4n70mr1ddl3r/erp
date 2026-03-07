-- Subcontracting Tables
CREATE TABLE IF NOT EXISTS subcontract_orders (
    id TEXT PRIMARY KEY,
    order_number TEXT NOT NULL UNIQUE,
    vendor_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    service_cost_amount INTEGER NOT NULL,
    service_cost_currency TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    warehouse_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS subcontract_components (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL REFERENCES subcontract_orders(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    sent_quantity INTEGER NOT NULL DEFAULT 0,
    consumed_quantity INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_subcontract_orders_vendor ON subcontract_orders(vendor_id);
CREATE INDEX IF NOT EXISTS idx_subcontract_orders_product ON subcontract_orders(product_id);
CREATE INDEX IF NOT EXISTS idx_subcontract_components_order ON subcontract_components(order_id);
