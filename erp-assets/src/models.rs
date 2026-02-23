use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ITAsset {
    pub base: BaseEntity,
    pub asset_tag: String,
    pub name: String,
    pub description: Option<String>,
    pub asset_type: ITAssetType,
    pub status: ITAssetStatus,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub purchase_cost: i64,
    pub currency: String,
    pub warranty_expiry: Option<NaiveDate>,
    pub location_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub assigned_date: Option<NaiveDate>,
    pub department_id: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ITAssetType {
    Hardware,
    Software,
    Network,
    Peripheral,
    Mobile,
    Server,
    Storage,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ITAssetStatus {
    Available,
    InUse,
    InMaintenance,
    Reserved,
    Retired,
    Lost,
    Disposed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareLicense {
    pub id: Uuid,
    pub license_key: String,
    pub product_name: String,
    pub vendor: String,
    pub license_type: LicenseType,
    pub seats_purchased: i32,
    pub seats_used: i32,
    pub purchase_date: NaiveDate,
    pub purchase_cost: i64,
    pub currency: String,
    pub start_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub auto_renew: bool,
    pub support_expiry: Option<NaiveDate>,
    pub status: Status,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LicenseType {
    Perpetual,
    Subscription,
    Volume,
    Site,
    Concurrent,
    NamedUser,
    Oem,
    Trial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareInstallation {
    pub id: Uuid,
    pub license_id: Uuid,
    pub asset_id: Uuid,
    pub installed_by: Option<Uuid>,
    pub installed_at: DateTime<Utc>,
    pub version: Option<String>,
    pub status: InstallationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum InstallationStatus {
    Installed,
    Uninstalled,
    Upgraded,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAssignment {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub assigned_to: Uuid,
    pub assigned_by: Uuid,
    pub assigned_at: DateTime<Utc>,
    pub expected_return: Option<NaiveDate>,
    pub returned_at: Option<DateTime<Utc>>,
    pub returned_by: Option<Uuid>,
    pub notes: Option<String>,
    pub status: AssignmentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AssignmentStatus {
    Active,
    Returned,
    Overdue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMaintenance {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub maintenance_type: MaintenanceType,
    pub description: String,
    pub scheduled_date: NaiveDate,
    pub performed_date: Option<NaiveDate>,
    pub performed_by: Option<Uuid>,
    pub cost: i64,
    pub currency: String,
    pub status: MaintenanceStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenanceType {
    Preventive,
    Corrective,
    Upgrade,
    Calibration,
    Inspection,
    Replacement,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenanceStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Overdue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDepreciation {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub depreciation_method: DepreciationMethod,
    pub useful_life_months: i32,
    pub salvage_value: i64,
    pub current_value: i64,
    pub accumulated_depreciation: i64,
    pub last_depreciation_date: Option<NaiveDate>,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DepreciationMethod {
    StraightLine,
    DecliningBalance,
    SumOfYearsDigits,
    UnitsOfProduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDisposal {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub disposal_type: DisposalType,
    pub disposal_date: NaiveDate,
    pub reason: String,
    pub proceeds: i64,
    pub currency: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DisposalType {
    Sold,
    Donated,
    Recycled,
    Scrapped,
    TradedIn,
    Lost,
    Stolen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetLocation {
    pub id: Uuid,
    pub name: String,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub room: Option<String>,
    pub address: Option<String>,
    pub parent_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub default_depreciation_method: Option<DepreciationMethod>,
    pub default_useful_life_months: Option<i32>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorContract {
    pub id: Uuid,
    pub vendor_name: String,
    pub contract_number: String,
    pub contract_type: ContractType,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub value: i64,
    pub currency: String,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub terms: Option<String>,
    pub auto_renew: bool,
    pub renewal_notice_days: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContractType {
    Support,
    Lease,
    Rental,
    Maintenance,
    Warranty,
    ServiceLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAsset {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub hostname: Option<String>,
    pub domain: Option<String>,
    pub network_segment: Option<String>,
    pub vlan: Option<i32>,
    pub port: Option<String>,
    pub switch_port: Option<String>,
    pub dns_servers: Option<String>,
    pub gateway: Option<String>,
    pub subnet_mask: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAsset {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub security_level: SecurityLevel,
    pub data_classification: DataClassification,
    pub encryption_status: bool,
    pub antivirus_installed: bool,
    pub antivirus_updated: Option<NaiveDate>,
    pub last_security_scan: Option<DateTime<Utc>>,
    pub vulnerabilities_found: i32,
    pub vulnerabilities_fixed: i32,
    pub compliance_status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SecurityLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    Pii,
    Phi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAudit {
    pub id: Uuid,
    pub audit_date: NaiveDate,
    pub auditor: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub total_assets: i32,
    pub verified_assets: i32,
    pub missing_assets: i32,
    pub extra_assets: i32,
    pub status: AuditStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AuditStatus {
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAuditItem {
    pub id: Uuid,
    pub audit_id: Uuid,
    pub asset_id: Uuid,
    pub expected_location_id: Option<Uuid>,
    pub actual_location_id: Option<Uuid>,
    pub expected_assignee_id: Option<Uuid>,
    pub actual_assignee_id: Option<Uuid>,
    pub status: AuditItemStatus,
    pub notes: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AuditItemStatus {
    Verified,
    Missing,
    WrongLocation,
    WrongAssignee,
    NotExpected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRelationship {
    pub id: Uuid,
    pub parent_asset_id: Uuid,
    pub child_asset_id: Uuid,
    pub relationship_type: AssetRelationshipType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssetRelationshipType {
    Contains,
    ConnectedTo,
    DependsOn,
    Powers,
    BacksUp,
    Virtualizes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCheckout {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub checked_out_to: Uuid,
    pub checked_out_by: Uuid,
    pub checked_out_at: DateTime<Utc>,
    pub expected_return: NaiveDate,
    pub actual_return: Option<DateTime<Utc>>,
    pub returned_to: Option<Uuid>,
    pub condition_on_checkout: AssetCondition,
    pub condition_on_return: Option<AssetCondition>,
    pub notes: Option<String>,
    pub status: CheckoutStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssetCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    Damaged,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum CheckoutStatus {
    Active,
    Returned,
    Overdue,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareMeter {
    pub id: Uuid,
    pub license_id: Uuid,
    pub meter_date: NaiveDate,
    pub peak_usage: i32,
    pub avg_usage: f64,
    pub total_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDocument {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub document_type: AssetDocumentType,
    pub title: String,
    pub description: Option<String>,
    pub file_path: String,
    pub file_size: i64,
    pub uploaded_by: Uuid,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssetDocumentType {
    Invoice,
    Warranty,
    Manual,
    License,
    Configuration,
    Photo,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetric {
    pub id: Uuid,
    pub metric_date: NaiveDate,
    pub total_assets: i32,
    pub assets_in_use: i32,
    pub assets_available: i32,
    pub assets_in_maintenance: i32,
    pub assets_retired: i32,
    pub total_value: i64,
    pub currency: String,
    pub total_depreciation: i64,
}
