use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait PricingRepository: Send + Sync {
    async fn create_price_book(&self, pool: &SqlitePool, pb: PriceBook) -> Result<PriceBook>;
    async fn get_price_book(&self, pool: &SqlitePool, id: Uuid) -> Result<PriceBook>;
    async fn list_price_books(&self, pool: &SqlitePool) -> Result<Vec<PriceBook>>;
    async fn create_price_book_entry(&self, pool: &SqlitePool, entry: PriceBookEntry) -> Result<PriceBookEntry>;
    async fn get_product_price(&self, pool: &SqlitePool, price_book_id: Uuid, product_id: Uuid) -> Result<Option<PriceBookEntry>>;
    async fn create_price_rule(&self, pool: &SqlitePool, rule: PriceRule) -> Result<PriceRule>;
    async fn get_price_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<PriceRule>;
    async fn list_price_rules(&self, pool: &SqlitePool) -> Result<Vec<PriceRule>>;
    async fn create_discount(&self, pool: &SqlitePool, discount: Discount) -> Result<Discount>;
    async fn get_discount(&self, pool: &SqlitePool, id: Uuid) -> Result<Discount>;
    async fn get_discount_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Option<Discount>>;
    async fn list_discounts(&self, pool: &SqlitePool) -> Result<Vec<Discount>>;
    async fn create_coupon(&self, pool: &SqlitePool, coupon: Coupon) -> Result<Coupon>;
    async fn get_coupon(&self, pool: &SqlitePool, code: &str) -> Result<Option<Coupon>>;
    async fn use_coupon(&self, pool: &SqlitePool, id: Uuid, order_id: Uuid) -> Result<()>;
    async fn create_promotion(&self, pool: &SqlitePool, promo: Promotion) -> Result<Promotion>;
    async fn get_promotion(&self, pool: &SqlitePool, id: Uuid) -> Result<Promotion>;
    async fn list_promotions(&self, pool: &SqlitePool) -> Result<Vec<Promotion>>;
    async fn create_price_tier(&self, pool: &SqlitePool, tier: PriceTier) -> Result<PriceTier>;
    async fn get_price_tiers(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<PriceTier>>;
    async fn create_customer_price_group(&self, pool: &SqlitePool, group: CustomerPriceGroup) -> Result<CustomerPriceGroup>;
    async fn add_customer_to_group(&self, pool: &SqlitePool, member: CustomerPriceGroupMember) -> Result<CustomerPriceGroupMember>;
}

pub struct SqlitePricingRepository;

#[async_trait]
impl PricingRepository for SqlitePricingRepository {
    async fn create_price_book(&self, pool: &SqlitePool, pb: PriceBook) -> Result<PriceBook> {
        sqlx::query!(
            r#"INSERT INTO price_books (id, name, code, description, currency, is_default, is_active,
               valid_from, valid_to, parent_id, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            pb.base.id.to_string(),
            pb.name,
            pb.code,
            pb.description,
            pb.currency,
            pb.is_default as i32,
            pb.is_active as i32,
            pb.valid_from.map(|d| d.to_rfc3339()),
            pb.valid_to.map(|d| d.to_rfc3339()),
            pb.parent_id.map(|id| id.to_string()),
            pb.base.created_at.to_rfc3339(),
            pb.base.updated_at.to_rfc3339(),
            pb.base.created_by.map(|id| id.to_string()),
            pb.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(pb)
    }

    async fn get_price_book(&self, pool: &SqlitePool, id: Uuid) -> Result<PriceBook> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, currency, is_default, is_active, valid_from, valid_to,
               parent_id, created_at, updated_at, created_by, updated_by FROM price_books WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(PriceBook {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            currency: row.currency,
            is_default: row.is_default == 1,
            is_active: row.is_active == 1,
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            parent_id: row.parent_id.and_then(|s| Uuid::parse_str(&s).ok()),
        })
    }

    async fn list_price_books(&self, pool: &SqlitePool) -> Result<Vec<PriceBook>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, description, currency, is_default, is_active, valid_from, valid_to,
               parent_id, created_at, updated_at, created_by, updated_by FROM price_books WHERE is_active = 1"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| PriceBook {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            currency: row.currency,
            is_default: row.is_default == 1,
            is_active: row.is_active == 1,
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            parent_id: row.parent_id.and_then(|s| Uuid::parse_str(&s).ok()),
        }).collect())
    }

    async fn create_price_book_entry(&self, pool: &SqlitePool, entry: PriceBookEntry) -> Result<PriceBookEntry> {
        sqlx::query!(
            r#"INSERT INTO price_book_entries (id, price_book_id, product_id, unit_price, currency,
               min_quantity, max_quantity, valid_from, valid_to, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            entry.base.id.to_string(),
            entry.price_book_id.to_string(),
            entry.product_id.to_string(),
            entry.unit_price,
            entry.currency,
            entry.min_quantity,
            entry.max_quantity,
            entry.valid_from.map(|d| d.to_rfc3339()),
            entry.valid_to.map(|d| d.to_rfc3339()),
            format!("{:?}", entry.status),
            entry.base.created_at.to_rfc3339(),
            entry.base.updated_at.to_rfc3339(),
            entry.base.created_by.map(|id| id.to_string()),
            entry.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(entry)
    }

    async fn get_product_price(&self, pool: &SqlitePool, price_book_id: Uuid, product_id: Uuid) -> Result<Option<PriceBookEntry>> {
        let row = sqlx::query!(
            r#"SELECT id, price_book_id, product_id, unit_price, currency, min_quantity, max_quantity,
               valid_from, valid_to, status, created_at, updated_at, created_by, updated_by
               FROM price_book_entries WHERE price_book_id = ? AND product_id = ?"#,
            price_book_id.to_string(), product_id.to_string()
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|row| PriceBookEntry {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            price_book_id: Uuid::parse_str(&row.price_book_id).unwrap(),
            product_id: Uuid::parse_str(&row.product_id).unwrap(),
            unit_price: row.unit_price,
            currency: row.currency,
            min_quantity: row.min_quantity,
            max_quantity: row.max_quantity,
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            status: erp_core::Status::Active,
        }))
    }

    async fn create_price_rule(&self, pool: &SqlitePool, rule: PriceRule) -> Result<PriceRule> {
        sqlx::query!(
            r#"INSERT INTO price_rules (id, name, code, description, rule_type, scope, priority, value,
               currency, conditions, valid_from, valid_to, is_active, is_stackable, max_applications,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.base.id.to_string(),
            rule.name,
            rule.code,
            rule.description,
            format!("{:?}", rule.rule_type),
            format!("{:?}", rule.scope),
            rule.priority,
            rule.value,
            rule.currency,
            rule.conditions,
            rule.valid_from.map(|d| d.to_rfc3339()),
            rule.valid_to.map(|d| d.to_rfc3339()),
            rule.is_active as i32,
            rule.is_stackable as i32,
            rule.max_applications,
            rule.base.created_at.to_rfc3339(),
            rule.base.updated_at.to_rfc3339(),
            rule.base.created_by.map(|id| id.to_string()),
            rule.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(rule)
    }

    async fn get_price_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<PriceRule> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, rule_type, scope, priority, value, currency,
               conditions, valid_from, valid_to, is_active, is_stackable, max_applications,
               created_at, updated_at, created_by, updated_by FROM price_rules WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(PriceRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            rule_type: PriceRuleType::Discount,
            scope: PriceRuleScope::Global,
            priority: row.priority,
            value: row.value,
            currency: row.currency,
            conditions: row.conditions,
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            is_active: row.is_active == 1,
            is_stackable: row.is_stackable == 1,
            max_applications: row.max_applications,
        })
    }

    async fn list_price_rules(&self, pool: &SqlitePool) -> Result<Vec<PriceRule>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, description, rule_type, scope, priority, value, currency,
               conditions, valid_from, valid_to, is_active, is_stackable, max_applications,
               created_at, updated_at, created_by, updated_by FROM price_rules WHERE is_active = 1"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| PriceRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            rule_type: PriceRuleType::Discount,
            scope: PriceRuleScope::Global,
            priority: row.priority,
            value: row.value,
            currency: row.currency,
            conditions: row.conditions,
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            is_active: row.is_active == 1,
            is_stackable: row.is_stackable == 1,
            max_applications: row.max_applications,
        }).collect())
    }

    async fn create_discount(&self, pool: &SqlitePool, discount: Discount) -> Result<Discount> {
        sqlx::query!(
            r#"INSERT INTO discounts (id, name, code, description, discount_type, value, max_discount,
               min_order_value, applicable_to, customer_groups, products, categories, valid_from, valid_to,
               usage_limit, usage_per_customer, current_usage, is_active, requires_code, auto_apply,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            discount.base.id.to_string(),
            discount.name,
            discount.code,
            discount.description,
            format!("{:?}", discount.discount_type),
            discount.value,
            discount.max_discount,
            discount.min_order_value,
            discount.applicable_to,
            discount.customer_groups,
            discount.products,
            discount.categories,
            discount.valid_from.map(|d| d.to_rfc3339()),
            discount.valid_to.map(|d| d.to_rfc3339()),
            discount.usage_limit,
            discount.usage_per_customer,
            discount.current_usage,
            discount.is_active as i32,
            discount.requires_code as i32,
            discount.auto_apply as i32,
            discount.base.created_at.to_rfc3339(),
            discount.base.updated_at.to_rfc3339(),
            discount.base.created_by.map(|id| id.to_string()),
            discount.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(discount)
    }

    async fn get_discount(&self, pool: &SqlitePool, id: Uuid) -> Result<Discount> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, discount_type, value, max_discount, min_order_value,
               applicable_to, customer_groups, products, categories, valid_from, valid_to, usage_limit,
               usage_per_customer, current_usage, is_active, requires_code, auto_apply,
               created_at, updated_at, created_by, updated_by FROM discounts WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(Discount {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            discount_type: DiscountType::Percentage,
            value: row.value,
            max_discount: row.max_discount,
            min_order_value: row.min_order_value,
            applicable_to: row.applicable_to,
            customer_groups: row.customer_groups,
            products: row.products,
            categories: row.categories,
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            usage_limit: row.usage_limit,
            usage_per_customer: row.usage_per_customer,
            current_usage: row.current_usage,
            is_active: row.is_active == 1,
            requires_code: row.requires_code == 1,
            auto_apply: row.auto_apply == 1,
        })
    }

    async fn get_discount_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Option<Discount>> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, discount_type, value, max_discount, min_order_value,
               applicable_to, customer_groups, products, categories, valid_from, valid_to, usage_limit,
               usage_per_customer, current_usage, is_active, requires_code, auto_apply,
               created_at, updated_at, created_by, updated_by FROM discounts WHERE code = ? AND is_active = 1"#,
            code
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|row| Discount {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            discount_type: DiscountType::Percentage,
            value: row.value,
            max_discount: row.max_discount,
            min_order_value: row.min_order_value,
            applicable_to: row.applicable_to,
            customer_groups: row.customer_groups,
            products: row.products,
            categories: row.categories,
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            usage_limit: row.usage_limit,
            usage_per_customer: row.usage_per_customer,
            current_usage: row.current_usage,
            is_active: row.is_active == 1,
            requires_code: row.requires_code == 1,
            auto_apply: row.auto_apply == 1,
        }))
    }

    async fn list_discounts(&self, pool: &SqlitePool) -> Result<Vec<Discount>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, description, discount_type, value, max_discount, min_order_value,
               applicable_to, customer_groups, products, categories, valid_from, valid_to, usage_limit,
               usage_per_customer, current_usage, is_active, requires_code, auto_apply,
               created_at, updated_at, created_by, updated_by FROM discounts WHERE is_active = 1"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| Discount {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            discount_type: DiscountType::Percentage,
            value: row.value,
            max_discount: row.max_discount,
            min_order_value: row.min_order_value,
            applicable_to: row.applicable_to,
            customer_groups: row.customer_groups,
            products: row.products,
            categories: row.categories,
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            usage_limit: row.usage_limit,
            usage_per_customer: row.usage_per_customer,
            current_usage: row.current_usage,
            is_active: row.is_active == 1,
            requires_code: row.requires_code == 1,
            auto_apply: row.auto_apply == 1,
        }).collect())
    }

    async fn create_coupon(&self, pool: &SqlitePool, coupon: Coupon) -> Result<Coupon> {
        sqlx::query!(
            r#"INSERT INTO coupons (id, code, discount_id, promotion_id, customer_id, is_used, used_at,
               order_id, valid_from, valid_to, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            coupon.base.id.to_string(),
            coupon.code,
            coupon.discount_id.to_string(),
            coupon.promotion_id.map(|id| id.to_string()),
            coupon.customer_id.map(|id| id.to_string()),
            coupon.is_used as i32,
            coupon.used_at.map(|d| d.to_rfc3339()),
            coupon.order_id.map(|id| id.to_string()),
            coupon.valid_from.map(|d| d.to_rfc3339()),
            coupon.valid_to.map(|d| d.to_rfc3339()),
            format!("{:?}", coupon.status),
            coupon.base.created_at.to_rfc3339(),
            coupon.base.updated_at.to_rfc3339(),
            coupon.base.created_by.map(|id| id.to_string()),
            coupon.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(coupon)
    }

    async fn get_coupon(&self, pool: &SqlitePool, code: &str) -> Result<Option<Coupon>> {
        let row = sqlx::query!(
            r#"SELECT id, code, discount_id, promotion_id, customer_id, is_used, used_at, order_id,
               valid_from, valid_to, status, created_at, updated_at, created_by, updated_by
               FROM coupons WHERE code = ?"#,
            code
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|row| Coupon {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            code: row.code,
            discount_id: Uuid::parse_str(&row.discount_id).unwrap(),
            promotion_id: row.promotion_id.and_then(|s| Uuid::parse_str(&s).ok()),
            customer_id: row.customer_id.and_then(|s| Uuid::parse_str(&s).ok()),
            is_used: row.is_used == 1,
            used_at: row.used_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            order_id: row.order_id.and_then(|s| Uuid::parse_str(&s).ok()),
            valid_from: row.valid_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.valid_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            status: erp_core::Status::Active,
        }))
    }

    async fn use_coupon(&self, pool: &SqlitePool, id: Uuid, order_id: Uuid) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query!(
            r#"UPDATE coupons SET is_used = 1, used_at = ?, order_id = ? WHERE id = ?"#,
            now, order_id.to_string(), id.to_string()
        ).execute(pool).await?;
        Ok(())
    }

    async fn create_promotion(&self, pool: &SqlitePool, promo: Promotion) -> Result<Promotion> {
        sqlx::query!(
            r#"INSERT INTO promotions (id, name, code, description, promotion_type, status, start_date,
               end_date, rules, rewards, target_segments, channels, budget, spent, usage_limit, current_usage,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            promo.base.id.to_string(),
            promo.name,
            promo.code,
            promo.description,
            format!("{:?}", promo.promotion_type),
            format!("{:?}", promo.status),
            promo.start_date.to_rfc3339(),
            promo.end_date.to_rfc3339(),
            promo.rules,
            promo.rewards,
            promo.target_segments,
            promo.channels,
            promo.budget,
            promo.spent,
            promo.usage_limit,
            promo.current_usage,
            promo.base.created_at.to_rfc3339(),
            promo.base.updated_at.to_rfc3339(),
            promo.base.created_by.map(|id| id.to_string()),
            promo.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(promo)
    }

    async fn get_promotion(&self, pool: &SqlitePool, id: Uuid) -> Result<Promotion> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, promotion_type, status, start_date, end_date, rules,
               rewards, target_segments, channels, budget, spent, usage_limit, current_usage,
               created_at, updated_at, created_by, updated_by FROM promotions WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(Promotion {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            promotion_type: PromotionType::ProductDiscount,
            status: PromotionStatus::Draft,
            start_date: chrono::DateTime::parse_from_rfc3339(&row.start_date).unwrap().with_timezone(&chrono::Utc),
            end_date: chrono::DateTime::parse_from_rfc3339(&row.end_date).unwrap().with_timezone(&chrono::Utc),
            rules: row.rules,
            rewards: row.rewards,
            target_segments: row.target_segments,
            channels: row.channels,
            budget: row.budget,
            spent: row.spent,
            usage_limit: row.usage_limit,
            current_usage: row.current_usage,
        })
    }

    async fn list_promotions(&self, pool: &SqlitePool) -> Result<Vec<Promotion>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, description, promotion_type, status, start_date, end_date, rules,
               rewards, target_segments, channels, budget, spent, usage_limit, current_usage,
               created_at, updated_at, created_by, updated_by FROM promotions"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| Promotion {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            code: row.code,
            description: row.description,
            promotion_type: PromotionType::ProductDiscount,
            status: PromotionStatus::Draft,
            start_date: chrono::DateTime::parse_from_rfc3339(&row.start_date).unwrap().with_timezone(&chrono::Utc),
            end_date: chrono::DateTime::parse_from_rfc3339(&row.end_date).unwrap().with_timezone(&chrono::Utc),
            rules: row.rules,
            rewards: row.rewards,
            target_segments: row.target_segments,
            channels: row.channels,
            budget: row.budget,
            spent: row.spent,
            usage_limit: row.usage_limit,
            current_usage: row.current_usage,
        }).collect())
    }

    async fn create_price_tier(&self, pool: &SqlitePool, tier: PriceTier) -> Result<PriceTier> {
        sqlx::query!(
            r#"INSERT INTO price_tiers (id, price_book_entry_id, product_id, min_quantity, max_quantity,
               unit_price, discount_percent, currency, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            tier.base.id.to_string(),
            tier.price_book_entry_id.map(|id| id.to_string()),
            tier.product_id.map(|id| id.to_string()),
            tier.min_quantity,
            tier.max_quantity,
            tier.unit_price,
            tier.discount_percent,
            tier.currency,
            tier.base.created_at.to_rfc3339(),
            tier.base.updated_at.to_rfc3339(),
            tier.base.created_by.map(|id| id.to_string()),
            tier.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(tier)
    }

    async fn get_price_tiers(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<PriceTier>> {
        let rows = sqlx::query!(
            r#"SELECT id, price_book_entry_id, product_id, min_quantity, max_quantity, unit_price,
               discount_percent, currency, created_at, updated_at, created_by, updated_by
               FROM price_tiers WHERE product_id = ? ORDER BY min_quantity"#,
            product_id.to_string()
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| PriceTier {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            price_book_entry_id: row.price_book_entry_id.and_then(|s| Uuid::parse_str(&s).ok()),
            product_id: row.product_id.and_then(|s| Uuid::parse_str(&s).ok()),
            min_quantity: row.min_quantity,
            max_quantity: row.max_quantity,
            unit_price: row.unit_price,
            discount_percent: row.discount_percent,
            currency: row.currency,
        }).collect())
    }

    async fn create_customer_price_group(&self, pool: &SqlitePool, group: CustomerPriceGroup) -> Result<CustomerPriceGroup> {
        sqlx::query!(
            r#"INSERT INTO customer_price_groups (id, name, code, description, price_book_id, discount_id,
               status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            group.base.id.to_string(),
            group.name,
            group.code,
            group.description,
            group.price_book_id.map(|id| id.to_string()),
            group.discount_id.map(|id| id.to_string()),
            format!("{:?}", group.status),
            group.base.created_at.to_rfc3339(),
            group.base.updated_at.to_rfc3339(),
            group.base.created_by.map(|id| id.to_string()),
            group.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(group)
    }

    async fn add_customer_to_group(&self, pool: &SqlitePool, member: CustomerPriceGroupMember) -> Result<CustomerPriceGroupMember> {
        sqlx::query!(
            r#"INSERT INTO customer_price_group_members (id, group_id, customer_id, joined_at,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            member.base.id.to_string(),
            member.group_id.to_string(),
            member.customer_id.to_string(),
            member.joined_at.to_rfc3339(),
            member.base.created_at.to_rfc3339(),
            member.base.updated_at.to_rfc3339(),
            member.base.created_by.map(|id| id.to_string()),
            member.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(member)
    }
}
