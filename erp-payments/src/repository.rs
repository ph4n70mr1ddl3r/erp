use crate::models::*;
use anyhow::{Result, Context};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct StripeRepository;

impl StripeRepository {
    pub async fn create_payment_intent(pool: &SqlitePool, intent: &StripePaymentIntent) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO stripe_payment_intents (id, stripe_intent_id, customer_id, invoice_id, amount, currency, status, client_secret, description, metadata, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(intent.id.to_string())
        .bind(&intent.stripe_intent_id)
        .bind(intent.customer_id.to_string())
        .bind(intent.invoice_id.map(|id| id.to_string()))
        .bind(intent.amount)
        .bind(&intent.currency)
        .bind(&intent.status)
        .bind(&intent.client_secret)
        .bind(&intent.description)
        .bind(&intent.metadata)
        .bind(intent.created_at.to_rfc3339())
        .bind(intent.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }
    
    pub async fn get_payment_intent_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<StripePaymentIntent>> {
        let row = sqlx::query(
            r#"SELECT id, stripe_intent_id, customer_id, invoice_id, amount, currency, status, client_secret, description, metadata, created_at, updated_at FROM stripe_payment_intents WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        
        match row {
            Some(r) => Ok(Some(Self::row_to_payment_intent(&r)?)),
            None => Ok(None),
        }
    }
    
    pub async fn get_payment_intent_by_stripe_id(pool: &SqlitePool, stripe_id: &str) -> Result<Option<StripePaymentIntent>> {
        let row = sqlx::query(
            r#"SELECT id, stripe_intent_id, customer_id, invoice_id, amount, currency, status, client_secret, description, metadata, created_at, updated_at FROM stripe_payment_intents WHERE stripe_intent_id = ?"#
        )
        .bind(stripe_id)
        .fetch_optional(pool).await?;
        
        match row {
            Some(r) => Ok(Some(Self::row_to_payment_intent(&r)?)),
            None => Ok(None),
        }
    }
    
    pub async fn update_payment_intent_status(pool: &SqlitePool, intent: &StripePaymentIntent) -> Result<()> {
        sqlx::query(
            r#"UPDATE stripe_payment_intents SET status = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&intent.status)
        .bind(intent.updated_at.to_rfc3339())
        .bind(intent.id.to_string())
        .execute(pool).await?;
        Ok(())
    }
    
    pub async fn list_payment_intents_by_customer(pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<StripePaymentIntent>> {
        let rows = sqlx::query(
            r#"SELECT id, stripe_intent_id, customer_id, invoice_id, amount, currency, status, client_secret, description, metadata, created_at, updated_at FROM stripe_payment_intents WHERE customer_id = ? ORDER BY created_at DESC"#
        )
        .bind(customer_id.to_string())
        .fetch_all(pool).await?;
        
        rows.iter().map(Self::row_to_payment_intent).collect()
    }
    
    fn row_to_payment_intent(r: &sqlx::sqlite::SqliteRow) -> Result<StripePaymentIntent> {
        Ok(StripePaymentIntent {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str())
                .context("Failed to parse payment intent id")?,
            stripe_intent_id: r.get("stripe_intent_id"),
            customer_id: Uuid::parse_str(r.get::<String, _>("customer_id").as_str())
                .context("Failed to parse customer_id")?,
            invoice_id: r.get::<Option<String>, _>("invoice_id")
                .and_then(|s| Uuid::parse_str(&s).ok()),
            amount: r.get("amount"),
            currency: r.get("currency"),
            status: r.get("status"),
            client_secret: r.get("client_secret"),
            description: r.get("description"),
            metadata: r.get("metadata"),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                .context("Failed to parse created_at")?
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                .context("Failed to parse updated_at")?
                .with_timezone(&chrono::Utc),
        })
    }
    
    pub async fn create_checkout_session(pool: &SqlitePool, session: &StripeCheckoutSession) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO stripe_checkout_sessions (id, stripe_session_id, customer_id, invoice_id, amount, currency, status, checkout_url, success_url, cancel_url, payment_intent_id, expires_at, completed_at, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(session.id.to_string())
        .bind(&session.stripe_session_id)
        .bind(session.customer_id.to_string())
        .bind(session.invoice_id.map(|id| id.to_string()))
        .bind(session.amount)
        .bind(&session.currency)
        .bind(&session.status)
        .bind(&session.checkout_url)
        .bind(&session.success_url)
        .bind(&session.cancel_url)
        .bind(&session.payment_intent_id)
        .bind(session.expires_at.map(|d| d.to_rfc3339()))
        .bind(session.completed_at.map(|d| d.to_rfc3339()))
        .bind(session.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }
    
    pub async fn get_checkout_session_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<StripeCheckoutSession>> {
        let row = sqlx::query(
            r#"SELECT id, stripe_session_id, customer_id, invoice_id, amount, currency, status, checkout_url, success_url, cancel_url, payment_intent_id, expires_at, completed_at, created_at FROM stripe_checkout_sessions WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        
        match row {
            Some(r) => Ok(Some(Self::row_to_checkout_session(&r)?)),
            None => Ok(None),
        }
    }
    
    pub async fn get_checkout_session_by_stripe_id(pool: &SqlitePool, stripe_id: &str) -> Result<Option<StripeCheckoutSession>> {
        let row = sqlx::query(
            r#"SELECT id, stripe_session_id, customer_id, invoice_id, amount, currency, status, checkout_url, success_url, cancel_url, payment_intent_id, expires_at, completed_at, created_at FROM stripe_checkout_sessions WHERE stripe_session_id = ?"#
        )
        .bind(stripe_id)
        .fetch_optional(pool).await?;
        
        match row {
            Some(r) => Ok(Some(Self::row_to_checkout_session(&r)?)),
            None => Ok(None),
        }
    }
    
    pub async fn update_checkout_session_status(pool: &SqlitePool, session: &StripeCheckoutSession) -> Result<()> {
        sqlx::query(
            r#"UPDATE stripe_checkout_sessions SET status = ?, completed_at = ? WHERE id = ?"#
        )
        .bind(&session.status)
        .bind(session.completed_at.map(|d| d.to_rfc3339()))
        .bind(session.id.to_string())
        .execute(pool).await?;
        Ok(())
    }
    
    fn row_to_checkout_session(r: &sqlx::sqlite::SqliteRow) -> Result<StripeCheckoutSession> {
        Ok(StripeCheckoutSession {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str())
                .context("Failed to parse checkout session id")?,
            stripe_session_id: r.get("stripe_session_id"),
            customer_id: Uuid::parse_str(r.get::<String, _>("customer_id").as_str())
                .context("Failed to parse customer_id")?,
            invoice_id: r.get::<Option<String>, _>("invoice_id").and_then(|s| Uuid::parse_str(&s).ok()),
            amount: r.get("amount"),
            currency: r.get("currency"),
            status: r.get("status"),
            checkout_url: r.get("checkout_url"),
            success_url: r.get("success_url"),
            cancel_url: r.get("cancel_url"),
            payment_intent_id: r.get("payment_intent_id"),
            expires_at: r.get::<Option<String>, _>("expires_at").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            completed_at: r.get::<Option<String>, _>("completed_at").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                .context("Failed to parse created_at")?
                .with_timezone(&chrono::Utc),
        })
    }
    
    pub async fn create_webhook_event(pool: &SqlitePool, event: &StripeWebhookEvent) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO stripe_webhook_events (id, stripe_event_id, event_type, payload, processed, processed_at, error_message, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(event.id.to_string())
        .bind(&event.stripe_event_id)
        .bind(&event.event_type)
        .bind(&event.payload)
        .bind(event.processed)
        .bind(event.processed_at.map(|d| d.to_rfc3339()))
        .bind(&event.error_message)
        .bind(event.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }
}

pub struct PaymentRepository;

impl PaymentRepository {
    pub async fn create(pool: &SqlitePool, payment: &Payment) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO payments (id, payment_number, gateway_id, invoice_id, customer_id, amount, currency, payment_method, status, gateway_transaction_id, gateway_response, card_last_four, card_brand, bank_name, bank_account_last_four, check_number, refunded_amount, refund_reason, processing_fee, notes, paid_at, created_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(payment.id.to_string())
        .bind(&payment.payment_number)
        .bind(payment.gateway_id.map(|id| id.to_string()))
        .bind(payment.invoice_id.map(|id| id.to_string()))
        .bind(payment.customer_id.to_string())
        .bind(payment.amount)
        .bind(&payment.currency)
        .bind(format!("{:?}", payment.payment_method))
        .bind(format!("{:?}", payment.status))
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
        .bind(payment.paid_at.map(|d| d.to_rfc3339()))
        .bind(payment.created_at.to_rfc3339())
        .bind(payment.created_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(())
    }

    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Payment>> {
        let row = sqlx::query(
            r#"SELECT id, payment_number, gateway_id, invoice_id, customer_id, amount, currency, payment_method, status, gateway_transaction_id, gateway_response, card_last_four, card_brand, bank_name, bank_account_last_four, check_number, refunded_amount, refund_reason, processing_fee, notes, paid_at, created_at, created_by FROM payments WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        match row {
            Some(r) => Ok(Some(Self::row_to_payment(&r)?)),
            None => Ok(None),
        }
    }

    pub async fn list_by_customer(pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<Payment>> {
        let rows = sqlx::query(
            r#"SELECT id, payment_number, gateway_id, invoice_id, customer_id, amount, currency, payment_method, status, gateway_transaction_id, gateway_response, card_last_four, card_brand, bank_name, bank_account_last_four, check_number, refunded_amount, refund_reason, processing_fee, notes, paid_at, created_at, created_by FROM payments WHERE customer_id = ? ORDER BY created_at DESC"#
        )
        .bind(customer_id.to_string())
        .fetch_all(pool).await?;
        rows.iter().map(Self::row_to_payment).collect()
    }

    fn row_to_payment(r: &sqlx::sqlite::SqliteRow) -> Result<Payment> {
        Ok(Payment {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str())
                .context("Failed to parse payment id")?,
            payment_number: r.get("payment_number"),
            gateway_id: r.get::<Option<String>, _>("gateway_id").and_then(|s| Uuid::parse_str(&s).ok()),
            invoice_id: r.get::<Option<String>, _>("invoice_id").and_then(|s| Uuid::parse_str(&s).ok()),
            customer_id: Uuid::parse_str(r.get::<String, _>("customer_id").as_str())
                .context("Failed to parse customer_id")?,
            amount: r.get("amount"),
            currency: r.get("currency"),
            payment_method: match r.get::<String, _>("payment_method").as_str() {
                "CreditCard" => PaymentMethod::CreditCard,
                "DebitCard" => PaymentMethod::DebitCard,
                "BankTransfer" => PaymentMethod::BankTransfer,
                "ACH" => PaymentMethod::ACH,
                "WireTransfer" => PaymentMethod::WireTransfer,
                "Check" => PaymentMethod::Check,
                "Cash" => PaymentMethod::Cash,
                "PayPal" => PaymentMethod::PayPal,
                "Stripe" => PaymentMethod::Stripe,
                _ => PaymentMethod::Other,
            },
            status: match r.get::<String, _>("status").as_str() {
                "Processing" => PaymentStatus::Processing,
                "Completed" => PaymentStatus::Completed,
                "Failed" => PaymentStatus::Failed,
                "Cancelled" => PaymentStatus::Cancelled,
                "Refunded" => PaymentStatus::Refunded,
                "PartiallyRefunded" => PaymentStatus::PartiallyRefunded,
                _ => PaymentStatus::Pending,
            },
            gateway_transaction_id: r.get("gateway_transaction_id"),
            gateway_response: r.get("gateway_response"),
            card_last_four: r.get("card_last_four"),
            card_brand: r.get("card_brand"),
            bank_name: r.get("bank_name"),
            bank_account_last_four: r.get("bank_account_last_four"),
            check_number: r.get("check_number"),
            refunded_amount: r.get("refunded_amount"),
            refund_reason: r.get("refund_reason"),
            processing_fee: r.get("processing_fee"),
            notes: r.get("notes"),
            paid_at: r.get::<Option<String>, _>("paid_at").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                .context("Failed to parse created_at")?
                .with_timezone(&chrono::Utc),
            created_by: r.get::<Option<String>, _>("created_by").and_then(|s| Uuid::parse_str(&s).ok()),
        })
    }
}

pub struct RefundRepository;

impl RefundRepository {
    pub async fn create(pool: &SqlitePool, refund: &Refund) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO refunds (id, refund_number, payment_id, amount, currency, reason, status, gateway_refund_id, processed_at, created_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(refund.id.to_string())
        .bind(&refund.refund_number)
        .bind(refund.payment_id.to_string())
        .bind(refund.amount)
        .bind(&refund.currency)
        .bind(&refund.reason)
        .bind(&refund.status)
        .bind(&refund.gateway_refund_id)
        .bind(refund.processed_at.map(|d| d.to_rfc3339()))
        .bind(refund.created_at.to_rfc3339())
        .bind(refund.created_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(())
    }
}

pub struct GatewayRepository;

impl GatewayRepository {
    pub async fn create(pool: &SqlitePool, gateway: &PaymentGateway) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO payment_gateways (id, code, name, gateway_type, api_key, api_secret, merchant_id, webhook_secret, is_live, is_active, supported_methods, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(gateway.id.to_string())
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
        .bind(gateway.created_at.to_rfc3339())
        .bind(gateway.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }

    pub async fn list_active(pool: &SqlitePool) -> Result<Vec<PaymentGateway>> {
        let rows = sqlx::query(
            r#"SELECT id, code, name, gateway_type, api_key, api_secret, merchant_id, webhook_secret, is_live, is_active, supported_methods, created_at, updated_at FROM payment_gateways WHERE is_active = 1"#
        )
        .fetch_all(pool).await?;
        rows.iter().map(|r| {
            Ok(PaymentGateway {
                id: Uuid::parse_str(r.get::<String, _>("id").as_str())
                    .context("Failed to parse gateway id")?,
                code: r.get("code"),
                name: r.get("name"),
                gateway_type: r.get("gateway_type"),
                api_key: r.get("api_key"),
                api_secret: r.get("api_secret"),
                merchant_id: r.get("merchant_id"),
                webhook_secret: r.get("webhook_secret"),
                is_live: r.get("is_live"),
                is_active: r.get("is_active"),
                supported_methods: r.get("supported_methods"),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                    .context("Failed to parse created_at")?
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                    .context("Failed to parse updated_at")?
                    .with_timezone(&chrono::Utc),
            })
        }).collect()
    }
}
