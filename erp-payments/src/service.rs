use crate::models::*;
use crate::repository::*;
use erp_core::{BaseEntity, Error, Result};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct PaymentService<
    P: PaymentRepository = SqlitePaymentRepository,
    R: RefundRepository = SqliteRefundRepository,
    A: PaymentAllocationRepository = SqlitePaymentAllocationRepository,
> {
    repo: P,
    refund_repo: R,
    allocation_repo: A,
    pool: SqlitePool,
}

impl PaymentService<SqlitePaymentRepository, SqliteRefundRepository, SqlitePaymentAllocationRepository> {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqlitePaymentRepository::new(pool.clone()),
            refund_repo: SqliteRefundRepository::new(pool.clone()),
            allocation_repo: SqlitePaymentAllocationRepository::new(pool.clone()),
            pool,
        }
    }
}

impl<P, R, A> PaymentService<P, R, A>
where
    P: PaymentRepository,
    R: RefundRepository,
    A: PaymentAllocationRepository,
{
    pub fn with_repos(repo: P, refund_repo: R, allocation_repo: A, pool: SqlitePool) -> Self {
        Self {
            repo,
            refund_repo,
            allocation_repo,
            pool,
        }
    }

    pub async fn create(&self, req: CreatePaymentRequest, user_id: Option<Uuid>) -> Result<Payment> {
        let now = Utc::now();
        let payment_number = format!("PAY-{}", now.format("%Y%m%d%H%M%S"));
        
        let payment = Payment {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: now,
                updated_at: now,
                created_by: user_id,
                updated_by: None,
            },
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
        };
        self.repo.create(payment.clone()).await?;
        
        if let Some(invoice_id) = req.invoice_id {
            self.allocate_to_invoice(payment.base.id, invoice_id, req.amount).await?;
        }
        
        Ok(payment)
    }

    async fn allocate_to_invoice(&self, payment_id: Uuid, invoice_id: Uuid, amount: i64) -> Result<()> {
        let now = Utc::now();
        let allocation = PaymentAllocation {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: now,
                updated_at: now,
                created_by: None,
                updated_by: None,
            },
            payment_id,
            invoice_id,
            amount,
        };
        self.allocation_repo.create(allocation).await?;
        Ok(())
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Payment>> {
        self.repo.get_by_id(id).await
    }

    pub async fn list_by_customer(&self, customer_id: Uuid) -> Result<Vec<Payment>> {
        self.repo.list_by_customer(customer_id).await
    }

    pub async fn refund(&self, req: CreateRefundRequest, user_id: Option<Uuid>) -> Result<Refund> {
        let payment = self.repo.get_by_id(req.payment_id).await?
            .ok_or_else(|| Error::not_found("Payment", &req.payment_id.to_string()))?;
        
        if req.amount > payment.amount - payment.refunded_amount {
            return Err(Error::business_rule("Refund amount exceeds available balance"));
        }
        
        let now = Utc::now();
        let refund_number = format!("RFD-{}", now.format("%Y%m%d%H%M%S"));
        
        let refund = Refund {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: now,
                updated_at: now,
                created_by: user_id,
                updated_by: None,
            },
            refund_number,
            payment_id: req.payment_id,
            amount: req.amount,
            currency: payment.currency.clone(),
            reason: req.reason,
            status: "Completed".to_string(),
            gateway_refund_id: None,
            processed_at: Some(now),
        };
        self.refund_repo.create(refund.clone()).await?;
        
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
        .bind(new_status)
        .bind(&refund.reason)
        .bind(req.payment_id)
        .execute(&self.pool).await?;
        
        Ok(refund)
    }
}

pub struct GatewayService<
    G: GatewayRepository = SqliteGatewayRepository,
    P: PaymentRepository = SqlitePaymentRepository,
    A: PaymentAllocationRepository = SqlitePaymentAllocationRepository,
> {
    repo: G,
    payment_repo: P,
    allocation_repo: A,
    pool: SqlitePool,
}

impl GatewayService<SqliteGatewayRepository, SqlitePaymentRepository, SqlitePaymentAllocationRepository> {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteGatewayRepository::new(pool.clone()),
            payment_repo: SqlitePaymentRepository::new(pool.clone()),
            allocation_repo: SqlitePaymentAllocationRepository::new(pool.clone()),
            pool,
        }
    }
}

impl<G, P, A> GatewayService<G, P, A>
where
    G: GatewayRepository,
    P: PaymentRepository,
    A: PaymentAllocationRepository,
{
    pub fn with_repos(repo: G, payment_repo: P, allocation_repo: A, pool: SqlitePool) -> Self {
        Self {
            repo,
            payment_repo,
            allocation_repo,
            pool,
        }
    }

    pub async fn create(&self, code: String, name: String, gateway_type: String, supported_methods: Vec<String>) -> Result<PaymentGateway> {
        let now = Utc::now();
        let gateway = PaymentGateway {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: now,
                updated_at: now,
                created_by: None,
                updated_by: None,
            },
            code,
            name,
            gateway_type,
            api_key: None,
            api_secret: None,
            merchant_id: None,
            webhook_secret: None,
            is_live: false,
            is_active: true,
            supported_methods: serde_json::to_string(&supported_methods)
                .map_err(|e| Error::Internal(anyhow::anyhow!("Failed to serialize supported methods: {}", e)))?,
        };
        self.repo.create(gateway.clone()).await?;
        Ok(gateway)
    }

    pub async fn list_active(&self) -> Result<Vec<PaymentGateway>> {
        self.repo.list_active().await
    }

    pub async fn process_payment(&self, req: ProcessPaymentRequest) -> Result<Payment> {
        let _gateway = self.repo.list_active().await?
            .into_iter().find(|g| g.base.id == req.gateway_id)
            .ok_or_else(|| Error::not_found("Gateway", &req.gateway_id.to_string()))?;
        
        let now = Utc::now();
        let payment_number = format!("PAY-{}", now.format("%Y%m%d%H%M%S"));
        let processing_fee = (req.amount as f64 * 0.029 + 30.0) as i64;
        
        let payment = Payment {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: now,
                updated_at: now,
                created_by: None,
                updated_by: None,
            },
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
        };
        self.payment_repo.create(payment.clone()).await?;
        
        if let Some(invoice_id) = req.invoice_id {
            let allocation = PaymentAllocation {
                base: BaseEntity {
                    id: Uuid::new_v4(),
                    created_at: now,
                    updated_at: now,
                    created_by: None,
                    updated_by: None,
                },
                payment_id: payment.base.id,
                invoice_id,
                amount: req.amount,
            };
            self.allocation_repo.create(allocation).await?;
        }
        
        Ok(payment)
    }
}


pub struct CustomerPaymentMethodService<
    R: CustomerPaymentMethodRepository = SqliteCustomerPaymentMethodRepository,
> {
    repo: R,
}

impl CustomerPaymentMethodService<SqliteCustomerPaymentMethodRepository> {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteCustomerPaymentMethodRepository::new(pool),
        }
    }
}

impl<R> CustomerPaymentMethodService<R>
where
    R: CustomerPaymentMethodRepository,
{
    pub fn with_repo(repo: R) -> Self {
        Self { repo }
    }

    pub async fn save(&self, customer_id: Uuid, payment_method: PaymentMethod, gateway_token: String, card_last_four: Option<String>, card_brand: Option<String>) -> Result<CustomerPaymentMethod> {
        let now = Utc::now();
        let method = CustomerPaymentMethod {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: now,
                updated_at: now,
                created_by: None,
                updated_by: None,
            },
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
        };
        self.repo.create(method).await
    }
}
