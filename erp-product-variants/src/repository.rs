use async_trait::async_trait;
use chrono::Utc;
use erp_core::{BaseEntity, Error, Paginated, Pagination, Result, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{AttributeType, AttributeValue, ProductAttribute, ProductVariant, VariantAttributeValue};

#[async_trait]
pub trait ProductAttributeRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductAttribute>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ProductAttribute>>;
    async fn create(&self, pool: &SqlitePool, attribute: ProductAttribute) -> Result<ProductAttribute>;
    async fn update(&self, pool: &SqlitePool, attribute: ProductAttribute) -> Result<ProductAttribute>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait ProductVariantRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductVariant>;
    async fn find_by_product(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<ProductVariant>>;
    async fn find_by_sku(&self, pool: &SqlitePool, sku: &str) -> Result<ProductVariant>;
    async fn create(&self, pool: &SqlitePool, variant: ProductVariant) -> Result<ProductVariant>;
    async fn update(&self, pool: &SqlitePool, variant: ProductVariant) -> Result<ProductVariant>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteProductAttributeRepository;
pub struct SqliteProductVariantRepository;
#[derive(sqlx::FromRow)]
struct AttributeRow {
    id: String,
    name: String,
    display_name: String,
    attribute_type: String,
    status: String,
    created_at: String,
    updated_at: String,
}
#[derive(sqlx::FromRow)]
struct AttributeValueRow {
    id: String,
    attribute_id: String,
    value: String,
    display_value: String,
    color_code: Option<String>,
    sort_order: i32,
}
impl AttributeRow {
    fn into_attribute(self, values: Vec<AttributeValue>) -> ProductAttribute {
        ProductAttribute {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
 .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
 .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            name: self.name,
            display_name: self.display_name,
            attribute_type: match self.attribute_type.as_str() {
                "MultiSelect" => AttributeType::MultiSelect,
                "Color" => AttributeType::Color,
                "Text" => AttributeType::Text,
                "Numeric" => AttributeType::Numeric,
                _ => AttributeType::Select,
            },
            values,
            status: match self.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
        }
    }
}
#[async_trait]
impl ProductAttributeRepository for SqliteProductAttributeRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductAttribute> {
        let row = sqlx::query_as::<_, AttributeRow>(
            "SELECT id, name, display_name, attribute_type, status, created_at, updated_at
 FROM product_attributes WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("ProductAttribute", &id.to_string()))?;
        let values = sqlx::query_as::<_, AttributeValueRow>(
            "SELECT id, attribute_id, value, display_value, color_code, sort_order FROM product_attribute_values WHERE attribute_id = ? ORDER BY sort_order"
        )
        .bind(id.to_string())
        .fetch_all(pool)
        .await?;
        Ok(row.into_attribute(values.into_iter().map(|v| AttributeValue {
            id: Uuid::parse_str(&v.id).unwrap_or_default(),
            attribute_id: Uuid::parse_str(&v.attribute_id).unwrap_or_default(),
            value: v.value,
            display_value: v.display_value,
            color_code: v.color_code,
            sort_order: v.sort_order,
        }).collect()))
    }
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ProductAttribute>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM product_attributes WHERE status != 'Deleted'")
            .fetch_one(pool)
            .await?;
        let rows = sqlx::query_as::<_, AttributeRow>(
            "SELECT id, name, display_name, attribute_type, status, created_at, updated_at FROM product_attributes WHERE status != 'Deleted' ORDER BY name LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;
        let mut attributes = Vec::new();
        for row in rows {
            let values = sqlx::query_as::<_, AttributeValueRow>(
                "SELECT id, attribute_id, value, display_value, color_code, sort_order FROM product_attribute_values WHERE attribute_id = ? ORDER BY sort_order"
            )
            .bind(&row.id)
            .fetch_all(pool)
            .await?;
            attributes.push(row.into_attribute(values.into_iter().map(|v| AttributeValue {
                id: Uuid::parse_str(&v.id).unwrap_or_default(),
                attribute_id: Uuid::parse_str(&v.attribute_id).unwrap_or_default(),
                value: v.value,
                display_value: v.display_value,
                color_code: v.color_code,
                sort_order: v.sort_order,
            }).collect()));
        }
        Ok(Paginated::new(attributes, count.0 as u64, pagination))
    }
    async fn create(&self, pool: &SqlitePool, attribute: ProductAttribute) -> Result<ProductAttribute> {
        let now = Utc::now();
        let attr_type = match attribute.attribute_type {
            AttributeType::Select => "Select",
            AttributeType::MultiSelect => "MultiSelect",
            AttributeType::Color => "Color",
            AttributeType::Text => "Text",
            AttributeType::Numeric => "Numeric",
        };
        sqlx::query(
            "INSERT INTO product_attributes (id, name, display_name, attribute_type, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(attribute.base.id.to_string())
        .bind(&attribute.name)
        .bind(&attribute.display_name)
        .bind(attr_type)
        .bind(format!("{:?}", attribute.status))
        .bind(attribute.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        for value in &attribute.values {
            sqlx::query(
                "INSERT INTO product_attribute_values (id, attribute_id, value, display_value, color_code, sort_order) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(value.id.to_string())
            .bind(value.attribute_id.to_string())
            .bind(&value.value)
            .bind(&value.display_value)
            .bind(&value.color_code)
            .bind(value.sort_order)
            .execute(pool)
            .await?;
        }
        Ok(attribute)
    }
    async fn update(&self, pool: &SqlitePool, attribute: ProductAttribute) -> Result<ProductAttribute> {
        let now = Utc::now();
        let attr_type = match attribute.attribute_type {
            AttributeType::Select => "Select",
            AttributeType::MultiSelect => "MultiSelect",
            AttributeType::Color => "Color",
            AttributeType::Text => "Text",
            AttributeType::Numeric => "Numeric",
        };
        let rows = sqlx::query(
            "UPDATE product_attributes SET name=?, display_name=?, attribute_type=?, status=?, updated_at=? WHERE id=?"
        )
        .bind(&attribute.name)
        .bind(&attribute.display_name)
        .bind(attr_type)
        .bind(format!("{:?}", attribute.status))
        .bind(now.to_rfc3339())
        .bind(attribute.base.id.to_string())
        .execute(pool)
        .await?;
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ProductAttribute", &attribute.base.id.to_string()));
        }
        sqlx::query("DELETE FROM product_attribute_values WHERE attribute_id = ?")
            .bind(attribute.base.id.to_string())
            .execute(pool)
            .await?;
        for value in &attribute.values {
            sqlx::query(
                "INSERT INTO product_attribute_values (id, attribute_id, value, display_value, color_code, sort_order) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(value.id.to_string())
            .bind(value.attribute_id.to_string())
            .bind(&value.value)
            .bind(&value.display_value)
            .bind(&value.color_code)
            .bind(value.sort_order)
            .execute(pool)
            .await?;
        }
        Ok(attribute)
    }
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("UPDATE product_attributes SET status = 'Deleted', updated_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ProductAttribute", &id.to_string()));
        }
        Ok(())
    }
}
#[derive(sqlx::FromRow)]
struct VariantRow {
    id: String,
    product_id: String,
    sku: String,
    name: String,
    price_adjustment: i64,
    cost_adjustment: i64,
    barcode: Option<String>,
    weight_kg: Option<f64>,
    status: String,
    created_at: String,
    updated_at: String,
}
#[derive(sqlx::FromRow)]
struct VariantAttributeValueRow {
    attribute_id: String,
    attribute_name: String,
    value_id: String,
    value: String,
}
impl VariantRow {
    fn into_variant(self, attribute_values: Vec<VariantAttributeValue>) -> ProductVariant {
        ProductVariant {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at) .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at) .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            product_id: Uuid::parse_str(&self.product_id).unwrap_or_default(),
            sku: self.sku,
            name: self.name,
            attribute_values,
            price_adjustment: self.price_adjustment,
            cost_adjustment: self.cost_adjustment,
            barcode: self.barcode,
            weight_kg: self.weight_kg,
            status: match self.status.as_str() {
                "Inactive" => Status::Inactive,
                "Draft" => Status::Draft,
                _ => Status::Active,
            },
        }
    }
}
#[async_trait]
impl ProductVariantRepository for SqliteProductVariantRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductVariant> {
        let row = sqlx::query_as::<_, VariantRow>(
            "SELECT id, product_id, sku, name, price_adjustment, cost_adjustment, barcode, weight_kg, status, created_at, updated_at FROM product_variants WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("ProductVariant", &id.to_string()))?;
        let attr_values = sqlx::query_as::<_, VariantAttributeValueRow>(
            "SELECT attribute_id, attribute_name, value_id, value FROM product_variant_attribute_values WHERE variant_id = ?"
        )
        .bind(id.to_string())
        .fetch_all(pool)
        .await?;
        Ok(row.into_variant(attr_values.into_iter().map(|v| VariantAttributeValue {
            attribute_id: Uuid::parse_str(&v.attribute_id).unwrap_or_default(),
            attribute_name: v.attribute_name,
            value_id: Uuid::parse_str(&v.value_id).unwrap_or_default(),
            value: v.value,
        }).collect()))
    }
    async fn find_by_product(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<ProductVariant>> {
        let rows = sqlx::query_as::<_, VariantRow>(
            "SELECT id, product_id, sku, name, price_adjustment, cost_adjustment, barcode, weight_kg, status, created_at, updated_at FROM product_variants WHERE product_id = ? AND status != 'Deleted' ORDER BY sku"
        )
        .bind(product_id.to_string())
        .fetch_all(pool)
        .await?;
        let mut variants = Vec::new();
        for row in rows {
            let attr_values = sqlx::query_as::<_, VariantAttributeValueRow>(
                "SELECT attribute_id, attribute_name, value_id, value FROM product_variant_attribute_values WHERE variant_id = ?"
            )
            .bind(&row.id)
            .fetch_all(pool)
            .await?;
            variants.push(row.into_variant(attr_values.into_iter().map(|v| VariantAttributeValue {
                attribute_id: Uuid::parse_str(&v.attribute_id).unwrap_or_default(),
                attribute_name: v.attribute_name,
                value_id: Uuid::parse_str(&v.value_id).unwrap_or_default(),
                value: v.value,
            }).collect()));
        }
        Ok(variants)
    }
    async fn find_by_sku(&self, pool: &SqlitePool, sku: &str) -> Result<ProductVariant> {
        let row = sqlx::query_as::<_, VariantRow>(
            "SELECT id, product_id, sku, name, price_adjustment, cost_adjustment, barcode, weight_kg, status, created_at, updated_at FROM product_variants WHERE sku = ?"
        )
        .bind(sku)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("ProductVariant", sku))?;
        let attr_values = sqlx::query_as::<_, VariantAttributeValueRow>(
            "SELECT attribute_id, attribute_name, value_id, value FROM product_variant_attribute_values WHERE variant_id = ?"
        )
        .bind(&row.id)
        .fetch_all(pool)
        .await?;
        Ok(row.into_variant(attr_values.into_iter().map(|v| VariantAttributeValue {
            attribute_id: Uuid::parse_str(&v.attribute_id).unwrap_or_default(),
            attribute_name: v.attribute_name,
            value_id: Uuid::parse_str(&v.value_id).unwrap_or_default(),
            value: v.value,
        }).collect()))
    }
    async fn create(&self, pool: &SqlitePool, variant: ProductVariant) -> Result<ProductVariant> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO product_variants (id, product_id, sku, name, price_adjustment, cost_adjustment, barcode, weight_kg, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(variant.base.id.to_string())
        .bind(variant.product_id.to_string())
        .bind(&variant.sku)
        .bind(&variant.name)
        .bind(variant.price_adjustment)
        .bind(variant.cost_adjustment)
        .bind(&variant.barcode)
        .bind(variant.weight_kg)
        .bind(format!("{:?}", variant.status))
        .bind(variant.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        for attr_value in &variant.attribute_values {
            sqlx::query(
                "INSERT INTO product_variant_attribute_values (variant_id, attribute_id, attribute_name, value_id, value) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(variant.base.id.to_string())
            .bind(attr_value.attribute_id.to_string())
            .bind(&attr_value.attribute_name)
            .bind(attr_value.value_id.to_string())
            .bind(&attr_value.value)
            .execute(pool)
            .await?;
        }
        Ok(variant)
    }
    async fn update(&self, pool: &SqlitePool, variant: ProductVariant) -> Result<ProductVariant> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE product_variants SET sku=?, name=?, price_adjustment=?, cost_adjustment=?, barcode=?, weight_kg=?, status=?, updated_at=? WHERE id=?"
        )
        .bind(&variant.sku)
        .bind(&variant.name)
        .bind(variant.price_adjustment)
        .bind(variant.cost_adjustment)
        .bind(&variant.barcode)
        .bind(variant.weight_kg)
        .bind(format!("{:?}", variant.status))
        .bind(now.to_rfc3339())
        .bind(variant.base.id.to_string())
        .execute(pool)
        .await?;
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ProductVariant", &variant.base.id.to_string()));
        }
        sqlx::query("DELETE FROM product_variant_attribute_values WHERE variant_id = ?")
            .bind(variant.base.id.to_string())
            .execute(pool)
            .await?;
        for attr_value in &variant.attribute_values {
            sqlx::query(
                "INSERT INTO product_variant_attribute_values (variant_id, attribute_id, attribute_name, value_id, value) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(variant.base.id.to_string())
            .bind(attr_value.attribute_id.to_string())
            .bind(&attr_value.attribute_name)
            .bind(attr_value.value_id.to_string())
            .bind(&attr_value.value)
            .execute(pool)
            .await?;
        }
        Ok(variant)
    }
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("UPDATE product_variants SET status = 'Deleted', updated_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ProductVariant", &id.to_string()));
        }
        Ok(())
    }
}
