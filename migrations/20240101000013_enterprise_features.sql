-- ============================================
-- ENTERPRISE ERP FEATURES - COMPREHENSIVE
-- ============================================

-- ============================================
-- FINANCIAL MANAGEMENT
-- ============================================

-- Bank Accounts & Reconciliation
CREATE TABLE IF NOT EXISTS bank_accounts (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    bank_name TEXT NOT NULL,
    account_number TEXT NOT NULL,
    account_type TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    gl_code TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

CREATE TABLE IF NOT EXISTS bank_statements (
    id TEXT PRIMARY KEY,
    bank_account_id TEXT NOT NULL,
    statement_date TEXT NOT NULL,
    opening_balance INTEGER NOT NULL,
    closing_balance INTEGER NOT NULL,
    status TEXT DEFAULT 'Pending',
    reconciled_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (bank_account_id) REFERENCES bank_accounts(id)
);

CREATE TABLE IF NOT EXISTS bank_transactions (
    id TEXT PRIMARY KEY,
    bank_account_id TEXT NOT NULL,
    statement_id TEXT,
    transaction_date TEXT NOT NULL,
    value_date TEXT,
    description TEXT,
    reference TEXT,
    debit INTEGER DEFAULT 0,
    credit INTEGER DEFAULT 0,
    balance INTEGER NOT NULL,
    reconciled INTEGER DEFAULT 0,
    journal_entry_id TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (bank_account_id) REFERENCES bank_accounts(id),
    FOREIGN KEY (statement_id) REFERENCES bank_statements(id)
);

CREATE TABLE IF NOT EXISTS reconciliation_rules (
    id TEXT PRIMARY KEY,
    bank_account_id TEXT NOT NULL,
    rule_type TEXT NOT NULL,
    match_field TEXT NOT NULL,
    match_pattern TEXT,
    tolerance_days INTEGER DEFAULT 0,
    tolerance_amount INTEGER DEFAULT 0,
    auto_match INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);

-- Cash Flow Management
CREATE TABLE IF NOT EXISTS cash_flow_forecasts (
    id TEXT PRIMARY KEY,
    forecast_date TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    opening_balance INTEGER NOT NULL,
    expected_inflows INTEGER DEFAULT 0,
    expected_outflows INTEGER DEFAULT 0,
    closing_balance INTEGER NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_flow_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category_type TEXT NOT NULL,
    parent_id TEXT,
    sort_order INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS cash_flow_items (
    id TEXT PRIMARY KEY,
    forecast_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    description TEXT NOT NULL,
    expected_date TEXT,
    amount INTEGER NOT NULL,
    probability INTEGER DEFAULT 100,
    actual_amount INTEGER,
    actual_date TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (forecast_id) REFERENCES cash_flow_forecasts(id)
);

-- Cost Accounting (Activity-Based)
CREATE TABLE IF NOT EXISTS cost_centers (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    department_id TEXT,
    manager_id TEXT,
    cost_center_type TEXT NOT NULL,
    allocation_method TEXT DEFAULT 'Direct',
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cost_elements (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    element_type TEXT NOT NULL,
    account_id TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cost_pools (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    cost_center_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_cost INTEGER DEFAULT 0,
    allocation_base TEXT NOT NULL,
    allocation_rate REAL DEFAULT 0,
    status TEXT DEFAULT 'Open',
    created_at TEXT NOT NULL,
    FOREIGN KEY (cost_center_id) REFERENCES cost_centers(id)
);

CREATE TABLE IF NOT EXISTS cost_allocations (
    id TEXT PRIMARY KEY,
    pool_id TEXT NOT NULL,
    from_cost_center_id TEXT NOT NULL,
    to_cost_center_id TEXT NOT NULL,
    allocation_base_value REAL NOT NULL,
    allocated_amount INTEGER NOT NULL,
    allocated_at TEXT NOT NULL,
    FOREIGN KEY (pool_id) REFERENCES cost_pools(id),
    FOREIGN KEY (from_cost_center_id) REFERENCES cost_centers(id),
    FOREIGN KEY (to_cost_center_id) REFERENCES cost_centers(id)
);

CREATE TABLE IF NOT EXISTS activity_types (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    cost_driver TEXT NOT NULL,
    unit_of_measure TEXT,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS activity_costs (
    id TEXT PRIMARY KEY,
    activity_type_id TEXT NOT NULL,
    cost_pool_id TEXT NOT NULL,
    total_activities INTEGER NOT NULL,
    cost_per_activity INTEGER NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    FOREIGN KEY (activity_type_id) REFERENCES activity_types(id),
    FOREIGN KEY (cost_pool_id) REFERENCES cost_pools(id)
);

-- Inter-company Transactions
CREATE TABLE IF NOT EXISTS companies (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    legal_name TEXT,
    tax_id TEXT,
    registration_number TEXT,
    currency TEXT NOT NULL DEFAULT 'USD',
    address TEXT,
    city TEXT,
    country TEXT,
    is_consolidation_entity INTEGER DEFAULT 0,
    parent_company_id TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS intercompany_transactions (
    id TEXT PRIMARY KEY,
    transaction_number TEXT NOT NULL UNIQUE,
    from_company_id TEXT NOT NULL,
    to_company_id TEXT NOT NULL,
    transaction_date TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    description TEXT,
    reference TEXT,
    from_journal_entry_id TEXT,
    to_journal_entry_id TEXT,
    status TEXT DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (from_company_id) REFERENCES companies(id),
    FOREIGN KEY (to_company_id) REFERENCES companies(id)
);

CREATE TABLE IF NOT EXISTS intercompany_accounts (
    id TEXT PRIMARY KEY,
    company_id TEXT NOT NULL,
    partner_company_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    due_to_account_id TEXT NOT NULL,
    due_from_account_id TEXT NOT NULL,
    FOREIGN KEY (company_id) REFERENCES companies(id),
    FOREIGN KEY (partner_company_id) REFERENCES companies(id)
);

-- Deferred Revenue Recognition
CREATE TABLE IF NOT EXISTS revenue_schedules (
    id TEXT PRIMARY KEY,
    schedule_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    recognition_method TEXT NOT NULL,
    total_amount INTEGER NOT NULL,
    recognized_amount INTEGER DEFAULT 0,
    deferred_amount INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS revenue_schedule_lines (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    recognition_date TEXT NOT NULL,
    amount INTEGER NOT NULL,
    recognized INTEGER DEFAULT 0,
    journal_entry_id TEXT,
    recognized_at TEXT,
    FOREIGN KEY (schedule_id) REFERENCES revenue_schedules(id)
);

CREATE TABLE IF NOT EXISTS revenue_recognition_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    recognition_type TEXT NOT NULL,
    periods INTEGER NOT NULL,
    recognition_rule TEXT NOT NULL,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

-- Multi-entity Consolidation
CREATE TABLE IF NOT EXISTS consolidation_schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    parent_company_id TEXT NOT NULL,
    status TEXT DEFAULT 'Draft',
    elimination_entries INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (parent_company_id) REFERENCES companies(id)
);

CREATE TABLE IF NOT EXISTS consolidation_companies (
    id TEXT PRIMARY KEY,
    consolidation_id TEXT NOT NULL,
    company_id TEXT NOT NULL,
    ownership_percent REAL DEFAULT 100,
    consolidation_method TEXT DEFAULT 'Full',
    exchange_rate REAL DEFAULT 1,
    translation_method TEXT DEFAULT 'Current',
    FOREIGN KEY (consolidation_id) REFERENCES consolidation_schedules(id),
    FOREIGN KEY (company_id) REFERENCES companies(id)
);

CREATE TABLE IF NOT EXISTS elimination_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    from_account_pattern TEXT NOT NULL,
    to_account_pattern TEXT NOT NULL,
    elimination_account_id TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS elimination_entries (
    id TEXT PRIMARY KEY,
    consolidation_id TEXT NOT NULL,
    elimination_rule_id TEXT,
    description TEXT NOT NULL,
    debit_account_id TEXT NOT NULL,
    credit_account_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    journal_entry_id TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (consolidation_id) REFERENCES consolidation_schedules(id)
);

-- ============================================
-- SUPPLY CHAIN MANAGEMENT
-- ============================================

-- Advanced WMS
CREATE TABLE IF NOT EXISTS warehouse_zones (
    id TEXT PRIMARY KEY,
    warehouse_id TEXT NOT NULL,
    zone_code TEXT NOT NULL,
    name TEXT NOT NULL,
    zone_type TEXT NOT NULL,
    temperature_controlled INTEGER DEFAULT 0,
    max_capacity INTEGER,
    current_utilization INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS warehouse_bins (
    id TEXT PRIMARY KEY,
    zone_id TEXT NOT NULL,
    bin_code TEXT NOT NULL,
    bin_type TEXT NOT NULL,
    aisle TEXT,
    row_number INTEGER,
    level_number INTEGER,
    capacity INTEGER,
    current_quantity INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    FOREIGN KEY (zone_id) REFERENCES warehouse_zones(id)
);

CREATE TABLE IF NOT EXISTS pick_lists (
    id TEXT PRIMARY KEY,
    pick_number TEXT NOT NULL UNIQUE,
    warehouse_id TEXT NOT NULL,
    order_id TEXT,
    assigned_to TEXT,
    priority INTEGER DEFAULT 5,
    status TEXT DEFAULT 'Pending',
    total_items INTEGER DEFAULT 0,
    picked_items INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    completed_at TEXT
);

CREATE TABLE IF NOT EXISTS pick_list_items (
    id TEXT PRIMARY KEY,
    pick_list_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    bin_id TEXT NOT NULL,
    lot_id TEXT,
    requested_qty INTEGER NOT NULL,
    picked_qty INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Pending',
    FOREIGN KEY (pick_list_id) REFERENCES pick_lists(id)
);

CREATE TABLE IF NOT EXISTS pack_lists (
    id TEXT PRIMARY KEY,
    pack_number TEXT NOT NULL UNIQUE,
    pick_list_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    packed_by TEXT,
    status TEXT DEFAULT 'Pending',
    total_weight INTEGER,
    tracking_number TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (pick_list_id) REFERENCES pick_lists(id)
);

CREATE TABLE IF NOT EXISTS pack_list_items (
    id TEXT PRIMARY KEY,
    pack_list_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    box_number INTEGER DEFAULT 1,
    FOREIGN KEY (pack_list_id) REFERENCES pack_lists(id)
);

CREATE TABLE IF NOT EXISTS shipment_orders (
    id TEXT PRIMARY KEY,
    shipment_number TEXT NOT NULL UNIQUE,
    warehouse_id TEXT NOT NULL,
    carrier_id TEXT,
    service_type TEXT,
    ship_to_name TEXT NOT NULL,
    ship_to_address TEXT NOT NULL,
    ship_to_city TEXT NOT NULL,
    ship_to_state TEXT,
    ship_to_postal TEXT NOT NULL,
    ship_to_country TEXT NOT NULL,
    total_weight INTEGER,
    tracking_number TEXT,
    ship_date TEXT,
    estimated_delivery TEXT,
    actual_delivery TEXT,
    status TEXT DEFAULT 'Draft',
    freight_charge INTEGER DEFAULT 0,
    insurance_charge INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS shipment_items (
    id TEXT PRIMARY KEY,
    shipment_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    weight INTEGER,
    foreign_key TEXT,
    FOREIGN KEY (shipment_id) REFERENCES shipment_orders(id)
);

-- Shipping Carriers
CREATE TABLE IF NOT EXISTS shipping_carriers (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    api_endpoint TEXT,
    api_key TEXT,
    account_number TEXT,
    supports_tracking INTEGER DEFAULT 1,
    supports_label_generation INTEGER DEFAULT 1,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS carrier_services (
    id TEXT PRIMARY KEY,
    carrier_id TEXT NOT NULL,
    service_code TEXT NOT NULL,
    service_name TEXT NOT NULL,
    delivery_days INTEGER,
    status TEXT DEFAULT 'Active',
    FOREIGN KEY (carrier_id) REFERENCES shipping_carriers(id)
);

CREATE TABLE IF NOT EXISTS shipping_rate_cards (
    id TEXT PRIMARY KEY,
    carrier_id TEXT NOT NULL,
    service_id TEXT NOT NULL,
    zone_from TEXT NOT NULL,
    zone_to TEXT NOT NULL,
    weight_from INTEGER NOT NULL,
    weight_to INTEGER NOT NULL,
    base_rate INTEGER NOT NULL,
    per_kg_rate INTEGER DEFAULT 0,
    effective_date TEXT NOT NULL,
    expiry_date TEXT,
    FOREIGN KEY (carrier_id) REFERENCES shipping_carriers(id),
    FOREIGN KEY (service_id) REFERENCES carrier_services(id)
);

-- EDI Integration
CREATE TABLE IF NOT EXISTS edi_partners (
    id TEXT PRIMARY KEY,
    partner_code TEXT NOT NULL UNIQUE,
    partner_name TEXT NOT NULL,
    partner_type TEXT NOT NULL,
    edi_standard TEXT DEFAULT 'X12',
    communication_method TEXT NOT NULL,
    ftp_host TEXT,
    ftp_username TEXT,
    ftp_password TEXT,
    api_endpoint TEXT,
    api_key TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS edi_documents (
    id TEXT PRIMARY KEY,
    document_number TEXT NOT NULL UNIQUE,
    partner_id TEXT NOT NULL,
    document_type TEXT NOT NULL,
    direction TEXT NOT NULL,
    reference_number TEXT,
    raw_content TEXT,
    parsed_data TEXT,
    status TEXT DEFAULT 'Pending',
    processed_at TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (partner_id) REFERENCES edi_partners(id)
);

CREATE TABLE IF NOT EXISTS edi_mappings (
    id TEXT PRIMARY KEY,
    partner_id TEXT NOT NULL,
    document_type TEXT NOT NULL,
    segment_id TEXT NOT NULL,
    element_position INTEGER NOT NULL,
    internal_field TEXT NOT NULL,
    transformation_rule TEXT,
    FOREIGN KEY (partner_id) REFERENCES edi_partners(id)
);

-- Supplier Portal
CREATE TABLE IF NOT EXISTS supplier_users (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT DEFAULT 'Supplier',
    last_login TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (vendor_id) REFERENCES vendors(id)
);

CREATE TABLE IF NOT EXISTS supplier_invitations (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    email TEXT NOT NULL,
    invitation_token TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    accepted_at TEXT,
    status TEXT DEFAULT 'Pending',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS supplier_documents (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    document_type TEXT NOT NULL,
    document_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    uploaded_by TEXT,
    expiry_date TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

-- RFQ Management
CREATE TABLE IF NOT EXISTS rfqs (
    id TEXT PRIMARY KEY,
    rfq_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    buyer_id TEXT NOT NULL,
    currency TEXT DEFAULT 'USD',
    submission_deadline TEXT NOT NULL,
    valid_until TEXT,
    status TEXT DEFAULT 'Draft',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS rfq_lines (
    id TEXT PRIMARY KEY,
    rfq_id TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    product_id TEXT,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit TEXT NOT NULL,
    delivery_date TEXT,
    specifications TEXT,
    FOREIGN KEY (rfq_id) REFERENCES rfqs(id)
);

CREATE TABLE IF NOT EXISTS rfq_vendors (
    id TEXT PRIMARY KEY,
    rfq_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    invited_at TEXT,
    responded_at TEXT,
    status TEXT DEFAULT 'Invited',
    FOREIGN KEY (rfq_id) REFERENCES rfqs(id),
    FOREIGN KEY (vendor_id) REFERENCES vendors(id)
);

CREATE TABLE IF NOT EXISTS rfq_responses (
    id TEXT PRIMARY KEY,
    rfq_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    response_number TEXT NOT NULL UNIQUE,
    response_date TEXT NOT NULL,
    valid_until TEXT,
    payment_terms INTEGER,
    delivery_terms TEXT,
    notes TEXT,
    status TEXT DEFAULT 'Submitted',
    created_at TEXT NOT NULL,
    FOREIGN KEY (rfq_id) REFERENCES rfqs(id),
    FOREIGN KEY (vendor_id) REFERENCES vendors(id)
);

CREATE TABLE IF NOT EXISTS rfq_response_lines (
    id TEXT PRIMARY KEY,
    response_id TEXT NOT NULL,
    rfq_line_id TEXT NOT NULL,
    unit_price INTEGER NOT NULL,
    lead_time_days INTEGER,
    minimum_order_qty INTEGER,
    notes TEXT,
    FOREIGN KEY (response_id) REFERENCES rfq_responses(id)
);

-- ============================================
-- MANUFACTURING
-- ============================================

-- Advanced Planning & Scheduling
CREATE TABLE IF NOT EXISTS mrp_runs (
    id TEXT PRIMARY KEY,
    run_number TEXT NOT NULL UNIQUE,
    run_date TEXT NOT NULL,
    planning_horizon_days INTEGER NOT NULL,
    status TEXT DEFAULT 'Running',
    created_at TEXT NOT NULL,
    completed_at TEXT
);

CREATE TABLE IF NOT EXISTS mrp_planned_orders (
    id TEXT PRIMARY KEY,
    mrp_run_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    order_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    due_date TEXT NOT NULL,
    release_date TEXT,
    source_type TEXT,
    source_id TEXT,
    status TEXT DEFAULT 'Planned',
    FOREIGN KEY (mrp_run_id) REFERENCES mrp_runs(id)
);

CREATE TABLE IF NOT EXISTS capacity_plans (
    id TEXT PRIMARY KEY,
    plan_number TEXT NOT NULL UNIQUE,
    work_center_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    available_hours INTEGER NOT NULL,
    planned_hours INTEGER DEFAULT 0,
    actual_hours INTEGER DEFAULT 0,
    utilization_percent REAL DEFAULT 0,
    status TEXT DEFAULT 'Planned',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS capacity_requirements (
    id TEXT PRIMARY KEY,
    capacity_plan_id TEXT NOT NULL,
    work_order_id TEXT NOT NULL,
    operation_sequence INTEGER NOT NULL,
    required_hours INTEGER NOT NULL,
    scheduled_start TEXT,
    scheduled_end TEXT,
    status TEXT DEFAULT 'Planned',
    FOREIGN KEY (capacity_plan_id) REFERENCES capacity_plans(id)
);

-- MES (Manufacturing Execution System)
CREATE TABLE IF NOT EXISTS shop_floor_operations (
    id TEXT PRIMARY KEY,
    operation_number TEXT NOT NULL UNIQUE,
    work_order_id TEXT NOT NULL,
    work_center_id TEXT NOT NULL,
    operation_code TEXT NOT NULL,
    description TEXT,
    setup_time INTEGER DEFAULT 0,
    run_time INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    completed_qty INTEGER DEFAULT 0,
    scrapped_qty INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Pending',
    started_at TEXT,
    completed_at TEXT,
    operator_id TEXT,
    FOREIGN KEY (work_order_id) REFERENCES work_orders(id)
);

CREATE TABLE IF NOT EXISTS shop_floor_logs (
    id TEXT PRIMARY KEY,
    operation_id TEXT NOT NULL,
    log_type TEXT NOT NULL,
    operator_id TEXT,
    quantity INTEGER,
    reason TEXT,
    notes TEXT,
    logged_at TEXT NOT NULL,
    FOREIGN KEY (operation_id) REFERENCES shop_floor_operations(id)
);

CREATE TABLE IF NOT EXISTS downtime_events (
    id TEXT PRIMARY KEY,
    work_center_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    reason_code TEXT,
    description TEXT,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    duration_minutes INTEGER,
    FOREIGN KEY (work_center_id) REFERENCES work_centers(id)
);

-- CMMS (Computerized Maintenance Management)
CREATE TABLE IF NOT EXISTS equipment (
    id TEXT PRIMARY KEY,
    equipment_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    equipment_type TEXT NOT NULL,
    manufacturer TEXT,
    model TEXT,
    serial_number TEXT,
    installation_date TEXT,
    warranty_expiry TEXT,
    location TEXT,
    work_center_id TEXT,
    parent_equipment_id TEXT,
    status TEXT DEFAULT 'Active',
    criticality TEXT DEFAULT 'Medium',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS maintenance_schedules (
    id TEXT PRIMARY KEY,
    schedule_number TEXT NOT NULL UNIQUE,
    equipment_id TEXT NOT NULL,
    maintenance_type TEXT NOT NULL,
    frequency_type TEXT NOT NULL,
    frequency_value INTEGER NOT NULL,
    last_maintenance TEXT,
    next_maintenance TEXT NOT NULL,
    estimated_duration INTEGER,
    assigned_to TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (equipment_id) REFERENCES equipment(id)
);

CREATE TABLE IF NOT EXISTS maintenance_work_orders (
    id TEXT PRIMARY KEY,
    work_order_number TEXT NOT NULL UNIQUE,
    equipment_id TEXT NOT NULL,
    schedule_id TEXT,
    maintenance_type TEXT NOT NULL,
    priority TEXT DEFAULT 'Medium',
    description TEXT NOT NULL,
    requested_by TEXT,
    assigned_to TEXT,
    scheduled_date TEXT,
    started_at TEXT,
    completed_at TEXT,
    downtime_hours REAL DEFAULT 0,
    labor_hours REAL DEFAULT 0,
    parts_cost INTEGER DEFAULT 0,
    labor_cost INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Requested',
    created_at TEXT NOT NULL,
    FOREIGN KEY (equipment_id) REFERENCES equipment(id),
    FOREIGN KEY (schedule_id) REFERENCES maintenance_schedules(id)
);

CREATE TABLE IF NOT EXISTS maintenance_parts (
    id TEXT PRIMARY KEY,
    work_order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_cost INTEGER,
    total_cost INTEGER,
    FOREIGN KEY (work_order_id) REFERENCES maintenance_work_orders(id)
);

-- PLM (Product Lifecycle Management)
CREATE TABLE IF NOT EXISTS engineering_change_requests (
    id TEXT PRIMARY KEY,
    ecr_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    change_type TEXT NOT NULL,
    reason TEXT NOT NULL,
    impact_assessment TEXT,
    requested_by TEXT NOT NULL,
    priority TEXT DEFAULT 'Medium',
    status TEXT DEFAULT 'Draft',
    submitted_at TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ecr_items (
    id TEXT PRIMARY KEY,
    ecr_id TEXT NOT NULL,
    item_type TEXT NOT NULL,
    item_id TEXT NOT NULL,
    current_state TEXT,
    proposed_state TEXT,
    FOREIGN KEY (ecr_id) REFERENCES engineering_change_requests(id)
);

CREATE TABLE IF NOT EXISTS engineering_change_orders (
    id TEXT PRIMARY KEY,
    eco_number TEXT NOT NULL UNIQUE,
    ecr_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    effective_date TEXT NOT NULL,
    approved_by TEXT,
    approval_date TEXT,
    status TEXT DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    FOREIGN KEY (ecr_id) REFERENCES engineering_change_requests(id)
);

CREATE TABLE IF NOT EXISTS document_revisions (
    id TEXT PRIMARY KEY,
    document_number TEXT NOT NULL,
    revision TEXT NOT NULL,
    title TEXT NOT NULL,
    document_type TEXT NOT NULL,
    file_path TEXT,
    product_id TEXT,
    status TEXT DEFAULT 'Draft',
    approved_by TEXT,
    approved_at TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    UNIQUE(document_number, revision)
);

-- ============================================
-- SALES & CRM
-- ============================================

-- Sales Territories
CREATE TABLE IF NOT EXISTS sales_territories (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    parent_territory_id TEXT,
    manager_id TEXT,
    geography_type TEXT,
    geography_codes TEXT,
    target_revenue INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS territory_assignments (
    id TEXT PRIMARY KEY,
    territory_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    sales_rep_id TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    is_primary INTEGER DEFAULT 1,
    FOREIGN KEY (territory_id) REFERENCES sales_territories(id)
);

-- Sales Commissions
CREATE TABLE IF NOT EXISTS commission_plans (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    plan_type TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    expiry_date TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS commission_tiers (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    tier_number INTEGER NOT NULL,
    min_amount INTEGER NOT NULL,
    max_amount INTEGER,
    rate_percent REAL NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES commission_plans(id)
);

CREATE TABLE IF NOT EXISTS sales_rep_commissions (
    id TEXT PRIMARY KEY,
    sales_rep_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    gross_sales INTEGER DEFAULT 0,
    returns INTEGER DEFAULT 0,
    net_sales INTEGER DEFAULT 0,
    commission_rate REAL DEFAULT 0,
    commission_amount INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Calculated',
    paid_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES commission_plans(id)
);

CREATE TABLE IF NOT EXISTS commission_transactions (
    id TEXT PRIMARY KEY,
    commission_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    reference_type TEXT,
    reference_id TEXT,
    amount INTEGER NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (commission_id) REFERENCES sales_rep_commissions(id)
);

-- Contract Management
CREATE TABLE IF NOT EXISTS contracts (
    id TEXT PRIMARY KEY,
    contract_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    contract_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    value INTEGER NOT NULL,
    currency TEXT DEFAULT 'USD',
    billing_cycle TEXT,
    auto_renew INTEGER DEFAULT 0,
    renewal_notice_days INTEGER DEFAULT 30,
    terms TEXT,
    status TEXT DEFAULT 'Draft',
    signed_date TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS contract_lines (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    product_id TEXT,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    billing_type TEXT NOT NULL,
    billing_frequency TEXT,
    next_billing_date TEXT,
    status TEXT DEFAULT 'Active',
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_renewals (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    renewal_date TEXT NOT NULL,
    new_start_date TEXT NOT NULL,
    new_end_date TEXT NOT NULL,
    new_value INTEGER,
    status TEXT DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

-- Subscription / Recurring Billing
CREATE TABLE IF NOT EXISTS subscription_plans (
    id TEXT PRIMARY KEY,
    plan_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    billing_cycle TEXT NOT NULL,
    billing_interval INTEGER NOT NULL DEFAULT 1,
    setup_fee INTEGER DEFAULT 0,
    base_price INTEGER NOT NULL,
    trial_days INTEGER DEFAULT 0,
    features TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY,
    subscription_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    contract_id TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT,
    current_period_start TEXT NOT NULL,
    current_period_end TEXT NOT NULL,
    quantity INTEGER DEFAULT 1,
    price_override INTEGER,
    status TEXT DEFAULT 'Active',
    cancelled_at TEXT,
    cancellation_reason TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES subscription_plans(id),
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS subscription_usage (
    id TEXT PRIMARY KEY,
    subscription_id TEXT NOT NULL,
    usage_date TEXT NOT NULL,
    usage_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER,
    total_amount INTEGER,
    invoice_id TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (subscription_id) REFERENCES subscriptions(id)
);

CREATE TABLE IF NOT EXISTS subscription_invoices (
    id TEXT PRIMARY KEY,
    invoice_number TEXT NOT NULL UNIQUE,
    subscription_id TEXT NOT NULL,
    invoice_date TEXT NOT NULL,
    due_date TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    subtotal INTEGER NOT NULL,
    tax_amount INTEGER DEFAULT 0,
    total INTEGER NOT NULL,
    amount_paid INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Draft',
    paid_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (subscription_id) REFERENCES subscriptions(id)
);

-- Marketing Campaigns
CREATE TABLE IF NOT EXISTS marketing_campaigns (
    id TEXT PRIMARY KEY,
    campaign_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    campaign_type TEXT NOT NULL,
    channel TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,
    budget INTEGER NOT NULL,
    actual_spend INTEGER DEFAULT 0,
    target_audience TEXT,
    objectives TEXT,
    status TEXT DEFAULT 'Draft',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS campaign_leads (
    id TEXT PRIMARY KEY,
    campaign_id TEXT NOT NULL,
    lead_id TEXT NOT NULL,
    responded_at TEXT,
    response_type TEXT,
    converted INTEGER DEFAULT 0,
    conversion_value INTEGER,
    FOREIGN KEY (campaign_id) REFERENCES marketing_campaigns(id),
    FOREIGN KEY (lead_id) REFERENCES leads(id)
);

-- ============================================
-- PROJECT MANAGEMENT
-- ============================================

CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    project_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    customer_id TEXT,
    project_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,
    budget INTEGER DEFAULT 0,
    billable INTEGER DEFAULT 1,
    billing_method TEXT DEFAULT 'FixedPrice',
    project_manager TEXT,
    status TEXT DEFAULT 'Planning',
    percent_complete INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS project_tasks (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    task_number INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    parent_task_id TEXT,
    assigned_to TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT,
    estimated_hours REAL,
    actual_hours REAL DEFAULT 0,
    percent_complete INTEGER DEFAULT 0,
    priority TEXT DEFAULT 'Medium',
    status TEXT DEFAULT 'NotStarted',
    billable INTEGER DEFAULT 1,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS project_milestones (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    planned_date TEXT NOT NULL,
    actual_date TEXT,
    billing_amount INTEGER DEFAULT 0,
    billing_status TEXT DEFAULT 'NotBilled',
    status TEXT DEFAULT 'Planned',
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS project_expenses (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    expense_type TEXT NOT NULL,
    description TEXT NOT NULL,
    amount INTEGER NOT NULL,
    expense_date TEXT NOT NULL,
    billable INTEGER DEFAULT 1,
    invoiced INTEGER DEFAULT 0,
    invoice_id TEXT,
    status TEXT DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS timesheets (
    id TEXT PRIMARY KEY,
    timesheet_number TEXT NOT NULL UNIQUE,
    employee_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_hours REAL DEFAULT 0,
    overtime_hours REAL DEFAULT 0,
    status TEXT DEFAULT 'Draft',
    submitted_at TEXT,
    approved_at TEXT,
    approved_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS timesheet_entries (
    id TEXT PRIMARY KEY,
    timesheet_id TEXT NOT NULL,
    project_id TEXT,
    task_id TEXT,
    entry_date TEXT NOT NULL,
    hours REAL NOT NULL,
    description TEXT,
    billable INTEGER DEFAULT 1,
    hourly_rate INTEGER,
    FOREIGN KEY (timesheet_id) REFERENCES timesheets(id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS project_billings (
    id TEXT PRIMARY KEY,
    billing_number TEXT NOT NULL UNIQUE,
    project_id TEXT NOT NULL,
    billing_type TEXT NOT NULL,
    milestone_id TEXT,
    period_start TEXT,
    period_end TEXT,
    amount INTEGER NOT NULL,
    invoice_id TEXT,
    status TEXT DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- ============================================
-- HR & WORKFORCE
-- ============================================

-- Full Payroll
CREATE TABLE IF NOT EXISTS pay_grades (
    id TEXT PRIMARY KEY,
    grade_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    min_salary INTEGER NOT NULL,
    max_salary INTEGER NOT NULL,
    midpoint INTEGER,
    currency TEXT DEFAULT 'USD',
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS pay_components (
    id TEXT PRIMARY KEY,
    component_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    component_type TEXT NOT NULL,
    calculation_type TEXT NOT NULL,
    default_value INTEGER,
    taxable INTEGER DEFAULT 1,
    affects_gross INTEGER DEFAULT 1,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS employee_salaries (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    pay_grade_id TEXT,
    effective_date TEXT NOT NULL,
    base_salary INTEGER NOT NULL,
    salary_type TEXT DEFAULT 'Annual',
    currency TEXT DEFAULT 'USD',
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);

CREATE TABLE IF NOT EXISTS employee_components (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    component_id TEXT NOT NULL,
    value INTEGER NOT NULL,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    FOREIGN KEY (employee_id) REFERENCES employees(id),
    FOREIGN KEY (component_id) REFERENCES pay_components(id)
);

CREATE TABLE IF NOT EXISTS payroll_runs (
    id TEXT PRIMARY KEY,
    run_number TEXT NOT NULL UNIQUE,
    pay_period_start TEXT NOT NULL,
    pay_period_end TEXT NOT NULL,
    pay_date TEXT NOT NULL,
    total_gross INTEGER DEFAULT 0,
    total_deductions INTEGER DEFAULT 0,
    total_net INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Draft',
    processed_at TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS payroll_entries (
    id TEXT PRIMARY KEY,
    payroll_run_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    gross_pay INTEGER NOT NULL,
    total_deductions INTEGER NOT NULL,
    net_pay INTEGER NOT NULL,
    payment_method TEXT DEFAULT 'BankTransfer',
    bank_account TEXT,
    status TEXT DEFAULT 'Pending',
    FOREIGN KEY (payroll_run_id) REFERENCES payroll_runs(id),
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);

CREATE TABLE IF NOT EXISTS payroll_line_items (
    id TEXT PRIMARY KEY,
    payroll_entry_id TEXT NOT NULL,
    component_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    is_deduction INTEGER DEFAULT 0,
    FOREIGN KEY (payroll_entry_id) REFERENCES payroll_entries(id)
);

CREATE TABLE IF NOT EXISTS tax_tables (
    id TEXT PRIMARY KEY,
    tax_name TEXT NOT NULL,
    tax_type TEXT NOT NULL,
    year INTEGER NOT NULL,
    bracket_min INTEGER NOT NULL,
    bracket_max INTEGER,
    rate REAL NOT NULL,
    base_amount INTEGER DEFAULT 0
);

-- Benefits Administration
CREATE TABLE IF NOT EXISTS benefit_plans (
    id TEXT PRIMARY KEY,
    plan_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    plan_type TEXT NOT NULL,
    provider TEXT,
    coverage_type TEXT,
    employee_contribution INTEGER NOT NULL,
    employer_contribution INTEGER NOT NULL,
    max_dependents INTEGER DEFAULT 0,
    waiting_period_days INTEGER DEFAULT 0,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS employee_benefits (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    coverage_level TEXT,
    enrollment_date TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    termination_date TEXT,
    employee_cost INTEGER NOT NULL,
    employer_cost INTEGER NOT NULL,
    status TEXT DEFAULT 'Active',
    FOREIGN KEY (employee_id) REFERENCES employees(id),
    FOREIGN KEY (plan_id) REFERENCES benefit_plans(id)
);

CREATE TABLE IF NOT EXISTS benefit_dependents (
    id TEXT PRIMARY KEY,
    employee_benefit_id TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    relationship TEXT NOT NULL,
    birth_date TEXT,
    gender TEXT,
    FOREIGN KEY (employee_benefit_id) REFERENCES employee_benefits(id)
);

-- Performance Management
CREATE TABLE IF NOT EXISTS performance_cycles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    cycle_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    review_due_date TEXT NOT NULL,
    status TEXT DEFAULT 'Draft',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS performance_goals (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    cycle_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    weight INTEGER DEFAULT 1,
    target_value TEXT,
    actual_value TEXT,
    self_rating INTEGER,
    manager_rating INTEGER,
    final_rating INTEGER,
    status TEXT DEFAULT 'Draft',
    FOREIGN KEY (cycle_id) REFERENCES performance_cycles(id)
);

CREATE TABLE IF NOT EXISTS performance_reviews (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    reviewer_id TEXT NOT NULL,
    cycle_id TEXT NOT NULL,
    review_type TEXT NOT NULL,
    overall_rating INTEGER,
    strengths TEXT,
    areas_for_improvement TEXT,
    comments TEXT,
    submitted_at TEXT,
    status TEXT DEFAULT 'Draft',
    FOREIGN KEY (cycle_id) REFERENCES performance_cycles(id)
);

CREATE TABLE IF NOT EXISTS competency_ratings (
    id TEXT PRIMARY KEY,
    review_id TEXT NOT NULL,
    competency_name TEXT NOT NULL,
    rating INTEGER NOT NULL,
    comments TEXT,
    FOREIGN KEY (review_id) REFERENCES performance_reviews(id)
);

-- Training / LMS
CREATE TABLE IF NOT EXISTS training_courses (
    id TEXT PRIMARY KEY,
    course_code TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    category TEXT,
    duration_hours REAL,
    delivery_method TEXT NOT NULL,
    provider TEXT,
    cost INTEGER DEFAULT 0,
    required_for TEXT,
    certification_valid_days INTEGER,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS course_modules (
    id TEXT PRIMARY KEY,
    course_id TEXT NOT NULL,
    module_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    content_type TEXT NOT NULL,
    content_path TEXT,
    duration_minutes INTEGER,
    FOREIGN KEY (course_id) REFERENCES training_courses(id)
);

CREATE TABLE IF NOT EXISTS employee_training (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    course_id TEXT NOT NULL,
    enrollment_date TEXT NOT NULL,
    start_date TEXT,
    completion_date TEXT,
    due_date TEXT,
    score REAL,
    passed INTEGER DEFAULT 0,
    certificate_number TEXT,
    certificate_expiry TEXT,
    status TEXT DEFAULT 'Enrolled',
    FOREIGN KEY (employee_id) REFERENCES employees(id),
    FOREIGN KEY (course_id) REFERENCES training_courses(id)
);

CREATE TABLE IF NOT EXISTS training_sessions (
    id TEXT PRIMARY KEY,
    course_id TEXT NOT NULL,
    session_date TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    location TEXT,
    instructor TEXT,
    max_attendees INTEGER,
    current_attendees INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Scheduled',
    FOREIGN KEY (course_id) REFERENCES training_courses(id)
);

-- Recruiting / ATS
CREATE TABLE IF NOT EXISTS job_postings (
    id TEXT PRIMARY KEY,
    job_code TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    department_id TEXT,
    location TEXT,
    employment_type TEXT NOT NULL,
    min_salary INTEGER,
    max_salary INTEGER,
    description TEXT NOT NULL,
    requirements TEXT,
    posted_date TEXT,
    closing_date TEXT,
    openings INTEGER DEFAULT 1,
    filled INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Draft',
    hiring_manager TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS job_applications (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    resume_path TEXT,
    cover_letter TEXT,
    source TEXT,
    applied_at TEXT NOT NULL,
    status TEXT DEFAULT 'New',
    rating INTEGER,
    FOREIGN KEY (job_id) REFERENCES job_postings(id)
);

CREATE TABLE IF NOT EXISTS application_stages (
    id TEXT PRIMARY KEY,
    application_id TEXT NOT NULL,
    stage TEXT NOT NULL,
    entered_at TEXT NOT NULL,
    exited_at TEXT,
    notes TEXT,
    FOREIGN KEY (application_id) REFERENCES job_applications(id)
);

CREATE TABLE IF NOT EXISTS interviews (
    id TEXT PRIMARY KEY,
    application_id TEXT NOT NULL,
    interview_type TEXT NOT NULL,
    scheduled_at TEXT NOT NULL,
    duration_minutes INTEGER DEFAULT 60,
    interviewer TEXT NOT NULL,
    location TEXT,
    notes TEXT,
    feedback TEXT,
    rating INTEGER,
    status TEXT DEFAULT 'Scheduled',
    FOREIGN KEY (application_id) REFERENCES job_applications(id)
);

CREATE TABLE IF NOT EXISTS job_offers (
    id TEXT PRIMARY KEY,
    application_id TEXT NOT NULL,
    offer_date TEXT NOT NULL,
    salary INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    expiration_date TEXT,
    terms TEXT,
    status TEXT DEFAULT 'Pending',
    responded_at TEXT,
    FOREIGN KEY (application_id) REFERENCES job_applications(id)
);

-- ============================================
-- ANALYTICS & BI
-- ============================================

CREATE TABLE IF NOT EXISTS dashboards (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    dashboard_type TEXT NOT NULL,
    is_default INTEGER DEFAULT 0,
    layout TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS dashboard_widgets (
    id TEXT PRIMARY KEY,
    dashboard_id TEXT NOT NULL,
    widget_type TEXT NOT NULL,
    title TEXT NOT NULL,
    data_source TEXT NOT NULL,
    query_text TEXT,
    refresh_interval INTEGER DEFAULT 300,
    position_x INTEGER DEFAULT 0,
    position_y INTEGER DEFAULT 0,
    width INTEGER DEFAULT 1,
    height INTEGER DEFAULT 1,
    config TEXT,
    FOREIGN KEY (dashboard_id) REFERENCES dashboards(id)
);

CREATE TABLE IF NOT EXISTS kpi_definitions (
    id TEXT PRIMARY KEY,
    kpi_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    unit TEXT,
    target_value REAL,
    warning_threshold REAL,
    critical_threshold REAL,
    calculation_formula TEXT,
    data_source TEXT,
    refresh_frequency TEXT DEFAULT 'Daily',
    owner TEXT,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS kpi_values (
    id TEXT PRIMARY KEY,
    kpi_id TEXT NOT NULL,
    period TEXT NOT NULL,
    value REAL NOT NULL,
    target REAL,
    variance REAL,
    variance_percent REAL,
    trend TEXT,
    calculated_at TEXT NOT NULL,
    FOREIGN KEY (kpi_id) REFERENCES kpi_definitions(id)
);

CREATE TABLE IF NOT EXISTS alerts (
    id TEXT PRIMARY KEY,
    alert_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    source_entity TEXT,
    source_id TEXT,
    rule_id TEXT,
    acknowledged INTEGER DEFAULT 0,
    acknowledged_by TEXT,
    acknowledged_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS alert_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    entity_type TEXT NOT NULL,
    condition_field TEXT NOT NULL,
    operator TEXT NOT NULL,
    threshold_value TEXT NOT NULL,
    severity TEXT NOT NULL,
    notification_channels TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS forecast_models (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    model_type TEXT NOT NULL,
    target_entity TEXT NOT NULL,
    features TEXT,
    parameters TEXT,
    accuracy_score REAL,
    last_trained TEXT,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS predictions (
    id TEXT PRIMARY KEY,
    model_id TEXT NOT NULL,
    entity_id TEXT,
    prediction_date TEXT NOT NULL,
    predicted_value REAL NOT NULL,
    confidence_lower REAL,
    confidence_upper REAL,
    actual_value REAL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (model_id) REFERENCES forecast_models(id)
);

-- ============================================
-- PLATFORM & INTEGRATION
-- ============================================

-- Multi-tenancy
CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY,
    tenant_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    plan_type TEXT NOT NULL,
    max_users INTEGER DEFAULT 10,
    max_storage_mb INTEGER DEFAULT 1000,
    settings TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    expires_at TEXT
);

CREATE TABLE IF NOT EXISTS tenant_users (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT DEFAULT 'User',
    joined_at TEXT NOT NULL,
    status TEXT DEFAULT 'Active',
    UNIQUE(tenant_id, user_id),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

-- Workflow Automation
CREATE TABLE IF NOT EXISTS automation_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    entity_type TEXT NOT NULL,
    trigger_event TEXT NOT NULL,
    conditions TEXT,
    actions TEXT NOT NULL,
    priority INTEGER DEFAULT 5,
    active INTEGER DEFAULT 1,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS automation_logs (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL,
    entity_type TEXT,
    entity_id TEXT,
    trigger_data TEXT,
    action_results TEXT,
    success INTEGER DEFAULT 1,
    error_message TEXT,
    executed_at TEXT NOT NULL,
    FOREIGN KEY (rule_id) REFERENCES automation_rules(id)
);

-- Email Integration
CREATE TABLE IF NOT EXISTS email_templates (
    id TEXT PRIMARY KEY,
    template_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subject TEXT NOT NULL,
    body_html TEXT NOT NULL,
    body_text TEXT,
    category TEXT,
    variables TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_queues (
    id TEXT PRIMARY KEY,
    template_id TEXT,
    to_address TEXT NOT NULL,
    cc_addresses TEXT,
    bcc_addresses TEXT,
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    attachments TEXT,
    priority INTEGER DEFAULT 5,
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    sent_at TEXT,
    error_message TEXT,
    status TEXT DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (template_id) REFERENCES email_templates(id)
);

CREATE TABLE IF NOT EXISTS email_logs (
    id TEXT PRIMARY KEY,
    message_id TEXT,
    direction TEXT NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    subject TEXT,
    body TEXT,
    attachments TEXT,
    related_entity_type TEXT,
    related_entity_id TEXT,
    sent_at TEXT NOT NULL,
    status TEXT DEFAULT 'Sent'
);

-- Reporting Builder
CREATE TABLE IF NOT EXISTS report_definitions (
    id TEXT PRIMARY KEY,
    report_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    data_source TEXT NOT NULL,
    query_text TEXT NOT NULL,
    parameters TEXT,
    columns TEXT,
    filters TEXT,
    sorting TEXT,
    grouping TEXT,
    chart_type TEXT,
    is_scheduled INTEGER DEFAULT 0,
    schedule_cron TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report_executions (
    id TEXT PRIMARY KEY,
    report_id TEXT NOT NULL,
    parameters TEXT,
    row_count INTEGER,
    file_path TEXT,
    file_format TEXT,
    file_size INTEGER,
    execution_time_ms INTEGER,
    status TEXT DEFAULT 'Running',
    error_message TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (report_id) REFERENCES report_definitions(id)
);

-- Mobile App Support
CREATE TABLE IF NOT EXISTS mobile_devices (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    device_type TEXT NOT NULL,
    device_token TEXT NOT NULL,
    device_name TEXT,
    os_version TEXT,
    app_version TEXT,
    last_active TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mobile_sessions (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    login_at TEXT NOT NULL,
    logout_at TEXT,
    ip_address TEXT,
    status TEXT DEFAULT 'Active',
    FOREIGN KEY (device_id) REFERENCES mobile_devices(id)
);

CREATE TABLE IF NOT EXISTS push_notifications (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    data TEXT,
    sent_at TEXT,
    delivered_at TEXT,
    read_at TEXT,
    status TEXT DEFAULT 'Pending',
    FOREIGN KEY (device_id) REFERENCES mobile_devices(id)
);

-- API Management
CREATE TABLE IF NOT EXISTS api_keys (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    key_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    permissions TEXT,
    rate_limit INTEGER DEFAULT 1000,
    last_used TEXT,
    expires_at TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS api_usage_logs (
    id TEXT PRIMARY KEY,
    api_key_id TEXT,
    endpoint TEXT NOT NULL,
    method TEXT NOT NULL,
    request_size INTEGER,
    response_size INTEGER,
    response_code INTEGER,
    response_time_ms INTEGER,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL
);

-- ============================================
-- INDEXES
-- ============================================

-- Financial
CREATE INDEX IF NOT EXISTS idx_bank_tx_account ON bank_transactions(bank_account_id);
CREATE INDEX IF NOT EXISTS idx_bank_tx_date ON bank_transactions(transaction_date);
CREATE INDEX IF NOT EXISTS idx_cash_flow_items_forecast ON cash_flow_items(forecast_id);
CREATE INDEX IF NOT EXISTS idx_cost_allocations_pool ON cost_allocations(pool_id);
CREATE INDEX IF NOT EXISTS idx_intercompany_from ON intercompany_transactions(from_company_id);
CREATE INDEX IF NOT EXISTS idx_intercompany_to ON intercompany_transactions(to_company_id);
CREATE INDEX IF NOT EXISTS idx_consolidation_companies ON consolidation_companies(consolidation_id);

-- Supply Chain
CREATE INDEX IF NOT EXISTS idx_pick_lists_warehouse ON pick_lists(warehouse_id);
CREATE INDEX IF NOT EXISTS idx_shipments_carrier ON shipment_orders(carrier_id);
CREATE INDEX IF NOT EXISTS idx_edi_docs_partner ON edi_documents(partner_id);
CREATE INDEX IF NOT EXISTS idx_rfq_responses_rfq ON rfq_responses(rfq_id);

-- Manufacturing
CREATE INDEX IF NOT EXISTS idx_mrp_orders_run ON mrp_planned_orders(mrp_run_id);
CREATE INDEX IF NOT EXISTS idx_shop_floor_wo ON shop_floor_operations(work_order_id);
CREATE INDEX IF NOT EXISTS idx_maintenance_wo_equipment ON maintenance_work_orders(equipment_id);
CREATE INDEX IF NOT EXISTS idx_equipment_work_center ON equipment(work_center_id);

-- Sales/CRM
CREATE INDEX IF NOT EXISTS idx_territory_assign_terr ON territory_assignments(territory_id);
CREATE INDEX IF NOT EXISTS idx_commissions_rep ON sales_rep_commissions(sales_rep_id);
CREATE INDEX IF NOT EXISTS idx_contracts_customer ON contracts(customer_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_customer ON subscriptions(customer_id);

-- Project Management
CREATE INDEX IF NOT EXISTS idx_project_tasks_project ON project_tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_timesheets_employee ON timesheets(employee_id);
CREATE INDEX IF NOT EXISTS idx_timesheet_entries_ts ON timesheet_entries(timesheet_id);

-- HR
CREATE INDEX IF NOT EXISTS idx_payroll_entries_run ON payroll_entries(payroll_run_id);
CREATE INDEX IF NOT EXISTS idx_employee_benefits_emp ON employee_benefits(employee_id);
CREATE INDEX IF NOT EXISTS idx_performance_goals_cycle ON performance_goals(cycle_id);
CREATE INDEX IF NOT EXISTS idx_employee_training_emp ON employee_training(employee_id);
CREATE INDEX IF NOT EXISTS idx_job_apps_job ON job_applications(job_id);

-- Analytics
CREATE INDEX IF NOT EXISTS idx_kpi_values_kpi ON kpi_values(kpi_id);
CREATE INDEX IF NOT EXISTS idx_alerts_created ON alerts(created_at);
CREATE INDEX IF NOT EXISTS idx_predictions_model ON predictions(model_id);

-- Platform
CREATE INDEX IF NOT EXISTS idx_tenant_users_tenant ON tenant_users(tenant_id);
CREATE INDEX IF NOT EXISTS idx_automation_rules_entity ON automation_rules(entity_type);
CREATE INDEX IF NOT EXISTS idx_email_queues_status ON email_queues(status);
CREATE INDEX IF NOT EXISTS idx_api_usage_key ON api_usage_logs(api_key_id);
