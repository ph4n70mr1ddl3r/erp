-- POS Module Tables
CREATE TABLE IF NOT EXISTS pos_stores (
    id TEXT PRIMARY KEY,
    store_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    country TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    manager_id TEXT,
    warehouse_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    opening_time TEXT NOT NULL DEFAULT '09:00',
    closing_time TEXT NOT NULL DEFAULT '21:00',
    timezone TEXT NOT NULL DEFAULT 'UTC',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pos_terminals (
    id TEXT PRIMARY KEY,
    terminal_code TEXT NOT NULL UNIQUE,
    store_id TEXT NOT NULL,
    name TEXT NOT NULL,
    location TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    printer_name TEXT,
    receipt_printer TEXT,
    cash_drawer INTEGER NOT NULL DEFAULT 1,
    customer_display INTEGER NOT NULL DEFAULT 0,
    barcode_scanner INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS registers (
    id TEXT PRIMARY KEY,
    register_number TEXT NOT NULL UNIQUE,
    store_id TEXT NOT NULL,
    terminal_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Closed',
    opened_at TEXT,
    opened_by TEXT,
    closed_at TEXT,
    closed_by TEXT,
    opening_float INTEGER NOT NULL DEFAULT 0,
    closing_float INTEGER,
    expected_cash INTEGER,
    cash_variance INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pos_transactions (
    id TEXT PRIMARY KEY,
    transaction_number TEXT NOT NULL UNIQUE,
    store_id TEXT NOT NULL,
    terminal_id TEXT NOT NULL,
    register_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL DEFAULT 'Sale',
    customer_id TEXT,
    sales_rep_id TEXT,
    subtotal INTEGER NOT NULL DEFAULT 0,
    discount_amount INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    change_amount INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    original_transaction_id TEXT,
    notes TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pos_transaction_lines (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    product_id TEXT NOT NULL,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    discount_percent REAL NOT NULL DEFAULT 0,
    discount_amount INTEGER NOT NULL DEFAULT 0,
    tax_rate_id TEXT,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    line_total INTEGER NOT NULL,
    lot_number TEXT,
    serial_number TEXT
);

CREATE TABLE IF NOT EXISTS pos_transaction_payments (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    payment_method TEXT NOT NULL,
    amount INTEGER NOT NULL,
    reference TEXT,
    card_last_four TEXT,
    card_type TEXT,
    authorization_code TEXT,
    gift_card_id TEXT
);

CREATE TABLE IF NOT EXISTS gift_cards (
    id TEXT PRIMARY KEY,
    card_number TEXT NOT NULL UNIQUE,
    initial_amount INTEGER NOT NULL,
    current_balance INTEGER NOT NULL,
    sold_at TEXT NOT NULL,
    sold_at_store_id TEXT NOT NULL,
    customer_id TEXT,
    expires_at TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS loyalty_accounts (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    program_id TEXT,
    points_balance INTEGER NOT NULL DEFAULT 0,
    tier TEXT NOT NULL DEFAULT 'Bronze',
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- E-commerce Module Tables
CREATE TABLE IF NOT EXISTS ecommerce_platforms (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    platform_type TEXT NOT NULL,
    base_url TEXT NOT NULL,
    api_key TEXT,
    api_secret TEXT,
    access_token TEXT,
    webhook_secret TEXT,
    store_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    sync_direction TEXT NOT NULL DEFAULT 'Import',
    last_sync_at TEXT,
    sync_interval_minutes INTEGER NOT NULL DEFAULT 60,
    auto_sync INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS product_listings (
    id TEXT PRIMARY KEY,
    platform_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    external_product_id TEXT NOT NULL,
    external_variant_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    price INTEGER NOT NULL,
    compare_at_price INTEGER,
    quantity INTEGER NOT NULL DEFAULT 0,
    sku TEXT,
    barcode TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    visibility TEXT NOT NULL DEFAULT 'Visible',
    seo_title TEXT,
    seo_description TEXT,
    tags TEXT,
    category TEXT,
    images TEXT,
    sync_status TEXT NOT NULL DEFAULT 'Pending',
    last_sync_at TEXT,
    sync_error TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ecommerce_orders (
    id TEXT PRIMARY KEY,
    platform_id TEXT NOT NULL,
    external_order_id TEXT NOT NULL,
    order_number TEXT NOT NULL,
    customer_id TEXT,
    external_customer_id TEXT,
    sales_order_id TEXT,
    order_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    fulfillment_status TEXT NOT NULL DEFAULT 'Unfulfilled',
    payment_status TEXT NOT NULL DEFAULT 'Pending',
    subtotal INTEGER NOT NULL DEFAULT 0,
    shipping_amount INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    discount_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    billing_name TEXT NOT NULL,
    billing_address TEXT NOT NULL,
    billing_city TEXT NOT NULL,
    billing_state TEXT NOT NULL,
    billing_postal_code TEXT NOT NULL,
    billing_country TEXT NOT NULL,
    shipping_name TEXT NOT NULL,
    shipping_address TEXT NOT NULL,
    shipping_city TEXT NOT NULL,
    shipping_state TEXT NOT NULL,
    shipping_postal_code TEXT NOT NULL,
    shipping_country TEXT NOT NULL,
    shipping_method TEXT,
    tracking_number TEXT,
    customer_email TEXT,
    customer_phone TEXT,
    notes TEXT,
    sync_status TEXT NOT NULL DEFAULT 'Pending',
    imported_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ecommerce_order_lines (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL,
    external_line_id TEXT,
    product_id TEXT,
    listing_id TEXT,
    sku TEXT,
    title TEXT NOT NULL,
    variant_title TEXT,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    discount_amount INTEGER NOT NULL DEFAULT 0,
    line_total INTEGER NOT NULL,
    fulfillment_status TEXT NOT NULL DEFAULT 'Unfulfilled',
    quantity_fulfilled INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS webhook_events (
    id TEXT PRIMARY KEY,
    platform_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    external_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    processed INTEGER NOT NULL DEFAULT 0,
    processed_at TEXT,
    error TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS inventory_syncs (
    id TEXT PRIMARY KEY,
    platform_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    warehouse_id TEXT,
    external_product_id TEXT NOT NULL,
    external_variant_id TEXT,
    local_quantity INTEGER NOT NULL DEFAULT 0,
    remote_quantity INTEGER NOT NULL DEFAULT 0,
    reserved_quantity INTEGER NOT NULL DEFAULT 0,
    sync_status TEXT NOT NULL DEFAULT 'Pending',
    last_sync_at TEXT,
    sync_error TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Tax Module Tables
CREATE TABLE IF NOT EXISTS tax_jurisdictions (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    country_code TEXT NOT NULL,
    state_code TEXT,
    county TEXT,
    city TEXT,
    postal_code_from TEXT,
    postal_code_to TEXT,
    parent_jurisdiction_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    effective_from TEXT NOT NULL,
    effective_to TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tax_rates (
    id TEXT PRIMARY KEY,
    jurisdiction_id TEXT NOT NULL,
    tax_type TEXT NOT NULL DEFAULT 'SalesTax',
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    rate REAL NOT NULL,
    is_compound INTEGER NOT NULL DEFAULT 0,
    is_recoverable INTEGER NOT NULL DEFAULT 0,
    calculation_method TEXT NOT NULL DEFAULT 'Exclusive',
    status TEXT NOT NULL DEFAULT 'Active',
    effective_from TEXT NOT NULL,
    effective_to TEXT,
    priority INTEGER NOT NULL DEFAULT 1,
    min_amount INTEGER,
    max_amount INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tax_classes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    class_type TEXT NOT NULL DEFAULT 'Product',
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tax_exemptions (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    exemption_type TEXT NOT NULL DEFAULT 'Resale',
    certificate_number TEXT NOT NULL,
    jurisdiction_id TEXT,
    issue_date TEXT NOT NULL,
    expiry_date TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    document_url TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tax_transactions (
    id TEXT PRIMARY KEY,
    transaction_type TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    transaction_date TEXT NOT NULL,
    customer_id TEXT,
    jurisdiction_id TEXT NOT NULL,
    tax_rate_id TEXT NOT NULL,
    tax_class_id TEXT,
    tax_type TEXT NOT NULL DEFAULT 'SalesTax',
    taxable_amount INTEGER NOT NULL DEFAULT 0,
    tax_rate REAL NOT NULL,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    exemption_id TEXT,
    exempt_amount INTEGER NOT NULL DEFAULT 0,
    source TEXT NOT NULL DEFAULT 'Manual',
    external_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tax_reports (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    jurisdiction_id TEXT NOT NULL,
    report_period TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_sales INTEGER NOT NULL DEFAULT 0,
    taxable_sales INTEGER NOT NULL DEFAULT 0,
    exempt_sales INTEGER NOT NULL DEFAULT 0,
    tax_collected INTEGER NOT NULL DEFAULT 0,
    tax_paid INTEGER NOT NULL DEFAULT 0,
    tax_due INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Draft',
    filed_at TEXT,
    filed_by TEXT,
    filing_reference TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Reports Module Tables
CREATE TABLE IF NOT EXISTS report_definitions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL DEFAULT 'Financial',
    description TEXT,
    data_source TEXT NOT NULL,
    query_template TEXT NOT NULL,
    parameters TEXT NOT NULL DEFAULT '[]',
    columns TEXT NOT NULL DEFAULT '[]',
    default_format TEXT NOT NULL DEFAULT 'PDF',
    allowed_formats TEXT NOT NULL DEFAULT '["PDF","Excel","CSV"]',
    is_scheduled INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Active',
    created_by TEXT,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report_schedules (
    id TEXT PRIMARY KEY,
    report_definition_id TEXT NOT NULL,
    name TEXT NOT NULL,
    frequency TEXT NOT NULL DEFAULT 'Daily',
    cron_expression TEXT,
    start_date TEXT NOT NULL,
    end_date TEXT,
    next_run_at TEXT,
    last_run_at TEXT,
    parameters TEXT NOT NULL DEFAULT '{}',
    output_format TEXT NOT NULL DEFAULT 'PDF',
    delivery_methods TEXT NOT NULL DEFAULT '["Email"]',
    recipients TEXT NOT NULL DEFAULT '[]',
    email_subject TEXT,
    email_body TEXT,
    include_attachments INTEGER NOT NULL DEFAULT 1,
    ftp_host TEXT,
    ftp_path TEXT,
    webhook_url TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Active',
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report_executions (
    id TEXT PRIMARY KEY,
    report_definition_id TEXT NOT NULL,
    schedule_id TEXT,
    parameters TEXT NOT NULL DEFAULT '{}',
    format TEXT NOT NULL DEFAULT 'PDF',
    status TEXT NOT NULL DEFAULT 'Draft',
    started_at TEXT,
    completed_at TEXT,
    duration_ms INTEGER,
    row_count INTEGER NOT NULL DEFAULT 0,
    file_path TEXT,
    file_size_bytes INTEGER,
    error_message TEXT,
    delivery_status TEXT,
    delivered_at TEXT,
    executed_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS report_dashboards (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    layout TEXT NOT NULL DEFAULT 'grid',
    widgets TEXT NOT NULL DEFAULT '[]',
    is_default INTEGER NOT NULL DEFAULT 0,
    is_public INTEGER NOT NULL DEFAULT 1,
    refresh_interval_seconds INTEGER,
    status TEXT NOT NULL DEFAULT 'Active',
    owner_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS dashboard_widgets (
    id TEXT PRIMARY KEY,
    dashboard_id TEXT NOT NULL,
    report_definition_id TEXT,
    widget_type TEXT NOT NULL DEFAULT 'Table',
    title TEXT NOT NULL,
    position_x INTEGER NOT NULL DEFAULT 0,
    position_y INTEGER NOT NULL DEFAULT 0,
    width INTEGER NOT NULL DEFAULT 1,
    height INTEGER NOT NULL DEFAULT 1,
    parameters TEXT NOT NULL DEFAULT '{}',
    refresh_interval_seconds INTEGER,
    chart_config TEXT
);

-- Barcode Module Tables
CREATE TABLE IF NOT EXISTS barcodes (
    id TEXT PRIMARY KEY,
    barcode TEXT NOT NULL UNIQUE,
    barcode_type TEXT NOT NULL DEFAULT 'EAN13',
    entity_type TEXT NOT NULL DEFAULT 'Product',
    entity_id TEXT NOT NULL,
    definition_id TEXT,
    is_primary INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS barcode_definitions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    barcode_type TEXT NOT NULL DEFAULT 'Code128',
    entity_type TEXT NOT NULL DEFAULT 'Product',
    prefix TEXT,
    suffix TEXT,
    include_check_digit INTEGER NOT NULL DEFAULT 1,
    auto_generate INTEGER NOT NULL DEFAULT 1,
    sequence_start INTEGER NOT NULL DEFAULT 1,
    sequence_current INTEGER NOT NULL DEFAULT 0,
    padding_length INTEGER NOT NULL DEFAULT 6,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS barcode_printers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    printer_type TEXT NOT NULL DEFAULT 'Thermal',
    ip_address TEXT,
    port INTEGER,
    connection_type TEXT NOT NULL DEFAULT 'Network',
    dpi INTEGER NOT NULL DEFAULT 203,
    label_width_mm REAL NOT NULL DEFAULT 50.0,
    label_height_mm REAL NOT NULL DEFAULT 25.0,
    status TEXT NOT NULL DEFAULT 'Active',
    last_seen_at TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS barcode_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    width_mm REAL NOT NULL DEFAULT 50.0,
    height_mm REAL NOT NULL DEFAULT 25.0,
    margin_top_mm REAL NOT NULL DEFAULT 2.0,
    margin_bottom_mm REAL NOT NULL DEFAULT 2.0,
    margin_left_mm REAL NOT NULL DEFAULT 2.0,
    margin_right_mm REAL NOT NULL DEFAULT 2.0,
    barcode_type TEXT NOT NULL DEFAULT 'Code128',
    barcode_width_mm REAL NOT NULL DEFAULT 40.0,
    barcode_height_mm REAL NOT NULL DEFAULT 15.0,
    barcode_position_x REAL NOT NULL DEFAULT 5.0,
    barcode_position_y REAL NOT NULL DEFAULT 5.0,
    include_text INTEGER NOT NULL DEFAULT 1,
    text_font TEXT NOT NULL DEFAULT 'Arial',
    text_size INTEGER NOT NULL DEFAULT 10,
    text_position TEXT NOT NULL DEFAULT 'Below',
    elements TEXT NOT NULL DEFAULT '[]',
    zpl_template TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS barcode_print_jobs (
    id TEXT PRIMARY KEY,
    job_number TEXT NOT NULL UNIQUE,
    printer_id TEXT NOT NULL,
    template_id TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    printed_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_by TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS barcode_print_items (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    barcode_id TEXT NOT NULL,
    barcode TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    copies INTEGER NOT NULL DEFAULT 1,
    printed_copies INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    error_message TEXT
);

CREATE TABLE IF NOT EXISTS scan_events (
    id TEXT PRIMARY KEY,
    barcode TEXT NOT NULL,
    barcode_type TEXT NOT NULL DEFAULT 'EAN13',
    scanner_id TEXT NOT NULL,
    user_id TEXT,
    location_id TEXT,
    entity_type TEXT NOT NULL DEFAULT 'Product',
    entity_id TEXT,
    action TEXT NOT NULL DEFAULT 'Lookup',
    quantity INTEGER NOT NULL DEFAULT 1,
    reference_type TEXT,
    reference_id TEXT,
    scanned_at TEXT NOT NULL,
    metadata TEXT
);

CREATE TABLE IF NOT EXISTS barcode_scanners (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    scanner_type TEXT NOT NULL DEFAULT 'Handheld',
    connection_type TEXT NOT NULL DEFAULT 'USB',
    device_id TEXT,
    location_id TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    last_used_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_pos_transactions_store ON pos_transactions(store_id);
CREATE INDEX IF NOT EXISTS idx_pos_transactions_date ON pos_transactions(created_at);
CREATE INDEX IF NOT EXISTS idx_pos_transaction_lines ON pos_transaction_lines(transaction_id);
CREATE INDEX IF NOT EXISTS idx_pos_transaction_payments ON pos_transaction_payments(transaction_id);
CREATE INDEX IF NOT EXISTS idx_gift_cards_number ON gift_cards(card_number);

CREATE INDEX IF NOT EXISTS idx_ecommerce_orders_platform ON ecommerce_orders(platform_id);
CREATE INDEX IF NOT EXISTS idx_ecommerce_orders_status ON ecommerce_orders(status);
CREATE INDEX IF NOT EXISTS idx_ecommerce_order_lines ON ecommerce_order_lines(order_id);
CREATE INDEX IF NOT EXISTS idx_product_listings_product ON product_listings(product_id);
CREATE INDEX IF NOT EXISTS idx_webhook_events ON webhook_events(platform_id, processed);

CREATE INDEX IF NOT EXISTS idx_tax_rates_jurisdiction ON tax_rates(jurisdiction_id);
CREATE INDEX IF NOT EXISTS idx_tax_transactions_date ON tax_transactions(transaction_date);
CREATE INDEX IF NOT EXISTS idx_tax_exemptions_customer ON tax_exemptions(customer_id);

CREATE INDEX IF NOT EXISTS idx_report_executions_def ON report_executions(report_definition_id);
CREATE INDEX IF NOT EXISTS idx_report_schedules_next ON report_schedules(next_run_at);

CREATE INDEX IF NOT EXISTS idx_barcodes_barcode ON barcodes(barcode);
CREATE INDEX IF NOT EXISTS idx_barcodes_entity ON barcodes(entity_type, entity_id);
CREATE INDEX IF NOT EXISTS idx_scan_events_barcode ON scan_events(barcode);
CREATE INDEX IF NOT EXISTS idx_scan_events_date ON scan_events(scanned_at);
