use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PartnerType {
    Reseller,
    Distributor,
    OEM,
    Technology,
    Consulting,
    Implementation,
    Referral,
    Affiliate,
    Supplier,
    ServiceProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PartnerTier {
    Registered,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Elite,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PartnerStatus {
    Prospective,
    Pending,
    Active,
    Suspended,
    Terminated,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partner {
    pub base: BaseEntity,
    pub partner_number: String,
    pub name: String,
    pub legal_name: Option<String>,
    pub partner_type: PartnerType,
    pub tier: PartnerTier,
    pub parent_partner_id: Option<Uuid>,
    pub primary_contact_id: Option<Uuid>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub tax_id: Option<String>,
    pub registration_date: Option<NaiveDate>,
    pub agreement_date: Option<NaiveDate>,
    pub agreement_expiry: Option<NaiveDate>,
    pub contract_value: Option<i64>,
    pub currency: String,
    pub commission_rate: f64,
    pub discount_rate: f64,
    pub credit_limit: Option<i64>,
    pub payment_terms_days: i32,
    pub certification_level: Option<String>,
    pub certifications: Option<String>,
    pub specializations: Option<String>,
    pub regions_served: Option<String>,
    pub industries_served: Option<String>,
    pub annual_revenue: Option<i64>,
    pub employee_count: Option<i32>,
    pub notes: Option<String>,
    pub status: PartnerStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerContact {
    pub base: BaseEntity,
    pub partner_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub title: Option<String>,
    pub department: Option<String>,
    pub is_primary: bool,
    pub receive_notifications: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerAgreement {
    pub base: BaseEntity,
    pub agreement_number: String,
    pub partner_id: Uuid,
    pub agreement_type: AgreementType,
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub auto_renew: bool,
    pub renewal_term_months: i32,
    pub notice_period_days: i32,
    pub commission_rate: f64,
    pub discount_rate: f64,
    pub min_sales_target: Option<i64>,
    pub max_sales_limit: Option<i64>,
    pub territory: Option<String>,
    pub exclusivity: bool,
    pub document_path: Option<String>,
    pub signed_date: Option<NaiveDate>,
    pub signed_by: Option<Uuid>,
    pub status: AgreementStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AgreementType {
    Partner,
    Reseller,
    Distribution,
    Referral,
    OEM,
    Service,
    NDA,
    MSA,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AgreementStatus {
    Draft,
    Pending,
    Active,
    Expired,
    Terminated,
    Renewed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerDeal {
    pub base: BaseEntity,
    pub deal_number: String,
    pub partner_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_name: String,
    pub deal_name: String,
    pub description: Option<String>,
    pub deal_type: DealType,
    pub stage: DealStage,
    pub amount: i64,
    pub currency: String,
    pub expected_close_date: NaiveDate,
    pub probability: i32,
    pub lead_source: Option<String>,
    pub products: Option<String>,
    pub partner_commission: i64,
    pub internal_sales_rep_id: Option<Uuid>,
    pub partner_contact_id: Option<Uuid>,
    pub notes: Option<String>,
    pub won_date: Option<NaiveDate>,
    pub lost_date: Option<NaiveDate>,
    pub lost_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DealType {
    NewBusiness,
    Renewal,
    Upsell,
    CrossSell,
    Expansion,
    Referral,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DealStage {
    Qualified,
    Proposal,
    Negotiation,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerDealRegistration {
    pub base: BaseEntity,
    pub registration_number: String,
    pub partner_id: Uuid,
    pub deal_id: Option<Uuid>,
    pub customer_name: String,
    pub opportunity_name: String,
    pub estimated_value: i64,
    pub currency: String,
    pub expected_close_date: NaiveDate,
    pub products: Option<String>,
    pub registration_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub status: RegistrationStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RegistrationStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    Converted,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerCommission {
    pub base: BaseEntity,
    pub commission_number: String,
    pub partner_id: Uuid,
    pub deal_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub commission_date: NaiveDate,
    pub revenue_amount: i64,
    pub commission_rate: f64,
    pub commission_amount: i64,
    pub currency: String,
    pub status: CommissionStatus,
    pub paid_date: Option<NaiveDate>,
    pub payment_reference: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommissionStatus {
    Accrued,
    Approved,
    Paid,
    Held,
    Reversed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerPerformance {
    pub base: BaseEntity,
    pub partner_id: Uuid,
    pub period_type: PeriodType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub deals_opened: i32,
    pub deals_won: i32,
    pub deals_lost: i32,
    pub total_pipeline: i64,
    pub total_revenue: i64,
    pub total_commission: i64,
    pub win_rate_percent: f64,
    pub avg_deal_size: i64,
    pub avg_sales_cycle_days: f64,
    pub customer_satisfaction: Option<f64>,
    pub target_revenue: Option<i64>,
    pub attainment_percent: Option<f64>,
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
pub struct PartnerTraining {
    pub base: BaseEntity,
    pub training_number: String,
    pub partner_id: Uuid,
    pub contact_id: Option<Uuid>,
    pub training_name: String,
    pub training_type: TrainingType,
    pub provider: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status: TrainingStatus,
    pub score: Option<i32>,
    pub certificate_number: Option<String>,
    pub certificate_expiry: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TrainingType {
    Product,
    Sales,
    Technical,
    Certification,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TrainingStatus {
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerPortalAccess {
    pub base: BaseEntity,
    pub partner_id: Uuid,
    pub contact_id: Uuid,
    pub user_id: Option<Uuid>,
    pub access_level: AccessLevel,
    pub modules: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub login_count: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AccessLevel {
    ReadOnly,
    Standard,
    Manager,
    Admin,
}
