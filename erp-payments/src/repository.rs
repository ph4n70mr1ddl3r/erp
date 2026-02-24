use crate::models::*;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

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
        Ok(row.map(|r| Self::row_to_payment(&r)))
    }

    pub async fn list_by_customer(pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<Payment>> {
        let rows = sqlx::query(
            r#"SELECT id, payment_number, gateway_id, invoice_id, customer_id, amount, currency, payment_method, status, gateway_transaction_id, gateway_response, card_last_four, card_brand, bank_name, bank_account_last_four, check_number, refunded_amount, refund_reason, processing_fee, notes, paid_at, created_at, created_by FROM payments WHERE customer_id = ? ORDER BY created_at DESC"#
        )
        .bind(customer_id.to_string())
        .fetch_all(pool).await?;
        Ok(rows.iter().map(|r| Self::row_to_payment(&r)).collect())
    }

    fn row_to_payment(r: &sqlx::sqlite::SqliteRow) -> Payment {
        Payment {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str()).unwrap(),
            payment_number: r.get("payment_number"),
            gateway_id: r.get::<Option<String>, _>("gateway_id").and_then(|s| Uuid::parse_str(&s).ok()),
            invoice_id: r.get::<Option<String>, _>("invoice_id").and_then(|s| Uuid::parse_str(&s).ok()),
            customer_id: Uuid::parse_str(r.get::<String, _>("customer_id").as_str()).unwrap(),
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
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
            created_by: r.get::<Option<String>, _>("created_by").and_then(|s| Uuid::parse_str(&s).ok()),
        }
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
        Ok(rows.iter().map(|r| PaymentGateway {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str()).unwrap(),
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
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
        }).collect())
    }
}
