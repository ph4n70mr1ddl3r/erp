use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
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
        sqlx::query(
            r#"INSERT INTO price_books (id, name, code, description, currency, is_default, is_active,
               valid_from, valid_to, parent_id, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(pb.base.id.to_string())
        .bind(pb.name.clone())
        .bind(pb.code.clone())
        .bind(pb.description.clone())
        .bind(pb.currency.clone())
        .bind(pb.is_default as i32)
        .bind(pb.is_active as i32)
        .bind(pb.valid_from.map(|d| d.to_rfc3339()))
        .bind(pb.valid_to.map(|d| d.to_rfc3339()))
        .bind(pb.parent_id.map(|id| id.to_string()))
        .bind(pb.base.created_at.to_rfc3339())
        .bind(pb.base.updated_at.to_rfc3339())
        .bind(pb.base.created_by.map(|id| id.to_string()))
        .bind(pb.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(pb)
    }

    async fn get_price_book(&self, pool: &SqlitePool, id: Uuid) -> Result<PriceBook> {
        let row = sqlx::query(
            r#"SELECT id, name, code, description, currency, is_default, is_active, valid_from, valid_to,
               parent_id, created_at, updated_at, created_by, updated_by FROM price_books WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(PriceBook {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            currency: row.get::<&str, _>("currency").to_string(),
            is_default: row.get::<i32, _>("is_default") == 1,
            is_active: row.get::<i32, _>("is_active") == 1,
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            parent_id: row.get::<Option<&str>, _>("parent_id").and_then(|s| Uuid::parse_str(s).ok()),
        })
    }

    async fn list_price_books(&self, pool: &SqlitePool) -> Result<Vec<PriceBook>> {
        let rows = sqlx::query(
            r#"SELECT id, name, code, description, currency, is_default, is_active, valid_from, valid_to,
               parent_id, created_at, updated_at, created_by, updated_by FROM price_books WHERE is_active = 1"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.iter().map(|row| PriceBook {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            currency: row.get::<&str, _>("currency").to_string(),
            is_default: row.get::<i32, _>("is_default") == 1,
            is_active: row.get::<i32, _>("is_active") == 1,
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            parent_id: row.get::<Option<&str>, _>("parent_id").and_then(|s| Uuid::parse_str(s).ok()),
        }).collect())
    }

    async fn create_price_book_entry(&self, pool: &SqlitePool, entry: PriceBookEntry) -> Result<PriceBookEntry> {
        sqlx::query(
            r#"INSERT INTO price_book_entries (id, price_book_id, product_id, unit_price, currency,
               min_quantity, max_quantity, valid_from, valid_to, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(entry.base.id.to_string())
        .bind(entry.price_book_id.to_string())
        .bind(entry.product_id.to_string())
        .bind(entry.unit_price)
        .bind(entry.currency.clone())
        .bind(entry.min_quantity)
        .bind(entry.max_quantity)
        .bind(entry.valid_from.map(|d| d.to_rfc3339()))
        .bind(entry.valid_to.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", entry.status))
        .bind(entry.base.created_at.to_rfc3339())
        .bind(entry.base.updated_at.to_rfc3339())
        .bind(entry.base.created_by.map(|id| id.to_string()))
        .bind(entry.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(entry)
    }

    async fn get_product_price(&self, pool: &SqlitePool, price_book_id: Uuid, product_id: Uuid) -> Result<Option<PriceBookEntry>> {
        let row = sqlx::query(
            r#"SELECT id, price_book_id, product_id, unit_price, currency, min_quantity, max_quantity,
               valid_from, valid_to, status, created_at, updated_at, created_by, updated_by
               FROM price_book_entries WHERE price_book_id = ? AND product_id = ?"#,
        )
        .bind(price_book_id.to_string())
        .bind(product_id.to_string())
        .fetch_optional(pool).await?;
        
        Ok(row.map(|row| PriceBookEntry {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            price_book_id: Uuid::parse_str(row.get::<&str, _>("price_book_id")).unwrap(),
            product_id: Uuid::parse_str(row.get::<&str, _>("product_id")).unwrap(),
            unit_price: row.get::<i64, _>("unit_price"),
            currency: row.get::<&str, _>("currency").to_string(),
            min_quantity: row.get::<i32, _>("min_quantity"),
            max_quantity: row.get::<Option<i32>, _>("max_quantity"),
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            status: erp_core::Status::Active,
        }))
    }

    async fn create_price_rule(&self, pool: &SqlitePool, rule: PriceRule) -> Result<PriceRule> {
        sqlx::query(
            r#"INSERT INTO price_rules (id, name, code, description, rule_type, scope, priority, value,
               currency, conditions, valid_from, valid_to, is_active, is_stackable, max_applications,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(rule.base.id.to_string())
        .bind(rule.name.clone())
        .bind(rule.code.clone())
        .bind(rule.description.clone())
        .bind(format!("{:?}", rule.rule_type))
        .bind(format!("{:?}", rule.scope))
        .bind(rule.priority)
        .bind(rule.value)
        .bind(rule.currency.clone())
        .bind(rule.conditions.clone())
        .bind(rule.valid_from.map(|d| d.to_rfc3339()))
        .bind(rule.valid_to.map(|d| d.to_rfc3339()))
        .bind(rule.is_active as i32)
        .bind(rule.is_stackable as i32)
        .bind(rule.max_applications)
        .bind(rule.base.created_at.to_rfc3339())
        .bind(rule.base.updated_at.to_rfc3339())
        .bind(rule.base.created_by.map(|id| id.to_string()))
        .bind(rule.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(rule)
    }

    async fn get_price_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<PriceRule> {
        let row = sqlx::query(
            r#"SELECT id, name, code, description, rule_type, scope, priority, value, currency,
               conditions, valid_from, valid_to, is_active, is_stackable, max_applications,
               created_at, updated_at, created_by, updated_by FROM price_rules WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(PriceRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            rule_type: PriceRuleType::Discount,
            scope: PriceRuleScope::Global,
            priority: row.get::<i32, _>("priority"),
            value: row.get::<f64, _>("value"),
            currency: row.get::<Option<&str>, _>("currency").map(|s| s.to_string()),
            conditions: row.get::<Option<&str>, _>("conditions").map(|s| s.to_string()),
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            is_active: row.get::<i32, _>("is_active") == 1,
            is_stackable: row.get::<i32, _>("is_stackable") == 1,
            max_applications: row.get::<Option<i32>, _>("max_applications"),
        })
    }

    async fn list_price_rules(&self, pool: &SqlitePool) -> Result<Vec<PriceRule>> {
        let rows = sqlx::query(
            r#"SELECT id, name, code, description, rule_type, scope, priority, value, currency,
               conditions, valid_from, valid_to, is_active, is_stackable, max_applications,
               created_at, updated_at, created_by, updated_by FROM price_rules WHERE is_active = 1"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.iter().map(|row| PriceRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            rule_type: PriceRuleType::Discount,
            scope: PriceRuleScope::Global,
            priority: row.get::<i32, _>("priority"),
            value: row.get::<f64, _>("value"),
            currency: row.get::<Option<&str>, _>("currency").map(|s| s.to_string()),
            conditions: row.get::<Option<&str>, _>("conditions").map(|s| s.to_string()),
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            is_active: row.get::<i32, _>("is_active") == 1,
            is_stackable: row.get::<i32, _>("is_stackable") == 1,
            max_applications: row.get::<Option<i32>, _>("max_applications"),
        }).collect())
    }

    async fn create_discount(&self, pool: &SqlitePool, discount: Discount) -> Result<Discount> {
        sqlx::query(
            r#"INSERT INTO discounts (id, name, code, description, discount_type, value, max_discount,
               min_order_value, applicable_to, customer_groups, products, categories, valid_from, valid_to,
               usage_limit, usage_per_customer, current_usage, is_active, requires_code, auto_apply,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(discount.base.id.to_string())
        .bind(discount.name.clone())
        .bind(discount.code.clone())
        .bind(discount.description.clone())
        .bind(format!("{:?}", discount.discount_type))
        .bind(discount.value)
        .bind(discount.max_discount)
        .bind(discount.min_order_value)
        .bind(discount.applicable_to.clone())
        .bind(discount.customer_groups.clone())
        .bind(discount.products.clone())
        .bind(discount.categories.clone())
        .bind(discount.valid_from.map(|d| d.to_rfc3339()))
        .bind(discount.valid_to.map(|d| d.to_rfc3339()))
        .bind(discount.usage_limit)
        .bind(discount.usage_per_customer)
        .bind(discount.current_usage)
        .bind(discount.is_active as i32)
        .bind(discount.requires_code as i32)
        .bind(discount.auto_apply as i32)
        .bind(discount.base.created_at.to_rfc3339())
        .bind(discount.base.updated_at.to_rfc3339())
        .bind(discount.base.created_by.map(|id| id.to_string()))
        .bind(discount.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(discount)
    }

    async fn get_discount(&self, pool: &SqlitePool, id: Uuid) -> Result<Discount> {
        let row = sqlx::query(
            r#"SELECT id, name, code, description, discount_type, value, max_discount, min_order_value,
               applicable_to, customer_groups, products, categories, valid_from, valid_to, usage_limit,
               usage_per_customer, current_usage, is_active, requires_code, auto_apply,
               created_at, updated_at, created_by, updated_by FROM discounts WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(Discount {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            discount_type: DiscountType::Percentage,
            value: row.get::<f64, _>("value"),
            max_discount: row.get::<Option<i64>, _>("max_discount"),
            min_order_value: row.get::<Option<i64>, _>("min_order_value"),
            applicable_to: row.get::<Option<&str>, _>("applicable_to").map(|s| s.to_string()),
            customer_groups: row.get::<Option<&str>, _>("customer_groups").map(|s| s.to_string()),
            products: row.get::<Option<&str>, _>("products").map(|s| s.to_string()),
            categories: row.get::<Option<&str>, _>("categories").map(|s| s.to_string()),
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            usage_limit: row.get::<Option<i32>, _>("usage_limit"),
            usage_per_customer: row.get::<Option<i32>, _>("usage_per_customer"),
            current_usage: row.get::<i32, _>("current_usage"),
            is_active: row.get::<i32, _>("is_active") == 1,
            requires_code: row.get::<i32, _>("requires_code") == 1,
            auto_apply: row.get::<i32, _>("auto_apply") == 1,
        })
    }

    async fn get_discount_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Option<Discount>> {
        let row = sqlx::query(
            r#"SELECT id, name, code, description, discount_type, value, max_discount, min_order_value,
               applicable_to, customer_groups, products, categories, valid_from, valid_to, usage_limit,
               usage_per_customer, current_usage, is_active, requires_code, auto_apply,
               created_at, updated_at, created_by, updated_by FROM discounts WHERE code = ? AND is_active = 1"#,
        )
        .bind(code)
        .fetch_optional(pool).await?;
        
        Ok(row.map(|row| Discount {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            discount_type: DiscountType::Percentage,
            value: row.get::<f64, _>("value"),
            max_discount: row.get::<Option<i64>, _>("max_discount"),
            min_order_value: row.get::<Option<i64>, _>("min_order_value"),
            applicable_to: row.get::<Option<&str>, _>("applicable_to").map(|s| s.to_string()),
            customer_groups: row.get::<Option<&str>, _>("customer_groups").map(|s| s.to_string()),
            products: row.get::<Option<&str>, _>("products").map(|s| s.to_string()),
            categories: row.get::<Option<&str>, _>("categories").map(|s| s.to_string()),
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            usage_limit: row.get::<Option<i32>, _>("usage_limit"),
            usage_per_customer: row.get::<Option<i32>, _>("usage_per_customer"),
            current_usage: row.get::<i32, _>("current_usage"),
            is_active: row.get::<i32, _>("is_active") == 1,
            requires_code: row.get::<i32, _>("requires_code") == 1,
            auto_apply: row.get::<i32, _>("auto_apply") == 1,
        }))
    }

    async fn list_discounts(&self, pool: &SqlitePool) -> Result<Vec<Discount>> {
        let rows = sqlx::query(
            r#"SELECT id, name, code, description, discount_type, value, max_discount, min_order_value,
               applicable_to, customer_groups, products, categories, valid_from, valid_to, usage_limit,
               usage_per_customer, current_usage, is_active, requires_code, auto_apply,
               created_at, updated_at, created_by, updated_by FROM discounts WHERE is_active = 1"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.iter().map(|row| Discount {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            discount_type: DiscountType::Percentage,
            value: row.get::<f64, _>("value"),
            max_discount: row.get::<Option<i64>, _>("max_discount"),
            min_order_value: row.get::<Option<i64>, _>("min_order_value"),
            applicable_to: row.get::<Option<&str>, _>("applicable_to").map(|s| s.to_string()),
            customer_groups: row.get::<Option<&str>, _>("customer_groups").map(|s| s.to_string()),
            products: row.get::<Option<&str>, _>("products").map(|s| s.to_string()),
            categories: row.get::<Option<&str>, _>("categories").map(|s| s.to_string()),
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            usage_limit: row.get::<Option<i32>, _>("usage_limit"),
            usage_per_customer: row.get::<Option<i32>, _>("usage_per_customer"),
            current_usage: row.get::<i32, _>("current_usage"),
            is_active: row.get::<i32, _>("is_active") == 1,
            requires_code: row.get::<i32, _>("requires_code") == 1,
            auto_apply: row.get::<i32, _>("auto_apply") == 1,
        }).collect())
    }

    async fn create_coupon(&self, pool: &SqlitePool, coupon: Coupon) -> Result<Coupon> {
        sqlx::query(
            r#"INSERT INTO coupons (id, code, discount_id, promotion_id, customer_id, is_used, used_at,
               order_id, valid_from, valid_to, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(coupon.base.id.to_string())
        .bind(coupon.code.clone())
        .bind(coupon.discount_id.to_string())
        .bind(coupon.promotion_id.map(|id| id.to_string()))
        .bind(coupon.customer_id.map(|id| id.to_string()))
        .bind(coupon.is_used as i32)
        .bind(coupon.used_at.map(|d| d.to_rfc3339()))
        .bind(coupon.order_id.map(|id| id.to_string()))
        .bind(coupon.valid_from.map(|d| d.to_rfc3339()))
        .bind(coupon.valid_to.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", coupon.status))
        .bind(coupon.base.created_at.to_rfc3339())
        .bind(coupon.base.updated_at.to_rfc3339())
        .bind(coupon.base.created_by.map(|id| id.to_string()))
        .bind(coupon.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(coupon)
    }

    async fn get_coupon(&self, pool: &SqlitePool, code: &str) -> Result<Option<Coupon>> {
        let row = sqlx::query(
            r#"SELECT id, code, discount_id, promotion_id, customer_id, is_used, used_at, order_id,
               valid_from, valid_to, status, created_at, updated_at, created_by, updated_by
               FROM coupons WHERE code = ?"#,
        )
        .bind(code)
        .fetch_optional(pool).await?;
        
        Ok(row.map(|row| Coupon {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            code: row.get::<&str, _>("code").to_string(),
            discount_id: Uuid::parse_str(row.get::<&str, _>("discount_id")).unwrap(),
            promotion_id: row.get::<Option<&str>, _>("promotion_id").and_then(|s| Uuid::parse_str(s).ok()),
            customer_id: row.get::<Option<&str>, _>("customer_id").and_then(|s| Uuid::parse_str(s).ok()),
            is_used: row.get::<i32, _>("is_used") == 1,
            used_at: row.get::<Option<&str>, _>("used_at").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            order_id: row.get::<Option<&str>, _>("order_id").and_then(|s| Uuid::parse_str(s).ok()),
            valid_from: row.get::<Option<&str>, _>("valid_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_to: row.get::<Option<&str>, _>("valid_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            status: erp_core::Status::Active,
        }))
    }

    async fn use_coupon(&self, pool: &SqlitePool, id: Uuid, order_id: Uuid) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            r#"UPDATE coupons SET is_used = 1, used_at = ?, order_id = ? WHERE id = ?"#,
        )
        .bind(now)
        .bind(order_id.to_string())
        .bind(id.to_string())
        .execute(pool).await?;
        Ok(())
    }

    async fn create_promotion(&self, pool: &SqlitePool, promo: Promotion) -> Result<Promotion> {
        sqlx::query(
            r#"INSERT INTO promotions (id, name, code, description, promotion_type, status, start_date,
               end_date, rules, rewards, target_segments, channels, budget, spent, usage_limit, current_usage,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(promo.base.id.to_string())
        .bind(promo.name.clone())
        .bind(promo.code.clone())
        .bind(promo.description.clone())
        .bind(format!("{:?}", promo.promotion_type))
        .bind(format!("{:?}", promo.status))
        .bind(promo.start_date.to_rfc3339())
        .bind(promo.end_date.to_rfc3339())
        .bind(promo.rules.clone())
        .bind(promo.rewards.clone())
        .bind(promo.target_segments.clone())
        .bind(promo.channels.clone())
        .bind(promo.budget)
        .bind(promo.spent)
        .bind(promo.usage_limit)
        .bind(promo.current_usage)
        .bind(promo.base.created_at.to_rfc3339())
        .bind(promo.base.updated_at.to_rfc3339())
        .bind(promo.base.created_by.map(|id| id.to_string()))
        .bind(promo.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(promo)
    }

    async fn get_promotion(&self, pool: &SqlitePool, id: Uuid) -> Result<Promotion> {
        let row = sqlx::query(
            r#"SELECT id, name, code, description, promotion_type, status, start_date, end_date, rules,
               rewards, target_segments, channels, budget, spent, usage_limit, current_usage,
               created_at, updated_at, created_by, updated_by FROM promotions WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(Promotion {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            promotion_type: PromotionType::ProductDiscount,
            status: PromotionStatus::Draft,
            start_date: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("start_date")).unwrap().with_timezone(&chrono::Utc),
            end_date: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("end_date")).unwrap().with_timezone(&chrono::Utc),
            rules: row.get::<&str, _>("rules").to_string(),
            rewards: row.get::<&str, _>("rewards").to_string(),
            target_segments: row.get::<Option<&str>, _>("target_segments").map(|s| s.to_string()),
            channels: row.get::<Option<&str>, _>("channels").map(|s| s.to_string()),
            budget: row.get::<Option<i64>, _>("budget"),
            spent: row.get::<i64, _>("spent"),
            usage_limit: row.get::<Option<i32>, _>("usage_limit"),
            current_usage: row.get::<i32, _>("current_usage"),
        })
    }

    async fn list_promotions(&self, pool: &SqlitePool) -> Result<Vec<Promotion>> {
        let rows = sqlx::query(
            r#"SELECT id, name, code, description, promotion_type, status, start_date, end_date, rules,
               rewards, target_segments, channels, budget, spent, usage_limit, current_usage,
               created_at, updated_at, created_by, updated_by FROM promotions"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.iter().map(|row| Promotion {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.get::<&str, _>("name").to_string(),
            code: row.get::<&str, _>("code").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            promotion_type: PromotionType::ProductDiscount,
            status: PromotionStatus::Draft,
            start_date: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("start_date")).unwrap().with_timezone(&chrono::Utc),
            end_date: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("end_date")).unwrap().with_timezone(&chrono::Utc),
            rules: row.get::<&str, _>("rules").to_string(),
            rewards: row.get::<&str, _>("rewards").to_string(),
            target_segments: row.get::<Option<&str>, _>("target_segments").map(|s| s.to_string()),
            channels: row.get::<Option<&str>, _>("channels").map(|s| s.to_string()),
            budget: row.get::<Option<i64>, _>("budget"),
            spent: row.get::<i64, _>("spent"),
            usage_limit: row.get::<Option<i32>, _>("usage_limit"),
            current_usage: row.get::<i32, _>("current_usage"),
        }).collect())
    }

    async fn create_price_tier(&self, pool: &SqlitePool, tier: PriceTier) -> Result<PriceTier> {
        sqlx::query(
            r#"INSERT INTO price_tiers (id, price_book_entry_id, product_id, min_quantity, max_quantity,
               unit_price, discount_percent, currency, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(tier.base.id.to_string())
        .bind(tier.price_book_entry_id.map(|id| id.to_string()))
        .bind(tier.product_id.map(|id| id.to_string()))
        .bind(tier.min_quantity)
        .bind(tier.max_quantity)
        .bind(tier.unit_price)
        .bind(tier.discount_percent)
        .bind(tier.currency.clone())
        .bind(tier.base.created_at.to_rfc3339())
        .bind(tier.base.updated_at.to_rfc3339())
        .bind(tier.base.created_by.map(|id| id.to_string()))
        .bind(tier.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(tier)
    }

    async fn get_price_tiers(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<PriceTier>> {
        let rows = sqlx::query(
            r#"SELECT id, price_book_entry_id, product_id, min_quantity, max_quantity, unit_price,
               discount_percent, currency, created_at, updated_at, created_by, updated_by
               FROM price_tiers WHERE product_id = ? ORDER BY min_quantity"#,
        )
        .bind(product_id.to_string())
        .fetch_all(pool).await?;
        
        Ok(rows.iter().map(|row| PriceTier {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            price_book_entry_id: row.get::<Option<&str>, _>("price_book_entry_id").and_then(|s| Uuid::parse_str(s).ok()),
            product_id: row.get::<Option<&str>, _>("product_id").and_then(|s| Uuid::parse_str(s).ok()),
            min_quantity: row.get::<i32, _>("min_quantity"),
            max_quantity: row.get::<Option<i32>, _>("max_quantity"),
            unit_price: row.get::<i64, _>("unit_price"),
            discount_percent: row.get::<Option<f64>, _>("discount_percent"),
            currency: row.get::<&str, _>("currency").to_string(),
        }).collect())
    }

    async fn create_customer_price_group(&self, pool: &SqlitePool, group: CustomerPriceGroup) -> Result<CustomerPriceGroup> {
        sqlx::query(
            r#"INSERT INTO customer_price_groups (id, name, code, description, price_book_id, discount_id,
               status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(group.base.id.to_string())
        .bind(group.name.clone())
        .bind(group.code.clone())
        .bind(group.description.clone())
        .bind(group.price_book_id.map(|id| id.to_string()))
        .bind(group.discount_id.map(|id| id.to_string()))
        .bind(format!("{:?}", group.status))
        .bind(group.base.created_at.to_rfc3339())
        .bind(group.base.updated_at.to_rfc3339())
        .bind(group.base.created_by.map(|id| id.to_string()))
        .bind(group.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(group)
    }

    async fn add_customer_to_group(&self, pool: &SqlitePool, member: CustomerPriceGroupMember) -> Result<CustomerPriceGroupMember> {
        sqlx::query(
            r#"INSERT INTO customer_price_group_members (id, group_id, customer_id, joined_at,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(member.base.id.to_string())
        .bind(member.group_id.to_string())
        .bind(member.customer_id.to_string())
        .bind(member.joined_at.to_rfc3339())
        .bind(member.base.created_at.to_rfc3339())
        .bind(member.base.updated_at.to_rfc3339())
        .bind(member.base.created_by.map(|id| id.to_string()))
        .bind(member.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(member)
    }
}
