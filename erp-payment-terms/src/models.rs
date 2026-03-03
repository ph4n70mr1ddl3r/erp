use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTerm {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub due_days: i32,
    pub discount_days: Option<i32>,
    pub discount_percent: Option<f64>,
    pub is_default: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTermCalculation {
    pub term_id: Uuid,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub discount_date: Option<DateTime<Utc>>,
    pub discount_amount: Option<i64>,
}
