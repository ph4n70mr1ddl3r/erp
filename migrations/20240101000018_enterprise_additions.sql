CREATE TABLE IF NOT EXISTS return_orders (
    id TEXT PRIMARY KEY,
    return_number TEXT NOT NULL UNIQUE,
    return_type TEXT NOT NULL DEFAULT 'CustomerReturn',
    customer_id TEXT,
    vendor_id TEXT,
    original_order_id TEXT,
    original_invoice_id TEXT,
    request_date TEXT NOT NULL,
    received_date TEXT,
    processed_date TEXT,
    reason TEXT NOT NULL DEFAULT 'Other',
    notes TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    total_credit INTEGER NOT NULL DEFAULT 0,
    warehouse_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS return_lines (
    id TEXT PRIMARY KEY,
    return_order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    description TEXT NOT NULL,
    quantity_requested INTEGER NOT NULL,
    quantity_received INTEGER NOT NULL DEFAULT 0,
    quantity_approved INTEGER NOT NULL DEFAULT 0,
    unit_price INTEGER NOT NULL,
    reason TEXT NOT NULL DEFAULT 'Other',
    disposition TEXT NOT NULL DEFAULT 'Restock',
    condition_type TEXT NOT NULL DEFAULT 'New',
    inspection_notes TEXT,
    credit_amount INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (return_order_id) REFERENCES return_orders(id)
);

CREATE TABLE IF NOT EXISTS return_inspections (
    id TEXT PRIMARY KEY,
    return_order_id TEXT NOT NULL,
    return_line_id TEXT NOT NULL,
    inspector_id TEXT,
    inspection_date TEXT NOT NULL,
    condition TEXT NOT NULL DEFAULT 'New',
    passes_quality INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    disposition TEXT NOT NULL DEFAULT 'Restock',
    created_at TEXT NOT NULL,
    FOREIGN KEY (return_order_id) REFERENCES return_orders(id),
    FOREIGN KEY (return_line_id) REFERENCES return_lines(id)
);

CREATE TABLE IF NOT EXISTS credit_memos (
    id TEXT PRIMARY KEY,
    memo_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    return_order_id TEXT,
    invoice_id TEXT,
    memo_date TEXT NOT NULL,
    subtotal INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Draft',
    applied_amount INTEGER NOT NULL DEFAULT 0,
    reason TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS credit_memo_lines (
    id TEXT PRIMARY KEY,
    credit_memo_id TEXT NOT NULL,
    product_id TEXT,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    line_total INTEGER NOT NULL,
    FOREIGN KEY (credit_memo_id) REFERENCES credit_memos(id)
);

CREATE TABLE IF NOT EXISTS refunds (
    id TEXT PRIMARY KEY,
    refund_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    credit_memo_id TEXT,
    return_order_id TEXT,
    refund_date TEXT NOT NULL,
    amount INTEGER NOT NULL,
    method TEXT NOT NULL DEFAULT 'OriginalPayment',
    reference TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    processed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS return_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    return_window_days INTEGER NOT NULL DEFAULT 30,
    requires_receipt INTEGER NOT NULL DEFAULT 1,
    requires_original_packaging INTEGER NOT NULL DEFAULT 0,
    restocking_fee_percent REAL NOT NULL DEFAULT 0,
    allows_exchange INTEGER NOT NULL DEFAULT 1,
    allows_refund INTEGER NOT NULL DEFAULT 1,
    allows_store_credit INTEGER NOT NULL DEFAULT 1,
    excluded_categories TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS dunning_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS dunning_level_configs (
    id TEXT PRIMARY KEY,
    policy_id TEXT NOT NULL,
    level TEXT NOT NULL DEFAULT 'Reminder',
    days_overdue INTEGER NOT NULL DEFAULT 0,
    fee_percent REAL NOT NULL DEFAULT 0,
    fee_fixed INTEGER NOT NULL DEFAULT 0,
    template_id TEXT,
    stop_services INTEGER NOT NULL DEFAULT 0,
    send_email INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (policy_id) REFERENCES dunning_policies(id)
);

CREATE TABLE IF NOT EXISTS dunning_runs (
    id TEXT PRIMARY KEY,
    run_number TEXT NOT NULL UNIQUE,
    policy_id TEXT NOT NULL,
    run_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    customers_processed INTEGER NOT NULL DEFAULT 0,
    total_amount INTEGER NOT NULL DEFAULT 0,
    total_fees INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (policy_id) REFERENCES dunning_policies(id)
);

CREATE TABLE IF NOT EXISTS dunning_letters (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    level TEXT NOT NULL DEFAULT 'Reminder',
    letter_date TEXT NOT NULL,
    invoice_amount INTEGER NOT NULL DEFAULT 0,
    fee_amount INTEGER NOT NULL DEFAULT 0,
    total_amount INTEGER NOT NULL DEFAULT 0,
    sent_at TEXT,
    acknowledged_at TEXT,
    status TEXT NOT NULL DEFAULT 'Generated',
    created_at TEXT NOT NULL,
    FOREIGN KEY (run_id) REFERENCES dunning_runs(id)
);

CREATE TABLE IF NOT EXISTS dunning_letter_invoices (
    letter_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL,
    PRIMARY KEY (letter_id, invoice_id),
    FOREIGN KEY (letter_id) REFERENCES dunning_letters(id)
);

CREATE TABLE IF NOT EXISTS collection_cases (
    id TEXT PRIMARY KEY,
    case_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    dunning_letter_id TEXT,
    assigned_to TEXT,
    open_date TEXT NOT NULL,
    close_date TEXT,
    total_amount INTEGER NOT NULL DEFAULT 0,
    collected_amount INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Open',
    priority TEXT NOT NULL DEFAULT 'Medium',
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (dunning_letter_id) REFERENCES dunning_letters(id)
);

CREATE TABLE IF NOT EXISTS collection_activities (
    id TEXT PRIMARY KEY,
    case_id TEXT NOT NULL,
    activity_type TEXT NOT NULL DEFAULT 'Note',
    description TEXT NOT NULL,
    performed_by TEXT,
    performed_at TEXT NOT NULL,
    result TEXT,
    next_action TEXT,
    next_action_date TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (case_id) REFERENCES collection_cases(id)
);

CREATE TABLE IF NOT EXISTS accounting_periods (
    id TEXT PRIMARY KEY,
    fiscal_year_id TEXT NOT NULL,
    period_number INTEGER NOT NULL,
    name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    lock_type TEXT NOT NULL DEFAULT 'Open',
    locked_at TEXT,
    locked_by TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (fiscal_year_id) REFERENCES fiscal_years(id)
);

CREATE TABLE IF NOT EXISTS period_close_checklists (
    id TEXT PRIMARY KEY,
    period_id TEXT NOT NULL,
    task_name TEXT NOT NULL,
    description TEXT,
    task_order INTEGER NOT NULL DEFAULT 0,
    is_required INTEGER NOT NULL DEFAULT 1,
    completed INTEGER NOT NULL DEFAULT 0,
    completed_at TEXT,
    completed_by TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (period_id) REFERENCES accounting_periods(id)
);

CREATE TABLE IF NOT EXISTS recurring_journals (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    frequency TEXT NOT NULL DEFAULT 'Monthly',
    interval_value INTEGER NOT NULL DEFAULT 1,
    day_of_month INTEGER,
    day_of_week INTEGER,
    start_date TEXT NOT NULL,
    end_date TEXT,
    next_run_date TEXT,
    last_run_date TEXT,
    auto_post INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS recurring_journal_lines (
    id TEXT PRIMARY KEY,
    recurring_journal_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    debit INTEGER NOT NULL DEFAULT 0,
    credit INTEGER NOT NULL DEFAULT 0,
    description TEXT,
    FOREIGN KEY (recurring_journal_id) REFERENCES recurring_journals(id)
);

CREATE TABLE IF NOT EXISTS recurring_journal_runs (
    id TEXT PRIMARY KEY,
    recurring_journal_id TEXT NOT NULL,
    run_date TEXT NOT NULL,
    journal_entry_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (recurring_journal_id) REFERENCES recurring_journals(id),
    FOREIGN KEY (journal_entry_id) REFERENCES journal_entries(id)
);

CREATE INDEX IF NOT EXISTS idx_return_orders_customer ON return_orders(customer_id);
CREATE INDEX IF NOT EXISTS idx_return_orders_status ON return_orders(status);
CREATE INDEX IF NOT EXISTS idx_credit_memos_customer ON credit_memos(customer_id);
CREATE INDEX IF NOT EXISTS idx_dunning_letters_customer ON dunning_letters(customer_id);
CREATE INDEX IF NOT EXISTS idx_collection_cases_customer ON collection_cases(customer_id);
CREATE INDEX IF NOT EXISTS idx_accounting_periods_fy ON accounting_periods(fiscal_year_id);
CREATE INDEX IF NOT EXISTS idx_recurring_journals_next_run ON recurring_journals(next_run_date);
