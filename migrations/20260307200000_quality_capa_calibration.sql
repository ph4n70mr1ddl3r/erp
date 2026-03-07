-- CAPA Management
CREATE TABLE IF NOT EXISTS quality_capas (
    id TEXT PRIMARY KEY,
    capa_number TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_id TEXT,
    description TEXT NOT NULL,
    priority TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    initiator_id TEXT NOT NULL,
    owner_id TEXT,
    root_cause_analysis TEXT,
    action_plan TEXT,
    verification_plan TEXT,
    effectiveness_criteria TEXT,
    target_completion_date TEXT,
    effectiveness_result INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS quality_capa_actions (
    id TEXT PRIMARY KEY,
    capa_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    description TEXT NOT NULL,
    assigned_to TEXT,
    due_date TEXT NOT NULL,
    completed_at TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    evidence TEXT,
    FOREIGN KEY (capa_id) REFERENCES quality_capas(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_capa_status ON quality_capas(status);
CREATE INDEX IF NOT EXISTS idx_capa_priority ON quality_capas(priority);
CREATE INDEX IF NOT EXISTS idx_capa_actions_capa ON quality_capa_actions(capa_id);

-- Calibration Management
CREATE TABLE IF NOT EXISTS calibration_devices (
    id TEXT PRIMARY KEY,
    device_number TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    manufacturer TEXT,
    model TEXT,
    serial_number TEXT,
    location TEXT,
    calibration_frequency_days INTEGER NOT NULL,
    last_calibration_date TEXT,
    next_calibration_date TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS calibration_records (
    id TEXT PRIMARY KEY,
    record_number TEXT NOT NULL UNIQUE,
    device_id TEXT NOT NULL,
    calibration_date TEXT NOT NULL,
    calibrated_by TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    certificate_number TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (device_id) REFERENCES calibration_devices(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS calibration_readings (
    id TEXT PRIMARY KEY,
    record_id TEXT NOT NULL,
    parameter TEXT NOT NULL,
    reference_value REAL NOT NULL,
    actual_value REAL NOT NULL,
    tolerance_min REAL NOT NULL,
    tolerance_max REAL NOT NULL,
    pass_fail INTEGER NOT NULL,
    uom TEXT NOT NULL,
    FOREIGN KEY (record_id) REFERENCES calibration_records(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_calibration_devices_status ON calibration_devices(status);
CREATE INDEX IF NOT EXISTS idx_calibration_records_device ON calibration_records(device_id);
CREATE INDEX IF NOT EXISTS idx_calibration_readings_record ON calibration_readings(record_id);
