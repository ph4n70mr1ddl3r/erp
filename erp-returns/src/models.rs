use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReturnType {
    CustomerReturn,
    VendorReturn,
    InternalReturn,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReturnReason {
    Defective,
    WrongItem,
    NotAsDescribed,
    Damaged,
    ChangedMind,
    Warranty,
    Recall,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReturnStatus {
    Draft,
    Requested,
    Approved,
    Received,
    Inspected,
    Processed,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReturnDisposition {
    Restock,
    Refund,
    Replace,
    Repair,
    Scrap,
    ReturnToVendor,
    Credit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnOrder {
    pub base: BaseEntity,
    pub return_number: String,
    pub return_type: ReturnType,
    pub customer_id: Option<Uuid>,
    pub vendor_id: Option<Uuid>,
    pub original_order_id: Option<Uuid>,
    pub original_invoice_id: Option<Uuid>,
    pub request_date: DateTime<Utc>,
    pub received_date: Option<DateTime<Utc>>,
    pub processed_date: Option<DateTime<Utc>>,
    pub reason: ReturnReason,
    pub notes: Option<String>,
    pub lines: Vec<ReturnLine>,
    pub status: ReturnStatus,
    pub total_credit: Money,
    pub warehouse_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnLine {
    pub id: Uuid,
    pub return_order_id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity_requested: i64,
    pub quantity_received: i64,
    pub quantity_approved: i64,
    pub unit_price: Money,
    pub reason: ReturnReason,
    pub disposition: ReturnDisposition,
    pub condition: ItemCondition,
    pub inspection_notes: Option<String>,
    pub credit_amount: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ItemCondition {
    New,
    UsedGood,
    UsedFair,
    Damaged,
    Defective,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnInspection {
    pub id: Uuid,
    pub return_order_id: Uuid,
    pub return_line_id: Uuid,
    pub inspector_id: Option<Uuid>,
    pub inspection_date: DateTime<Utc>,
    pub condition: ItemCondition,
    pub passes_quality: bool,
    pub notes: Option<String>,
    pub disposition: ReturnDisposition,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditMemo {
    pub base: BaseEntity,
    pub memo_number: String,
    pub customer_id: Uuid,
    pub return_order_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub memo_date: DateTime<Utc>,
    pub lines: Vec<CreditMemoLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub status: CreditMemoStatus,
    pub applied_amount: Money,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditMemoLine {
    pub id: Uuid,
    pub credit_memo_id: Uuid,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditMemoStatus {
    Draft,
    Issued,
    Applied,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RefundMethod {
    OriginalPayment,
    Check,
    BankTransfer,
    StoreCredit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id: Uuid,
    pub refund_number: String,
    pub customer_id: Uuid,
    pub credit_memo_id: Option<Uuid>,
    pub return_order_id: Option<Uuid>,
    pub refund_date: DateTime<Utc>,
    pub amount: Money,
    pub method: RefundMethod,
    pub reference: Option<String>,
    pub status: RefundStatus,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RefundStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPolicy {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub return_window_days: i32,
    pub requires_receipt: bool,
    pub requires_original_packaging: bool,
    pub restocking_fee_percent: f64,
    pub allows_exchange: bool,
    pub allows_refund: bool,
    pub allows_store_credit: bool,
    pub excluded_categories: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}
