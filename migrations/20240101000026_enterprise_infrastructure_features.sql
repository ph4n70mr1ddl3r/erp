CREATE TABLE IF NOT EXISTS feature_flags (
    id TEXT PRIMARY KEY,
    key TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    enabled INTEGER NOT NULL DEFAULT 0,
    rollout_percentage INTEGER NOT NULL DEFAULT 100,
    target_type TEXT NOT NULL DEFAULT 'All',
    target_ids TEXT,
    start_time TEXT,
    end_time TEXT,
    prerequisites TEXT,
    variants TEXT,
    default_variant TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS feature_flag_overrides (
    id TEXT PRIMARY KEY,
    flag_id TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 0,
    variant TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (flag_id) REFERENCES feature_flags(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS feature_flag_usage (
    id TEXT PRIMARY KEY,
    flag_id TEXT NOT NULL,
    user_id TEXT,
    variant TEXT,
    evaluated_at TEXT NOT NULL,
    context TEXT,
    FOREIGN KEY (flag_id) REFERENCES feature_flags(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS encryption_keys (
    id TEXT PRIMARY KEY,
    key_id TEXT NOT NULL UNIQUE,
    key_type TEXT NOT NULL,
    algorithm TEXT NOT NULL,
    key_version INTEGER NOT NULL DEFAULT 1,
    public_key TEXT,
    encrypted_private_key TEXT,
    key_derivation_info TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    is_primary INTEGER NOT NULL DEFAULT 0,
    rotation_days INTEGER,
    last_rotated TEXT,
    expires_at TEXT,
    max_usage_count INTEGER,
    current_usage_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS key_rotations (
    id TEXT PRIMARY KEY,
    key_id TEXT NOT NULL,
    from_version INTEGER NOT NULL,
    to_version INTEGER NOT NULL,
    rotation_type TEXT NOT NULL,
    status TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    re_encrypted_count INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    initiated_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (key_id) REFERENCES encryption_keys(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS key_usage_logs (
    id TEXT PRIMARY KEY,
    key_id TEXT NOT NULL,
    operation TEXT NOT NULL,
    entity_type TEXT,
    entity_id TEXT,
    success INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    performed_at TEXT NOT NULL,
    performed_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (key_id) REFERENCES encryption_keys(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS encrypted_data (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    field_name TEXT NOT NULL,
    key_id TEXT NOT NULL,
    key_version INTEGER NOT NULL,
    iv TEXT NOT NULL,
    auth_tag TEXT,
    encrypted_value TEXT NOT NULL,
    encryption_algorithm TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (key_id) REFERENCES encryption_keys(id)
);

CREATE TABLE IF NOT EXISTS key_policies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    key_type TEXT NOT NULL,
    algorithm TEXT NOT NULL,
    key_size_bits INTEGER NOT NULL,
    rotation_days INTEGER NOT NULL,
    max_usage_count INTEGER,
    require_hsm INTEGER NOT NULL DEFAULT 0,
    allow_export INTEGER NOT NULL DEFAULT 0,
    allowed_operations TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS backup_schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    backup_type TEXT NOT NULL DEFAULT 'Full',
    schedule_cron TEXT NOT NULL,
    retention_days INTEGER NOT NULL DEFAULT 30,
    max_backups INTEGER NOT NULL DEFAULT 10,
    compression INTEGER NOT NULL DEFAULT 1,
    encryption_enabled INTEGER NOT NULL DEFAULT 0,
    encryption_key_id TEXT,
    storage_type TEXT NOT NULL DEFAULT 'Local',
    storage_path TEXT NOT NULL,
    include_attachments INTEGER NOT NULL DEFAULT 1,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_run TEXT,
    next_run TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS backup_records (
    id TEXT PRIMARY KEY,
    schedule_id TEXT,
    backup_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    file_path TEXT NOT NULL,
    file_size_bytes INTEGER NOT NULL DEFAULT 0,
    compressed_size_bytes INTEGER,
    checksum TEXT,
    checksum_algorithm TEXT,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    duration_seconds INTEGER,
    tables_included TEXT,
    records_count INTEGER,
    error_message TEXT,
    verification_status TEXT,
    verified_at TEXT,
    is_restorable INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (schedule_id) REFERENCES backup_schedules(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS restore_operations (
    id TEXT PRIMARY KEY,
    backup_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    restore_type TEXT NOT NULL DEFAULT 'Full',
    target_tables TEXT,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    duration_seconds INTEGER,
    records_restored INTEGER,
    error_message TEXT,
    initiated_by TEXT,
    backup_before_restore TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (backup_id) REFERENCES backup_records(id)
);

CREATE TABLE IF NOT EXISTS backup_verifications (
    id TEXT PRIMARY KEY,
    backup_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    checked_at TEXT NOT NULL,
    checksum_valid INTEGER NOT NULL DEFAULT 0,
    file_readable INTEGER NOT NULL DEFAULT 0,
    schema_valid INTEGER NOT NULL DEFAULT 0,
    sample_data_valid INTEGER NOT NULL DEFAULT 0,
    error_details TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (backup_id) REFERENCES backup_records(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS system_metrics (
    id TEXT PRIMARY KEY,
    metric_type TEXT NOT NULL,
    metric_name TEXT NOT NULL,
    value REAL NOT NULL,
    unit TEXT NOT NULL,
    tags TEXT,
    recorded_at TEXT NOT NULL,
    hostname TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS health_checks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    check_type TEXT NOT NULL,
    endpoint TEXT,
    timeout_seconds INTEGER NOT NULL DEFAULT 30,
    interval_seconds INTEGER NOT NULL DEFAULT 60,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_check TEXT,
    last_status TEXT,
    last_response_time_ms INTEGER,
    consecutive_failures INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS health_check_results (
    id TEXT PRIMARY KEY,
    check_id TEXT NOT NULL,
    status TEXT NOT NULL,
    response_time_ms INTEGER NOT NULL,
    message TEXT,
    details TEXT,
    checked_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (check_id) REFERENCES health_checks(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS alert_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    metric_type TEXT NOT NULL,
    metric_name TEXT NOT NULL,
    condition TEXT NOT NULL,
    threshold REAL NOT NULL,
    duration_minutes INTEGER NOT NULL DEFAULT 5,
    severity TEXT NOT NULL DEFAULT 'Warning',
    notification_channels TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_triggered TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS alerts (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Firing',
    severity TEXT NOT NULL,
    message TEXT NOT NULL,
    value REAL NOT NULL,
    threshold REAL NOT NULL,
    triggered_at TEXT NOT NULL,
    acknowledged_at TEXT,
    acknowledged_by TEXT,
    resolved_at TEXT,
    resolution_note TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (rule_id) REFERENCES alert_rules(id)
);

CREATE TABLE IF NOT EXISTS custom_roles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    parent_role_id TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    FOREIGN KEY (parent_role_id) REFERENCES custom_roles(id)
);

CREATE TABLE IF NOT EXISTS permissions (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    module TEXT NOT NULL,
    resource TEXT NOT NULL,
    action TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS role_permissions (
    id TEXT PRIMARY KEY,
    role_id TEXT NOT NULL,
    permission_id TEXT NOT NULL,
    granted_at TEXT NOT NULL,
    granted_by TEXT,
    UNIQUE(role_id, permission_id),
    FOREIGN KEY (role_id) REFERENCES custom_roles(id) ON DELETE CASCADE,
    FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_role_assignments (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    assigned_at TEXT NOT NULL,
    assigned_by TEXT,
    expires_at TEXT,
    UNIQUE(user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES custom_roles(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS data_permissions (
    id TEXT PRIMARY KEY,
    role_id TEXT NOT NULL,
    resource TEXT NOT NULL,
    filter_type TEXT NOT NULL DEFAULT 'All',
    filter_value TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (role_id) REFERENCES custom_roles(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS field_permissions (
    id TEXT PRIMARY KEY,
    role_id TEXT NOT NULL,
    resource TEXT NOT NULL,
    field_name TEXT NOT NULL,
    can_read INTEGER NOT NULL DEFAULT 1,
    can_write INTEGER NOT NULL DEFAULT 0,
    can_create INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    UNIQUE(role_id, resource, field_name),
    FOREIGN KEY (role_id) REFERENCES custom_roles(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS permission_audit_logs (
    id TEXT PRIMARY KEY,
    action TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    old_value TEXT,
    new_value TEXT,
    performed_at TEXT NOT NULL,
    ip_address TEXT,
    FOREIGN KEY (actor_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_feature_flags_key ON feature_flags(key);
CREATE INDEX IF NOT EXISTS idx_feature_flag_overrides_flag ON feature_flag_overrides(flag_id);
CREATE INDEX IF NOT EXISTS idx_encryption_keys_active ON encryption_keys(is_active, is_primary);
CREATE INDEX IF NOT EXISTS idx_backup_records_status ON backup_records(status);
CREATE INDEX IF NOT EXISTS idx_system_metrics_type ON system_metrics(metric_type, recorded_at);
CREATE INDEX IF NOT EXISTS idx_health_checks_active ON health_checks(is_active);
CREATE INDEX IF NOT EXISTS idx_alerts_status ON alerts(status, triggered_at);
CREATE INDEX IF NOT EXISTS idx_role_permissions_role ON role_permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_user_role_assignments_user ON user_role_assignments(user_id);
