-- Quality Management System Tables
CREATE TABLE IF NOT EXISTS capas (
    id TEXT PRIMARY KEY,
    capa_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    source TEXT NOT NULL,
    severity TEXT NOT NULL,
    status TEXT NOT NULL,
    current_phase TEXT NOT NULL,
    product_id TEXT,
    process_id TEXT,
    supplier_id TEXT,
    customer_id TEXT,
    department_id TEXT,
    detected_date TEXT NOT NULL,
    detected_by TEXT,
    assigned_to TEXT,
    assigned_date TEXT,
    target_completion_date TEXT,
    actual_completion_date TEXT,
    effectiveness_review_date TEXT,
    is_effective INTEGER,
    closure_date TEXT,
    closed_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS capa_investigations (
    id TEXT PRIMARY KEY,
    capa_id TEXT NOT NULL,
    investigation_date TEXT NOT NULL,
    investigator_id TEXT,
    what_happened TEXT NOT NULL,
    when_it_happened TEXT,
    where_it_happened TEXT,
    who_was_involved TEXT,
    immediate_action_taken TEXT,
    evidence_collected TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (capa_id) REFERENCES capas(id)
);

CREATE TABLE IF NOT EXISTS root_cause_analyses (
    id TEXT PRIMARY KEY,
    capa_id TEXT NOT NULL,
    analysis_method TEXT NOT NULL,
    root_cause_category TEXT NOT NULL,
    root_cause_description TEXT NOT NULL,
    contributing_factors TEXT,
    is_primary INTEGER NOT NULL,
    verified INTEGER NOT NULL,
    verified_by TEXT,
    verified_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (capa_id) REFERENCES capas(id)
);

CREATE TABLE IF NOT EXISTS capa_actions (
    id TEXT PRIMARY KEY,
    capa_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    action_number INTEGER NOT NULL,
    description TEXT NOT NULL,
    root_cause_id TEXT,
    responsible_person_id TEXT,
    planned_date TEXT NOT NULL,
    completed_date TEXT,
    status TEXT NOT NULL,
    verification_method TEXT,
    verification_date TEXT,
    verified_by TEXT,
    effectiveness_notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (capa_id) REFERENCES capas(id)
);

CREATE TABLE IF NOT EXISTS audit_programs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    audit_type TEXT NOT NULL,
    standard_reference TEXT,
    frequency_months INTEGER NOT NULL,
    next_audit_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS audit_schedules (
    id TEXT PRIMARY KEY,
    audit_number TEXT NOT NULL UNIQUE,
    program_id TEXT,
    title TEXT NOT NULL,
    audit_type TEXT NOT NULL,
    scope TEXT NOT NULL,
    objectives TEXT,
    criteria TEXT,
    planned_start_date TEXT NOT NULL,
    planned_end_date TEXT NOT NULL,
    actual_start_date TEXT,
    actual_end_date TEXT,
    lead_auditor_id TEXT,
    status TEXT NOT NULL,
    overall_rating TEXT,
    summary TEXT,
    conclusions TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (program_id) REFERENCES audit_programs(id)
);

CREATE TABLE IF NOT EXISTS calibration_equipment (
    id TEXT PRIMARY KEY,
    equipment_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    serial_number TEXT,
    model TEXT,
    manufacturer TEXT,
    equipment_type TEXT NOT NULL,
    location_id TEXT,
    department_id TEXT,
    responsible_person_id TEXT,
    calibration_frequency_months INTEGER NOT NULL,
    last_calibration_date TEXT,
    next_calibration_date TEXT,
    calibration_status TEXT NOT NULL,
    accuracy_class TEXT,
    measurement_range TEXT,
    resolution TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS calibration_records (
    id TEXT PRIMARY KEY,
    calibration_number TEXT NOT NULL UNIQUE,
    equipment_id TEXT NOT NULL,
    calibration_date TEXT NOT NULL,
    calibration_type TEXT NOT NULL,
    calibration_lab TEXT,
    lab_certificate_number TEXT,
    performed_by TEXT,
    environmental_conditions TEXT,
    standards_used TEXT,
    before_calibration TEXT,
    after_calibration TEXT,
    as_found_status TEXT NOT NULL,
    as_left_status TEXT NOT NULL,
    result TEXT NOT NULL,
    next_calibration_date TEXT NOT NULL,
    cost INTEGER NOT NULL,
    currency TEXT NOT NULL,
    certificate_file TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (equipment_id) REFERENCES calibration_equipment(id)
);

CREATE TABLE IF NOT EXISTS control_plans (
    id TEXT PRIMARY KEY,
    plan_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    product_id TEXT,
    process_id TEXT,
    revision TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    approval_status TEXT NOT NULL,
    approved_by TEXT,
    approved_at TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS document_controls (
    id TEXT PRIMARY KEY,
    document_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    document_type TEXT NOT NULL,
    category TEXT,
    department_id TEXT,
    owner_id TEXT,
    current_revision TEXT NOT NULL,
    effective_date TEXT,
    review_frequency_months INTEGER NOT NULL,
    next_review_date TEXT,
    approval_workflow_id TEXT,
    distribution_list TEXT,
    retention_years INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS spc_charts (
    id TEXT PRIMARY KEY,
    spc_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    product_id TEXT,
    process_id TEXT,
    characteristic TEXT NOT NULL,
    specification_min REAL NOT NULL,
    specification_max REAL NOT NULL,
    target_value REAL NOT NULL,
    unit TEXT NOT NULL,
    sample_size INTEGER NOT NULL,
    sampling_frequency TEXT NOT NULL,
    subgroup_size INTEGER NOT NULL,
    control_chart_type TEXT NOT NULL,
    ucl REAL NOT NULL,
    lcl REAL NOT NULL,
    center_line REAL NOT NULL,
    rule_set TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fmea_analyses (
    id TEXT PRIMARY KEY,
    fmea_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    fmea_type TEXT NOT NULL,
    product_id TEXT,
    process_id TEXT,
    prepared_by TEXT,
    prepared_date TEXT NOT NULL,
    revision TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Treasury Management Tables
CREATE TABLE IF NOT EXISTS cash_pools (
    id TEXT PRIMARY KEY,
    pool_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    pool_type TEXT NOT NULL,
    header_account_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    target_balance INTEGER NOT NULL,
    min_balance INTEGER NOT NULL,
    max_balance INTEGER,
    interest_allocation_method TEXT,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_positions (
    id TEXT PRIMARY KEY,
    position_date TEXT NOT NULL,
    currency TEXT NOT NULL,
    opening_balance INTEGER NOT NULL,
    receipts INTEGER NOT NULL,
    disbursements INTEGER NOT NULL,
    transfers_in INTEGER NOT NULL,
    transfers_out INTEGER NOT NULL,
    fx_gains INTEGER NOT NULL,
    fx_losses INTEGER NOT NULL,
    closing_balance INTEGER NOT NULL,
    available_balance INTEGER NOT NULL,
    invested_balance INTEGER NOT NULL,
    borrowed_balance INTEGER NOT NULL,
    net_position INTEGER NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_forecasts (
    id TEXT PRIMARY KEY,
    forecast_number TEXT NOT NULL UNIQUE,
    forecast_date TEXT NOT NULL,
    horizon_days INTEGER NOT NULL,
    currency TEXT NOT NULL,
    company_id TEXT,
    scenario TEXT NOT NULL,
    status TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS investments (
    id TEXT PRIMARY KEY,
    investment_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    investment_type TEXT NOT NULL,
    issuer TEXT NOT NULL,
    currency TEXT NOT NULL,
    principal_amount INTEGER NOT NULL,
    current_value INTEGER NOT NULL,
    purchase_date TEXT NOT NULL,
    maturity_date TEXT NOT NULL,
    purchase_price INTEGER NOT NULL,
    coupon_rate REAL,
    yield_to_maturity REAL,
    yield_current REAL,
    credit_rating TEXT,
    cusip TEXT,
    isin TEXT,
    counterparty_id TEXT,
    custodian TEXT,
    account_id TEXT,
    accrued_interest INTEGER NOT NULL,
    unrealized_gain_loss INTEGER NOT NULL,
    realized_gain_loss INTEGER NOT NULL,
    day_count_convention TEXT NOT NULL,
    payment_frequency TEXT NOT NULL,
    next_payment_date TEXT,
    call_date TEXT,
    call_price INTEGER,
    put_date TEXT,
    put_price INTEGER,
    status TEXT NOT NULL,
    policy_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS credit_facilities (
    id TEXT PRIMARY KEY,
    facility_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    facility_type TEXT NOT NULL,
    lender_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    committed_amount INTEGER NOT NULL,
    available_amount INTEGER NOT NULL,
    drawn_amount INTEGER NOT NULL,
    undrawn_amount INTEGER NOT NULL,
    interest_rate_type TEXT NOT NULL,
    base_rate TEXT,
    margin_rate REAL NOT NULL,
    all_in_rate REAL,
    commitment_fee REAL NOT NULL,
    facility_fee INTEGER NOT NULL,
    utilization_fee REAL,
    effective_date TEXT NOT NULL,
    maturity_date TEXT NOT NULL,
    renewal_date TEXT,
    financial_covenants TEXT,
    borrowing_base_formula TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS hedge_instruments (
    id TEXT PRIMARY KEY,
    hedge_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    hedge_type TEXT NOT NULL,
    hedge_purpose TEXT NOT NULL,
    underlying TEXT NOT NULL,
    notional_amount INTEGER NOT NULL,
    notional_currency TEXT NOT NULL,
    counter_currency TEXT,
    strike_price REAL,
    forward_rate REAL,
    spot_rate REAL,
    premium INTEGER,
    premium_currency TEXT,
    trade_date TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    maturity_date TEXT NOT NULL,
    settlement_date TEXT,
    counterparty_id TEXT,
    hedge_accounting INTEGER NOT NULL,
    effectiveness_method TEXT,
    effectiveness_status TEXT,
    designation_document TEXT,
    fair_value INTEGER,
    unrealized_gain_loss INTEGER,
    realized_gain_loss INTEGER,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Sustainability/ESG Tables
CREATE TABLE IF NOT EXISTS carbon_accounts (
    id TEXT PRIMARY KEY,
    account_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    emission_scope TEXT NOT NULL,
    emission_category TEXT NOT NULL,
    location_id TEXT,
    department_id TEXT,
    responsible_person_id TEXT,
    reporting_standard TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS carbon_emissions (
    id TEXT PRIMARY KEY,
    emission_number TEXT NOT NULL UNIQUE,
    account_id TEXT NOT NULL,
    emission_date TEXT NOT NULL,
    reporting_period TEXT NOT NULL,
    activity_data REAL NOT NULL,
    activity_unit TEXT NOT NULL,
    emission_factor REAL NOT NULL,
    emission_factor_unit TEXT NOT NULL,
    emission_factor_source TEXT,
    co2_equivalent REAL NOT NULL,
    co2_equivalent_unit TEXT NOT NULL,
    uncertainty_percent REAL,
    data_quality_score INTEGER,
    source_type TEXT,
    source_id TEXT,
    verification_status TEXT NOT NULL,
    verified_by TEXT,
    verified_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES carbon_accounts(id)
);

CREATE TABLE IF NOT EXISTS emission_factors (
    id TEXT PRIMARY KEY,
    factor_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    emission_category TEXT NOT NULL,
    fuel_type TEXT,
    region TEXT,
    year INTEGER NOT NULL,
    factor_value REAL NOT NULL,
    factor_unit TEXT NOT NULL,
    co2_factor REAL,
    ch4_factor REAL,
    n2o_factor REAL,
    gwp_co2 REAL,
    gwp_ch4 REAL,
    gwp_n2o REAL,
    source TEXT NOT NULL,
    source_url TEXT,
    uncertainty_percent REAL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS carbon_offsets (
    id TEXT PRIMARY KEY,
    offset_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    project_name TEXT NOT NULL,
    project_type TEXT NOT NULL,
    standard TEXT NOT NULL,
    registry TEXT NOT NULL,
    registry_id TEXT,
    vintage_year INTEGER NOT NULL,
    quantity_tonnes INTEGER NOT NULL,
    remaining_tonnes INTEGER NOT NULL,
    price_per_tonne INTEGER NOT NULL,
    currency TEXT NOT NULL,
    purchase_date TEXT NOT NULL,
    retirement_date TEXT,
    project_location TEXT,
    co_benefits TEXT,
    sdg_contributions TEXT,
    verification_body TEXT,
    verification_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS energy_consumption (
    id TEXT PRIMARY KEY,
    consumption_number TEXT NOT NULL UNIQUE,
    facility_id TEXT,
    consumption_date TEXT NOT NULL,
    reporting_period TEXT NOT NULL,
    energy_type TEXT NOT NULL,
    consumption_value REAL NOT NULL,
    consumption_unit TEXT NOT NULL,
    supplier TEXT,
    meter_id TEXT,
    cost INTEGER NOT NULL,
    currency TEXT NOT NULL,
    renewable_percent REAL NOT NULL,
    renewable_source TEXT,
    location_based_emissions REAL,
    market_based_emissions REAL,
    data_source TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS water_consumption (
    id TEXT PRIMARY KEY,
    consumption_number TEXT NOT NULL UNIQUE,
    facility_id TEXT,
    consumption_date TEXT NOT NULL,
    reporting_period TEXT NOT NULL,
    water_source TEXT NOT NULL,
    consumption_cubic_meters REAL NOT NULL,
    withdrawal_cubic_meters REAL NOT NULL,
    discharge_cubic_meters REAL NOT NULL,
    recycled_cubic_meters REAL NOT NULL,
    supplier TEXT,
    meter_id TEXT,
    cost INTEGER NOT NULL,
    currency TEXT NOT NULL,
    water_stress_area INTEGER NOT NULL,
    quality_treatment_required TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS waste_records (
    id TEXT PRIMARY KEY,
    waste_number TEXT NOT NULL UNIQUE,
    facility_id TEXT,
    waste_date TEXT NOT NULL,
    reporting_period TEXT NOT NULL,
    waste_type TEXT NOT NULL,
    waste_category TEXT NOT NULL,
    quantity_kg REAL NOT NULL,
    disposal_method TEXT NOT NULL,
    contractor TEXT,
    manifest_number TEXT,
    cost INTEGER NOT NULL,
    currency TEXT NOT NULL,
    hazardous INTEGER NOT NULL,
    recycled_kg REAL,
    recovery_rate_percent REAL,
    destination_facility TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sustainability_targets (
    id TEXT PRIMARY KEY,
    target_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    target_type TEXT NOT NULL,
    baseline_year INTEGER NOT NULL,
    target_year INTEGER NOT NULL,
    baseline_value REAL NOT NULL,
    target_value REAL NOT NULL,
    current_value REAL,
    progress_percent REAL,
    unit TEXT NOT NULL,
    scope TEXT,
    alignment TEXT,
    science_based INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS esg_reports (
    id TEXT PRIMARY KEY,
    report_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    reporting_period TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    report_type TEXT NOT NULL,
    framework TEXT NOT NULL,
    scope_1_emissions REAL,
    scope_2_emissions_location REAL,
    scope_2_emissions_market REAL,
    scope_3_emissions REAL,
    total_energy_consumed REAL,
    renewable_energy_percent REAL,
    total_water_withdrawn REAL,
    total_waste_generated REAL,
    waste_diverted_percent REAL,
    employee_count INTEGER,
    employee_turnover_percent REAL,
    diversity_data TEXT,
    safety_incidents INTEGER,
    training_hours REAL,
    community_investment INTEGER,
    currency TEXT NOT NULL,
    assurance_status TEXT,
    assurance_provider TEXT,
    status TEXT NOT NULL,
    published_at TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Planning Tables
CREATE TABLE IF NOT EXISTS sop_cycles (
    id TEXT PRIMARY KEY,
    cycle_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    fiscal_year INTEGER NOT NULL,
    planning_horizon_months INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    current_status TEXT NOT NULL,
    demand_review_date TEXT,
    supply_review_date TEXT,
    pre_sop_date TEXT,
    executive_sop_date TEXT,
    total_demand INTEGER NOT NULL,
    total_supply INTEGER NOT NULL,
    gap INTEGER NOT NULL,
    currency TEXT NOT NULL,
    owner_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS demand_plans (
    id TEXT PRIMARY KEY,
    plan_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    sop_cycle_id TEXT NOT NULL,
    plan_type TEXT NOT NULL,
    planning_horizon_months INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL,
    created_by TEXT,
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (sop_cycle_id) REFERENCES sop_cycles(id)
);

CREATE TABLE IF NOT EXISTS supply_plans (
    id TEXT PRIMARY KEY,
    plan_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    sop_cycle_id TEXT NOT NULL,
    planning_horizon_months INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    currency TEXT NOT NULL,
    total_production_capacity INTEGER NOT NULL,
    total_external_supply INTEGER NOT NULL,
    total_available INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_by TEXT,
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (sop_cycle_id) REFERENCES sop_cycles(id)
);

CREATE TABLE IF NOT EXISTS drp_plans (
    id TEXT PRIMARY KEY,
    plan_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    planning_horizon_days INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS distribution_networks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    source_warehouse_id TEXT NOT NULL,
    destination_warehouse_id TEXT NOT NULL,
    lead_time_days INTEGER NOT NULL,
    transportation_mode TEXT NOT NULL,
    shipping_cost_per_unit INTEGER NOT NULL,
    min_order_quantity INTEGER NOT NULL,
    max_order_quantity INTEGER,
    lot_size_multiple INTEGER,
    priority INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS planning_parameters (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    parameter_type TEXT NOT NULL,
    value REAL NOT NULL,
    unit TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    source TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS promotion_events (
    id TEXT PRIMARY KEY,
    event_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    event_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Expenses Tables
CREATE TABLE IF NOT EXISTS corporate_cards (
    id TEXT PRIMARY KEY,
    card_number_masked TEXT NOT NULL,
    cardholder_id TEXT NOT NULL,
    card_type TEXT NOT NULL,
    issuer TEXT NOT NULL,
    credit_limit INTEGER NOT NULL,
    current_balance INTEGER NOT NULL,
    available_credit INTEGER NOT NULL,
    currency TEXT NOT NULL,
    issue_date TEXT NOT NULL,
    expiry_date TEXT NOT NULL,
    billing_day INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS card_transactions (
    id TEXT PRIMARY KEY,
    card_id TEXT NOT NULL,
    transaction_date TEXT NOT NULL,
    posting_date TEXT,
    merchant_name TEXT NOT NULL,
    merchant_category TEXT,
    merchant_category_code TEXT,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    original_amount INTEGER,
    original_currency TEXT,
    expense_type TEXT,
    expense_report_id TEXT,
    receipt_attached INTEGER NOT NULL,
    receipt_required INTEGER NOT NULL,
    verified INTEGER NOT NULL,
    verified_by TEXT,
    verified_at TEXT,
    status TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (card_id) REFERENCES corporate_cards(id)
);

CREATE TABLE IF NOT EXISTS expense_reports (
    id TEXT PRIMARY KEY,
    report_number TEXT NOT NULL UNIQUE,
    employee_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    purpose TEXT,
    project_id TEXT,
    cost_center_id TEXT,
    department_id TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    reimbursable_amount INTEGER NOT NULL,
    approved_amount INTEGER,
    submitted_at TEXT,
    approved_by TEXT,
    approved_at TEXT,
    rejected_at TEXT,
    rejection_reason TEXT,
    paid_at TEXT,
    payment_id TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS per_diem_rates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    country_code TEXT NOT NULL,
    city TEXT,
    rate_type TEXT NOT NULL,
    lodging_rate INTEGER NOT NULL,
    meals_rate INTEGER NOT NULL,
    incidentals_rate INTEGER NOT NULL,
    total_rate INTEGER NOT NULL,
    currency TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mileage_rates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    vehicle_type TEXT NOT NULL,
    rate_per_mile INTEGER NOT NULL,
    rate_per_km INTEGER NOT NULL,
    currency TEXT NOT NULL,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS travel_requests (
    id TEXT PRIMARY KEY,
    request_number TEXT NOT NULL UNIQUE,
    employee_id TEXT NOT NULL,
    purpose TEXT NOT NULL,
    destination_city TEXT NOT NULL,
    destination_country TEXT NOT NULL,
    departure_date TEXT NOT NULL,
    return_date TEXT NOT NULL,
    estimated_airfare INTEGER NOT NULL,
    estimated_lodging INTEGER NOT NULL,
    estimated_meals INTEGER NOT NULL,
    estimated_transportation INTEGER NOT NULL,
    estimated_other INTEGER NOT NULL,
    total_estimated INTEGER NOT NULL,
    currency TEXT NOT NULL,
    advance_required INTEGER NOT NULL,
    advance_amount INTEGER,
    expense_report_id TEXT,
    approved_by TEXT,
    approved_at TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS expense_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    expense_type TEXT NOT NULL,
    daily_limit INTEGER,
    per_transaction_limit INTEGER,
    monthly_limit INTEGER,
    annual_limit INTEGER,
    requires_receipt_above INTEGER,
    requires_preapproval_above INTEGER,
    currency TEXT NOT NULL,
    approval_workflow_id TEXT,
    effective_date TEXT NOT NULL,
    end_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Learning Tables
CREATE TABLE IF NOT EXISTS learning_courses (
    id TEXT PRIMARY KEY,
    course_code TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    category_id TEXT,
    instructor_id TEXT,
    difficulty_level TEXT NOT NULL,
    estimated_duration_minutes INTEGER NOT NULL,
    language TEXT NOT NULL,
    keywords TEXT,
    prerequisites TEXT,
    learning_objectives TEXT,
    passing_score_percent INTEGER NOT NULL,
    max_attempts INTEGER NOT NULL,
    certificate_template_id TEXT,
    certificate_validity_days INTEGER,
    is_mandatory INTEGER NOT NULL,
    is_featured INTEGER NOT NULL,
    enrollment_type TEXT NOT NULL,
    price INTEGER NOT NULL,
    currency TEXT NOT NULL,
    thumbnail_url TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS learning_paths (
    id TEXT PRIMARY KEY,
    path_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category_id TEXT,
    total_courses INTEGER NOT NULL,
    estimated_duration_hours INTEGER NOT NULL,
    is_mandatory INTEGER NOT NULL,
    target_roles TEXT,
    target_departments TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS course_enrollments (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL,
    course_id TEXT NOT NULL,
    learning_path_id TEXT,
    enrolled_by TEXT,
    enrolled_at TEXT NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    due_date TEXT,
    progress_percent INTEGER NOT NULL,
    time_spent_minutes INTEGER NOT NULL,
    score REAL,
    passed INTEGER,
    attempts INTEGER NOT NULL,
    status TEXT NOT NULL,
    certificate_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS assessments (
    id TEXT PRIMARY KEY,
    course_id TEXT,
    assessment_type TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    instructions TEXT,
    time_limit_minutes INTEGER,
    passing_score_percent INTEGER NOT NULL,
    max_attempts INTEGER NOT NULL,
    shuffle_questions INTEGER NOT NULL,
    shuffle_answers INTEGER NOT NULL,
    show_correct_answers INTEGER NOT NULL,
    show_score_immediately INTEGER NOT NULL,
    randomize_questions INTEGER NOT NULL,
    questions_per_attempt INTEGER,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS question_banks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    category_id TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    question_bank_id TEXT,
    assessment_id TEXT,
    question_type TEXT NOT NULL,
    question_text TEXT NOT NULL,
    explanation TEXT,
    hint TEXT,
    points INTEGER NOT NULL,
    difficulty TEXT NOT NULL,
    media_url TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS certificates (
    id TEXT PRIMARY KEY,
    certificate_number TEXT NOT NULL UNIQUE,
    employee_id TEXT NOT NULL,
    course_id TEXT,
    learning_path_id TEXT,
    certificate_template_id TEXT NOT NULL,
    issue_date TEXT NOT NULL,
    expiry_date TEXT,
    verification_code TEXT NOT NULL,
    pdf_path TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS competencies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    category_id TEXT,
    proficiency_levels TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- GRC Tables
CREATE TABLE IF NOT EXISTS risks (
    id TEXT PRIMARY KEY,
    risk_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    subcategory TEXT,
    owner_id TEXT,
    department_id TEXT,
    process_id TEXT,
    identified_date TEXT NOT NULL,
    likelihood INTEGER NOT NULL,
    impact INTEGER NOT NULL,
    inherent_risk_score INTEGER NOT NULL,
    inherent_risk_level TEXT NOT NULL,
    control_effectiveness INTEGER,
    residual_likelihood INTEGER,
    residual_impact INTEGER,
    residual_risk_score INTEGER,
    residual_risk_level TEXT,
    target_risk_score INTEGER,
    risk_response TEXT,
    status TEXT NOT NULL,
    review_frequency_days INTEGER NOT NULL,
    last_review_date TEXT,
    next_review_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS risk_assessments (
    id TEXT PRIMARY KEY,
    risk_id TEXT NOT NULL,
    assessment_date TEXT NOT NULL,
    assessor_id TEXT,
    likelihood_before INTEGER NOT NULL,
    impact_before INTEGER NOT NULL,
    score_before INTEGER NOT NULL,
    likelihood_after INTEGER,
    impact_after INTEGER,
    score_after INTEGER,
    assessment_method TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (risk_id) REFERENCES risks(id)
);

CREATE TABLE IF NOT EXISTS controls (
    id TEXT PRIMARY KEY,
    control_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    control_type TEXT NOT NULL,
    control_nature TEXT NOT NULL,
    control_frequency TEXT NOT NULL,
    control_owner_id TEXT,
    department_id TEXT,
    process_id TEXT,
    framework_reference TEXT,
    key_control INTEGER NOT NULL,
    automated INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS risk_control_mappings (
    id TEXT PRIMARY KEY,
    risk_id TEXT NOT NULL,
    control_id TEXT NOT NULL,
    mapping_type TEXT NOT NULL,
    effectiveness INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (risk_id) REFERENCES risks(id),
    FOREIGN KEY (control_id) REFERENCES controls(id)
);

CREATE TABLE IF NOT EXISTS control_tests (
    id TEXT PRIMARY KEY,
    test_number TEXT NOT NULL UNIQUE,
    control_id TEXT NOT NULL,
    test_date TEXT NOT NULL,
    tester_id TEXT,
    test_type TEXT NOT NULL,
    sample_size INTEGER,
    population_size INTEGER,
    exceptions_found INTEGER NOT NULL,
    test_result TEXT NOT NULL,
    effectiveness_rating INTEGER,
    findings TEXT,
    remediation_required INTEGER NOT NULL,
    remediation_due_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (control_id) REFERENCES controls(id)
);

CREATE TABLE IF NOT EXISTS policies (
    id TEXT PRIMARY KEY,
    policy_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    owner_id TEXT,
    approver_id TEXT,
    effective_date TEXT NOT NULL,
    review_frequency_months INTEGER NOT NULL,
    next_review_date TEXT,
    version TEXT NOT NULL,
    document_path TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS policy_acknowledgments (
    id TEXT PRIMARY KEY,
    policy_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    acknowledged_at TEXT NOT NULL,
    version TEXT NOT NULL,
    notes TEXT,
    FOREIGN KEY (policy_id) REFERENCES policies(id)
);

CREATE TABLE IF NOT EXISTS compliance_frameworks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    framework_type TEXT NOT NULL,
    regulatory_body TEXT,
    jurisdiction TEXT,
    effective_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS compliance_requirements (
    id TEXT PRIMARY KEY,
    framework_id TEXT NOT NULL,
    requirement_code TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    parent_id TEXT,
    control_owner_id TEXT,
    evidence_required INTEGER NOT NULL,
    testing_required INTEGER NOT NULL,
    frequency TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (framework_id) REFERENCES compliance_frameworks(id)
);

CREATE TABLE IF NOT EXISTS incidents (
    id TEXT PRIMARY KEY,
    incident_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    incident_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    reported_by TEXT,
    reported_date TEXT NOT NULL,
    occurred_date TEXT,
    discovered_date TEXT,
    location TEXT,
    department_id TEXT,
    affected_systems TEXT,
    affected_data TEXT,
    affected_parties TEXT,
    root_cause TEXT,
    immediate_actions TEXT,
    assigned_to TEXT,
    status TEXT NOT NULL,
    resolved_date TEXT,
    closure_notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS third_party_risks (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    risk_tier TEXT NOT NULL,
    assessment_date TEXT,
    next_assessment_date TEXT,
    inherent_risk_score INTEGER,
    residual_risk_score INTEGER,
    data_access_level TEXT NOT NULL,
    business_impact TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS kri_definitions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    measurement_unit TEXT NOT NULL,
    calculation_method TEXT,
    data_source TEXT,
    frequency TEXT NOT NULL,
    threshold_green REAL NOT NULL,
    threshold_yellow REAL NOT NULL,
    threshold_red REAL NOT NULL,
    direction TEXT NOT NULL,
    owner_id TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_capas_status ON capas(status);
CREATE INDEX IF NOT EXISTS idx_capas_source ON capas(source);
CREATE INDEX IF NOT EXISTS idx_carbon_emissions_date ON carbon_emissions(emission_date);
CREATE INDEX IF NOT EXISTS idx_carbon_emissions_account ON carbon_emissions(account_id);
CREATE INDEX IF NOT EXISTS idx_investments_status ON investments(status);
CREATE INDEX IF NOT EXISTS idx_risks_status ON risks(status);
CREATE INDEX IF NOT EXISTS idx_risks_category ON risks(category);
CREATE INDEX IF NOT EXISTS idx_controls_owner ON controls(control_owner_id);
CREATE INDEX IF NOT EXISTS idx_policies_status ON policies(status);
CREATE INDEX IF NOT EXISTS idx_expense_reports_employee ON expense_reports(employee_id);
CREATE INDEX IF NOT EXISTS idx_expense_reports_status ON expense_reports(status);
CREATE INDEX IF NOT EXISTS idx_course_enrollments_employee ON course_enrollments(employee_id);
CREATE INDEX IF NOT EXISTS idx_course_enrollments_status ON course_enrollments(status);
CREATE INDEX IF NOT EXISTS idx_sop_cycles_status ON sop_cycles(current_status);
CREATE INDEX IF NOT EXISTS idx_incidents_status ON incidents(status);
CREATE INDEX IF NOT EXISTS idx_incidents_severity ON incidents(severity);
