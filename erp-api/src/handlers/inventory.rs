use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Pagination};
use erp_inventory::{Product, ProductType, Warehouse, StockMovement, StockLevel, MovementType, 
                    ProductService, WarehouseService, StockService};

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub product_type: Option<String>,
    pub category_id: Option<Uuid>,
    pub unit_of_measure: String,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub product_type: String,
    pub category_id: Option<Uuid>,
    pub unit_of_measure: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Product> for ProductResponse {
    fn from(p: Product) -> Self {
        Self {
            id: p.base.id,
            sku: p.sku,
            name: p.name,
            description: p.description,
            product_type: format!("{:?}", p.product_type),
            category_id: p.category_id,
            unit_of_measure: p.unit_of_measure,
            status: format!("{:?}", p.status),
            created_at: p.base.created_at.to_rfc3339(),
            updated_at: p.base.updated_at.to_rfc3339(),
        }
    }
}

pub async fn list_products(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<ProductResponse>>> {
    let service = ProductService::new();
    let result = service.list_products(&state.pool, pagination).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(ProductResponse::from).collect(),
        result.total,
        erp_core::Pagination { page: result.page, per_page: result.per_page }
    )))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ProductResponse>> {
    let service = ProductService::new();
    let product = service.get_product(&state.pool, id).await?;
    Ok(Json(ProductResponse::from(product)))
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(req): Json<CreateProductRequest>,
) -> ApiResult<Json<ProductResponse>> {
    let service = ProductService::new();
    
    let product = Product {
        base: BaseEntity::new(),
        sku: req.sku,
        name: req.name,
        description: req.description,
        product_type: match req.product_type.as_deref() {
            Some("Service") => ProductType::Service,
            Some("Digital") => ProductType::Digital,
            _ => ProductType::Goods,
        },
        category_id: req.category_id,
        unit_of_measure: req.unit_of_measure,
        status: Status::Active,
    };
    
    let created = service.create_product(&state.pool, product).await?;
    Ok(Json(ProductResponse::from(created)))
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateProductRequest>,
) -> ApiResult<Json<ProductResponse>> {
    let service = ProductService::new();
    
    let mut product = service.get_product(&state.pool, id).await?;
    product.sku = req.sku;
    product.name = req.name;
    product.description = req.description;
    product.product_type = match req.product_type.as_deref() {
        Some("Service") => ProductType::Service,
        Some("Digital") => ProductType::Digital,
        _ => ProductType::Goods,
    };
    product.category_id = req.category_id;
    product.unit_of_measure = req.unit_of_measure;
    
    let updated = service.update_product(&state.pool, product).await?;
    Ok(Json(ProductResponse::from(updated)))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<()>> {
    let service = ProductService::new();
    service.delete_product(&state.pool, id).await?;
    Ok(Json(()))
}

#[derive(Debug, Deserialize)]
pub struct CreateWarehouseRequest {
    pub code: String,
    pub name: String,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WarehouseResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub address: AddressResponse,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
}

impl From<Warehouse> for WarehouseResponse {
    fn from(w: Warehouse) -> Self {
        Self {
            id: w.base.id,
            code: w.code,
            name: w.name,
            address: AddressResponse {
                street: w.address.street,
                city: w.address.city,
                state: w.address.state,
                postal_code: w.address.postal_code,
                country: w.address.country,
            },
            status: format!("{:?}", w.status),
        }
    }
}

pub async fn list_warehouses(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<WarehouseResponse>>> {
    let service = WarehouseService::new();
    let warehouses = service.list_warehouses(&state.pool).await?;
    Ok(Json(warehouses.into_iter().map(WarehouseResponse::from).collect()))
}

pub async fn get_warehouse(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WarehouseResponse>> {
    let service = WarehouseService::new();
    let warehouse = service.get_warehouse(&state.pool, id).await?;
    Ok(Json(WarehouseResponse::from(warehouse)))
}

pub async fn create_warehouse(
    State(state): State<AppState>,
    Json(req): Json<CreateWarehouseRequest>,
) -> ApiResult<Json<WarehouseResponse>> {
    let service = WarehouseService::new();
    
    use erp_core::Address;
    let warehouse = Warehouse {
        base: BaseEntity::new(),
        code: req.code,
        name: req.name,
        address: Address {
            street: req.street.unwrap_or_default(),
            city: req.city.unwrap_or_default(),
            state: req.state,
            postal_code: req.postal_code.unwrap_or_default(),
            country: req.country.unwrap_or_default(),
        },
        status: Status::Active,
    };
    
    let created = service.create_warehouse(&state.pool, warehouse).await?;
    Ok(Json(WarehouseResponse::from(created)))
}

#[derive(Debug, Deserialize)]
pub struct CreateStockMovementRequest {
    pub product_id: Uuid,
    pub to_location_id: Uuid,
    pub from_location_id: Option<Uuid>,
    pub quantity: i64,
    pub movement_type: String,
    pub reference: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StockMovementResponse {
    pub id: Uuid,
    pub movement_number: String,
    pub movement_type: String,
    pub product_id: Uuid,
    pub to_location_id: Uuid,
    pub from_location_id: Option<Uuid>,
    pub quantity: i64,
    pub reference: Option<String>,
    pub date: String,
}

pub async fn create_stock_movement(
    State(state): State<AppState>,
    Json(req): Json<CreateStockMovementRequest>,
) -> ApiResult<Json<StockMovementResponse>> {
    let service = StockService::new();
    
    let movement = StockMovement {
        base: BaseEntity::new(),
        movement_number: String::new(),
        movement_type: match req.movement_type.as_str() {
            "Issue" => MovementType::Issue,
            "Transfer" => MovementType::Transfer,
            "Adjustment" => MovementType::Adjustment,
            _ => MovementType::Receipt,
        },
        product_id: req.product_id,
        from_location_id: req.from_location_id,
        to_location_id: req.to_location_id,
        quantity: req.quantity,
        reference: req.reference,
        date: chrono::Utc::now(),
    };
    
    let created = service.record_movement(&state.pool, movement).await?;
    
    Ok(Json(StockMovementResponse {
        id: created.base.id,
        movement_number: created.movement_number,
        movement_type: format!("{:?}", created.movement_type),
        product_id: created.product_id,
        to_location_id: created.to_location_id,
        from_location_id: created.from_location_id,
        quantity: created.quantity,
        reference: created.reference,
        date: created.date.to_rfc3339(),
    }))
}

#[derive(Debug, Serialize)]
pub struct StockLevelResponse {
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub quantity: i64,
    pub reserved_quantity: i64,
    pub available_quantity: i64,
}

impl From<StockLevel> for StockLevelResponse {
    fn from(s: StockLevel) -> Self {
        Self {
            product_id: s.product_id,
            location_id: s.location_id,
            quantity: s.quantity,
            reserved_quantity: s.reserved_quantity,
            available_quantity: s.available_quantity,
        }
    }
}

pub async fn get_stock(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> ApiResult<Json<Vec<StockLevelResponse>>> {
    let service = StockService::new();
    let levels = service.get_product_stock(&state.pool, product_id).await?;
    Ok(Json(levels.into_iter().map(StockLevelResponse::from).collect()))
}
