CREATE TABLE IF NOT EXISTS payment_terms (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    due_days INTEGER NOT NULL DEFAULT 30,
    discount_days INTEGER,
    discount_percent REAL,
    is_default INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_payment_terms_code ON payment_terms(code);
CREATE INDEX IF NOT EXISTS idx_payment_terms_is_default ON payment_terms(is_default);
CREATE INDEX IF NOT EXISTS idx_payment_terms_status ON payment_terms(status);

INSERT INTO payment_terms (id, code, name, description, due_days, discount_days, discount_percent, is_default, status, created_at, updated_at)
VALUES 
    ('00000000-0000-0000-0000-000000000001', 'NET30', 'Net 30', 'Payment due within 30 days', 30, NULL, NULL, 1, 'Active', datetime('now'), datetime('now')),
    ('00000000-0000-0000-0000-000000000002', 'NET60', 'Net 60', 'Payment due within 60 days', 60, NULL, NULL, 0, 'Active', datetime('now'), datetime('now')),
    ('00000000-0000-0000-0000-000000000003', 'NET90', 'Net 90', 'Payment due within 90 days', 90, NULL, NULL, 0, 'Active', datetime('now'), datetime('now')),
    ('00000000-0000-0000-0000-000000000004', '2_10_NET30', '2/10 Net 30', '2% discount if paid within 10 days, otherwise due in 30 days', 30, 10, 2.0, 0, 'Active', datetime('now'), datetime('now')),
    ('00000000-0000-0000-0000-000000000005', 'COD', 'Cash on Delivery', 'Payment required upon delivery', 0, NULL, NULL, 0, 'Active', datetime('now'), datetime('now')),
    ('00000000-0000-0000-0000-000000000006', 'PREPAID', 'Prepaid', 'Payment required before shipment', 0, NULL, NULL, 0, 'Active', datetime('now'), datetime('now')),
    ('00000000-0000-0000-0000-000000000007', 'EOM', 'End of Month', 'Payment due at the end of the month following invoice', 30, NULL, NULL, 0, 'Active', datetime('now'), datetime('now')),
    ('00000000-0000-0000-0000-000000000008', '1_10_NET30', '1/10 Net 30', '1% discount if paid within 10 days, otherwise due in 30 days', 30, 10, 1.0, 0, 'Active', datetime('now'), datetime('now'));
