-- MRP Tables
CREATE TABLE IF NOT EXISTS mrp_runs (
    id TEXT PRIMARY KEY,
    run_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    planning_horizon_days INTEGER NOT NULL,
    run_date TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    include_forecasts INTEGER NOT NULL DEFAULT 1,
    include_sales_orders INTEGER NOT NULL DEFAULT 1,
    include_work_orders INTEGER NOT NULL DEFAULT 1,
    safety_stock_method TEXT NOT NULL,
    status TEXT NOT NULL,
    total_items_planned INTEGER NOT NULL DEFAULT 0,
    total_suggestions INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    completed_at TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mrp_item_plans (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    starting_inventory INTEGER NOT NULL,
    safety_stock INTEGER NOT NULL,
    total_demand INTEGER NOT NULL,
    total_supply INTEGER NOT NULL,
    ending_inventory INTEGER NOT NULL,
    shortage_quantity INTEGER NOT NULL,
    suggested_actions INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mrp_demands (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    demand_type TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    source_line_id TEXT,
    required_date TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    allocated_quantity INTEGER NOT NULL DEFAULT 0,
    remaining_quantity INTEGER NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mrp_supplies (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    supply_type TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    available_date TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    allocated_quantity INTEGER NOT NULL DEFAULT 0,
    remaining_quantity INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mrp_suggestions (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    required_date TEXT NOT NULL,
    suggested_date TEXT NOT NULL,
    lead_time_days INTEGER NOT NULL,
    priority INTEGER NOT NULL,
    reason TEXT NOT NULL,
    source_demand_ids TEXT NOT NULL,
    status TEXT NOT NULL,
    converted_type TEXT,
    converted_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mrp_parameters (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    planning_method TEXT NOT NULL,
    lot_size_method TEXT NOT NULL,
    fixed_lot_size INTEGER NOT NULL DEFAULT 0,
    min_lot_size INTEGER NOT NULL DEFAULT 1,
    max_lot_size INTEGER NOT NULL DEFAULT 0,
    multiple_lot_size INTEGER NOT NULL DEFAULT 1,
    safety_stock INTEGER NOT NULL DEFAULT 0,
    safety_time_days INTEGER NOT NULL DEFAULT 0,
    lead_time_days INTEGER NOT NULL DEFAULT 0,
    planning_time_fence_days INTEGER NOT NULL DEFAULT 7,
    order_policy TEXT NOT NULL,
    min_order_days INTEGER NOT NULL DEFAULT 1,
    max_order_days INTEGER NOT NULL DEFAULT 365,
    days_of_supply INTEGER NOT NULL DEFAULT 14,
    service_level_percent INTEGER NOT NULL DEFAULT 95,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(product_id, warehouse_id)
);

CREATE TABLE IF NOT EXISTS demand_forecasts (
    id TEXT PRIMARY KEY,
    forecast_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    forecast_method TEXT NOT NULL,
    status TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS demand_forecast_lines (
    id TEXT PRIMARY KEY,
    forecast_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    forecast_quantity INTEGER NOT NULL,
    actual_quantity INTEGER,
    variance INTEGER,
    confidence_level INTEGER,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS planned_orders (
    id TEXT PRIMARY KEY,
    order_number TEXT NOT NULL UNIQUE,
    run_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    order_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    due_date TEXT NOT NULL,
    bom_id TEXT,
    routing_id TEXT,
    source_demand_ids TEXT NOT NULL,
    status TEXT NOT NULL,
    firmed INTEGER NOT NULL DEFAULT 0,
    converted_type TEXT,
    converted_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS planned_order_components (
    id TEXT PRIMARY KEY,
    planned_order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    required_quantity INTEGER NOT NULL,
    issued_quantity INTEGER NOT NULL DEFAULT 0,
    required_date TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mrp_exceptions (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    exception_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    message TEXT NOT NULL,
    details TEXT,
    suggested_action TEXT,
    acknowledged INTEGER NOT NULL DEFAULT 0,
    acknowledged_by TEXT,
    acknowledged_at TEXT,
    created_at TEXT NOT NULL
);

-- EAM Tables
CREATE TABLE IF NOT EXISTS equipment_assets (
    id TEXT PRIMARY KEY,
    asset_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    asset_type TEXT NOT NULL,
    category TEXT,
    manufacturer TEXT,
    model TEXT,
    serial_number TEXT,
    location_id TEXT,
    department_id TEXT,
    parent_asset_id TEXT,
    installation_date TEXT,
    warranty_end_date TEXT,
    criticality TEXT NOT NULL,
    status TEXT NOT NULL,
    acquisition_cost INTEGER NOT NULL,
    depreciation_method TEXT,
    useful_life_years INTEGER,
    current_book_value INTEGER NOT NULL,
    meter_type TEXT,
    meter_unit TEXT,
    current_meter_reading INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS asset_meter_readings (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    reading_date TEXT NOT NULL,
    reading_value INTEGER NOT NULL,
    reading_type TEXT NOT NULL,
    entered_by TEXT,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS work_orders (
    id TEXT PRIMARY KEY,
    wo_number TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    work_order_type TEXT NOT NULL,
    priority TEXT NOT NULL,
    asset_id TEXT,
    location_id TEXT,
    failure_code_id TEXT,
    problem_description TEXT,
    cause_description TEXT,
    remedy_description TEXT,
    requested_by TEXT,
    requested_date TEXT NOT NULL,
    required_date TEXT,
    scheduled_start TEXT,
    scheduled_end TEXT,
    actual_start TEXT,
    actual_end TEXT,
    assigned_to TEXT,
    assigned_team_id TEXT,
    status TEXT NOT NULL,
    estimated_labor_hours REAL NOT NULL DEFAULT 0,
    actual_labor_hours REAL NOT NULL DEFAULT 0,
    estimated_cost INTEGER NOT NULL DEFAULT 0,
    actual_cost INTEGER NOT NULL DEFAULT 0,
    downtime_hours REAL NOT NULL DEFAULT 0,
    completion_notes TEXT,
    closed_by TEXT,
    closed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS work_order_tasks (
    id TEXT PRIMARY KEY,
    work_order_id TEXT NOT NULL,
    task_number INTEGER NOT NULL,
    description TEXT NOT NULL,
    estimated_hours REAL NOT NULL DEFAULT 0,
    actual_hours REAL NOT NULL DEFAULT 0,
    assigned_to TEXT,
    completed INTEGER NOT NULL DEFAULT 0,
    completed_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS work_order_labor (
    id TEXT PRIMARY KEY,
    work_order_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    labor_type TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT,
    hours REAL NOT NULL DEFAULT 0,
    hourly_rate INTEGER NOT NULL DEFAULT 0,
    total_cost INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS work_order_parts (
    id TEXT PRIMARY KEY,
    work_order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    quantity_required INTEGER NOT NULL,
    quantity_issued INTEGER NOT NULL DEFAULT 0,
    unit_cost INTEGER NOT NULL DEFAULT 0,
    total_cost INTEGER NOT NULL DEFAULT 0,
    issued_at TEXT,
    issued_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pm_schedules (
    id TEXT PRIMARY KEY,
    pm_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    asset_id TEXT NOT NULL,
    maintenance_strategy TEXT NOT NULL,
    frequency_type TEXT NOT NULL,
    frequency_value INTEGER NOT NULL,
    last_performed_date TEXT,
    next_due_date TEXT NOT NULL,
    meter_based INTEGER NOT NULL DEFAULT 0,
    last_meter_reading INTEGER,
    next_meter_due INTEGER,
    estimated_duration_hours REAL NOT NULL DEFAULT 0,
    estimated_cost INTEGER NOT NULL DEFAULT 0,
    auto_generate_wo INTEGER NOT NULL DEFAULT 1,
    lead_time_days INTEGER NOT NULL DEFAULT 7,
    checklist_id TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pm_tasks (
    id TEXT PRIMARY KEY,
    pm_schedule_id TEXT NOT NULL,
    task_number INTEGER NOT NULL,
    description TEXT NOT NULL,
    estimated_minutes INTEGER NOT NULL DEFAULT 0,
    required_skills TEXT,
    safety_notes TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS failure_codes (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    problem_type TEXT NOT NULL,
    cause_type TEXT,
    remedy_type TEXT,
    parent_id TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS asset_failure_history (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    failure_date TEXT NOT NULL,
    failure_code_id TEXT,
    problem_description TEXT NOT NULL,
    cause_description TEXT,
    remedy_description TEXT,
    downtime_hours REAL NOT NULL DEFAULT 0,
    repair_cost INTEGER NOT NULL DEFAULT 0,
    work_order_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS maintenance_calendars (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    working_days TEXT NOT NULL,
    shift_start TEXT NOT NULL,
    shift_end TEXT NOT NULL,
    holidays TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS maintenance_shifts (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    shift_name TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    days_of_week TEXT NOT NULL,
    crew_size INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS spare_parts (
    id TEXT PRIMARY KEY,
    part_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    manufacturer TEXT,
    unit_of_measure TEXT NOT NULL,
    unit_cost INTEGER NOT NULL DEFAULT 0,
    min_stock_level INTEGER NOT NULL DEFAULT 0,
    max_stock_level INTEGER NOT NULL DEFAULT 0,
    reorder_point INTEGER NOT NULL DEFAULT 0,
    current_stock INTEGER NOT NULL DEFAULT 0,
    warehouse_id TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS asset_spare_parts (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    spare_part_id TEXT NOT NULL,
    quantity_required INTEGER NOT NULL,
    installation_date TEXT,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS maintenance_budgets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    fiscal_year INTEGER NOT NULL,
    department_id TEXT,
    total_budget INTEGER NOT NULL DEFAULT 0,
    labor_budget INTEGER NOT NULL DEFAULT 0,
    parts_budget INTEGER NOT NULL DEFAULT 0,
    contractor_budget INTEGER NOT NULL DEFAULT 0,
    spent_to_date INTEGER NOT NULL DEFAULT 0,
    committed_amount INTEGER NOT NULL DEFAULT 0,
    remaining_budget INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS maintenance_kpis (
    id TEXT PRIMARY KEY,
    kpi_type TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    asset_id TEXT,
    department_id TEXT,
    value REAL NOT NULL,
    target REAL NOT NULL,
    variance REAL NOT NULL,
    trend TEXT,
    calculated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS service_contracts (
    id TEXT PRIMARY KEY,
    contract_number TEXT NOT NULL UNIQUE,
    vendor_id TEXT NOT NULL,
    contract_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    annual_cost INTEGER NOT NULL DEFAULT 0,
    response_time_hours INTEGER NOT NULL DEFAULT 0,
    coverage_type TEXT NOT NULL,
    terms TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS asset_locations (
    id TEXT PRIMARY KEY,
    location_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT,
    site_id TEXT,
    building TEXT,
    floor TEXT,
    room TEXT,
    area TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS asset_down_events (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    down_start TEXT NOT NULL,
    down_end TEXT,
    downtime_hours REAL,
    reason TEXT NOT NULL,
    work_order_id TEXT,
    created_at TEXT NOT NULL
);

-- Inventory Valuation Tables
CREATE TABLE IF NOT EXISTS product_valuations (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    valuation_method TEXT NOT NULL,
    standard_cost INTEGER NOT NULL DEFAULT 0,
    current_unit_cost INTEGER NOT NULL DEFAULT 0,
    total_quantity INTEGER NOT NULL DEFAULT 0,
    total_value INTEGER NOT NULL DEFAULT 0,
    last_receipt_cost INTEGER NOT NULL DEFAULT 0,
    last_receipt_date TEXT,
    last_issue_cost INTEGER NOT NULL DEFAULT 0,
    last_issue_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(product_id, warehouse_id)
);

CREATE TABLE IF NOT EXISTS inventory_cost_layers (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    layer_date TEXT NOT NULL,
    receipt_reference TEXT NOT NULL,
    receipt_id TEXT,
    quantity INTEGER NOT NULL,
    unit_cost INTEGER NOT NULL,
    remaining_quantity INTEGER NOT NULL,
    total_value INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS inventory_valuation_summaries (
    id TEXT PRIMARY KEY,
    valuation_date TEXT NOT NULL,
    warehouse_id TEXT,
    category_id TEXT,
    total_products INTEGER NOT NULL DEFAULT 0,
    total_quantity INTEGER NOT NULL DEFAULT 0,
    total_value INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS inventory_valuation_lines (
    id TEXT PRIMARY KEY,
    summary_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    quantity_on_hand INTEGER NOT NULL,
    unit_cost INTEGER NOT NULL,
    total_value INTEGER NOT NULL,
    valuation_method TEXT NOT NULL,
    previous_value INTEGER NOT NULL DEFAULT 0,
    value_change INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS cost_adjustments (
    id TEXT PRIMARY KEY,
    adjustment_number TEXT NOT NULL UNIQUE,
    adjustment_type TEXT NOT NULL,
    adjustment_date TEXT NOT NULL,
    reason TEXT NOT NULL,
    status TEXT NOT NULL,
    approved_by TEXT,
    approved_at TEXT,
    journal_entry_id TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cost_adjustment_lines (
    id TEXT PRIMARY KEY,
    adjustment_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    old_unit_cost INTEGER NOT NULL,
    new_unit_cost INTEGER NOT NULL,
    old_total_value INTEGER NOT NULL,
    new_total_value INTEGER NOT NULL,
    value_change INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS abc_classifications (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    classification TEXT NOT NULL,
    annual_value INTEGER NOT NULL DEFAULT 0,
    annual_quantity INTEGER NOT NULL DEFAULT 0,
    cumulative_value_percent REAL NOT NULL DEFAULT 0,
    classification_date TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS inventory_turnovers (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    beginning_inventory INTEGER NOT NULL DEFAULT 0,
    ending_inventory INTEGER NOT NULL DEFAULT 0,
    average_inventory INTEGER NOT NULL DEFAULT 0,
    cost_of_goods_sold INTEGER NOT NULL DEFAULT 0,
    turnover_ratio REAL NOT NULL DEFAULT 0,
    days_of_inventory REAL NOT NULL DEFAULT 0,
    calculated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS consignment_stocks (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    agreement_id TEXT,
    quantity INTEGER NOT NULL,
    unit_cost INTEGER NOT NULL DEFAULT 0,
    total_value INTEGER NOT NULL DEFAULT 0,
    ownership_status TEXT NOT NULL,
    received_date TEXT NOT NULL,
    consumption_start_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS consignment_consumptions (
    id TEXT PRIMARY KEY,
    consignment_id TEXT NOT NULL,
    consumption_date TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_cost INTEGER NOT NULL DEFAULT 0,
    total_cost INTEGER NOT NULL DEFAULT 0,
    purchase_order_id TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Blanket Purchase Order Tables
CREATE TABLE IF NOT EXISTS blanket_purchase_orders (
    id TEXT PRIMARY KEY,
    bpo_number TEXT NOT NULL UNIQUE,
    vendor_id TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_amount_limit INTEGER NOT NULL DEFAULT 0,
    total_quantity_limit INTEGER,
    total_released INTEGER NOT NULL DEFAULT 0,
    total_invoiced INTEGER NOT NULL DEFAULT 0,
    payment_terms INTEGER NOT NULL DEFAULT 0,
    terms_conditions TEXT,
    status TEXT NOT NULL,
    created_by TEXT,
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS blanket_purchase_order_lines (
    id TEXT PRIMARY KEY,
    bpo_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    unit_price INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    min_release_qty INTEGER NOT NULL DEFAULT 1,
    max_quantity INTEGER NOT NULL DEFAULT 0,
    released_quantity INTEGER NOT NULL DEFAULT 0,
    remaining_quantity INTEGER NOT NULL DEFAULT 0,
    uom TEXT NOT NULL,
    lead_time_days INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS release_orders (
    id TEXT PRIMARY KEY,
    release_number TEXT NOT NULL UNIQUE,
    bpo_id TEXT NOT NULL,
    bpo_line_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL DEFAULT 0,
    total_amount INTEGER NOT NULL DEFAULT 0,
    required_date TEXT NOT NULL,
    ship_to_warehouse_id TEXT NOT NULL,
    status TEXT NOT NULL,
    purchase_order_id TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Cash Management Tables
CREATE TABLE IF NOT EXISTS cash_pools (
    id TEXT PRIMARY KEY,
    pool_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    header_account_id TEXT NOT NULL,
    pooling_type TEXT NOT NULL,
    pooling_frequency TEXT NOT NULL,
    target_balance INTEGER NOT NULL DEFAULT 0,
    min_balance INTEGER NOT NULL DEFAULT 0,
    max_balance INTEGER NOT NULL DEFAULT 0,
    interest_calculation_method TEXT NOT NULL,
    interest_rate REAL NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_pool_members (
    id TEXT PRIMARY KEY,
    pool_id TEXT NOT NULL,
    bank_account_id TEXT NOT NULL,
    company_id TEXT NOT NULL,
    member_type TEXT NOT NULL,
    participation_percent REAL NOT NULL DEFAULT 100,
    target_balance INTEGER NOT NULL DEFAULT 0,
    min_balance INTEGER NOT NULL DEFAULT 0,
    max_balance INTEGER NOT NULL DEFAULT 0,
    interest_rate_override REAL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_sweeps (
    id TEXT PRIMARY KEY,
    sweep_number TEXT NOT NULL UNIQUE,
    pool_id TEXT NOT NULL,
    sweep_date TEXT NOT NULL,
    sweep_type TEXT NOT NULL,
    total_amount INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    processed_at TEXT,
    journal_entry_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_sweep_lines (
    id TEXT PRIMARY KEY,
    sweep_id TEXT NOT NULL,
    member_id TEXT NOT NULL,
    bank_account_id TEXT NOT NULL,
    opening_balance INTEGER NOT NULL DEFAULT 0,
    target_balance INTEGER NOT NULL DEFAULT 0,
    sweep_amount INTEGER NOT NULL DEFAULT 0,
    closing_balance INTEGER NOT NULL DEFAULT 0,
    direction TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_positions (
    id TEXT PRIMARY KEY,
    position_date TEXT NOT NULL,
    company_id TEXT,
    bank_account_id TEXT,
    opening_balance INTEGER NOT NULL DEFAULT 0,
    receipts INTEGER NOT NULL DEFAULT 0,
    disbursements INTEGER NOT NULL DEFAULT 0,
    transfers_in INTEGER NOT NULL DEFAULT 0,
    transfers_out INTEGER NOT NULL DEFAULT 0,
    closing_balance INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    exchange_rate REAL NOT NULL DEFAULT 1,
    base_currency_balance INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cash_position_details (
    id TEXT PRIMARY KEY,
    position_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    reference_type TEXT,
    reference_id TEXT,
    description TEXT,
    amount INTEGER NOT NULL,
    value_date TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS intercompany_loans (
    id TEXT PRIMARY KEY,
    loan_number TEXT NOT NULL UNIQUE,
    from_company_id TEXT NOT NULL,
    to_company_id TEXT NOT NULL,
    from_account_id TEXT NOT NULL,
    to_account_id TEXT NOT NULL,
    principal_amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    interest_rate REAL NOT NULL DEFAULT 0,
    interest_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    maturity_date TEXT,
    repayment_schedule TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS intercompany_loan_payments (
    id TEXT PRIMARY KEY,
    loan_id TEXT NOT NULL,
    payment_date TEXT NOT NULL,
    principal_amount INTEGER NOT NULL,
    interest_amount INTEGER NOT NULL DEFAULT 0,
    total_amount INTEGER NOT NULL,
    from_journal_entry_id TEXT,
    to_journal_entry_id TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Supplier Qualification Tables
CREATE TABLE IF NOT EXISTS supplier_qualifications (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    qualification_type TEXT NOT NULL,
    status TEXT NOT NULL,
    submitted_at TEXT,
    reviewed_by TEXT,
    reviewed_at TEXT,
    approved_by TEXT,
    approved_at TEXT,
    valid_from TEXT,
    valid_until TEXT,
    score INTEGER,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS supplier_qualification_documents (
    id TEXT PRIMARY KEY,
    qualification_id TEXT NOT NULL,
    document_type TEXT NOT NULL,
    document_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    expiry_date TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS supplier_evaluations (
    id TEXT PRIMARY KEY,
    evaluation_number TEXT NOT NULL UNIQUE,
    vendor_id TEXT NOT NULL,
    evaluation_period_start TEXT NOT NULL,
    evaluation_period_end TEXT NOT NULL,
    quality_score INTEGER NOT NULL DEFAULT 0,
    delivery_score INTEGER NOT NULL DEFAULT 0,
    price_score INTEGER NOT NULL DEFAULT 0,
    service_score INTEGER NOT NULL DEFAULT 0,
    overall_score REAL NOT NULL DEFAULT 0,
    grade TEXT NOT NULL,
    evaluator_id TEXT,
    evaluated_at TEXT NOT NULL,
    comments TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS supplier_evaluation_criteria (
    id TEXT PRIMARY KEY,
    criteria_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    weight INTEGER NOT NULL DEFAULT 1,
    max_score INTEGER NOT NULL DEFAULT 100,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS supplier_evaluation_lines (
    id TEXT PRIMARY KEY,
    evaluation_id TEXT NOT NULL,
    criteria_id TEXT NOT NULL,
    score INTEGER NOT NULL DEFAULT 0,
    weighted_score REAL NOT NULL DEFAULT 0,
    comments TEXT
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_mrp_runs_date ON mrp_runs(run_date);
CREATE INDEX IF NOT EXISTS idx_mrp_suggestions_product ON mrp_suggestions(product_id);
CREATE INDEX IF NOT EXISTS idx_equipment_assets_type ON equipment_assets(asset_type);
CREATE INDEX IF NOT EXISTS idx_work_orders_status ON work_orders(status);
CREATE INDEX IF NOT EXISTS idx_work_orders_asset ON work_orders(asset_id);
CREATE INDEX IF NOT EXISTS idx_pm_schedules_next_due ON pm_schedules(next_due_date);
CREATE INDEX IF NOT EXISTS idx_inventory_cost_layers_product ON inventory_cost_layers(product_id, warehouse_id);
CREATE INDEX IF NOT EXISTS idx_cash_pools_status ON cash_pools(status);
CREATE INDEX IF NOT EXISTS idx_cash_positions_date ON cash_positions(position_date);
CREATE INDEX IF NOT EXISTS idx_blanket_pos_vendor ON blanket_purchase_orders(vendor_id);
CREATE INDEX IF NOT EXISTS idx_supplier_qual_vendor ON supplier_qualifications(vendor_id);
