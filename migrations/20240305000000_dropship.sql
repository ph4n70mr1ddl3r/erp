-- Drop Shipping Module Migration
-- Enables vendors to ship directly to customers

-- Vendor Drop Ship Settings
CREATE TABLE IF NOT EXISTS vendor_dropship_settings (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL UNIQUE,
    enabled INTEGER NOT NULL DEFAULT 0,
    tier TEXT NOT NULL DEFAULT 'Standard',
    auto_confirm INTEGER NOT NULL DEFAULT 0,
    require_approval INTEGER NOT NULL DEFAULT 0,
    min_order_value INTEGER NOT NULL DEFAULT 0,
    max_order_value INTEGER,
    processing_time_days INTEGER NOT NULL DEFAULT 2,
    shipping_carrier TEXT,
    shipping_method TEXT,
    free_shipping_threshold INTEGER,
    handling_fee INTEGER NOT NULL DEFAULT 0,
    allow_partial_shipment INTEGER NOT NULL DEFAULT 1,
    return_policy_days INTEGER NOT NULL DEFAULT 30,
    notification_email TEXT,
    api_endpoint TEXT,
    api_key TEXT,
    sync_inventory INTEGER NOT NULL DEFAULT 0,
    inventory_sync_hours INTEGER NOT NULL DEFAULT 24,
    product_feed_url TEXT,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Drop Ship Orders
CREATE TABLE IF NOT EXISTS drop_ship_orders (
    id TEXT PRIMARY KEY,
    order_number TEXT NOT NULL UNIQUE,
    sales_order_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    purchase_order_id TEXT,
    ship_to_name TEXT NOT NULL,
    ship_to_company TEXT,
    ship_to_address TEXT NOT NULL,
    ship_to_city TEXT NOT NULL,
    ship_to_state TEXT,
    ship_to_postal_code TEXT NOT NULL,
    ship_to_country TEXT NOT NULL,
    ship_to_phone TEXT,
    ship_to_email TEXT,
    subtotal INTEGER NOT NULL DEFAULT 0,
    shipping_cost INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Pending',
    vendor_confirmation_number TEXT,
    expected_ship_date TEXT,
    actual_ship_date TEXT,
    expected_delivery_date TEXT,
    actual_delivery_date TEXT,
    notes TEXT,
    internal_notes TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    created_by TEXT,
    approved_by TEXT,
    approved_at TEXT,
    sent_to_vendor_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    updated_by TEXT
);

-- Drop Ship Order Lines
CREATE TABLE IF NOT EXISTS drop_ship_order_lines (
    id TEXT PRIMARY KEY,
    drop_ship_order_id TEXT NOT NULL,
    sales_order_line_id TEXT,
    product_id TEXT NOT NULL,
    vendor_sku TEXT,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    quantity_shipped INTEGER NOT NULL DEFAULT 0,
    quantity_cancelled INTEGER NOT NULL DEFAULT 0,
    unit_price INTEGER NOT NULL,
    line_total INTEGER NOT NULL,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'Pending',
    FOREIGN KEY (drop_ship_order_id) REFERENCES drop_ship_orders(id)
);

-- Drop Ship Purchase Orders (to vendors)
CREATE TABLE IF NOT EXISTS drop_ship_purchase_orders (
    id TEXT PRIMARY KEY,
    po_number TEXT NOT NULL UNIQUE,
    vendor_id TEXT NOT NULL,
    drop_ship_order_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    ship_to_name TEXT NOT NULL,
    ship_to_address TEXT NOT NULL,
    ship_to_city TEXT NOT NULL,
    ship_to_state TEXT,
    ship_to_postal_code TEXT NOT NULL,
    ship_to_country TEXT NOT NULL,
    subtotal INTEGER NOT NULL DEFAULT 0,
    shipping_cost INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Draft',
    vendor_acknowledgment TEXT,
    vendor_acknowledged_at TEXT,
    expected_delivery TEXT,
    terms TEXT,
    notes TEXT,
    created_by TEXT,
    approved_by TEXT,
    approved_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (drop_ship_order_id) REFERENCES drop_ship_orders(id)
);

-- Drop Ship PO Lines
CREATE TABLE IF NOT EXISTS drop_ship_po_lines (
    id TEXT PRIMARY KEY,
    po_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    vendor_sku TEXT,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    quantity_received INTEGER NOT NULL DEFAULT 0,
    unit_cost INTEGER NOT NULL,
    line_total INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    FOREIGN KEY (po_id) REFERENCES drop_ship_purchase_orders(id)
);

-- Drop Ship Shipments
CREATE TABLE IF NOT EXISTS drop_ship_shipments (
    id TEXT PRIMARY KEY,
    shipment_number TEXT NOT NULL UNIQUE,
    drop_ship_order_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    carrier TEXT NOT NULL,
    carrier_service TEXT,
    tracking_number TEXT,
    tracking_url TEXT,
    shipping_label_url TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    ship_date TEXT,
    estimated_delivery TEXT,
    actual_delivery TEXT,
    weight REAL,
    weight_unit TEXT,
    dimensions TEXT,
    shipped_from_address TEXT,
    shipped_from_city TEXT,
    shipped_from_state TEXT,
    shipped_from_postal TEXT,
    shipped_from_country TEXT,
    signature_required INTEGER NOT NULL DEFAULT 0,
    signed_by TEXT,
    signed_at TEXT,
    delivery_notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (drop_ship_order_id) REFERENCES drop_ship_orders(id)
);

-- Drop Ship Shipment Lines
CREATE TABLE IF NOT EXISTS drop_ship_shipment_lines (
    id TEXT PRIMARY KEY,
    shipment_id TEXT NOT NULL,
    drop_ship_order_line_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    condition TEXT,
    notes TEXT,
    FOREIGN KEY (shipment_id) REFERENCES drop_ship_shipments(id),
    FOREIGN KEY (drop_ship_order_line_id) REFERENCES drop_ship_order_lines(id)
);

-- Drop Ship Tracking Events
CREATE TABLE IF NOT EXISTS drop_ship_tracking_events (
    id TEXT PRIMARY KEY,
    shipment_id TEXT NOT NULL,
    event_timestamp TEXT NOT NULL,
    status_code TEXT NOT NULL,
    status_description TEXT NOT NULL,
    location_city TEXT,
    location_state TEXT,
    location_country TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (shipment_id) REFERENCES drop_ship_shipments(id)
);

-- Drop Ship Inventory Feed
CREATE TABLE IF NOT EXISTS drop_ship_inventory_feeds (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    feed_url TEXT,
    feed_format TEXT NOT NULL DEFAULT 'CSV',
    last_sync_at TEXT,
    last_sync_status TEXT,
    products_count INTEGER NOT NULL DEFAULT 0,
    in_stock_count INTEGER NOT NULL DEFAULT 0,
    out_of_stock_count INTEGER NOT NULL DEFAULT 0,
    sync_enabled INTEGER NOT NULL DEFAULT 0,
    sync_interval_hours INTEGER NOT NULL DEFAULT 24,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Drop Ship Inventory Items
CREATE TABLE IF NOT EXISTS drop_ship_inventory_items (
    id TEXT PRIMARY KEY,
    feed_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    vendor_sku TEXT NOT NULL,
    quantity_available INTEGER NOT NULL DEFAULT 0,
    quantity_reserved INTEGER NOT NULL DEFAULT 0,
    reorder_point INTEGER,
    expected_restock_date TEXT,
    cost INTEGER NOT NULL DEFAULT 0,
    msrp INTEGER,
    map_price INTEGER,
    weight REAL,
    status TEXT NOT NULL DEFAULT 'Active',
    last_updated TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (feed_id) REFERENCES drop_ship_inventory_feeds(id)
);

-- Drop Ship Invoices
CREATE TABLE IF NOT EXISTS drop_ship_invoices (
    id TEXT PRIMARY KEY,
    invoice_number TEXT NOT NULL UNIQUE,
    vendor_id TEXT NOT NULL,
    drop_ship_order_id TEXT NOT NULL,
    purchase_order_id TEXT,
    invoice_date TEXT NOT NULL,
    due_date TEXT NOT NULL,
    subtotal INTEGER NOT NULL DEFAULT 0,
    shipping_cost INTEGER NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Draft',
    vendor_invoice_number TEXT,
    paid_amount INTEGER NOT NULL DEFAULT 0,
    paid_at TEXT,
    payment_reference TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (drop_ship_order_id) REFERENCES drop_ship_orders(id)
);

-- Drop Ship Invoice Lines
CREATE TABLE IF NOT EXISTS drop_ship_invoice_lines (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_cost INTEGER NOT NULL,
    line_total INTEGER NOT NULL,
    FOREIGN KEY (invoice_id) REFERENCES drop_ship_invoices(id)
);

-- Drop Ship Returns
CREATE TABLE IF NOT EXISTS drop_ship_returns (
    id TEXT PRIMARY KEY,
    return_number TEXT NOT NULL UNIQUE,
    drop_ship_order_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    return_type TEXT NOT NULL,
    reason TEXT NOT NULL,
    request_date TEXT NOT NULL,
    authorized_date TEXT,
    received_date TEXT,
    processed_date TEXT,
    total_amount INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'Requested',
    vendor_rma_number TEXT,
    refund_method TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (drop_ship_order_id) REFERENCES drop_ship_orders(id)
);

-- Drop Ship Return Lines
CREATE TABLE IF NOT EXISTS drop_ship_return_lines (
    id TEXT PRIMARY KEY,
    return_id TEXT NOT NULL,
    drop_ship_order_line_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    condition TEXT,
    disposition TEXT,
    refund_amount INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    FOREIGN KEY (return_id) REFERENCES drop_ship_returns(id),
    FOREIGN KEY (drop_ship_order_line_id) REFERENCES drop_ship_order_lines(id)
);

-- Drop Ship Performance Metrics
CREATE TABLE IF NOT EXISTS drop_ship_performance (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    total_orders INTEGER NOT NULL DEFAULT 0,
    orders_on_time INTEGER NOT NULL DEFAULT 0,
    orders_late INTEGER NOT NULL DEFAULT 0,
    orders_cancelled INTEGER NOT NULL DEFAULT 0,
    total_items INTEGER NOT NULL DEFAULT 0,
    items_shipped INTEGER NOT NULL DEFAULT 0,
    items_backordered INTEGER NOT NULL DEFAULT 0,
    avg_processing_time_hours REAL NOT NULL DEFAULT 0,
    avg_shipping_time_hours REAL NOT NULL DEFAULT 0,
    on_time_delivery_rate REAL NOT NULL DEFAULT 0,
    fill_rate REAL NOT NULL DEFAULT 0,
    cancellation_rate REAL NOT NULL DEFAULT 0,
    return_rate REAL NOT NULL DEFAULT 0,
    customer_complaints INTEGER NOT NULL DEFAULT 0,
    quality_score REAL NOT NULL DEFAULT 0,
    overall_score REAL NOT NULL DEFAULT 0,
    tier TEXT NOT NULL DEFAULT 'Standard',
    created_at TEXT NOT NULL
);

-- Drop Ship Product Mappings
CREATE TABLE IF NOT EXISTS drop_ship_product_mappings (
    id TEXT PRIMARY KEY,
    vendor_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    vendor_sku TEXT NOT NULL,
    vendor_product_name TEXT,
    vendor_cost INTEGER NOT NULL DEFAULT 0,
    vendor_msrp INTEGER,
    vendor_map INTEGER,
    min_order_qty INTEGER NOT NULL DEFAULT 1,
    lead_time_days INTEGER NOT NULL DEFAULT 2,
    enabled INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'Active',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(vendor_id, product_id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_dropship_orders_vendor ON drop_ship_orders(vendor_id);
CREATE INDEX IF NOT EXISTS idx_dropship_orders_customer ON drop_ship_orders(customer_id);
CREATE INDEX IF NOT EXISTS idx_dropship_orders_status ON drop_ship_orders(status);
CREATE INDEX IF NOT EXISTS idx_dropship_orders_sales_order ON drop_ship_orders(sales_order_id);
CREATE INDEX IF NOT EXISTS idx_dropship_order_lines_order ON drop_ship_order_lines(drop_ship_order_id);
CREATE INDEX IF NOT EXISTS idx_dropship_order_lines_product ON drop_ship_order_lines(product_id);
CREATE INDEX IF NOT EXISTS idx_dropship_po_vendor ON drop_ship_purchase_orders(vendor_id);
CREATE INDEX IF NOT EXISTS idx_dropship_po_lines_product ON drop_ship_po_lines(product_id);
CREATE INDEX IF NOT EXISTS idx_dropship_shipments_order ON drop_ship_shipments(drop_ship_order_id);
CREATE INDEX IF NOT EXISTS idx_dropship_shipments_tracking ON drop_ship_shipments(tracking_number);
CREATE INDEX IF NOT EXISTS idx_dropship_shipments_vendor ON drop_ship_shipments(vendor_id);
CREATE INDEX IF NOT EXISTS idx_dropship_inventory_vendor ON drop_ship_inventory_items(vendor_id);
CREATE INDEX IF NOT EXISTS idx_dropship_inventory_product ON drop_ship_inventory_items(product_id);
CREATE INDEX IF NOT EXISTS idx_dropship_invoices_vendor ON drop_ship_invoices(vendor_id);
CREATE INDEX IF NOT EXISTS idx_dropship_invoices_order ON drop_ship_invoices(drop_ship_order_id);
CREATE INDEX IF NOT EXISTS idx_dropship_invoice_lines_product ON drop_ship_invoice_lines(product_id);
CREATE INDEX IF NOT EXISTS idx_dropship_returns_vendor ON drop_ship_returns(vendor_id);
CREATE INDEX IF NOT EXISTS idx_dropship_returns_order ON drop_ship_returns(drop_ship_order_id);
CREATE INDEX IF NOT EXISTS idx_dropship_performance_vendor ON drop_ship_performance(vendor_id);
CREATE INDEX IF NOT EXISTS idx_dropship_performance_period ON drop_ship_performance(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_dropship_mappings_vendor ON drop_ship_product_mappings(vendor_id);
CREATE INDEX IF NOT EXISTS idx_dropship_mappings_product ON drop_ship_product_mappings(product_id);
