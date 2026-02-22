CREATE TABLE IF NOT EXISTS quotations (
    id TEXT PRIMARY KEY,
    quote_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    quote_date TEXT NOT NULL,
    valid_until TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    subtotal INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    terms TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS quotation_lines (
    id TEXT PRIMARY KEY,
    quotation_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    description TEXT,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    discount INTEGER DEFAULT 0,
    line_total INTEGER NOT NULL,
    FOREIGN KEY (quotation_id) REFERENCES quotations(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_quotations_customer ON quotations(customer_id);
CREATE INDEX IF NOT EXISTS idx_quotations_status ON quotations(status);
