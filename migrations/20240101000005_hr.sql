CREATE TABLE departments (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    parent_id TEXT REFERENCES departments(id),
    manager_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE positions (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    department_id TEXT REFERENCES departments(id),
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE employees (
    id TEXT PRIMARY KEY,
    employee_number TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    fax TEXT,
    website TEXT,
    street TEXT,
    city TEXT,
    state TEXT,
    postal_code TEXT,
    country TEXT,
    birth_date TEXT NOT NULL,
    hire_date TEXT NOT NULL,
    termination_date TEXT,
    department_id TEXT REFERENCES departments(id),
    position_id TEXT REFERENCES positions(id),
    manager_id TEXT REFERENCES employees(id),
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT,
    updated_by TEXT
);

CREATE TABLE attendance (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL REFERENCES employees(id),
    date TEXT NOT NULL,
    check_in TEXT,
    check_out TEXT,
    work_hours REAL NOT NULL DEFAULT 0,
    overtime_hours REAL NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Present',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(employee_id, date)
);

CREATE TABLE leave_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL UNIQUE,
    days_per_year INTEGER NOT NULL DEFAULT 0,
    carry_over INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active'
);

CREATE TABLE leave_requests (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL REFERENCES employees(id),
    leave_type TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    days REAL NOT NULL,
    reason TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    approved_by TEXT,
    approved_at TEXT
);

CREATE TABLE salary_structures (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL REFERENCES employees(id),
    base_salary INTEGER NOT NULL,
    currency TEXT DEFAULT 'USD',
    effective_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE allowances (
    id TEXT PRIMARY KEY,
    salary_structure_id TEXT NOT NULL REFERENCES salary_structures(id),
    name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    taxable INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE payroll (
    id TEXT PRIMARY KEY,
    employee_id TEXT NOT NULL REFERENCES employees(id),
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    base_salary INTEGER NOT NULL,
    overtime INTEGER NOT NULL DEFAULT 0,
    bonuses INTEGER NOT NULL DEFAULT 0,
    deductions INTEGER NOT NULL DEFAULT 0,
    net_salary INTEGER NOT NULL,
    currency TEXT DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    approved_by TEXT,
    approved_at TEXT
);
