use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiResult;
use crate::db::AppState;
use erp_bundles::{
    BundleService, ProductBundle, BundleComponent, BundleAvailability, BundlePriceRule,
    BundleAnalytics, CreateBundleRequest, UpdateBundleRequest, CreateBundleComponentRequest,
    BundleListResponse, ProductBundleSummary, BundleType, BundlePricingMethod,
};
use erp_core::Status;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateBundleBody {
    pub bundle_code: String,
    pub name: String,
    pub description: Option<String>,
    pub bundle_type: String,
    pub pricing_method: String,
    pub list_price_cents: i64,
    pub currency: String,
    pub discount_percent: Option<f64>,
    pub components: Vec<CreateComponentBody>,
    pub auto_explode: Option<bool>,
    pub track_inventory: Option<bool>,
    pub availability_date: Option<String>,
    pub expiry_date: Option<String>,
    pub max_quantity_per_order: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreateComponentBody {
    pub product_id: Uuid,
    pub quantity: i64,
    pub unit_of_measure: String,
    pub is_mandatory: Option<bool>,
    pub discount_percent: Option<f64>,
    pub can_substitute: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateBundleBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub bundle_type: Option<String>,
    pub pricing_method: Option<String>,
    pub list_price_cents: Option<i64>,
    pub discount_percent: Option<f64>,
    pub auto_explode: Option<bool>,
    pub track_inventory: Option<bool>,
    pub availability_date: Option<String>,
    pub expiry_date: Option<String>,
    pub max_quantity_per_order: Option<i64>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct AddComponentBody {
    pub product_id: Uuid,
    pub quantity: i64,
    pub unit_of_measure: String,
    pub is_mandatory: Option<bool>,
    pub discount_percent: Option<f64>,
    pub can_substitute: Option<bool>,
}

#[derive(Deserialize)]
pub struct AddPriceRuleBody {
    pub rule_name: String,
    pub rule_type: String,
    pub min_quantity: i64,
    pub max_quantity: Option<i64>,
    pub discount_percent: Option<f64>,
    pub fixed_price: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub customer_group_id: Option<Uuid>,
    pub priority: Option<i32>,
}

#[derive(Deserialize)]
pub struct CalculatePriceQuery {
    pub quantity: i64,
    pub customer_group_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct AnalyticsQuery {
    pub period_start: String,
    pub period_end: String,
}

#[derive(Serialize)]
pub struct BundleResponse {
    pub id: String,
    pub bundle_code: String,
    pub name: String,
    pub description: Option<String>,
    pub bundle_type: String,
    pub pricing_method: String,
    pub list_price: PriceResponse,
    pub calculated_price: PriceResponse,
    pub discount_percent: Option<f64>,
    pub components: Vec<ComponentResponse>,
    pub auto_explode: bool,
    pub track_inventory: bool,
    pub availability_date: Option<String>,
    pub expiry_date: Option<String>,
    pub max_quantity_per_order: Option<i64>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct PriceResponse {
    pub cents: i64,
    pub currency: String,
}

#[derive(Serialize)]
pub struct ComponentResponse {
    pub id: String,
    pub product_id: String,
    pub quantity: i64,
    pub unit_of_measure: String,
    pub is_mandatory: bool,
    pub sort_order: i32,
    pub discount_percent: Option<f64>,
    pub can_substitute: bool,
}

#[derive(Serialize)]
pub struct BundleSummaryResponse {
    pub id: String,
    pub bundle_code: String,
    pub name: String,
    pub bundle_type: String,
    pub list_price: PriceResponse,
    pub component_count: i32,
    pub status: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct BundleListApiResponse {
    pub items: Vec<BundleSummaryResponse>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Serialize)]
pub struct AvailabilityResponse {
    pub bundle_id: String,
    pub bundle_code: String,
    pub bundle_name: String,
    pub total_available: i64,
    pub can_fulfill: bool,
    pub component_shortages: Vec<ShortageResponse>,
}

#[derive(Serialize)]
pub struct ShortageResponse {
    pub product_id: String,
    pub required_quantity: i64,
    pub available_quantity: i64,
    pub shortage_quantity: i64,
}

#[derive(Serialize)]
pub struct PriceRuleResponse {
    pub id: String,
    pub bundle_id: String,
    pub rule_name: String,
    pub rule_type: String,
    pub min_quantity: i64,
    pub max_quantity: Option<i64>,
    pub discount_percent: Option<f64>,
    pub fixed_price: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub priority: i32,
    pub status: String,
}

#[derive(Serialize)]
pub struct AnalyticsResponse {
    pub bundle_id: String,
    pub bundle_code: String,
    pub bundle_name: String,
    pub period_start: String,
    pub period_end: String,
    pub total_sold: i64,
    pub total_revenue: i64,
    pub total_margin: i64,
    pub margin_percent: f64,
    pub order_count: i32,
    pub customer_count: i32,
}

impl From<ProductBundle> for BundleResponse {
    fn from(b: ProductBundle) -> Self {
        Self {
            id: b.base.id.to_string(),
            bundle_code: b.bundle_code,
            name: b.name,
            description: b.description,
            bundle_type: format!("{:?}", b.bundle_type),
            pricing_method: format!("{:?}", b.pricing_method),
            list_price: PriceResponse { cents: b.list_price.cents, currency: b.list_price.currency },
            calculated_price: PriceResponse { cents: b.calculated_price.cents, currency: b.calculated_price.currency },
            discount_percent: b.discount_percent,
            components: b.components.into_iter().map(|c| ComponentResponse {
                id: c.id.to_string(),
                product_id: c.product_id.to_string(),
                quantity: c.quantity,
                unit_of_measure: c.unit_of_measure,
                is_mandatory: c.is_mandatory,
                sort_order: c.sort_order,
                discount_percent: c.discount_percent,
                can_substitute: c.can_substitute,
            }).collect(),
            auto_explode: b.auto_explode,
            track_inventory: b.track_inventory,
            availability_date: b.availability_date.map(|d| d.to_rfc3339()),
            expiry_date: b.expiry_date.map(|d| d.to_rfc3339()),
            max_quantity_per_order: b.max_quantity_per_order,
            status: format!("{:?}", b.status),
            created_at: b.base.created_at.to_rfc3339(),
            updated_at: b.base.updated_at.to_rfc3339(),
        }
    }
}

impl From<ProductBundleSummary> for BundleSummaryResponse {
    fn from(s: ProductBundleSummary) -> Self {
        Self {
            id: s.id.to_string(),
            bundle_code: s.bundle_code,
            name: s.name,
            bundle_type: format!("{:?}", s.bundle_type),
            list_price: PriceResponse { cents: s.list_price.cents, currency: s.list_price.currency },
            component_count: s.component_count,
            status: format!("{:?}", s.status),
            created_at: s.created_at.to_rfc3339(),
        }
    }
}

pub async fn create_bundle(
    State(state): State<AppState>,
    Json(body): Json<CreateBundleBody>,
) -> ApiResult<Json<BundleResponse>> {
    let service = BundleService::new();
    let req = CreateBundleRequest {
        bundle_code: body.bundle_code,
        name: body.name,
        description: body.description,
        bundle_type: parse_bundle_type(&body.bundle_type),
        pricing_method: parse_pricing_method(&body.pricing_method),
        list_price_cents: body.list_price_cents,
        currency: body.currency,
        discount_percent: body.discount_percent,
        components: body.components.into_iter().map(|c| CreateBundleComponentRequest {
            product_id: c.product_id,
            quantity: c.quantity,
            unit_of_measure: c.unit_of_measure,
            is_mandatory: c.is_mandatory.unwrap_or(true),
            discount_percent: c.discount_percent,
            can_substitute: c.can_substitute.unwrap_or(false),
        }).collect(),
        auto_explode: body.auto_explode.unwrap_or(false),
        track_inventory: body.track_inventory.unwrap_or(true),
        availability_date: body.availability_date.and_then(|d| DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&Utc))),
        expiry_date: body.expiry_date.and_then(|d| DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&Utc))),
        max_quantity_per_order: body.max_quantity_per_order,
    };

    let bundle = service.create_bundle(&state.pool, req).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;
    Ok(Json(BundleResponse::from(bundle)))
}

pub async fn get_bundle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<BundleResponse>> {
    let service = BundleService::new();
    let bundle = service.get_bundle(&state.pool, id).await
        .map_err(|e| crate::error::Error::NotFound(e.to_string()))?;
    Ok(Json(BundleResponse::from(bundle)))
}

pub async fn list_bundles(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<BundleListApiResponse>> {
    let service = BundleService::new();
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let status = query.status.and_then(|s| parse_status(&s));

    let result = service.list_bundles(&state.pool, page, per_page, status).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;

    Ok(Json(BundleListApiResponse {
        items: result.items.into_iter().map(BundleSummaryResponse::from).collect(),
        total: result.total,
        page: result.page,
        per_page: result.per_page,
    }))
}

pub async fn update_bundle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateBundleBody>,
) -> ApiResult<Json<BundleResponse>> {
    let service = BundleService::new();
    let req = UpdateBundleRequest {
        name: body.name,
        description: body.description,
        bundle_type: body.bundle_type.and_then(|t| Some(parse_bundle_type(&t))),
        pricing_method: body.pricing_method.and_then(|m| Some(parse_pricing_method(&m))),
        list_price_cents: body.list_price_cents,
        discount_percent: body.discount_percent,
        auto_explode: body.auto_explode,
        track_inventory: body.track_inventory,
        availability_date: body.availability_date.and_then(|d| DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&Utc))),
        expiry_date: body.expiry_date.and_then(|d| DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&Utc))),
        max_quantity_per_order: body.max_quantity_per_order,
        status: body.status.and_then(|s| parse_status(&s)),
    };

    let bundle = service.update_bundle(&state.pool, id, req).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;
    Ok(Json(BundleResponse::from(bundle)))
}

pub async fn delete_bundle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = BundleService::new();
    service.delete_bundle(&state.pool, id).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_component(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<AddComponentBody>,
) -> ApiResult<Json<ComponentResponse>> {
    let service = BundleService::new();
    let req = CreateBundleComponentRequest {
        product_id: body.product_id,
        quantity: body.quantity,
        unit_of_measure: body.unit_of_measure,
        is_mandatory: body.is_mandatory.unwrap_or(true),
        discount_percent: body.discount_percent,
        can_substitute: body.can_substitute.unwrap_or(false),
    };

    let component = service.add_component(&state.pool, id, req).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;

    Ok(Json(ComponentResponse {
        id: component.id.to_string(),
        product_id: component.product_id.to_string(),
        quantity: component.quantity,
        unit_of_measure: component.unit_of_measure,
        is_mandatory: component.is_mandatory,
        sort_order: component.sort_order,
        discount_percent: component.discount_percent,
        can_substitute: component.can_substitute,
    }))
}

pub async fn remove_component(
    State(state): State<AppState>,
    Path((bundle_id, component_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<StatusCode> {
    let service = BundleService::new();
    service.remove_component(&state.pool, bundle_id, component_id).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_availability(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AvailabilityResponse>> {
    let service = BundleService::new();
    let availability = service.get_availability(&state.pool, id).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;

    Ok(Json(AvailabilityResponse {
        bundle_id: availability.bundle_id.to_string(),
        bundle_code: availability.bundle_code,
        bundle_name: availability.bundle_name,
        total_available: availability.total_available,
        can_fulfill: availability.can_fulfill,
        component_shortages: availability.component_shortages.into_iter().map(|s| ShortageResponse {
            product_id: s.product_id.to_string(),
            required_quantity: s.required_quantity,
            available_quantity: s.available_quantity,
            shortage_quantity: s.shortage_quantity,
        }).collect(),
    }))
}

pub async fn add_price_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<AddPriceRuleBody>,
) -> ApiResult<Json<PriceRuleResponse>> {
    let service = BundleService::new();
    let rule = BundlePriceRule {
        id: Uuid::new_v4(),
        bundle_id: id,
        rule_name: body.rule_name,
        rule_type: parse_rule_type(&body.rule_type),
        min_quantity: body.min_quantity,
        max_quantity: body.max_quantity,
        discount_percent: body.discount_percent,
        fixed_price: body.fixed_price,
        start_date: body.start_date.and_then(|d| DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&Utc))),
        end_date: body.end_date.and_then(|d| DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&Utc))),
        customer_group_id: body.customer_group_id,
        priority: body.priority.unwrap_or(0),
        status: Status::Active,
        created_at: Utc::now(),
    };

    let rule = service.add_price_rule(&state.pool, id, rule).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;

    Ok(Json(PriceRuleResponse {
        id: rule.id.to_string(),
        bundle_id: rule.bundle_id.to_string(),
        rule_name: rule.rule_name,
        rule_type: format!("{:?}", rule.rule_type),
        min_quantity: rule.min_quantity,
        max_quantity: rule.max_quantity,
        discount_percent: rule.discount_percent,
        fixed_price: rule.fixed_price,
        start_date: rule.start_date.map(|d| d.to_rfc3339()),
        end_date: rule.end_date.map(|d| d.to_rfc3339()),
        priority: rule.priority,
        status: format!("{:?}", rule.status),
    }))
}

pub async fn get_price_rules(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Vec<PriceRuleResponse>>> {
    let service = BundleService::new();
    let rules = service.get_price_rules(&state.pool, id).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;

    Ok(Json(rules.into_iter().map(|r| PriceRuleResponse {
        id: r.id.to_string(),
        bundle_id: r.bundle_id.to_string(),
        rule_name: r.rule_name,
        rule_type: format!("{:?}", r.rule_type),
        min_quantity: r.min_quantity,
        max_quantity: r.max_quantity,
        discount_percent: r.discount_percent,
        fixed_price: r.fixed_price,
        start_date: r.start_date.map(|d| d.to_rfc3339()),
        end_date: r.end_date.map(|d| d.to_rfc3339()),
        priority: r.priority,
        status: format!("{:?}", r.status),
    }).collect()))
}

pub async fn calculate_price(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(query): Query<CalculatePriceQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = BundleService::new();
    let price = service.calculate_price_for_quantity(&state.pool, id, query.quantity, query.customer_group_id).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "bundle_id": id.to_string(),
        "quantity": query.quantity,
        "unit_price": price,
        "total_price": price * query.quantity,
        "currency": "USD"
    })))
}

pub async fn get_analytics(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(query): Query<AnalyticsQuery>,
) -> ApiResult<Json<AnalyticsResponse>> {
    let service = BundleService::new();
    let period_start = DateTime::parse_from_rfc3339(&query.period_start)
        .map_err(|_| crate::error::Error::BadRequest("Invalid period_start".to_string()))?
        .with_timezone(&Utc);
    let period_end = DateTime::parse_from_rfc3339(&query.period_end)
        .map_err(|_| crate::error::Error::BadRequest("Invalid period_end".to_string()))?
        .with_timezone(&Utc);

    let analytics = service.get_analytics(&state.pool, id, period_start, period_end).await
        .map_err(|e| crate::error::Error::BadRequest(e.to_string()))?;

    Ok(Json(AnalyticsResponse {
        bundle_id: analytics.bundle_id.to_string(),
        bundle_code: analytics.bundle_code,
        bundle_name: analytics.bundle_name,
        period_start: analytics.period_start.to_rfc3339(),
        period_end: analytics.period_end.to_rfc3339(),
        total_sold: analytics.total_sold,
        total_revenue: analytics.total_revenue,
        total_margin: analytics.total_margin,
        margin_percent: analytics.margin_percent,
        order_count: analytics.order_count,
        customer_count: analytics.customer_count,
    }))
}

fn parse_bundle_type(s: &str) -> BundleType {
    match s.to_lowercase().as_str() {
        "saleskit" => BundleType::SalesKit,
        "promotional" => BundleType::Promotional,
        "assembly" => BundleType::Assembly,
        "servicepackage" => BundleType::ServicePackage,
        "dynamic" => BundleType::Dynamic,
        _ => BundleType::SalesKit,
    }
}

fn parse_pricing_method(s: &str) -> BundlePricingMethod {
    match s.to_lowercase().as_str() {
        "fixedprice" => BundlePricingMethod::FixedPrice,
        "componentsum" => BundlePricingMethod::ComponentSum,
        "componentsumlessdiscount" => BundlePricingMethod::ComponentSumLessDiscount,
        "markuponcost" => BundlePricingMethod::MarkupOnCost,
        _ => BundlePricingMethod::FixedPrice,
    }
}

fn parse_status(s: &str) -> Option<Status> {
    match s.to_lowercase().as_str() {
        "active" => Some(Status::Active),
        "inactive" => Some(Status::Inactive),
        "draft" => Some(Status::Draft),
        _ => None,
    }
}

fn parse_rule_type(s: &str) -> erp_bundles::BundlePriceRuleType {
    match s.to_lowercase().as_str() {
        "quantitybreak" => erp_bundles::BundlePriceRuleType::QuantityBreak,
        "customergroup" => erp_bundles::BundlePriceRuleType::CustomerGroup,
        "daterange" => erp_bundles::BundlePriceRuleType::DateRange,
        "promotional" => erp_bundles::BundlePriceRuleType::Promotional,
        _ => erp_bundles::BundlePriceRuleType::QuantityBreak,
    }
}
