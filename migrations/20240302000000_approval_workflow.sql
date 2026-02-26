CREATE TABLE IF NOT EXISTS approval_workflows (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    document_type TEXT NOT NULL,
    approval_type TEXT NOT NULL DEFAULT 'Sequential',
    min_amount INTEGER,
    max_amount INTEGER,
    auto_approve_below INTEGER,
    escalation_hours INTEGER,
    notify_requester INTEGER NOT NULL DEFAULT 1,
    notify_approver INTEGER NOT NULL DEFAULT 1,
    allow_delegation INTEGER NOT NULL DEFAULT 1,
    allow_reassignment INTEGER NOT NULL DEFAULT 0,
    require_comments INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS approval_workflow_levels (
    id TEXT PRIMARY KEY,
    workflow_id TEXT NOT NULL,
    level_number INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    approver_type TEXT NOT NULL DEFAULT 'SpecificUser',
    approver_ids TEXT NOT NULL DEFAULT '[]',
    min_approvers INTEGER NOT NULL DEFAULT 1,
    skip_if_approved_above INTEGER NOT NULL DEFAULT 0,
    due_hours INTEGER,
    escalation_to TEXT,
    FOREIGN KEY (workflow_id) REFERENCES approval_workflows(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS approval_requests (
    id TEXT PRIMARY KEY,
    request_number TEXT NOT NULL UNIQUE,
    workflow_id TEXT NOT NULL,
    document_type TEXT NOT NULL,
    document_id TEXT NOT NULL,
    document_number TEXT NOT NULL,
    requested_by TEXT NOT NULL,
    requested_at TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Pending',
    current_level INTEGER,
    due_date TEXT,
    approved_at TEXT,
    approved_by TEXT,
    rejected_at TEXT,
    rejected_by TEXT,
    rejection_reason TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (workflow_id) REFERENCES approval_workflows(id)
);

CREATE TABLE IF NOT EXISTS approval_records (
    id TEXT PRIMARY KEY,
    request_id TEXT NOT NULL,
    level_number INTEGER NOT NULL,
    approver_id TEXT NOT NULL,
    action TEXT NOT NULL,
    comments TEXT,
    delegated_to TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (request_id) REFERENCES approval_requests(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS approval_delegations (
    id TEXT PRIMARY KEY,
    from_user_id TEXT NOT NULL,
    to_user_id TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    document_types TEXT NOT NULL DEFAULT '[]',
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_approval_workflows_document_type ON approval_workflows(document_type);
CREATE INDEX IF NOT EXISTS idx_approval_workflows_status ON approval_workflows(status);
CREATE INDEX IF NOT EXISTS idx_approval_workflow_levels_workflow ON approval_workflow_levels(workflow_id);
CREATE INDEX IF NOT EXISTS idx_approval_requests_status ON approval_requests(status);
CREATE INDEX IF NOT EXISTS idx_approval_requests_document ON approval_requests(document_type, document_id);
CREATE INDEX IF NOT EXISTS idx_approval_requests_requested_by ON approval_requests(requested_by);
CREATE INDEX IF NOT EXISTS idx_approval_records_request ON approval_records(request_id);
CREATE INDEX IF NOT EXISTS idx_approval_delegations_from ON approval_delegations(from_user_id);
CREATE INDEX IF NOT EXISTS idx_approval_delegations_to ON approval_delegations(to_user_id);
