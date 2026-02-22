use chrono::{DateTime, Utc};
use erp_core::{Address, BaseEntity, ContactInfo, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vendor {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub contact: ContactInfo,
    pub address: Address,
    pub payment_terms: u32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrder {
    pub base: BaseEntity,
    pub po_number: String,
    pub vendor_id: Uuid,
    pub order_date: DateTime<Utc>,
    pub expected_date: Option<DateTime<Utc>>,
    pub lines: Vec<PurchaseOrderLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderLine {
    pub id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub tax_rate: f64,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseRequisition {
    pub base: BaseEntity,
    pub requisition_number: String,
    pub requested_by: Uuid,
    pub request_date: DateTime<Utc>,
    pub required_date: Option<DateTime<Utc>>,
    pub lines: Vec<RequisitionLine>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequisitionLine {
    pub id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodsReceipt {
    pub base: BaseEntity,
    pub receipt_number: String,
    pub purchase_order_id: Uuid,
    pub warehouse_id: Uuid,
    pub receipt_date: DateTime<Utc>,
    pub lines: Vec<GoodsReceiptLine>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodsReceiptLine {
    pub id: Uuid,
    pub po_line_id: Uuid,
    pub product_id: Uuid,
    pub quantity_ordered: i64,
    pub quantity_received: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierScorecard {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub period: String,
    pub on_time_delivery: i32,
    pub quality_score: i32,
    pub price_competitiveness: i32,
    pub responsiveness: i32,
    pub overall_score: i32,
    pub total_orders: i32,
    pub total_value: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorPerformance {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub order_id: Uuid,
    pub delivery_date: Option<DateTime<Utc>>,
    pub expected_date: Option<DateTime<Utc>>,
    pub on_time: bool,
    pub quality_rating: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
