use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::Pagination;
use erp_promotions::{
    PromotionService, CouponService, PromotionReportingService,
    CreatePromotionRequest, UpdatePromotionRequest, CreateCouponRequest, GenerateCouponBatchRequest,
    Promotion, Coupon,
};

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

#[derive(Serialize)]
pub struct PromotionListResponse {
    pub items: Vec<PromotionResponse>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

pub async fn list_promotions(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<PromotionListResponse>> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
    };
    
    let svc = PromotionService::new();
    let result = svc.list(&state.pool, pagination).await?;
    
    Ok(Json(PromotionListResponse {
        items: result.items.into_iter().map(PromotionResponse::from).collect(),
        total: result.total as i64,
        page: result.page,
        per_page: result.per_page,
        total_pages: result.total_pages,
    }))
}

pub async fn get_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PromotionResponse>> {
    let svc = PromotionService::new();
    let promo = svc.get(&state.pool, id).await?;
    Ok(Json(PromotionResponse::from(promo)))
}

pub async fn create_promotion(
    State(state): State<AppState>,
    Json(req): Json<CreatePromotionRequest>,
) -> ApiResult<Json<PromotionResponse>> {
    let svc = PromotionService::new();
    let promo = svc.create(&state.pool, req).await?;
    Ok(Json(PromotionResponse::from(promo)))
}

pub async fn update_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePromotionRequest>,
) -> ApiResult<Json<PromotionResponse>> {
    let svc = PromotionService::new();
    let promo = svc.update(&state.pool, id, req).await?;
    Ok(Json(PromotionResponse::from(promo)))
}

pub async fn activate_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PromotionResponse>> {
    let svc = PromotionService::new();
    let promo = svc.activate(&state.pool, id).await?;
    Ok(Json(PromotionResponse::from(promo)))
}

pub async fn deactivate_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PromotionResponse>> {
    let svc = PromotionService::new();
    let promo = svc.deactivate(&state.pool, id).await?;
    Ok(Json(PromotionResponse::from(promo)))
}

#[derive(Serialize)]
pub struct DiscountResponse {
    pub discount_amount: i64,
    pub final_amount: i64,
}

pub async fn calculate_promotion_discount(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CalculateDiscountRequest>,
) -> ApiResult<Json<DiscountResponse>> {
    let svc = PromotionService::new();
    let discount = svc.calculate_discount(&state.pool, id, req.order_amount, req.product_ids).await?;
    Ok(Json(DiscountResponse {
        discount_amount: discount,
        final_amount: req.order_amount - discount,
    }))
}

pub async fn get_promotion_report(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let report = PromotionReportingService::get_report(&state.pool, id).await?;
    Ok(Json(serde_json::to_value(report)?))
}

#[derive(Serialize)]
pub struct CouponListResponse {
    pub items: Vec<CouponResponse>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

pub async fn list_coupons(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<CouponListResponse>> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
    };
    
    let svc = CouponService::new();
    let result = svc.list(&state.pool, pagination).await?;
    
    Ok(Json(CouponListResponse {
        items: result.items.into_iter().map(CouponResponse::from).collect(),
        total: result.total as i64,
        page: result.page,
        per_page: result.per_page,
        total_pages: result.total_pages,
    }))
}

pub async fn get_coupon(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<CouponResponse>> {
    let svc = CouponService::new();
    let coupon = svc.get(&state.pool, id).await?;
    Ok(Json(CouponResponse::from(coupon)))
}

pub async fn create_coupon(
    State(state): State<AppState>,
    Json(req): Json<CreateCouponRequest>,
) -> ApiResult<Json<CouponResponse>> {
    let svc = CouponService::new();
    let coupon = svc.create(&state.pool, req).await?;
    Ok(Json(CouponResponse::from(coupon)))
}

#[derive(Serialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub coupon: Option<CouponResponse>,
    pub error_message: Option<String>,
    pub discount_amount: i64,
    pub final_amount: i64,
}

pub async fn validate_coupon(
    State(state): State<AppState>,
    Json(req): Json<ValidateCouponRequest>,
) -> ApiResult<Json<ValidationResponse>> {
    let svc = CouponService::new();
    let validation = svc.validate(
        &state.pool,
        &req.code,
        req.order_amount,
        req.customer_id,
        req.customer_email.as_deref(),
        req.is_first_order.unwrap_or(false),
    ).await?;
    
    let discount_amount = validation.discount_preview.as_ref().map(|p| p.discount_amount).unwrap_or(0);
    let final_amount = validation.discount_preview.as_ref().map(|p| p.final_amount).unwrap_or(req.order_amount);
    
    Ok(Json(ValidationResponse {
        valid: validation.valid,
        coupon: validation.coupon.map(CouponResponse::from),
        error_message: validation.error_message,
        discount_amount,
        final_amount,
    }))
}

#[derive(Serialize)]
pub struct ApplyCouponResponse {
    pub coupon: CouponResponse,
    pub discount_applied: i64,
    pub final_amount: i64,
}

pub async fn apply_coupon(
    State(state): State<AppState>,
    Json(req): Json<ApplyCouponRequest>,
) -> ApiResult<Json<ApplyCouponResponse>> {
    let svc = CouponService::new();
    let (coupon, discount) = svc.apply(
        &state.pool,
        &req.code,
        req.order_id,
        req.customer_id,
        req.order_amount,
    ).await?;
    
    Ok(Json(ApplyCouponResponse {
        coupon: CouponResponse::from(coupon),
        discount_applied: discount,
        final_amount: req.order_amount - discount,
    }))
}

#[derive(Serialize)]
pub struct BatchResponse {
    pub generated_count: usize,
    pub coupons: Vec<CouponResponse>,
}

pub async fn generate_coupon_batch_handler(
    State(state): State<AppState>,
    Json(req): Json<GenerateCouponBatchRequest>,
) -> ApiResult<Json<BatchResponse>> {
    let svc = CouponService::new();
    let coupons = svc.generate_batch(&state.pool, req).await?;
    
    Ok(Json(BatchResponse {
        generated_count: coupons.len(),
        coupons: coupons.into_iter().map(CouponResponse::from).collect(),
    }))
}
