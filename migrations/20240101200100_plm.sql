CREATE TABLE IF NOT EXISTS plm_items (
    id TEXT PRIMARY KEY,
    item_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    version TEXT NOT NULL DEFAULT '1.0',
    revision INTEGER NOT NULL DEFAULT 1,
    lifecycle_phase TEXT NOT NULL DEFAULT 'Concept',
    owner_id TEXT,
    product_id TEXT,
    parent_item_id TEXT,
    effective_date TEXT,
    obsolete_date TEXT,
    security_classification TEXT NOT NULL DEFAULT 'Internal',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_documents (
    id TEXT PRIMARY KEY,
    document_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    document_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    version TEXT NOT NULL DEFAULT '1.0',
    revision INTEGER NOT NULL DEFAULT 1,
    file_path TEXT,
    file_size INTEGER,
    file_format TEXT,
    checksum TEXT,
    owner_id TEXT,
    checked_out_by TEXT,
    checked_out_at TEXT,
    effective_date TEXT,
    obsolete_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_boms (
    id TEXT PRIMARY KEY,
    bom_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    item_id TEXT NOT NULL,
    version TEXT NOT NULL DEFAULT '1.0',
    revision INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Draft',
    bom_type TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit_of_measure TEXT NOT NULL,
    effective_date TEXT,
    obsolete_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_bom_lines (
    id TEXT PRIMARY KEY,
    bom_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    quantity REAL NOT NULL,
    unit_of_measure TEXT NOT NULL,
    find_number INTEGER,
    reference_designator TEXT,
    substitute_item_id TEXT,
    is_phantom INTEGER DEFAULT 0,
    sort_order INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_ecrs (
    id TEXT PRIMARY KEY,
    ecr_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    reason TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'Medium',
    status TEXT NOT NULL DEFAULT 'Draft',
    change_type TEXT NOT NULL,
    requested_by TEXT NOT NULL,
    submitted_at TEXT,
    target_date TEXT,
    implemented_date TEXT,
    impact_assessment TEXT,
    cost_estimate INTEGER,
    currency TEXT,
    approved_by TEXT,
    approved_at TEXT,
    rejected_reason TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_ecns (
    id TEXT PRIMARY KEY,
    ecn_number TEXT NOT NULL UNIQUE,
    ecr_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    effective_date TEXT NOT NULL,
    implementation_instructions TEXT,
    created_by TEXT NOT NULL,
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_ecn_affected_items (
    id TEXT PRIMARY KEY,
    ecn_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    old_revision TEXT NOT NULL,
    new_revision TEXT NOT NULL,
    old_version TEXT NOT NULL,
    new_version TEXT NOT NULL,
    change_description TEXT NOT NULL,
    disposition TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_workflows (
    id TEXT PRIMARY KEY,
    workflow_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    workflow_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    initiated_by TEXT NOT NULL,
    current_step INTEGER DEFAULT 0,
    total_steps INTEGER NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_workflow_steps (
    id TEXT PRIMARY KEY,
    workflow_id TEXT NOT NULL,
    step_number INTEGER NOT NULL,
    step_name TEXT NOT NULL,
    step_type TEXT NOT NULL,
    assignee_id TEXT,
    role_id TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    due_date TEXT,
    completed_at TEXT,
    completed_by TEXT,
    comments TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_cad_files (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    cad_system TEXT NOT NULL,
    format TEXT NOT NULL,
    version TEXT NOT NULL,
    thumbnail_path TEXT,
    geometry_data TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_specifications (
    id TEXT PRIMARY KEY,
    spec_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    item_id TEXT,
    spec_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    version TEXT NOT NULL DEFAULT '1.0',
    revision INTEGER NOT NULL DEFAULT 1,
    parameters TEXT NOT NULL,
    owner_id TEXT,
    effective_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_spec_parameters (
    id TEXT PRIMARY KEY,
    spec_id TEXT NOT NULL,
    parameter_name TEXT NOT NULL,
    parameter_type TEXT NOT NULL,
    target_value TEXT NOT NULL,
    min_value TEXT,
    max_value TEXT,
    unit TEXT,
    test_method TEXT,
    is_critical INTEGER DEFAULT 0,
    sort_order INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_design_reviews (
    id TEXT PRIMARY KEY,
    review_number TEXT NOT NULL UNIQUE,
    item_id TEXT NOT NULL,
    review_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Scheduled',
    scheduled_date TEXT NOT NULL,
    conducted_date TEXT,
    facilitator_id TEXT,
    location TEXT,
    outcome TEXT,
    action_items TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_design_review_attendees (
    id TEXT PRIMARY KEY,
    review_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT NOT NULL,
    attended INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_compliance_requirements (
    id TEXT PRIMARY KEY,
    requirement_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    regulation TEXT NOT NULL,
    category TEXT NOT NULL,
    mandatory INTEGER DEFAULT 1,
    verification_method TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS plm_item_compliances (
    id TEXT PRIMARY KEY,
    item_id TEXT NOT NULL,
    requirement_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    certified INTEGER DEFAULT 0,
    certification_date TEXT,
    certification_expiry TEXT,
    certifying_body TEXT,
    certificate_number TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
