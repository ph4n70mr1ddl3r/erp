use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Currency, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBundle {
    pub base: BaseEntity,
    pub bundle_code: String,
    pub name: String,
    pub description: Option<String>,
    pub bundle_type: BundleType,
    pub pricing_method: BundlePricingMethod,
    pub list_price: Money,
    pub calculated_price: Money,
    pub discount_percent: Option<f64>,
    pub components: Vec<BundleComponent>,
    pub auto_explode: bool,
    pub track_inventory: bool,
    pub availability_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub max_quantity_per_order: Option<i64>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BundleType {
    SalesKit,
    Promotional,
    Assembly,
    ServicePackage,
    Dynamic,
}

impl std::str::FromStr for BundleType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "saleskit" | "sales_kit" => Ok(BundleType::SalesKit),
            "promotional" => Ok(BundleType::Promotional),
            "assembly" => Ok(BundleType::Assembly),
            "servicepackage" | "service_package" => Ok(BundleType::ServicePackage),
            "dynamic" => Ok(BundleType::Dynamic),
            _ => Ok(BundleType::SalesKit),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BundlePricingMethod {
    FixedPrice,
    ComponentSum,
    ComponentSumLessDiscount,
    MarkupOnCost,
}

impl std::str::FromStr for BundlePricingMethod {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fixedprice" | "fixed_price" => Ok(BundlePricingMethod::FixedPrice),
            "componentsum" | "component_sum" => Ok(BundlePricingMethod::ComponentSum),
            "componentsumlessdiscount" | "component_sum_less_discount" => {
                Ok(BundlePricingMethod::ComponentSumLessDiscount)
            }
            "markuponcost" | "markup_on_cost" => Ok(BundlePricingMethod::MarkupOnCost),
            _ => Ok(BundlePricingMethod::FixedPrice),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleComponent {
    pub id: Uuid,
    pub bundle_id: Uuid,
    pub product_id: Uuid,
    pub product_code: Option<String>,
    pub product_name: Option<String>,
    pub quantity: i64,
    pub unit_of_measure: String,
    pub is_mandatory: bool,
    pub sort_order: i32,
    pub component_price: Money,
    pub discount_percent: Option<f64>,
    pub can_substitute: bool,
    pub substitute_group_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleSubstituteGroup {
    pub id: Uuid,
    pub bundle_id: Uuid,
    pub group_name: String,
    pub min_select: i32,
    pub max_select: i32,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundlePriceRule {
    pub id: Uuid,
    pub bundle_id: Uuid,
    pub rule_name: String,
    pub rule_type: BundlePriceRuleType,
    pub min_quantity: i64,
    pub max_quantity: Option<i64>,
    pub discount_percent: Option<f64>,
    pub fixed_price: Option<i64>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub customer_group_id: Option<Uuid>,
    pub priority: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BundlePriceRuleType {
    QuantityBreak,
    CustomerGroup,
    DateRange,
    Promotional,
}

impl std::str::FromStr for BundlePriceRuleType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "quantitybreak" | "quantity_break" => Ok(BundlePriceRuleType::QuantityBreak),
            "customergroup" | "customer_group" => Ok(BundlePriceRuleType::CustomerGroup),
            "daterange" | "date_range" => Ok(BundlePriceRuleType::DateRange),
            "promotional" => Ok(BundlePriceRuleType::Promotional),
            _ => Ok(BundlePriceRuleType::QuantityBreak),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleInventory {
    pub id: Uuid,
    pub bundle_id: Uuid,
    pub warehouse_id: Uuid,
    pub available_quantity: i64,
    pub allocated_quantity: i64,
    pub backorder_quantity: i64,
    pub last_calculated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleAvailability {
    pub bundle_id: Uuid,
    pub bundle_code: String,
    pub bundle_name: String,
    pub total_available: i64,
    pub warehouse_availability: Vec<WarehouseBundleAvailability>,
    pub component_shortages: Vec<ComponentShortage>,
    pub can_fulfill: bool,
    pub earliest_available_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseBundleAvailability {
    pub warehouse_id: Uuid,
    pub warehouse_code: String,
    pub warehouse_name: String,
    pub available_quantity: i64,
    pub can_fulfill: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentShortage {
    pub product_id: Uuid,
    pub product_code: String,
    pub product_name: String,
    pub required_quantity: i64,
    pub available_quantity: i64,
    pub shortage_quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleTemplate {
    pub base: BaseEntity,
    pub template_code: String,
    pub name: String,
    pub description: Option<String>,
    pub default_components: Vec<TemplateComponent>,
    pub default_pricing_method: BundlePricingMethod,
    pub default_markup_percent: Option<f64>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateComponent {
    pub id: Uuid,
    pub template_id: Uuid,
    pub product_id: Option<Uuid>,
    pub product_category_id: Option<Uuid>,
    pub quantity: i64,
    pub is_mandatory: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleUsage {
    pub id: Uuid,
    pub bundle_id: Uuid,
    pub order_id: Option<Uuid>,
    pub order_line_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub quantity: i64,
    pub unit_price: i64,
    pub total_price: i64,
    pub margin_amount: i64,
    pub margin_percent: f64,
    pub usage_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleAnalytics {
    pub bundle_id: Uuid,
    pub bundle_code: String,
    pub bundle_name: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_sold: i64,
    pub total_revenue: i64,
    pub total_margin: i64,
    pub margin_percent: f64,
    pub avg_discount_percent: f64,
    pub order_count: i32,
    pub customer_count: i32,
    pub top_warehouses: Vec<WarehouseSales>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseSales {
    pub warehouse_id: Uuid,
    pub warehouse_name: String,
    pub quantity_sold: i64,
    pub revenue: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBundleRequest {
    pub bundle_code: String,
    pub name: String,
    pub description: Option<String>,
    pub bundle_type: BundleType,
    pub pricing_method: BundlePricingMethod,
    pub list_price_amount: i64,
    pub currency: Currency,
    pub discount_percent: Option<f64>,
    pub components: Vec<CreateBundleComponentRequest>,
    pub auto_explode: bool,
    pub track_inventory: bool,
    pub availability_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub max_quantity_per_order: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBundleComponentRequest {
    pub product_id: Uuid,
    pub quantity: i64,
    pub unit_of_measure: String,
    pub is_mandatory: bool,
    pub discount_percent: Option<f64>,
    pub can_substitute: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBundleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub bundle_type: Option<BundleType>,
    pub pricing_method: Option<BundlePricingMethod>,
    pub list_price_amount: Option<i64>,
    pub discount_percent: Option<f64>,
    pub auto_explode: Option<bool>,
    pub track_inventory: Option<bool>,
    pub availability_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub max_quantity_per_order: Option<i64>,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleListResponse {
    pub items: Vec<ProductBundleSummary>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBundleSummary {
    pub id: Uuid,
    pub bundle_code: String,
    pub name: String,
    pub bundle_type: BundleType,
    pub list_price: Money,
    pub component_count: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}
