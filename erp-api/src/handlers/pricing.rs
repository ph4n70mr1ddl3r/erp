use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::BaseEntity;
use erp_pricing::{
    PricingService, PriceBook, PriceBookEntry, PriceRule, Discount, Promotion, Coupon,
    PriceTier, CustomerPriceGroup, PriceRuleType, PriceRuleScope, DiscountType, PromotionType, PromotionStatus,
};

#[derive(Serialize)]
pub struct PriceBookResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub currency: String,
    pub is_default: bool,
}

pub async fn list_price_books(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<PriceBookResponse>>> {
    let service = PricingService::new();
    let books = service.list_price_books(&state.pool).await?;
    Ok(Json(books.into_iter().map(|b| PriceBookResponse {
        id: b.base.id,
        name: b.name,
        code: b.code,
        currency: b.currency,
        is_default: b.is_default,
    }).collect()))
}

#[derive(Deserialize)]
pub struct CreatePriceBookRequest {
    pub name: String,
    pub code: String,
    pub currency: String,
}

pub async fn create_price_book(
    State(state): State<AppState>,
    Json(req): Json<CreatePriceBookRequest>,
) -> ApiResult<Json<PriceBookResponse>> {
    let service = PricingService::new();
    let book = service.create_price_book(&state.pool, req.name, req.code, req.currency).await?;
    Ok(Json(PriceBookResponse {
        id: book.base.id,
        name: book.name,
        code: book.code,
        currency: book.currency,
        is_default: book.is_default,
    }))
}

#[derive(Deserialize)]
pub struct SetProductPriceRequest {
    pub price_book_id: Uuid,
    pub product_id: Uuid,
    pub unit_price: i64,
    pub currency: String,
}

#[derive(Serialize)]
pub struct PriceResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub unit_price: f64,
    pub currency: String,
}

pub async fn set_product_price(
    State(state): State<AppState>,
    Json(req): Json<SetProductPriceRequest>,
) -> ApiResult<Json<PriceResponse>> {
    let service = PricingService::new();
    let entry = service.set_product_price(&state.pool, req.price_book_id, req.product_id, req.unit_price, req.currency).await?;
    Ok(Json(PriceResponse {
        id: entry.base.id,
        product_id: entry.product_id,
        unit_price: entry.unit_price as f64 / 100.0,
        currency: entry.currency,
    }))
}

#[derive(Deserialize)]
pub struct CalculatePriceRequest {
    pub price_book_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

pub async fn calculate_price(
    State(state): State<AppState>,
    Json(req): Json<CalculatePriceRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = PricingService::new();
    let total = service.calculate_price(&state.pool, req.price_book_id, req.product_id, req.quantity).await?;
    Ok(Json(serde_json::json!({
        "total": total as f64 / 100.0
    })))
}

#[derive(Deserialize)]
pub struct CreateDiscountRequest {
    pub name: String,
    pub code: String,
    pub discount_type: String,
    pub value: f64,
    pub requires_code: bool,
}

#[derive(Serialize)]
pub struct DiscountResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub discount_type: String,
    pub value: f64,
}

pub async fn list_discounts(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<DiscountResponse>>> {
    let _service = PricingService::new();
    let _ = state;
    Ok(Json(vec![]))
}

pub async fn create_discount(
    State(state): State<AppState>,
    Json(req): Json<CreateDiscountRequest>,
) -> ApiResult<Json<DiscountResponse>> {
    let service = PricingService::new();
    let discount_type = match req.discount_type.as_str() {
        "FixedAmount" => DiscountType::FixedAmount,
        "BuyXGetY" => DiscountType::BuyXGetY,
        _ => DiscountType::Percentage,
    };
    let discount = Discount {
        base: BaseEntity::new(),
        name: req.name,
        code: req.code,
        description: None,
        discount_type,
        value: req.value,
        max_discount: None,
        min_order_value: None,
        applicable_to: None,
        customer_groups: None,
        products: None,
        categories: None,
        valid_from: None,
        valid_to: None,
        usage_limit: None,
        usage_per_customer: None,
        current_usage: 0,
        is_active: true,
        requires_code: req.requires_code,
        auto_apply: false,
    };
    let created = service.create_discount(&state.pool, discount).await?;
    Ok(Json(DiscountResponse {
        id: created.base.id,
        name: created.name,
        code: created.code,
        discount_type: format!("{:?}", created.discount_type),
        value: created.value,
    }))
}

#[derive(Deserialize)]
pub struct ValidateDiscountRequest {
    pub code: String,
}

pub async fn validate_discount(
    State(state): State<AppState>,
    Json(req): Json<ValidateDiscountRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = PricingService::new();
    let discount = service.validate_discount(&state.pool, &req.code).await?;
    Ok(Json(serde_json::json!({
        "valid": discount.is_some(),
        "discount": discount.map(|d| serde_json::json!({
            "id": d.base.id,
            "name": d.name,
            "value": d.value
        }))
    })))
}

#[derive(Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub discount_id: Uuid,
}

#[derive(Serialize)]
pub struct CouponResponse {
    pub id: Uuid,
    pub code: String,
    pub discount_id: Uuid,
}

pub async fn create_coupon(
    State(state): State<AppState>,
    Json(req): Json<CreateCouponRequest>,
) -> ApiResult<Json<CouponResponse>> {
    let service = PricingService::new();
    let coupon = service.create_coupon(&state.pool, req.code, req.discount_id).await?;
    Ok(Json(CouponResponse {
        id: coupon.base.id,
        code: coupon.code,
        discount_id: coupon.discount_id,
    }))
}

#[derive(Deserialize)]
pub struct CreatePromotionRequest {
    pub name: String,
    pub code: String,
    pub start_date: String,
    pub end_date: String,
    pub rules: String,
    pub rewards: String,
}

#[derive(Serialize)]
pub struct PromotionResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub status: String,
}

pub async fn list_promotions(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<PromotionResponse>>> {
    let service = PricingService::new();
    let promos = service.list_promotions(&state.pool).await?;
    Ok(Json(promos.into_iter().map(|p| PromotionResponse {
        id: p.base.id,
        name: p.name,
        code: p.code,
        status: format!("{:?}", p.status),
    }).collect()))
}

pub async fn create_promotion(
    State(state): State<AppState>,
    Json(req): Json<CreatePromotionRequest>,
) -> ApiResult<Json<PromotionResponse>> {
    let service = PricingService::new();
    let start_date = chrono::DateTime::parse_from_rfc3339(&req.start_date)
        .map_err(|_| erp_core::Error::validation("Invalid start date"))?
        .with_timezone(&chrono::Utc);
    let end_date = chrono::DateTime::parse_from_rfc3339(&req.end_date)
        .map_err(|_| erp_core::Error::validation("Invalid end date"))?
        .with_timezone(&chrono::Utc);
    
    let promo = Promotion {
        base: BaseEntity::new(),
        name: req.name,
        code: req.code,
        description: None,
        promotion_type: PromotionType::ProductDiscount,
        status: PromotionStatus::Draft,
        start_date,
        end_date,
        rules: req.rules,
        rewards: req.rewards,
        target_segments: None,
        channels: None,
        budget: None,
        spent: 0,
        usage_limit: None,
        current_usage: 0,
    };
    let created = service.create_promotion(&state.pool, promo).await?;
    Ok(Json(PromotionResponse {
        id: created.base.id,
        name: created.name,
        code: created.code,
        status: format!("{:?}", created.status),
    }))
}

#[derive(Deserialize)]
pub struct CreatePriceTierRequest {
    pub product_id: Uuid,
    pub min_quantity: i32,
    pub max_quantity: Option<i32>,
    pub unit_price: i64,
    pub currency: String,
}

pub async fn create_price_tier(
    State(state): State<AppState>,
    Json(req): Json<CreatePriceTierRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = PricingService::new();
    service.create_price_tier(&state.pool, req.product_id, req.min_quantity, req.max_quantity, req.unit_price, req.currency).await?;
    Ok(Json(serde_json::json!({ "status": "created" })))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/price-books", axum::routing::get(list_price_books).post(create_price_book))
        .route("/prices", axum::routing::post(set_product_price))
        .route("/prices/calculate", axum::routing::post(calculate_price))
        .route("/discounts", axum::routing::get(list_discounts).post(create_discount))
        .route("/discounts/validate", axum::routing::post(validate_discount))
        .route("/coupons", axum::routing::post(create_coupon))
        .route("/promotions", axum::routing::get(list_promotions).post(create_promotion))
        .route("/price-tiers", axum::routing::post(create_price_tier))
}
