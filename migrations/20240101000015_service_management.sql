CREATE TABLE service_tickets (
    id TEXT PRIMARY KEY,
    ticket_number TEXT NOT NULL UNIQUE,
    subject TEXT NOT NULL,
    description TEXT NOT NULL,
    customer_id TEXT,
    contact_id TEXT,
    assigned_to TEXT,
    team_id TEXT,
    priority TEXT NOT NULL DEFAULT '"Medium"',
    status TEXT NOT NULL DEFAULT '"New"',
    ticket_type TEXT NOT NULL DEFAULT '"Incident"',
    source TEXT NOT NULL DEFAULT '"WebPortal"',
    category_id TEXT,
    sla_id TEXT,
    due_date TEXT,
    resolved_at TEXT,
    closed_at TEXT,
    first_response_at TEXT,
    satisfaction_rating INTEGER,
    satisfaction_comment TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE ticket_comments (
    id TEXT PRIMARY KEY,
    ticket_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    author_type TEXT NOT NULL DEFAULT '"Agent"',
    body TEXT NOT NULL,
    is_internal INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES service_tickets(id)
);

CREATE TABLE ticket_attachments (
    id TEXT PRIMARY KEY,
    ticket_id TEXT NOT NULL,
    comment_id TEXT,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    content_type TEXT NOT NULL,
    uploaded_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES service_tickets(id)
);

CREATE TABLE slas (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    response_time_hours INTEGER NOT NULL,
    resolution_time_hours INTEGER NOT NULL,
    business_hours_only INTEGER NOT NULL DEFAULT 1,
    timezone TEXT NOT NULL DEFAULT 'UTC',
    escalation_rule_id TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE sla_policies (
    id TEXT PRIMARY KEY,
    sla_id TEXT NOT NULL,
    priority TEXT NOT NULL,
    response_time_hours INTEGER NOT NULL,
    resolution_time_hours INTEGER NOT NULL,
    FOREIGN KEY (sla_id) REFERENCES slas(id)
);

CREATE TABLE escalation_rules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    conditions TEXT NOT NULL,
    actions TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL
);

CREATE TABLE ticket_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT,
    sla_id TEXT,
    status TEXT NOT NULL DEFAULT '"Active"'
);

CREATE TABLE service_teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    manager_id TEXT,
    members TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL
);

CREATE TABLE knowledge_articles (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    summary TEXT,
    category_id TEXT,
    author_id TEXT NOT NULL,
    article_type TEXT NOT NULL DEFAULT '"KnowledgeBase"',
    status TEXT NOT NULL DEFAULT '"Draft"',
    view_count INTEGER NOT NULL DEFAULT 0,
    helpful_count INTEGER NOT NULL DEFAULT 0,
    not_helpful_count INTEGER NOT NULL DEFAULT 0,
    tags TEXT NOT NULL DEFAULT '[]',
    published_at TEXT,
    expires_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE knowledge_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    parent_id TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT '"Active"'
);

CREATE TABLE service_catalogs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    category_id TEXT,
    owner_id TEXT,
    approval_required INTEGER NOT NULL DEFAULT 0,
    sla_id TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL
);

CREATE TABLE service_catalog_items (
    id TEXT PRIMARY KEY,
    catalog_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    service_type TEXT NOT NULL DEFAULT '"Request"',
    price INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    delivery_time_hours INTEGER NOT NULL DEFAULT 0,
    form_schema TEXT,
    status TEXT NOT NULL DEFAULT '"Active"',
    FOREIGN KEY (catalog_id) REFERENCES service_catalogs(id)
);

CREATE TABLE service_contracts (
    id TEXT PRIMARY KEY,
    contract_number TEXT NOT NULL UNIQUE,
    customer_id TEXT NOT NULL,
    name TEXT NOT NULL,
    contract_type TEXT NOT NULL DEFAULT '"PerIncident"',
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    sla_id TEXT,
    max_tickets INTEGER,
    max_hours REAL,
    used_tickets INTEGER NOT NULL DEFAULT 0,
    used_hours REAL NOT NULL DEFAULT 0,
    value INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT '"Active"',
    created_at TEXT NOT NULL
);

CREATE TABLE problems (
    id TEXT PRIMARY KEY,
    problem_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    root_cause TEXT,
    workaround TEXT,
    impact TEXT NOT NULL DEFAULT '"Moderate"',
    urgency TEXT NOT NULL DEFAULT '"Medium"',
    priority TEXT NOT NULL DEFAULT '"Medium"',
    status TEXT NOT NULL DEFAULT '"New"',
    assigned_to TEXT,
    category_id TEXT,
    related_incidents TEXT NOT NULL DEFAULT '[]',
    resolution TEXT,
    resolved_at TEXT,
    closed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE change_requests (
    id TEXT PRIMARY KEY,
    change_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    reason TEXT NOT NULL,
    change_type TEXT NOT NULL DEFAULT '"Normal"',
    risk_level TEXT NOT NULL DEFAULT '"Medium"',
    impact_assessment TEXT,
    rollback_plan TEXT,
    status TEXT NOT NULL DEFAULT '"Draft"',
    requester_id TEXT NOT NULL,
    approver_id TEXT,
    assigned_to TEXT,
    planned_start TEXT,
    planned_end TEXT,
    actual_start TEXT,
    actual_end TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE change_tasks (
    id TEXT PRIMARY KEY,
    change_request_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    assigned_to TEXT,
    sequence INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT '"Pending"',
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (change_request_id) REFERENCES change_requests(id)
);

CREATE TABLE canned_responses (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    subject TEXT,
    body TEXT NOT NULL,
    category TEXT,
    tags TEXT NOT NULL DEFAULT '[]',
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE customer_contacts (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    title TEXT,
    is_primary INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE service_metrics (
    id TEXT PRIMARY KEY,
    metric_date TEXT NOT NULL,
    team_id TEXT,
    agent_id TEXT,
    tickets_created INTEGER NOT NULL DEFAULT 0,
    tickets_resolved INTEGER NOT NULL DEFAULT 0,
    tickets_open INTEGER NOT NULL DEFAULT 0,
    avg_first_response_hours REAL NOT NULL DEFAULT 0,
    avg_resolution_hours REAL NOT NULL DEFAULT 0,
    sla_breached INTEGER NOT NULL DEFAULT 0,
    customer_satisfaction REAL
);

CREATE INDEX idx_service_tickets_status ON service_tickets(status);
CREATE INDEX idx_service_tickets_assigned ON service_tickets(assigned_to);
CREATE INDEX idx_service_tickets_customer ON service_tickets(customer_id);
CREATE INDEX idx_service_tickets_created ON service_tickets(created_at);
CREATE INDEX idx_knowledge_articles_status ON knowledge_articles(status);
