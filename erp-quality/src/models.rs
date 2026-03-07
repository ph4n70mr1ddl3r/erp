use chrono::{DateTime, NaiveDate, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum InspectionType {
    Incoming,
    InProcess,
    Final,
    Outgoing,
    Supplier,
    Customer,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum InspectionStatus {
    Pending,
    InProgress,
    Passed,
    Failed,
    Partial,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum InspectionResult {
    Pass,
    Fail,
    ConditionalPass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityInspection {
    pub base: BaseEntity,
    pub inspection_number: String,
    pub inspection_type: InspectionType,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub inspector_id: Option<Uuid>,
    pub inspection_date: NaiveDate,
    pub status: InspectionStatus,
    pub result: Option<InspectionResult>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionItem {
    pub id: Uuid,
    pub inspection_id: Uuid,
    pub criterion: String,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
    pub pass_fail: Option<bool>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityInspectionWithItems {
    pub inspection: QualityInspection,
    pub items: Vec<InspectionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInspectionRequest {
    pub inspection_type: InspectionType,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub inspector_id: Option<Uuid>,
    pub inspection_date: NaiveDate,
    pub notes: Option<String>,
    pub items: Vec<CreateInspectionItemRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInspectionItemRequest {
    pub criterion: String,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
    pub pass_fail: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInspectionItemRequest {
    pub actual_value: Option<String>,
    pub pass_fail: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum NCRSource {
    IncomingInspection,
    InProcessInspection,
    FinalInspection,
    CustomerComplaint,
    InternalAudit,
    SupplierIssue,
    ProductionIssue,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum NCRSeverity {
    Minor,
    Major,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum NCRStatus {
    Open,
    UnderInvestigation,
    CorrectiveAction,
    Verification,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonConformanceReport {
    pub base: BaseEntity,
    pub ncr_number: String,
    pub source_type: NCRSource,
    pub source_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub severity: NCRSeverity,
    pub status: NCRStatus,
    pub assigned_to: Option<Uuid>,
    pub root_cause: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub resolution_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNCRRequest {
    pub source_type: NCRSource,
    pub source_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub severity: NCRSeverity,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNCRRequest {
    pub root_cause: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub status: Option<NCRStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAnalytics {
    pub total_inspections: i64,
    pub passed_inspections: i64,
    pub failed_inspections: i64,
    pub pass_rate: f64,
    pub total_ncrs: i64,
    pub open_ncrs: i64,
    pub closed_ncrs: i64,
    pub ncrs_by_severity: serde_json::Value,
    pub inspections_by_type: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum CalibrationStatus {
    Pending,
    InProgress,
    Passed,
    Failed,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationDevice {
    pub base: BaseEntity,
    pub device_number: String,
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub location: Option<String>,
    pub calibration_frequency_days: i32,
    pub last_calibration_date: Option<NaiveDate>,
    pub next_calibration_date: Option<NaiveDate>,
    pub status: CalibrationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationRecord {
    pub base: BaseEntity,
    pub record_number: String,
    pub device_id: Uuid,
    pub calibration_date: NaiveDate,
    pub calibrated_by: Option<Uuid>,
    pub status: CalibrationStatus,
    pub certificate_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationReading {
    pub id: Uuid,
    pub record_id: Uuid,
    pub parameter: String,
    pub reference_value: f64,
    pub actual_value: f64,
    pub tolerance_min: f64,
    pub tolerance_max: f64,
    pub pass_fail: bool,
    pub uom: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationRecordWithReadings {
    pub record: CalibrationRecord,
    pub readings: Vec<CalibrationReading>,
}
