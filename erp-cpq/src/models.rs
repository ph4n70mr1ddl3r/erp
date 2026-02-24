use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConfigDataType {
    Text,
    Number,
    Boolean,
    Select,
    MultiSelect,
    Date,
    Color,
    Dimension,
    Weight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigAttribute {
    pub id: Uuid,
    pub template_id: Uuid,
    pub name: String,
    pub display_name: String,
    pub data_type: ConfigDataType,
    pub required: bool,
    pub default_value: Option<String>,
    pub options: Option<String>,
    pub validation_regex: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub unit: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigAttributeGroup {
    pub id: Uuid,
    pub template_id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDependency {
    pub id: Uuid,
    pub template_id: Uuid,
    pub source_attribute_id: Uuid,
    pub target_attribute_id: Uuid,
    pub condition_value: String,
    pub action_type: String,
    pub action_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PricingRuleType {
    Fixed,
    Markup,
    Margin,
    Matrix,
    Formula,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRule {
    pub id: Uuid,
    pub template_id: Uuid,
    pub name: String,
    pub rule_type: PricingRuleType,
    pub attribute_id: Option<Uuid>,
    pub attribute_value: Option<String>,
    pub base_price_modifier: Option<f64>,
    pub fixed_price: Option<i64>,
    pub markup_percent: Option<f64>,
    pub formula: Option<String>,
    pub priority: i32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingMatrix {
    pub id: Uuid,
    pub pricing_rule_id: Uuid,
    pub row_attribute_id: Uuid,
    pub column_attribute_id: Uuid,
    pub rows: String,
    pub columns: String,
    pub prices: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductConfiguration {
    pub id: Uuid,
    pub configuration_number: String,
    pub template_id: Uuid,
    pub product_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Vec<ConfigurationValue>,
    pub base_price: i64,
    pub configured_price: i64,
    pub margin_percent: f64,
    pub is_valid: bool,
    pub validation_errors: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationValue {
    pub id: Uuid,
    pub configuration_id: Uuid,
    pub attribute_id: Uuid,
    pub value: String,
    pub display_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationTemplate {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub product_id: Option<Uuid>,
    pub base_price: i64,
    pub min_margin_percent: f64,
    pub max_discount_percent: f64,
    pub attributes: Vec<ConfigAttribute>,
    pub pricing_rules: Vec<PricingRule>,
    pub status: Status,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConstraintType {
    Requires,
    Excludes,
    RequiresAny,
    RequiresAll,
    MinMax,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationConstraint {
    pub id: Uuid,
    pub template_id: Uuid,
    pub name: String,
    pub constraint_type: ConstraintType,
    pub source_attribute_id: Uuid,
    pub source_values: String,
    pub target_attribute_id: Uuid,
    pub target_values: String,
    pub error_message: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurableBom {
    pub id: Uuid,
    pub template_id: Uuid,
    pub parent_attribute_id: Option<Uuid>,
    pub parent_value: Option<String>,
    pub component_product_id: Uuid,
    pub quantity: i64,
    pub quantity_formula: Option<String>,
    pub is_optional: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurableRouting {
    pub id: Uuid,
    pub template_id: Uuid,
    pub work_center_id: Uuid,
    pub operation_name: String,
    pub setup_time: i64,
    pub run_time: i64,
    pub run_time_formula: Option<String>,
    pub condition_attribute_id: Option<Uuid>,
    pub condition_value: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QuoteStatus {
    Draft,
    Pending,
    Sent,
    Accepted,
    Rejected,
    Expired,
    Converted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguredQuote {
    pub base: BaseEntity,
    pub quote_number: String,
    pub configuration_id: Uuid,
    pub customer_id: Uuid,
    pub opportunity_id: Option<Uuid>,
    pub valid_until: DateTime<Utc>,
    pub base_price: Money,
    pub configured_price: Money,
    pub discount_percent: f64,
    pub discount_amount: Money,
    pub margin_percent: f64,
    pub total_price: Money,
    pub terms: Option<String>,
    pub internal_notes: Option<String>,
    pub status: QuoteStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguredQuoteLine {
    pub id: Uuid,
    pub quote_id: Uuid,
    pub line_type: String,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub total_price: Money,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteApproval {
    pub id: Uuid,
    pub quote_id: Uuid,
    pub approver_id: Uuid,
    pub approval_type: String,
    pub status: String,
    pub comments: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteVersion {
    pub id: Uuid,
    pub quote_id: Uuid,
    pub version_number: i32,
    pub configuration_snapshot: String,
    pub price_snapshot: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuidedSellingStep {
    pub id: Uuid,
    pub template_id: Uuid,
    pub step_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub help_text: Option<String>,
    pub attribute_ids: String,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationRule {
    pub id: Uuid,
    pub template_id: Uuid,
    pub name: String,
    pub condition_logic: String,
    pub recommendation_text: String,
    pub recommended_values: String,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDiscountTier {
    pub id: Uuid,
    pub template_id: Uuid,
    pub min_quantity: i64,
    pub max_quantity: Option<i64>,
    pub discount_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumePriceBreak {
    pub id: Uuid,
    pub pricing_rule_id: Uuid,
    pub min_quantity: i64,
    pub max_quantity: Option<i64>,
    pub unit_price: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationPricingResult {
    pub base_price: i64,
    pub configured_price: i64,
    pub price_breakdown: Vec<PriceBreakdownItem>,
    pub applied_rules: Vec<AppliedPricingRule>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceBreakdownItem {
    pub name: String,
    pub description: String,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedPricingRule {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub modification: f64,
    pub modification_type: String,
}
