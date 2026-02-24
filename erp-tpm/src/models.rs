use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionStatus {
    Draft,
    Planned,
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum RebateStatus {
    Pending,
    Approved,
    Paid,
    PartiallyPaid,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ClaimStatus {
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Paid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionType {
    OffInvoice,
    OnInvoice,
    BillBack,
    ScanDown,
    LumpSum,
    VolumeIncentive,
    GrowthIncentive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum FundType {
    MarketingDevelopment,
    CooperativeAdvertising,
    Display,
    Sampling,
    TradeShow,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePromotion {
    pub id: Uuid,
    pub promotion_number: String,
    pub name: String,
    pub description: Option<String>,
    pub promotion_type: PromotionType,
    pub status: PromotionStatus,
    pub customer_id: Option<Uuid>,
    pub customer_group_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub product_group_id: Option<Uuid>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub planned_budget: i64,
    pub committed_budget: i64,
    pub spent_budget: i64,
    pub accrued_budget: i64,
    pub currency: String,
    pub discount_percent: Option<f64>,
    pub discount_amount: Option<i64>,
    pub buy_quantity: Option<i32>,
    pub get_quantity: Option<i32>,
    pub max_redemptions: Option<i32>,
    pub redemptions_count: i32,
    pub forecasted_sales: Option<i64>,
    pub actual_sales: Option<i64>,
    pub roi: Option<f64>,
    pub owner_id: Option<Uuid>,
    pub approval_status: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionProduct {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub product_id: Uuid,
    pub discount_percent: Option<f64>,
    pub discount_amount: Option<i64>,
    pub buy_qty: Option<i32>,
    pub get_qty: Option<i32>,
    pub max_qty: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionCustomer {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_group_id: Option<Uuid>,
    pub territory_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFund {
    pub id: Uuid,
    pub fund_number: String,
    pub name: String,
    pub fund_type: FundType,
    pub customer_id: Option<Uuid>,
    pub fiscal_year: i32,
    pub total_budget: i64,
    pub committed_amount: i64,
    pub spent_amount: i64,
    pub available_amount: i64,
    pub currency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFundTransaction {
    pub id: Uuid,
    pub fund_id: Uuid,
    pub promotion_id: Option<Uuid>,
    pub transaction_type: String,
    pub amount: i64,
    pub currency: String,
    pub reference_number: Option<String>,
    pub description: Option<String>,
    pub transaction_date: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebateAgreement {
    pub id: Uuid,
    pub agreement_number: String,
    pub name: String,
    pub customer_id: Uuid,
    pub agreement_type: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub basis: String,
    pub calculation_method: String,
    pub payment_terms: String,
    pub status: RebateStatus,
    pub total_eligible_sales: i64,
    pub total_rebate_earned: i64,
    pub total_rebate_paid: i64,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebateTier {
    pub id: Uuid,
    pub agreement_id: Uuid,
    pub tier_number: i32,
    pub min_quantity: f64,
    pub max_quantity: Option<f64>,
    pub min_value: i64,
    pub max_value: Option<i64>,
    pub rebate_percent: f64,
    pub rebate_amount: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebateProduct {
    pub id: Uuid,
    pub agreement_id: Uuid,
    pub product_id: Option<Uuid>,
    pub product_group_id: Option<Uuid>,
    pub specific_rate: Option<f64>,
    pub specific_amount: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebateAccrual {
    pub id: Uuid,
    pub agreement_id: Uuid,
    pub sales_order_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub sales_amount: i64,
    pub rebate_rate: f64,
    pub rebate_amount: i64,
    pub currency: String,
    pub accrual_date: DateTime<Utc>,
    pub status: String,
    pub paid_amount: i64,
    pub remaining_amount: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebatePayment {
    pub id: Uuid,
    pub payment_number: String,
    pub agreement_id: Uuid,
    pub customer_id: Uuid,
    pub payment_date: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_amount: i64,
    pub currency: String,
    pub payment_method: String,
    pub reference_number: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebatePaymentLine {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub accrual_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chargeback {
    pub id: Uuid,
    pub chargeback_number: String,
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub promotion_id: Option<Uuid>,
    pub chargeback_date: DateTime<Utc>,
    pub amount_claimed: i64,
    pub amount_approved: i64,
    pub amount_rejected: i64,
    pub currency: String,
    pub status: ClaimStatus,
    pub claim_type: String,
    pub description: Option<String>,
    pub rejection_reason: Option<String>,
    pub submitted_by: Option<Uuid>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargebackLine {
    pub id: Uuid,
    pub chargeback_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: i64,
    pub claimed_amount: i64,
    pub approved_amount: i64,
    pub rejected_amount: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargebackDocument {
    pub id: Uuid,
    pub chargeback_id: Uuid,
    pub document_type: String,
    pub file_name: String,
    pub file_path: String,
    pub uploaded_by: Option<Uuid>,
    pub uploaded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionPerformance {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub baseline_sales: i64,
    pub incremental_sales: i64,
    pub total_sales: i64,
    pub units_sold: i32,
    pub promotion_cost: i64,
    pub roi_percent: f64,
    pub lift_percent: f64,
    pub cannibalization: Option<i64>,
    pub forward_buy: Option<i64>,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionPlan {
    pub id: Uuid,
    pub plan_number: String,
    pub name: String,
    pub fiscal_year: i32,
    pub customer_id: Option<Uuid>,
    pub customer_group_id: Option<Uuid>,
    pub total_budget: i64,
    pub allocated_budget: i64,
    pub spent_budget: i64,
    pub remaining_budget: i64,
    pub currency: String,
    pub status: String,
    pub owner_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionPlanLine {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub promotion_id: Uuid,
    pub quarter: i32,
    pub planned_amount: i64,
    pub actual_amount: i64,
    pub variance: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerTradeProfile {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub trade_class: String,
    pub annual_volume: Option<i64>,
    pub growth_rate: Option<f64>,
    pub avg_promotion_response: Option<f64>,
    pub preferred_promotion_type: Option<String>,
    pub credit_limit: Option<i64>,
    pub payment_terms: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipAndDebit {
    pub id: Uuid,
    pub sad_number: String,
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub authorized_price: i64,
    pub list_price: i64,
    pub authorized_discount: i64,
    pub quantity_authorized: i32,
    pub quantity_shipped: i32,
    pub quantity_debited: i32,
    pub currency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceProtection {
    pub id: Uuid,
    pub pp_number: String,
    pub customer_id: Uuid,
    pub product_id: Option<Uuid>,
    pub product_group_id: Option<Uuid>,
    pub old_price: i64,
    pub new_price: i64,
    pub price_reduction: i64,
    pub effective_date: DateTime<Utc>,
    pub inventory_on_hand: i32,
    pub claim_amount: i64,
    pub approved_amount: i64,
    pub currency: String,
    pub status: ClaimStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePromotionRequest {
    pub name: String,
    pub description: Option<String>,
    pub promotion_type: PromotionType,
    pub customer_id: Option<Uuid>,
    pub customer_group_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub product_group_id: Option<Uuid>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub planned_budget: i64,
    pub currency: String,
    pub discount_percent: Option<f64>,
    pub discount_amount: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRebateAgreementRequest {
    pub name: String,
    pub customer_id: Uuid,
    pub agreement_type: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub basis: String,
    pub calculation_method: String,
    pub payment_terms: String,
    pub tiers: Vec<CreateRebateTierRequest>,
    pub products: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRebateTierRequest {
    pub tier_number: i32,
    pub min_quantity: f64,
    pub max_quantity: Option<f64>,
    pub min_value: i64,
    pub max_value: Option<i64>,
    pub rebate_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitChargebackRequest {
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub promotion_id: Option<Uuid>,
    pub claim_type: String,
    pub description: Option<String>,
    pub lines: Vec<ChargebackLineRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargebackLineRequest {
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: i64,
    pub claimed_amount: i64,
}
