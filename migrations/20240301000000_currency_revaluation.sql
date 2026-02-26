-- Currency Revaluation Tables (IFRS/GAAP Compliance)

CREATE TABLE IF NOT EXISTS currency_revaluations (
    id TEXT PRIMARY KEY,
    revaluation_number TEXT NOT NULL UNIQUE,
    revaluation_date TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    base_currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Draft',
    total_unrealized_gain INTEGER NOT NULL DEFAULT 0,
    total_unrealized_loss INTEGER NOT NULL DEFAULT 0,
    net_unrealized INTEGER NOT NULL DEFAULT 0,
    journal_entry_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    FOREIGN KEY (journal_entry_id) REFERENCES journal_entries(id)
);

CREATE INDEX IF NOT EXISTS idx_currency_revaluations_date ON currency_revaluations(revaluation_date);
CREATE INDEX IF NOT EXISTS idx_currency_revaluations_status ON currency_revaluations(status);
CREATE INDEX IF NOT EXISTS idx_currency_revaluations_period ON currency_revaluations(period_start, period_end);

CREATE TABLE IF NOT EXISTS currency_revaluation_lines (
    id TEXT PRIMARY KEY,
    revaluation_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    account_code TEXT NOT NULL,
    account_name TEXT NOT NULL,
    currency TEXT NOT NULL,
    original_balance INTEGER NOT NULL,
    original_rate REAL NOT NULL,
    revaluation_rate REAL NOT NULL,
    base_currency_balance INTEGER NOT NULL,
    revalued_balance INTEGER NOT NULL,
    unrealized_gain INTEGER NOT NULL DEFAULT 0,
    unrealized_loss INTEGER NOT NULL DEFAULT 0,
    gain_account_id TEXT,
    loss_account_id TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (revaluation_id) REFERENCES currency_revaluations(id) ON DELETE CASCADE,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (gain_account_id) REFERENCES accounts(id),
    FOREIGN KEY (loss_account_id) REFERENCES accounts(id)
);

CREATE INDEX IF NOT EXISTS idx_currency_reval_lines_reval ON currency_revaluation_lines(revaluation_id);
CREATE INDEX IF NOT EXISTS idx_currency_reval_lines_account ON currency_revaluation_lines(account_id);
CREATE INDEX IF NOT EXISTS idx_currency_reval_lines_currency ON currency_revaluation_lines(currency);
