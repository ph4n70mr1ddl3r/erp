CREATE TABLE IF NOT EXISTS customer_credit_profiles (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL UNIQUE,
    credit_limit INTEGER NOT NULL DEFAULT 0,
    credit_used INTEGER NOT NULL DEFAULT 0,
    available_credit INTEGER NOT NULL DEFAULT 0,
    outstanding_invoices INTEGER NOT NULL DEFAULT 0,
    pending_orders INTEGER NOT NULL DEFAULT 0,
    overdue_amount INTEGER NOT NULL DEFAULT 0,
    overdue_days_avg INTEGER NOT NULL DEFAULT 0,
    credit_score INTEGER,
    risk_level TEXT NOT NULL DEFAULT 'Low',
    payment_history_score REAL,
    last_credit_review TEXT,
    next_review_date TEXT,
    auto_hold_enabled INTEGER NOT NULL DEFAULT 1,
    hold_threshold_percent INTEGER NOT NULL DEFAULT 90,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_credit_profiles_customer ON customer_credit_profiles(customer_id);
CREATE INDEX IF NOT EXISTS idx_credit_profiles_risk ON customer_credit_profiles(risk_level);
CREATE INDEX IF NOT EXISTS idx_credit_profiles_status ON customer_credit_profiles(status);

CREATE TABLE IF NOT EXISTS credit_transactions (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    previous_credit_used INTEGER NOT NULL,
    new_credit_used INTEGER NOT NULL,
    reference_type TEXT,
    reference_id TEXT,
    reference_number TEXT,
    description TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES customer_credit_profiles(id),
    FOREIGN KEY (customer_id) REFERENCES customers(id)
);

CREATE INDEX IF NOT EXISTS idx_credit_transactions_customer ON credit_transactions(customer_id);
CREATE INDEX IF NOT EXISTS idx_credit_transactions_type ON credit_transactions(transaction_type);
CREATE INDEX IF NOT EXISTS idx_credit_transactions_date ON credit_transactions(created_at);

CREATE TABLE IF NOT EXISTS credit_holds (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    hold_type TEXT NOT NULL,
    reason TEXT NOT NULL,
    amount_over_limit INTEGER NOT NULL DEFAULT 0,
    related_order_id TEXT,
    related_invoice_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    placed_by TEXT,
    placed_at TEXT NOT NULL,
    released_by TEXT,
    released_at TEXT,
    override_reason TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES customer_credit_profiles(id),
    FOREIGN KEY (customer_id) REFERENCES customers(id)
);

CREATE INDEX IF NOT EXISTS idx_credit_holds_customer ON credit_holds(customer_id);
CREATE INDEX IF NOT EXISTS idx_credit_holds_status ON credit_holds(status);
CREATE INDEX IF NOT EXISTS idx_credit_holds_active ON credit_holds(customer_id, status);

CREATE TABLE IF NOT EXISTS credit_limit_changes (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    previous_limit INTEGER NOT NULL,
    new_limit INTEGER NOT NULL,
    change_reason TEXT NOT NULL,
    approved_by TEXT,
    approved_at TEXT,
    effective_date TEXT NOT NULL,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES customer_credit_profiles(id),
    FOREIGN KEY (customer_id) REFERENCES customers(id)
);

CREATE INDEX IF NOT EXISTS idx_credit_limit_changes_customer ON credit_limit_changes(customer_id);

CREATE TABLE IF NOT EXISTS credit_alerts (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    alert_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    message TEXT NOT NULL,
    threshold_value INTEGER NOT NULL,
    actual_value INTEGER NOT NULL,
    is_read INTEGER NOT NULL DEFAULT 0,
    acknowledged_by TEXT,
    acknowledged_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES customer_credit_profiles(id),
    FOREIGN KEY (customer_id) REFERENCES customers(id)
);

CREATE INDEX IF NOT EXISTS idx_credit_alerts_customer ON credit_alerts(customer_id);
CREATE INDEX IF NOT EXISTS idx_credit_alerts_unread ON credit_alerts(is_read);
CREATE INDEX IF NOT EXISTS idx_credit_alerts_severity ON credit_alerts(severity);
