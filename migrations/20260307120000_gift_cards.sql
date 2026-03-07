CREATE TABLE gift_cards (
    id TEXT PRIMARY KEY,
    card_number TEXT NOT NULL UNIQUE,
    pin TEXT,
    barcode TEXT,
    gift_card_type TEXT NOT NULL DEFAULT 'Digital',
    initial_balance INTEGER NOT NULL,
    current_balance INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    customer_id TEXT,
    order_id TEXT,
    purchased_by TEXT,
    recipient_email TEXT,
    recipient_name TEXT,
    message TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    issued_date TEXT NOT NULL,
    expiry_date TEXT,
    last_used_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE gift_card_transactions (
    id TEXT PRIMARY KEY,
    transaction_number TEXT NOT NULL UNIQUE,
    gift_card_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    balance_before INTEGER NOT NULL,
    balance_after INTEGER NOT NULL,
    order_id TEXT,
    reference TEXT,
    notes TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (gift_card_id) REFERENCES gift_cards(id)
);

CREATE TABLE gift_card_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    initial_amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    is_reloadable INTEGER NOT NULL DEFAULT 1,
    validity_months INTEGER NOT NULL DEFAULT 12,
    design_url TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_gift_cards_card_number ON gift_cards(card_number);
CREATE INDEX idx_gift_cards_customer_id ON gift_cards(customer_id);
CREATE INDEX idx_gift_cards_status ON gift_cards(status);
CREATE INDEX idx_gift_card_transactions_gift_card_id ON gift_card_transactions(gift_card_id);
