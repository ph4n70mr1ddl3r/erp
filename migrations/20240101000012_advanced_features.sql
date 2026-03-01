-- Fixed Assets
CREATE TABLE IF NOT EXISTS fixed_assets (
    id TEXT PRIMARY KEY,
    asset_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    location TEXT,
    cost INTEGER NOT NULL,
    salvage_value INTEGER DEFAULT 0,
    useful_life_years INTEGER NOT NULL,
    depreciation_method TEXT NOT NULL DEFAULT 'StraightLine',
    acquisition_date TEXT NOT NULL,
    depreciation_start_date TEXT,
    accumulated_depreciation INTEGER DEFAULT 0,
    net_book_value INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS asset_depreciation (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    period TEXT NOT NULL,
    depreciation_amount INTEGER NOT NULL,
    accumulated_depreciation INTEGER NOT NULL,
    posted_at TEXT NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES fixed_assets(id)
);

CREATE INDEX IF NOT EXISTS idx_assets_code ON fixed_assets(asset_code);
CREATE INDEX IF NOT EXISTS idx_assets_status ON fixed_assets(status);
CREATE INDEX IF NOT EXISTS idx_depreciation_asset ON asset_depreciation(asset_id);

-- Quality Control
CREATE TABLE IF NOT EXISTS quality_inspections (
    id TEXT PRIMARY KEY,
    inspection_number TEXT NOT NULL UNIQUE,
    inspection_type TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    inspector_id TEXT,
    inspection_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    result TEXT,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS inspection_items (
    id TEXT PRIMARY KEY,
    inspection_id TEXT NOT NULL,
    criterion TEXT NOT NULL,
    expected_value TEXT,
    actual_value TEXT,
    pass_fail TEXT,
    notes TEXT,
    FOREIGN KEY (inspection_id) REFERENCES quality_inspections(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS non_conformance_reports (
    id TEXT PRIMARY KEY,
    ncr_number TEXT NOT NULL UNIQUE,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL,
    description TEXT NOT NULL,
    severity TEXT NOT NULL DEFAULT 'Minor',
    status TEXT NOT NULL DEFAULT 'Open',
    assigned_to TEXT,
    root_cause TEXT,
    corrective_action TEXT,
    preventive_action TEXT,
    resolution_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_inspections_entity ON quality_inspections(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_ncr_status ON non_conformance_reports(status);

-- Lead/Opportunity Management
CREATE TABLE IF NOT EXISTS leads (
    id TEXT PRIMARY KEY,
    lead_number TEXT NOT NULL UNIQUE,
    company_name TEXT NOT NULL,
    contact_name TEXT,
    email TEXT,
    phone TEXT,
    source TEXT,
    industry TEXT,
    estimated_value INTEGER DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'New',
    assigned_to TEXT,
    notes TEXT,
    converted_to_customer TEXT,
    converted_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS opportunities (
    id TEXT PRIMARY KEY,
    opportunity_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    customer_id TEXT,
    lead_id TEXT,
    stage TEXT NOT NULL DEFAULT 'Prospecting',
    probability INTEGER DEFAULT 0,
    expected_close_date TEXT,
    amount INTEGER DEFAULT 0,
    description TEXT,
    assigned_to TEXT,
    status TEXT NOT NULL DEFAULT 'Open',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS opportunity_activities (
    id TEXT PRIMARY KEY,
    opportunity_id TEXT NOT NULL,
    activity_type TEXT NOT NULL,
    subject TEXT NOT NULL,
    description TEXT,
    due_date TEXT,
    completed INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (opportunity_id) REFERENCES opportunities(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_leads_status ON leads(status);
CREATE INDEX IF NOT EXISTS idx_opportunities_stage ON opportunities(stage);

-- Demand Planning
CREATE TABLE IF NOT EXISTS demand_forecasts (
    id TEXT PRIMARY KEY,
    forecast_number TEXT,
    name TEXT,
    product_id TEXT NOT NULL,
    warehouse_id TEXT,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    forecast_quantity INTEGER NOT NULL,
    confidence_level INTEGER DEFAULT 80,
    method TEXT NOT NULL DEFAULT 'MovingAverage',
    forecast_method TEXT,
    status TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT
);

CREATE TABLE IF NOT EXISTS safety_stock (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    safety_stock INTEGER NOT NULL,
    reorder_point INTEGER NOT NULL,
    reorder_quantity INTEGER NOT NULL,
    lead_time_days INTEGER DEFAULT 0,
    service_level INTEGER DEFAULT 95,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS replenishment_orders (
    id TEXT PRIMARY KEY,
    order_number TEXT NOT NULL UNIQUE,
    product_id TEXT NOT NULL,
    warehouse_id TEXT NOT NULL,
    order_type TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    source TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_forecasts_product ON demand_forecasts(product_id);
CREATE INDEX IF NOT EXISTS idx_safety_stock_product ON safety_stock(product_id);

-- Production Scheduling
CREATE TABLE IF NOT EXISTS work_centers (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    capacity INTEGER DEFAULT 0,
    efficiency INTEGER DEFAULT 100,
    status TEXT DEFAULT 'Active'
);

CREATE TABLE IF NOT EXISTS production_schedules (
    id TEXT PRIMARY KEY,
    schedule_number TEXT NOT NULL UNIQUE,
    work_order_id TEXT NOT NULL,
    work_center_id TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Planned',
    actual_start TEXT,
    actual_end TEXT,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_schedules_work_center ON production_schedules(work_center_id);
CREATE INDEX IF NOT EXISTS idx_schedules_status ON production_schedules(status);

-- Supplier Scorecards
CREATE TABLE IF NOT EXISTS supplier_scorecards (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    period TEXT NOT NULL,
    on_time_delivery INTEGER DEFAULT 0,
    quality_score INTEGER DEFAULT 0,
    price_competitiveness INTEGER DEFAULT 0,
    responsiveness INTEGER DEFAULT 0,
    overall_score INTEGER DEFAULT 0,
    total_orders INTEGER DEFAULT 0,
    total_value INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS vendor_performance (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    delivery_date TEXT,
    expected_date TEXT,
    on_time INTEGER DEFAULT 0,
    quality_rating INTEGER DEFAULT 0,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_scorecards_vendor ON supplier_scorecards(vendor_id);
CREATE INDEX IF NOT EXISTS idx_performance_vendor ON vendor_performance(vendor_id);

-- Custom Fields
CREATE TABLE IF NOT EXISTS custom_field_definitions (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    field_name TEXT NOT NULL,
    field_label TEXT NOT NULL,
    field_type TEXT NOT NULL,
    required INTEGER DEFAULT 0,
    options TEXT,
    sort_order INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS custom_field_values (
    id TEXT PRIMARY KEY,
    definition_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    value TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (definition_id) REFERENCES custom_field_definitions(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_custom_field_unique ON custom_field_definitions(entity_type, field_name);
CREATE INDEX IF NOT EXISTS idx_custom_values_entity ON custom_field_values(entity_id);
