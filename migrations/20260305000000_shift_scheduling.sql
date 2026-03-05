CREATE TABLE IF NOT EXISTS shifts (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    break_minutes INTEGER NOT NULL DEFAULT 30,
    grace_period_minutes INTEGER NOT NULL DEFAULT 15,
    color_code TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS schedules (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    department_id TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (department_id) REFERENCES departments(id)
);

CREATE TABLE IF NOT EXISTS shift_assignments (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    shift_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    assignment_date TEXT NOT NULL,
    actual_start_time TEXT,
    actual_end_time TEXT,
    overtime_minutes INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    status TEXT NOT NULL DEFAULT 'Scheduled',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (schedule_id) REFERENCES schedules(id) ON DELETE CASCADE,
    FOREIGN KEY (shift_id) REFERENCES shifts(id),
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);

CREATE TABLE IF NOT EXISTS schedule_entries (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    day_of_week INTEGER NOT NULL,
    shift_id TEXT NOT NULL,
    employee_id TEXT NOT NULL,
    is_recurring INTEGER NOT NULL DEFAULT 1,
    effective_from TEXT,
    effective_to TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (schedule_id) REFERENCES schedules(id) ON DELETE CASCADE,
    FOREIGN KEY (shift_id) REFERENCES shifts(id),
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);

CREATE TABLE IF NOT EXISTS shift_swap_requests (
    id TEXT PRIMARY KEY,
    schedule_id TEXT NOT NULL,
    requesting_employee_id TEXT NOT NULL,
    target_employee_id TEXT NOT NULL,
    request_date TEXT NOT NULL,
    reason TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (schedule_id) REFERENCES schedules(id) ON DELETE CASCADE,
    FOREIGN KEY (requesting_employee_id) REFERENCES employees(id),
    FOREIGN KEY (target_employee_id) REFERENCES employees(id),
    FOREIGN KEY (approved_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_shift_assignments_schedule ON shift_assignments(schedule_id);
CREATE INDEX IF NOT EXISTS idx_shift_assignments_employee ON shift_assignments(employee_id);
CREATE INDEX IF NOT EXISTS idx_shift_assignments_date ON shift_assignments(assignment_date);
CREATE INDEX IF NOT EXISTS idx_schedules_department ON schedules(department_id);
CREATE INDEX IF NOT EXISTS idx_schedules_status ON schedules(status);
CREATE INDEX IF NOT EXISTS idx_shifts_status ON shifts(status);
