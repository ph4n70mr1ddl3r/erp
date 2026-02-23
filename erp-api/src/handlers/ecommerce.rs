use axum::{extract::{Path, Query, State}, Json, routing::{get, post}};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Pagination, BaseEntity, Money, Currency, Status};
use erp_ecommerce::{
    EcommercePlatform, PlatformType, SyncDirection, SyncStatus,
    EcommerceOrder, EcommerceOrderStatus, FulfillmentStatus, PaymentStatus, EcommerceOrderLine,
    ProductListing, ProductListingStatus, ProductVisibility,
    EcommercePlatformService, EcommerceOrderService, ProductListingService, WebhookService,
};

#[derive(Serialize)]
pub struct PlatformResponse {
    pub id: Uuid,
    pub name: String,
    pub platform_type: String,
    pub status: String,
    pub last_sync_at: Option<String>,
}

impl From<EcommercePlatform> for PlatformResponse {
    fn from(p: EcommercePlatform) -> Self {
        Self {
            id: p.base.id,
            name: p.name,
            platform_type: format!("{:?}", p.platform_type),
            status: format!("{:?}", p.status),
            last_sync_at: p.last_sync_at.map(|d| d.to_rfc3339()),
        }
    }
}

#[derive(Deserialize)]
pub struct CreatePlatformRequest {
    pub name: String,
    pub platform_type: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub access_token: Option<String>,
    pub store_id: Option<String>,
    pub sync_direction: Option<String>,
    pub auto_sync: Option<bool>,
}

pub async fn list_platforms(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<PlatformResponse>>> {
    let svc = EcommercePlatformService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(PlatformResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn create_platform(
    State(state): State<AppState>,
    Json(req): Json<CreatePlatformRequest>,
) -> ApiResult<Json<PlatformResponse>> {
    let svc = EcommercePlatformService::new();
    let platform = EcommercePlatform {
        base: BaseEntity::new(),
        name: req.name,
        platform_type: match req.platform_type.as_str() {
            "WooCommerce" => PlatformType::WooCommerce,
            "Magento" => PlatformType::Magento,
            "BigCommerce" => PlatformType::BigCommerce,
            "Amazon" => PlatformType::Amazon,
            "EBay" => PlatformType::EBay,
            "Etsy" => PlatformType::Etsy,
            "Walmart" => PlatformType::Walmart,
            "Custom" => PlatformType::Custom,
            _ => PlatformType::Shopify,
        },
        base_url: req.base_url,
        api_key: req.api_key,
        api_secret: req.api_secret,
        access_token: req.access_token,
        webhook_secret: None,
        store_id: req.store_id,
        status: Status::Active,
        sync_direction: match req.sync_direction.as_deref() {
            Some("Export") => SyncDirection::Export,
            Some("Bidirectional") => SyncDirection::Bidirectional,
            _ => SyncDirection::Import,
        },
        last_sync_at: None,
        sync_interval_minutes: 60,
        auto_sync: req.auto_sync.unwrap_or(false),
    };
    Ok(Json(PlatformResponse::from(svc.create(&state.pool, platform).await?)))
}

#[derive(Serialize)]
pub struct EcommerceOrderResponse {
    pub id: Uuid,
    pub order_number: String,
    pub platform_id: Uuid,
    pub status: String,
    pub total: f64,
    pub sync_status: String,
}

impl From<EcommerceOrder> for EcommerceOrderResponse {
    fn from(o: EcommerceOrder) -> Self {
        Self {
            id: o.base.id,
            order_number: o.order_number,
            platform_id: o.platform_id,
            status: format!("{:?}", o.status),
            total: o.total.to_decimal(),
            sync_status: format!("{:?}", o.sync_status),
        }
    }
}

#[derive(Deserialize)]
pub struct ImportOrderRequest {
    pub platform_id: Uuid,
    pub external_order_id: String,
    pub order_number: String,
    pub subtotal: i64,
    pub shipping_amount: i64,
    pub tax_amount: i64,
    pub total: i64,
    pub currency: String,
    pub customer_email: Option<String>,
    pub billing_name: String,
    pub billing_address: String,
    pub billing_city: String,
    pub billing_state: String,
    pub billing_postal_code: String,
    pub billing_country: String,
    pub shipping_name: String,
    pub shipping_address: String,
    pub shipping_city: String,
    pub shipping_state: String,
    pub shipping_postal_code: String,
    pub shipping_country: String,
    pub lines: Vec<ImportOrderLineRequest>,
}

#[derive(Deserialize)]
pub struct ImportOrderLineRequest {
    pub sku: Option<String>,
    pub title: String,
    pub quantity: i64,
    pub unit_price: i64,
}

pub async fn list_orders(
    State(state): State<AppState>,
    Query(platform_id): Query<Option<Uuid>>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<EcommerceOrderResponse>>> {
    let svc = EcommerceOrderService::new();
    let res = svc.list(&state.pool, platform_id, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(EcommerceOrderResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn import_order(
    State(state): State<AppState>,
    Json(req): Json<ImportOrderRequest>,
) -> ApiResult<Json<EcommerceOrderResponse>> {
    let svc = EcommerceOrderService::new();
    
    let lines: Vec<EcommerceOrderLine> = req.lines.into_iter().map(|l| {
        EcommerceOrderLine {
            id: Uuid::new_v4(),
            order_id: Uuid::nil(),
            external_line_id: None,
            product_id: None,
            listing_id: None,
            sku: l.sku,
            title: l.title,
            variant_title: None,
            quantity: l.quantity,
            unit_price: Money::new(l.unit_price, Currency::USD),
            tax_amount: Money::zero(Currency::USD),
            discount_amount: Money::zero(Currency::USD),
            line_total: Money::new(l.quantity * l.unit_price, Currency::USD),
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            quantity_fulfilled: 0,
        }
    }).collect();
    
    let order = EcommerceOrder {
        base: BaseEntity::new(),
        platform_id: req.platform_id,
        external_order_id: req.external_order_id,
        order_number: req.order_number,
        customer_id: None,
        external_customer_id: None,
        sales_order_id: None,
        order_date: Utc::now(),
        status: EcommerceOrderStatus::Pending,
        fulfillment_status: FulfillmentStatus::Unfulfilled,
        payment_status: PaymentStatus::Pending,
        subtotal: Money::new(req.subtotal, Currency::USD),
        shipping_amount: Money::new(req.shipping_amount, Currency::USD),
        tax_amount: Money::new(req.tax_amount, Currency::USD),
        discount_amount: Money::zero(Currency::USD),
        total: Money::new(req.total, Currency::USD),
        currency: req.currency,
        billing_name: req.billing_name,
        billing_address: req.billing_address,
        billing_city: req.billing_city,
        billing_state: req.billing_state,
        billing_postal_code: req.billing_postal_code,
        billing_country: req.billing_country,
        shipping_name: req.shipping_name,
        shipping_address: req.shipping_address,
        shipping_city: req.shipping_city,
        shipping_state: req.shipping_state,
        shipping_postal_code: req.shipping_postal_code,
        shipping_country: req.shipping_country,
        shipping_method: None,
        tracking_number: None,
        customer_email: req.customer_email,
        customer_phone: None,
        notes: None,
        lines,
        sync_status: SyncStatus::Pending,
        imported_at: None,
    };
    
    Ok(Json(EcommerceOrderResponse::from(svc.import(&state.pool, order).await?)))
}

#[derive(Deserialize)]
pub struct LinkSalesOrderRequest {
    pub sales_order_id: Uuid,
}

pub async fn link_sales_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<LinkSalesOrderRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    EcommerceOrderService::new().link_sales_order(&state.pool, id, req.sales_order_id).await?;
    Ok(Json(serde_json::json!({ "status": "linked" })))
}

#[derive(Deserialize)]
pub struct UpdateFulfillmentRequest {
    pub tracking_number: String,
    pub carrier: Option<String>,
}

pub async fn update_fulfillment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateFulfillmentRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    EcommerceOrderService::new().update_fulfillment(&state.pool, id, &req.tracking_number, req.carrier.as_deref()).await?;
    Ok(Json(serde_json::json!({ "status": "shipped" })))
}

#[derive(Serialize)]
pub struct ProductListingResponse {
    pub id: Uuid,
    pub platform_id: Uuid,
    pub product_id: Uuid,
    pub title: String,
    pub price: f64,
    pub status: String,
    pub sync_status: String,
}

impl From<ProductListing> for ProductListingResponse {
    fn from(l: ProductListing) -> Self {
        Self {
            id: l.base.id,
            platform_id: l.platform_id,
            product_id: l.product_id,
            title: l.title,
            price: l.price.to_decimal(),
            status: format!("{:?}", l.status),
            sync_status: format!("{:?}", l.sync_status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateListingRequest {
    pub platform_id: Uuid,
    pub product_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: i64,
    pub sku: Option<String>,
    pub barcode: Option<String>,
}

pub async fn create_listing(
    State(state): State<AppState>,
    Json(req): Json<CreateListingRequest>,
) -> ApiResult<Json<ProductListingResponse>> {
    let svc = ProductListingService::new();
    let listing = ProductListing {
        base: BaseEntity::new(),
        platform_id: req.platform_id,
        product_id: req.product_id,
        external_product_id: String::new(),
        external_variant_id: None,
        title: req.title,
        description: req.description,
        price: Money::new(req.price, Currency::USD),
        compare_at_price: None,
        quantity: 0,
        sku: req.sku,
        barcode: req.barcode,
        status: ProductListingStatus::Draft,
        visibility: ProductVisibility::Visible,
        seo_title: None,
        seo_description: None,
        tags: None,
        category: None,
        images: None,
        sync_status: SyncStatus::Pending,
        last_sync_at: None,
        sync_error: None,
    };
    Ok(Json(ProductListingResponse::from(svc.create(&state.pool, listing).await?)))
}

pub async fn publish_listing(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ProductListingResponse>> {
    let svc = ProductListingService::new();
    Ok(Json(ProductListingResponse::from(svc.publish(&state.pool, id).await?)))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/platforms", get(list_platforms).post(create_platform))
        .route("/orders", get(list_orders).post(import_order))
        .route("/orders/:id/link", post(link_sales_order))
        .route("/orders/:id/fulfillment", post(update_fulfillment))
        .route("/listings", post(create_listing))
        .route("/listings/:id/publish", post(publish_listing))
}
