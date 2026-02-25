CREATE TABLE IF NOT EXISTS bi_kpis (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    kpi_type TEXT NOT NULL DEFAULT 'Counter',
    aggregation TEXT NOT NULL DEFAULT 'Sum',
    data_source TEXT NOT NULL,
    query TEXT,
    target_value REAL,
    warning_threshold REAL,
    critical_threshold REAL,
    unit TEXT,
    refresh_interval_seconds INTEGER NOT NULL DEFAULT 300,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS bi_kpi_values (
    id TEXT PRIMARY KEY,
    kpi_id TEXT NOT NULL,
    value REAL NOT NULL,
    previous_value REAL,
    change_percent REAL,
    trend TEXT,
    recorded_at TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (kpi_id) REFERENCES bi_kpis(id)
);

CREATE INDEX IF NOT EXISTS idx_bi_kpi_values_kpi_id ON bi_kpi_values(kpi_id);
CREATE INDEX IF NOT EXISTS idx_bi_kpi_values_recorded_at ON bi_kpi_values(recorded_at);

CREATE TABLE IF NOT EXISTS bi_dashboards (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    owner_id TEXT NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0,
    is_public INTEGER NOT NULL DEFAULT 0,
    layout_config TEXT NOT NULL,
    refresh_interval_seconds INTEGER NOT NULL DEFAULT 300,
    filters TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS bi_dashboard_widgets (
    id TEXT PRIMARY KEY,
    dashboard_id TEXT NOT NULL,
    kpi_id TEXT,
    widget_type TEXT NOT NULL DEFAULT 'Number',
    title TEXT NOT NULL,
    position_x INTEGER NOT NULL DEFAULT 0,
    position_y INTEGER NOT NULL DEFAULT 0,
    width INTEGER NOT NULL DEFAULT 4,
    height INTEGER NOT NULL DEFAULT 3,
    config TEXT NOT NULL,
    data_source TEXT,
    custom_query TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (dashboard_id) REFERENCES bi_dashboards(id),
    FOREIGN KEY (kpi_id) REFERENCES bi_kpis(id)
);

CREATE INDEX IF NOT EXISTS idx_bi_widgets_dashboard_id ON bi_dashboard_widgets(dashboard_id);

CREATE TABLE IF NOT EXISTS bi_reports (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    query TEXT NOT NULL,
    parameters TEXT,
    columns TEXT NOT NULL,
    chart_config TEXT,
    is_scheduled INTEGER NOT NULL DEFAULT 0,
    schedule_cron TEXT,
    last_run_at TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS bi_report_executions (
    id TEXT PRIMARY KEY,
    report_id TEXT NOT NULL,
    executed_by TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    status TEXT NOT NULL DEFAULT 'running',
    parameters_used TEXT,
    row_count INTEGER,
    result_data TEXT,
    export_url TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (report_id) REFERENCES bi_reports(id)
);

CREATE INDEX IF NOT EXISTS idx_bi_report_executions_report_id ON bi_report_executions(report_id);

CREATE TABLE IF NOT EXISTS bi_data_cubes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    source_table TEXT NOT NULL,
    dimensions TEXT NOT NULL,
    measures TEXT NOT NULL,
    aggregations TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_refreshed TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS bi_alert_rules (
    id TEXT PRIMARY KEY,
    kpi_id TEXT NOT NULL,
    name TEXT NOT NULL,
    condition TEXT NOT NULL,
    threshold REAL NOT NULL,
    severity TEXT NOT NULL DEFAULT 'warning',
    notification_channels TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    cooldown_minutes INTEGER NOT NULL DEFAULT 60,
    last_triggered TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (kpi_id) REFERENCES bi_kpis(id)
);

CREATE TABLE IF NOT EXISTS bi_scorecards (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    perspective TEXT NOT NULL,
    objectives TEXT NOT NULL,
    owner_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    overall_score REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS i18n_locales (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    native_name TEXT NOT NULL,
    language_code TEXT NOT NULL,
    country_code TEXT,
    is_rtl INTEGER NOT NULL DEFAULT 0,
    date_format TEXT NOT NULL DEFAULT 'YYYY-MM-DD',
    time_format TEXT NOT NULL DEFAULT 'HH:mm:ss',
    number_format TEXT NOT NULL DEFAULT '#,##0.00',
    currency_symbol TEXT NOT NULL DEFAULT '$',
    currency_position TEXT NOT NULL DEFAULT 'before',
    decimal_separator TEXT NOT NULL DEFAULT '.',
    thousand_separator TEXT NOT NULL DEFAULT ',',
    status TEXT NOT NULL DEFAULT 'Active',
    is_default INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT OR IGNORE INTO i18n_locales (id, code, name, native_name, language_code, is_default, created_at, updated_at)
VALUES ('en-default', 'en-US', 'English (US)', 'English (US)', 'en', 1, datetime('now'), datetime('now'));

CREATE TABLE IF NOT EXISTS i18n_translations (
    id TEXT PRIMARY KEY,
    locale_code TEXT NOT NULL,
    namespace TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    plural_form TEXT,
    context TEXT,
    is_approved INTEGER NOT NULL DEFAULT 0,
    translated_by TEXT,
    reviewed_by TEXT,
    reviewed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(locale_code, namespace, key, COALESCE(plural_form, ''), COALESCE(context, ''))
);

CREATE INDEX IF NOT EXISTS idx_i18n_translations_locale ON i18n_translations(locale_code);
CREATE INDEX IF NOT EXISTS idx_i18n_translations_namespace ON i18n_translations(namespace);

CREATE TABLE IF NOT EXISTS i18n_namespaces (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    module TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS i18n_user_preferences (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL UNIQUE,
    locale_code TEXT NOT NULL,
    timezone TEXT NOT NULL DEFAULT 'UTC',
    date_format_override TEXT,
    time_format_override TEXT,
    number_format_override TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS i18n_import_jobs (
    id TEXT PRIMARY KEY,
    locale_code TEXT NOT NULL,
    namespace TEXT NOT NULL,
    file_url TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    total_keys INTEGER NOT NULL DEFAULT 0,
    imported_keys INTEGER NOT NULL DEFAULT 0,
    skipped_keys INTEGER NOT NULL DEFAULT 0,
    error_count INTEGER NOT NULL DEFAULT 0,
    errors TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS i18n_export_jobs (
    id TEXT PRIMARY KEY,
    locale_code TEXT,
    namespace TEXT,
    format TEXT NOT NULL DEFAULT 'json',
    file_url TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    total_keys INTEGER NOT NULL DEFAULT 0,
    started_at TEXT,
    completed_at TEXT,
    expires_at TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS i18n_missing_translations (
    id TEXT PRIMARY KEY,
    locale_code TEXT NOT NULL,
    namespace TEXT NOT NULL,
    key TEXT NOT NULL,
    first_seen_at TEXT NOT NULL,
    usage_count INTEGER NOT NULL DEFAULT 1,
    last_used_at TEXT,
    priority TEXT NOT NULL DEFAULT 'medium',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(locale_code, namespace, key)
);

CREATE TABLE IF NOT EXISTS push_devices (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    device_token TEXT NOT NULL,
    platform TEXT NOT NULL DEFAULT 'Web',
    device_name TEXT,
    device_model TEXT,
    os_version TEXT,
    app_version TEXT,
    language TEXT,
    timezone TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_used_at TEXT,
    push_enabled INTEGER NOT NULL DEFAULT 1,
    badge_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(user_id, device_token)
);

CREATE INDEX IF NOT EXISTS idx_push_devices_user ON push_devices(user_id);

CREATE TABLE IF NOT EXISTS push_notifications (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    icon TEXT,
    image TEXT,
    sound TEXT,
    badge INTEGER,
    priority TEXT NOT NULL DEFAULT 'Normal',
    data TEXT,
    action_url TEXT,
    category TEXT,
    ttl_seconds INTEGER,
    scheduled_at TEXT,
    sent_at TEXT,
    expires_at TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_push_notifications_status ON push_notifications(status);

CREATE TABLE IF NOT EXISTS push_notification_recipients (
    id TEXT PRIMARY KEY,
    notification_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    device_id TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    sent_at TEXT,
    delivered_at TEXT,
    opened_at TEXT,
    error_message TEXT,
    external_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (notification_id) REFERENCES push_notifications(id)
);

CREATE INDEX IF NOT EXISTS idx_push_recipients_notification ON push_notification_recipients(notification_id);
CREATE INDEX IF NOT EXISTS idx_push_recipients_user ON push_notification_recipients(user_id);

CREATE TABLE IF NOT EXISTS push_templates (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    title_template TEXT NOT NULL,
    body_template TEXT NOT NULL,
    default_data TEXT,
    category TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'Normal',
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS push_campaigns (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    template_id TEXT,
    title_override TEXT,
    body_override TEXT,
    data TEXT,
    target_audience TEXT NOT NULL,
    scheduled_at TEXT,
    started_at TEXT,
    completed_at TEXT,
    status TEXT NOT NULL DEFAULT 'draft',
    total_recipients INTEGER NOT NULL DEFAULT 0,
    sent_count INTEGER NOT NULL DEFAULT 0,
    delivered_count INTEGER NOT NULL DEFAULT 0,
    opened_count INTEGER NOT NULL DEFAULT 0,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (template_id) REFERENCES push_templates(id)
);

CREATE TABLE IF NOT EXISTS push_preferences (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    category TEXT NOT NULL,
    push_enabled INTEGER NOT NULL DEFAULT 1,
    email_enabled INTEGER NOT NULL DEFAULT 1,
    sms_enabled INTEGER NOT NULL DEFAULT 0,
    in_app_enabled INTEGER NOT NULL DEFAULT 1,
    quiet_hours_start TEXT,
    quiet_hours_end TEXT,
    quiet_hours_timezone TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(user_id, category)
);

CREATE TABLE IF NOT EXISTS push_providers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    config TEXT NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    daily_limit INTEGER,
    daily_sent INTEGER NOT NULL DEFAULT 0,
    last_reset TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS bpm_process_definitions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Draft',
    bpmn_xml TEXT,
    diagram_data TEXT,
    variables TEXT,
    forms TEXT,
    owner_id TEXT NOT NULL,
    published_at TEXT,
    published_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_bpm_definitions_status ON bpm_process_definitions(status);
CREATE INDEX IF NOT EXISTS idx_bpm_definitions_category ON bpm_process_definitions(category);

CREATE TABLE IF NOT EXISTS bpm_process_nodes (
    id TEXT PRIMARY KEY,
    process_definition_id TEXT NOT NULL,
    node_id TEXT NOT NULL,
    name TEXT NOT NULL,
    task_type TEXT NOT NULL DEFAULT 'UserTask',
    gateway_type TEXT,
    config TEXT,
    assignee_expression TEXT,
    candidate_groups TEXT,
    form_key TEXT,
    script TEXT,
    service_name TEXT,
    position_x INTEGER NOT NULL DEFAULT 0,
    position_y INTEGER NOT NULL DEFAULT 0,
    incoming_flows TEXT,
    outgoing_flows TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_definition_id) REFERENCES bpm_process_definitions(id),
    UNIQUE(process_definition_id, node_id)
);

CREATE INDEX IF NOT EXISTS idx_bpm_nodes_definition ON bpm_process_nodes(process_definition_id);

CREATE TABLE IF NOT EXISTS bpm_process_flows (
    id TEXT PRIMARY KEY,
    process_definition_id TEXT NOT NULL,
    flow_id TEXT NOT NULL,
    name TEXT,
    source_node_id TEXT NOT NULL,
    target_node_id TEXT NOT NULL,
    condition_expression TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_definition_id) REFERENCES bpm_process_definitions(id),
    UNIQUE(process_definition_id, flow_id)
);

CREATE INDEX IF NOT EXISTS idx_bpm_flows_definition ON bpm_process_flows(process_definition_id);

CREATE TABLE IF NOT EXISTS bpm_process_instances (
    id TEXT PRIMARY KEY,
    process_definition_id TEXT NOT NULL,
    process_definition_version INTEGER NOT NULL,
    business_key TEXT,
    status TEXT NOT NULL DEFAULT 'Running',
    variables TEXT,
    started_by TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    current_node_id TEXT,
    parent_instance_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_definition_id) REFERENCES bpm_process_definitions(id)
);

CREATE INDEX IF NOT EXISTS idx_bpm_instances_status ON bpm_process_instances(status);
CREATE INDEX IF NOT EXISTS idx_bpm_instances_definition ON bpm_process_instances(process_definition_id);
CREATE INDEX IF NOT EXISTS idx_bpm_instances_business_key ON bpm_process_instances(business_key);

CREATE TABLE IF NOT EXISTS bpm_process_tasks (
    id TEXT PRIMARY KEY,
    process_instance_id TEXT NOT NULL,
    process_definition_id TEXT NOT NULL,
    node_id TEXT NOT NULL,
    name TEXT NOT NULL,
    task_type TEXT NOT NULL DEFAULT 'UserTask',
    status TEXT NOT NULL DEFAULT 'Created',
    assignee_id TEXT,
    candidate_users TEXT,
    candidate_groups TEXT,
    form_key TEXT,
    form_data TEXT,
    variables TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    due_date TEXT,
    created_at TEXT NOT NULL,
    claimed_at TEXT,
    completed_at TEXT,
    completed_by TEXT,
    outcome TEXT,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_instance_id) REFERENCES bpm_process_instances(id),
    FOREIGN KEY (process_definition_id) REFERENCES bpm_process_definitions(id)
);

CREATE INDEX IF NOT EXISTS idx_bpm_tasks_instance ON bpm_process_tasks(process_instance_id);
CREATE INDEX IF NOT EXISTS idx_bpm_tasks_assignee ON bpm_process_tasks(assignee_id);
CREATE INDEX IF NOT EXISTS idx_bpm_tasks_status ON bpm_process_tasks(status);

CREATE TABLE IF NOT EXISTS bpm_process_events (
    id TEXT PRIMARY KEY,
    process_instance_id TEXT NOT NULL,
    task_id TEXT,
    event_type TEXT NOT NULL,
    event_data TEXT,
    occurred_at TEXT NOT NULL,
    user_id TEXT,
    details TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_instance_id) REFERENCES bpm_process_instances(id)
);

CREATE INDEX IF NOT EXISTS idx_bpm_events_instance ON bpm_process_events(process_instance_id);

CREATE TABLE IF NOT EXISTS bpm_sla_definitions (
    id TEXT PRIMARY KEY,
    process_definition_id TEXT,
    node_id TEXT,
    name TEXT NOT NULL,
    target_duration_minutes INTEGER NOT NULL,
    warning_threshold_minutes INTEGER,
    escalation_actions TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_definition_id) REFERENCES bpm_process_definitions(id)
);

CREATE TABLE IF NOT EXISTS bpm_process_metrics (
    id TEXT PRIMARY KEY,
    process_definition_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    instances_started INTEGER NOT NULL DEFAULT 0,
    instances_completed INTEGER NOT NULL DEFAULT 0,
    instances_cancelled INTEGER NOT NULL DEFAULT 0,
    avg_duration_minutes REAL,
    min_duration_minutes INTEGER,
    max_duration_minutes INTEGER,
    tasks_completed INTEGER NOT NULL DEFAULT 0,
    avg_task_duration_minutes REAL,
    sla_breaches INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (process_definition_id) REFERENCES bpm_process_definitions(id)
);
