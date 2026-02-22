-- Multi-currency support
CREATE TABLE IF NOT EXISTS currencies (
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    is_base INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS exchange_rates (
    id TEXT PRIMARY KEY,
    from_currency TEXT NOT NULL,
    to_currency TEXT NOT NULL,
    rate REAL NOT NULL,
    effective_date TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_exchange_rates_currencies ON exchange_rates(from_currency, to_currency);
CREATE INDEX IF NOT EXISTS idx_exchange_rates_date ON exchange_rates(effective_date);

-- Budgeting
CREATE TABLE IF NOT EXISTS budgets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    fiscal_year_id TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_amount INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_lines (
    id TEXT PRIMARY KEY,
    budget_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    period INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    actual INTEGER DEFAULT 0,
    variance INTEGER DEFAULT 0,
    FOREIGN KEY (budget_id) REFERENCES budgets(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_budget_lines_budget ON budget_lines(budget_id);
CREATE INDEX IF NOT EXISTS idx_budget_lines_account ON budget_lines(account_id);

-- Lot/Serial tracking
CREATE TABLE IF NOT EXISTS lots (
    id TEXT PRIMARY KEY,
    lot_number TEXT NOT NULL,
    product_id TEXT NOT NULL,
    serial_number TEXT,
    manufacture_date TEXT,
    expiry_date TEXT,
    quantity INTEGER NOT NULL DEFAULT 0,
    status TEXT DEFAULT 'Active',
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lot_transactions (
    id TEXT PRIMARY KEY,
    lot_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    reference_type TEXT,
    reference_id TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_lots_product ON lots(product_id);
CREATE INDEX IF NOT EXISTS idx_lots_lot_number ON lots(lot_number);
CREATE INDEX IF NOT EXISTS idx_lot_transactions_lot ON lot_transactions(lot_id);

-- Leave Management
CREATE TABLE IF NOT EXISTS leave_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    days_per_year INTEGER DEFAULT 0,
    carry_over INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS leave_balances (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    leave_type_id TEXT NOT NULL,
    year INTEGER NOT NULL,
    entitled INTEGER DEFAULT 0,
    used INTEGER DEFAULT 0,
    remaining INTEGER DEFAULT 0,
    carried_over INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS leave_requests (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    leave_type_id TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    days INTEGER NOT NULL,
    reason TEXT,
    status TEXT DEFAULT 'Pending',
    approved_by TEXT,
    approved_at TEXT,
    rejection_reason TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_leave_balances_employee ON leave_balances(employee_id);
CREATE INDEX IF NOT EXISTS idx_leave_requests_employee ON leave_requests(employee_id);
CREATE INDEX IF NOT EXISTS idx_leave_requests_status ON leave_requests(status);

-- Expense Management
CREATE TABLE IF NOT EXISTS expense_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS expense_reports (
    id TEXT PRIMARY KEY,
    report_number TEXT NOT NULL UNIQUE,
    employee_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    total_amount INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Draft',
    submitted_at TEXT,
    approved_by TEXT,
    approved_at TEXT,
    rejected_at TEXT,
    rejection_reason TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS expense_lines (
    id TEXT PRIMARY KEY,
    expense_report_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    expense_date TEXT NOT NULL,
    description TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT DEFAULT 'USD',
    receipt_path TEXT,
    FOREIGN KEY (expense_report_id) REFERENCES expense_reports(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_expense_reports_employee ON expense_reports(employee_id);
CREATE INDEX IF NOT EXISTS idx_expense_reports_status ON expense_reports(status);
CREATE INDEX IF NOT EXISTS idx_expense_lines_report ON expense_lines(expense_report_id);
