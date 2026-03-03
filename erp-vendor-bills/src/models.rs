use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum VendorBillStatus {
    #[default]
    Draft,
    Pending,
    Approved,
    PartiallyPaid,
    Paid,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MatchStatus {
    #[default]
    Unmatched,
    PartiallyMatched,
    FullyMatched,
    Exception,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorBill {
    pub base: BaseEntity,
    pub bill_number: String,
    pub vendor_invoice_number: String,
    pub vendor_id: Uuid,
    pub purchase_order_id: Option<Uuid>,
    pub bill_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub lines: Vec<VendorBillLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub amount_paid: Money,
    pub status: VendorBillStatus,
    pub match_status: MatchStatus,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorBillLine {
    pub id: Uuid,
    pub bill_id: Uuid,
    pub po_line_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub tax_rate: f64,
    pub line_total: Money,
    pub match_quantity: i64,
    pub match_status: MatchStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorBillPayment {
    pub id: Uuid,
    pub bill_id: Uuid,
    pub payment_id: Uuid,
    pub amount: Money,
    pub applied_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeWayMatchResult {
    pub bill_id: Uuid,
    pub po_id: Option<Uuid>,
    pub total_matched_lines: i32,
    pub total_unmatched_lines: i32,
    pub total_exceptions: i32,
    pub match_status: MatchStatus,
    pub exceptions: Vec<MatchException>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchException {
    pub bill_line_id: Uuid,
    pub exception_type: MatchExceptionType,
    pub expected_value: String,
    pub actual_value: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchExceptionType {
    QuantityVariance,
    PriceVariance,
    MissingPO,
    MissingReceipt,
    MissingProduct,
    ProductMismatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorBillSummary {
    pub total_bills: i64,
    pub total_amount: i64,
    pub total_paid: i64,
    pub total_outstanding: i64,
    pub overdue_count: i64,
    pub overdue_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorBillCreateRequest {
    pub vendor_invoice_number: String,
    pub vendor_id: Uuid,
    pub purchase_order_id: Option<Uuid>,
    pub bill_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub lines: Vec<VendorBillLineCreateRequest>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorBillLineCreateRequest {
    pub po_line_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub tax_rate: f64,
}
