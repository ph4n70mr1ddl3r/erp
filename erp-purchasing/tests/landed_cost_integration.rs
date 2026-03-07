use erp_purchasing::*;
use erp_inventory::*;
use erp_core::*;
use uuid::Uuid;
use chrono::Utc;
use sqlx::SqlitePool;

async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // Run migrations
    let migrations = [
        include_str!("../../migrations/20240101000001_inventory.sql"),
        include_str!("../../migrations/20240101000003_purchasing.sql"),
        include_str!("../../migrations/20240101200800_mrp_eam_valuation_cash_supplier.sql"),
        include_str!("../../migrations/20260307170000_landed_costs.sql"),
    ];
    
    for m in migrations {
        sqlx::query(m).execute(&pool).await.unwrap();
    }
    
    pool
}

#[tokio::test]
async fn test_landed_cost_posting_updates_inventory() {
    let pool = setup_db().await;
    let purchasing_service = LandedCostService::new();
    let inventory_repo = SqliteValuationRepository;
    let vendor_repo = SqliteVendorRepository;
    let po_repo = SqlitePurchaseOrderRepository;
    let product_repo = SqliteProductRepository;
    
    // 1. Create a vendor
    let vendor = vendor_repo.create(&pool, Vendor {
        base: BaseEntity::new(),
        code: "V001".to_string(),
        name: "Test Vendor".to_string(),
        contact: ContactInfo { email: Some("test@example.com".to_string()), phone: None, fax: None, website: None },
        address: Address { street: "123 St".to_string(), city: "City".to_string(), state: None, postal_code: "12345".to_string(), country: "USA".to_string() },
        payment_terms: 30,
        status: Status::Active,
    }).await.unwrap();
    
    // 2. Create a product
    let product = product_repo.create(&pool, Product {
        base: BaseEntity::new(),
        sku: "P001".to_string(),
        name: "Test Product".to_string(),
        description: None,
        product_type: ProductType::Goods,
        category_id: None,
        unit_of_measure: "EA".to_string(),
        status: Status::Active,
    }).await.unwrap();
    
    // 3. Create a PO
    let po = po_repo.create(&pool, PurchaseOrder {
        base: BaseEntity::new(),
        po_number: "PO-001".to_string(),
        vendor_id: vendor.base.id,
        order_date: Utc::now(),
        expected_date: None,
        lines: vec![
            PurchaseOrderLine {
                id: Uuid::new_v4(),
                product_id: product.base.id,
                description: "Test Product".to_string(),
                quantity: 10,
                unit_price: Money::new(1000, Currency::USD), // $10.00
                tax_rate: 0.0,
                line_total: Money::new(10000, Currency::USD), // $100.00
            }
        ],
        subtotal: Money::new(10000, Currency::USD),
        tax_amount: Money::new(0, Currency::USD),
        total: Money::new(10000, Currency::USD),
        status: Status::Approved,
    }).await.unwrap();
    
    // 4. Initialize inventory valuation (e.g. from a previous receipt)
    let warehouse_id = Uuid::nil();
    inventory_repo.update_valuation(&pool, ProductValuation {
        id: Uuid::new_v4(),
        product_id: product.base.id,
        warehouse_id,
        valuation_method: ValuationMethod::MovingAverage,
        standard_cost: 0,
        current_unit_cost: 1000,
        total_quantity: 10,
        total_value: 10000,
        last_receipt_cost: 1000,
        last_receipt_date: Some(Utc::now()),
        last_issue_cost: 0,
        last_issue_date: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }).await.unwrap();
    
    // 5. Create a Landed Cost Category
    sqlx::query("INSERT INTO landed_cost_categories (id, code, name, allocation_method, status, created_at, updated_at) VALUES (?, 'FREIGHT', 'Freight', 'ByValue', 'Active', ?, ?)")
        .bind(Uuid::new_v4().to_string())
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(&pool).await.unwrap();
    
    let categories = SqliteLandedCostRepository.find_categories(&pool).await.unwrap();
    let freight_cat = categories.iter().find(|c| c.code == "FREIGHT").unwrap();
    
    // 6. Create Landed Cost Voucher
    let voucher = purchasing_service.create_voucher(
        &pool,
        "LCV-001",
        LandedCostReferenceType::PurchaseOrder,
        po.base.id,
        vec![
            LandedCostLine {
                id: Uuid::new_v4(),
                voucher_id: Uuid::nil(), // set in create_voucher
                category_id: freight_cat.base.id,
                description: "Shipping".to_string(),
                amount: Money::new(2000, Currency::USD), // $20.00
            }
        ]
    ).await.unwrap();
    
    // 7. Post the voucher
    purchasing_service.post_voucher(&pool, voucher.base.id).await.unwrap();
    
    // 8. Verify inventory update
    let valuation = inventory_repo.get_valuation(&pool, product.base.id, warehouse_id).await.unwrap();
    
    // Original value = 10000. Landed cost = 2000. New value = 12000.
    // Total quantity = 10. New unit cost = 12000 / 10 = 1200 ($12.00).
    assert_eq!(valuation.total_value, 12000);
    assert_eq!(valuation.current_unit_cost, 1200);
    
    // Verify cost adjustment record
    let rows = sqlx::query("SELECT count(*) as count FROM cost_adjustments").fetch_one(&pool).await.unwrap();
    let count: i64 = sqlx::Row::get(&rows, "count");
    assert_eq!(count, 1);
}
