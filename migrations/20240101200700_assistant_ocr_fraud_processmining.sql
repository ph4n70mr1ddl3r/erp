CREATE TABLE IF NOT EXISTS assistant_conversations (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    context TEXT NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS assistant_messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    intent TEXT,
    entities TEXT NOT NULL DEFAULT '{}',
    action_taken TEXT,
    action_result TEXT,
    feedback_rating INTEGER,
    feedback_comment TEXT,
    feedback_created_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (conversation_id) REFERENCES assistant_conversations(id)
);

CREATE TABLE IF NOT EXISTS assistant_intents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    training_phrases TEXT NOT NULL DEFAULT '[]',
    action_template TEXT NOT NULL,
    parameters TEXT NOT NULL DEFAULT '[]',
    confidence_threshold REAL NOT NULL DEFAULT 0.8,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS assistant_skills (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    trigger_phrases TEXT NOT NULL DEFAULT '[]',
    handler_module TEXT NOT NULL,
    handler_function TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS assistant_quick_actions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    icon TEXT NOT NULL DEFAULT 'action',
    action TEXT NOT NULL,
    category TEXT NOT NULL,
    shortcut TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ocr_documents (
    id TEXT PRIMARY KEY,
    document_type TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    processing_started_at TEXT,
    processing_completed_at TEXT,
    confidence_score REAL,
    raw_text TEXT,
    extracted_data TEXT,
    validation_errors TEXT NOT NULL DEFAULT '[]',
    reviewed_by TEXT,
    reviewed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ocr_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    document_type TEXT NOT NULL,
    vendor_id TEXT,
    field_mappings TEXT NOT NULL DEFAULT '[]',
    sample_images TEXT NOT NULL DEFAULT '[]',
    accuracy_score REAL NOT NULL DEFAULT 0.0,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ocr_batch_jobs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    document_ids TEXT NOT NULL DEFAULT '[]',
    template_id TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    total_documents INTEGER NOT NULL DEFAULT 0,
    processed_documents INTEGER NOT NULL DEFAULT 0,
    successful_documents INTEGER NOT NULL DEFAULT 0,
    failed_documents INTEGER NOT NULL DEFAULT 0,
    started_at TEXT,
    completed_at TEXT,
    error_details TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fraud_alerts (
    id TEXT PRIMARY KEY,
    alert_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'new',
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    rule_id TEXT,
    score REAL NOT NULL DEFAULT 0.0,
    risk_factors TEXT NOT NULL DEFAULT '[]',
    description TEXT NOT NULL,
    detected_at TEXT NOT NULL,
    reviewed_by TEXT,
    reviewed_at TEXT,
    resolution_type TEXT,
    resolution_notes TEXT,
    resolution_actions TEXT,
    resolution_resolved_by TEXT,
    resolution_resolved_at TEXT,
    assigned_to TEXT,
    due_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fraud_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    rule_type TEXT NOT NULL,
    conditions TEXT NOT NULL DEFAULT '[]',
    actions TEXT NOT NULL DEFAULT '[]',
    enabled INTEGER NOT NULL DEFAULT 1,
    priority INTEGER NOT NULL DEFAULT 0,
    false_positive_rate REAL NOT NULL DEFAULT 0.0,
    true_positive_rate REAL NOT NULL DEFAULT 0.0,
    last_triggered TEXT,
    trigger_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fraud_cases (
    id TEXT PRIMARY KEY,
    case_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    priority TEXT NOT NULL DEFAULT 'medium',
    assigned_investigator TEXT,
    alert_ids TEXT NOT NULL DEFAULT '[]',
    related_entities TEXT NOT NULL DEFAULT '[]',
    timeline TEXT NOT NULL DEFAULT '[]',
    evidence TEXT NOT NULL DEFAULT '[]',
    estimated_loss INTEGER NOT NULL DEFAULT 0,
    actual_loss INTEGER,
    recovery_amount INTEGER,
    opened_at TEXT NOT NULL,
    closed_at TEXT,
    resolution_outcome TEXT,
    resolution_summary TEXT,
    resolution_actions TEXT NOT NULL DEFAULT '[]',
    resolution_recommendations TEXT NOT NULL DEFAULT '[]',
    resolution_resolved_by TEXT,
    resolution_resolved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS vendor_risk_profiles (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL UNIQUE,
    risk_score REAL NOT NULL DEFAULT 0.0,
    risk_level TEXT NOT NULL DEFAULT 'low',
    risk_factors TEXT NOT NULL DEFAULT '[]',
    historical_alerts INTEGER NOT NULL DEFAULT 0,
    payment_anomalies INTEGER NOT NULL DEFAULT 0,
    days_since_first_transaction INTEGER NOT NULL DEFAULT 0,
    total_transaction_value INTEGER NOT NULL DEFAULT 0,
    average_transaction_value INTEGER NOT NULL DEFAULT 0,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    duplicate_invoice_attempts INTEGER NOT NULL DEFAULT 0,
    address_changes INTEGER NOT NULL DEFAULT 0,
    bank_account_changes INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS employee_risk_profiles (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL UNIQUE,
    risk_score REAL NOT NULL DEFAULT 0.0,
    risk_level TEXT NOT NULL DEFAULT 'low',
    risk_factors TEXT NOT NULL DEFAULT '[]',
    expense_anomalies INTEGER NOT NULL DEFAULT 0,
    access_violations INTEGER NOT NULL DEFAULT 0,
    policy_violations INTEGER NOT NULL DEFAULT 0,
    total_expense_value INTEGER NOT NULL DEFAULT 0,
    average_expense_value INTEGER NOT NULL DEFAULT 0,
    expense_count INTEGER NOT NULL DEFAULT 0,
    after_hours_access INTEGER NOT NULL DEFAULT 0,
    data_export_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS process_definitions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    version TEXT NOT NULL DEFAULT '1.0',
    status TEXT NOT NULL DEFAULT 'draft',
    owner_id TEXT,
    start_event TEXT NOT NULL,
    end_events TEXT NOT NULL DEFAULT '[]',
    expected_duration_hours INTEGER,
    sla_hours INTEGER,
    tags TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS process_instances (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    case_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'running',
    start_time TEXT NOT NULL,
    end_time TEXT,
    duration_hours REAL,
    initiator_id TEXT,
    entity_type TEXT,
    entity_id TEXT,
    variant_id TEXT,
    is_compliant INTEGER,
    deviation_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_id) REFERENCES process_definitions(id)
);

CREATE TABLE IF NOT EXISTS process_events (
    id TEXT PRIMARY KEY,
    instance_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    activity_name TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    user_id TEXT,
    role_id TEXT,
    department_id TEXT,
    resource TEXT,
    previous_state TEXT,
    new_state TEXT,
    duration_ms INTEGER,
    metadata TEXT NOT NULL DEFAULT '{}',
    cost_cents INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (instance_id) REFERENCES process_instances(id)
);

CREATE TABLE IF NOT EXISTS process_variants (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    variant_hash TEXT NOT NULL,
    activity_sequence TEXT NOT NULL DEFAULT '[]',
    frequency INTEGER NOT NULL DEFAULT 0,
    percentage REAL NOT NULL DEFAULT 0.0,
    avg_duration_hours REAL NOT NULL DEFAULT 0.0,
    min_duration_hours REAL NOT NULL DEFAULT 0.0,
    max_duration_hours REAL NOT NULL DEFAULT 0.0,
    is_happy_path INTEGER NOT NULL DEFAULT 0,
    deviation_from_standard REAL NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_id) REFERENCES process_definitions(id)
);

CREATE TABLE IF NOT EXISTS process_discoveries (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    discovery_date TEXT NOT NULL,
    total_cases INTEGER NOT NULL DEFAULT 0,
    total_events INTEGER NOT NULL DEFAULT 0,
    unique_activities INTEGER NOT NULL DEFAULT 0,
    unique_variants INTEGER NOT NULL DEFAULT 0,
    avg_case_duration_hours REAL NOT NULL DEFAULT 0.0,
    median_case_duration_hours REAL NOT NULL DEFAULT 0.0,
    activity_frequencies TEXT NOT NULL DEFAULT '{}',
    transition_frequencies TEXT NOT NULL DEFAULT '{}',
    start_activities TEXT NOT NULL DEFAULT '[]',
    end_activities TEXT NOT NULL DEFAULT '[]',
    self_loops TEXT NOT NULL DEFAULT '[]',
    rework_loops TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_id) REFERENCES process_definitions(id)
);

CREATE TABLE IF NOT EXISTS bottleneck_analyses (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    analysis_date TEXT NOT NULL,
    bottlenecks TEXT NOT NULL DEFAULT '[]',
    waiting_time_analysis TEXT NOT NULL DEFAULT '[]',
    resource_utilization TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_id) REFERENCES process_definitions(id)
);

CREATE TABLE IF NOT EXISTS conformance_checks (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    check_date TEXT NOT NULL,
    total_cases INTEGER NOT NULL DEFAULT 0,
    conformant_cases INTEGER NOT NULL DEFAULT 0,
    conformance_rate REAL NOT NULL DEFAULT 0.0,
    deviations TEXT NOT NULL DEFAULT '[]',
    deviating_variants TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_id) REFERENCES process_definitions(id)
);

CREATE TABLE IF NOT EXISTS process_simulations (
    id TEXT PRIMARY KEY,
    process_id TEXT NOT NULL,
    simulation_name TEXT NOT NULL,
    scenario_name TEXT NOT NULL,
    scenario_modifications TEXT NOT NULL DEFAULT '[]',
    scenario_duration_days INTEGER NOT NULL DEFAULT 30,
    scenario_case_arrival_rate REAL NOT NULL DEFAULT 0.0,
    scenario_resource_allocation TEXT NOT NULL DEFAULT '{}',
    start_time TEXT NOT NULL,
    end_time TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    results_avg_cycle_time_hours REAL,
    results_throughput_per_day REAL,
    results_resource_utilization TEXT,
    results_bottleneck_activities TEXT,
    results_improvement_percentage REAL,
    results_projected_cost_savings INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_id) REFERENCES process_definitions(id)
);

CREATE INDEX IF NOT EXISTS idx_assistant_messages_conversation ON assistant_messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_ocr_documents_status ON ocr_documents(status);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_status ON fraud_alerts(status);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_severity ON fraud_alerts(severity);
CREATE INDEX IF NOT EXISTS idx_fraud_cases_status ON fraud_cases(status);
CREATE INDEX IF NOT EXISTS idx_process_instances_process ON process_instances(process_id);
CREATE INDEX IF NOT EXISTS idx_process_events_instance ON process_events(instance_id);
CREATE INDEX IF NOT EXISTS idx_process_events_timestamp ON process_events(timestamp);
