use erp_purchasing::*;
use erp_core::*;
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_calculate_landed_cost_by_value() {
    let service = LandedCostService::new();
    
    let product_id1 = Uuid::new_v4();
    let product_id2 = Uuid::new_v4();
    
    let po = PurchaseOrder {
        base: BaseEntity::new(),
        po_number: "PO-001".to_string(),
        vendor_id: Uuid::new_v4(),
        order_date: Utc::now(),
        expected_date: None,
        lines: vec![
            PurchaseOrderLine {
                id: Uuid::new_v4(),
                product_id: product_id1,
                description: "Product 1".to_string(),
                quantity: 10,
                unit_price: Money::new(100, Currency::USD),
                tax_rate: 0.0,
                line_total: Money::new(1000, Currency::USD),
            },
            PurchaseOrderLine {
                id: Uuid::new_v4(),
                product_id: product_id2,
                description: "Product 2".to_string(),
                quantity: 5,
                unit_price: Money::new(400, Currency::USD),
                tax_rate: 0.0,
                line_total: Money::new(2000, Currency::USD),
            },
        ],
        subtotal: Money::new(3000, Currency::USD),
        tax_amount: Money::new(0, Currency::USD),
        total: Money::new(3000, Currency::USD),
        status: Status::Approved,
    };
    
    let category_id = Uuid::new_v4();
    let categories = vec![
        LandedCostCategory {
            base: BaseEntity::new_with_id(category_id),
            code: "FREIGHT".to_string(),
            name: "Freight".to_string(),
            description: None,
            allocation_method: LandedCostAllocationMethod::ByValue,
            status: Status::Active,
        }
    ];
    
    let voucher = LandedCostVoucher {
        base: BaseEntity::new(),
        voucher_number: "LCV-001".to_string(),
        voucher_date: Utc::now(),
        reference_type: LandedCostReferenceType::PurchaseOrder,
        reference_id: po.base.id,
        total_landed_cost: Money::new(300, Currency::USD),
        status: Status::Active,
        lines: vec![
            LandedCostLine {
                id: Uuid::new_v4(),
                voucher_id: Uuid::new_v4(), // Placeholder
                category_id,
                description: "Shipping cost".to_string(),
                amount: Money::new(300, Currency::USD),
            }
        ],
    };
    
    let allocations = service.calculate_allocations(&voucher, &po, &categories).unwrap();
    
    assert_eq!(allocations.len(), 2);
    
    let alloc1 = allocations.iter().find(|a| a.item_id == product_id1).unwrap();
    let alloc2 = allocations.iter().find(|a| a.item_id == product_id2).unwrap();
    
    // Total PO value = 3000. Product 1 = 1000 (1/3), Product 2 = 2000 (2/3).
    // Total Landed Cost = 300. 
    // Product 1 should get 100. Product 2 should get 200.
    assert_eq!(alloc1.allocated_amount.amount, 100);
    assert_eq!(alloc2.allocated_amount.amount, 200);
}

#[test]
fn test_calculate_landed_cost_by_quantity() {
    let service = LandedCostService::new();
    
    let product_id1 = Uuid::new_v4();
    let product_id2 = Uuid::new_v4();
    
    let po = PurchaseOrder {
        base: BaseEntity::new(),
        po_number: "PO-002".to_string(),
        vendor_id: Uuid::new_v4(),
        order_date: Utc::now(),
        expected_date: None,
        lines: vec![
            PurchaseOrderLine {
                id: Uuid::new_v4(),
                product_id: product_id1,
                description: "Product 1".to_string(),
                quantity: 10,
                unit_price: Money::new(100, Currency::USD),
                tax_rate: 0.0,
                line_total: Money::new(1000, Currency::USD),
            },
            PurchaseOrderLine {
                id: Uuid::new_v4(),
                product_id: product_id2,
                description: "Product 2".to_string(),
                quantity: 40,
                unit_price: Money::new(50, Currency::USD),
                tax_rate: 0.0,
                line_total: Money::new(2000, Currency::USD),
            },
        ],
        subtotal: Money::new(3000, Currency::USD),
        tax_amount: Money::new(0, Currency::USD),
        total: Money::new(3000, Currency::USD),
        status: Status::Approved,
    };
    
    let category_id = Uuid::new_v4();
    let categories = vec![
        LandedCostCategory {
            base: BaseEntity::new_with_id(category_id),
            code: "DUTY".to_string(),
            name: "Customs Duty".to_string(),
            description: None,
            allocation_method: LandedCostAllocationMethod::ByQuantity,
            status: Status::Active,
        }
    ];
    
    let voucher = LandedCostVoucher {
        base: BaseEntity::new(),
        voucher_number: "LCV-002".to_string(),
        voucher_date: Utc::now(),
        reference_type: LandedCostReferenceType::PurchaseOrder,
        reference_id: po.base.id,
        total_landed_cost: Money::new(500, Currency::USD),
        status: Status::Active,
        lines: vec![
            LandedCostLine {
                id: Uuid::new_v4(),
                voucher_id: Uuid::new_v4(),
                category_id,
                description: "Duty".to_string(),
                amount: Money::new(500, Currency::USD),
            }
        ],
    };
    
    let allocations = service.calculate_allocations(&voucher, &po, &categories).unwrap();
    
    assert_eq!(allocations.len(), 2);
    
    let alloc1 = allocations.iter().find(|a| a.item_id == product_id1).unwrap();
    let alloc2 = allocations.iter().find(|a| a.item_id == product_id2).unwrap();
    
    // Total quantity = 10 + 40 = 50.
    // Product 1 (10/50 = 20%) -> 500 * 0.2 = 100.
    // Product 2 (40/50 = 80%) -> 500 * 0.8 = 400.
    assert_eq!(alloc1.allocated_amount.amount, 100);
    assert_eq!(alloc2.allocated_amount.amount, 400);
}
