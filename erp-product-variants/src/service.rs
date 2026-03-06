use erp_core::{BaseEntity, Error, Paginated, Pagination, Result, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

    use crate::models::{ProductAttribute, ProductVariant};
use crate::repository::{ProductAttributeRepository, ProductVariantRepository, SqliteProductAttributeRepository, SqliteProductVariantRepository};
pub struct ProductAttributeService {
    repo: SqliteProductAttributeRepository,
}
pub struct ProductVariantService {
    repo: SqliteProductVariantRepository,
}
impl Default for ProductAttributeService {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for ProductVariantService {
    fn default() -> Self {
        Self::new()
    }
}
impl ProductAttributeService {
    pub fn new() -> Self {
        Self { repo: SqliteProductAttributeRepository }
    }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductAttribute> {
        self.repo.find_by_id(pool, id).await
    }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ProductAttribute>> {
        self.repo.find_all(pool, pagination).await
    }
    pub async fn create(&self, pool: &SqlitePool, mut attribute: ProductAttribute) -> Result<ProductAttribute> {
        if attribute.name.is_empty() {
            return Err(Error::validation("Attribute name is required"));
        }
        if attribute.display_name.is_empty() {
            return Err(Error::validation("Display name is required"));
        }
        if attribute.values.is_empty() {
            return Err(Error::validation("At least one attribute value is required"));
        }
        attribute.base = BaseEntity::new();
        attribute.status = Status::Active;
        for value in &mut attribute.values {
            value.id = Uuid::new_v4();
            value.attribute_id = attribute.base.id;
        }
        self.repo.create(pool, attribute).await
    }
    pub async fn update(&self, pool: &SqlitePool, mut attribute: ProductAttribute) -> Result<ProductAttribute> {
        if attribute.name.is_empty() {
            return Err(Error::validation("Attribute name is required"));
        }
        if attribute.display_name.is_empty() {
            return Err(Error::validation("Display name is required"));
        }
        if attribute.values.is_empty() {
            return Err(Error::validation("At least one attribute value is required"));
        }
        let existing = self.repo.find_by_id(pool, attribute.base.id).await?;
        attribute.base.created_at = existing.base.created_at;
        for value in &mut attribute.values {
            if value.id.is_nil() {
                value.id = Uuid::new_v4();
            }
        }
        self.repo.update(pool, attribute).await
    }
    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }
}
impl ProductVariantService {
    pub fn new() -> Self {
        Self { repo: SqliteProductVariantRepository }
    }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductVariant> {
        self.repo.find_by_id(pool, id).await
    }
    pub async fn get_by_sku(&self, pool: &SqlitePool, sku: &str) -> Result<ProductVariant> {
        self.repo.find_by_sku(pool, sku).await
    }
    pub async fn list_by_product(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<ProductVariant>> {
        self.repo.find_by_product(pool, product_id).await
    }
    pub async fn create(&self, pool: &SqlitePool, mut variant: ProductVariant) -> Result<ProductVariant> {
        if variant.sku.is_empty() {
            return Err(Error::validation("Variant SKU is required"));
        }
        if variant.name.is_empty() {
            return Err(Error::validation("Variant name is required"));
        }
        if variant.attribute_values.is_empty() {
            return Err(Error::validation("At least one attribute value is required"));
        }
        variant.base = BaseEntity::new();
        variant.status = Status::Active;
        self.repo.create(pool, variant).await
    }
    pub async fn update(&self, pool: &SqlitePool, mut variant: ProductVariant) -> Result<ProductVariant> {
        if variant.sku.is_empty() {
            return Err(Error::validation("Variant SKU is required"));
        }
        if variant.name.is_empty() {
            return Err(Error::validation("Variant name is required"));
        }
        if variant.attribute_values.is_empty() {
            return Err(Error::validation("At least one attribute value is required"));
        }
        let existing = self.repo.find_by_id(pool, variant.base.id).await?;
        variant.base.created_at = existing.base.created_at;
        variant.product_id = existing.product_id;
        self.repo.update(pool, variant).await
    }
    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AttributeType, AttributeValue};
    #[test]
    fn test_validation_empty_attribute_name() {
        let attr = ProductAttribute {
            base: BaseEntity::new(),
            name: "".to_string(),
            display_name: "Size".to_string(),
            attribute_type: AttributeType::Select,
            values: vec![AttributeValue {
                id: Uuid::new_v4(),
                attribute_id: Uuid::nil(),
                value: "S".to_string(),
                display_value: "Small".to_string(),
                color_code: None,
                sort_order: 0,
            }],
            status: Status::Active,
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(async { sqlx::SqlitePool::connect(":memory:").await.unwrap() });
        let svc = ProductAttributeService::new();
        let result = rt.block_on(svc.create(&pool, attr));
        assert!(result.is_err());
    }
}
