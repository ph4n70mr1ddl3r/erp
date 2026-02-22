CREATE TABLE product_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id TEXT REFERENCES product_categories(id),
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE products (
    id TEXT PRIMARY KEY,
    sku TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    product_type TEXT NOT NULL DEFAULT 'Goods',
    category_id TEXT REFERENCES product_categories(id),
    unit_of_measure TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE warehouses (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    address_street TEXT,
    address_city TEXT,
    address_state TEXT,
    address_postal_code TEXT,
    address_country TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE stock_locations (
    id TEXT PRIMARY KEY,
    warehouse_id TEXT NOT NULL REFERENCES warehouses(id),
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    location_type TEXT NOT NULL DEFAULT 'Storage',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(warehouse_id, code)
);

CREATE TABLE stock_levels (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id),
    location_id TEXT NOT NULL REFERENCES stock_locations(id),
    quantity INTEGER NOT NULL DEFAULT 0,
    reserved_quantity INTEGER NOT NULL DEFAULT 0,
    available_quantity INTEGER NOT NULL DEFAULT 0,
    UNIQUE(product_id, location_id)
);

CREATE TABLE stock_movements (
    id TEXT PRIMARY KEY,
    movement_number TEXT NOT NULL UNIQUE,
    movement_type TEXT NOT NULL,
    product_id TEXT NOT NULL REFERENCES products(id),
    from_location_id TEXT REFERENCES stock_locations(id),
    to_location_id TEXT REFERENCES stock_locations(id),
    quantity INTEGER NOT NULL,
    reference TEXT,
    movement_date TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE price_lists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE price_list_items (
    id TEXT PRIMARY KEY,
    price_list_id TEXT NOT NULL REFERENCES price_lists(id),
    product_id TEXT NOT NULL REFERENCES products(id),
    price INTEGER NOT NULL,
    min_quantity INTEGER NOT NULL DEFAULT 0,
    UNIQUE(price_list_id, product_id)
);
