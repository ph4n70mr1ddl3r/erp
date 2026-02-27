CREATE TABLE IF NOT EXISTS payment_gateways (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    gateway_type TEXT NOT NULL,
    api_key TEXT,
    api_secret TEXT,
    merchant_id TEXT,
    webhook_secret TEXT,
    is_live INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    supported_methods TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    payment_number TEXT NOT NULL UNIQUE,
    gateway_id TEXT,
    invoice_id TEXT,
    customer_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    payment_method TEXT NOT NULL,
    status TEXT NOT NULL,
    gateway_transaction_id TEXT,
    gateway_response TEXT,
    card_last_four TEXT,
    card_brand TEXT,
    bank_name TEXT,
    bank_account_last_four TEXT,
    check_number TEXT,
    refunded_amount INTEGER NOT NULL DEFAULT 0,
    refund_reason TEXT,
    processing_fee INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    paid_at TEXT,
    created_at TEXT NOT NULL,
    created_by TEXT
);

CREATE TABLE IF NOT EXISTS payment_allocations (
    id TEXT PRIMARY KEY,
    payment_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS refunds (
    id TEXT PRIMARY KEY,
    refund_number TEXT NOT NULL UNIQUE,
    payment_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    reason TEXT NOT NULL,
    status TEXT NOT NULL,
    gateway_refund_id TEXT,
    processed_at TEXT,
    created_at TEXT NOT NULL,
    created_by TEXT
);

CREATE TABLE IF NOT EXISTS customer_payment_methods (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    payment_method TEXT NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0,
    card_last_four TEXT,
    card_brand TEXT,
    card_expiry_month INTEGER,
    card_expiry_year INTEGER,
    card_holder_name TEXT,
    bank_name TEXT,
    bank_account_type TEXT,
    gateway_token TEXT,
    nickname TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS payment_batches (
    id TEXT PRIMARY KEY,
    batch_number TEXT NOT NULL UNIQUE,
    gateway_id TEXT NOT NULL,
    total_amount INTEGER NOT NULL,
    total_count INTEGER NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL,
    settled_at TEXT,
    settlement_reference TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS stripe_payment_intents (
    id TEXT PRIMARY KEY,
    stripe_intent_id TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    invoice_id TEXT,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL,
    client_secret TEXT,
    description TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS stripe_checkout_sessions (
    id TEXT PRIMARY KEY,
    stripe_session_id TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    invoice_id TEXT,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL,
    checkout_url TEXT,
    success_url TEXT NOT NULL,
    cancel_url TEXT NOT NULL,
    payment_intent_id TEXT,
    expires_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS stripe_webhook_events (
    id TEXT PRIMARY KEY,
    stripe_event_id TEXT NOT NULL UNIQUE,
    event_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    processed INTEGER NOT NULL DEFAULT 0,
    processed_at TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_payments_customer ON payments(customer_id);
CREATE INDEX IF NOT EXISTS idx_payments_invoice ON payments(invoice_id);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments(status);
CREATE INDEX IF NOT EXISTS idx_stripe_intents_customer ON stripe_payment_intents(customer_id);
CREATE INDEX IF NOT EXISTS idx_stripe_intents_status ON stripe_payment_intents(status);
CREATE INDEX IF NOT EXISTS idx_stripe_sessions_customer ON stripe_checkout_sessions(customer_id);
CREATE INDEX IF NOT EXISTS idx_stripe_webhook_type ON stripe_webhook_events(event_type);
CREATE INDEX IF NOT EXISTS idx_stripe_webhook_processed ON stripe_webhook_events(processed);
