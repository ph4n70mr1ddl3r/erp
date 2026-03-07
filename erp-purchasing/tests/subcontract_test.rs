use erp_purchasing::service::SubcontractService;
use erp_purchasing::models::*;
use erp_core::{Money, Currency};
use sqlx::SqlitePool;
use uuid::Uuid;

#[tokio::test]
async fn test_create_subcontract_order() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // Create tables
    sqlx::query(
        "CREATE TABLE subcontract_orders (
            id TEXT PRIMARY KEY,
            order_number TEXT NOT NULL UNIQUE,
            vendor_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            service_cost_amount INTEGER NOT NULL,
            service_cost_currency TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Draft',
            warehouse_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            created_by TEXT,
            updated_by TEXT
        )"
    ).execute(&pool).await.unwrap();
    
    sqlx::query(
        "CREATE TABLE subcontract_components (
            id TEXT PRIMARY KEY,
            order_id TEXT NOT NULL REFERENCES subcontract_orders(id),
            product_id TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            sent_quantity INTEGER NOT NULL DEFAULT 0,
            consumed_quantity INTEGER NOT NULL DEFAULT 0
        )"
    ).execute(&pool).await.unwrap();

    let service = SubcontractService::new();
    let vendor_id = Uuid::new_v4();
    let product_id = Uuid::new_v4();
    let warehouse_id = Uuid::new_v4();
    let service_cost = Money::new(50000, Currency::USD);
    
    let components = vec![
        SubcontractComponent {
            id: Uuid::new_v4(),
            product_id: Uuid::new_v4(),
            quantity: 10,
            sent_quantity: 0,
            consumed_quantity: 0,
        }
    ];

    let result = service.create_order(
        &pool,
        vendor_id,
        product_id,
        100,
        service_cost,
        warehouse_id,
        components,
    ).await;

    assert!(result.is_ok());
    let order = result.unwrap();
    assert_eq!(order.quantity, 100);
    assert_eq!(order.status, SubcontractOrderStatus::Draft);
    assert_eq!(order.components.len(), 1);
}
