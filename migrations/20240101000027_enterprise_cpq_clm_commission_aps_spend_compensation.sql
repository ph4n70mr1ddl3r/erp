-- CPQ (Configure Price Quote) Module
CREATE TABLE IF NOT EXISTS configuration_templates (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    product_id TEXT,
    base_price INTEGER NOT NULL DEFAULT 0,
    min_margin_percent REAL NOT NULL DEFAULT 0,
    max_discount_percent REAL NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS config_attributes (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    data_type TEXT NOT NULL DEFAULT 'Text',
    required INTEGER NOT NULL DEFAULT 0,
    default_value TEXT,
    options TEXT,
    validation_regex TEXT,
    min_value REAL,
    max_value REAL,
    unit TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS pricing_rules (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    name TEXT NOT NULL,
    rule_type TEXT NOT NULL DEFAULT 'Fixed',
    attribute_id TEXT,
    attribute_value TEXT,
    base_price_modifier REAL,
    fixed_price INTEGER,
    markup_percent REAL,
    formula TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS pricing_matrices (
    id TEXT PRIMARY KEY,
    pricing_rule_id TEXT NOT NULL,
    row_attribute_id TEXT NOT NULL,
    column_attribute_id TEXT NOT NULL,
    rows TEXT NOT NULL,
    columns TEXT NOT NULL,
    prices TEXT NOT NULL,
    FOREIGN KEY (pricing_rule_id) REFERENCES pricing_rules(id)
);

CREATE TABLE IF NOT EXISTS product_configurations (
    id TEXT PRIMARY KEY,
    configuration_number TEXT NOT NULL UNIQUE,
    template_id TEXT NOT NULL,
    product_id TEXT,
    name TEXT NOT NULL,
    description TEXT,
    base_price INTEGER NOT NULL DEFAULT 0,
    configured_price INTEGER NOT NULL DEFAULT 0,
    margin_percent REAL NOT NULL DEFAULT 0,
    is_valid INTEGER NOT NULL DEFAULT 1,
    validation_errors TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS configuration_values (
    id TEXT PRIMARY KEY,
    configuration_id TEXT NOT NULL,
    attribute_id TEXT NOT NULL,
    value TEXT NOT NULL,
    display_value TEXT,
    FOREIGN KEY (configuration_id) REFERENCES product_configurations(id),
    FOREIGN KEY (attribute_id) REFERENCES config_attributes(id)
);

CREATE TABLE IF NOT EXISTS configuration_constraints (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    name TEXT NOT NULL,
    constraint_type TEXT NOT NULL DEFAULT 'Requires',
    source_attribute_id TEXT NOT NULL,
    source_values TEXT NOT NULL,
    target_attribute_id TEXT NOT NULL,
    target_values TEXT NOT NULL,
    error_message TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS configurable_boms (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    parent_attribute_id TEXT,
    parent_value TEXT,
    component_product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    quantity_formula TEXT,
    is_optional INTEGER NOT NULL DEFAULT 0,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS configurable_routings (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    work_center_id TEXT NOT NULL,
    operation_name TEXT NOT NULL,
    setup_time INTEGER NOT NULL DEFAULT 0,
    run_time INTEGER NOT NULL DEFAULT 0,
    run_time_formula TEXT,
    condition_attribute_id TEXT,
    condition_value TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS configured_quotes (
    id TEXT PRIMARY KEY,
    quote_number TEXT NOT NULL UNIQUE,
    configuration_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    opportunity_id TEXT,
    valid_until TEXT NOT NULL,
    base_price INTEGER NOT NULL DEFAULT 0,
    configured_price INTEGER NOT NULL DEFAULT 0,
    discount_percent REAL NOT NULL DEFAULT 0,
    discount_amount INTEGER NOT NULL DEFAULT 0,
    margin_percent REAL NOT NULL DEFAULT 0,
    total_price INTEGER NOT NULL DEFAULT 0,
    terms TEXT,
    internal_notes TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    sent_at TEXT,
    responded_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (configuration_id) REFERENCES product_configurations(id)
);

CREATE TABLE IF NOT EXISTS configured_quote_lines (
    id TEXT PRIMARY KEY,
    quote_id TEXT NOT NULL,
    line_type TEXT NOT NULL,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price INTEGER NOT NULL DEFAULT 0,
    total_price INTEGER NOT NULL DEFAULT 0,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (quote_id) REFERENCES configured_quotes(id)
);

CREATE TABLE IF NOT EXISTS quote_approvals (
    id TEXT PRIMARY KEY,
    quote_id TEXT NOT NULL,
    approver_id TEXT NOT NULL,
    approval_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    comments TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (quote_id) REFERENCES configured_quotes(id)
);

CREATE TABLE IF NOT EXISTS quote_versions (
    id TEXT PRIMARY KEY,
    quote_id TEXT NOT NULL,
    version_number INTEGER NOT NULL DEFAULT 1,
    configuration_snapshot TEXT NOT NULL,
    price_snapshot TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT NOT NULL,
    notes TEXT,
    FOREIGN KEY (quote_id) REFERENCES configured_quotes(id)
);

CREATE TABLE IF NOT EXISTS guided_selling_steps (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    step_number INTEGER NOT NULL DEFAULT 1,
    title TEXT NOT NULL,
    description TEXT,
    help_text TEXT,
    attribute_ids TEXT NOT NULL,
    is_required INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS recommendation_rules (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    name TEXT NOT NULL,
    condition_logic TEXT NOT NULL,
    recommendation_text TEXT NOT NULL,
    recommended_values TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS bulk_discount_tiers (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    min_quantity INTEGER NOT NULL DEFAULT 1,
    max_quantity INTEGER,
    discount_percent REAL NOT NULL DEFAULT 0,
    FOREIGN KEY (template_id) REFERENCES configuration_templates(id)
);

CREATE TABLE IF NOT EXISTS volume_price_breaks (
    id TEXT PRIMARY KEY,
    pricing_rule_id TEXT NOT NULL,
    min_quantity INTEGER NOT NULL DEFAULT 1,
    max_quantity INTEGER,
    unit_price INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (pricing_rule_id) REFERENCES pricing_rules(id)
);

-- CLM (Contract Lifecycle Management) Module
CREATE TABLE IF NOT EXISTS contract_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    default_term_months INTEGER NOT NULL DEFAULT 12,
    auto_renew_default INTEGER NOT NULL DEFAULT 0,
    required_clauses TEXT,
    approval_workflow_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS contracts (
    id TEXT PRIMARY KEY,
    contract_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL DEFAULT 'Purchase',
    vendor_id TEXT,
    customer_id TEXT,
    contract_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    risk_level TEXT NOT NULL DEFAULT 'Medium',
    value INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    effective_date TEXT,
    auto_renew INTEGER NOT NULL DEFAULT 0,
    renewal_term_months INTEGER NOT NULL DEFAULT 12,
    notice_period_days INTEGER NOT NULL DEFAULT 30,
    owner_id TEXT NOT NULL,
    department_id TEXT,
    parent_contract_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS contract_clauses (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    clause_type TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    section_number TEXT NOT NULL,
    is_standard INTEGER NOT NULL DEFAULT 1,
    is_negotiable INTEGER NOT NULL DEFAULT 1,
    deviation_notes TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_clause_library (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    clause_type TEXT NOT NULL,
    content TEXT NOT NULL,
    description TEXT,
    is_mandatory INTEGER NOT NULL DEFAULT 0,
    risk_level TEXT NOT NULL DEFAULT 'Low',
    version INTEGER NOT NULL DEFAULT 1,
    effective_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS contract_milestones (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    due_date TEXT NOT NULL,
    completed_date TEXT,
    is_billing_event INTEGER NOT NULL DEFAULT 0,
    billing_amount INTEGER,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_obligations (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    obligation_type TEXT NOT NULL,
    description TEXT NOT NULL,
    responsible_party TEXT NOT NULL,
    frequency TEXT NOT NULL,
    next_due_date TEXT NOT NULL,
    last_completed TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_amendments (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    amendment_number TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    changes_summary TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT NOT NULL,
    approved_at TEXT,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_documents (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    document_type TEXT NOT NULL,
    document_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL DEFAULT 0,
    version INTEGER NOT NULL DEFAULT 1,
    uploaded_by TEXT,
    uploaded_at TEXT NOT NULL,
    is_signed INTEGER NOT NULL DEFAULT 0,
    signed_at TEXT,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS approval_workflows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    contract_type TEXT NOT NULL,
    min_value INTEGER,
    max_value INTEGER,
    levels INTEGER NOT NULL DEFAULT 1,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS approval_workflow_levels (
    id TEXT PRIMARY KEY,
    workflow_id TEXT NOT NULL,
    level_number INTEGER NOT NULL DEFAULT 1,
    approver_type TEXT NOT NULL,
    approver_id TEXT,
    role_id TEXT,
    is_parallel INTEGER NOT NULL DEFAULT 0,
    timeout_days INTEGER NOT NULL DEFAULT 7,
    FOREIGN KEY (workflow_id) REFERENCES approval_workflows(id)
);

CREATE TABLE IF NOT EXISTS contract_approvals (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    approver_id TEXT NOT NULL,
    approval_level INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Pending',
    comments TEXT,
    due_date TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_signatures (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    signer_type TEXT NOT NULL,
    signer_id TEXT,
    signer_name TEXT NOT NULL,
    signer_email TEXT NOT NULL,
    signer_title TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    signed_at TEXT,
    signature_data TEXT,
    ip_address TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_renewals (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    renewal_type TEXT NOT NULL,
    new_start_date TEXT NOT NULL,
    new_end_date TEXT NOT NULL,
    new_value INTEGER,
    status TEXT NOT NULL DEFAULT 'Pending',
    initiated_by TEXT,
    initiated_at TEXT NOT NULL,
    completed_at TEXT,
    notes TEXT,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_alerts (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    alert_type TEXT NOT NULL,
    alert_date TEXT NOT NULL,
    message TEXT NOT NULL,
    recipients TEXT NOT NULL,
    is_sent INTEGER NOT NULL DEFAULT 0,
    sent_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_spend (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    committed_amount INTEGER NOT NULL DEFAULT 0,
    spent_amount INTEGER NOT NULL DEFAULT 0,
    remaining_amount INTEGER NOT NULL DEFAULT 0,
    utilization_percent REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_vendor_performance (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    evaluation_period TEXT NOT NULL,
    on_time_delivery_pct REAL NOT NULL DEFAULT 0,
    quality_score REAL NOT NULL DEFAULT 0,
    responsiveness_score REAL NOT NULL DEFAULT 0,
    compliance_score REAL NOT NULL DEFAULT 0,
    overall_score REAL NOT NULL DEFAULT 0,
    notes TEXT,
    evaluated_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_risk_assessments (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    assessment_date TEXT NOT NULL,
    financial_risk TEXT NOT NULL,
    legal_risk TEXT NOT NULL,
    operational_risk TEXT NOT NULL,
    compliance_risk TEXT NOT NULL,
    overall_risk TEXT NOT NULL DEFAULT 'Medium',
    mitigation_notes TEXT,
    assessed_by TEXT,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS contract_terms (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    term_name TEXT NOT NULL,
    term_value TEXT NOT NULL,
    term_unit TEXT NOT NULL,
    is_custom INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS compliance_checks (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    check_type TEXT NOT NULL,
    check_name TEXT NOT NULL,
    result TEXT NOT NULL,
    details TEXT,
    checked_at TEXT NOT NULL,
    checked_by TEXT,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

-- Commission Management Module
CREATE TABLE IF NOT EXISTS commission_plans (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    commission_type TEXT NOT NULL DEFAULT 'Percentage',
    basis TEXT NOT NULL DEFAULT 'Revenue',
    frequency TEXT NOT NULL DEFAULT 'Monthly',
    default_rate REAL NOT NULL DEFAULT 0,
    min_rate REAL,
    max_rate REAL,
    cap_amount INTEGER,
    clawback_period_days INTEGER NOT NULL DEFAULT 90,
    effective_date TEXT NOT NULL,
    expiry_date TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS commission_tiers (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    tier_name TEXT NOT NULL,
    min_amount INTEGER NOT NULL DEFAULT 0,
    max_amount INTEGER,
    rate REAL NOT NULL DEFAULT 0,
    is_accelerator INTEGER NOT NULL DEFAULT 0,
    sort_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (plan_id) REFERENCES commission_plans(id)
);

CREATE TABLE IF NOT EXISTS commission_splits (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    name TEXT NOT NULL,
    split_type TEXT NOT NULL DEFAULT 'Percentage',
    FOREIGN KEY (plan_id) REFERENCES commission_plans(id)
);

CREATE TABLE IF NOT EXISTS commission_split_participants (
    id TEXT PRIMARY KEY,
    split_id TEXT NOT NULL,
    sales_rep_id TEXT NOT NULL,
    split_percent REAL NOT NULL DEFAULT 0,
    role TEXT NOT NULL,
    FOREIGN KEY (split_id) REFERENCES commission_splits(id)
);

CREATE TABLE IF NOT EXISTS sales_rep_assignments (
    id TEXT PRIMARY KEY,
    sales_rep_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    territory_id TEXT,
    product_id TEXT,
    customer_id TEXT,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    is_primary INTEGER NOT NULL DEFAULT 1,
    split_percent REAL NOT NULL DEFAULT 100,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS commission_calculations (
    id TEXT PRIMARY KEY,
    calculation_number TEXT NOT NULL UNIQUE,
    sales_rep_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    gross_revenue INTEGER NOT NULL DEFAULT 0,
    returns INTEGER NOT NULL DEFAULT 0,
    net_revenue INTEGER NOT NULL DEFAULT 0,
    cost_of_goods INTEGER NOT NULL DEFAULT 0,
    gross_margin INTEGER NOT NULL DEFAULT 0,
    base_commission INTEGER NOT NULL DEFAULT 0,
    tier_bonus INTEGER NOT NULL DEFAULT 0,
    override_commission INTEGER NOT NULL DEFAULT 0,
    adjustments INTEGER NOT NULL DEFAULT 0,
    clawbacks INTEGER NOT NULL DEFAULT 0,
    total_commission INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    calculated_at TEXT,
    approved_at TEXT,
    approved_by TEXT,
    paid_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS commission_lines (
    id TEXT PRIMARY KEY,
    calculation_id TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    transaction_date TEXT NOT NULL,
    customer_id TEXT,
    product_id TEXT,
    quantity INTEGER NOT NULL DEFAULT 0,
    revenue INTEGER NOT NULL DEFAULT 0,
    cost INTEGER NOT NULL DEFAULT 0,
    margin INTEGER NOT NULL DEFAULT 0,
    rate_applied REAL NOT NULL DEFAULT 0,
    commission_amount INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    FOREIGN KEY (calculation_id) REFERENCES commission_calculations(id)
);

CREATE TABLE IF NOT EXISTS commission_adjustments (
    id TEXT PRIMARY KEY,
    calculation_id TEXT NOT NULL,
    adjustment_type TEXT NOT NULL,
    amount INTEGER NOT NULL DEFAULT 0,
    reason TEXT NOT NULL,
    approved_by TEXT,
    approved_at TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (calculation_id) REFERENCES commission_calculations(id)
);

CREATE TABLE IF NOT EXISTS commission_payments (
    id TEXT PRIMARY KEY,
    payment_number TEXT NOT NULL UNIQUE,
    calculation_id TEXT NOT NULL,
    sales_rep_id TEXT NOT NULL,
    payment_date TEXT NOT NULL,
    amount INTEGER NOT NULL DEFAULT 0,
    payment_method TEXT NOT NULL,
    reference TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (calculation_id) REFERENCES commission_calculations(id)
);

CREATE TABLE IF NOT EXISTS commission_draws (
    id TEXT PRIMARY KEY,
    sales_rep_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    draw_type TEXT NOT NULL DEFAULT 'Recoverable',
    amount INTEGER NOT NULL DEFAULT 0,
    frequency TEXT NOT NULL DEFAULT 'Monthly',
    start_date TEXT NOT NULL,
    end_date TEXT,
    balance INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES commission_plans(id)
);

CREATE TABLE IF NOT EXISTS draw_transactions (
    id TEXT PRIMARY KEY,
    draw_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    amount INTEGER NOT NULL DEFAULT 0,
    balance_after INTEGER NOT NULL DEFAULT 0,
    period TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (draw_id) REFERENCES commission_draws(id)
);

CREATE TABLE IF NOT EXISTS sales_quotas (
    id TEXT PRIMARY KEY,
    sales_rep_id TEXT NOT NULL,
    quota_type TEXT NOT NULL DEFAULT 'Revenue',
    period TEXT NOT NULL DEFAULT 'Annual',
    year INTEGER NOT NULL,
    quarter INTEGER,
    month INTEGER,
    target_amount INTEGER NOT NULL DEFAULT 0,
    stretch_amount INTEGER,
    territory_id TEXT,
    product_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS quota_progress (
    id TEXT PRIMARY KEY,
    quota_id TEXT NOT NULL,
    as_of_date TEXT NOT NULL,
    achieved_amount INTEGER NOT NULL DEFAULT 0,
    percent_achieved REAL NOT NULL DEFAULT 0,
    forecast_amount INTEGER NOT NULL DEFAULT 0,
    gap_to_quota INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quota_id) REFERENCES sales_quotas(id)
);

CREATE TABLE IF NOT EXISTS commission_disputes (
    id TEXT PRIMARY KEY,
    calculation_id TEXT NOT NULL,
    sales_rep_id TEXT NOT NULL,
    dispute_type TEXT NOT NULL,
    description TEXT NOT NULL,
    requested_amount INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Open',
    resolution TEXT,
    resolved_amount INTEGER,
    resolved_by TEXT,
    resolved_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (calculation_id) REFERENCES commission_calculations(id)
);

CREATE TABLE IF NOT EXISTS sales_teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    manager_id TEXT,
    parent_team_id TEXT,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sales_team_members (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    sales_rep_id TEXT NOT NULL,
    role TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (team_id) REFERENCES sales_teams(id)
);

CREATE TABLE IF NOT EXISTS override_rules (
    id TEXT PRIMARY KEY,
    manager_id TEXT NOT NULL,
    plan_id TEXT,
    override_type TEXT NOT NULL DEFAULT 'Team',
    rate REAL NOT NULL DEFAULT 0,
    applies_to_team INTEGER NOT NULL DEFAULT 1,
    team_id TEXT,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    FOREIGN KEY (plan_id) REFERENCES commission_plans(id)
);

CREATE TABLE IF NOT EXISTS commission_reports (
    id TEXT PRIMARY KEY,
    report_type TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    sales_rep_id TEXT,
    team_id TEXT,
    total_revenue INTEGER NOT NULL DEFAULT 0,
    total_commission INTEGER NOT NULL DEFAULT 0,
    avg_commission_rate REAL NOT NULL DEFAULT 0,
    top_performer_id TEXT,
    generated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS commission_forecasts (
    id TEXT PRIMARY KEY,
    sales_rep_id TEXT NOT NULL,
    period TEXT NOT NULL,
    pipeline_revenue INTEGER NOT NULL DEFAULT 0,
    weighted_revenue INTEGER NOT NULL DEFAULT 0,
    forecast_commission INTEGER NOT NULL DEFAULT 0,
    confidence_level REAL NOT NULL DEFAULT 0,
    calculated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spiffs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    product_id TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    reward_type TEXT NOT NULL DEFAULT 'Fixed',
    reward_amount INTEGER NOT NULL DEFAULT 0,
    qualification_criteria TEXT NOT NULL,
    max_payouts INTEGER,
    current_payouts INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spiff_achievements (
    id TEXT PRIMARY KEY,
    spiff_id TEXT NOT NULL,
    sales_rep_id TEXT NOT NULL,
    achieved_date TEXT NOT NULL,
    achievement_value INTEGER NOT NULL DEFAULT 0,
    payout_amount INTEGER NOT NULL DEFAULT 0,
    paid INTEGER NOT NULL DEFAULT 0,
    paid_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (spiff_id) REFERENCES spiffs(id)
);

-- APS (Advanced Planning & Scheduling) Module
CREATE TABLE IF NOT EXISTS planning_calendars (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    working_days TEXT NOT NULL DEFAULT '1,2,3,4,5',
    shift_pattern TEXT NOT NULL,
    holidays TEXT,
    capacity_per_day INTEGER NOT NULL DEFAULT 480,
    effective_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS planning_shifts (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    shift_name TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    break_start TEXT,
    break_end TEXT,
    capacity_percent REAL NOT NULL DEFAULT 100,
    is_active INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (calendar_id) REFERENCES planning_calendars(id)
);

CREATE TABLE IF NOT EXISTS resource_capacities (
    id TEXT PRIMARY KEY,
    resource_id TEXT NOT NULL,
    resource_type TEXT NOT NULL DEFAULT 'Machine',
    resource_name TEXT NOT NULL,
    work_center_id TEXT,
    calendar_id TEXT,
    daily_capacity INTEGER NOT NULL DEFAULT 480,
    unit_of_measure TEXT NOT NULL DEFAULT 'Minutes',
    efficiency_percent REAL NOT NULL DEFAULT 100,
    utilization_percent REAL NOT NULL DEFAULT 85,
    available_from TEXT NOT NULL,
    available_to TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS resource_availabilities (
    id TEXT PRIMARY KEY,
    resource_id TEXT NOT NULL,
    date TEXT NOT NULL,
    shift_id TEXT,
    available_capacity INTEGER NOT NULL DEFAULT 0,
    planned_capacity INTEGER NOT NULL DEFAULT 0,
    actual_capacity INTEGER NOT NULL DEFAULT 0,
    downtime_minutes INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    FOREIGN KEY (resource_id) REFERENCES resource_capacities(id)
);

CREATE TABLE IF NOT EXISTS master_production_schedules (
    id TEXT PRIMARY KEY,
    schedule_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    planning_horizon_days INTEGER NOT NULL DEFAULT 90,
    time_bucket TEXT NOT NULL DEFAULT 'Weekly',
    status TEXT NOT NULL DEFAULT 'Draft',
    schedule_method TEXT NOT NULL DEFAULT 'Forward',
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    released_at TEXT
);

CREATE TABLE IF NOT EXISTS mps_items (
    id TEXT PRIMARY KEY,
    mps_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    planning_start TEXT NOT NULL,
    planning_end TEXT NOT NULL,
    total_planned INTEGER NOT NULL DEFAULT 0,
    total_demand INTEGER NOT NULL DEFAULT 0,
    total_supply INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    FOREIGN KEY (mps_id) REFERENCES master_production_schedules(id)
);

CREATE TABLE IF NOT EXISTS mps_time_buckets (
    id TEXT PRIMARY KEY,
    mps_item_id TEXT NOT NULL,
    bucket_start TEXT NOT NULL,
    bucket_end TEXT NOT NULL,
    gross_requirement INTEGER NOT NULL DEFAULT 0,
    scheduled_receipts INTEGER NOT NULL DEFAULT 0,
    projected_on_hand INTEGER NOT NULL DEFAULT 0,
    net_requirement INTEGER NOT NULL DEFAULT 0,
    planned_order_receipt INTEGER NOT NULL DEFAULT 0,
    planned_order_release INTEGER NOT NULL DEFAULT 0,
    available_to_promise INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (mps_item_id) REFERENCES mps_items(id)
);

CREATE TABLE IF NOT EXISTS material_requirements_plans (
    id TEXT PRIMARY KEY,
    mrp_number TEXT NOT NULL UNIQUE,
    mps_id TEXT,
    planning_date TEXT NOT NULL,
    planning_horizon_days INTEGER NOT NULL DEFAULT 90,
    regenerate INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Draft',
    run_started_at TEXT,
    run_completed_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (mps_id) REFERENCES master_production_schedules(id)
);

CREATE TABLE IF NOT EXISTS mrp_items (
    id TEXT PRIMARY KEY,
    mrp_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    bom_id TEXT,
    lead_time_days INTEGER NOT NULL DEFAULT 0,
    safety_stock INTEGER NOT NULL DEFAULT 0,
    lot_size INTEGER NOT NULL DEFAULT 1,
    lot_size_method TEXT NOT NULL DEFAULT 'Discrete',
    on_hand INTEGER NOT NULL DEFAULT 0,
    allocated INTEGER NOT NULL DEFAULT 0,
    on_order INTEGER NOT NULL DEFAULT 0,
    net_requirement INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    FOREIGN KEY (mrp_id) REFERENCES material_requirements_plans(id)
);

CREATE TABLE IF NOT EXISTS mrp_suggestions (
    id TEXT PRIMARY KEY,
    mrp_item_id TEXT NOT NULL,
    suggestion_type TEXT NOT NULL DEFAULT 'PlannedOrder',
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    due_date TEXT NOT NULL,
    release_date TEXT NOT NULL,
    source_type TEXT,
    source_id TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    processed_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (mrp_item_id) REFERENCES mrp_items(id)
);

CREATE TABLE IF NOT EXISTS detailed_schedules (
    id TEXT PRIMARY KEY,
    schedule_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    schedule_type TEXT NOT NULL DEFAULT 'Production',
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    optimization_method TEXT NOT NULL DEFAULT 'Priority',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS schedule_operations (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    work_order_id TEXT NOT NULL,
    routing_operation_id TEXT,
    resource_id TEXT NOT NULL,
    resource_type TEXT NOT NULL DEFAULT 'Machine',
    operation_name TEXT NOT NULL,
    scheduled_start TEXT NOT NULL,
    scheduled_end TEXT NOT NULL,
    setup_time INTEGER NOT NULL DEFAULT 0,
    run_time INTEGER NOT NULL DEFAULT 0,
    quantity INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Planned',
    priority INTEGER NOT NULL DEFAULT 0,
    sequence INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (schedule_id) REFERENCES detailed_schedules(id)
);

CREATE TABLE IF NOT EXISTS schedule_constraints (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    constraint_type TEXT NOT NULL DEFAULT 'Precedence',
    operation_id TEXT,
    related_operation_id TEXT,
    offset_minutes INTEGER NOT NULL DEFAULT 0,
    is_hard INTEGER NOT NULL DEFAULT 1,
    description TEXT,
    FOREIGN KEY (schedule_id) REFERENCES detailed_schedules(id)
);

CREATE TABLE IF NOT EXISTS capacity_plans (
    id TEXT PRIMARY KEY,
    plan_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    planning_horizon_days INTEGER NOT NULL DEFAULT 30,
    bucket_size TEXT NOT NULL DEFAULT 'Daily',
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS capacity_plan_items (
    id TEXT PRIMARY KEY,
    capacity_plan_id TEXT NOT NULL,
    resource_id TEXT NOT NULL,
    bucket_start TEXT NOT NULL,
    bucket_end TEXT NOT NULL,
    available_capacity INTEGER NOT NULL DEFAULT 0,
    required_capacity INTEGER NOT NULL DEFAULT 0,
    overload_capacity INTEGER NOT NULL DEFAULT 0,
    utilization_percent REAL NOT NULL DEFAULT 0,
    FOREIGN KEY (capacity_plan_id) REFERENCES capacity_plans(id)
);

CREATE TABLE IF NOT EXISTS production_lines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    work_center_id TEXT,
    capacity_per_hour INTEGER NOT NULL DEFAULT 0,
    efficiency REAL NOT NULL DEFAULT 100,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS production_line_stations (
    id TEXT PRIMARY KEY,
    production_line_id TEXT NOT NULL,
    station_number INTEGER NOT NULL DEFAULT 1,
    station_name TEXT NOT NULL,
    work_center_id TEXT,
    cycle_time INTEGER NOT NULL DEFAULT 0,
    buffer_capacity INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (production_line_id) REFERENCES production_lines(id)
);

CREATE TABLE IF NOT EXISTS finite_schedules (
    id TEXT PRIMARY KEY,
    schedule_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    resource_id TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    optimization_goal TEXT NOT NULL DEFAULT 'MinimizeTardiness',
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS finite_schedule_blocks (
    id TEXT PRIMARY KEY,
    finite_schedule_id TEXT NOT NULL,
    work_order_id TEXT NOT NULL,
    operation_id TEXT,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    setup_time INTEGER NOT NULL DEFAULT 0,
    run_time INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Planned',
    FOREIGN KEY (finite_schedule_id) REFERENCES finite_schedules(id)
);

CREATE TABLE IF NOT EXISTS what_if_scenarios (
    id TEXT PRIMARY KEY,
    scenario_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    base_date TEXT NOT NULL,
    changes TEXT NOT NULL,
    results TEXT,
    comparison_baseline_id TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS planning_exceptions (
    id TEXT PRIMARY KEY,
    exception_type TEXT NOT NULL,
    severity TEXT NOT NULL DEFAULT 'Warning',
    product_id TEXT,
    resource_id TEXT,
    work_order_id TEXT,
    message TEXT NOT NULL,
    suggested_action TEXT,
    is_resolved INTEGER NOT NULL DEFAULT 0,
    resolved_at TEXT,
    resolved_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS order_priorities (
    id TEXT PRIMARY KEY,
    order_type TEXT NOT NULL,
    order_id TEXT NOT NULL,
    customer_id TEXT,
    priority_score INTEGER NOT NULL DEFAULT 0,
    due_date TEXT NOT NULL,
    value INTEGER NOT NULL DEFAULT 0,
    customer_priority INTEGER NOT NULL DEFAULT 0,
    strategic_value INTEGER NOT NULL DEFAULT 0,
    calculated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS resource_maintenance_windows (
    id TEXT PRIMARY KEY,
    resource_id TEXT NOT NULL,
    window_start TEXT NOT NULL,
    window_end TEXT NOT NULL,
    maintenance_type TEXT NOT NULL DEFAULT 'Planned',
    description TEXT,
    capacity_reduction_percent REAL NOT NULL DEFAULT 100,
    status TEXT NOT NULL DEFAULT 'Scheduled',
    created_at TEXT NOT NULL,
    FOREIGN KEY (resource_id) REFERENCES resource_capacities(id)
);

CREATE TABLE IF NOT EXISTS sequencing_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    rule_type TEXT NOT NULL DEFAULT 'Priority',
    priority_criteria TEXT NOT NULL,
    constraints TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS schedule_performance (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    metric_date TEXT NOT NULL,
    on_time_percent REAL NOT NULL DEFAULT 0,
    utilization_percent REAL NOT NULL DEFAULT 0,
    efficiency_percent REAL NOT NULL DEFAULT 0,
    throughput INTEGER NOT NULL DEFAULT 0,
    wip_value INTEGER NOT NULL DEFAULT 0,
    tardy_orders INTEGER NOT NULL DEFAULT 0,
    calculated_at TEXT NOT NULL,
    FOREIGN KEY (schedule_id) REFERENCES detailed_schedules(id)
);

-- Spend Analytics Module
CREATE TABLE IF NOT EXISTS spend_category_trees (
    id TEXT PRIMARY KEY,
    parent_id TEXT,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    level INTEGER NOT NULL DEFAULT 0,
    path TEXT NOT NULL,
    is_leaf INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spend_transactions (
    id TEXT PRIMARY KEY,
    transaction_number TEXT NOT NULL UNIQUE,
    transaction_date TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    cost_center_id TEXT,
    department_id TEXT,
    project_id TEXT,
    gl_account_id TEXT,
    amount INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    amount_base INTEGER NOT NULL DEFAULT 0,
    quantity INTEGER,
    unit_of_measure TEXT,
    source_type TEXT NOT NULL,
    source_id TEXT,
    description TEXT,
    is_contracted INTEGER NOT NULL DEFAULT 0,
    contract_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spend_summaries (
    id TEXT PRIMARY KEY,
    period_type TEXT NOT NULL DEFAULT 'Monthly',
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    category_id TEXT,
    vendor_id TEXT,
    department_id TEXT,
    cost_center_id TEXT,
    total_spend INTEGER NOT NULL DEFAULT 0,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    avg_transaction INTEGER NOT NULL DEFAULT 0,
    min_transaction INTEGER NOT NULL DEFAULT 0,
    max_transaction INTEGER NOT NULL DEFAULT 0,
    contracted_spend INTEGER NOT NULL DEFAULT 0,
    uncontracted_spend INTEGER NOT NULL DEFAULT 0,
    maverick_spend INTEGER NOT NULL DEFAULT 0,
    savings_identified INTEGER NOT NULL DEFAULT 0,
    savings_realized INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS vendor_spend_analyses (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    analysis_period TEXT NOT NULL DEFAULT 'Annual',
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_spend INTEGER NOT NULL DEFAULT 0,
    invoice_count INTEGER NOT NULL DEFAULT 0,
    po_count INTEGER NOT NULL DEFAULT 0,
    contract_count INTEGER NOT NULL DEFAULT 0,
    spend_under_contract INTEGER NOT NULL DEFAULT 0,
    spend_off_contract INTEGER NOT NULL DEFAULT 0,
    avg_invoice_value INTEGER NOT NULL DEFAULT 0,
    payment_terms_avg INTEGER NOT NULL DEFAULT 0,
    early_payment_savings INTEGER NOT NULL DEFAULT 0,
    duplicate_spend INTEGER NOT NULL DEFAULT 0,
    category_breakdown TEXT,
    trend_percent REAL NOT NULL DEFAULT 0,
    market_share REAL NOT NULL DEFAULT 0,
    risk_score REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS category_spend_analyses (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL,
    analysis_period TEXT NOT NULL DEFAULT 'Annual',
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_spend INTEGER NOT NULL DEFAULT 0,
    vendor_count INTEGER NOT NULL DEFAULT 0,
    top_vendor_spend INTEGER NOT NULL DEFAULT 0,
    top_vendor_share REAL NOT NULL DEFAULT 0,
    avg_contract_value INTEGER NOT NULL DEFAULT 0,
    contract_coverage REAL NOT NULL DEFAULT 0,
    savings_opportunity INTEGER NOT NULL DEFAULT 0,
    price_variance REAL NOT NULL DEFAULT 0,
    volume_trend REAL NOT NULL DEFAULT 0,
    supplier_concentration REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS department_spend_analyses (
    id TEXT PRIMARY KEY,
    department_id TEXT NOT NULL,
    analysis_period TEXT NOT NULL DEFAULT 'Annual',
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_spend INTEGER NOT NULL DEFAULT 0,
    budget INTEGER NOT NULL DEFAULT 0,
    variance INTEGER NOT NULL DEFAULT 0,
    variance_percent REAL NOT NULL DEFAULT 0,
    category_breakdown TEXT,
    top_vendors TEXT,
    maverick_spend INTEGER NOT NULL DEFAULT 0,
    maverick_percent REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spend_trends (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    metric_type TEXT NOT NULL,
    period TEXT NOT NULL,
    value INTEGER NOT NULL DEFAULT 0,
    previous_value INTEGER NOT NULL DEFAULT 0,
    change_percent REAL NOT NULL DEFAULT 0,
    moving_avg_3m INTEGER NOT NULL DEFAULT 0,
    moving_avg_12m INTEGER NOT NULL DEFAULT 0,
    trend_direction TEXT NOT NULL DEFAULT 'Stable',
    calculated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spend_forecasts (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT,
    forecast_date TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    forecast_amount INTEGER NOT NULL DEFAULT 0,
    confidence_low INTEGER NOT NULL DEFAULT 0,
    confidence_high INTEGER NOT NULL DEFAULT 0,
    confidence_level REAL NOT NULL DEFAULT 0,
    method TEXT NOT NULL DEFAULT 'Linear',
    assumptions TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS maverick_spends (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    amount INTEGER NOT NULL DEFAULT 0,
    maverick_type TEXT NOT NULL,
    reason TEXT NOT NULL,
    preferred_vendor_id TEXT,
    preferred_contract_id TEXT,
    potential_savings INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Identified',
    reviewed_by TEXT,
    reviewed_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (transaction_id) REFERENCES spend_transactions(id)
);

CREATE TABLE IF NOT EXISTS duplicate_spends (
    id TEXT PRIMARY KEY,
    transaction_ids TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    category_id TEXT,
    total_amount INTEGER NOT NULL DEFAULT 0,
    duplicate_amount INTEGER NOT NULL DEFAULT 0,
    match_type TEXT NOT NULL,
    confidence REAL NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Identified',
    resolved_at TEXT,
    resolution TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS price_variances (
    id TEXT PRIMARY KEY,
    product_id TEXT,
    category_id TEXT,
    vendor_id TEXT NOT NULL,
    standard_price INTEGER NOT NULL DEFAULT 0,
    actual_price INTEGER NOT NULL DEFAULT 0,
    variance_amount INTEGER NOT NULL DEFAULT 0,
    variance_percent REAL NOT NULL DEFAULT 0,
    quantity INTEGER NOT NULL DEFAULT 0,
    total_variance INTEGER NOT NULL DEFAULT 0,
    period TEXT NOT NULL,
    analysis_date TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS contract_compliances (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    category_id TEXT,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_spend INTEGER NOT NULL DEFAULT 0,
    contracted_spend INTEGER NOT NULL DEFAULT 0,
    compliance_rate REAL NOT NULL DEFAULT 0,
    off_contract_spend INTEGER NOT NULL DEFAULT 0,
    off_contract_transactions INTEGER NOT NULL DEFAULT 0,
    contract_utilization REAL NOT NULL DEFAULT 0,
    savings_achieved INTEGER NOT NULL DEFAULT 0,
    missed_savings INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (contract_id) REFERENCES contracts(id)
);

CREATE TABLE IF NOT EXISTS savings_opportunities (
    id TEXT PRIMARY KEY,
    opportunity_number TEXT NOT NULL UNIQUE,
    category_id TEXT NOT NULL,
    vendor_id TEXT,
    opportunity_type TEXT NOT NULL,
    description TEXT NOT NULL,
    current_spend INTEGER NOT NULL DEFAULT 0,
    potential_savings INTEGER NOT NULL DEFAULT 0,
    savings_percent REAL NOT NULL DEFAULT 0,
    effort_level TEXT NOT NULL DEFAULT 'Medium',
    implementation_timeframe TEXT NOT NULL DEFAULT '3-6 months',
    status TEXT NOT NULL DEFAULT 'Identified',
    owner_id TEXT,
    created_at TEXT NOT NULL,
    realized_at TEXT,
    realized_savings INTEGER
);

CREATE TABLE IF NOT EXISTS spend_kpis (
    id TEXT PRIMARY KEY,
    kpi_name TEXT NOT NULL,
    kpi_type TEXT NOT NULL,
    period TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    target_value REAL NOT NULL DEFAULT 0,
    actual_value REAL NOT NULL DEFAULT 0,
    variance REAL NOT NULL DEFAULT 0,
    variance_percent REAL NOT NULL DEFAULT 0,
    trend TEXT NOT NULL DEFAULT 'Stable',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS supplier_risk_scores (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    score_date TEXT NOT NULL,
    financial_risk REAL NOT NULL DEFAULT 0,
    operational_risk REAL NOT NULL DEFAULT 0,
    compliance_risk REAL NOT NULL DEFAULT 0,
    geographic_risk REAL NOT NULL DEFAULT 0,
    concentration_risk REAL NOT NULL DEFAULT 0,
    overall_risk REAL NOT NULL DEFAULT 0,
    risk_category TEXT NOT NULL DEFAULT 'Medium',
    mitigation_recommendations TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spend_budgets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    fiscal_year INTEGER NOT NULL,
    category_id TEXT,
    department_id TEXT,
    cost_center_id TEXT,
    annual_budget INTEGER NOT NULL DEFAULT 0,
    q1_budget INTEGER NOT NULL DEFAULT 0,
    q2_budget INTEGER NOT NULL DEFAULT 0,
    q3_budget INTEGER NOT NULL DEFAULT 0,
    q4_budget INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spend_budget_actuals (
    id TEXT PRIMARY KEY,
    budget_id TEXT NOT NULL,
    period TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    budget_amount INTEGER NOT NULL DEFAULT 0,
    actual_amount INTEGER NOT NULL DEFAULT 0,
    variance INTEGER NOT NULL DEFAULT 0,
    variance_percent REAL NOT NULL DEFAULT 0,
    forecast_amount INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (budget_id) REFERENCES spend_budgets(id)
);

CREATE TABLE IF NOT EXISTS tail_spend_analyses (
    id TEXT PRIMARY KEY,
    analysis_date TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_spend INTEGER NOT NULL DEFAULT 0,
    tail_spend INTEGER NOT NULL DEFAULT 0,
    tail_percent REAL NOT NULL DEFAULT 0,
    tail_vendor_count INTEGER NOT NULL DEFAULT 0,
    tail_transaction_count INTEGER NOT NULL DEFAULT 0,
    avg_tail_transaction INTEGER NOT NULL DEFAULT 0,
    consolidation_opportunity INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tail_vendors (
    id TEXT PRIMARY KEY,
    analysis_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    vendor_name TEXT NOT NULL,
    total_spend INTEGER NOT NULL DEFAULT 0,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    is_tail INTEGER NOT NULL DEFAULT 1,
    consolidation_candidate INTEGER NOT NULL DEFAULT 0,
    recommended_action TEXT,
    FOREIGN KEY (analysis_id) REFERENCES tail_spend_analyses(id)
);

CREATE TABLE IF NOT EXISTS spend_dashboards (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    owner_id TEXT,
    filters TEXT NOT NULL,
    widgets TEXT NOT NULL,
    refresh_interval INTEGER NOT NULL DEFAULT 300,
    last_refreshed TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spend_reports (
    id TEXT PRIMARY KEY,
    report_name TEXT NOT NULL,
    report_type TEXT NOT NULL,
    parameters TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    generated_by TEXT,
    generated_at TEXT NOT NULL,
    file_path TEXT,
    file_format TEXT NOT NULL DEFAULT 'PDF',
    status TEXT NOT NULL DEFAULT 'Pending'
);

CREATE TABLE IF NOT EXISTS payment_term_analyses (
    id TEXT PRIMARY KEY,
    vendor_id TEXT,
    category_id TEXT,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_spend INTEGER NOT NULL DEFAULT 0,
    avg_payment_terms REAL NOT NULL DEFAULT 0,
    early_payment_count INTEGER NOT NULL DEFAULT 0,
    on_time_count INTEGER NOT NULL DEFAULT 0,
    late_payment_count INTEGER NOT NULL DEFAULT 0,
    early_payment_discounts INTEGER NOT NULL DEFAULT 0,
    late_payment_penalties INTEGER NOT NULL DEFAULT 0,
    working_capital_impact INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

-- Compensation Planning Module
CREATE TABLE IF NOT EXISTS compensation_plans (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    plan_year INTEGER NOT NULL,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    budget_amount INTEGER NOT NULL DEFAULT 0,
    allocated_amount INTEGER NOT NULL DEFAULT 0,
    spent_amount INTEGER NOT NULL DEFAULT 0,
    review_cycle TEXT NOT NULL DEFAULT 'Annual',
    default_merit_budget_percent REAL NOT NULL DEFAULT 3,
    max_merit_percent REAL NOT NULL DEFAULT 10,
    promotion_budget_percent REAL NOT NULL DEFAULT 2,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS compensation_budgets (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    department_id TEXT,
    cost_center_id TEXT,
    budget_type TEXT NOT NULL DEFAULT 'Merit',
    allocated_amount INTEGER NOT NULL DEFAULT 0,
    committed_amount INTEGER NOT NULL DEFAULT 0,
    spent_amount INTEGER NOT NULL DEFAULT 0,
    remaining_amount INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES compensation_plans(id)
);

CREATE TABLE IF NOT EXISTS salary_ranges (
    id TEXT PRIMARY KEY,
    grade_id TEXT,
    job_family_id TEXT,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    min_salary INTEGER NOT NULL DEFAULT 0,
    mid_salary INTEGER NOT NULL DEFAULT 0,
    max_salary INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS job_grades (
    id TEXT PRIMARY KEY,
    grade_code TEXT NOT NULL UNIQUE,
    grade_name TEXT NOT NULL,
    description TEXT,
    level INTEGER NOT NULL DEFAULT 1,
    job_family_id TEXT,
    salary_range_id TEXT,
    min_experience_years INTEGER NOT NULL DEFAULT 0,
    typical_title TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS job_families (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    department_id TEXT,
    parent_family_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS employee_compensations (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    base_salary INTEGER NOT NULL DEFAULT 0,
    hourly_rate INTEGER,
    pay_frequency TEXT NOT NULL DEFAULT 'Monthly',
    currency TEXT NOT NULL DEFAULT 'USD',
    grade_id TEXT,
    salary_range_id TEXT,
    compa_ratio REAL NOT NULL DEFAULT 0,
    position_in_range REAL NOT NULL DEFAULT 0,
    is_current INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS compensation_components (
    id TEXT PRIMARY KEY,
    employee_compensation_id TEXT NOT NULL,
    component_type TEXT NOT NULL DEFAULT 'BaseSalary',
    name TEXT NOT NULL,
    amount INTEGER NOT NULL DEFAULT 0,
    frequency TEXT NOT NULL DEFAULT 'Monthly',
    is_recurring INTEGER NOT NULL DEFAULT 1,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    FOREIGN KEY (employee_compensation_id) REFERENCES employee_compensations(id)
);

CREATE TABLE IF NOT EXISTS compensation_adjustments (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    plan_id TEXT,
    adjustment_type TEXT NOT NULL DEFAULT 'Merit',
    current_base INTEGER NOT NULL DEFAULT 0,
    new_base INTEGER NOT NULL DEFAULT 0,
    adjustment_amount INTEGER NOT NULL DEFAULT 0,
    adjustment_percent REAL NOT NULL DEFAULT 0,
    effective_date TEXT NOT NULL,
    reason TEXT NOT NULL,
    justification TEXT,
    old_grade_id TEXT,
    new_grade_id TEXT,
    old_position_id TEXT,
    new_position_id TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    approved_by TEXT,
    approved_at TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES compensation_plans(id)
);

CREATE TABLE IF NOT EXISTS merit_matrices (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    performance_levels INTEGER NOT NULL DEFAULT 5,
    compa_ratio_buckets INTEGER NOT NULL DEFAULT 3,
    matrix_data TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES compensation_plans(id)
);

CREATE TABLE IF NOT EXISTS merit_guidelines (
    id TEXT PRIMARY KEY,
    matrix_id TEXT NOT NULL,
    performance_rating INTEGER NOT NULL DEFAULT 3,
    compa_ratio_min REAL NOT NULL DEFAULT 0,
    compa_ratio_max REAL NOT NULL DEFAULT 100,
    recommended_increase_min REAL NOT NULL DEFAULT 0,
    recommended_increase_mid REAL NOT NULL DEFAULT 0,
    recommended_increase_max REAL NOT NULL DEFAULT 0,
    FOREIGN KEY (matrix_id) REFERENCES merit_matrices(id)
);

CREATE TABLE IF NOT EXISTS bonus_plans (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    plan_type TEXT NOT NULL DEFAULT 'Annual',
    target_percent REAL NOT NULL DEFAULT 10,
    max_percent REAL NOT NULL DEFAULT 20,
    funding_formula TEXT NOT NULL,
    performance_weights TEXT NOT NULL,
    fiscal_year INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS employee_bonuses (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    bonus_plan_id TEXT NOT NULL,
    fiscal_year INTEGER NOT NULL,
    target_amount INTEGER NOT NULL DEFAULT 0,
    company_performance_factor REAL NOT NULL DEFAULT 1,
    individual_performance_factor REAL NOT NULL DEFAULT 1,
    calculated_amount INTEGER NOT NULL DEFAULT 0,
    recommended_amount INTEGER NOT NULL DEFAULT 0,
    approved_amount INTEGER,
    status TEXT NOT NULL DEFAULT 'Draft',
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (bonus_plan_id) REFERENCES bonus_plans(id)
);

CREATE TABLE IF NOT EXISTS equity_grants (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    grant_number TEXT NOT NULL UNIQUE,
    grant_type TEXT NOT NULL DEFAULT 'RSU',
    shares INTEGER NOT NULL DEFAULT 0,
    strike_price INTEGER NOT NULL DEFAULT 0,
    grant_date TEXT NOT NULL,
    vest_start_date TEXT NOT NULL,
    vest_schedule TEXT NOT NULL DEFAULT '4-year cliff',
    vesting_years INTEGER NOT NULL DEFAULT 4,
    cliff_months INTEGER NOT NULL DEFAULT 12,
    vested_shares INTEGER NOT NULL DEFAULT 0,
    unvested_shares INTEGER NOT NULL DEFAULT 0,
    forfeited_shares INTEGER NOT NULL DEFAULT 0,
    expiration_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS equity_vesting_events (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    vest_date TEXT NOT NULL,
    shares INTEGER NOT NULL DEFAULT 0,
    cumulative_shares INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    processed_at TEXT,
    FOREIGN KEY (grant_id) REFERENCES equity_grants(id)
);

CREATE TABLE IF NOT EXISTS market_data (
    id TEXT PRIMARY KEY,
    source TEXT NOT NULL,
    survey_date TEXT NOT NULL,
    job_code TEXT NOT NULL,
    job_title TEXT NOT NULL,
    industry TEXT,
    region TEXT,
    company_size TEXT,
    p10 INTEGER NOT NULL DEFAULT 0,
    p25 INTEGER NOT NULL DEFAULT 0,
    p50 INTEGER NOT NULL DEFAULT 0,
    p75 INTEGER NOT NULL DEFAULT 0,
    p90 INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS market_data_mappings (
    id TEXT PRIMARY KEY,
    position_id TEXT NOT NULL,
    market_data_id TEXT NOT NULL,
    match_quality REAL NOT NULL DEFAULT 0,
    effective_date TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (market_data_id) REFERENCES market_data(id)
);

CREATE TABLE IF NOT EXISTS compensation_reviews (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    reviewer_id TEXT NOT NULL,
    review_date TEXT NOT NULL,
    current_salary INTEGER NOT NULL DEFAULT 0,
    proposed_salary INTEGER NOT NULL DEFAULT 0,
    proposed_increase_percent REAL NOT NULL DEFAULT 0,
    performance_rating INTEGER,
    potential_rating INTEGER,
    compa_ratio_current REAL NOT NULL DEFAULT 0,
    compa_ratio_proposed REAL NOT NULL DEFAULT 0,
    merit_recommendation TEXT NOT NULL,
    promotion_recommendation INTEGER NOT NULL DEFAULT 0,
    retention_risk TEXT NOT NULL DEFAULT 'Low',
    comments TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (plan_id) REFERENCES compensation_plans(id)
);

CREATE TABLE IF NOT EXISTS compensation_approvals (
    id TEXT PRIMARY KEY,
    review_id TEXT NOT NULL,
    approver_id TEXT NOT NULL,
    approval_level INTEGER NOT NULL DEFAULT 1,
    original_amount INTEGER NOT NULL DEFAULT 0,
    approved_amount INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    comments TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (review_id) REFERENCES compensation_reviews(id)
);

CREATE TABLE IF NOT EXISTS compensation_histories (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    change_type TEXT NOT NULL,
    previous_value INTEGER NOT NULL DEFAULT 0,
    new_value INTEGER NOT NULL DEFAULT 0,
    change_amount INTEGER NOT NULL DEFAULT 0,
    change_percent REAL NOT NULL DEFAULT 0,
    reason TEXT NOT NULL,
    approved_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS total_rewards_statements (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    statement_year INTEGER NOT NULL,
    base_salary INTEGER NOT NULL DEFAULT 0,
    variable_pay INTEGER NOT NULL DEFAULT 0,
    benefits_value INTEGER NOT NULL DEFAULT 0,
    equity_value INTEGER NOT NULL DEFAULT 0,
    other_compensation INTEGER NOT NULL DEFAULT 0,
    total_compensation INTEGER NOT NULL DEFAULT 0,
    generated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pay_equity_analyses (
    id TEXT PRIMARY KEY,
    analysis_date TEXT NOT NULL,
    analysis_type TEXT NOT NULL DEFAULT 'Gender',
    group_type TEXT NOT NULL,
    group_id TEXT,
    employee_count INTEGER NOT NULL DEFAULT 0,
    avg_salary_male INTEGER NOT NULL DEFAULT 0,
    avg_salary_female INTEGER NOT NULL DEFAULT 0,
    pay_gap_percent REAL NOT NULL DEFAULT 0,
    adjusted_gap_percent REAL NOT NULL DEFAULT 0,
    statistical_significance REAL NOT NULL DEFAULT 0,
    findings TEXT,
    recommendations TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS compensation_benchmarks (
    id TEXT PRIMARY KEY,
    employee_id TEXT,
    position_id TEXT,
    current_salary INTEGER NOT NULL DEFAULT 0,
    market_p50 INTEGER NOT NULL DEFAULT 0,
    market_p75 INTEGER NOT NULL DEFAULT 0,
    market_p90 INTEGER NOT NULL DEFAULT 0,
    compa_ratio_p50 REAL NOT NULL DEFAULT 0,
    percentile REAL NOT NULL DEFAULT 50,
    benchmark_date TEXT NOT NULL,
    market_data_source TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS retention_allowances (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    allowance_type TEXT NOT NULL DEFAULT 'Retention',
    amount INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    clawback_period_months INTEGER NOT NULL DEFAULT 12,
    payment_schedule TEXT NOT NULL DEFAULT 'LumpSum',
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS geographic_differentials (
    id TEXT PRIMARY KEY,
    location_id TEXT NOT NULL,
    location_name TEXT NOT NULL,
    differential_percent REAL NOT NULL DEFAULT 0,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL
);

-- Indexes for new tables
CREATE INDEX IF NOT EXISTS idx_configurations_template ON product_configurations(template_id);
CREATE INDEX IF NOT EXISTS idx_configured_quotes_customer ON configured_quotes(customer_id);
CREATE INDEX IF NOT EXISTS idx_contracts_vendor ON contracts(vendor_id);
CREATE INDEX IF NOT EXISTS idx_contracts_status ON contracts(status);
CREATE INDEX IF NOT EXISTS idx_contracts_end_date ON contracts(end_date);
CREATE INDEX IF NOT EXISTS idx_commission_calcs_rep ON commission_calculations(sales_rep_id);
CREATE INDEX IF NOT EXISTS idx_commission_calcs_status ON commission_calculations(status);
CREATE INDEX IF NOT EXISTS idx_spend_transactions_vendor ON spend_transactions(vendor_id);
CREATE INDEX IF NOT EXISTS idx_spend_transactions_date ON spend_transactions(transaction_date);
CREATE INDEX IF NOT EXISTS idx_mps_items_product ON mps_items(product_id);
CREATE INDEX IF NOT EXISTS idx_mrp_items_product ON mrp_items(product_id);
CREATE INDEX IF NOT EXISTS idx_schedule_ops_resource ON schedule_operations(resource_id);
CREATE INDEX IF NOT EXISTS idx_employee_comp_employee ON employee_compensations(employee_id);
CREATE INDEX IF NOT EXISTS idx_comp_adjustments_employee ON compensation_adjustments(employee_id);
