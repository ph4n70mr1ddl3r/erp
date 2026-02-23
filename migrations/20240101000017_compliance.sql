CREATE TABLE data_subjects (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT,
    last_name TEXT,
    phone TEXT,
    address TEXT,
    identifier_type TEXT,
    identifier_value TEXT,
    verification_status TEXT NOT NULL DEFAULT '"Unverified"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE consent_records (
    id TEXT PRIMARY KEY,
    data_subject_id TEXT NOT NULL,
    consent_type TEXT NOT NULL,
    purpose TEXT NOT NULL,
    legal_basis TEXT NOT NULL DEFAULT '"Consent"',
    granted_at TEXT,
    withdrawn_at TEXT,
    source TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    evidence_path TEXT,
    status TEXT NOT NULL DEFAULT '"Pending"',
    expiry_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (data_subject_id) REFERENCES data_subjects(id)
);

CREATE TABLE data_processing_activities (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    purpose TEXT NOT NULL,
    legal_basis TEXT NOT NULL DEFAULT '"Consent"',
    data_categories TEXT NOT NULL DEFAULT '[]',
    data_subjects TEXT NOT NULL DEFAULT '[]',
    recipients TEXT NOT NULL DEFAULT '[]',
    third_country_transfers TEXT NOT NULL DEFAULT '[]',
    retention_period_days INTEGER NOT NULL DEFAULT 365,
    security_measures TEXT NOT NULL DEFAULT '[]',
    dpo_review_date TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    owner_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE data_breaches (
    id TEXT PRIMARY KEY,
    breach_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    breach_type TEXT NOT NULL DEFAULT '"Confidentiality"',
    severity TEXT NOT NULL DEFAULT '"Medium"',
    discovered_at TEXT NOT NULL,
    occurred_at TEXT,
    reported_at TEXT,
    affected_records INTEGER NOT NULL DEFAULT 0,
    affected_data_subjects INTEGER NOT NULL DEFAULT 0,
    data_categories TEXT NOT NULL DEFAULT '[]',
    containment_measures TEXT,
    remediation_measures TEXT,
    authority_notified INTEGER NOT NULL DEFAULT 0,
    authority_notification_date TEXT,
    subjects_notified INTEGER NOT NULL DEFAULT 0,
    subject_notification_date TEXT,
    status TEXT NOT NULL DEFAULT '"Detected"',
    assigned_to TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE dsar_requests (
    id TEXT PRIMARY KEY,
    request_number TEXT NOT NULL UNIQUE,
    data_subject_id TEXT NOT NULL,
    request_type TEXT NOT NULL DEFAULT '"Access"',
    description TEXT,
    received_at TEXT NOT NULL,
    due_date TEXT NOT NULL,
    completed_at TEXT,
    status TEXT NOT NULL DEFAULT '"Received"',
    assigned_to TEXT,
    response TEXT,
    rejection_reason TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (data_subject_id) REFERENCES data_subjects(id)
);

CREATE TABLE data_retention_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    data_category TEXT NOT NULL,
    retention_period_days INTEGER NOT NULL,
    legal_basis TEXT,
    disposal_method TEXT NOT NULL DEFAULT '"SecureDeletion"',
    review_frequency_days INTEGER NOT NULL DEFAULT 365,
    last_review_date TEXT,
    next_review_date TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE privacy_impact_assessments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    project_name TEXT NOT NULL,
    description TEXT NOT NULL,
    assessor_id TEXT,
    assessment_date TEXT NOT NULL,
    data_types TEXT NOT NULL DEFAULT '[]',
    processing_purposes TEXT NOT NULL DEFAULT '[]',
    data_subjects TEXT NOT NULL DEFAULT '[]',
    risks TEXT NOT NULL DEFAULT '[]',
    mitigation_measures TEXT NOT NULL DEFAULT '[]',
    residual_risk_level TEXT NOT NULL DEFAULT '"Low"',
    recommendation TEXT NOT NULL DEFAULT '"Proceed"',
    dpo_approval INTEGER NOT NULL DEFAULT 0,
    dpo_approved_at TEXT,
    dpo_comments TEXT,
    status TEXT NOT NULL DEFAULT '"Draft"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE third_party_processors (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    contact_name TEXT,
    contact_email TEXT,
    contact_phone TEXT,
    address TEXT,
    country TEXT NOT NULL,
    processing_activities TEXT NOT NULL DEFAULT '[]',
    data_categories TEXT NOT NULL DEFAULT '[]',
    contract_date TEXT,
    contract_expiry TEXT,
    dpa_signed INTEGER NOT NULL DEFAULT 0,
    security_assessment_date TEXT,
    security_assessment_result TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE cookie_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    cookie_type TEXT NOT NULL DEFAULT '"Essential"',
    purpose TEXT NOT NULL,
    provider TEXT,
    expiry TEXT NOT NULL,
    required INTEGER NOT NULL DEFAULT 0,
    consent_required INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE compliance_frameworks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    framework_type TEXT NOT NULL DEFAULT '"GDPR"',
    version TEXT NOT NULL,
    applicable_regions TEXT NOT NULL DEFAULT '[]',
    requirements TEXT NOT NULL DEFAULT '[]',
    assessment_frequency_days INTEGER NOT NULL DEFAULT 365,
    last_assessment TEXT,
    next_assessment TEXT,
    compliance_status TEXT NOT NULL DEFAULT '"NotStarted"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE data_inventory (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    data_type TEXT NOT NULL,
    source_system TEXT NOT NULL,
    location TEXT NOT NULL,
    owner_id TEXT,
    data_classification TEXT NOT NULL DEFAULT '"Internal"',
    contains_pii INTEGER NOT NULL DEFAULT 0,
    contains_phi INTEGER NOT NULL DEFAULT 0,
    contains_pci INTEGER NOT NULL DEFAULT 0,
    retention_policy_id TEXT,
    last_accessed TEXT,
    last_reviewed TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (retention_policy_id) REFERENCES data_retention_policies(id)
);

CREATE INDEX idx_consent_records_subject ON consent_records(data_subject_id);
CREATE INDEX idx_consent_records_status ON consent_records(status);
CREATE INDEX idx_dsar_requests_status ON dsar_requests(status);
CREATE INDEX idx_dsar_requests_due ON dsar_requests(due_date);
CREATE INDEX idx_data_breaches_status ON data_breaches(status);
CREATE INDEX idx_data_inventory_classification ON data_inventory(data_classification);
