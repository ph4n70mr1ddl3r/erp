-- Notifications System
CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    channel TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'Normal',
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    action_url TEXT,
    action_text TEXT,
    icon TEXT,
    image_url TEXT,
    data TEXT,
    template_id TEXT,
    related_entity_type TEXT,
    related_entity_id TEXT,
    scheduled_at TEXT,
    sent_at TEXT,
    delivered_at TEXT,
    read_at TEXT,
    dismissed_at TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    last_error TEXT,
    expires_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS notification_preferences (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    channel TEXT NOT NULL,
    enabled INTEGER DEFAULT 1,
    priority_threshold TEXT DEFAULT 'Normal',
    quiet_hours_start TEXT,
    quiet_hours_end TEXT,
    quiet_hours_timezone TEXT,
    digest_enabled INTEGER DEFAULT 0,
    digest_frequency TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(user_id, notification_type, channel)
);

CREATE TABLE IF NOT EXISTS notification_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    notification_type TEXT NOT NULL,
    channel TEXT NOT NULL,
    subject_template TEXT,
    body_template TEXT NOT NULL,
    html_template TEXT,
    variables TEXT,
    default_priority TEXT DEFAULT 'Normal',
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS notification_batches (
    id TEXT PRIMARY KEY,
    batch_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    channel TEXT NOT NULL,
    template_id TEXT,
    total_recipients INTEGER DEFAULT 0,
    sent_count INTEGER DEFAULT 0,
    delivered_count INTEGER DEFAULT 0,
    failed_count INTEGER DEFAULT 0,
    opened_count INTEGER DEFAULT 0,
    clicked_count INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Draft',
    scheduled_at TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS notification_providers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    channel TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    configuration TEXT,
    credentials_encrypted TEXT,
    webhook_url TEXT,
    is_default INTEGER DEFAULT 0,
    daily_limit INTEGER,
    daily_used INTEGER DEFAULT 0,
    monthly_limit INTEGER,
    monthly_used INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS device_tokens (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    device_type TEXT NOT NULL,
    token TEXT NOT NULL,
    device_name TEXT,
    os_version TEXT,
    app_version TEXT,
    last_used_at TEXT,
    is_active INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Webhooks System
CREATE TABLE IF NOT EXISTS webhook_endpoints (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    url TEXT NOT NULL,
    secret TEXT NOT NULL,
    events TEXT NOT NULL,
    headers TEXT,
    authentication TEXT,
    timeout_seconds INTEGER DEFAULT 30,
    retry_policy TEXT,
    status TEXT DEFAULT 'Active',
    created_by TEXT NOT NULL,
    last_triggered_at TEXT,
    last_success_at TEXT,
    last_failure_at TEXT,
    total_triggers INTEGER DEFAULT 0,
    successful_triggers INTEGER DEFAULT 0,
    failed_triggers INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS webhook_deliveries (
    id TEXT PRIMARY KEY,
    endpoint_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    event_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    headers TEXT,
    response_status INTEGER,
    response_body TEXT,
    response_headers TEXT,
    duration_ms INTEGER,
    attempt_number INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 5,
    next_retry_at TEXT,
    delivered_at TEXT,
    status TEXT DEFAULT 'Pending',
    error_message TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS webhook_events (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    source_entity_type TEXT NOT NULL,
    source_entity_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    triggered_by TEXT NOT NULL,
    triggered_at TEXT NOT NULL,
    delivered INTEGER DEFAULT 0,
    delivery_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);

-- Jobs System
CREATE TABLE IF NOT EXISTS scheduled_jobs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    job_type TEXT NOT NULL,
    handler TEXT NOT NULL,
    payload TEXT,
    priority TEXT DEFAULT 'Normal',
    cron_expression TEXT,
    interval_seconds INTEGER,
    scheduled_at TEXT,
    started_at TEXT,
    completed_at TEXT,
    next_run_at TEXT,
    last_run_at TEXT,
    last_success_at TEXT,
    last_failure_at TEXT,
    status TEXT DEFAULT 'Pending',
    run_count INTEGER DEFAULT 0,
    success_count INTEGER DEFAULT 0,
    failure_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    retry_count INTEGER DEFAULT 0,
    retry_delay_seconds INTEGER DEFAULT 60,
    timeout_seconds INTEGER DEFAULT 300,
    last_error TEXT,
    last_duration_ms INTEGER,
    avg_duration_ms INTEGER,
    tags TEXT,
    created_by TEXT,
    locked_by TEXT,
    locked_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS job_executions (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    execution_number INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    duration_ms INTEGER,
    status TEXT DEFAULT 'Running',
    result TEXT,
    error_message TEXT,
    error_stack_trace TEXT,
    retry_of_id TEXT,
    retry_number INTEGER DEFAULT 0,
    worker_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS job_schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    job_template_id TEXT,
    job_name TEXT NOT NULL,
    handler TEXT NOT NULL,
    default_payload TEXT,
    schedule_type TEXT NOT NULL,
    cron_expression TEXT,
    interval_minutes INTEGER,
    specific_times TEXT,
    run_on_days TEXT,
    timezone TEXT DEFAULT 'UTC',
    start_date TEXT,
    end_date TEXT,
    next_scheduled_run TEXT,
    last_run TEXT,
    enabled INTEGER DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS job_queues (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    max_concurrent_jobs INTEGER DEFAULT 10,
    current_jobs INTEGER DEFAULT 0,
    total_processed INTEGER DEFAULT 0,
    total_failed INTEGER DEFAULT 0,
    avg_wait_time_ms INTEGER,
    avg_process_time_ms INTEGER,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Integration System
CREATE TABLE IF NOT EXISTS api_keys (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    key_hash TEXT NOT NULL,
    key_prefix TEXT NOT NULL UNIQUE,
    user_id TEXT,
    scopes TEXT NOT NULL,
    rate_limit_per_minute INTEGER,
    rate_limit_per_hour INTEGER,
    rate_limit_per_day INTEGER,
    allowed_ips TEXT,
    allowed_origins TEXT,
    expires_at TEXT,
    last_used_at TEXT,
    usage_count INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Active',
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS api_key_usage (
    id TEXT PRIMARY KEY,
    api_key_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    method TEXT NOT NULL,
    status_code INTEGER NOT NULL,
    response_time_ms INTEGER NOT NULL,
    request_size INTEGER DEFAULT 0,
    response_size INTEGER DEFAULT 0,
    ip_address TEXT,
    user_agent TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS external_connections (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    connection_type TEXT NOT NULL,
    description TEXT,
    endpoint_url TEXT,
    configuration TEXT,
    credentials_encrypted TEXT,
    auth_type TEXT DEFAULT 'None',
    auth_config TEXT,
    status TEXT DEFAULT 'Pending',
    last_sync_at TEXT,
    last_sync_status TEXT,
    last_error TEXT,
    sync_interval_minutes INTEGER,
    auto_sync INTEGER DEFAULT 0,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS connection_sync_logs (
    id TEXT PRIMARY KEY,
    connection_id TEXT NOT NULL,
    sync_type TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    status TEXT DEFAULT 'Running',
    records_processed INTEGER DEFAULT 0,
    records_created INTEGER DEFAULT 0,
    records_updated INTEGER DEFAULT 0,
    records_failed INTEGER DEFAULT 0,
    error_message TEXT,
    details TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS integration_flows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    trigger_type TEXT NOT NULL,
    trigger_config TEXT,
    steps TEXT NOT NULL,
    error_handling TEXT DEFAULT 'StopOnError',
    retry_policy TEXT,
    enabled INTEGER DEFAULT 1,
    execution_count INTEGER DEFAULT 0,
    success_count INTEGER DEFAULT 0,
    failure_count INTEGER DEFAULT 0,
    last_execution_at TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS flow_executions (
    id TEXT PRIMARY KEY,
    flow_id TEXT NOT NULL,
    trigger_type TEXT NOT NULL,
    trigger_data TEXT,
    status TEXT DEFAULT 'Pending',
    started_at TEXT NOT NULL,
    completed_at TEXT,
    current_step INTEGER,
    total_steps INTEGER DEFAULT 0,
    step_results TEXT,
    error_message TEXT,
    error_step INTEGER,
    created_at TEXT NOT NULL
);

-- Templates System
CREATE TABLE IF NOT EXISTS templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    template_type TEXT NOT NULL,
    format TEXT NOT NULL,
    subject TEXT,
    body TEXT NOT NULL,
    html_body TEXT,
    variables TEXT,
    default_values TEXT,
    styles TEXT,
    header_template_id TEXT,
    footer_template_id TEXT,
    version INTEGER DEFAULT 1,
    parent_id TEXT,
    status TEXT DEFAULT 'Active',
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS template_versions (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    version INTEGER NOT NULL,
    subject TEXT,
    body TEXT NOT NULL,
    html_body TEXT,
    variables TEXT,
    change_summary TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE(template_id, version)
);

CREATE TABLE IF NOT EXISTS template_variables (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    description TEXT,
    variable_type TEXT DEFAULT 'String',
    default_value TEXT,
    required INTEGER DEFAULT 0,
    validation_regex TEXT,
    options TEXT,
    sort_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    UNIQUE(template_id, name)
);

CREATE TABLE IF NOT EXISTS generated_documents (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    template_version INTEGER NOT NULL,
    name TEXT NOT NULL,
    output_format TEXT NOT NULL,
    content TEXT,
    file_path TEXT,
    file_size INTEGER,
    variables_used TEXT,
    related_entity_type TEXT,
    related_entity_id TEXT,
    generated_by TEXT NOT NULL,
    generated_at TEXT NOT NULL,
    expires_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    subject_template TEXT NOT NULL,
    body_text TEXT,
    body_html TEXT,
    from_name TEXT,
    from_email TEXT,
    reply_to TEXT,
    cc_addresses TEXT,
    bcc_addresses TEXT,
    attachments TEXT,
    variables TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_campaigns (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    template_id TEXT NOT NULL,
    recipient_list_id TEXT,
    recipient_query TEXT,
    variables TEXT,
    total_recipients INTEGER DEFAULT 0,
    sent_count INTEGER DEFAULT 0,
    delivered_count INTEGER DEFAULT 0,
    opened_count INTEGER DEFAULT 0,
    clicked_count INTEGER DEFAULT 0,
    bounced_count INTEGER DEFAULT 0,
    unsubscribed_count INTEGER DEFAULT 0,
    status TEXT DEFAULT 'Draft',
    scheduled_at TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_logs (
    id TEXT PRIMARY KEY,
    campaign_id TEXT,
    template_id TEXT,
    recipient_email TEXT NOT NULL,
    recipient_name TEXT,
    subject TEXT NOT NULL,
    status TEXT DEFAULT 'Queued',
    sent_at TEXT,
    delivered_at TEXT,
    opened_at TEXT,
    clicked_at TEXT,
    bounced_at TEXT,
    bounce_reason TEXT,
    error_message TEXT,
    message_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS print_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    paper_size TEXT DEFAULT 'A4',
    orientation TEXT DEFAULT 'Portrait',
    margin_top_mm REAL DEFAULT 25.4,
    margin_bottom_mm REAL DEFAULT 25.4,
    margin_left_mm REAL DEFAULT 25.4,
    margin_right_mm REAL DEFAULT 25.4,
    header_template TEXT,
    footer_template TEXT,
    body_template TEXT NOT NULL,
    css_styles TEXT,
    variables TEXT,
    status TEXT DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    report_type TEXT NOT NULL,
    data_source TEXT NOT NULL,
    query TEXT,
    parameters TEXT,
    columns TEXT,
    groupings TEXT,
    filters TEXT,
    sort_order TEXT,
    chart_config TEXT,
    template_content TEXT,
    default_format TEXT DEFAULT 'PDF',
    schedule_id TEXT,
    status TEXT DEFAULT 'Active',
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Indexes for new tables
CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_status ON notifications(status);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);
CREATE INDEX IF NOT EXISTS idx_webhook_endpoints_status ON webhook_endpoints(status);
CREATE INDEX IF NOT EXISTS idx_webhook_deliveries_status ON webhook_deliveries(status);
CREATE INDEX IF NOT EXISTS idx_scheduled_jobs_status ON scheduled_jobs(status);
CREATE INDEX IF NOT EXISTS idx_scheduled_jobs_next_run ON scheduled_jobs(next_run_at);
CREATE INDEX IF NOT EXISTS idx_api_keys_prefix ON api_keys(key_prefix);
CREATE INDEX IF NOT EXISTS idx_api_key_usage_key_id ON api_key_usage(api_key_id);
CREATE INDEX IF NOT EXISTS idx_templates_type ON templates(template_type);
CREATE INDEX IF NOT EXISTS idx_templates_code ON templates(code);
