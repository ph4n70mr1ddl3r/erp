use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use chrono::{DateTime, Utc};
use crate::models::*;

pub struct SqlitePromotionRepository;

impl SqlitePromotionRepository {
    pub async fn find_by_id(pool: &SqlitePool, id: Uuid) -> Result<Promotion> {
        let row = sqlx::query_as::<_, PromotionRow>(
            "SELECT id, code, name, description, promotion_type, discount_type, discount_value,
                    max_discount, min_order_amount, start_date, end_date, usage_limit, usage_count,
                    per_customer_limit, stackable, auto_apply, priority, status, created_at, updated_at
             FROM promotions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Promotion", &id.to_string()))?;

        let applies_to = Self::load_applies_to(pool, id).await?;
        Ok(row.into_promotion(applies_to))
    }

    pub async fn find_by_code(pool: &SqlitePool, code: &str) -> Result<Promotion> {
        let row = sqlx::query_as::<_, PromotionRow>(
            "SELECT id, code, name, description, promotion_type, discount_type, discount_value,
                    max_discount, min_order_amount, start_date, end_date, usage_limit, usage_count,
                    per_customer_limit, stackable, auto_apply, priority, status, created_at, updated_at
             FROM promotions WHERE code = ?"
        )
        .bind(code)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Promotion", code))?;

        let id = Uuid::parse_str(&row.id).unwrap_or_default();
        let applies_to = Self::load_applies_to(pool, id).await?;
        Ok(row.into_promotion(applies_to))
    }

    pub async fn find_all(pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Promotion>> {
        let offset = (pagination.page - 1) * pagination.per_page;
        
        let rows = sqlx::query_as::<_, PromotionRow>(
            "SELECT id, code, name, description, promotion_type, discount_type, discount_value,
                    max_discount, min_order_amount, start_date, end_date, usage_limit, usage_count,
                    per_customer_limit, stackable, auto_apply, priority, status, created_at, updated_at
             FROM promotions ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM promotions")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))?;

        let mut promotions = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let applies_to = Self::load_applies_to(pool, id).await?;
            promotions.push(row.into_promotion(applies_to));
        }

        Ok(Paginated {
            items: promotions,
            total: total.0 as u64,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages: ((total.0 as f64) / (pagination.per_page as f64)).ceil() as u32,
        })
    }

    pub async fn find_active(pool: &SqlitePool) -> Result<Vec<Promotion>> {
        let now = chrono::Utc::now().to_rfc3339();
        let rows = sqlx::query_as::<_, PromotionRow>(
            "SELECT id, code, name, description, promotion_type, discount_type, discount_value,
                    max_discount, min_order_amount, start_date, end_date, usage_limit, usage_count,
                    per_customer_limit, stackable, auto_apply, priority, status, created_at, updated_at
             FROM promotions 
             WHERE status = 'Active' 
             AND start_date <= ? 
             AND end_date >= ?
             ORDER BY priority DESC"
        )
        .bind(&now)
        .bind(&now)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let mut promotions = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let applies_to = Self::load_applies_to(pool, id).await?;
            promotions.push(row.into_promotion(applies_to));
        }

        Ok(promotions)
    }

    pub async fn create(pool: &SqlitePool, promotion: &Promotion) -> Result<()> {
        sqlx::query(
            "INSERT INTO promotions (id, code, name, description, promotion_type, discount_type, 
             discount_value, max_discount, min_order_amount, start_date, end_date, usage_limit, 
             usage_count, per_customer_limit, stackable, auto_apply, priority, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(promotion.base.id.to_string())
        .bind(&promotion.code)
        .bind(&promotion.name)
        .bind(&promotion.description)
        .bind(format!("{:?}", promotion.promotion_type))
        .bind(format!("{:?}", promotion.discount_type))
        .bind(promotion.discount_value)
        .bind(promotion.max_discount)
        .bind(promotion.min_order_amount)
        .bind(promotion.start_date.to_rfc3339())
        .bind(promotion.end_date.to_rfc3339())
        .bind(promotion.usage_limit)
        .bind(promotion.usage_count)
        .bind(promotion.per_customer_limit)
        .bind(promotion.stackable as i32)
        .bind(promotion.auto_apply as i32)
        .bind(promotion.priority)
        .bind(format!("{:?}", promotion.status))
        .bind(promotion.base.created_at.to_rfc3339())
        .bind(promotion.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Self::save_applies_to(pool, promotion.base.id, &promotion.applies_to).await?;
        Ok(())
    }

    pub async fn update(pool: &SqlitePool, promotion: &Promotion) -> Result<()> {
        sqlx::query(
            "UPDATE promotions SET code=?, name=?, description=?, promotion_type=?, discount_type=?,
             discount_value=?, max_discount=?, min_order_amount=?, start_date=?, end_date=?,
             usage_limit=?, usage_count=?, per_customer_limit=?, stackable=?, auto_apply=?,
             priority=?, status=?, updated_at=? WHERE id=?"
        )
        .bind(&promotion.code)
        .bind(&promotion.name)
        .bind(&promotion.description)
        .bind(format!("{:?}", promotion.promotion_type))
        .bind(format!("{:?}", promotion.discount_type))
        .bind(promotion.discount_value)
        .bind(promotion.max_discount)
        .bind(promotion.min_order_amount)
        .bind(promotion.start_date.to_rfc3339())
        .bind(promotion.end_date.to_rfc3339())
        .bind(promotion.usage_limit)
        .bind(promotion.usage_count)
        .bind(promotion.per_customer_limit)
        .bind(promotion.stackable as i32)
        .bind(promotion.auto_apply as i32)
        .bind(promotion.priority)
        .bind(format!("{:?}", promotion.status))
        .bind(promotion.base.updated_at.to_rfc3339())
        .bind(promotion.base.id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Self::save_applies_to(pool, promotion.base.id, &promotion.applies_to).await?;
        Ok(())
    }

    pub async fn increment_usage(pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE promotions SET usage_count = usage_count + 1 WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        Ok(())
    }

    async fn load_applies_to(pool: &SqlitePool, promotion_id: Uuid) -> Result<PromotionAppliesTo> {
        #[derive(sqlx::FromRow)]
        struct IdRow {
            product_id: String,
        }
        #[derive(sqlx::FromRow)]
        struct CatRow {
            category_id: String,
        }
        #[derive(sqlx::FromRow)]
        struct GroupRow {
            customer_group_id: String,
        }

        let product_rows: Vec<IdRow> = sqlx::query_as(
            "SELECT product_id FROM promotion_products WHERE promotion_id = ? AND include = 1"
        )
        .bind(promotion_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let category_rows: Vec<CatRow> = sqlx::query_as(
            "SELECT category_id FROM promotion_categories WHERE promotion_id = ? AND include = 1"
        )
        .bind(promotion_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let exclude_product_rows: Vec<IdRow> = sqlx::query_as(
            "SELECT product_id FROM promotion_products WHERE promotion_id = ? AND include = 0"
        )
        .bind(promotion_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let exclude_category_rows: Vec<CatRow> = sqlx::query_as(
            "SELECT category_id FROM promotion_categories WHERE promotion_id = ? AND include = 0"
        )
        .bind(promotion_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let group_rows: Vec<GroupRow> = sqlx::query_as(
            "SELECT customer_group_id FROM promotion_customer_groups WHERE promotion_id = ?"
        )
        .bind(promotion_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(PromotionAppliesTo {
            product_ids: product_rows.iter().filter_map(|r| Uuid::parse_str(&r.product_id).ok()).collect(),
            category_ids: category_rows.iter().filter_map(|r| Uuid::parse_str(&r.category_id).ok()).collect(),
            customer_group_ids: group_rows.iter().filter_map(|r| Uuid::parse_str(&r.customer_group_id).ok()).collect(),
            exclude_product_ids: exclude_product_rows.iter().filter_map(|r| Uuid::parse_str(&r.product_id).ok()).collect(),
            exclude_category_ids: exclude_category_rows.iter().filter_map(|r| Uuid::parse_str(&r.category_id).ok()).collect(),
        })
    }

    async fn save_applies_to(pool: &SqlitePool, promotion_id: Uuid, applies_to: &PromotionAppliesTo) -> Result<()> {
        sqlx::query("DELETE FROM promotion_products WHERE promotion_id = ?")
            .bind(promotion_id.to_string())
            .execute(pool)
            .await
            .ok();

        sqlx::query("DELETE FROM promotion_categories WHERE promotion_id = ?")
            .bind(promotion_id.to_string())
            .execute(pool)
            .await
            .ok();

        sqlx::query("DELETE FROM promotion_customer_groups WHERE promotion_id = ?")
            .bind(promotion_id.to_string())
            .execute(pool)
            .await
            .ok();

        for product_id in &applies_to.product_ids {
            sqlx::query("INSERT INTO promotion_products (promotion_id, product_id, include) VALUES (?, ?, 1)")
                .bind(promotion_id.to_string())
                .bind(product_id.to_string())
                .execute(pool)
                .await
                .ok();
        }

        for product_id in &applies_to.exclude_product_ids {
            sqlx::query("INSERT INTO promotion_products (promotion_id, product_id, include) VALUES (?, ?, 0)")
                .bind(promotion_id.to_string())
                .bind(product_id.to_string())
                .execute(pool)
                .await
                .ok();
        }

        for category_id in &applies_to.category_ids {
            sqlx::query("INSERT INTO promotion_categories (promotion_id, category_id, include) VALUES (?, ?, 1)")
                .bind(promotion_id.to_string())
                .bind(category_id.to_string())
                .execute(pool)
                .await
                .ok();
        }

        for category_id in &applies_to.exclude_category_ids {
            sqlx::query("INSERT INTO promotion_categories (promotion_id, category_id, include) VALUES (?, ?, 0)")
                .bind(promotion_id.to_string())
                .bind(category_id.to_string())
                .execute(pool)
                .await
                .ok();
        }

        for group_id in &applies_to.customer_group_ids {
            sqlx::query("INSERT INTO promotion_customer_groups (promotion_id, customer_group_id) VALUES (?, ?)")
                .bind(promotion_id.to_string())
                .bind(group_id.to_string())
                .execute(pool)
                .await
                .ok();
        }

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PromotionRow {
    id: String,
    code: String,
    name: String,
    description: Option<String>,
    promotion_type: String,
    discount_type: String,
    discount_value: i64,
    max_discount: Option<i64>,
    min_order_amount: Option<i64>,
    start_date: String,
    end_date: String,
    usage_limit: Option<i64>,
    usage_count: i64,
    per_customer_limit: Option<i64>,
    stackable: i64,
    auto_apply: i64,
    priority: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

impl PromotionRow {
    fn into_promotion(self, applies_to: PromotionAppliesTo) -> Promotion {
        Promotion {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            code: self.code,
            name: self.name,
            description: self.description,
            promotion_type: match self.promotion_type.as_str() {
                "LineItemDiscount" => PromotionType::LineItemDiscount,
                "BuyXGetY" => PromotionType::BuyXGetY,
                "FreeShipping" => PromotionType::FreeShipping,
                "BundleDiscount" => PromotionType::BundleDiscount,
                "LoyaltyPoints" => PromotionType::LoyaltyPoints,
                _ => PromotionType::OrderDiscount,
            },
            discount_type: match self.discount_type.as_str() {
                "FixedAmount" => DiscountType::FixedAmount,
                "FixedPrice" => DiscountType::FixedPrice,
                _ => DiscountType::Percentage,
            },
            discount_value: self.discount_value,
            max_discount: self.max_discount,
            min_order_amount: self.min_order_amount,
            start_date: DateTime::parse_from_rfc3339(&self.start_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            end_date: DateTime::parse_from_rfc3339(&self.end_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            usage_limit: self.usage_limit.map(|v| v as i32),
            usage_count: self.usage_count as i32,
            per_customer_limit: self.per_customer_limit.map(|v| v as i32),
            applies_to,
            stackable: self.stackable != 0,
            auto_apply: self.auto_apply != 0,
            priority: self.priority as i32,
            status: match self.status.as_str() {
                "Inactive" => Status::Inactive,
                "Draft" => Status::Draft,
                _ => Status::Active,
            },
        }
    }
}

pub struct SqliteCouponRepository;

impl SqliteCouponRepository {
    pub async fn find_by_id(pool: &SqlitePool, id: Uuid) -> Result<Coupon> {
        let row = sqlx::query_as::<_, CouponRow>(
            "SELECT id, code, promotion_id, coupon_type, discount_type, discount_value,
                    max_discount, min_order_amount, start_date, end_date, usage_limit, usage_count,
                    per_customer_limit, customer_email, first_time_only, status, created_at, updated_at
             FROM coupons WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Coupon", &id.to_string()))?;

        Ok(row.into())
    }

    pub async fn find_by_code(pool: &SqlitePool, code: &str) -> Result<Coupon> {
        let row = sqlx::query_as::<_, CouponRow>(
            "SELECT id, code, promotion_id, coupon_type, discount_type, discount_value,
                    max_discount, min_order_amount, start_date, end_date, usage_limit, usage_count,
                    per_customer_limit, customer_email, first_time_only, status, created_at, updated_at
             FROM coupons WHERE code = ?"
        )
        .bind(code)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Coupon", code))?;

        Ok(row.into())
    }

    pub async fn find_all(pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Coupon>> {
        let offset = (pagination.page - 1) * pagination.per_page;
        
        let rows = sqlx::query_as::<_, CouponRow>(
            "SELECT id, code, promotion_id, coupon_type, discount_type, discount_value,
                    max_discount, min_order_amount, start_date, end_date, usage_limit, usage_count,
                    per_customer_limit, customer_email, first_time_only, status, created_at, updated_at
             FROM coupons ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM coupons")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))?;

        Ok(Paginated {
            items: rows.into_iter().map(|r| r.into()).collect(),
            total: total.0 as u64,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages: ((total.0 as f64) / (pagination.per_page as f64)).ceil() as u32,
        })
    }

    pub async fn create(pool: &SqlitePool, coupon: &Coupon) -> Result<()> {
        sqlx::query(
            "INSERT INTO coupons (id, code, promotion_id, coupon_type, discount_type, discount_value,
             max_discount, min_order_amount, start_date, end_date, usage_limit, usage_count,
             per_customer_limit, customer_email, first_time_only, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(coupon.base.id.to_string())
        .bind(&coupon.code)
        .bind(coupon.promotion_id.to_string())
        .bind(format!("{:?}", coupon.coupon_type))
        .bind(format!("{:?}", coupon.discount_type))
        .bind(coupon.discount_value)
        .bind(coupon.max_discount)
        .bind(coupon.min_order_amount)
        .bind(coupon.start_date.to_rfc3339())
        .bind(coupon.end_date.to_rfc3339())
        .bind(coupon.usage_limit)
        .bind(coupon.usage_count)
        .bind(coupon.per_customer_limit)
        .bind(&coupon.customer_email)
        .bind(coupon.first_time_only as i32)
        .bind(format!("{:?}", coupon.status))
        .bind(coupon.base.created_at.to_rfc3339())
        .bind(coupon.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(())
    }

    pub async fn increment_usage(pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE coupons SET usage_count = usage_count + 1 WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        Ok(())
    }

    pub async fn delete_by_code(pool: &SqlitePool, code: &str) -> Result<()> {
        sqlx::query("DELETE FROM coupons WHERE code = ?")
            .bind(code)
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct CouponRow {
    id: String,
    code: String,
    promotion_id: String,
    coupon_type: String,
    discount_type: String,
    discount_value: i64,
    max_discount: Option<i64>,
    min_order_amount: Option<i64>,
    start_date: String,
    end_date: String,
    usage_limit: Option<i64>,
    usage_count: i64,
    per_customer_limit: Option<i64>,
    customer_email: Option<String>,
    first_time_only: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<CouponRow> for Coupon {
    fn from(r: CouponRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            code: r.code,
            promotion_id: Uuid::parse_str(&r.promotion_id).unwrap_or_default(),
            coupon_type: match r.coupon_type.as_str() {
                "MultiUse" => CouponType::MultiUse,
                "Unlimited" => CouponType::Unlimited,
                "Referral" => CouponType::Referral,
                _ => CouponType::SingleUse,
            },
            discount_type: match r.discount_type.as_str() {
                "FixedAmount" => DiscountType::FixedAmount,
                "FixedPrice" => DiscountType::FixedPrice,
                _ => DiscountType::Percentage,
            },
            discount_value: r.discount_value,
            max_discount: r.max_discount,
            min_order_amount: r.min_order_amount,
            start_date: DateTime::parse_from_rfc3339(&r.start_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            end_date: DateTime::parse_from_rfc3339(&r.end_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            usage_limit: r.usage_limit.map(|v| v as i32),
            usage_count: r.usage_count as i32,
            per_customer_limit: r.per_customer_limit.map(|v| v as i32),
            customer_email: r.customer_email,
            first_time_only: r.first_time_only != 0,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                "Used" => Status::Completed,
                _ => Status::Active,
            },
        }
    }
}

pub struct SqlitePromotionUsageRepository;

impl SqlitePromotionUsageRepository {
    pub async fn create(pool: &SqlitePool, usage: &PromotionUsage) -> Result<()> {
        sqlx::query(
            "INSERT INTO promotion_usages (id, promotion_id, coupon_id, order_id, customer_id,
             customer_email, discount_amount, original_amount, final_amount, used_at, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(usage.id.to_string())
        .bind(usage.promotion_id.map(|id| id.to_string()))
        .bind(usage.coupon_id.map(|id| id.to_string()))
        .bind(usage.order_id.to_string())
        .bind(usage.customer_id.map(|id| id.to_string()))
        .bind(&usage.customer_email)
        .bind(usage.discount_amount)
        .bind(usage.original_amount)
        .bind(usage.final_amount)
        .bind(usage.used_at.to_rfc3339())
        .bind(format!("{:?}", usage.status))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }

    pub async fn find_by_customer(pool: &SqlitePool, customer_id: Uuid, promotion_id: Uuid) -> Result<Vec<PromotionUsage>> {
        let rows = sqlx::query_as::<_, UsageRow>(
            "SELECT id, promotion_id, coupon_id, order_id, customer_id, customer_email,
                    discount_amount, original_amount, final_amount, used_at, status
             FROM promotion_usages 
             WHERE customer_id = ? AND promotion_id = ? AND status = 'Applied'"
        )
        .bind(customer_id.to_string())
        .bind(promotion_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_usage_report(pool: &SqlitePool, promotion_id: Uuid) -> Result<PromotionReport> {
        let promotion = SqlitePromotionRepository::find_by_id(pool, promotion_id).await?;
        
        let stats: (i64, i64, i64, i64) = sqlx::query_as(
            "SELECT COUNT(*), COALESCE(SUM(discount_amount), 0), COALESCE(SUM(original_amount), 0), 
                    COUNT(DISTINCT customer_id)
             FROM promotion_usages WHERE promotion_id = ? AND status = 'Applied'"
        )
        .bind(promotion_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(PromotionReport {
            promotion_id,
            promotion_code: promotion.code,
            promotion_name: promotion.name,
            total_usage: stats.0 as i32,
            total_discount: stats.1,
            total_revenue: stats.2,
            unique_customers: stats.3 as i32,
            avg_order_value: if stats.0 > 0 { stats.2 / stats.0 } else { 0 },
            conversion_rate: 0.0,
        })
    }
}

#[derive(sqlx::FromRow)]
struct UsageRow {
    id: String,
    promotion_id: Option<String>,
    coupon_id: Option<String>,
    order_id: String,
    customer_id: Option<String>,
    customer_email: Option<String>,
    discount_amount: i64,
    original_amount: i64,
    final_amount: i64,
    used_at: String,
    status: String,
}

impl From<UsageRow> for PromotionUsage {
    fn from(r: UsageRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            promotion_id: r.promotion_id.and_then(|s| Uuid::parse_str(&s).ok()),
            coupon_id: r.coupon_id.and_then(|s| Uuid::parse_str(&s).ok()),
            order_id: Uuid::parse_str(&r.order_id).unwrap_or_default(),
            customer_id: r.customer_id.and_then(|s| Uuid::parse_str(&s).ok()),
            customer_email: r.customer_email,
            discount_amount: r.discount_amount,
            original_amount: r.original_amount,
            final_amount: r.final_amount,
            used_at: DateTime::parse_from_rfc3339(&r.used_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            status: match r.status.as_str() {
                "Reverted" => PromotionUsageStatus::Reverted,
                "Expired" => PromotionUsageStatus::Expired,
                _ => PromotionUsageStatus::Applied,
            },
        }
    }
}
