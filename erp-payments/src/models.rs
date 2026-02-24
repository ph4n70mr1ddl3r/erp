use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartiallyRefunded,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    BankTransfer,
    ACH,
    WireTransfer,
    Check,
    Cash,
    PayPal,
    Stripe,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGateway {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub gateway_type: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub merchant_id: Option<String>,
    pub webhook_secret: Option<String>,
    pub is_live: bool,
    pub is_active: bool,
    pub supported_methods: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub payment_number: String,
    pub gateway_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub gateway_transaction_id: Option<String>,
    pub gateway_response: Option<String>,
    pub card_last_four: Option<String>,
    pub card_brand: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account_last_four: Option<String>,
    pub check_number: Option<String>,
    pub refunded_amount: i64,
    pub refund_reason: Option<String>,
    pub processing_fee: i64,
    pub notes: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAllocation {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub invoice_id: Uuid,
    pub amount: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id: Uuid,
    pub refund_number: String,
    pub payment_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub reason: String,
    pub status: String,
    pub gateway_refund_id: Option<String>,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPaymentMethod {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub payment_method: PaymentMethod,
    pub is_default: bool,
    pub card_last_four: Option<String>,
    pub card_brand: Option<String>,
    pub card_expiry_month: Option<i32>,
    pub card_expiry_year: Option<i32>,
    pub card_holder_name: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account_type: Option<String>,
    pub gateway_token: Option<String>,
    pub nickname: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBatch {
    pub id: Uuid,
    pub batch_number: String,
    pub gateway_id: Uuid,
    pub total_amount: i64,
    pub total_count: i32,
    pub currency: String,
    pub status: String,
    pub settled_at: Option<DateTime<Utc>>,
    pub settlement_reference: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub gateway_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub card_last_four: Option<String>,
    pub card_brand: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account_last_four: Option<String>,
    pub check_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPaymentRequest {
    pub gateway_id: Uuid,
    pub customer_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub payment_method_token: String,
    pub invoice_id: Option<Uuid>,
    pub description: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    pub payment_id: Uuid,
    pub amount: i64,
    pub reason: String,
}
