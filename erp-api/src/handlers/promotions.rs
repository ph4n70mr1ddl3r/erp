use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_promotions::{
    PromotionService, CouponService, PromotionReportingService,
    CreatePromotionRequest, UpdatePromotionRequest, CreateCouponRequest, GenerateCouponBatchRequest,
    Promotion, Coupon, CouponValidation, PromotionReport,
};
use erp_core::{Pagination, Status};

#[derive(Deserialize)]
pub struct ListQuery {
    page: Option<u32>,
    per_page: Option<u32>,
}

#[derive(Deserialize)]
pub struct ValidateCouponRequest {
    pub code: String,
    pub order_amount: i64,
    pub customer_id: Option<Uuid>,
    pub customer_email: Option<String>,
    pub is_first_order: Option<bool>,
}

#[derive(Deserialize)]
pub struct ApplyCouponRequest {
    pub code: String,
    pub order_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub order_amount: i64,
}

#[derive(Deserialize)]
pub struct CalculateDiscountRequest {
    pub order_amount: i64,
    pub product_ids: Vec<Uuid>,
}

#[derive(Serialize)]
pub struct PromotionResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub promotion_type: String,
    pub discount_type: String,
    pub discount_value: i64,
    pub max_discount: Option<i64>,
    pub min_order_amount: Option<i64>,
    pub start_date: String,
    pub end_date: String,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub per_customer_limit: Option<i32>,
    pub stackable: bool,
    pub auto_apply: bool,
    pub priority: i32,
    pub status: String,
    pub created_at: String,
}

impl From<Promotion> for PromotionResponse {
    fn from(p: Promotion) -> Self {
        Self {
            id: p.base.id,
            code: p.code,
            name: p.name,
            description: p.description,
            promotion_type: format!("{:?}", p.promotion_type),
            discount_type: format!("{:?}", p.discount_type),
            discount_value: p.discount_value,
            max_discount: p.max_discount,
            min_order_amount: p.min_order_amount,
            start_date: p.start_date.to_rfc3339(),
            end_date: p.end_date.to_rfc3339(),
            usage_limit: p.usage_limit,
            usage_count: p.usage_count,
            per_customer_limit: p.per_customer_limit,
            stackable: p.stackable,
            auto_apply: p.auto_apply,
            priority: p.priority,
            status: format!("{:?}", p.status),
            created_at: p.base.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct CouponResponse {
    pub id: Uuid,
    pub code: String,
    pub promotion_id: Uuid,
    pub coupon_type: String,
    pub discount_type: String,
    pub discount_value: i64,
    pub max_discount: Option<i64>,
    pub min_order_amount: Option<i64>,
    pub start_date: String,
    pub end_date: String,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub per_customer_limit: Option<i32>,
    pub customer_email: Option<String>,
    pub first_time_only: bool,
    pub status: String,
    pub created_at: String,
}

impl From<Coupon> for CouponResponse {
    fn from(c: Coupon) -> Self {
        Self {
            id: c.base.id,
            code: c.code,
            promotion_id: c.promotion_id,
            coupon_type: format!("{:?}", c.coupon_type),
            discount_type: format!("{:?}", c.discount_type),
            discount_value: c.discount_value,
            max_discount: c.max_discount,
            min_order_amount: c.min_order_amount,
            start_date: c.start_date.to_rfc3339(),
            end_date: c.end_date.to_rfc3339(),
            usage_limit: c.usage_limit,
            usage_count: c.usage_count,
            per_customer_limit: c.per_customer_limit,
            customer_email: c.customer_email,
            first_time_only: c.first_time_only,
            status: format!("{:?}", c.status),
            created_at: c.base.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_promotions(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Json<serde_json::Value> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
    };
    
    let svc = PromotionService::new();
    match svc.list(&state.pool, pagination).await {
        Ok(result) => Json(serde_json::json!({
            "items": result.items.into_iter().map(PromotionResponse::from).collect::<Vec<_>>(),
            "total": result.total,
            "page": result.page,
            "per_page": result.per_page,
            "total_pages": result.total_pages
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn get_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let svc = PromotionService::new();
    match svc.get(&state.pool, id).await {
        Ok(promo) => Json(serde_json::to_value(PromotionResponse::from(promo)).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn create_promotion(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let promo_type = req.get("promotion_type").and_then(|v| v.as_str()).unwrap_or("OrderDiscount");
    let disc_type = req.get("discount_type").and_then(|v| v.as_str()).unwrap_or("Percentage");
    
    let request = CreatePromotionRequest {
        code: req.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        name: req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        description: req.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
        promotion_type: match promo_type {
            "LineItemDiscount" => erp_promotions::PromotionType::LineItemDiscount,
            "BuyXGetY" => erp_promotions::PromotionType::BuyXGetY,
            "FreeShipping" => erp_promotions::PromotionType::FreeShipping,
            "BundleDiscount" => erp_promotions::PromotionType::BundleDiscount,
            "LoyaltyPoints" => erp_promotions::PromotionType::LoyaltyPoints,
            _ => erp_promotions::PromotionType::OrderDiscount,
        },
        discount_type: match disc_type {
            "FixedAmount" => erp_promotions::DiscountType::FixedAmount,
            "FixedPrice" => erp_promotions::DiscountType::FixedPrice,
            _ => erp_promotions::DiscountType::Percentage,
        },
        discount_value: req.get("discount_value").and_then(|v| v.as_i64()).unwrap_or(0),
        max_discount: req.get("max_discount").and_then(|v| v.as_i64()),
        min_order_amount: req.get("min_order_amount").and_then(|v| v.as_i64()),
        start_date: req.get("start_date").and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now),
        end_date: req.get("end_date").and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now() + chrono::Duration::days(30)),
        usage_limit: req.get("usage_limit").and_then(|v| v.as_i64()).map(|v| v as i32),
        per_customer_limit: req.get("per_customer_limit").and_then(|v| v.as_i64()).map(|v| v as i32),
        applies_to: None,
        stackable: req.get("stackable").and_then(|v| v.as_bool()),
        auto_apply: req.get("auto_apply").and_then(|v| v.as_bool()),
        priority: req.get("priority").and_then(|v| v.as_i64()).map(|v| v as i32),
    };

    let svc = PromotionService::new();
    match svc.create(&state.pool, request).await {
        Ok(promo) => Json(serde_json::to_value(PromotionResponse::from(promo)).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn update_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let request = UpdatePromotionRequest {
        name: req.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
        description: req.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
        discount_value: req.get("discount_value").and_then(|v| v.as_i64()),
        max_discount: req.get("max_discount").and_then(|v| v.as_i64()),
        min_order_amount: req.get("min_order_amount").and_then(|v| v.as_i64()),
        start_date: req.get("start_date").and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&chrono::Utc)),
        end_date: req.get("end_date").and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&chrono::Utc)),
        usage_limit: req.get("usage_limit").and_then(|v| v.as_i64()).map(|v| v as i32),
        per_customer_limit: req.get("per_customer_limit").and_then(|v| v.as_i64()).map(|v| v as i32),
        stackable: req.get("stackable").and_then(|v| v.as_bool()),
        auto_apply: req.get("auto_apply").and_then(|v| v.as_bool()),
        priority: req.get("priority").and_then(|v| v.as_i64()).map(|v| v as i32),
        applies_to: None,
    };

    let svc = PromotionService::new();
    match svc.update(&state.pool, id, request).await {
        Ok(promo) => Json(serde_json::to_value(PromotionResponse::from(promo)).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn activate_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let svc = PromotionService::new();
    match svc.activate(&state.pool, id).await {
        Ok(promo) => Json(serde_json::to_value(PromotionResponse::from(promo)).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn deactivate_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let svc = PromotionService::new();
    match svc.deactivate(&state.pool, id).await {
        Ok(promo) => Json(serde_json::to_value(PromotionResponse::from(promo)).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn calculate_promotion_discount(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CalculateDiscountRequest>,
) -> Json<serde_json::Value> {
    let svc = PromotionService::new();
    match svc.calculate_discount(&state.pool, id, req.order_amount, req.product_ids).await {
        Ok(discount) => Json(serde_json::json!({
            "discount_amount": discount,
            "final_amount": req.order_amount - discount
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn get_promotion_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    match PromotionReportingService::get_report(&state.pool, id).await {
        Ok(report) => Json(serde_json::to_value(report).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn list_coupons(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Json<serde_json::Value> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
    };
    
    let svc = CouponService::new();
    match svc.list(&state.pool, pagination).await {
        Ok(result) => Json(serde_json::json!({
            "items": result.items.into_iter().map(CouponResponse::from).collect::<Vec<_>>(),
            "total": result.total,
            "page": result.page,
            "per_page": result.per_page,
            "total_pages": result.total_pages
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn get_coupon(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let svc = CouponService::new();
    match svc.get(&state.pool, id).await {
        Ok(coupon) => Json(serde_json::to_value(CouponResponse::from(coupon)).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn create_coupon(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let coupon_type = req.get("coupon_type").and_then(|v| v.as_str()).unwrap_or("SingleUse");
    let disc_type = req.get("discount_type").and_then(|v| v.as_str()).unwrap_or("Percentage");
    
    let request = CreateCouponRequest {
        code: req.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        promotion_id: req.get("promotion_id").and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_default(),
        coupon_type: match coupon_type {
            "MultiUse" => erp_promotions::CouponType::MultiUse,
            "Unlimited" => erp_promotions::CouponType::Unlimited,
            "Referral" => erp_promotions::CouponType::Referral,
            _ => erp_promotions::CouponType::SingleUse,
        },
        discount_type: match disc_type {
            "FixedAmount" => erp_promotions::DiscountType::FixedAmount,
            "FixedPrice" => erp_promotions::DiscountType::FixedPrice,
            _ => erp_promotions::DiscountType::Percentage,
        },
        discount_value: req.get("discount_value").and_then(|v| v.as_i64()).unwrap_or(0),
        max_discount: req.get("max_discount").and_then(|v| v.as_i64()),
        min_order_amount: req.get("min_order_amount").and_then(|v| v.as_i64()),
        start_date: req.get("start_date").and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now),
        end_date: req.get("end_date").and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now() + chrono::Duration::days(30)),
        usage_limit: req.get("usage_limit").and_then(|v| v.as_i64()).map(|v| v as i32),
        per_customer_limit: req.get("per_customer_limit").and_then(|v| v.as_i64()).map(|v| v as i32),
        customer_email: req.get("customer_email").and_then(|v| v.as_str()).map(|s| s.to_string()),
        first_time_only: req.get("first_time_only").and_then(|v| v.as_bool()),
    };

    let svc = CouponService::new();
    match svc.create(&state.pool, request).await {
        Ok(coupon) => Json(serde_json::to_value(CouponResponse::from(coupon)).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn validate_coupon(
    State(state): State<AppState>,
    Json(req): Json<ValidateCouponRequest>,
) -> Json<serde_json::Value> {
    let svc = CouponService::new();
    match svc.validate(
        &state.pool,
        &req.code,
        req.order_amount,
        req.customer_id,
        req.customer_email.as_deref(),
        req.is_first_order.unwrap_or(false),
    ).await {
        Ok(validation) => Json(serde_json::to_value(validation).unwrap()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn apply_coupon(
    State(state): State<AppState>,
    Json(req): Json<ApplyCouponRequest>,
) -> Json<serde_json::Value> {
    let svc = CouponService::new();
    match svc.apply(
        &state.pool,
        &req.code,
        req.order_id,
        req.customer_id,
        req.order_amount,
    ).await {
        Ok((coupon, discount)) => Json(serde_json::json!({
            "coupon": CouponResponse::from(coupon),
            "discount_applied": discount,
            "final_amount": req.order_amount - discount
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn generate_coupon_batch_handler(
    state: State<AppState>,
    req: Json<serde_json::Value>,
) -> impl IntoResponse {
    let disc_type = req.get("discount_type").and_then(|v| v.as_str()).unwrap_or("Percentage");
    
    let request = GenerateCouponBatchRequest {
        promotion_id: req.get("promotion_id").and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_default(),
        prefix: req.get("prefix").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        quantity: req.get("quantity").and_then(|v| v.as_i64()).unwrap_or(10) as i32,
        length: req.get("length").and_then(|v| v.as_i64()).unwrap_or(8) as i32,
        discount_type: match disc_type {
            "FixedAmount" => erp_promotions::DiscountType::FixedAmount,
            "FixedPrice" => erp_promotions::DiscountType::FixedPrice,
            _ => erp_promotions::DiscountType::Percentage,
        },
        discount_value: req.get("discount_value").and_then(|v| v.as_i64()).unwrap_or(0),
        max_discount: req.get("max_discount").and_then(|v| v.as_i64()),
        min_order_amount: req.get("min_order_amount").and_then(|v| v.as_i64()),
        start_date: req.get("start_date").and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now),
        end_date: req.get("end_date").and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now() + chrono::Duration::days(30)),
        coupon_type: req.get("coupon_type").and_then(|v| v.as_str()).map(|s| match s {
            "MultiUse" => erp_promotions::CouponType::MultiUse,
            "Unlimited" => erp_promotions::CouponType::Unlimited,
            "Referral" => erp_promotions::CouponType::Referral,
            _ => erp_promotions::CouponType::SingleUse,
        }),
    };

    let svc = CouponService::new();
    match svc.generate_batch(&state.pool, request).await {
        Ok(coupons) => Json(serde_json::json!({
            "generated_count": coupons.len(),
            "coupons": coupons.into_iter().map(CouponResponse::from).collect::<Vec<_>>()
        })).into_response(),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })).into_response(),
    }
}
