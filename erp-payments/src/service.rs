use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct PaymentService;

impl PaymentService {
    pub async fn create(pool: &SqlitePool, req: CreatePaymentRequest, user_id: Option<Uuid>) -> Result<Payment> {
        let now = Utc::now();
        let payment_number = format!("PAY-{}", now.format("%Y%m%d%H%M%S"));
        
        let payment = Payment {
            id: Uuid::new_v4(),
            payment_number,
            gateway_id: req.gateway_id,
            invoice_id: req.invoice_id,
            customer_id: req.customer_id,
            amount: req.amount,
            currency: req.currency,
            payment_method: req.payment_method,
            status: PaymentStatus::Completed,
            gateway_transaction_id: None,
            gateway_response: None,
            card_last_four: req.card_last_four,
            card_brand: req.card_brand,
            bank_name: req.bank_name,
            bank_account_last_four: req.bank_account_last_four,
            check_number: req.check_number,
            refunded_amount: 0,
            refund_reason: None,
            processing_fee: 0,
            notes: req.notes,
            paid_at: Some(now),
            created_at: now,
            created_by: user_id,
        };
        PaymentRepository::create(pool, &payment).await?;
        
        if let Some(invoice_id) = req.invoice_id {
            Self::allocate_to_invoice(pool, payment.id, invoice_id, req.amount).await?;
        }
        
        Ok(payment)
    }

    async fn allocate_to_invoice(pool: &SqlitePool, payment_id: Uuid, invoice_id: Uuid, amount: i64) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO payment_allocations (id, payment_id, invoice_id, amount, created_at)
               VALUES (?, ?, ?, ?, ?)"#
        )
        .bind(Uuid::new_v4().to_string())
        .bind(payment_id.to_string())
        .bind(invoice_id.to_string())
        .bind(amount)
        .bind(now.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<Option<Payment>> {
        PaymentRepository::get_by_id(pool, id).await
    }

    pub async fn list_by_customer(pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<Payment>> {
        PaymentRepository::list_by_customer(pool, customer_id).await
    }

    pub async fn refund(pool: &SqlitePool, req: CreateRefundRequest, user_id: Option<Uuid>) -> Result<Refund> {
        let payment = PaymentRepository::get_by_id(pool, req.payment_id).await?
            .ok_or_else(|| anyhow::anyhow!("Payment not found"))?;
        
        if req.amount > payment.amount - payment.refunded_amount {
            return Err(anyhow::anyhow!("Refund amount exceeds available balance"));
        }
        
        let now = Utc::now();
        let refund_number = format!("RFD-{}", now.format("%Y%m%d%H%M%S"));
        
        let refund = Refund {
            id: Uuid::new_v4(),
            refund_number,
            payment_id: req.payment_id,
            amount: req.amount,
            currency: payment.currency.clone(),
            reason: req.reason,
            status: "Completed".to_string(),
            gateway_refund_id: None,
            processed_at: Some(now),
            created_at: now,
            created_by: user_id,
        };
        RefundRepository::create(pool, &refund).await?;
        
        let new_refunded = payment.refunded_amount + req.amount;
        let new_status = if new_refunded >= payment.amount {
            PaymentStatus::Refunded
        } else {
            PaymentStatus::PartiallyRefunded
        };
        
        sqlx::query(
            r#"UPDATE payments SET refunded_amount = ?, status = ?, refund_reason = ? WHERE id = ?"#
        )
        .bind(new_refunded)
        .bind(format!("{:?}", new_status))
        .bind(&refund.reason)
        .bind(req.payment_id.to_string())
        .execute(pool).await?;
        
        Ok(refund)
    }
}

pub struct GatewayService;

impl GatewayService {
    pub async fn create(pool: &SqlitePool, code: String, name: String, gateway_type: String, supported_methods: Vec<String>) -> Result<PaymentGateway> {
        let now = Utc::now();
        let gateway = PaymentGateway {
            id: Uuid::new_v4(),
            code,
            name,
            gateway_type,
            api_key: None,
            api_secret: None,
            merchant_id: None,
            webhook_secret: None,
            is_live: false,
            is_active: true,
            supported_methods: serde_json::to_string(&supported_methods)?,
            created_at: now,
            updated_at: now,
        };
        GatewayRepository::create(pool, &gateway).await?;
        Ok(gateway)
    }

    pub async fn list_active(pool: &SqlitePool) -> Result<Vec<PaymentGateway>> {
        GatewayRepository::list_active(pool).await
    }

    pub async fn process_payment(pool: &SqlitePool, req: ProcessPaymentRequest) -> Result<Payment> {
        let gateway = GatewayRepository::list_active(pool).await?
            .into_iter().find(|g| g.id == req.gateway_id)
            .ok_or_else(|| anyhow::anyhow!("Gateway not found"))?;
        
        let now = Utc::now();
        let payment_number = format!("PAY-{}", now.format("%Y%m%d%H%M%S"));
        let processing_fee = (req.amount as f64 * 0.029 + 30.0) as i64;
        
        let payment = Payment {
            id: Uuid::new_v4(),
            payment_number,
            gateway_id: Some(req.gateway_id),
            invoice_id: req.invoice_id,
            customer_id: req.customer_id,
            amount: req.amount,
            currency: req.currency,
            payment_method: PaymentMethod::CreditCard,
            status: PaymentStatus::Completed,
            gateway_transaction_id: Some(format!("txn_{}", Uuid::new_v4())),
            gateway_response: Some(r#"{"status": "succeeded"}"#.to_string()),
            card_last_four: Some("4242".to_string()),
            card_brand: Some("Visa".to_string()),
            bank_name: None,
            bank_account_last_four: None,
            check_number: None,
            refunded_amount: 0,
            refund_reason: None,
            processing_fee,
            notes: req.description,
            paid_at: Some(now),
            created_at: now,
            created_by: None,
        };
        PaymentRepository::create(pool, &payment).await?;
        
        if let Some(invoice_id) = req.invoice_id {
            sqlx::query(
                r#"INSERT INTO payment_allocations (id, payment_id, invoice_id, amount, created_at)
                   VALUES (?, ?, ?, ?, ?)"#
            )
            .bind(Uuid::new_v4().to_string())
            .bind(payment.id.to_string())
            .bind(invoice_id.to_string())
            .bind(req.amount)
            .bind(now.to_rfc3339())
            .execute(pool).await?;
        }
        
        Ok(payment)
    }
}

pub struct CustomerPaymentMethodService;

impl CustomerPaymentMethodService {
    pub async fn save(pool: &SqlitePool, customer_id: Uuid, payment_method: PaymentMethod, gateway_token: String, card_last_four: Option<String>, card_brand: Option<String>) -> Result<CustomerPaymentMethod> {
        let now = Utc::now();
        let method = CustomerPaymentMethod {
            id: Uuid::new_v4(),
            customer_id,
            payment_method,
            is_default: false,
            card_last_four,
            card_brand,
            card_expiry_month: None,
            card_expiry_year: None,
            card_holder_name: None,
            bank_name: None,
            bank_account_type: None,
            gateway_token: Some(gateway_token),
            nickname: None,
            created_at: now,
        };
        sqlx::query(
            r#"INSERT INTO customer_payment_methods (id, customer_id, payment_method, is_default, card_last_four, card_brand, card_expiry_month, card_expiry_year, card_holder_name, bank_name, bank_account_type, gateway_token, nickname, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(method.id.to_string())
        .bind(method.customer_id.to_string())
        .bind(format!("{:?}", method.payment_method))
        .bind(method.is_default)
        .bind(&method.card_last_four)
        .bind(&method.card_brand)
        .bind(method.card_expiry_month)
        .bind(method.card_expiry_year)
        .bind(&method.card_holder_name)
        .bind(&method.bank_name)
        .bind(&method.bank_account_type)
        .bind(&method.gateway_token)
        .bind(&method.nickname)
        .bind(method.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(method)
    }
}
