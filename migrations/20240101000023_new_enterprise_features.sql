-- Document Management System (DMS)
CREATE TABLE IF NOT EXISTS document_folders (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id TEXT REFERENCES document_folders(id),
    path TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,
    document_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    document_type TEXT DEFAULT 'Other',
    folder_id TEXT REFERENCES document_folders(id),
    status TEXT DEFAULT 'Draft',
    version INTEGER DEFAULT 1,
    revision TEXT DEFAULT 'A',
    access_level TEXT DEFAULT 'Internal',
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,
    checksum TEXT NOT NULL,
    author_id TEXT,
    owner_id TEXT,
    checked_out_by TEXT,
    checked_out_at TEXT,
    approved_by TEXT,
    approved_at TEXT,
    published_at TEXT,
    expires_at TEXT,
    tags TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS document_versions (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES documents(id),
    version INTEGER NOT NULL,
    revision TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    checksum TEXT NOT NULL,
    change_summary TEXT,
    changed_by TEXT,
    status TEXT DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS document_checkouts (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES documents(id),
    user_id TEXT NOT NULL,
    checkout_at TEXT NOT NULL,
    expected_return TEXT,
    checkin_at TEXT,
    notes TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS document_reviews (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL REFERENCES documents(id),
    version INTEGER NOT NULL,
    reviewer_id TEXT NOT NULL,
    requested_at TEXT NOT NULL,
    reviewed_at TEXT,
    status TEXT DEFAULT 'Pending',
    comments TEXT,
    approved INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS document_permissions (
    id TEXT PRIMARY KEY,
    document_id TEXT REFERENCES documents(id),
    folder_id TEXT REFERENCES document_folders(id),
    user_id TEXT,
    role_id TEXT,
    can_read INTEGER DEFAULT 1,
    can_write INTEGER DEFAULT 0,
    can_delete INTEGER DEFAULT 0,
    can_share INTEGER DEFAULT 0,
    can_approve INTEGER DEFAULT 0,
    can_checkout INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS document_relations (
    id TEXT PRIMARY KEY,
    source_document_id TEXT NOT NULL REFERENCES documents(id),
    target_document_id TEXT NOT NULL REFERENCES documents(id),
    relation_type TEXT DEFAULT 'Related',
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS document_workflows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    document_type TEXT,
    steps TEXT NOT NULL,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS document_workflow_instances (
    id TEXT PRIMARY KEY,
    workflow_id TEXT NOT NULL REFERENCES document_workflows(id),
    document_id TEXT NOT NULL REFERENCES documents(id),
    current_step INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    started_at TEXT NOT NULL,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS retention_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    document_types TEXT,
    retention_years INTEGER NOT NULL DEFAULT 7,
    review_after_years INTEGER,
    disposition TEXT DEFAULT 'Destroy',
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

-- Advanced Pricing Engine
CREATE TABLE IF NOT EXISTS price_books (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    currency TEXT NOT NULL DEFAULT 'USD',
    is_default INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    valid_from TEXT,
    valid_to TEXT,
    parent_id TEXT REFERENCES price_books(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS price_book_entries (
    id TEXT PRIMARY KEY,
    price_book_id TEXT NOT NULL REFERENCES price_books(id),
    product_id TEXT NOT NULL,
    unit_price INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    min_quantity INTEGER DEFAULT 1,
    max_quantity INTEGER,
    valid_from TEXT,
    valid_to TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT,
    UNIQUE(price_book_id, product_id, min_quantity)
);

CREATE TABLE IF NOT EXISTS price_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    rule_type TEXT DEFAULT 'Discount',
    scope TEXT DEFAULT 'Global',
    priority INTEGER DEFAULT 100,
    value REAL NOT NULL,
    currency TEXT,
    conditions TEXT,
    valid_from TEXT,
    valid_to TEXT,
    is_active INTEGER DEFAULT 1,
    is_stackable INTEGER DEFAULT 0,
    max_applications INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS price_rule_assignments (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL REFERENCES price_rules(id),
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS discounts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    discount_type TEXT DEFAULT 'Percentage',
    value REAL NOT NULL,
    max_discount INTEGER,
    min_order_value INTEGER,
    applicable_to TEXT,
    customer_groups TEXT,
    products TEXT,
    categories TEXT,
    valid_from TEXT,
    valid_to TEXT,
    usage_limit INTEGER,
    usage_per_customer INTEGER,
    current_usage INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    requires_code INTEGER DEFAULT 1,
    auto_apply INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS discount_usages (
    id TEXT PRIMARY KEY,
    discount_id TEXT NOT NULL REFERENCES discounts(id),
    order_id TEXT NOT NULL,
    customer_id TEXT,
    discount_amount INTEGER NOT NULL,
    applied_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS promotions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    promotion_type TEXT DEFAULT 'ProductDiscount',
    status TEXT DEFAULT 'Draft',
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    rules TEXT NOT NULL,
    rewards TEXT NOT NULL,
    target_segments TEXT,
    channels TEXT,
    budget INTEGER,
    spent INTEGER DEFAULT 0,
    usage_limit INTEGER,
    current_usage INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS coupons (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    discount_id TEXT NOT NULL REFERENCES discounts(id),
    promotion_id TEXT REFERENCES promotions(id),
    customer_id TEXT,
    is_used INTEGER DEFAULT 0,
    used_at TEXT,
    order_id TEXT,
    valid_from TEXT,
    valid_to TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS price_tiers (
    id TEXT PRIMARY KEY,
    price_book_entry_id TEXT REFERENCES price_book_entries(id),
    product_id TEXT,
    min_quantity INTEGER NOT NULL,
    max_quantity INTEGER,
    unit_price INTEGER NOT NULL,
    discount_percent REAL,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS customer_price_groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    price_book_id TEXT REFERENCES price_books(id),
    discount_id TEXT REFERENCES discounts(id),
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS customer_price_group_members (
    id TEXT PRIMARY KEY,
    group_id TEXT NOT NULL REFERENCES customer_price_groups(id),
    customer_id TEXT NOT NULL,
    joined_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT,
    UNIQUE(group_id, customer_id)
);

-- Sourcing/Auctions
CREATE TABLE IF NOT EXISTS sourcing_events (
    id TEXT PRIMARY KEY,
    event_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    event_type TEXT DEFAULT 'RFQ',
    status TEXT DEFAULT 'Draft',
    auction_type TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    estimated_value INTEGER,
    budget INTEGER,
    requirements TEXT,
    evaluation_criteria TEXT,
    terms_conditions TEXT,
    buyer_id TEXT,
    category_id TEXT,
    is_public INTEGER DEFAULT 1,
    allow_reverse_auction INTEGER DEFAULT 0,
    min_bid_decrement INTEGER,
    auto_extend INTEGER DEFAULT 0,
    extension_minutes INTEGER,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by_user TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_items (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES sourcing_events(id),
    product_id TEXT,
    sku TEXT,
    name TEXT NOT NULL,
    description TEXT,
    quantity INTEGER NOT NULL,
    unit_of_measure TEXT NOT NULL,
    target_price INTEGER,
    max_price INTEGER,
    specifications TEXT,
    delivery_date TEXT,
    delivery_location TEXT,
    sort_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_suppliers (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES sourcing_events(id),
    vendor_id TEXT NOT NULL,
    invited_at TEXT NOT NULL,
    accepted_at TEXT,
    declined_at TEXT,
    status TEXT DEFAULT 'Active',
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_bids (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES sourcing_events(id),
    vendor_id TEXT NOT NULL,
    bid_number TEXT NOT NULL UNIQUE,
    status TEXT DEFAULT 'Draft',
    submitted_at TEXT,
    valid_until TEXT,
    total_amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    terms TEXT,
    notes TEXT,
    rank INTEGER,
    score REAL,
    is_winner INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_bid_lines (
    id TEXT PRIMARY KEY,
    bid_id TEXT NOT NULL REFERENCES sourcing_bids(id),
    item_id TEXT NOT NULL REFERENCES sourcing_items(id),
    unit_price INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    total_price INTEGER NOT NULL,
    delivery_date TEXT,
    lead_time_days INTEGER,
    specifications_met INTEGER DEFAULT 1,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_bid_attachments (
    id TEXT PRIMARY KEY,
    bid_id TEXT NOT NULL REFERENCES sourcing_bids(id),
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,
    uploaded_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_auction_rounds (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES sourcing_events(id),
    round_number INTEGER NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    status TEXT DEFAULT 'Active',
    min_bid INTEGER,
    max_bid INTEGER,
    bid_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_auction_bids (
    id TEXT PRIMARY KEY,
    round_id TEXT NOT NULL REFERENCES sourcing_auction_rounds(id),
    vendor_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    bid_time TEXT NOT NULL,
    rank INTEGER,
    is_winning INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_evaluation_criteria (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES sourcing_events(id),
    name TEXT NOT NULL,
    description TEXT,
    weight REAL NOT NULL DEFAULT 1.0,
    max_score INTEGER NOT NULL DEFAULT 100,
    evaluation_method TEXT DEFAULT 'Score',
    sort_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_bid_evaluations (
    id TEXT PRIMARY KEY,
    bid_id TEXT NOT NULL REFERENCES sourcing_bids(id),
    criteria_id TEXT NOT NULL REFERENCES sourcing_evaluation_criteria(id),
    score INTEGER NOT NULL,
    comments TEXT,
    evaluated_by TEXT,
    evaluated_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_awards (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES sourcing_events(id),
    bid_id TEXT NOT NULL REFERENCES sourcing_bids(id),
    vendor_id TEXT NOT NULL,
    item_id TEXT REFERENCES sourcing_items(id),
    awarded_quantity INTEGER NOT NULL,
    awarded_price INTEGER NOT NULL,
    total_value INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    awarded_at TEXT NOT NULL,
    award_type TEXT DEFAULT 'Full',
    purchase_order_id TEXT,
    contract_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS sourcing_contracts (
    id TEXT PRIMARY KEY,
    event_id TEXT REFERENCES sourcing_events(id),
    vendor_id TEXT NOT NULL,
    contract_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_value INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    terms TEXT,
    status TEXT DEFAULT 'Active',
    renewal_type TEXT,
    auto_renew INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS supplier_qualifications (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    qualification_type TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    issued_by TEXT,
    issued_date TEXT,
    expiry_date TEXT,
    certificate_number TEXT,
    document_path TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

-- System Configuration
CREATE TABLE IF NOT EXISTS system_configs (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    value_type TEXT DEFAULT 'String',
    description TEXT,
    is_encrypted INTEGER DEFAULT 0,
    is_system INTEGER DEFAULT 0,
    is_public INTEGER DEFAULT 0,
    default_value TEXT,
    validation_regex TEXT,
    group_name TEXT,
    sort_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT,
    UNIQUE(category, key)
);

CREATE TABLE IF NOT EXISTS config_history (
    id TEXT PRIMARY KEY,
    config_id TEXT NOT NULL REFERENCES system_configs(id),
    old_value TEXT,
    new_value TEXT NOT NULL,
    changed_by TEXT,
    changed_at TEXT NOT NULL,
    reason TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS company_settings (
    id TEXT PRIMARY KEY,
    company_name TEXT NOT NULL,
    legal_name TEXT,
    tax_id TEXT,
    registration_number TEXT,
    logo_url TEXT,
    favicon_url TEXT,
    primary_color TEXT,
    secondary_color TEXT,
    timezone TEXT DEFAULT 'UTC',
    date_format TEXT DEFAULT 'YYYY-MM-DD',
    time_format TEXT DEFAULT 'HH:mm',
    currency TEXT DEFAULT 'USD',
    language TEXT DEFAULT 'en',
    fiscal_year_start INTEGER DEFAULT 1,
    week_start INTEGER DEFAULT 1,
    address TEXT,
    city TEXT,
    state TEXT,
    country TEXT,
    postal_code TEXT,
    phone TEXT,
    email TEXT,
    website TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS number_sequences (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    prefix TEXT,
    suffix TEXT,
    current_value INTEGER DEFAULT 0,
    increment INTEGER DEFAULT 1,
    padding INTEGER DEFAULT 4,
    reset_period TEXT,
    last_reset TEXT,
    format TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS email_configs (
    id TEXT PRIMARY KEY,
    smtp_host TEXT NOT NULL,
    smtp_port INTEGER NOT NULL DEFAULT 587,
    smtp_user TEXT NOT NULL,
    smtp_password TEXT,
    use_tls INTEGER DEFAULT 1,
    use_ssl INTEGER DEFAULT 0,
    from_address TEXT NOT NULL,
    from_name TEXT,
    reply_to TEXT,
    is_default INTEGER DEFAULT 1,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS storage_configs (
    id TEXT PRIMARY KEY,
    storage_type TEXT DEFAULT 'Local',
    name TEXT NOT NULL,
    endpoint TEXT,
    bucket TEXT,
    region TEXT,
    access_key TEXT,
    secret_key TEXT,
    base_path TEXT,
    max_file_size INTEGER DEFAULT 10485760,
    allowed_types TEXT,
    is_default INTEGER DEFAULT 1,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS payment_gateways (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    gateway_type TEXT NOT NULL,
    api_key TEXT,
    api_secret TEXT,
    merchant_id TEXT,
    endpoint_url TEXT,
    webhook_url TEXT,
    supported_currencies TEXT,
    supported_methods TEXT,
    fee_percent REAL,
    fee_fixed INTEGER,
    is_sandbox INTEGER DEFAULT 0,
    is_default INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS shipping_providers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    api_key TEXT,
    api_secret TEXT,
    account_number TEXT,
    endpoint_url TEXT,
    tracking_url TEXT,
    supported_services TEXT,
    supported_countries TEXT,
    is_default INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS localizations (
    id TEXT PRIMARY KEY,
    language_code TEXT NOT NULL,
    locale TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    native_name TEXT NOT NULL,
    date_format TEXT NOT NULL,
    time_format TEXT NOT NULL,
    number_format TEXT NOT NULL,
    currency_symbol TEXT,
    currency_position TEXT DEFAULT 'before',
    decimal_separator TEXT DEFAULT '.',
    thousand_separator TEXT DEFAULT ',',
    is_rtl INTEGER DEFAULT 0,
    is_default INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS audit_settings (
    id TEXT PRIMARY KEY,
    log_retention_days INTEGER DEFAULT 365,
    log_sensitive_data INTEGER DEFAULT 0,
    log_login_attempts INTEGER DEFAULT 1,
    log_data_changes INTEGER DEFAULT 1,
    log_api_requests INTEGER DEFAULT 1,
    alert_on_suspicious INTEGER DEFAULT 1,
    max_login_attempts INTEGER DEFAULT 5,
    lockout_duration_minutes INTEGER DEFAULT 30,
    password_expiry_days INTEGER,
    require_mfa INTEGER DEFAULT 0,
    session_timeout_minutes INTEGER DEFAULT 60,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS integration_configs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    integration_type TEXT NOT NULL,
    api_endpoint TEXT,
    api_key TEXT,
    api_secret TEXT,
    config_json TEXT,
    sync_enabled INTEGER DEFAULT 0,
    sync_frequency TEXT,
    last_sync TEXT,
    sync_status TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

-- Business Rules Engine
CREATE TABLE IF NOT EXISTS business_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    rule_type TEXT DEFAULT 'Validation',
    entity_type TEXT NOT NULL,
    status TEXT DEFAULT 'Active',
    priority INTEGER DEFAULT 100,
    version INTEGER DEFAULT 1,
    effective_from TEXT,
    effective_to TEXT,
    conditions TEXT NOT NULL,
    actions TEXT NOT NULL,
    else_actions TEXT,
    tags TEXT,
    owner_id TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS rule_conditions (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL REFERENCES business_rules(id),
    group_id TEXT REFERENCES rule_condition_groups(id),
    field TEXT NOT NULL,
    operator TEXT NOT NULL,
    value TEXT NOT NULL,
    value_type TEXT DEFAULT 'String',
    logical_operator TEXT DEFAULT 'And',
    sort_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_condition_groups (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL REFERENCES business_rules(id),
    parent_group_id TEXT REFERENCES rule_condition_groups(id),
    logical_operator TEXT DEFAULT 'And',
    sort_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_actions (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL REFERENCES business_rules(id),
    action_type TEXT NOT NULL,
    target TEXT NOT NULL,
    parameters TEXT,
    execution_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_sets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    entity_type TEXT NOT NULL,
    status TEXT DEFAULT 'Active',
    version INTEGER DEFAULT 1,
    effective_from TEXT,
    effective_to TEXT,
    execution_mode TEXT DEFAULT 'Sequential',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_set_members (
    id TEXT PRIMARY KEY,
    ruleset_id TEXT NOT NULL REFERENCES rule_sets(id),
    rule_id TEXT NOT NULL REFERENCES business_rules(id),
    sort_order INTEGER DEFAULT 0,
    is_required INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_executions (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL REFERENCES business_rules(id),
    ruleset_id TEXT REFERENCES rule_sets(id),
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    triggered_at TEXT NOT NULL,
    conditions_evaluated TEXT,
    matched INTEGER DEFAULT 0,
    actions_executed TEXT,
    result TEXT,
    error TEXT,
    execution_time_ms INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_variables (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    data_type TEXT NOT NULL DEFAULT 'String',
    default_value TEXT,
    source_type TEXT DEFAULT 'Static',
    source_config TEXT,
    is_constant INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_functions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    return_type TEXT NOT NULL DEFAULT 'String',
    parameters TEXT,
    function_body TEXT,
    is_builtin INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    rule_type TEXT DEFAULT 'Validation',
    entity_type TEXT NOT NULL,
    template TEXT NOT NULL,
    variables TEXT,
    is_builtin INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS decision_tables (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    entity_type TEXT NOT NULL,
    input_columns TEXT NOT NULL,
    output_columns TEXT NOT NULL,
    hit_policy TEXT DEFAULT 'First',
    status TEXT DEFAULT 'Active',
    version INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS decision_table_rows (
    id TEXT PRIMARY KEY,
    table_id TEXT NOT NULL REFERENCES decision_tables(id),
    row_number INTEGER NOT NULL,
    inputs TEXT NOT NULL,
    outputs TEXT NOT NULL,
    description TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS rule_versions (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL REFERENCES business_rules(id),
    version INTEGER NOT NULL,
    conditions TEXT NOT NULL,
    actions TEXT NOT NULL,
    changed_by TEXT,
    changed_at TEXT NOT NULL,
    change_reason TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_documents_folder ON documents(folder_id);
CREATE INDEX IF NOT EXISTS idx_documents_status ON documents(status);
CREATE INDEX IF NOT EXISTS idx_price_book_entries_product ON price_book_entries(product_id);
CREATE INDEX IF NOT EXISTS idx_discounts_code ON discounts(code);
CREATE INDEX IF NOT EXISTS idx_coupons_code ON coupons(code);
CREATE INDEX IF NOT EXISTS idx_sourcing_events_status ON sourcing_events(status);
CREATE INDEX IF NOT EXISTS idx_sourcing_bids_event ON sourcing_bids(event_id);
CREATE INDEX IF NOT EXISTS idx_business_rules_entity ON business_rules(entity_type);
CREATE INDEX IF NOT EXISTS idx_business_rules_status ON business_rules(status);
CREATE INDEX IF NOT EXISTS idx_rule_executions_entity ON rule_executions(entity_type, entity_id);
