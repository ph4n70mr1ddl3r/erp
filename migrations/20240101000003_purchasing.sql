CREATE TABLE vendors (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    fax TEXT,
    website TEXT,
    street TEXT,
    city TEXT,
    state TEXT,
    postal_code TEXT,
    country TEXT,
    payment_terms INTEGER NOT NULL DEFAULT 30,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE purchase_requisitions (
    id TEXT PRIMARY KEY,
    requisition_number TEXT NOT NULL UNIQUE,
    requested_by TEXT,
    request_date TEXT NOT NULL,
    required_date TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE requisition_lines (
    id TEXT PRIMARY KEY,
    requisition_id TEXT NOT NULL REFERENCES purchase_requisitions(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL
);

CREATE TABLE purchase_orders (
    id TEXT PRIMARY KEY,
    po_number TEXT NOT NULL UNIQUE,
    vendor_id TEXT NOT NULL REFERENCES vendors(id),
    order_date TEXT NOT NULL,
    expected_date TEXT,
    subtotal INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    currency TEXT DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE purchase_order_lines (
    id TEXT PRIMARY KEY,
    purchase_order_id TEXT NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    tax_rate REAL NOT NULL DEFAULT 0,
    line_total INTEGER NOT NULL
);

CREATE TABLE goods_receipts (
    id TEXT PRIMARY KEY,
    receipt_number TEXT NOT NULL UNIQUE,
    purchase_order_id TEXT NOT NULL REFERENCES purchase_orders(id),
    warehouse_id TEXT NOT NULL REFERENCES warehouses(id),
    receipt_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE goods_receipt_lines (
    id TEXT PRIMARY KEY,
    goods_receipt_id TEXT NOT NULL REFERENCES goods_receipts(id) ON DELETE CASCADE,
    po_line_id TEXT NOT NULL REFERENCES purchase_order_lines(id),
    product_id TEXT NOT NULL REFERENCES products(id),
    quantity_ordered INTEGER NOT NULL,
    quantity_received INTEGER NOT NULL
);
