use chrono::{DateTime, Utc};
use erp_core::{Address, BaseEntity, ContactInfo, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub contact: ContactInfo,
    pub billing_address: Address,
    pub shipping_address: Option<Address>,
    pub credit_limit: Option<Money>,
    pub payment_terms: u32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOrder {
    pub base: BaseEntity,
    pub order_number: String,
    pub customer_id: Uuid,
    pub order_date: DateTime<Utc>,
    pub required_date: Option<DateTime<Utc>>,
    pub lines: Vec<SalesOrderLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOrderLine {
    pub id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub discount_percent: f64,
    pub tax_rate: f64,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesQuote {
    pub base: BaseEntity,
    pub quote_number: String,
    pub customer_id: Uuid,
    pub quote_date: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub lines: Vec<SalesQuoteLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesQuoteLine {
    pub id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub discount_percent: f64,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub base: BaseEntity,
    pub invoice_number: String,
    pub customer_id: Uuid,
    pub sales_order_id: Option<Uuid>,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub lines: Vec<InvoiceLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub amount_paid: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub base: BaseEntity,
    pub payment_number: String,
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub payment_date: DateTime<Utc>,
    pub amount: Money,
    pub payment_method: PaymentMethod,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethod {
    Cash,
    Check,
    CreditCard,
    BankTransfer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: Uuid,
    pub lead_number: String,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub source: Option<String>,
    pub industry: Option<String>,
    pub estimated_value: i64,
    pub status: LeadStatus,
    pub assigned_to: Option<Uuid>,
    pub notes: Option<String>,
    pub converted_to_customer: Option<Uuid>,
    pub converted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Unqualified,
    Converted,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: Uuid,
    pub opportunity_number: String,
    pub name: String,
    pub customer_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub stage: OpportunityStage,
    pub probability: i32,
    pub expected_close_date: Option<DateTime<Utc>>,
    pub amount: i64,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub status: OpportunityStatus,
    pub activities: Vec<OpportunityActivity>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum OpportunityStage {
    Prospecting,
    Qualification,
    Proposal,
    Negotiation,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum OpportunityStatus {
    Open,
    Won,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityActivity {
    pub id: Uuid,
    pub opportunity_id: Uuid,
    pub activity_type: ActivityType,
    pub subject: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ActivityType {
    Call,
    Meeting,
    Email,
    Task,
    Note,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesTerritory {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_territory_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub geography_type: Option<String>,
    pub geography_codes: Option<String>,
    pub target_revenue: i64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryAssignment {
    pub id: Uuid,
    pub territory_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub sales_rep_id: Uuid,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommissionPlanType {
    Revenue,
    GrossMargin,
    Unit,
    Tiered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionPlan {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub plan_type: CommissionPlanType,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionTier {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub tier_number: i32,
    pub min_amount: i64,
    pub max_amount: Option<i64>,
    pub rate_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommissionStatus {
    Calculated,
    Approved,
    Paid,
    Adjusted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesRepCommission {
    pub id: Uuid,
    pub sales_rep_id: Uuid,
    pub plan_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub gross_sales: i64,
    pub returns: i64,
    pub net_sales: i64,
    pub commission_rate: f64,
    pub commission_amount: i64,
    pub status: CommissionStatus,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommissionTransactionType {
    Earned,
    Clawback,
    Adjustment,
    Bonus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionTransaction {
    pub id: Uuid,
    pub commission_id: Uuid,
    pub transaction_type: CommissionTransactionType,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub amount: i64,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContractType {
    Fixed,
    TimeAndMaterials,
    Subscription,
    Milestone,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Annually,
    OneTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContractStatus {
    Draft,
    Pending,
    Active,
    Expired,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: Uuid,
    pub contract_number: String,
    pub title: String,
    pub customer_id: Uuid,
    pub contract_type: ContractType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub value: i64,
    pub currency: String,
    pub billing_cycle: Option<BillingCycle>,
    pub auto_renew: bool,
    pub renewal_notice_days: i32,
    pub terms: Option<String>,
    pub status: ContractStatus,
    pub signed_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BillingType {
    Fixed,
    Usage,
    Tiered,
    Overage,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BillingFrequency {
    Monthly,
    Quarterly,
    Annually,
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContractLineStatus {
    Active,
    Suspended,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractLine {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub billing_type: BillingType,
    pub billing_frequency: Option<BillingFrequency>,
    pub next_billing_date: Option<DateTime<Utc>>,
    pub status: ContractLineStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RenewalStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRenewal {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub renewal_date: DateTime<Utc>,
    pub new_start_date: DateTime<Utc>,
    pub new_end_date: DateTime<Utc>,
    pub new_value: Option<i64>,
    pub status: RenewalStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BillingCycleInterval {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: Uuid,
    pub plan_code: String,
    pub name: String,
    pub description: Option<String>,
    pub billing_cycle: BillingCycleInterval,
    pub billing_interval: i32,
    pub setup_fee: i64,
    pub base_price: i64,
    pub trial_days: i32,
    pub features: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SubscriptionStatus {
    Trial,
    Active,
    PastDue,
    Suspended,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub subscription_number: String,
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    pub contract_id: Option<Uuid>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub quantity: i32,
    pub price_override: Option<i64>,
    pub status: SubscriptionStatus,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum UsageType {
    ApiCalls,
    Storage,
    Users,
    Transactions,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUsage {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub usage_date: DateTime<Utc>,
    pub usage_type: UsageType,
    pub quantity: i64,
    pub unit_price: Option<i64>,
    pub total_amount: Option<i64>,
    pub invoice_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SubscriptionInvoiceStatus {
    Draft,
    Sent,
    Paid,
    Void,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInvoice {
    pub id: Uuid,
    pub invoice_number: String,
    pub subscription_id: Uuid,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub subtotal: i64,
    pub tax_amount: i64,
    pub total: i64,
    pub amount_paid: i64,
    pub status: SubscriptionInvoiceStatus,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CampaignType {
    Email,
    Social,
    Event,
    Content,
    PPC,
    Display,
    DirectMail,
    MultiChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CampaignChannel {
    Email,
    SocialMedia,
    Web,
    Mobile,
    Print,
    TV,
    Radio,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Running,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketingCampaign {
    pub id: Uuid,
    pub campaign_code: String,
    pub name: String,
    pub description: Option<String>,
    pub campaign_type: CampaignType,
    pub channel: CampaignChannel,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub budget: i64,
    pub actual_spend: i64,
    pub target_audience: Option<String>,
    pub objectives: Option<String>,
    pub status: CampaignStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ResponseType {
    Opened,
    Clicked,
    Replied,
    Bounced,
    Unsubscribed,
    Converted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignLead {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub lead_id: Uuid,
    pub responded_at: Option<DateTime<Utc>>,
    pub response_type: Option<ResponseType>,
    pub converted: bool,
    pub conversion_value: Option<i64>,
}
