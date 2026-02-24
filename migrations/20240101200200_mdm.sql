CREATE TABLE IF NOT EXISTS mdm_entities (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_code TEXT NOT NULL,
    entity_name TEXT NOT NULL,
    source_system TEXT NOT NULL,
    source_id TEXT NOT NULL,
    golden_record_id TEXT,
    quality_score INTEGER DEFAULT 0,
    completeness_score INTEGER DEFAULT 0,
    accuracy_score INTEGER DEFAULT 0,
    timeliness_score INTEGER DEFAULT 0,
    consistency_score INTEGER DEFAULT 0,
    last_verified TEXT,
    next_verification TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(entity_type, source_system, source_id)
);

CREATE TABLE IF NOT EXISTS mdm_golden_records (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    golden_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    attributes TEXT NOT NULL,
    source_count INTEGER DEFAULT 0,
    confidence_score INTEGER DEFAULT 0,
    steward_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    version INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_golden_record_sources (
    id TEXT PRIMARY KEY,
    golden_record_id TEXT NOT NULL,
    source_entity_id TEXT NOT NULL,
    match_score INTEGER NOT NULL,
    is_primary INTEGER DEFAULT 0,
    contributed_at TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_quality_rules (
    id TEXT PRIMARY KEY,
    rule_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    entity_type TEXT NOT NULL,
    field_name TEXT NOT NULL,
    rule_type TEXT NOT NULL,
    rule_expression TEXT NOT NULL,
    severity TEXT NOT NULL DEFAULT 'Medium',
    is_active INTEGER DEFAULT 1,
    auto_fix INTEGER DEFAULT 0,
    fix_expression TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_quality_violations (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    field_name TEXT NOT NULL,
    current_value TEXT,
    expected_value TEXT,
    severity TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Open',
    detected_at TEXT NOT NULL,
    resolved_at TEXT,
    resolved_by TEXT,
    resolution_notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_match_rules (
    id TEXT PRIMARY KEY,
    rule_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    entity_type TEXT NOT NULL,
    match_type TEXT NOT NULL,
    blocking_rules TEXT NOT NULL,
    matching_rules TEXT NOT NULL,
    threshold_score INTEGER NOT NULL DEFAULT 80,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_match_results (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL,
    entity1_id TEXT NOT NULL,
    entity2_id TEXT NOT NULL,
    match_score INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    matched_at TEXT NOT NULL,
    reviewed_by TEXT,
    reviewed_at TEXT,
    decision_notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_data_domains (
    id TEXT PRIMARY KEY,
    domain_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    parent_domain_id TEXT,
    owner_id TEXT,
    steward_id TEXT,
    data_classification TEXT NOT NULL DEFAULT 'Internal',
    retention_policy TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_data_attributes (
    id TEXT PRIMARY KEY,
    domain_id TEXT NOT NULL,
    attribute_code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    data_type TEXT NOT NULL,
    max_length INTEGER,
    is_required INTEGER DEFAULT 0,
    is_unique INTEGER DEFAULT 0,
    default_value TEXT,
    validation_regex TEXT,
    allowed_values TEXT,
    business_rules TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(domain_id, attribute_code)
);

CREATE TABLE IF NOT EXISTS mdm_data_stewards (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    domain_id TEXT,
    entity_types TEXT NOT NULL,
    responsibilities TEXT NOT NULL,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_data_lineage (
    id TEXT PRIMARY KEY,
    source_entity_id TEXT NOT NULL,
    target_entity_id TEXT NOT NULL,
    transformation_type TEXT NOT NULL,
    transformation_logic TEXT,
    flow_type TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_data_workflows (
    id TEXT PRIMARY KEY,
    workflow_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    entity_type TEXT NOT NULL,
    workflow_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    initiated_by TEXT,
    current_step INTEGER DEFAULT 0,
    total_steps INTEGER NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_data_workflow_steps (
    id TEXT PRIMARY KEY,
    workflow_id TEXT NOT NULL,
    step_number INTEGER NOT NULL,
    step_name TEXT NOT NULL,
    action_type TEXT NOT NULL,
    assignee_id TEXT,
    role_id TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    due_date TEXT,
    completed_at TEXT,
    completed_by TEXT,
    comments TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_duplicate_records (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    primary_entity_id TEXT NOT NULL,
    duplicate_entity_id TEXT NOT NULL,
    similarity_score INTEGER NOT NULL,
    matched_fields TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    merge_initiated_by TEXT,
    merge_initiated_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_import_jobs (
    id TEXT PRIMARY KEY,
    job_name TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    source_file TEXT NOT NULL,
    total_records INTEGER DEFAULT 0,
    processed_records INTEGER DEFAULT 0,
    success_records INTEGER DEFAULT 0,
    failed_records INTEGER DEFAULT 0,
    duplicate_records INTEGER DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    started_at TEXT,
    completed_at TEXT,
    error_log TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_export_jobs (
    id TEXT PRIMARY KEY,
    job_name TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    filter_criteria TEXT,
    export_format TEXT NOT NULL,
    output_file TEXT,
    total_records INTEGER DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mdm_reference_data (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    parent_code TEXT,
    sort_order INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    effective_date TEXT,
    expiry_date TEXT,
    attributes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(category, code)
);
