CREATE TABLE IF NOT EXISTS fsm_service_orders (
    id TEXT PRIMARY KEY,
    order_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    contact_name TEXT NOT NULL,
    contact_phone TEXT NOT NULL,
    contact_email TEXT,
    service_address TEXT NOT NULL,
    service_city TEXT NOT NULL,
    service_state TEXT,
    service_postal_code TEXT NOT NULL,
    service_country TEXT NOT NULL,
    service_lat REAL,
    service_lng REAL,
    work_type TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'Medium',
    status TEXT NOT NULL DEFAULT 'Scheduled',
    description TEXT NOT NULL,
    asset_id TEXT,
    asset_serial TEXT,
    contract_id TEXT,
    sla_id TEXT,
    assigned_technician_id TEXT,
    scheduled_date TEXT,
    scheduled_start TEXT,
    scheduled_end TEXT,
    actual_start TEXT,
    actual_end TEXT,
    travel_time_minutes INTEGER,
    work_duration_minutes INTEGER,
    resolution_notes TEXT,
    customer_signature TEXT,
    customer_rating INTEGER,
    customer_feedback TEXT,
    total_charges INTEGER DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_order_tasks (
    id TEXT PRIMARY KEY,
    service_order_id TEXT NOT NULL,
    task_number INTEGER NOT NULL,
    task_type TEXT NOT NULL,
    description TEXT NOT NULL,
    estimated_duration_minutes INTEGER NOT NULL,
    actual_duration_minutes INTEGER,
    status TEXT NOT NULL DEFAULT 'Pending',
    completed_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_technicians (
    id TEXT PRIMARY KEY,
    employee_id TEXT,
    technician_code TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    phone TEXT NOT NULL,
    email TEXT,
    status TEXT NOT NULL DEFAULT 'Available',
    skills TEXT NOT NULL,
    certifications TEXT NOT NULL,
    home_location_lat REAL,
    home_location_lng REAL,
    current_location_lat REAL,
    current_location_lng REAL,
    current_order_id TEXT,
    service_region TEXT,
    hourly_rate INTEGER NOT NULL,
    overtime_rate INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    work_start_time TEXT,
    work_end_time TEXT,
    working_days TEXT NOT NULL DEFAULT 'Mon,Tue,Wed,Thu,Fri',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_technician_availability (
    id TEXT PRIMARY KEY,
    technician_id TEXT NOT NULL,
    date TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Available',
    reason TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_territories (
    id TEXT PRIMARY KEY,
    territory_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    parent_territory_id TEXT,
    boundary_type TEXT NOT NULL,
    boundary_data TEXT NOT NULL,
    manager_id TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_technician_territories (
    id TEXT PRIMARY KEY,
    technician_id TEXT NOT NULL,
    territory_id TEXT NOT NULL,
    is_primary INTEGER DEFAULT 0,
    effective_date TEXT NOT NULL,
    expiry_date TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_appointments (
    id TEXT PRIMARY KEY,
    appointment_number TEXT NOT NULL UNIQUE,
    service_order_id TEXT NOT NULL,
    technician_id TEXT NOT NULL,
    scheduled_start TEXT NOT NULL,
    scheduled_end TEXT NOT NULL,
    actual_start TEXT,
    actual_end TEXT,
    status TEXT NOT NULL DEFAULT 'Scheduled',
    confirmation_status TEXT NOT NULL DEFAULT 'Pending',
    reminder_sent INTEGER DEFAULT 0,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_routes (
    id TEXT PRIMARY KEY,
    route_number TEXT NOT NULL UNIQUE,
    technician_id TEXT NOT NULL,
    route_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Planned',
    total_appointments INTEGER DEFAULT 0,
    completed_appointments INTEGER DEFAULT 0,
    total_distance REAL DEFAULT 0,
    total_duration_minutes INTEGER DEFAULT 0,
    optimization_score REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_route_stops (
    id TEXT PRIMARY KEY,
    route_id TEXT NOT NULL,
    appointment_id TEXT NOT NULL,
    stop_sequence INTEGER NOT NULL,
    planned_arrival TEXT NOT NULL,
    actual_arrival TEXT,
    planned_departure TEXT NOT NULL,
    actual_departure TEXT,
    travel_distance REAL NOT NULL,
    travel_time_minutes INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'Planned',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_parts (
    id TEXT PRIMARY KEY,
    service_order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    total_price INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    disposition TEXT NOT NULL DEFAULT 'Used',
    returned INTEGER DEFAULT 0,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_time_entries (
    id TEXT PRIMARY KEY,
    service_order_id TEXT NOT NULL,
    technician_id TEXT NOT NULL,
    entry_date TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    hours REAL NOT NULL,
    overtime_hours REAL DEFAULT 0,
    work_type TEXT NOT NULL,
    billable INTEGER DEFAULT 1,
    rate INTEGER NOT NULL,
    total_amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    notes TEXT,
    approved INTEGER DEFAULT 0,
    approved_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_expenses (
    id TEXT PRIMARY KEY,
    service_order_id TEXT NOT NULL,
    technician_id TEXT NOT NULL,
    expense_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    description TEXT,
    receipt_url TEXT,
    approved INTEGER DEFAULT 0,
    approved_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_checklists (
    id TEXT PRIMARY KEY,
    service_order_id TEXT NOT NULL,
    checklist_type TEXT NOT NULL,
    name TEXT NOT NULL,
    completed INTEGER DEFAULT 0,
    completed_at TEXT,
    completed_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_checklist_items (
    id TEXT PRIMARY KEY,
    checklist_id TEXT NOT NULL,
    item_number INTEGER NOT NULL,
    description TEXT NOT NULL,
    is_required INTEGER DEFAULT 1,
    response_type TEXT NOT NULL DEFAULT 'Boolean',
    response_value TEXT,
    notes TEXT,
    completed INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_service_contracts (
    id TEXT PRIMARY KEY,
    contract_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    contract_name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    contract_type TEXT NOT NULL,
    coverage_type TEXT NOT NULL,
    response_time_hours INTEGER NOT NULL,
    resolution_time_hours INTEGER NOT NULL,
    visit_limit INTEGER,
    visits_used INTEGER DEFAULT 0,
    coverage_hours TEXT NOT NULL,
    coverage_days TEXT NOT NULL,
    annual_fee INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_dispatch_rules (
    id TEXT PRIMARY KEY,
    rule_name TEXT NOT NULL,
    description TEXT,
    priority INTEGER NOT NULL,
    conditions TEXT NOT NULL,
    actions TEXT NOT NULL,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_technician_skills (
    id TEXT PRIMARY KEY,
    skill_code TEXT NOT NULL UNIQUE,
    skill_name TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    proficiency_levels TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fsm_technician_skill_assignments (
    id TEXT PRIMARY KEY,
    technician_id TEXT NOT NULL,
    skill_id TEXT NOT NULL,
    proficiency_level INTEGER NOT NULL,
    certified INTEGER DEFAULT 0,
    certified_date TEXT,
    expiry_date TEXT,
    created_at TEXT NOT NULL
);
