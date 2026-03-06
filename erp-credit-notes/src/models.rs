use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNote {
    pub base: BaseEntity,
    pub credit_note_number: String,
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub credit_note_date: DateTime<Utc>,
    pub lines: Vec<CreditNoteLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub reason: CreditNoteReason,
    pub notes: Option<String>,
    pub status: CreditNoteStatus,
    pub applied_amount: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNoteLine {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditNoteReason {
    Return,
    Damaged,
    WrongItem,
    PricingError,
    QualityIssue,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum CreditNoteStatus {
    Draft,
    Issued,
    Applied,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNoteApplication {
    pub id: Uuid,
    pub credit_note_id: Uuid,
    pub invoice_id: Uuid,
    pub amount: Money,
    pub applied_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCreditNoteRequest {
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub credit_note_date: Option<DateTime<Utc>>,
    pub lines: Vec<CreateCreditNoteLineRequest>,
    pub reason: CreditNoteReason,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCreditNoteLineRequest {
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyCreditNoteRequest {
    pub invoice_id: Uuid,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNoteResponse {
    pub id: Uuid,
    pub credit_note_number: String,
    pub customer_id: Uuid,
    pub customer_name: Option<String>,
    pub invoice_id: Option<Uuid>,
    pub invoice_number: Option<String>,
    pub credit_note_date: DateTime<Utc>,
    pub lines: Vec<CreditNoteLineResponse>,
    pub subtotal: i64,
    pub tax_amount: i64,
    pub total: i64,
    pub currency: String,
    pub reason: CreditNoteReason,
    pub notes: Option<String>,
    pub status: CreditNoteStatus,
    pub applied_amount: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNoteLineResponse {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub line_total: i64,
}

impl From<CreditNote> for CreditNoteResponse {
    fn from(cn: CreditNote) -> Self {
        Self {
            id: cn.base.id,
            credit_note_number: cn.credit_note_number,
            customer_id: cn.customer_id,
            customer_name: None,
            invoice_id: cn.invoice_id,
            invoice_number: None,
            credit_note_date: cn.credit_note_date,
            lines: cn.lines.into_iter().map(|l| l.into()).collect(),
            subtotal: cn.subtotal.amount,
            tax_amount: cn.tax_amount.amount,
            total: cn.total.amount,
            currency: cn.total.currency.to_string(),
            reason: cn.reason,
            notes: cn.notes,
            status: cn.status,
            applied_amount: cn.applied_amount.amount,
            created_at: cn.base.created_at,
            updated_at: cn.base.updated_at,
        }
    }
}

impl From<CreditNoteLine> for CreditNoteLineResponse {
    fn from(line: CreditNoteLine) -> Self {
        Self {
            id: line.id,
            product_id: line.product_id,
            description: line.description,
            quantity: line.quantity,
            unit_price: line.unit_price.amount,
            line_total: line.line_total.amount,
        }
    }
}
