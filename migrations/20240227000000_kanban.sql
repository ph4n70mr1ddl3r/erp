-- Kanban Module Migration
-- Visual workflow management for lean manufacturing and project tracking

-- Kanban Boards
CREATE TABLE IF NOT EXISTS kanban_boards (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    board_type TEXT NOT NULL DEFAULT 'Generic',
    team_id TEXT,
    project_id TEXT,
    swimlane_type TEXT NOT NULL DEFAULT 'None',
    default_wip_limit INTEGER,
    allow_card_reordering INTEGER NOT NULL DEFAULT 1,
    show_card_count INTEGER NOT NULL DEFAULT 1,
    show_wip_limits INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Kanban Columns
CREATE TABLE IF NOT EXISTS kanban_columns (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL,
    name TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    wip_limit INTEGER,
    is_done_column INTEGER NOT NULL DEFAULT 0,
    is_backlog INTEGER NOT NULL DEFAULT 0,
    color TEXT,
    auto_assign_on_move TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES kanban_boards(id) ON DELETE CASCADE
);

-- Kanban Swimlanes
CREATE TABLE IF NOT EXISTS kanban_swimlanes (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL,
    name TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    color TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES kanban_boards(id) ON DELETE CASCADE
);

-- Kanban Cards
CREATE TABLE IF NOT EXISTS kanban_cards (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL,
    column_id TEXT NOT NULL,
    swimlane_id TEXT,
    card_type TEXT NOT NULL DEFAULT 'Task',
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT NOT NULL DEFAULT 'Medium',
    position INTEGER NOT NULL DEFAULT 0,
    assignee_ids TEXT NOT NULL DEFAULT '[]',
    reporter_id TEXT,
    due_date TEXT,
    start_date TEXT,
    completed_date TEXT,
    estimated_hours REAL,
    actual_hours REAL,
    story_points INTEGER,
    tags TEXT NOT NULL DEFAULT '[]',
    external_ref_type TEXT,
    external_ref_id TEXT,
    blocked INTEGER NOT NULL DEFAULT 0,
    blocked_reason TEXT,
    parent_card_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES kanban_boards(id) ON DELETE CASCADE,
    FOREIGN KEY (column_id) REFERENCES kanban_columns(id) ON DELETE CASCADE,
    FOREIGN KEY (swimlane_id) REFERENCES kanban_swimlanes(id) ON DELETE SET NULL,
    FOREIGN KEY (parent_card_id) REFERENCES kanban_cards(id) ON DELETE SET NULL
);

-- Kanban Card Comments
CREATE TABLE IF NOT EXISTS kanban_card_comments (
    id TEXT PRIMARY KEY,
    card_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    FOREIGN KEY (card_id) REFERENCES kanban_cards(id) ON DELETE CASCADE
);

-- Kanban Card Attachments
CREATE TABLE IF NOT EXISTS kanban_card_attachments (
    id TEXT PRIMARY KEY,
    card_id TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL DEFAULT 0,
    content_type TEXT NOT NULL,
    uploaded_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (card_id) REFERENCES kanban_cards(id) ON DELETE CASCADE
);

-- Kanban Card Checklists
CREATE TABLE IF NOT EXISTS kanban_card_checklists (
    id TEXT PRIMARY KEY,
    card_id TEXT NOT NULL,
    title TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (card_id) REFERENCES kanban_cards(id) ON DELETE CASCADE
);

-- Kanban Card Checklist Items
CREATE TABLE IF NOT EXISTS kanban_card_checklist_items (
    id TEXT PRIMARY KEY,
    checklist_id TEXT NOT NULL,
    content TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    completed INTEGER NOT NULL DEFAULT 0,
    completed_by TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (checklist_id) REFERENCES kanban_card_checklists(id) ON DELETE CASCADE
);

-- Kanban Card Moves (for tracking cycle time)
CREATE TABLE IF NOT EXISTS kanban_card_moves (
    id TEXT PRIMARY KEY,
    card_id TEXT NOT NULL,
    from_column_id TEXT NOT NULL,
    to_column_id TEXT NOT NULL,
    from_position INTEGER NOT NULL,
    to_position INTEGER NOT NULL,
    moved_by TEXT NOT NULL,
    moved_at TEXT NOT NULL,
    time_in_from_column_seconds INTEGER,
    FOREIGN KEY (card_id) REFERENCES kanban_cards(id) ON DELETE CASCADE
);

-- Kanban Card Labels
CREATE TABLE IF NOT EXISTS kanban_card_labels (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL,
    name TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#6B7280',
    created_at TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES kanban_boards(id) ON DELETE CASCADE
);

-- Kanban Card Label Assignments
CREATE TABLE IF NOT EXISTS kanban_card_label_assignments (
    id TEXT PRIMARY KEY,
    card_id TEXT NOT NULL,
    label_id TEXT NOT NULL,
    assigned_at TEXT NOT NULL,
    FOREIGN KEY (card_id) REFERENCES kanban_cards(id) ON DELETE CASCADE,
    FOREIGN KEY (label_id) REFERENCES kanban_card_labels(id) ON DELETE CASCADE,
    UNIQUE(card_id, label_id)
);

-- Kanban Activity Log
CREATE TABLE IF NOT EXISTS kanban_activity_logs (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL,
    card_id TEXT,
    action_type TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    description TEXT NOT NULL,
    old_value TEXT,
    new_value TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES kanban_boards(id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES kanban_cards(id) ON DELETE SET NULL
);

-- Kanban WIP Violations
CREATE TABLE IF NOT EXISTS kanban_wip_violations (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL,
    column_id TEXT NOT NULL,
    current_count INTEGER NOT NULL,
    wip_limit INTEGER NOT NULL,
    violated_at TEXT NOT NULL,
    resolved_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES kanban_boards(id) ON DELETE CASCADE,
    FOREIGN KEY (column_id) REFERENCES kanban_columns(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_kanban_boards_status ON kanban_boards(status);
CREATE INDEX IF NOT EXISTS idx_kanban_columns_board ON kanban_columns(board_id);
CREATE INDEX IF NOT EXISTS idx_kanban_cards_board ON kanban_cards(board_id);
CREATE INDEX IF NOT EXISTS idx_kanban_cards_column ON kanban_cards(column_id);
CREATE INDEX IF NOT EXISTS idx_kanban_cards_status ON kanban_cards(status);
CREATE INDEX IF NOT EXISTS idx_kanban_cards_blocked ON kanban_cards(blocked);
CREATE INDEX IF NOT EXISTS idx_kanban_cards_due_date ON kanban_cards(due_date);
CREATE INDEX IF NOT EXISTS idx_kanban_card_comments_card ON kanban_card_comments(card_id);
CREATE INDEX IF NOT EXISTS idx_kanban_card_moves_card ON kanban_card_moves(card_id);
CREATE INDEX IF NOT EXISTS idx_kanban_card_moves_moved_at ON kanban_card_moves(moved_at);
CREATE INDEX IF NOT EXISTS idx_kanban_activity_logs_board ON kanban_activity_logs(board_id);
CREATE INDEX IF NOT EXISTS idx_kanban_activity_logs_created ON kanban_activity_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_kanban_wip_violations_board ON kanban_wip_violations(board_id);
CREATE INDEX IF NOT EXISTS idx_kanban_wip_violations_resolved ON kanban_wip_violations(resolved_at);
