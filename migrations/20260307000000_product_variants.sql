CREATE TABLE IF NOT EXISTS product_attributes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    attribute_type TEXT NOT NULL DEFAULT 'Select',
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS product_attribute_values (
    id TEXT PRIMARY KEY,
    attribute_id TEXT NOT NULL,
    value TEXT NOT NULL,
    display_value TEXT NOT NULL,
    color_code TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (attribute_id) REFERENCES product_attributes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS product_variants (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    sku TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    price_adjustment INTEGER NOT NULL DEFAULT 0,
    cost_adjustment INTEGER NOT NULL DEFAULT 0,
    barcode TEXT,
    weight_kg REAL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS product_variant_attribute_values (
    variant_id TEXT NOT NULL,
    attribute_id TEXT NOT NULL,
    attribute_name TEXT NOT NULL,
    value_id TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (variant_id, attribute_id),
    FOREIGN KEY (variant_id) REFERENCES product_variants(id) ON DELETE CASCADE,
    FOREIGN KEY (attribute_id) REFERENCES product_attributes(id),
    FOREIGN KEY (value_id) REFERENCES product_attribute_values(id)
);

CREATE INDEX IF NOT EXISTS idx_product_attributes_name ON product_attributes(name);
CREATE INDEX IF NOT EXISTS idx_product_attribute_values_attribute ON product_attribute_values(attribute_id);
CREATE INDEX IF NOT EXISTS idx_product_variants_product ON product_variants(product_id);
CREATE INDEX IF NOT EXISTS idx_product_variants_sku ON product_variants(sku);
CREATE INDEX IF NOT EXISTS idx_product_variant_attr_values_variant ON product_variant_attribute_values(variant_id);
