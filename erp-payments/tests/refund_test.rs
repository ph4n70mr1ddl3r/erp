use erp_payments::*;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[tokio::test]
async fn test_payment_refund() -> Result<()> {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // Create tables
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS payments (
            id TEXT PRIMARY KEY,
            payment_number TEXT NOT NULL UNIQUE,
            gateway_id TEXT,
            invoice_id TEXT,
            customer_id TEXT NOT NULL,
            amount INTEGER NOT NULL,
            currency TEXT NOT NULL,
            payment_method TEXT NOT NULL,
            status TEXT NOT NULL,
            gateway_transaction_id TEXT,
            gateway_response TEXT,
            card_last_four TEXT,
            card_brand TEXT,
            bank_name TEXT,
            bank_account_last_four TEXT,
            check_number TEXT,
            refunded_amount INTEGER NOT NULL DEFAULT 0,
            refund_reason TEXT,
            processing_fee INTEGER NOT NULL DEFAULT 0,
            notes TEXT,
            paid_at TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            created_by TEXT,
            updated_by TEXT
        )"#,
    ).execute(&pool).await.unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS refunds (
            id TEXT PRIMARY KEY,
            refund_number TEXT NOT NULL UNIQUE,
            payment_id TEXT NOT NULL,
            amount INTEGER NOT NULL,
            currency TEXT NOT NULL,
            reason TEXT NOT NULL,
            status TEXT NOT NULL,
            gateway_refund_id TEXT,
            processed_at TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            created_by TEXT,
            updated_by TEXT
        )"#,
    ).execute(&pool).await.unwrap();

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS payment_allocations (
            id TEXT PRIMARY KEY,
            payment_id TEXT NOT NULL,
            invoice_id TEXT NOT NULL,
            amount INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            created_by TEXT,
            updated_by TEXT
        )"#,
    ).execute(&pool).await.unwrap();

    let service = PaymentService::new(pool.clone());
    let customer_id = Uuid::new_v4();
    
    let req = CreatePaymentRequest {
        customer_id,
        amount: 10000, // $100.00
        currency: "USD".to_string(),
        payment_method: PaymentMethod::CreditCard,
        invoice_id: None,
        gateway_id: None,
        card_last_four: Some("4242".to_string()),
        card_brand: Some("Visa".to_string()),
        bank_name: None,
        bank_account_last_four: None,
        check_number: None,
        notes: Some("Test payment".to_string()),
    };
    
    let payment = service.create(req, None).await?;
    assert_eq!(payment.amount, 10000);
    assert_eq!(payment.refunded_amount, 0);
    assert_eq!(payment.status, PaymentStatus::Completed);
    
    // Partially refund
    let refund_req = CreateRefundRequest {
        payment_id: payment.base.id,
        amount: 4000,
        reason: "Customer request".to_string(),
    };
    
    let refund = service.refund(refund_req, None).await?;
    assert_eq!(refund.amount, 4000);
    
    // Verify payment state
    let updated_payment = service.get(payment.base.id).await?.unwrap();
    assert_eq!(updated_payment.refunded_amount, 4000);
    assert_eq!(updated_payment.status, PaymentStatus::PartiallyRefunded);
    
    // Refund the rest
    let refund_req2 = CreateRefundRequest {
        payment_id: payment.base.id,
        amount: 6000,
        reason: "Full refund".to_string(),
    };
    
    let _ = service.refund(refund_req2, None).await?;
    
    // Verify payment state
    let final_payment = service.get(payment.base.id).await?.unwrap();
    assert_eq!(final_payment.refunded_amount, 10000);
    assert_eq!(final_payment.status, PaymentStatus::Refunded);
    
    Ok(())
}
