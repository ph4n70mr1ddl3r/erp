use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TransferStatus {
    Draft,
    Pending,
    Approved,
    InTransit,
    Received,
    PartiallyReceived,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TransferPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockTransfer {
    pub base: BaseEntity,
    pub transfer_number: String,
    pub from_warehouse_id: Uuid,
    pub to_warehouse_id: Uuid,
    pub status: TransferStatus,
    pub priority: TransferPriority,
    pub requested_date: Option<DateTime<Utc>>,
    pub expected_date: Option<DateTime<Utc>>,
    pub shipped_date: Option<DateTime<Utc>>,
    pub received_date: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub shipped_by: Option<Uuid>,
    pub received_by: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockTransferLine {
    pub id: Uuid,
    pub transfer_id: Uuid,
    pub product_id: Uuid,
    pub requested_quantity: i64,
    pub shipped_quantity: i64,
    pub received_quantity: i64,
    pub unit_cost: i64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockTransferWithLines {
    pub transfer: StockTransfer,
    pub lines: Vec<StockTransferLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransferRequest {
    pub from_warehouse_id: Uuid,
    pub to_warehouse_id: Uuid,
    pub priority: TransferPriority,
    pub requested_date: Option<DateTime<Utc>>,
    pub expected_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub lines: Vec<CreateTransferLineRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransferLineRequest {
    pub product_id: Uuid,
    pub requested_quantity: i64,
    pub unit_cost: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipTransferRequest {
    pub lines: Vec<ShipLineRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipLineRequest {
    pub product_id: Uuid,
    pub shipped_quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveTransferRequest {
    pub lines: Vec<ReceiveLineRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveLineRequest {
    pub product_id: Uuid,
    pub received_quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAnalytics {
    pub total_transfers: i64,
    pub pending_transfers: i64,
    pub in_transit: i64,
    pub completed_transfers: i64,
    pub total_value_in_transit: i64,
    pub transfers_by_status: serde_json::Value,
    pub average_transfer_time_hours: f64,
}
