-- Product Bundles / Kits Management
-- Enables grouping of products as sellable units with flexible pricing

CREATE TABLE IF NOT EXISTS product_bundles (
    id TEXT PRIMARY KEY,
    bundle_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    bundle_type TEXT NOT NULL DEFAULT 'SalesKit',
    pricing_method TEXT NOT NULL DEFAULT 'FixedPrice',
    list_price_cents INTEGER NOT NULL DEFAULT 0,
    list_price_currency TEXT NOT NULL DEFAULT 'USD',
    calculated_price_cents INTEGER NOT NULL DEFAULT 0,
    calculated_price_currency TEXT NOT NULL DEFAULT 'USD',
    discount_percent REAL,
    auto_explode INTEGER NOT NULL DEFAULT 0,
    track_inventory INTEGER NOT NULL DEFAULT 1,
    availability_date TEXT,
    expiry_date TEXT,
    max_quantity_per_order INTEGER,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_bundles_code ON product_bundles(bundle_code);
CREATE INDEX IF NOT EXISTS idx_bundles_status ON product_bundles(status);
CREATE INDEX IF NOT EXISTS idx_bundles_type ON product_bundles(bundle_type);

CREATE TABLE IF NOT EXISTS bundle_components (
    id TEXT PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_of_measure TEXT NOT NULL DEFAULT 'PCS',
    is_mandatory INTEGER NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    component_price_cents INTEGER NOT NULL DEFAULT 0,
    component_price_currency TEXT NOT NULL DEFAULT 'USD',
    discount_percent REAL,
    can_substitute INTEGER NOT NULL DEFAULT 0,
    substitute_group_id TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (bundle_id) REFERENCES product_bundles(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE INDEX IF NOT EXISTS idx_bundle_components_bundle ON bundle_components(bundle_id);
CREATE INDEX IF NOT EXISTS idx_bundle_components_product ON bundle_components(product_id);

CREATE TABLE IF NOT EXISTS bundle_substitute_groups (
    id TEXT PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    group_name TEXT NOT NULL,
    min_select INTEGER NOT NULL DEFAULT 1,
    max_select INTEGER NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (bundle_id) REFERENCES product_bundles(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS bundle_price_rules (
    id TEXT PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    rule_name TEXT NOT NULL,
    rule_type TEXT NOT NULL DEFAULT 'QuantityBreak',
    min_quantity INTEGER NOT NULL DEFAULT 1,
    max_quantity INTEGER,
    discount_percent REAL,
    fixed_price INTEGER,
    start_date TEXT,
    end_date TEXT,
    customer_group_id TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (bundle_id) REFERENCES product_bundles(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_bundle_price_rules_bundle ON bundle_price_rules(bundle_id);
CREATE INDEX IF NOT EXISTS idx_bundle_price_rules_type ON bundle_price_rules(rule_type);

CREATE TABLE IF NOT EXISTS bundle_inventory (
    id TEXT PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    available_quantity INTEGER NOT NULL DEFAULT 0,
    allocated_quantity INTEGER NOT NULL DEFAULT 0,
    backorder_quantity INTEGER NOT NULL DEFAULT 0,
    last_calculated TEXT NOT NULL,
    FOREIGN KEY (bundle_id) REFERENCES product_bundles(id) ON DELETE CASCADE,
    FOREIGN KEY (warehouse_id) REFERENCES warehouses(id),
    UNIQUE(bundle_id, warehouse_id)
);

CREATE TABLE IF NOT EXISTS bundle_usage (
    id TEXT PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    order_id TEXT,
    order_line_id TEXT,
    invoice_id TEXT,
    customer_id TEXT,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price INTEGER NOT NULL DEFAULT 0,
    total_price INTEGER NOT NULL DEFAULT 0,
    margin_amount INTEGER NOT NULL DEFAULT 0,
    margin_percent REAL NOT NULL DEFAULT 0,
    usage_date TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (bundle_id) REFERENCES product_bundles(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_bundle_usage_bundle ON bundle_usage(bundle_id);
CREATE INDEX IF NOT EXISTS idx_bundle_usage_date ON bundle_usage(usage_date);
CREATE INDEX IF NOT EXISTS idx_bundle_usage_customer ON bundle_usage(customer_id);

CREATE TABLE IF NOT EXISTS bundle_templates (
    id TEXT PRIMARY KEY,
    template_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    default_pricing_method TEXT NOT NULL DEFAULT 'FixedPrice',
    default_markup_percent REAL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS bundle_template_components (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    product_id TEXT,
    product_category_id TEXT,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_mandatory INTEGER NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (template_id) REFERENCES bundle_templates(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_template_components_template ON bundle_template_components(template_id);
