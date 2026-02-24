-- WMS Tables
CREATE TABLE IF NOT EXISTS wms_storage_locations (
    id TEXT PRIMARY KEY,
    warehouse_id TEXT NOT NULL,
    zone TEXT NOT NULL,
    aisle TEXT NOT NULL,
    rack TEXT NOT NULL,
    shelf TEXT NOT NULL,
    bin TEXT NOT NULL,
    location_type TEXT NOT NULL,
    capacity INTEGER NOT NULL,
    occupied INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_wms_locations_warehouse ON wms_storage_locations(warehouse_id);
CREATE INDEX IF NOT EXISTS idx_wms_locations_zone ON wms_storage_locations(zone);

CREATE TABLE IF NOT EXISTS wms_zones (
    id TEXT PRIMARY KEY,
    warehouse_id TEXT NOT NULL,
    zone_code TEXT NOT NULL,
    zone_name TEXT NOT NULL,
    zone_type TEXT NOT NULL,
    temperature_controlled INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_putaway_tasks (
    id TEXT PRIMARY KEY,
    receipt_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    source_location TEXT,
    suggested_location TEXT,
    actual_location TEXT,
    status TEXT NOT NULL,
    priority INTEGER NOT NULL,
    assigned_to TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_pick_tasks (
    id TEXT PRIMARY KEY,
    wave_id TEXT,
    order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    location_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    picked_quantity INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    pick_type TEXT NOT NULL,
    priority INTEGER NOT NULL,
    assigned_to TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_waves (
    id TEXT PRIMARY KEY,
    wave_number TEXT NOT NULL UNIQUE,
    warehouse_id TEXT NOT NULL,
    status TEXT NOT NULL,
    planned_date TEXT NOT NULL,
    released_at TEXT,
    completed_at TEXT,
    total_picks INTEGER NOT NULL DEFAULT 0,
    completed_picks INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_crossdock_orders (
    id TEXT PRIMARY KEY,
    inbound_shipment_id TEXT NOT NULL,
    outbound_order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    status TEXT NOT NULL,
    dock_location TEXT,
    processed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_receiving_receipts (
    id TEXT PRIMARY KEY,
    receipt_number TEXT NOT NULL UNIQUE,
    warehouse_id TEXT NOT NULL,
    po_id TEXT,
    carrier TEXT,
    tracking_number TEXT,
    status TEXT NOT NULL,
    received_by TEXT,
    received_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_receipt_lines (
    id TEXT PRIMARY KEY,
    receipt_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    expected_qty INTEGER NOT NULL,
    received_qty INTEGER NOT NULL,
    damaged_qty INTEGER NOT NULL DEFAULT 0,
    lot_number TEXT,
    expiry_date TEXT,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_shipping_manifests (
    id TEXT PRIMARY KEY,
    manifest_number TEXT NOT NULL UNIQUE,
    warehouse_id TEXT NOT NULL,
    carrier TEXT NOT NULL,
    service_level TEXT NOT NULL,
    status TEXT NOT NULL,
    total_packages INTEGER NOT NULL,
    total_weight REAL NOT NULL,
    shipped_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_cycle_counts (
    id TEXT PRIMARY KEY,
    count_number TEXT NOT NULL UNIQUE,
    warehouse_id TEXT NOT NULL,
    count_type TEXT NOT NULL,
    status TEXT NOT NULL,
    scheduled_date TEXT NOT NULL,
    completed_date TEXT,
    variance_count INTEGER NOT NULL DEFAULT 0,
    accuracy_rate REAL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS wms_count_lines (
    id TEXT PRIMARY KEY,
    cycle_count_id TEXT NOT NULL,
    location_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    system_qty INTEGER NOT NULL,
    counted_qty INTEGER NOT NULL,
    variance INTEGER NOT NULL,
    counted_by TEXT,
    recounted INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL
);

-- Demand Planning Tables
CREATE TABLE IF NOT EXISTS demand_forecasts (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT,
    forecast_date TEXT NOT NULL,
    period_type TEXT NOT NULL,
    forecast_qty REAL NOT NULL,
    lower_bound REAL NOT NULL,
    upper_bound REAL NOT NULL,
    confidence REAL NOT NULL,
    method TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_demand_forecasts_product ON demand_forecasts(product_id);
CREATE INDEX IF NOT EXISTS idx_demand_forecasts_date ON demand_forecasts(forecast_date);

CREATE TABLE IF NOT EXISTS demand_forecast_models (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    method TEXT NOT NULL,
    parameters TEXT NOT NULL,
    accuracy_mape REAL,
    accuracy_mse REAL,
    last_trained TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS demand_plans (
    id TEXT PRIMARY KEY,
    plan_name TEXT NOT NULL,
    plan_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    baseline_id TEXT,
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS demand_plan_lines (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    location_id TEXT,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    forecast_qty REAL NOT NULL,
    adjusted_qty REAL NOT NULL,
    final_qty REAL NOT NULL,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS demand_safety_stock (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    safety_qty INTEGER NOT NULL,
    reorder_point INTEGER NOT NULL,
    service_level REAL NOT NULL,
    lead_time_days INTEGER NOT NULL,
    demand_variability REAL NOT NULL,
    last_calculated TEXT NOT NULL,
    UNIQUE(product_id, warehouse_id)
);

CREATE TABLE IF NOT EXISTS demand_history (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT,
    date TEXT NOT NULL,
    actual_qty INTEGER NOT NULL,
    forecast_qty REAL,
    variance REAL,
    source TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS demand_promotion_impacts (
    id TEXT PRIMARY KEY,
    promotion_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    lift_factor REAL NOT NULL,
    cannibalization TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS demand_sensing_signals (
    id TEXT PRIMARY KEY,
    signal_type TEXT NOT NULL,
    source TEXT NOT NULL,
    value REAL NOT NULL,
    weight REAL NOT NULL,
    timestamp TEXT NOT NULL,
    product_ids TEXT NOT NULL
);

-- EDI Tables
CREATE TABLE IF NOT EXISTS edi_partners (
    id TEXT PRIMARY KEY,
    partner_code TEXT NOT NULL UNIQUE,
    partner_name TEXT NOT NULL,
    partner_type TEXT NOT NULL,
    qualifier TEXT NOT NULL,
    interchange_id TEXT NOT NULL,
    communication_type TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    encryption TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS edi_transactions (
    id TEXT PRIMARY KEY,
    partner_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    direction TEXT NOT NULL,
    control_number TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL,
    raw_content TEXT,
    parsed_data TEXT,
    error_message TEXT,
    processed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_edi_transactions_partner ON edi_transactions(partner_id);

CREATE TABLE IF NOT EXISTS edi_mappings (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    version TEXT NOT NULL,
    mapping_rules TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS edi_acknowledgments (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    ack_type TEXT NOT NULL,
    accepted INTEGER NOT NULL,
    error_codes TEXT NOT NULL,
    segment_errors TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS edi_850_orders (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    po_number TEXT NOT NULL,
    po_date TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    ship_to TEXT NOT NULL,
    bill_to TEXT NOT NULL,
    lines TEXT NOT NULL,
    total_amount INTEGER NOT NULL,
    currency TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS edi_810_invoices (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    invoice_number TEXT NOT NULL,
    invoice_date TEXT NOT NULL,
    po_number TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    lines TEXT NOT NULL,
    subtotal INTEGER NOT NULL,
    tax INTEGER NOT NULL,
    total INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS edi_856_asns (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    asn_number TEXT NOT NULL,
    shipment_date TEXT NOT NULL,
    expected_date TEXT NOT NULL,
    po_number TEXT NOT NULL,
    carrier TEXT NOT NULL,
    tracking_number TEXT,
    packages TEXT NOT NULL,
    total_items INTEGER NOT NULL
);

-- Tenant Tables
CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    plan TEXT NOT NULL,
    status TEXT NOT NULL,
    settings TEXT NOT NULL,
    branding TEXT,
    limits TEXT NOT NULL,
    trial_ends_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tenant_users (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT NOT NULL,
    permissions TEXT NOT NULL,
    is_primary INTEGER NOT NULL DEFAULT 0,
    invited_by TEXT,
    invited_at TEXT,
    joined_at TEXT,
    status TEXT NOT NULL,
    UNIQUE(tenant_id, user_id)
);

CREATE TABLE IF NOT EXISTS tenant_invitations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    email TEXT NOT NULL,
    role TEXT NOT NULL,
    invited_by TEXT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    expires_at TEXT NOT NULL,
    accepted_at TEXT,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tenant_usage (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    period TEXT NOT NULL,
    users_count INTEGER NOT NULL,
    products_count INTEGER NOT NULL,
    orders_count INTEGER NOT NULL,
    storage_used_mb INTEGER NOT NULL,
    api_calls INTEGER NOT NULL,
    computed_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tenant_features (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    feature_key TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    settings TEXT,
    updated_at TEXT NOT NULL,
    UNIQUE(tenant_id, feature_key)
);

CREATE TABLE IF NOT EXISTS tenant_audit_logs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    user_id TEXT,
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT,
    old_value TEXT,
    new_value TEXT,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tenant_billing (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL UNIQUE,
    billing_email TEXT NOT NULL,
    billing_address TEXT NOT NULL,
    payment_method TEXT,
    subscription_id TEXT,
    next_billing_date TEXT,
    amount_due INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD'
);

-- Revenue Recognition Tables
CREATE TABLE IF NOT EXISTS revrec_contracts (
    id TEXT PRIMARY KEY,
    contract_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    contract_date TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_value INTEGER NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL,
    transaction_price INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS revrec_performance_obligations (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    standalone_price INTEGER NOT NULL,
    allocated_price INTEGER NOT NULL,
    recognition_type TEXT NOT NULL,
    recognition_method TEXT NOT NULL,
    total_periods INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS revrec_schedules (
    id TEXT PRIMARY KEY,
    obligation_id TEXT NOT NULL,
    period TEXT NOT NULL,
    planned_revenue INTEGER NOT NULL,
    recognized_revenue INTEGER NOT NULL DEFAULT 0,
    deferred_revenue INTEGER NOT NULL,
    status TEXT NOT NULL,
    recognized_at TEXT
);

CREATE TABLE IF NOT EXISTS revrec_modifications (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    modification_date TEXT NOT NULL,
    modification_type TEXT NOT NULL,
    description TEXT NOT NULL,
    price_change INTEGER NOT NULL,
    new_total_value INTEGER NOT NULL,
    approved_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS revrec_deferred_revenue (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    obligation_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    original_amount INTEGER NOT NULL,
    recognized_amount INTEGER NOT NULL DEFAULT 0,
    remaining_amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    recognition_start TEXT NOT NULL,
    recognition_end TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS revrec_events (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    obligation_id TEXT,
    event_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    event_date TEXT NOT NULL,
    period TEXT NOT NULL,
    description TEXT,
    journal_entry_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS revrec_allocation_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    method TEXT NOT NULL,
    basis TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS revrec_vatb (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    calculation_date TEXT NOT NULL,
    total_complete_percent REAL NOT NULL,
    costs_incurred INTEGER NOT NULL,
    total_estimated_costs INTEGER NOT NULL,
    revenue_to_date INTEGER NOT NULL,
    revenue_to_recognize INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

-- Intercompany Tables
CREATE TABLE IF NOT EXISTS intercompany_entities (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    legal_entity_id TEXT,
    currency TEXT NOT NULL,
    timezone TEXT NOT NULL,
    tax_id TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS intercompany_transactions (
    id TEXT PRIMARY KEY,
    transaction_number TEXT NOT NULL UNIQUE,
    source_entity_id TEXT NOT NULL,
    target_entity_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    source_amount INTEGER NOT NULL,
    source_currency TEXT NOT NULL,
    target_amount INTEGER NOT NULL,
    target_currency TEXT NOT NULL,
    exchange_rate REAL NOT NULL,
    status TEXT NOT NULL,
    source_document_id TEXT,
    target_document_id TEXT,
    due_date TEXT,
    created_at TEXT NOT NULL,
    settled_at TEXT
);

CREATE TABLE IF NOT EXISTS intercompany_transfer_prices (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    source_entity_id TEXT NOT NULL,
    target_entity_id TEXT NOT NULL,
    price INTEGER NOT NULL,
    currency TEXT NOT NULL,
    method TEXT NOT NULL,
    effective_from TEXT NOT NULL,
    effective_to TEXT,
    approved_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS intercompany_due_to_from (
    id TEXT PRIMARY KEY,
    source_entity_id TEXT NOT NULL,
    target_entity_id TEXT NOT NULL,
    account_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    as_of_date TEXT NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE(source_entity_id, target_entity_id, account_type)
);

CREATE TABLE IF NOT EXISTS intercompany_eliminations (
    id TEXT PRIMARY KEY,
    consolidation_id TEXT NOT NULL,
    source_transaction_id TEXT NOT NULL,
    debit_entity_id TEXT NOT NULL,
    credit_entity_id TEXT NOT NULL,
    account_code TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS intercompany_consolidations (
    id TEXT PRIMARY KEY,
    consolidation_number TEXT NOT NULL UNIQUE,
    period TEXT NOT NULL,
    status TEXT NOT NULL,
    entities TEXT NOT NULL,
    elimination_entries TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    created_by TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS intercompany_agreements (
    id TEXT PRIMARY KEY,
    agreement_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    source_entity_id TEXT NOT NULL,
    target_entity_id TEXT NOT NULL,
    agreement_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,
    terms TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- LMS Tables
CREATE TABLE IF NOT EXISTS lms_courses (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    difficulty TEXT NOT NULL,
    duration_hours REAL NOT NULL,
    format TEXT NOT NULL,
    instructor_id TEXT,
    max_enrollments INTEGER,
    credits INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lms_modules (
    id TEXT PRIMARY KEY,
    course_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    sequence INTEGER NOT NULL,
    duration_minutes INTEGER NOT NULL,
    content_type TEXT NOT NULL,
    content_url TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lms_enrollments (
    id TEXT PRIMARY KEY,
    course_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    enrolled_at TEXT NOT NULL,
    status TEXT NOT NULL,
    progress_percent REAL NOT NULL DEFAULT 0,
    started_at TEXT,
    completed_at TEXT,
    due_date TEXT,
    score REAL,
    certificate_id TEXT,
    UNIQUE(course_id, employee_id)
);

CREATE TABLE IF NOT EXISTS lms_learning_paths (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    target_role TEXT,
    courses TEXT NOT NULL,
    total_credits INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lms_assessments (
    id TEXT PRIMARY KEY,
    course_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    assessment_type TEXT NOT NULL,
    time_limit_minutes INTEGER,
    passing_score REAL NOT NULL,
    max_attempts INTEGER NOT NULL DEFAULT 1,
    questions TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lms_questions (
    id TEXT PRIMARY KEY,
    assessment_id TEXT NOT NULL,
    question_text TEXT NOT NULL,
    question_type TEXT NOT NULL,
    options TEXT,
    correct_answer TEXT NOT NULL,
    points INTEGER NOT NULL DEFAULT 1,
    sequence INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS lms_assessment_attempts (
    id TEXT PRIMARY KEY,
    assessment_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    attempt_number INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    submitted_at TEXT,
    score REAL,
    passed INTEGER,
    answers TEXT NOT NULL,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lms_certificates (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    course_id TEXT NOT NULL,
    certificate_number TEXT NOT NULL UNIQUE,
    issued_at TEXT NOT NULL,
    expires_at TEXT,
    verification_code TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS lms_skills (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS lms_skill_matrix (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    skill_id TEXT NOT NULL,
    proficiency_level INTEGER NOT NULL,
    assessed_at TEXT NOT NULL,
    assessed_by TEXT,
    notes TEXT,
    UNIQUE(employee_id, skill_id)
);

CREATE TABLE IF NOT EXISTS lms_training_records (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    training_type TEXT NOT NULL,
    training_name TEXT NOT NULL,
    provider TEXT,
    completed_at TEXT NOT NULL,
    hours REAL NOT NULL,
    credits INTEGER NOT NULL DEFAULT 0,
    certificate_number TEXT,
    cost INTEGER NOT NULL DEFAULT 0
);
