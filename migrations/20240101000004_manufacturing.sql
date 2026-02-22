CREATE TABLE work_centers (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    capacity INTEGER NOT NULL DEFAULT 0,
    cost_per_hour INTEGER NOT NULL DEFAULT 0,
    cost_currency TEXT DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE bills_of_material (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id),
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(product_id, version)
);

CREATE TABLE bom_components (
    id TEXT PRIMARY KEY,
    bom_id TEXT NOT NULL REFERENCES bills_of_material(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL,
    unit TEXT NOT NULL,
    scrap_percent REAL NOT NULL DEFAULT 0
);

CREATE TABLE bom_operations (
    id TEXT PRIMARY KEY,
    bom_id TEXT NOT NULL REFERENCES bills_of_material(id) ON DELETE CASCADE,
    sequence INTEGER NOT NULL,
    name TEXT NOT NULL,
    work_center_id TEXT NOT NULL REFERENCES work_centers(id),
    setup_time INTEGER NOT NULL DEFAULT 0,
    run_time INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE routings (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    product_id TEXT NOT NULL REFERENCES products(id),
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE routing_operations (
    id TEXT PRIMARY KEY,
    routing_id TEXT NOT NULL REFERENCES routings(id) ON DELETE CASCADE,
    sequence INTEGER NOT NULL,
    work_center_id TEXT NOT NULL REFERENCES work_centers(id),
    operation TEXT NOT NULL,
    setup_time INTEGER NOT NULL DEFAULT 0,
    run_time INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE work_orders (
    id TEXT PRIMARY KEY,
    order_number TEXT NOT NULL UNIQUE,
    product_id TEXT NOT NULL REFERENCES products(id),
    bom_id TEXT NOT NULL REFERENCES bills_of_material(id),
    quantity INTEGER NOT NULL,
    planned_start TEXT NOT NULL,
    planned_end TEXT NOT NULL,
    actual_start TEXT,
    actual_end TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE production_orders (
    id TEXT PRIMARY KEY,
    order_number TEXT NOT NULL UNIQUE,
    planned_quantity INTEGER NOT NULL,
    produced_quantity INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE production_order_work_orders (
    production_order_id TEXT NOT NULL REFERENCES production_orders(id),
    work_order_id TEXT NOT NULL REFERENCES work_orders(id),
    PRIMARY KEY (production_order_id, work_order_id)
);
