use erp_inventory::service::InventoryValuationService;
use sqlx::sqlite::SqlitePoolOptions;
use uuid::Uuid;

#[tokio::test]
async fn test_fifo_inventory_valuation() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    // Create necessary tables for the test
    sqlx::query(
        "CREATE TABLE product_valuations (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            warehouse_id TEXT NOT NULL,
            valuation_method TEXT NOT NULL,
            standard_cost INTEGER NOT NULL DEFAULT 0,
            current_unit_cost INTEGER NOT NULL DEFAULT 0,
            total_quantity INTEGER NOT NULL DEFAULT 0,
            total_value INTEGER NOT NULL DEFAULT 0,
            last_receipt_cost INTEGER NOT NULL DEFAULT 0,
            last_receipt_date TEXT,
            last_issue_cost INTEGER NOT NULL DEFAULT 0,
            last_issue_date TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            UNIQUE(product_id, warehouse_id)
        )"
    ).execute(&pool).await.unwrap();

    sqlx::query(
        "CREATE TABLE inventory_cost_layers (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            warehouse_id TEXT NOT NULL,
            layer_date TEXT NOT NULL,
            receipt_reference TEXT NOT NULL,
            receipt_id TEXT,
            quantity INTEGER NOT NULL,
            unit_cost INTEGER NOT NULL,
            remaining_quantity INTEGER NOT NULL,
            total_value INTEGER NOT NULL,
            created_at TEXT NOT NULL
        )"
    ).execute(&pool).await.unwrap();

    let service = InventoryValuationService::new();
    let product_id = Uuid::new_v4();
    let warehouse_id = Uuid::new_v4();

    // 1. Record first receipt: 10 units @ $100
    service.record_receipt(&pool, product_id, warehouse_id, 10, 100, "PO-001").await.unwrap();

    // 2. Record second receipt: 5 units @ $120
    service.record_receipt(&pool, product_id, warehouse_id, 5, 120, "PO-002").await.unwrap();

    // Total should be 15 units, total value = (10*100) + (5*120) = 1000 + 600 = 1600
    // Avg unit cost = 1600 / 15 = 106.66 -> 106

    // 3. Issue 8 units (should come from first layer @ $100)
    let cost8 = service.issue_inventory_fifo(&pool, product_id, warehouse_id, 8).await.unwrap();
    assert_eq!(cost8, 800); // 8 * 100

    // Remaining: 2 units @ $100, 5 units @ $120. Total = 200 + 600 = 800. Quantity = 7.

    // 4. Issue 4 units (should take remaining 2 from first layer @ $100, and 2 from second layer @ $120)
    let cost4 = service.issue_inventory_fifo(&pool, product_id, warehouse_id, 4).await.unwrap();
    assert_eq!(cost4, (2 * 100) + (2 * 120)); // 200 + 240 = 440
    assert_eq!(cost4, 440);

    // Remaining: 3 units @ $120. Total = 360. Quantity = 3.
}

#[tokio::test]
async fn test_insufficient_stock_error() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    // Create necessary tables
    sqlx::query("CREATE TABLE product_valuations (id TEXT PRIMARY KEY, product_id TEXT NOT NULL, warehouse_id TEXT NOT NULL, valuation_method TEXT NOT NULL, standard_cost INTEGER NOT NULL DEFAULT 0, current_unit_cost INTEGER NOT NULL DEFAULT 0, total_quantity INTEGER NOT NULL DEFAULT 0, total_value INTEGER NOT NULL DEFAULT 0, last_receipt_cost INTEGER NOT NULL DEFAULT 0, last_receipt_date TEXT, last_issue_cost INTEGER NOT NULL DEFAULT 0, last_issue_date TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, UNIQUE(product_id, warehouse_id))").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE inventory_cost_layers (id TEXT PRIMARY KEY, product_id TEXT NOT NULL, warehouse_id TEXT NOT NULL, layer_date TEXT NOT NULL, receipt_reference TEXT NOT NULL, receipt_id TEXT, quantity INTEGER NOT NULL, unit_cost INTEGER NOT NULL, remaining_quantity INTEGER NOT NULL, total_value INTEGER NOT NULL, created_at TEXT NOT NULL)").execute(&pool).await.unwrap();

    let service = InventoryValuationService::new();
    let product_id = Uuid::new_v4();
    let warehouse_id = Uuid::new_v4();

    service.record_receipt(&pool, product_id, warehouse_id, 5, 100, "PO-001").await.unwrap();

    let result = service.issue_inventory_fifo(&pool, product_id, warehouse_id, 10).await;
    assert!(result.is_err());
}
