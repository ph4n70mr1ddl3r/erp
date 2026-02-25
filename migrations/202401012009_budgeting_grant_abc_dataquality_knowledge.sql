-- Budgeting & Forecasting
CREATE TABLE IF NOT EXISTS budgets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    budget_type TEXT NOT NULL,
    status TEXT NOT NULL,
    fiscal_year INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_amount_cents INTEGER NOT NULL,
    currency TEXT NOT NULL,
    department_id TEXT,
    project_id TEXT,
    owner_id TEXT NOT NULL,
    approval_workflow_id TEXT,
    version INTEGER NOT NULL DEFAULT 1,
    parent_budget_id TEXT,
    is_template INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_lines (
    id TEXT PRIMARY KEY,
    budget_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    account_code TEXT NOT NULL,
    account_name TEXT NOT NULL,
    description TEXT,
    planned_amount_cents INTEGER NOT NULL DEFAULT 0,
    committed_amount_cents INTEGER NOT NULL DEFAULT 0,
    actual_amount_cents INTEGER NOT NULL DEFAULT 0,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    cost_center_id TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_periods (
    id TEXT PRIMARY KEY,
    budget_id TEXT NOT NULL,
    period_type TEXT NOT NULL,
    period_number INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    planned_amount_cents INTEGER NOT NULL DEFAULT 0,
    actual_amount_cents INTEGER NOT NULL DEFAULT 0,
    is_locked INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS forecasts (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    forecast_type TEXT NOT NULL,
    method TEXT NOT NULL,
    fiscal_year INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_forecast_cents INTEGER NOT NULL DEFAULT 0,
    confidence_level REAL NOT NULL DEFAULT 0,
    created_by TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS forecast_lines (
    id TEXT PRIMARY KEY,
    forecast_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    period_date TEXT NOT NULL,
    forecasted_amount_cents INTEGER NOT NULL DEFAULT 0,
    actual_amount_cents,
    accuracy_score REAL,
    factors TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_transfers (
    id TEXT PRIMARY KEY,
    from_budget_line_id TEXT NOT NULL,
    to_budget_line_id TEXT NOT NULL,
    amount_cents INTEGER NOT NULL,
    reason TEXT NOT NULL,
    requested_by TEXT NOT NULL,
    approved_by TEXT,
    approved_at TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_scenarios (
    id TEXT PRIMARY KEY,
    budget_id TEXT NOT NULL,
    name TEXT NOT NULL,
    scenario_type TEXT NOT NULL,
    adjustment_factor REAL NOT NULL DEFAULT 1.0,
    description TEXT,
    total_amount_cents INTEGER NOT NULL DEFAULT 0,
    is_baseline INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_alerts (
    id TEXT PRIMARY KEY,
    budget_id TEXT NOT NULL,
    budget_line_id_id TEXT,
    alert_type TEXT NOT NULL,
    threshold_percent REAL NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_triggered TEXT,
    notify_users TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_approvals (
    id TEXT PRIMARY KEY,
    budget_id TEXT NOT NULL,
    approver_id TEXT NOT NULL,
    approval_level INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL,
    comments TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_actuals (
    id TEXT PRIMARY KEY,
    budget_line_id TEXT NOT NULL,
    transaction_date TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    reference_id TEXT,
    amount_cents INTEGER NOT NULL,
    description TEXT,
    source_module TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Grant Management
CREATE TABLE IF NOT EXISTS grants (
    id TEXT PRIMARY KEY,
    grant_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    grant_type TEXT NOT NULL,
    status TEXT NOT NULL,
    funding_source TEXT NOT NULL,
    funder_name TEXT NOT NULL,
    funder_contact TEXT,
    total_award_amount_cents INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    indirect_cost_rate REAL NOT NULL DEFAULT 0,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    principal_investigator_id TEXT NOT NULL,
    department_id TEXT,
    program_id TEXT,
    cfda_number TEXT,
    award_number TEXT,
    is_cost_sharing INTEGER NOT NULL DEFAULT 0,
    cost_sharing_amount_cents INTEGER,
    reporting_frequency TEXT NOT NULL DEFAULT 'Quarterly',
    next_report_due TEXT,
    compliance_requirements TEXT,
    special_conditions TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grant_budgets (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    budget_category TEXT NOT NULL,
    description TEXT NOT NULL,
    approved_amount_cents INTEGER NOT NULL DEFAULT 0,
    budgeted_amount_cents INTEGER NOT NULL DEFAULT 0,
    expended_amount_cents INTEGER NOT NULL DEFAULT 0,
    encumbered_amount_cents INTEGER NOT NULL DEFAULT 0,
    available_balance_cents INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grant_personnel (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    role TEXT NOT NULL,
    effort_percent REAL NOT NULL DEFAULT 0,
    hourly_rate_cents INTEGER NOT NULL DEFAULT 0,
    total_budgeted_cents INTEGER NOT NULL DEFAULT 0,
    total_charged_cents INTEGER NOT NULL DEFAULT 0,
    start_date TEXT NOT NULL,
    end_date TEXT,
    is_key_personnel INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grant_transactions (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    budget_category TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    transaction_date TEXT NOT NULL,
    amount_cents INTEGER NOT NULL,
    description TEXT NOT NULL,
    reference_number TEXT,
    invoice_id TEXT,
    journal_entry_id TEXT,
    approved_by TEXT,
    cost_sharing_flag INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grant_milestones (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    due_date TEXT NOT NULL,
    completed_date TEXT,
    status TEXT NOT NULL DEFAULT 'NotStarted',
    deliverables TEXT,
    is_payment_trigger INTEGER NOT NULL DEFAULT 0,
    payment_amount_cents INTEGER,
    completed_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grant_reports (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    report_type TEXT NOT NULL,
    reporting_period_start TEXT NOT NULL,
    reporting_period_end TEXT NOT NULL,
    due_date TEXT NOT NULL,
    submitted_date TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    prepared_by TEXT,
    approved_by TEXT,
    notes TEXT,
    attachment_ids TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grant_compliance (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    requirement_type TEXT NOT NULL,
    description TEXT NOT NULL,
    is_mandatory INTEGER NOT NULL DEFAULT 1,
    due_date TEXT,
    completed_date TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    responsible_party TEXT,
    documentation_ids TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grant_subawards (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    subrecipient_id TEXT NOT NULL,
    subrecipient_name TEXT NOT NULL,
    subaward_number TEXT NOT NULL,
    total_amount_cents INTEGER NOT NULL DEFAULT 0,
    disbursed_amount_cents INTEGER NOT NULL DEFAULT 0,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    contact_person TEXT,
    contact_email TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS grant_closeouts (
    id TEXT PRIMARY KEY,
    grant_id TEXT NOT NULL,
    closeout_date TEXT NOT NULL,
    final_expenditure_cents INTEGER NOT NULL DEFAULT 0,
    unexpended_balance_cents INTEGER NOT NULL DEFAULT 0,
    final_report_submitted INTEGER NOT NULL DEFAULT 0,
    equipment_inventory_complete INTEGER NOT NULL DEFAULT 0,
    inventions_reported INTEGER NOT NULL DEFAULT 0,
    subawards_closed INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Initiated',
    closed_by TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Activity-Based Costing
CREATE TABLE IF NOT EXISTS cost_pools (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    pool_type TEXT NOT NULL,
    total_cost_cents INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    fiscal_year INTEGER NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS activities (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    activity_type TEXT NOT NULL,
    cost_pool_id TEXT NOT NULL,
    total_cost_cents INTEGER NOT NULL DEFAULT 0,
    cost_driver_id TEXT,
    driver_quantity REAL NOT NULL DEFAULT 0,
    cost_per_driver_cents INTEGER NOT NULL DEFAULT 0,
    department_id TEXT,
    process_id TEXT,
    is_value_added INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cost_drivers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    driver_type TEXT NOT NULL,
    unit_of_measure TEXT NOT NULL,
    total_capacity REAL NOT NULL DEFAULT 0,
    used_capacity REAL NOT NULL DEFAULT 0,
    unused_capacity REAL NOT NULL DEFAULT 0,
    utilization_percent REAL NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cost_objects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    object_type TEXT NOT NULL,
    parent_id TEXT,
    direct_cost_cents INTEGER NOT NULL DEFAULT 0,
    indirect_cost_cents INTEGER NOT NULL DEFAULT 0,
    total_cost_cents INTEGER NOT NULL DEFAULT 0,
    revenue_cents INTEGER NOT NULL DEFAULT 0,
    profit_margin_cents INTEGER NOT NULL DEFAULT 0,
    profit_margin_percent REAL NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS activity_allocations (
    id TEXT PRIMARY KEY,
    activity_id TEXT NOT NULL,
    cost_object_id TEXT NOT NULL,
    cost_pool_id TEXT NOT NULL,
    driver_quantity REAL NOT NULL DEFAULT 0,
    allocation_rate_cents INTEGER NOT NULL DEFAULT 0,
    allocated_amount_cents INTEGER NOT NULL DEFAULT 0,
    allocation_date TEXT NOT NULL,
    fiscal_period TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS processes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    owner_id TEXT,
    department_id TEXT,
    total_activities INTEGER NOT NULL DEFAULT 0,
    total_cost_cents INTEGER NOT NULL DEFAULT 0,
    cycle_time_hours REAL NOT NULL DEFAULT 0,
    is_core_process INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Knowledge Base
CREATE TABLE IF NOT EXISTS knowledge_articles (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL,
    summary TEXT,
    article_type TEXT NOT NULL,
    category TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    author_id TEXT NOT NULL,
    reviewer_id TEXT,
    published_at TEXT,
    expires_at TEXT,
    version INTEGER NOT NULL DEFAULT 1,
    parent_id TEXT,
    tags TEXT,
    view_count INTEGER NOT NULL DEFAULT 0,
    helpful_count INTEGER NOT NULL DEFAULT 0,
    not_helpful_count INTEGER NOT NULL DEFAULT 0,
    average_rating REAL NOT NULL DEFAULT 0,
    rating_count INTEGER NOT NULL DEFAULT 0,
    is_featured INTEGER NOT NULL DEFAULT 0,
    is_internal INTEGER NOT NULL DEFAULT 0,
    language TEXT NOT NULL DEFAULT 'en',
    related_articles TEXT,
    attachments TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS knowledge_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT,
    parent_id TEXT,
    icon TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    article_count INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS article_versions (
    id TEXT PRIMARY KEY,
    article_id TEXT NOT NULL,
    version INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    change_summary TEXT,
    author_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    is_current INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS article_feedback (
    id TEXT PRIMARY KEY,
    article_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    is_helpful INTEGER NOT NULL,
    rating INTEGER,
    comment TEXT,
    submitted_at TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Help Desk
CREATE TABLE IF NOT EXISTS tickets (
    id TEXT PRIMARY KEY,
    ticket_number TEXT NOT NULL UNIQUE,
    subject TEXT NOT NULL,
    description TEXT NOT NULL,
    ticket_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'New',
    priority TEXT NOT NULL DEFAULT 'Medium',
    source TEXT NOT NULL DEFAULT 'Web',
    requester_id TEXT NOT NULL,
    requester_email TEXT NOT NULL,
    requester_name TEXT NOT NULL,
    assignee_id TEXT,
    team_id TEXT,
    department_id TEXT,
    category_id TEXT,
    subcategory_id TEXT,
    due_date TEXT,
    resolution_date TEXT,
    first_response_at TEXT,
    closed_at TEXT,
    sla_id TEXT,
    sla_breached INTEGER NOT NULL DEFAULT 0,
    satisfaction_rating INTEGER,
    satisfaction_comment TEXT,
    tags TEXT,
    custom_fields TEXT,
    related_tickets TEXT,
    parent_ticket_id TEXT,
    knowledge_article_id TEXT,
    asset_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ticket_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ticket_comments (
    id TEXT PRIMARY KEY,
    ticket_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    author_name TEXT NOT NULL,
    content TEXT NOT NULL,
    comment_type TEXT NOT NULL DEFAULT 'Reply',
    is_internal INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    attachments TEXT
);

CREATE TABLE IF NOT EXISTS support_teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    email TEXT NOT NULL,
    leader_id TEXT,
    members TEXT,
    category_ids TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    working_hours TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sla_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    priority_rules TEXT,
    calendar_id TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Data Quality
CREATE TABLE IF NOT EXISTS data_quality_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    rule_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    target_entity TEXT NOT NULL,
    target_field TEXT NOT NULL,
    condition TEXT NOT NULL,
    threshold REAL,
    is_active INTEGER NOT NULL DEFAULT 1,
    schedule TEXT,
    last_run TEXT,
    last_result TEXT,
    tags TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS data_quality_executions (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL,
    executed_at TEXT NOT NULL,
    duration_ms INTEGER NOT NULL,
    status TEXT NOT NULL,
    score TEXT NOT NULL,
    errors TEXT,
    warnings TEXT,
    records_processed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS data_quality_profiles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    entity TEXT NOT NULL,
    profile_date TEXT NOT NULL,
    total_records INTEGER NOT NULL DEFAULT 0,
    field_profiles TEXT,
    overall_quality_score REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS data_cleansing_jobs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    source_entity TEXT NOT NULL,
    target_entity TEXT,
    transformations TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_by TEXT NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    records_processed INTEGER NOT NULL DEFAULT 0,
    records_modified INTEGER NOT NULL DEFAULT 0,
    records_failed INTEGER NOT NULL DEFAULT 0,
    error_log TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS data_matching_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    entity TEXT NOT NULL,
    match_fields TEXT,
    blocking_keys TEXT,
    match_threshold REAL NOT NULL DEFAULT 0.8,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS duplicate_groups (
    id TEXT PRIMARY KEY,
    entity TEXT NOT NULL,
    canonical_id TEXT NOT NULL,
    duplicate_ids TEXT,
    match_score REAL NOT NULL,
    detected_at TEXT NOT NULL,
    resolved_at TEXT,
    resolved_by TEXT,
    resolution_type TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
