CREATE TABLE IF NOT EXISTS vendor_bills (
    id TEXT PRIMARY KEY,
    bill_number TEXT NOT NULL UNIQUE,
    vendor_invoice_number TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    purchase_order_id TEXT,
    bill_date TEXT NOT NULL,
    due_date TEXT NOT NULL,
    subtotal INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    amount_paid INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Draft',
    match_status TEXT NOT NULL DEFAULT 'Unmatched',
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS vendor_bill_lines (
    id TEXT PRIMARY KEY,
    bill_id TEXT NOT NULL,
    po_line_id TEXT,
    product_id TEXT,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    tax_rate REAL NOT NULL DEFAULT 0,
    line_total INTEGER NOT NULL DEFAULT 0,
    match_quantity INTEGER NOT NULL DEFAULT 0,
    match_status TEXT NOT NULL DEFAULT 'Unmatched',
    FOREIGN KEY (bill_id) REFERENCES vendor_bills(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS vendor_bill_payments (
    id TEXT PRIMARY KEY,
    bill_id TEXT NOT NULL,
    payment_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    applied_at TEXT NOT NULL,
    FOREIGN KEY (bill_id) REFERENCES vendor_bills(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_vendor_bills_vendor_id ON vendor_bills(vendor_id);
CREATE INDEX IF NOT EXISTS idx_vendor_bills_status ON vendor_bills(status);
CREATE INDEX IF NOT EXISTS idx_vendor_bills_bill_date ON vendor_bills(bill_date);
CREATE INDEX IF NOT EXISTS idx_vendor_bills_due_date ON vendor_bills(due_date);
CREATE INDEX IF NOT EXISTS idx_vendor_bills_po_id ON vendor_bills(purchase_order_id);
CREATE INDEX IF NOT EXISTS idx_vendor_bill_lines_bill_id ON vendor_bill_lines(bill_id);
CREATE INDEX IF NOT EXISTS idx_vendor_bill_payments_bill_id ON vendor_bill_payments(bill_id);
