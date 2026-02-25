-- Report Scheduling Module
CREATE TABLE IF NOT EXISTS report_schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    report_type TEXT NOT NULL,
    report_config TEXT NOT NULL,
    frequency TEXT NOT NULL DEFAULT 'Daily',
    cron_expression TEXT,
    next_run_at TEXT,
    last_run_at TEXT,
    start_date TEXT,
    end_date TEXT,
    timezone TEXT NOT NULL DEFAULT 'UTC',
    format TEXT NOT NULL DEFAULT 'PDF',
    delivery_method TEXT NOT NULL DEFAULT 'Email',
    delivery_config TEXT,
    recipients TEXT NOT NULL,
    cc_recipients TEXT,
    bcc_recipients TEXT,
    email_subject TEXT,
    email_body TEXT,
    include_attachment INTEGER NOT NULL DEFAULT 1,
    compress_output INTEGER NOT NULL DEFAULT 0,
    compression_format TEXT,
    max_file_size_mb INTEGER,
    retry_on_failure INTEGER NOT NULL DEFAULT 1,
    max_retries INTEGER NOT NULL DEFAULT 3,
    retry_interval_minutes INTEGER NOT NULL DEFAULT 30,
    notify_on_success INTEGER NOT NULL DEFAULT 0,
    notify_on_failure INTEGER NOT NULL DEFAULT 1,
    notification_recipients TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    priority INTEGER NOT NULL DEFAULT 5,
    tags TEXT,
    owner_id TEXT NOT NULL,
    department_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS schedule_executions (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    execution_number INTEGER NOT NULL,
    started_at TEXT,
    completed_at TEXT,
    duration_seconds INTEGER,
    status TEXT NOT NULL DEFAULT 'Pending',
    report_url TEXT,
    file_path TEXT,
    file_size_bytes INTEGER,
    record_count INTEGER,
    error_message TEXT,
    error_stack TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    delivery_attempts INTEGER NOT NULL DEFAULT 0,
    delivery_status TEXT,
    delivery_error TEXT,
    delivered_at TEXT,
    delivery_details TEXT,
    parameters TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report_subscriptions (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    email TEXT NOT NULL,
    delivery_method TEXT NOT NULL DEFAULT 'Email',
    format TEXT,
    active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS schedule_audit (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    execution_id TEXT,
    action TEXT NOT NULL,
    old_values TEXT,
    new_values TEXT,
    performed_by TEXT NOT NULL,
    performed_at TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    notes TEXT
);

-- Chat/Messaging Module
CREATE TABLE IF NOT EXISTS chat_channels (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    channel_type TEXT NOT NULL DEFAULT 'Group',
    avatar_url TEXT,
    is_private INTEGER NOT NULL DEFAULT 0,
    is_archived INTEGER NOT NULL DEFAULT 0,
    owner_id TEXT NOT NULL,
    parent_channel_id TEXT,
    related_entity_type TEXT,
    related_entity_id TEXT,
    topic TEXT,
    slow_mode INTEGER NOT NULL DEFAULT 0,
    slow_mode_delay INTEGER,
    allow_mentions INTEGER NOT NULL DEFAULT 1,
    allow_reactions INTEGER NOT NULL DEFAULT 1,
    allow_threads INTEGER NOT NULL DEFAULT 1,
    auto_join INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS chat_messages (
    id TEXT PRIMARY KEY,
    channel_id TEXT NOT NULL,
    sender_id TEXT NOT NULL,
    parent_message_id TEXT,
    thread_id TEXT,
    message_type TEXT NOT NULL DEFAULT 'Text',
    content TEXT NOT NULL,
    formatted_content TEXT,
    attachments TEXT,
    mentions TEXT,
    reactions TEXT,
    reply_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Sent',
    edited_at TEXT,
    edited_by TEXT,
    deleted_at TEXT,
    deleted_by TEXT,
    pinned_at TEXT,
    pinned_by TEXT,
    starred_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS chat_memberships (
    id TEXT PRIMARY KEY,
    channel_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'Member',
    nickname TEXT,
    muted INTEGER NOT NULL DEFAULT 0,
    muted_until TEXT,
    notifications_enabled INTEGER NOT NULL DEFAULT 1,
    last_read_at TEXT,
    last_read_message_id TEXT,
    unread_count INTEGER NOT NULL DEFAULT 0,
    unread_mentions INTEGER NOT NULL DEFAULT 0,
    starred INTEGER NOT NULL DEFAULT 0,
    hidden INTEGER NOT NULL DEFAULT 0,
    joined_at TEXT NOT NULL,
    invited_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(channel_id, user_id)
);

CREATE TABLE IF NOT EXISTS direct_messages (
    id TEXT PRIMARY KEY,
    sender_id TEXT NOT NULL,
    recipient_id TEXT NOT NULL,
    message_type TEXT NOT NULL DEFAULT 'Text',
    content TEXT NOT NULL,
    formatted_content TEXT,
    attachments TEXT,
    status TEXT NOT NULL DEFAULT 'Sent',
    read_at TEXT,
    edited_at TEXT,
    deleted_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS message_reactions (
    id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    emoji TEXT NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE(message_id, user_id, emoji)
);

CREATE TABLE IF NOT EXISTS user_presence (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL DEFAULT 'Offline',
    status_message TEXT,
    last_seen_at TEXT,
    online INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Calendar Module
CREATE TABLE IF NOT EXISTS calendars (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    color TEXT NOT NULL DEFAULT '#3B82F6',
    owner_id TEXT NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0,
    is_public INTEGER NOT NULL DEFAULT 0,
    timezone TEXT NOT NULL DEFAULT 'UTC',
    working_hours_start TEXT,
    working_hours_end TEXT,
    working_days TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS calendar_events (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    location TEXT,
    is_virtual INTEGER NOT NULL DEFAULT 0,
    virtual_meeting_url TEXT,
    virtual_meeting_provider TEXT,
    event_type TEXT NOT NULL DEFAULT 'Meeting',
    status TEXT NOT NULL DEFAULT 'Confirmed',
    start_at TEXT NOT NULL,
    end_at TEXT NOT NULL,
    is_all_day INTEGER NOT NULL DEFAULT 0,
    timezone TEXT NOT NULL DEFAULT 'UTC',
    recurrence_pattern TEXT NOT NULL DEFAULT 'None',
    recurrence_rule TEXT,
    recurrence_end_date TEXT,
    recurrence_count INTEGER,
    parent_event_id TEXT,
    organizer_id TEXT NOT NULL,
    calendar_id TEXT,
    color TEXT,
    visibility TEXT NOT NULL DEFAULT 'Private',
    priority INTEGER NOT NULL DEFAULT 5,
    capacity INTEGER,
    current_attendees INTEGER NOT NULL DEFAULT 0,
    allow_registration INTEGER NOT NULL DEFAULT 0,
    registration_url TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS event_attendees (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL,
    user_id TEXT,
    email TEXT NOT NULL,
    name TEXT,
    status TEXT NOT NULL DEFAULT 'NeedsAction',
    role TEXT NOT NULL DEFAULT 'Required',
    response_message TEXT,
    responded_at TEXT,
    reminder_sent INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS event_reminders (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    reminder_type TEXT NOT NULL DEFAULT 'Notification',
    minutes_before INTEGER NOT NULL DEFAULT 15,
    sent_at TEXT,
    snoozed_until TEXT,
    active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS calendar_shares (
    id TEXT PRIMARY KEY,
    calendar_id TEXT NOT NULL,
    shared_with_user_id TEXT,
    shared_with_email TEXT,
    permission TEXT NOT NULL DEFAULT 'ViewOnly',
    share_token TEXT,
    expires_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS event_resources (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    resource_type TEXT NOT NULL DEFAULT 'Room',
    location TEXT,
    capacity INTEGER NOT NULL DEFAULT 1,
    email TEXT,
    calendar_id TEXT,
    available INTEGER NOT NULL DEFAULT 1,
    booking_enabled INTEGER NOT NULL DEFAULT 1,
    auto_accept INTEGER NOT NULL DEFAULT 0,
    approval_required INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS resource_bookings (
    id TEXT PRIMARY KEY,
    resource_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    booked_by TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    approved_by TEXT,
    approved_at TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Document Signing Module
CREATE TABLE IF NOT EXISTS signing_documents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    document_type TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    file_hash TEXT NOT NULL,
    pages INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Draft',
    envelope_id TEXT,
    sender_id TEXT NOT NULL,
    message TEXT,
    expires_at TEXT,
    completed_at TEXT,
    sent_at TEXT,
    viewed_at TEXT,
    reminder_count INTEGER NOT NULL DEFAULT 0,
    last_reminder_at TEXT,
    auto_remind INTEGER NOT NULL DEFAULT 1,
    remind_days INTEGER NOT NULL DEFAULT 3,
    sequential_signing INTEGER NOT NULL DEFAULT 0,
    current_signer_order INTEGER,
    final_signed_file TEXT,
    final_signed_at TEXT,
    audit_trail_file TEXT,
    certificate_of_completion TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS signers (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    user_id TEXT,
    order_index INTEGER NOT NULL DEFAULT 0,
    role TEXT NOT NULL DEFAULT 'Signer',
    status TEXT NOT NULL DEFAULT 'Pending',
    authentication_method TEXT NOT NULL DEFAULT 'Email',
    access_code TEXT,
    viewed_at TEXT,
    signed_at TEXT,
    declined_at TEXT,
    declined_reason TEXT,
    delegated_to TEXT,
    email_sent_at TEXT,
    reminder_sent_at TEXT,
    signature_ip TEXT,
    signature_user_agent TEXT,
    signature_location TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS signature_fields (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    signer_id TEXT NOT NULL,
    field_type TEXT NOT NULL DEFAULT 'Signature',
    page INTEGER NOT NULL DEFAULT 1,
    x_position REAL NOT NULL,
    y_position REAL NOT NULL,
    width REAL NOT NULL DEFAULT 100,
    height REAL NOT NULL DEFAULT 30,
    required INTEGER NOT NULL DEFAULT 1,
    value TEXT,
    signature_data TEXT,
    signature_type TEXT,
    signed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS signatures (
    id TEXT PRIMARY KEY,
    signer_id TEXT NOT NULL,
    signature_type TEXT NOT NULL,
    signature_data TEXT NOT NULL,
    initials_data TEXT,
    signed_at TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    geolocation TEXT,
    device_fingerprint TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS signing_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    template_type TEXT NOT NULL,
    document_path TEXT,
    field_config TEXT NOT NULL,
    signer_config TEXT NOT NULL,
    message_template TEXT,
    auto_expire_days INTEGER NOT NULL DEFAULT 30,
    remind_days INTEGER NOT NULL DEFAULT 3,
    sequential_signing INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    usage_count INTEGER NOT NULL DEFAULT 0,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS signing_audit (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    signer_id TEXT,
    action TEXT NOT NULL,
    details TEXT,
    ip_address TEXT,
    user_agent TEXT,
    geolocation TEXT,
    timestamp TEXT NOT NULL
);

-- Email Marketing Module
CREATE TABLE IF NOT EXISTS email_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    category TEXT,
    subject TEXT NOT NULL,
    preheader TEXT,
    html_body TEXT NOT NULL,
    text_body TEXT,
    variables TEXT,
    attachments TEXT,
    tracking_enabled INTEGER NOT NULL DEFAULT 1,
    track_opens INTEGER NOT NULL DEFAULT 1,
    track_clicks INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Active',
    version INTEGER NOT NULL DEFAULT 1,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_campaigns (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    template_id TEXT,
    subject TEXT NOT NULL,
    preheader TEXT,
    html_body TEXT NOT NULL,
    text_body TEXT,
    from_name TEXT NOT NULL,
    from_email TEXT NOT NULL,
    reply_to TEXT,
    list_ids TEXT NOT NULL,
    segment_rules TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    scheduled_at TEXT,
    sent_at TEXT,
    completed_at TEXT,
    total_recipients INTEGER NOT NULL DEFAULT 0,
    sent_count INTEGER NOT NULL DEFAULT 0,
    delivered_count INTEGER NOT NULL DEFAULT 0,
    opened_count INTEGER NOT NULL DEFAULT 0,
    clicked_count INTEGER NOT NULL DEFAULT 0,
    bounced_count INTEGER NOT NULL DEFAULT 0,
    unsubscribed_count INTEGER NOT NULL DEFAULT 0,
    complaint_count INTEGER NOT NULL DEFAULT 0,
    track_opens INTEGER NOT NULL DEFAULT 1,
    track_clicks INTEGER NOT NULL DEFAULT 1,
    attachments TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_lists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    list_type TEXT NOT NULL DEFAULT 'Static',
    subscriber_count INTEGER NOT NULL DEFAULT 0,
    active_count INTEGER NOT NULL DEFAULT 0,
    bounced_count INTEGER NOT NULL DEFAULT 0,
    unsubscribed_count INTEGER NOT NULL DEFAULT 0,
    double_optin INTEGER NOT NULL DEFAULT 0,
    welcome_email_id TEXT,
    unsubscribe_page TEXT,
    confirmation_page TEXT,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_subscribers (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT,
    last_name TEXT,
    company TEXT,
    phone TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    subscribed_at TEXT,
    unsubscribed_at TEXT,
    bounced_at TEXT,
    bounce_reason TEXT,
    confirmed_at TEXT,
    confirmation_token TEXT,
    preferences TEXT,
    custom_fields TEXT,
    source TEXT,
    ip_address TEXT,
    user_agent TEXT,
    last_email_at TEXT,
    open_count INTEGER NOT NULL DEFAULT 0,
    click_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS list_memberships (
    id TEXT PRIMARY KEY,
    list_id TEXT NOT NULL,
    subscriber_id TEXT NOT NULL,
    added_at TEXT NOT NULL,
    added_by TEXT,
    source TEXT,
    UNIQUE(list_id, subscriber_id)
);

CREATE TABLE IF NOT EXISTS email_queue (
    id TEXT PRIMARY KEY,
    campaign_id TEXT,
    subscriber_id TEXT,
    to_email TEXT NOT NULL,
    to_name TEXT,
    subject TEXT NOT NULL,
    html_body TEXT NOT NULL,
    text_body TEXT,
    from_email TEXT NOT NULL,
    from_name TEXT NOT NULL,
    reply_to TEXT,
    headers TEXT,
    attachments TEXT,
    variables TEXT,
    tracking_id TEXT,
    status TEXT NOT NULL DEFAULT 'Queued',
    priority INTEGER NOT NULL DEFAULT 5,
    scheduled_at TEXT,
    sent_at TEXT,
    delivered_at TEXT,
    opened_at TEXT,
    clicked_at TEXT,
    bounced_at TEXT,
    bounce_reason TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    next_retry_at TEXT,
    error_message TEXT,
    external_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_events (
    id TEXT PRIMARY KEY,
    queue_id TEXT,
    campaign_id TEXT,
    subscriber_id TEXT,
    event_type TEXT NOT NULL,
    email TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    url TEXT,
    details TEXT
);

CREATE TABLE IF NOT EXISTS email_suppressions (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    reason TEXT NOT NULL,
    source TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_report_schedules_owner ON report_schedules(owner_id);
CREATE INDEX IF NOT EXISTS idx_report_schedules_status ON report_schedules(status);
CREATE INDEX IF NOT EXISTS idx_report_schedules_next_run ON report_schedules(next_run_at);
CREATE INDEX IF NOT EXISTS idx_schedule_executions_schedule ON schedule_executions(schedule_id);
CREATE INDEX IF NOT EXISTS idx_chat_channels_owner ON chat_channels(owner_id);
CREATE INDEX IF NOT EXISTS idx_chat_messages_channel ON chat_messages(channel_id);
CREATE INDEX IF NOT EXISTS idx_chat_memberships_user ON chat_memberships(user_id);
CREATE INDEX IF NOT EXISTS idx_direct_messages_sender ON direct_messages(sender_id);
CREATE INDEX IF NOT EXISTS idx_direct_messages_recipient ON direct_messages(recipient_id);
CREATE INDEX IF NOT EXISTS idx_calendar_events_organizer ON calendar_events(organizer_id);
CREATE INDEX IF NOT EXISTS idx_calendar_events_calendar ON calendar_events(calendar_id);
CREATE INDEX IF NOT EXISTS idx_calendar_events_start ON calendar_events(start_at);
CREATE INDEX IF NOT EXISTS idx_event_attendees_event ON event_attendees(event_id);
CREATE INDEX IF NOT EXISTS idx_signing_documents_sender ON signing_documents(sender_id);
CREATE INDEX IF NOT EXISTS idx_signing_documents_status ON signing_documents(status);
CREATE INDEX IF NOT EXISTS idx_signers_document ON signers(document_id);
CREATE INDEX IF NOT EXISTS idx_signers_email ON signers(email);
CREATE INDEX IF NOT EXISTS idx_signature_fields_document ON signature_fields(document_id);
CREATE INDEX IF NOT EXISTS idx_email_campaigns_status ON email_campaigns(status);
CREATE INDEX IF NOT EXISTS idx_email_subscribers_email ON email_subscribers(email);
CREATE INDEX IF NOT EXISTS idx_email_queue_status ON email_queue(status);
CREATE INDEX IF NOT EXISTS idx_list_memberships_list ON list_memberships(list_id);
CREATE INDEX IF NOT EXISTS idx_list_memberships_subscriber ON list_memberships(subscriber_id);
