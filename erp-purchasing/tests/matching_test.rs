use erp_purchasing::models::*;
use erp_purchasing::service::ThreeWayMatchService;
use erp_core::models::{BaseEntity, Money, Currency, Status};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_three_way_match_success() {
    let product_id = Uuid::new_v4();
    let po_id = Uuid::new_v4();
    
    let po = PurchaseOrder {
        base: BaseEntity { id: po_id, ..BaseEntity::new() },
        po_number: "PO-001".to_string(),
        vendor_id: Uuid::new_v4(),
        order_date: Utc::now(),
        expected_date: None,
        lines: vec![PurchaseOrderLine {
            id: Uuid::new_v4(),
            product_id,
            description: "Widget".to_string(),
            quantity: 10,
            unit_price: Money::new(100, Currency::USD),
            tax_rate: 0.0,
            line_total: Money::new(1000, Currency::USD),
        }],
        subtotal: Money::new(1000, Currency::USD),
        tax_amount: Money::new(0, Currency::USD),
        total: Money::new(1000, Currency::USD),
        status: Status::Approved,
    };

    let gr = GoodsReceipt {
        base: BaseEntity::new(),
        receipt_number: "GR-001".to_string(),
        purchase_order_id: po_id,
        warehouse_id: Uuid::new_v4(),
        receipt_date: Utc::now(),
        lines: vec![GoodsReceiptLine {
            id: Uuid::new_v4(),
            po_line_id: po.lines[0].id,
            product_id,
            quantity_ordered: 10,
            quantity_received: 10,
        }],
        status: Status::Completed,
    };

    let invoice = VendorInvoice {
        base: BaseEntity::new(),
        invoice_number: "INV-001".to_string(),
        vendor_id: po.vendor_id,
        purchase_order_id: po_id,
        invoice_date: Utc::now(),
        due_date: Utc::now(),
        lines: vec![VendorInvoiceLine {
            id: Uuid::new_v4(),
            product_id,
            description: "Widget".to_string(),
            quantity: 10,
            unit_price: Money::new(100, Currency::USD),
            line_total: Money::new(1000, Currency::USD),
        }],
        subtotal: Money::new(1000, Currency::USD),
        tax_amount: Money::new(0, Currency::USD),
        total: Money::new(1000, Currency::USD),
        status: Status::Pending,
    };

    let result = ThreeWayMatchService::perform_match(&invoice, &po, &gr).unwrap();
    assert_eq!(result.status, MatchStatus::Matched);
    assert!(result.price_matched);
    assert!(result.quantity_matched);
}

#[test]
fn test_three_way_match_price_variance() {
    let product_id = Uuid::new_v4();
    let po_id = Uuid::new_v4();
    
    let po = PurchaseOrder {
        base: BaseEntity { id: po_id, ..BaseEntity::new() },
        po_number: "PO-001".to_string(),
        vendor_id: Uuid::new_v4(),
        order_date: Utc::now(),
        expected_date: None,
        lines: vec![PurchaseOrderLine {
            id: Uuid::new_v4(),
            product_id,
            description: "Widget".to_string(),
            quantity: 10,
            unit_price: Money::new(100, Currency::USD),
            tax_rate: 0.0,
            line_total: Money::new(1000, Currency::USD),
        }],
        subtotal: Money::new(1000, Currency::USD),
        tax_amount: Money::new(0, Currency::USD),
        total: Money::new(1000, Currency::USD),
        status: Status::Approved,
    };

    let gr = GoodsReceipt {
        base: BaseEntity::new(),
        receipt_number: "GR-001".to_string(),
        purchase_order_id: po_id,
        warehouse_id: Uuid::new_v4(),
        receipt_date: Utc::now(),
        lines: vec![GoodsReceiptLine {
            id: Uuid::new_v4(),
            po_line_id: po.lines[0].id,
            product_id,
            quantity_ordered: 10,
            quantity_received: 10,
        }],
        status: Status::Completed,
    };

    // Invoice price is $120 instead of $100
    let invoice = VendorInvoice {
        base: BaseEntity::new(),
        invoice_number: "INV-001".to_string(),
        vendor_id: po.vendor_id,
        purchase_order_id: po_id,
        invoice_date: Utc::now(),
        due_date: Utc::now(),
        lines: vec![VendorInvoiceLine {
            id: Uuid::new_v4(),
            product_id,
            description: "Widget".to_string(),
            quantity: 10,
            unit_price: Money::new(120, Currency::USD),
            line_total: Money::new(1200, Currency::USD),
        }],
        subtotal: Money::new(1200, Currency::USD),
        tax_amount: Money::new(0, Currency::USD),
        total: Money::new(1200, Currency::USD),
        status: Status::Pending,
    };

    let result = ThreeWayMatchService::perform_match(&invoice, &po, &gr).unwrap();
    assert_eq!(result.status, MatchStatus::Variance);
    assert!(!result.price_matched);
    assert!(result.quantity_matched);
    assert_eq!(result.price_variance, 200); // (120 - 100) * 10 = 200
}
