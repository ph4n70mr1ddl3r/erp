CREATE TABLE IF NOT EXISTS tpm_promotions (
    id TEXT PRIMARY KEY,
    promotion_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    promotion_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    customer_id TEXT,
    customer_group_id TEXT,
    product_id TEXT,
    product_group_id TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    planned_budget INTEGER NOT NULL,
    committed_budget INTEGER DEFAULT 0,
    spent_budget INTEGER DEFAULT 0,
    accrued_budget INTEGER DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    discount_percent REAL,
    discount_amount INTEGER,
    buy_quantity INTEGER,
    get_quantity INTEGER,
    max_redemptions INTEGER,
    redemptions_count INTEGER DEFAULT 0,
    forecasted_sales INTEGER,
    actual_sales INTEGER,
    roi REAL,
    owner_id TEXT,
    approval_status TEXT NOT NULL DEFAULT 'Pending',
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_promotion_products (
    id TEXT PRIMARY KEY,
    promotion_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    discount_percent REAL,
    discount_amount INTEGER,
    buy_qty INTEGER,
    get_qty INTEGER,
    max_qty INTEGER,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_promotion_customers (
    id TEXT PRIMARY KEY,
    promotion_id TEXT NOT NULL,
    customer_id TEXT,
    customer_group_id TEXT,
    territory_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_trade_funds (
    id TEXT PRIMARY KEY,
    fund_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    fund_type TEXT NOT NULL,
    customer_id TEXT,
    fiscal_year INTEGER NOT NULL,
    total_budget INTEGER NOT NULL,
    committed_amount INTEGER DEFAULT 0,
    spent_amount INTEGER DEFAULT 0,
    available_amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_fund_transactions (
    id TEXT PRIMARY KEY,
    fund_id TEXT NOT NULL,
    promotion_id TEXT,
    transaction_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    reference_number TEXT,
    description TEXT,
    transaction_date TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_rebate_agreements (
    id TEXT PRIMARY KEY,
    agreement_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    agreement_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    basis TEXT NOT NULL,
    calculation_method TEXT NOT NULL,
    payment_terms TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    total_eligible_sales INTEGER DEFAULT 0,
    total_rebate_earned INTEGER DEFAULT 0,
    total_rebate_paid INTEGER DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_rebate_tiers (
    id TEXT PRIMARY KEY,
    agreement_id TEXT NOT NULL,
    tier_number INTEGER NOT NULL,
    min_quantity REAL NOT NULL,
    max_quantity REAL,
    min_value INTEGER NOT NULL,
    max_value INTEGER,
    rebate_percent REAL NOT NULL,
    rebate_amount INTEGER,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_rebate_products (
    id TEXT PRIMARY KEY,
    agreement_id TEXT NOT NULL,
    product_id TEXT,
    product_group_id TEXT,
    specific_rate REAL,
    specific_amount INTEGER,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_rebate_accruals (
    id TEXT PRIMARY KEY,
    agreement_id TEXT NOT NULL,
    sales_order_id TEXT,
    invoice_id TEXT,
    product_id TEXT,
    sales_amount INTEGER NOT NULL,
    rebate_rate REAL NOT NULL,
    rebate_amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    accrual_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Accrued',
    paid_amount INTEGER DEFAULT 0,
    remaining_amount INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_rebate_payments (
    id TEXT PRIMARY KEY,
    payment_number TEXT NOT NULL UNIQUE,
    agreement_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    payment_date TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    payment_method TEXT NOT NULL,
    reference_number TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_rebate_payment_lines (
    id TEXT PRIMARY KEY,
    payment_id TEXT NOT NULL,
    accrual_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_chargebacks (
    id TEXT PRIMARY KEY,
    chargeback_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    invoice_id TEXT,
    promotion_id TEXT,
    chargeback_date TEXT NOT NULL,
    amount_claimed INTEGER NOT NULL,
    amount_approved INTEGER DEFAULT 0,
    amount_rejected INTEGER DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Submitted',
    claim_type TEXT NOT NULL,
    description TEXT,
    rejection_reason TEXT,
    submitted_by TEXT,
    reviewed_by TEXT,
    reviewed_at TEXT,
    paid_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_chargeback_lines (
    id TEXT PRIMARY KEY,
    chargeback_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    claimed_amount INTEGER NOT NULL,
    approved_amount INTEGER DEFAULT 0,
    rejected_amount INTEGER DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_chargeback_documents (
    id TEXT PRIMARY KEY,
    chargeback_id TEXT NOT NULL,
    document_type TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    uploaded_by TEXT,
    uploaded_at TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_promotion_performance (
    id TEXT PRIMARY KEY,
    promotion_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    baseline_sales INTEGER NOT NULL,
    incremental_sales INTEGER NOT NULL,
    total_sales INTEGER NOT NULL,
    units_sold INTEGER NOT NULL,
    promotion_cost INTEGER NOT NULL,
    roi_percent REAL NOT NULL,
    lift_percent REAL NOT NULL,
    cannibalization INTEGER,
    forward_buy INTEGER,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_promotion_plans (
    id TEXT PRIMARY KEY,
    plan_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    fiscal_year INTEGER NOT NULL,
    customer_id TEXT,
    customer_group_id TEXT,
    total_budget INTEGER NOT NULL,
    allocated_budget INTEGER DEFAULT 0,
    spent_budget INTEGER DEFAULT 0,
    remaining_budget INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Draft',
    owner_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_promotion_plan_lines (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    promotion_id TEXT NOT NULL,
    quarter INTEGER NOT NULL,
    planned_amount INTEGER NOT NULL,
    actual_amount INTEGER DEFAULT 0,
    variance INTEGER DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_customer_trade_profiles (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL UNIQUE,
    trade_class TEXT NOT NULL,
    annual_volume INTEGER,
    growth_rate REAL,
    avg_promotion_response REAL,
    preferred_promotion_type TEXT,
    credit_limit INTEGER,
    payment_terms TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_ship_and_debits (
    id TEXT PRIMARY KEY,
    sad_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    authorized_price INTEGER NOT NULL,
    list_price INTEGER NOT NULL,
    authorized_discount INTEGER NOT NULL,
    quantity_authorized INTEGER NOT NULL,
    quantity_shipped INTEGER DEFAULT 0,
    quantity_debited INTEGER DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tpm_price_protections (
    id TEXT PRIMARY KEY,
    pp_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    product_id TEXT,
    product_group_id TEXT,
    old_price INTEGER NOT NULL,
    new_price INTEGER NOT NULL,
    price_reduction INTEGER NOT NULL,
    effective_date TEXT NOT NULL,
    inventory_on_hand INTEGER NOT NULL,
    claim_amount INTEGER NOT NULL,
    approved_amount INTEGER DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Submitted',
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
