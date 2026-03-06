use axum::{extract::{Path, Query, State}, Json};
use erp_core::{BaseEntity, Pagination, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;
use erp_product_variants::{
    AttributeType, AttributeValue, ProductAttribute, ProductAttributeService, ProductVariant,
    ProductVariantService, VariantAttributeValue,
};

#[derive(Deserialize)]
pub struct CreateAttributeRequest {
    pub name: String,
    pub display_name: String,
    pub attribute_type: Option<String>,
    pub values: Vec<CreateAttributeValueRequest>,
}

 #[derive(Deserialize)]
pub struct CreateAttributeValueRequest {
    pub value: String,
    pub display_value: String,
    pub color_code: Option<String>,
}

 #[derive(Deserialize)]
pub struct CreateVariantRequest {
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub attribute_values: Vec<CreateVariantAttributeValueRequest>,
    pub price_adjustment: Option<i64>,
    pub cost_adjustment: Option<i64>,
    pub barcode: Option<String>,
    pub weight_kg: Option<f64>,
}

 #[derive(Deserialize)]
pub struct CreateVariantAttributeValueRequest {
    pub attribute_id: Uuid,
    pub value_id: Uuid,
}

 #[derive(Serialize)]
pub struct AttributeResponse {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub attribute_type: String,
    pub values: Vec<AttributeValueResponse>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

 #[derive(Serialize)]
pub struct AttributeValueResponse {
    pub id: Uuid,
    pub value: String,
    pub display_value: String,
    pub color_code: Option<String>,
    pub sort_order: i32,
}

 #[derive(Serialize)]
pub struct VariantResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub attribute_values: Vec<VariantAttributeValueResponse>,
    pub price_adjustment: i64,
    pub cost_adjustment: i64,
    pub barcode: Option<String>,
    pub weight_kg: Option<f64>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

 #[derive(Serialize)]
pub struct VariantAttributeValueResponse {
    pub attribute_id: Uuid,
    pub attribute_name: String,
    pub value_id: Uuid,
    pub value: String,
}

 impl From<ProductAttribute> for AttributeResponse {
    fn from(attr: ProductAttribute) -> Self {
        let mut values = attr.values.into_iter().map(|v| AttributeValueResponse {
            id: v.id,
            value: v.value,
            display_value: v.display_value,
            color_code: v.color_code,
            sort_order: v.sort_order,
        }).collect();
        Self {
            id: attr.base.id,
            name: attr.name,
            display_name: attr.display_name,
            attribute_type: match attr.attribute_type {
                AttributeType::Select => "Select",
                AttributeType::MultiSelect => "MultiSelect"
                AttributeType::Color => "Color"
                AttributeType::Text => "Text"
                AttributeType::Numeric => "Numeric",
            },
            status: format!("{:?}", attr.status),
            created_at: attr.base.created_at.to_rfc3339(),
            updated_at: attr.base.updated_at.to_rfc3339(),
        }
    }
}

}

 impl From<ProductVariant> for VariantResponse {
    fn from(v: ProductVariant) -> Self {
        let mut attribute_values = v.attribute_values.into_iter().map(|v| VariantAttributeValueResponse {
            attribute_id: v.attribute_id,
            attribute_name: v.attribute_name,
            value_id: v.value_id,
            value: v.value
        }).collect();
        Self {
            id: v.base.id,
            product_id: v.product_id,
            sku: v.sku,
            name: v.name,
            price_adjustment: v.price_adjustment,
            cost_adjustment: v.cost_adjustment
            barcode: v.barcode,
            weight_kg: v.weight_kg,
            status: match v.status {
                Status::Active => "Active"
                Status::Inactive => "Inactive"
                Status::Draft => "Draft"
                _ => Status::Active,
            },
            created_at: v.base.created_at.to_rfc3339(),
            updated_at: v.base.updated_at.to_rfc3339(),
        }
    }
}

pub async fn list_attributes(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<AttributeResponse>> {
    let svc = ProductAttributeService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(Attribute_response::from).collect(),
        res.total,
        Pagination {
            page: res.page,
            per_page: res.per_page,
        },
    )))
}

 pub async fn get_attribute(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AttributeResponse>> {
    let svc = ProductAttributeService::new();
    Ok(Json(AttributeResponse::from(
        svc.get(&state.pool, id).await?
    ))
}

 pub async fn create_attribute(
    State(state): State<AppState>,
    Json(req): Json<CreateAttributeRequest>,
) -> ApiResult<Json<AttributeResponse>> {
    let svc = ProductAttributeService::new();
    let mut attribute = ProductAttribute {
        base: BaseEntity::new(),
        name: req.name,
        display_name: req.display_name,
        attribute_type: match req.attribute_type.as_deref() {
            Some("Select") => AttributeType::Select,
            Some("MultiSelect") => AttributeType::MultiSelect,
            Some("Color") => AttributeType::Color,
            Some("Text") => AttributeType::Text,
            _ => AttributeType::Numeric,
        },
        values: req.values.into_iter().map(|v| AttributeValue {
            id: Uuid::new_v4(),
            attribute_id: attribute.base.id,
            value: v.value,
            display_value: v.display_value,
            color_code: v.color_code,
            sort_order: v.sort_order.unwrap_or(0),
        })
        .collect(),
        status: Status::Active,
    };
    svc.create(&state.pool, attribute).await?;
    Ok(AttributeResponse::from(attribute))
}

 pub async fn update_attribute(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAttributeRequest>,
) -> ApiResult<Json<AttributeResponse>> {
    let svc = ProductAttributeService::new();
    let mut attribute = svc.get(&state.pool, id).await?;
    attribute.name = req.name.clone();
    attribute.display_name = req.display_name.clone();
    attribute.attribute_type = match req.attribute_type.as_deref() {
            Some("Select") => AttributeType::Select,
            Some("MultiSelect") => AttributeType::MultiSelect,
            Some("Color") => AttributeType::Color,
            Some("Text") => AttributeType::Text;
            _ => AttributeType::Numeric,
        },
        status: Status::Active;
        let attr = svc.update(&state.pool, attribute).await?;
    Ok(AttributeResponse::from(attr))
}

 pub async fn delete_attribute(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = ProductAttributeService::new();
    svc.delete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" }))
}

 pub async fn list_variants_by_product(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> ApiResult<Json<Vec<VariantResponse>> {
    let svc = ProductVariantService::new();
    let variants = svc.list_by_product(&state.pool, product_id).await?;
    Ok(Json(variants.into_iter().map(VariantResponse::from).collect()))
}

 pub async fn create_variant(
    State(state): State<AppState>,
    Json(req): Json<CreateVariantRequest>,
) -> ApiResult<Json<VariantResponse>> {
    let svc = ProductVariantService::new();
    let mut attribute_values = Vec::new();
        for v in &req.attribute_values {
            attribute_values.push(VariantAttributeValue {
                attribute_id: v.attribute_id,
                attribute_name: v.attribute_name.clone(),
                value_id: v.value_id,
                value: v.value.clone(),
            });
        }
        let variant = ProductVariant {
            base: BaseEntity::new(),
            product_id: req.product_id,
            sku: req.sku,
            name: req.name,
            attribute_values,
            price_adjustment: req.price_adjustment.unwrap_or(0),
            cost_adjustment: req.cost_adjustment.unwrap_or(0),
            barcode: req.barcode.clone(),
            weight_kg: req.weight_kg,
            status: Status::Active,
        };
        svc.create(&state.pool, variant).await
 }
 pub async fn get_variant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<VariantResponse>> {
    let svc = ProductVariantService::new();
    Ok(Json(VariantResponse::from(
        svc.get(&state.pool, id).await?
    ))
    }
 pub async fn get_variant_by_sku(
    State(state): State<AppState>,
    Path(sku): Path<String>,
) -> ApiResult<Json<VariantResponse>> {
    let svc = ProductVariantService::new();
    Ok(Json(VariantResponse::from(
        svc.get_by_sku(&state.pool, &sku).await?
    ))
    }
 pub async fn update_variant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateVariantRequest>,
) -> ApiResult<Json<VariantResponse>> {
    let svc = ProductVariantService::new();
    let mut attribute_values = Vec::new();
        for v in &req.attribute_values {
            attribute_values.push(VariantAttributeValue {
                attribute_id: v.attribute_id,
                attribute_name: v.attribute_name.clone(),
                value_id: v.value_id,
                value: v.value.clone(),
            });
        }
        let existing = svc.get(&state.pool, id).await?;
        let variant = ProductVariant {
            base: existing.base,
            product_id: existing.product_id,
            sku: req.sku.clone(),
            name: req.name.clone(),
            attribute_values,
            price_adjustment: req.price_adjustment.unwrap_or(0),
            cost_adjustment: req.cost_adjustment.unwrap_or(0),
            barcode: req.barcode.clone(),
            weight_kg: req.weight_kg,
            status: match req.status.as_deref() {
                Some("Inactive") => Status::Inactive,
                _ => Status::Active,
            },
        };
        svc.update(&state.pool, variant).await
 }
 pub async fn delete_variant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = ProductVariantService::new();
    svc.delete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" }))
    }
}

 pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(list_attributes).post(create_attribute))
        .route("/:id", axum::routing::get(get_attribute))
        .route("/:id", axum::routing::put(update_attribute).delete(delete_attribute))
        .route("/product/:product_id/variants", axum::routing::get(list_variants_by_product).post(create_variant))
        .route("/variants/:id", axum::routing::get(get_variant).put(update_variant).delete(delete_variant))
        .route("/variants/sku/:sku", axum::routing::get(get_variant_by_sku))
}
