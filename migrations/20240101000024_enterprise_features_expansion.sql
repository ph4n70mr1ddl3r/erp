-- Multi-company/Entity Support
CREATE TABLE IF NOT EXISTS companies (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    legal_name TEXT NOT NULL,
    company_type TEXT NOT NULL DEFAULT 'Subsidiary',
    parent_id TEXT REFERENCES companies(id),
    tax_id TEXT,
    registration_number TEXT,
    currency TEXT NOT NULL DEFAULT 'USD',
    fiscal_year_start INTEGER NOT NULL DEFAULT 1,
    consolidation_method TEXT NOT NULL DEFAULT 'Full',
    ownership_percentage REAL NOT NULL DEFAULT 100.0,
    street TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT,
    postal_code TEXT NOT NULL,
    country TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    website TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS business_units (
    id TEXT PRIMARY KEY,
    company_id TEXT NOT NULL REFERENCES companies(id),
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    manager_id TEXT,
    budget INTEGER,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    UNIQUE(company_id, code)
);

-- Intercompany Transactions
CREATE TABLE IF NOT EXISTS intercompany_transactions (
    id TEXT PRIMARY KEY,
    transaction_number TEXT NOT NULL UNIQUE,
    from_company_id TEXT NOT NULL REFERENCES companies(id),
    to_company_id TEXT NOT NULL REFERENCES companies(id),
    transaction_type TEXT NOT NULL,
    reference_type TEXT,
    reference_id TEXT,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    exchange_rate REAL NOT NULL DEFAULT 1.0,
    base_amount INTEGER NOT NULL,
    description TEXT NOT NULL,
    due_date TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    elimination_entry_id TEXT,
    created_at TEXT NOT NULL,
    created_by TEXT
);

-- Consolidations
CREATE TABLE IF NOT EXISTS consolidations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    total_eliminations INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    created_by TEXT
);

CREATE TABLE IF NOT EXISTS consolidation_entries (
    id TEXT PRIMARY KEY,
    consolidation_id TEXT NOT NULL REFERENCES consolidations(id),
    company_id TEXT NOT NULL REFERENCES companies(id),
    account_code TEXT NOT NULL,
    debit INTEGER NOT NULL DEFAULT 0,
    credit INTEGER NOT NULL DEFAULT 0,
    elimination_type TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Subscription Plans
CREATE TABLE IF NOT EXISTS subscription_plans (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    price INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    billing_interval TEXT NOT NULL DEFAULT 'Monthly',
    interval_count INTEGER NOT NULL DEFAULT 1,
    trial_days INTEGER NOT NULL DEFAULT 0,
    features TEXT NOT NULL DEFAULT '{}',
    max_users INTEGER,
    max_transactions INTEGER,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Subscriptions
CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    plan_id TEXT NOT NULL REFERENCES subscription_plans(id),
    status TEXT NOT NULL DEFAULT 'Pending',
    quantity INTEGER NOT NULL DEFAULT 1,
    price_override INTEGER,
    current_period_start TEXT NOT NULL,
    current_period_end TEXT NOT NULL,
    trial_start TEXT,
    trial_end TEXT,
    cancelled_at TEXT,
    cancel_at_period_end INTEGER NOT NULL DEFAULT 0,
    metadata TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subscription_items (
    id TEXT PRIMARY KEY,
    subscription_id TEXT NOT NULL REFERENCES subscriptions(id),
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price INTEGER NOT NULL,
    discount_percent REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subscription_invoices (
    id TEXT PRIMARY KEY,
    subscription_id TEXT NOT NULL REFERENCES subscriptions(id),
    invoice_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subscription_usage (
    id TEXT PRIMARY KEY,
    subscription_id TEXT NOT NULL REFERENCES subscriptions(id),
    usage_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit TEXT NOT NULL,
    recorded_at TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS metered_prices (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL REFERENCES subscription_plans(id),
    meter_type TEXT NOT NULL,
    unit_price INTEGER NOT NULL,
    included_units INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL
);

-- Carriers
CREATE TABLE IF NOT EXISTS carriers (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    carrier_type TEXT NOT NULL DEFAULT 'Other',
    api_key TEXT,
    api_secret TEXT,
    account_number TEXT,
    tracking_url_template TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS carrier_services (
    id TEXT PRIMARY KEY,
    carrier_id TEXT NOT NULL REFERENCES carriers(id),
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    service_type TEXT NOT NULL,
    estimated_days_min INTEGER NOT NULL DEFAULT 1,
    estimated_days_max INTEGER NOT NULL DEFAULT 7,
    base_rate INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    UNIQUE(carrier_id, code)
);

-- Shipments
CREATE TABLE IF NOT EXISTS shipments (
    id TEXT PRIMARY KEY,
    shipment_number TEXT NOT NULL UNIQUE,
    order_id TEXT,
    carrier_id TEXT NOT NULL REFERENCES carriers(id),
    carrier_service_id TEXT NOT NULL REFERENCES carrier_services(id),
    status TEXT NOT NULL DEFAULT 'Pending',
    ship_from_name TEXT NOT NULL,
    ship_from_street TEXT NOT NULL,
    ship_from_city TEXT NOT NULL,
    ship_from_state TEXT,
    ship_from_postal_code TEXT NOT NULL,
    ship_from_country TEXT NOT NULL,
    ship_to_name TEXT NOT NULL,
    ship_to_street TEXT NOT NULL,
    ship_to_city TEXT NOT NULL,
    ship_to_state TEXT,
    ship_to_postal_code TEXT NOT NULL,
    ship_to_country TEXT NOT NULL,
    ship_to_phone TEXT,
    weight REAL NOT NULL DEFAULT 0,
    weight_unit TEXT NOT NULL DEFAULT 'lb',
    length REAL NOT NULL DEFAULT 0,
    width REAL NOT NULL DEFAULT 0,
    height REAL NOT NULL DEFAULT 0,
    dimension_unit TEXT NOT NULL DEFAULT 'in',
    shipping_cost INTEGER NOT NULL DEFAULT 0,
    insurance_cost INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    tracking_number TEXT,
    tracking_url TEXT,
    label_url TEXT,
    shipped_at TEXT,
    estimated_delivery TEXT,
    delivered_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS shipment_items (
    id TEXT PRIMARY KEY,
    shipment_id TEXT NOT NULL REFERENCES shipments(id),
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    weight REAL NOT NULL DEFAULT 0,
    declared_value INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tracking_events (
    id TEXT PRIMARY KEY,
    shipment_id TEXT NOT NULL REFERENCES shipments(id),
    event_type TEXT NOT NULL,
    description TEXT NOT NULL,
    location TEXT,
    occurred_at TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Shipping Zones and Rates
CREATE TABLE IF NOT EXISTS shipping_zones (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    countries TEXT NOT NULL,
    states TEXT,
    postal_codes TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS shipping_rates (
    id TEXT PRIMARY KEY,
    zone_id TEXT NOT NULL REFERENCES shipping_zones(id),
    carrier_service_id TEXT NOT NULL REFERENCES carrier_services(id),
    min_weight REAL NOT NULL DEFAULT 0,
    max_weight REAL NOT NULL DEFAULT 999999,
    min_value INTEGER NOT NULL DEFAULT 0,
    max_value INTEGER NOT NULL DEFAULT 999999999,
    rate INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

-- Payment Gateways
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

-- Payments
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    payment_number TEXT NOT NULL UNIQUE,
    gateway_id TEXT REFERENCES payment_gateways(id),
    invoice_id TEXT,
    customer_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    payment_method TEXT NOT NULL DEFAULT 'Other',
    status TEXT NOT NULL DEFAULT 'Pending',
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
    payment_id TEXT NOT NULL REFERENCES payments(id),
    invoice_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS refunds (
    id TEXT PRIMARY KEY,
    refund_number TEXT NOT NULL UNIQUE,
    payment_id TEXT NOT NULL REFERENCES payments(id),
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    reason TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
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
    gateway_id TEXT NOT NULL REFERENCES payment_gateways(id),
    total_amount INTEGER NOT NULL DEFAULT 0,
    total_count INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Open',
    settled_at TEXT,
    settlement_reference TEXT,
    created_at TEXT NOT NULL
);

-- Risk Management
CREATE TABLE IF NOT EXISTS risks (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'Operational',
    status TEXT NOT NULL DEFAULT 'Identified',
    probability REAL NOT NULL DEFAULT 0.5,
    impact TEXT NOT NULL DEFAULT 'Medium',
    risk_score INTEGER NOT NULL DEFAULT 0,
    inherent_risk_level TEXT NOT NULL DEFAULT 'Medium',
    residual_risk_level TEXT NOT NULL DEFAULT 'Medium',
    owner_id TEXT,
    department TEXT,
    identified_date TEXT NOT NULL,
    target_resolution_date TEXT,
    actual_resolution_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT
);

CREATE TABLE IF NOT EXISTS risk_assessments (
    id TEXT PRIMARY KEY,
    risk_id TEXT NOT NULL REFERENCES risks(id),
    assessment_date TEXT NOT NULL,
    assessor_id TEXT,
    probability_before REAL NOT NULL,
    impact_before TEXT NOT NULL,
    probability_after REAL NOT NULL,
    impact_after TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mitigation_plans (
    id TEXT PRIMARY KEY,
    risk_id TEXT NOT NULL REFERENCES risks(id),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    strategy TEXT NOT NULL,
    owner_id TEXT,
    status TEXT NOT NULL DEFAULT 'Planned',
    priority TEXT NOT NULL DEFAULT 'Medium',
    start_date TEXT NOT NULL,
    target_date TEXT NOT NULL,
    completion_date TEXT,
    budget INTEGER NOT NULL DEFAULT 0,
    actual_cost INTEGER NOT NULL DEFAULT 0,
    effectiveness REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mitigation_tasks (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL REFERENCES mitigation_plans(id),
    title TEXT NOT NULL,
    description TEXT,
    assigned_to TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    due_date TEXT NOT NULL,
    completed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS risk_controls (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    control_type TEXT NOT NULL DEFAULT 'Preventive',
    frequency TEXT NOT NULL DEFAULT 'Continuous',
    owner_id TEXT,
    effectiveness TEXT NOT NULL DEFAULT 'Effective',
    last_test_date TEXT,
    next_test_date TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS risk_control_mappings (
    id TEXT PRIMARY KEY,
    risk_id TEXT NOT NULL REFERENCES risks(id),
    control_id TEXT NOT NULL REFERENCES risk_controls(id),
    control_effectiveness TEXT NOT NULL DEFAULT 'Effective',
    created_at TEXT NOT NULL,
    UNIQUE(risk_id, control_id)
);

CREATE TABLE IF NOT EXISTS risk_incidents (
    id TEXT PRIMARY KEY,
    risk_id TEXT NOT NULL REFERENCES risks(id),
    incident_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    incident_date TEXT NOT NULL,
    detected_date TEXT NOT NULL,
    reported_by TEXT,
    impact_amount INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Open',
    root_cause TEXT,
    lessons_learned TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS risk_registers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    department TEXT,
    owner_id TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

-- Warehouse Management (Zones, Bins, Locations)
CREATE TABLE IF NOT EXISTS warehouse_zones (
    id TEXT PRIMARY KEY,
    warehouse_id TEXT NOT NULL,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    zone_type TEXT NOT NULL DEFAULT 'Storage',
    temperature_controlled INTEGER NOT NULL DEFAULT 0,
    min_temperature REAL,
    max_temperature REAL,
    capacity INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(warehouse_id, code)
);

CREATE TABLE IF NOT EXISTS warehouse_locations (
    id TEXT PRIMARY KEY,
    zone_id TEXT NOT NULL REFERENCES warehouse_zones(id),
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    aisle TEXT,
    rack TEXT,
    shelf TEXT,
    bin TEXT,
    location_type TEXT NOT NULL DEFAULT 'Standard',
    capacity INTEGER NOT NULL DEFAULT 0,
    current_quantity INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(zone_id, code)
);

CREATE TABLE IF NOT EXISTS stock_location_quantities (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    location_id TEXT NOT NULL REFERENCES warehouse_locations(id),
    quantity INTEGER NOT NULL DEFAULT 0,
    reserved_quantity INTEGER NOT NULL DEFAULT 0,
    lot_number TEXT,
    expiry_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(product_id, location_id, lot_number)
);

CREATE TABLE IF NOT EXISTS putaway_tasks (
    id TEXT PRIMARY KEY,
    task_number TEXT NOT NULL UNIQUE,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    source_location TEXT,
    suggested_location TEXT,
    actual_location TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    priority INTEGER NOT NULL DEFAULT 0,
    assigned_to TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pick_tasks (
    id TEXT PRIMARY KEY,
    task_number TEXT NOT NULL UNIQUE,
    order_id TEXT,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    location_id TEXT REFERENCES warehouse_locations(id),
    status TEXT NOT NULL DEFAULT 'Pending',
    priority INTEGER NOT NULL DEFAULT 0,
    assigned_to TEXT,
    picked_quantity INTEGER NOT NULL DEFAULT 0,
    completed_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS replenishment_tasks (
    id TEXT PRIMARY KEY,
    task_number TEXT NOT NULL UNIQUE,
    product_id TEXT NOT NULL,
    from_location TEXT NOT NULL,
    to_location TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    priority INTEGER NOT NULL DEFAULT 0,
    assigned_to TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL
);

-- Create indexes for new tables
CREATE INDEX IF NOT EXISTS idx_companies_parent ON companies(parent_id);
CREATE INDEX IF NOT EXISTS idx_intercompany_from ON intercompany_transactions(from_company_id);
CREATE INDEX IF NOT EXISTS idx_intercompany_to ON intercompany_transactions(to_company_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_customer ON subscriptions(customer_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_plan ON subscriptions(plan_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);
CREATE INDEX IF NOT EXISTS idx_shipments_carrier ON shipments(carrier_id);
CREATE INDEX IF NOT EXISTS idx_shipments_status ON shipments(status);
CREATE INDEX IF NOT EXISTS idx_tracking_shipment ON tracking_events(shipment_id);
CREATE INDEX IF NOT EXISTS idx_payments_customer ON payments(customer_id);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments(status);
CREATE INDEX IF NOT EXISTS idx_risks_status ON risks(status);
CREATE INDEX IF NOT EXISTS idx_risks_category ON risks(category);
CREATE INDEX IF NOT EXISTS idx_warehouse_zones_wh ON warehouse_zones(warehouse_id);
CREATE INDEX IF NOT EXISTS idx_stock_loc_product ON stock_location_quantities(product_id);
CREATE INDEX IF NOT EXISTS idx_pick_tasks_status ON pick_tasks(status);
