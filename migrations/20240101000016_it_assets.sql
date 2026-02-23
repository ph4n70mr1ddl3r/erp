CREATE TABLE it_assets (
    id TEXT PRIMARY KEY,
    asset_tag TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    asset_type TEXT NOT NULL DEFAULT '"Hardware"',
    status TEXT NOT NULL DEFAULT '"Available"',
    model TEXT,
    manufacturer TEXT,
    serial_number TEXT,
    purchase_date TEXT,
    purchase_cost INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    warranty_expiry TEXT,
    location_id TEXT,
    assigned_to TEXT,
    assigned_date TEXT,
    department_id TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE software_licenses (
    id TEXT PRIMARY KEY,
    license_key TEXT NOT NULL,
    product_name TEXT NOT NULL,
    vendor TEXT NOT NULL,
    license_type TEXT NOT NULL DEFAULT '"Perpetual"',
    seats_purchased INTEGER NOT NULL DEFAULT 1,
    seats_used INTEGER NOT NULL DEFAULT 0,
    purchase_date TEXT NOT NULL,
    purchase_cost INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    start_date TEXT NOT NULL,
    expiry_date TEXT,
    auto_renew INTEGER NOT NULL DEFAULT 0,
    support_expiry TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE software_installations (
    id TEXT PRIMARY KEY,
    license_id TEXT NOT NULL,
    asset_id TEXT NOT NULL,
    installed_by TEXT,
    installed_at TEXT NOT NULL,
    version TEXT,
    status TEXT NOT NULL DEFAULT '"Installed"',
    FOREIGN KEY (license_id) REFERENCES software_licenses(id),
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_assignments (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    assigned_to TEXT NOT NULL,
    assigned_by TEXT NOT NULL,
    assigned_at TEXT NOT NULL,
    expected_return TEXT,
    returned_at TEXT,
    returned_by TEXT,
    notes TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_maintenance (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    maintenance_type TEXT NOT NULL DEFAULT '"Preventive"',
    description TEXT NOT NULL,
    scheduled_date TEXT NOT NULL,
    performed_date TEXT,
    performed_by TEXT,
    cost INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT '"Scheduled"',
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_depreciation (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL UNIQUE,
    depreciation_method TEXT NOT NULL DEFAULT '"StraightLine"',
    useful_life_months INTEGER NOT NULL,
    salvage_value INTEGER NOT NULL DEFAULT 0,
    current_value INTEGER NOT NULL DEFAULT 0,
    accumulated_depreciation INTEGER NOT NULL DEFAULT 0,
    last_depreciation_date TEXT,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_disposals (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    disposal_type TEXT NOT NULL DEFAULT '"Scrapped"',
    disposal_date TEXT NOT NULL,
    reason TEXT NOT NULL,
    proceeds INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    approved_by TEXT,
    approved_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_locations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    building TEXT,
    floor TEXT,
    room TEXT,
    address TEXT,
    parent_id TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL
);

CREATE TABLE asset_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT,
    default_depreciation_method TEXT,
    default_useful_life_months INTEGER,
    status TEXT NOT NULL DEFAULT '"Active"'
);

CREATE TABLE vendor_contracts (
    id TEXT PRIMARY KEY,
    vendor_name TEXT NOT NULL,
    contract_number TEXT NOT NULL UNIQUE,
    contract_type TEXT NOT NULL DEFAULT '"Support"',
    start_date TEXT NOT NULL,
    end_date TEXT,
    value INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    contact_name TEXT,
    contact_email TEXT,
    contact_phone TEXT,
    terms TEXT,
    auto_renew INTEGER NOT NULL DEFAULT 0,
    renewal_notice_days INTEGER NOT NULL DEFAULT 30,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE network_assets (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL UNIQUE,
    ip_address TEXT,
    mac_address TEXT,
    hostname TEXT,
    domain TEXT,
    network_segment TEXT,
    vlan INTEGER,
    port TEXT,
    switch_port TEXT,
    dns_servers TEXT,
    gateway TEXT,
    subnet_mask TEXT,
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE security_assets (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL UNIQUE,
    security_level TEXT NOT NULL DEFAULT '"Internal"',
    data_classification TEXT NOT NULL DEFAULT '"Internal"',
    encryption_status INTEGER NOT NULL DEFAULT 0,
    antivirus_installed INTEGER NOT NULL DEFAULT 0,
    antivirus_updated TEXT,
    last_security_scan TEXT,
    vulnerabilities_found INTEGER NOT NULL DEFAULT 0,
    vulnerabilities_fixed INTEGER NOT NULL DEFAULT 0,
    compliance_status TEXT,
    notes TEXT,
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_audits (
    id TEXT PRIMARY KEY,
    audit_date TEXT NOT NULL,
    auditor TEXT,
    location_id TEXT,
    total_assets INTEGER NOT NULL DEFAULT 0,
    verified_assets INTEGER NOT NULL DEFAULT 0,
    missing_assets INTEGER NOT NULL DEFAULT 0,
    extra_assets INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT '"InProgress"',
    notes TEXT,
    created_at TEXT NOT NULL,
    completed_at TEXT
);

CREATE TABLE asset_audit_items (
    id TEXT PRIMARY KEY,
    audit_id TEXT NOT NULL,
    asset_id TEXT NOT NULL,
    expected_location_id TEXT,
    actual_location_id TEXT,
    expected_assignee_id TEXT,
    actual_assignee_id TEXT,
    status TEXT NOT NULL DEFAULT '"Pending"',
    notes TEXT,
    verified_at TEXT,
    verified_by TEXT,
    FOREIGN KEY (audit_id) REFERENCES asset_audits(id),
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_relationships (
    id TEXT PRIMARY KEY,
    parent_asset_id TEXT NOT NULL,
    child_asset_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL DEFAULT '"Contains"',
    created_at TEXT NOT NULL,
    FOREIGN KEY (parent_asset_id) REFERENCES it_assets(id),
    FOREIGN KEY (child_asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_checkouts (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    checked_out_to TEXT NOT NULL,
    checked_out_by TEXT NOT NULL,
    checked_out_at TEXT NOT NULL,
    expected_return TEXT NOT NULL,
    actual_return TEXT,
    returned_to TEXT,
    condition_on_checkout TEXT NOT NULL DEFAULT '"Good"',
    condition_on_return TEXT,
    notes TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE software_meters (
    id TEXT PRIMARY KEY,
    license_id TEXT NOT NULL,
    meter_date TEXT NOT NULL,
    peak_usage INTEGER NOT NULL DEFAULT 0,
    avg_usage REAL NOT NULL DEFAULT 0,
    total_hours REAL NOT NULL DEFAULT 0,
    FOREIGN KEY (license_id) REFERENCES software_licenses(id)
);

CREATE TABLE asset_documents (
    id TEXT PRIMARY KEY,
    asset_id TEXT NOT NULL,
    document_type TEXT NOT NULL DEFAULT '"Other"',
    title TEXT NOT NULL,
    description TEXT,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL DEFAULT 0,
    uploaded_by TEXT NOT NULL,
    uploaded_at TEXT NOT NULL,
    FOREIGN KEY (asset_id) REFERENCES it_assets(id)
);

CREATE TABLE asset_metrics (
    id TEXT PRIMARY KEY,
    metric_date TEXT NOT NULL,
    total_assets INTEGER NOT NULL DEFAULT 0,
    assets_in_use INTEGER NOT NULL DEFAULT 0,
    assets_available INTEGER NOT NULL DEFAULT 0,
    assets_in_maintenance INTEGER NOT NULL DEFAULT 0,
    assets_retired INTEGER NOT NULL DEFAULT 0,
    total_value INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    total_depreciation INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_it_assets_status ON it_assets(status);
CREATE INDEX idx_it_assets_assigned ON it_assets(assigned_to);
CREATE INDEX idx_it_assets_type ON it_assets(asset_type);
CREATE INDEX idx_software_licenses_expiry ON software_licenses(expiry_date);
CREATE INDEX idx_asset_assignments_active ON asset_assignments(status);
