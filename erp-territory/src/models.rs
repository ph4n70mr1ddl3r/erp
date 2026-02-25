use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesTerritory {
    pub base: BaseEntity,
    pub territory_number: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_territory_id: Option<Uuid>,
    pub territory_type: TerritoryType,
    pub manager_id: Option<Uuid>,
    pub geography: Option<String>,
    pub countries: Option<String>,
    pub states: Option<String>,
    pub cities: Option<String>,
    pub postal_codes: Option<String>,
    pub industries: Option<String>,
    pub company_size: Option<String>,
    pub custom_criteria: Option<String>,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TerritoryType {
    Geographic,
    Industry,
    Product,
    Customer,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryAssignment {
    pub id: Uuid,
    pub territory_id: Uuid,
    pub sales_rep_id: Uuid,
    pub assignment_type: AssignmentType,
    pub primary_rep: bool,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub allocation_percent: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssignmentType {
    Primary,
    Overlay,
    Shared,
    AccountBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesQuota {
    pub base: BaseEntity,
    pub quota_number: String,
    pub name: String,
    pub description: Option<String>,
    pub quota_type: QuotaType,
    pub owner_type: QuotaOwnerType,
    pub owner_id: Uuid,
    pub territory_id: Option<Uuid>,
    pub fiscal_year: i32,
    pub period_type: QuotaPeriodType,
    pub currency: String,
    pub annual_target: i64,
    pub q1_target: i64,
    pub q2_target: i64,
    pub q3_target: i64,
    pub q4_target: i64,
    pub m1_target: i64,
    pub m2_target: i64,
    pub m3_target: i64,
    pub m4_target: i64,
    pub m5_target: i64,
    pub m6_target: i64,
    pub m7_target: i64,
    pub m8_target: i64,
    pub m9_target: i64,
    pub m10_target: i64,
    pub m11_target: i64,
    pub m12_target: i64,
    pub product_id: Option<Uuid>,
    pub product_category_id: Option<Uuid>,
    pub status: QuotaStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QuotaType {
    Revenue,
    Units,
    Margin,
    NewCustomers,
    Renewals,
    Pipeline,
    Activities,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QuotaOwnerType {
    SalesRep,
    Territory,
    Team,
    Region,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QuotaPeriodType {
    Annual,
    Quarterly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QuotaStatus {
    Draft,
    Submitted,
    Approved,
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaAttainment {
    pub base: BaseEntity,
    pub quota_id: Uuid,
    pub period_type: QuotaPeriodType,
    pub period_number: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub target: i64,
    pub actual: i64,
    pub attainment_percent: f64,
    pub pipeline: i64,
    pub pipeline_coverage: f64,
    pub gap_to_quota: i64,
    pub currency: String,
    pub calculated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryAccount {
    pub id: Uuid,
    pub territory_id: Uuid,
    pub account_id: Uuid,
    pub assignment_type: AssignmentType,
    pub assigned_date: NaiveDate,
    pub assigned_by: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryRule {
    pub base: BaseEntity,
    pub territory_id: Uuid,
    pub rule_name: String,
    pub description: Option<String>,
    pub rule_type: TerritoryRuleType,
    pub field_name: String,
    pub operator: RuleOperator,
    pub value: String,
    pub priority: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TerritoryRuleType {
    Include,
    Exclude,
    Override,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RuleOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    InList,
    NotInList,
    GreaterThan,
    LessThan,
    Between,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryPerformance {
    pub base: BaseEntity,
    pub territory_id: Uuid,
    pub period_type: PeriodType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub quota: i64,
    pub revenue: i64,
    pub attainment_percent: f64,
    pub new_accounts: i32,
    pub total_accounts: i32,
    pub opportunities_created: i32,
    pub opportunities_won: i32,
    pub win_rate_percent: f64,
    pub avg_deal_size: i64,
    pub pipeline_value: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PeriodType {
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaSplit {
    pub id: Uuid,
    pub quota_id: Uuid,
    pub sales_rep_id: Uuid,
    pub split_percent: i32,
    pub split_type: SplitType,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SplitType {
    Primary,
    Overlay,
    Shared,
}
