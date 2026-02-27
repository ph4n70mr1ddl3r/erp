CREATE TABLE warranty_policies (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    warranty_type TEXT NOT NULL DEFAULT 'Standard',
    duration_value INTEGER NOT NULL DEFAULT 12,
    duration_unit TEXT NOT NULL DEFAULT 'Months',
    coverage_percentage REAL NOT NULL DEFAULT 100.0,
    labor_covered INTEGER NOT NULL DEFAULT 1,
    parts_covered INTEGER NOT NULL DEFAULT 1,
    on_site_service INTEGER NOT NULL DEFAULT 0,
    max_claims INTEGER,
    deductible_amount INTEGER NOT NULL DEFAULT 0,
    terms_and_conditions TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE INDEX idx_warranty_policies_code ON warranty_policies(code);
CREATE INDEX idx_warranty_policies_status ON warranty_policies(status);

CREATE TABLE product_warranties (
    id TEXT PRIMARY KEY,
    warranty_number TEXT NOT NULL UNIQUE,
    policy_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    sales_order_id TEXT,
    sales_order_line_id TEXT,
    serial_number TEXT,
    lot_number TEXT,
    purchase_date TEXT NOT NULL,
    activation_date TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    transferred_to_customer_id TEXT,
    transferred_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT,
    FOREIGN KEY (policy_id) REFERENCES warranty_policies(id)
);

CREATE INDEX idx_product_warranties_number ON product_warranties(warranty_number);
CREATE INDEX idx_product_warranties_customer ON product_warranties(customer_id);
CREATE INDEX idx_product_warranties_product ON product_warranties(product_id);
CREATE INDEX idx_product_warranties_status ON product_warranties(status);
CREATE INDEX idx_product_warranties_end_date ON product_warranties(end_date);

CREATE TABLE warranty_claims (
    id TEXT PRIMARY KEY,
    claim_number TEXT NOT NULL UNIQUE,
    product_warranty_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    reported_date TEXT NOT NULL,
    issue_description TEXT NOT NULL,
    issue_category TEXT,
    symptom_codes TEXT,
    status TEXT NOT NULL DEFAULT 'Submitted',
    priority INTEGER NOT NULL DEFAULT 3,
    assigned_to TEXT,
    assigned_at TEXT,
    resolution_type TEXT,
    resolution_notes TEXT,
    resolved_at TEXT,
    resolved_by TEXT,
    customer_notified INTEGER NOT NULL DEFAULT 0,
    notification_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT,
    FOREIGN KEY (product_warranty_id) REFERENCES product_warranties(id)
);

CREATE INDEX idx_warranty_claims_number ON warranty_claims(claim_number);
CREATE INDEX idx_warranty_claims_warranty ON warranty_claims(product_warranty_id);
CREATE INDEX idx_warranty_claims_customer ON warranty_claims(customer_id);
CREATE INDEX idx_warranty_claims_status ON warranty_claims(status);
CREATE INDEX idx_warranty_claims_assigned ON warranty_claims(assigned_to);

CREATE TABLE warranty_claim_lines (
    id TEXT PRIMARY KEY,
    claim_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_cost INTEGER NOT NULL,
    total_cost INTEGER NOT NULL,
    coverage_percentage REAL NOT NULL DEFAULT 100.0,
    covered_amount INTEGER NOT NULL DEFAULT 0,
    customer_amount INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (claim_id) REFERENCES warranty_claims(id) ON DELETE CASCADE
);

CREATE INDEX idx_warranty_claim_lines_claim ON warranty_claim_lines(claim_id);

CREATE TABLE warranty_claim_labor (
    id TEXT PRIMARY KEY,
    claim_id TEXT NOT NULL,
    technician_id TEXT,
    work_description TEXT NOT NULL,
    labor_hours REAL NOT NULL,
    hourly_rate INTEGER NOT NULL,
    total_cost INTEGER NOT NULL,
    covered_amount INTEGER NOT NULL DEFAULT 0,
    customer_amount INTEGER NOT NULL DEFAULT 0,
    work_date TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (claim_id) REFERENCES warranty_claims(id) ON DELETE CASCADE
);

CREATE INDEX idx_warranty_claim_labor_claim ON warranty_claim_labor(claim_id);

CREATE TABLE warranty_registrations (
    id TEXT PRIMARY KEY,
    product_warranty_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    registration_date TEXT NOT NULL,
    registration_source TEXT NOT NULL,
    verified INTEGER NOT NULL DEFAULT 0,
    verified_at TEXT,
    verified_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT,
    FOREIGN KEY (product_warranty_id) REFERENCES product_warranties(id)
);

CREATE INDEX idx_warranty_registrations_warranty ON warranty_registrations(product_warranty_id);
CREATE INDEX idx_warranty_registrations_customer ON warranty_registrations(customer_id);

CREATE TABLE warranty_extensions (
    id TEXT PRIMARY KEY,
    product_warranty_id TEXT NOT NULL,
    policy_id TEXT NOT NULL,
    extension_date TEXT NOT NULL,
    additional_duration_value INTEGER NOT NULL,
    additional_duration_unit TEXT NOT NULL DEFAULT 'Months',
    new_end_date TEXT NOT NULL,
    cost INTEGER NOT NULL DEFAULT 0,
    invoice_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT,
    FOREIGN KEY (product_warranty_id) REFERENCES product_warranties(id),
    FOREIGN KEY (policy_id) REFERENCES warranty_policies(id)
);

CREATE INDEX idx_warranty_extensions_warranty ON warranty_extensions(product_warranty_id);
CREATE INDEX idx_warranty_extensions_status ON warranty_extensions(status);
