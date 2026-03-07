use crate::models::*;
use erp_core::{Error, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct StripeRepository {
    pool: SqlitePool,
}

impl StripeRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_payment_intent(&self, intent: StripePaymentIntent) -> Result<StripePaymentIntent> {
        sqlx::query(
            r#"INSERT INTO stripe_payment_intents (id, stripe_intent_id, customer_id, invoice_id, amount, currency, status, client_secret, description, metadata, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(intent.id)
        .bind(&intent.stripe_intent_id)
        .bind(intent.customer_id)
        .bind(intent.invoice_id)
        .bind(intent.amount)
        .bind(&intent.currency)
        .bind(&intent.status)
        .bind(&intent.client_secret)
        .bind(&intent.description)
        .bind(&intent.metadata)
        .bind(intent.created_at)
        .bind(intent.updated_at)
        .execute(&self.pool).await?;
        Ok(intent)
    }
    
    pub async fn get_payment_intent_by_stripe_id(&self, stripe_id: &str) -> Result<Option<StripePaymentIntent>> {
        sqlx::query_as::<_, StripePaymentIntent>(
            "SELECT * FROM stripe_payment_intents WHERE stripe_intent_id = ?"
        )
        .bind(stripe_id)
        .fetch_optional(&self.pool).await
        .map_err(|e| Error::Internal(e.into()))
    }
    
    pub async fn update_payment_intent(&self, intent: StripePaymentIntent) -> Result<StripePaymentIntent> {
        sqlx::query(
            "UPDATE stripe_payment_intents SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&intent.status)
        .bind(intent.updated_at)
        .bind(intent.id)
        .execute(&self.pool).await?;
        Ok(intent)
    }
    
    pub async fn list_payment_intents_by_customer(&self, customer_id: Uuid) -> Result<Vec<StripePaymentIntent>> {
        sqlx::query_as::<_, StripePaymentIntent>(
            "SELECT * FROM stripe_payment_intents WHERE customer_id = ?"
        )
        .bind(customer_id)
        .fetch_all(&self.pool).await
        .map_err(|e| Error::Internal(e.into()))
    }

    pub async fn create_checkout_session(&self, session: StripeCheckoutSession) -> Result<StripeCheckoutSession> {
        sqlx::query(
            r#"INSERT INTO stripe_checkout_sessions (id, stripe_session_id, customer_id, invoice_id, amount, currency, status, checkout_url, success_url, cancel_url, payment_intent_id, expires_at, completed_at, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(session.id)
        .bind(&session.stripe_session_id)
        .bind(session.customer_id)
        .bind(session.invoice_id)
        .bind(session.amount)
        .bind(&session.currency)
        .bind(&session.status)
        .bind(&session.checkout_url)
        .bind(&session.success_url)
        .bind(&session.cancel_url)
        .bind(&session.payment_intent_id)
        .bind(session.expires_at)
        .bind(session.completed_at)
        .bind(session.created_at)
        .execute(&self.pool).await?;
        Ok(session)
    }
    
    pub async fn get_checkout_session_by_stripe_id(&self, stripe_id: &str) -> Result<Option<StripeCheckoutSession>> {
        sqlx::query_as::<_, StripeCheckoutSession>(
            "SELECT * FROM stripe_checkout_sessions WHERE stripe_session_id = ?"
        )
        .bind(stripe_id)
        .fetch_optional(&self.pool).await
        .map_err(|e| Error::Internal(e.into()))
    }

    pub async fn get_payment_intent_by_id(&self, id: Uuid) -> Result<Option<StripePaymentIntent>> {
        sqlx::query_as::<_, StripePaymentIntent>(
            "SELECT * FROM stripe_payment_intents WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool).await
        .map_err(|e| Error::Internal(e.into()))
    }

    pub async fn get_checkout_session_by_id(&self, id: Uuid) -> Result<Option<StripeCheckoutSession>> {
        sqlx::query_as::<_, StripeCheckoutSession>(
            "SELECT * FROM stripe_checkout_sessions WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool).await
        .map_err(|e| Error::Internal(e.into()))
    }

    pub async fn update_payment_intent_status(&self, intent: &StripePaymentIntent) -> Result<()> {
        sqlx::query(
            "UPDATE stripe_payment_intents SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&intent.status)
        .bind(intent.updated_at)
        .bind(intent.id)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn update_checkout_session_status(&self, session: &StripeCheckoutSession) -> Result<()> {
        sqlx::query(
            "UPDATE stripe_checkout_sessions SET status = ?, completed_at = ? WHERE id = ?"
        )
        .bind(&session.status)
        .bind(session.completed_at)
        .bind(session.id)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn create_webhook_event(&self, event: &StripeWebhookEvent) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO stripe_webhook_events (id, stripe_event_id, event_type, payload, processed, processed_at, error_message, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(event.id)
        .bind(&event.stripe_event_id)
        .bind(&event.event_type)
        .bind(&event.payload)
        .bind(event.processed)
        .bind(event.processed_at)
        .bind(&event.error_message)
        .bind(event.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }
}


pub struct PaymentRepository {
    pool: SqlitePool,
}

impl PaymentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, payment: Payment) -> Result<Payment> {
        sqlx::query(
            r#"INSERT INTO payments (id, payment_number, gateway_id, invoice_id, customer_id, amount, currency, payment_method, status, gateway_transaction_id, gateway_response, card_last_four, card_brand, bank_name, bank_account_last_four, check_number, refunded_amount, refund_reason, processing_fee, notes, paid_at, created_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(payment.id)
        .bind(&payment.payment_number)
        .bind(payment.gateway_id)
        .bind(payment.invoice_id)
        .bind(payment.customer_id)
        .bind(payment.amount)
        .bind(&payment.currency)
        .bind(&payment.payment_method)
        .bind(&payment.status)
        .bind(&payment.gateway_transaction_id)
        .bind(&payment.gateway_response)
        .bind(&payment.card_last_four)
        .bind(&payment.card_brand)
        .bind(&payment.bank_name)
        .bind(&payment.bank_account_last_four)
        .bind(&payment.check_number)
        .bind(payment.refunded_amount)
        .bind(&payment.refund_reason)
        .bind(payment.processing_fee)
        .bind(&payment.notes)
        .bind(payment.paid_at)
        .bind(payment.created_at)
        .bind(payment.created_by)
        .execute(&self.pool).await?;
        Ok(payment)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Payment>> {
        sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool).await
        .map_err(|e| Error::Internal(e.into()))
    }

    pub async fn list_by_customer(&self, customer_id: Uuid) -> Result<Vec<Payment>> {
        sqlx::query_as::<_, Payment>(
            "SELECT * FROM payments WHERE customer_id = ?"
        )
        .bind(customer_id)
        .fetch_all(&self.pool).await
        .map_err(|e| Error::Internal(e.into()))
    }
}

pub struct GatewayRepository {
    pool: SqlitePool,
}

impl GatewayRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, gateway: PaymentGateway) -> Result<PaymentGateway> {
        sqlx::query(
            r#"INSERT INTO payment_gateways (id, code, name, gateway_type, api_key, api_secret, merchant_id, webhook_secret, is_live, is_active, supported_methods, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(gateway.id)
        .bind(&gateway.code)
        .bind(&gateway.name)
        .bind(&gateway.gateway_type)
        .bind(&gateway.api_key)
        .bind(&gateway.api_secret)
        .bind(&gateway.merchant_id)
        .bind(&gateway.webhook_secret)
        .bind(gateway.is_live)
        .bind(gateway.is_active)
        .bind(&gateway.supported_methods)
        .bind(gateway.created_at)
        .bind(gateway.updated_at)
        .execute(&self.pool).await?;
        Ok(gateway)
    }

    pub async fn list_active(&self) -> Result<Vec<PaymentGateway>> {
        sqlx::query_as::<_, PaymentGateway>(
            "SELECT * FROM payment_gateways WHERE is_active = 1"
        )
        .fetch_all(&self.pool).await
        .map_err(|e| Error::Internal(e.into()))
    }
}

pub struct RefundRepository {
    pool: SqlitePool,
}

impl RefundRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, refund: Refund) -> Result<Refund> {
        sqlx::query(
            r#"INSERT INTO refunds (id, refund_number, payment_id, amount, currency, reason, status, gateway_refund_id, processed_at, created_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(refund.id)
        .bind(&refund.refund_number)
        .bind(refund.payment_id)
        .bind(refund.amount)
        .bind(&refund.currency)
        .bind(&refund.reason)
        .bind(&refund.status)
        .bind(&refund.gateway_refund_id)
        .bind(refund.processed_at)
        .bind(refund.created_at)
        .bind(refund.created_by)
        .execute(&self.pool).await?;
        Ok(refund)
    }
}

pub struct CustomerPaymentMethodRepository {
    pool: SqlitePool,
}

impl CustomerPaymentMethodRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, method: CustomerPaymentMethod) -> Result<CustomerPaymentMethod> {
        sqlx::query(
            r#"INSERT INTO customer_payment_methods (id, customer_id, payment_method, is_default, card_last_four, card_brand, card_expiry_month, card_expiry_year, card_holder_name, bank_name, bank_account_type, gateway_token, nickname, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(method.id)
        .bind(method.customer_id)
        .bind(&method.payment_method)
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
        .bind(method.created_at)
        .execute(&self.pool).await?;
        Ok(method)
    }
}

