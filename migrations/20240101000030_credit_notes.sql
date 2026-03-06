CREATE TABLE credit_notes (
    id TEXT PRIMARY KEY,
    credit_note_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL REFERENCES customers(id),
    invoice_id TEXT REFERENCES invoices(id),
    credit_note_date TEXT NOT NULL,
    subtotal INTEGER NOT NULL DEFAULT 0,
    subtotal_currency TEXT DEFAULT 'USD',
    tax_amount INTEGER NOT NULL DEFAULT 0,
    tax_currency TEXT DEFAULT 'USD',
    total INTEGER NOT NULL DEFAULT 0,
    total_currency TEXT DEFAULT 'USD',
    reason TEXT NOT NULL DEFAULT '"Return"',
    notes TEXT,
    status TEXT NOT NULL DEFAULT '"Draft"',
    applied_amount INTEGER NOT NULL DEFAULT 0,
    applied_currency TEXT DEFAULT 'USD',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE credit_note_lines (
    id TEXT PRIMARY KEY,
    credit_note_id TEXT NOT NULL REFERENCES credit_notes(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id),
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    unit_price_currency TEXT DEFAULT 'USD',
    line_total INTEGER NOT NULL,
    line_total_currency TEXT DEFAULT 'USD'
);

CREATE TABLE credit_note_applications (
    id TEXT PRIMARY KEY,
    credit_note_id TEXT NOT NULL REFERENCES credit_notes(id) ON DELETE CASCADE,
    invoice_id TEXT NOT NULL REFERENCES invoices(id),
    amount INTEGER NOT NULL,
    currency TEXT DEFAULT 'USD',
    applied_at TEXT NOT NULL
);
