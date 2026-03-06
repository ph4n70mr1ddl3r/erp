use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttributeType {
    Select,
    MultiSelect,
    Color,
    Text,
    Numeric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAttribute {
    pub base: BaseEntity,
    pub name: String,
    pub display_name: String,
    pub attribute_type: AttributeType,
    pub values: Vec<AttributeValue>,
    pub status: Status,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeValue {
    pub id: Uuid,
    pub attribute_id: Uuid,
    pub value: String,
    pub display_value: String,
    pub color_code: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub base: BaseEntity,
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub attribute_values: Vec<VariantAttributeValue>,
    pub price_adjustment: i64,
    pub cost_adjustment: i64,
    pub barcode: Option<String>,
    pub weight_kg: Option<f64>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantAttributeValue {
    pub attribute_id: Uuid,
    pub attribute_name: String,
    pub value_id: Uuid,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariantMatrix {
    pub product_id: Uuid,
    pub attributes: Vec<AttributeInfo>,
    pub variants: Vec<VariantInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeInfo {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub values: Vec<ValueInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueInfo {
    pub id: Uuid,
    pub value: String,
    pub display_value: String,
    pub color_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantInfo {
    pub id: Option<Uuid>,
    pub sku: String,
    pub name: String,
    pub attribute_combination: Vec<AttributeCombination>,
    pub price_adjustment: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeCombination {
    pub attribute_id: Uuid,
    pub value_id: Uuid,
}
