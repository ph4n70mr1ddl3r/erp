use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WarrantyType {
    Standard,
    Extended,
    Lifetime,
    ProRated,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WarrantyDurationUnit {
    Days,
    Months,
    Years,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WarrantyClaimStatus {
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ClaimResolutionType {
    Repair,
    Replacement,
    Refund,
    Credit,
    PartialRefund,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum WarrantyStatus {
    Active,
    Expired,
    Voided,
    Claimed,
    Transferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyPolicy {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub warranty_type: WarrantyType,
    pub duration_value: i32,
    pub duration_unit: WarrantyDurationUnit,
    pub coverage_percentage: f64,
    pub labor_covered: bool,
    pub parts_covered: bool,
    pub on_site_service: bool,
    pub max_claims: Option<i32>,
    pub deductible_amount: i64,
    pub terms_and_conditions: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductWarranty {
    pub base: BaseEntity,
    pub warranty_number: String,
    pub policy_id: Uuid,
    pub product_id: Uuid,
    pub customer_id: Uuid,
    pub sales_order_id: Option<Uuid>,
    pub sales_order_line_id: Option<Uuid>,
    pub serial_number: Option<String>,
    pub lot_number: Option<String>,
    pub purchase_date: DateTime<Utc>,
    pub activation_date: Option<DateTime<Utc>>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: WarrantyStatus,
    pub transferred_to_customer_id: Option<Uuid>,
    pub transferred_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyClaim {
    pub base: BaseEntity,
    pub claim_number: String,
    pub product_warranty_id: Uuid,
    pub customer_id: Uuid,
    pub reported_date: DateTime<Utc>,
    pub issue_description: String,
    pub issue_category: Option<String>,
    pub symptom_codes: Option<String>,
    pub status: WarrantyClaimStatus,
    pub priority: i32,
    pub assigned_to: Option<Uuid>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub resolution_type: Option<ClaimResolutionType>,
    pub resolution_notes: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub customer_notified: bool,
    pub notification_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyClaimLine {
    pub id: Uuid,
    pub claim_id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_cost: i64,
    pub total_cost: i64,
    pub coverage_percentage: f64,
    pub covered_amount: i64,
    pub customer_amount: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyClaimLabor {
    pub id: Uuid,
    pub claim_id: Uuid,
    pub technician_id: Option<Uuid>,
    pub work_description: String,
    pub labor_hours: f64,
    pub hourly_rate: i64,
    pub total_cost: i64,
    pub covered_amount: i64,
    pub customer_amount: i64,
    pub work_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyRegistration {
    pub base: BaseEntity,
    pub product_warranty_id: Uuid,
    pub customer_id: Uuid,
    pub registration_date: DateTime<Utc>,
    pub registration_source: String,
    pub verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyExtension {
    pub base: BaseEntity,
    pub product_warranty_id: Uuid,
    pub policy_id: Uuid,
    pub extension_date: DateTime<Utc>,
    pub additional_duration_value: i32,
    pub additional_duration_unit: WarrantyDurationUnit,
    pub new_end_date: DateTime<Utc>,
    pub cost: i64,
    pub invoice_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarrantyAnalytics {
    pub total_warranties: i64,
    pub active_warranties: i64,
    pub expired_warranties: i64,
    pub total_claims: i64,
    pub open_claims: i64,
    pub approved_claims: i64,
    pub rejected_claims: i64,
    pub total_claim_cost: i64,
    pub average_resolution_days: f64,
    pub claims_by_category: serde_json::Value,
    pub claims_by_month: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarrantyPolicyRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub warranty_type: WarrantyType,
    pub duration_value: i32,
    pub duration_unit: WarrantyDurationUnit,
    pub coverage_percentage: f64,
    pub labor_covered: bool,
    pub parts_covered: bool,
    pub on_site_service: bool,
    pub max_claims: Option<i32>,
    pub deductible_amount: i64,
    pub terms_and_conditions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductWarrantyRequest {
    pub policy_id: Uuid,
    pub product_id: Uuid,
    pub customer_id: Uuid,
    pub sales_order_id: Option<Uuid>,
    pub sales_order_line_id: Option<Uuid>,
    pub serial_number: Option<String>,
    pub lot_number: Option<String>,
    pub purchase_date: DateTime<Utc>,
    pub activation_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWarrantyClaimRequest {
    pub product_warranty_id: Uuid,
    pub customer_id: Uuid,
    pub reported_date: DateTime<Utc>,
    pub issue_description: String,
    pub issue_category: Option<String>,
    pub symptom_codes: Option<String>,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddClaimLineRequest {
    pub claim_id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_cost: i64,
    pub coverage_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddClaimLaborRequest {
    pub claim_id: Uuid,
    pub technician_id: Option<Uuid>,
    pub work_description: String,
    pub labor_hours: f64,
    pub hourly_rate: i64,
    pub work_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveClaimRequest {
    pub resolution_type: ClaimResolutionType,
    pub resolution_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferWarrantyRequest {
    pub new_customer_id: Uuid,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendWarrantyRequest {
    pub policy_id: Uuid,
    pub additional_duration_value: i32,
    pub additional_duration_unit: WarrantyDurationUnit,
    pub cost: i64,
    pub invoice_id: Option<Uuid>,
}
