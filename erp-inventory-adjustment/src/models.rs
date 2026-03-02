use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AdjustmentType {
    CountVariance,
    Damage,
    Theft,
    Expired,
    Obsolete,
    Found,
    TransferCorrection,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AdjustmentStatus {
    Draft,
    Pending,
    Approved,
    Rejected,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAdjustment {
    pub base: BaseEntity,
    pub adjustment_number: String,
    pub warehouse_id: Uuid,
    pub adjustment_type: AdjustmentType,
    pub reason: String,
    pub status: AdjustmentStatus,
    pub total_value_change: i64,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAdjustmentLine {
    pub id: Uuid,
    pub adjustment_id: Uuid,
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub system_quantity: i64,
    pub counted_quantity: i64,
    pub adjustment_quantity: i64,
    pub unit_cost: i64,
    pub total_value_change: i64,
    pub lot_number: Option<String>,
    pub serial_number: Option<String>,
    pub reason_code: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAdjustmentWithLines {
    pub adjustment: InventoryAdjustment,
    pub lines: Vec<InventoryAdjustmentLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAdjustmentRequest {
    pub warehouse_id: Uuid,
    pub adjustment_type: AdjustmentType,
    pub reason: String,
    pub notes: Option<String>,
    pub lines: Vec<CreateAdjustmentLineRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAdjustmentLineRequest {
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub system_quantity: i64,
    pub counted_quantity: i64,
    pub unit_cost: i64,
    pub lot_number: Option<String>,
    pub serial_number: Option<String>,
    pub reason_code: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustmentAnalytics {
    pub total_adjustments: i64,
    pub pending_adjustments: i64,
    pub completed_adjustments: i64,
    pub total_value_increase: i64,
    pub total_value_decrease: i64,
    pub adjustments_by_type: serde_json::Value,
    pub adjustments_by_month: serde_json::Value,
}
