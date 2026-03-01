use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use erp_core::{Error, Result, Pagination, BaseEntity, Status, Paginated};
use rand::Rng;
use crate::models::*;
use crate::repository::*;

pub struct PromotionService;

impl Default for PromotionService {
    fn default() -> Self {
        Self::new()
    }
}

impl PromotionService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Promotion> {
        SqlitePromotionRepository::find_by_id(pool, id).await
    }

    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Promotion> {
        SqlitePromotionRepository::find_by_code(pool, code).await
    }

    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Promotion>> {
        SqlitePromotionRepository::find_all(pool, pagination).await
    }

    pub async fn list_active(&self, pool: &SqlitePool) -> Result<Vec<Promotion>> {
        SqlitePromotionRepository::find_active(pool).await
    }

    pub async fn create(&self, pool: &SqlitePool, req: CreatePromotionRequest) -> Result<Promotion> {
        if SqlitePromotionRepository::find_by_code(pool, &req.code).await.is_ok() {
            return Err(Error::Conflict(format!("Promotion code '{}' already exists", req.code)));
        }

        let _now = Utc::now();
        let promotion = Promotion {
            base: BaseEntity::new(),
            code: req.code,
            name: req.name,
            description: req.description,
            promotion_type: req.promotion_type,
            discount_type: req.discount_type,
            discount_value: req.discount_value,
            max_discount: req.max_discount,
            min_order_amount: req.min_order_amount,
            start_date: req.start_date,
            end_date: req.end_date,
            usage_limit: req.usage_limit,
            usage_count: 0,
            per_customer_limit: req.per_customer_limit,
            applies_to: req.applies_to.unwrap_or(PromotionAppliesTo {
                product_ids: vec![],
                category_ids: vec![],
                customer_group_ids: vec![],
                exclude_product_ids: vec![],
                exclude_category_ids: vec![],
            }),
            stackable: req.stackable.unwrap_or(false),
            auto_apply: req.auto_apply.unwrap_or(false),
            priority: req.priority.unwrap_or(0),
            status: Status::Draft,
        };

        SqlitePromotionRepository::create(pool, &promotion).await?;
        Ok(promotion)
    }

    pub async fn update(&self, pool: &SqlitePool, id: Uuid, req: UpdatePromotionRequest) -> Result<Promotion> {
        let mut promotion = SqlitePromotionRepository::find_by_id(pool, id).await?;
        
        if let Some(name) = req.name {
            promotion.name = name;
        }
        if let Some(description) = req.description {
            promotion.description = Some(description);
        }
        if let Some(discount_value) = req.discount_value {
            promotion.discount_value = discount_value;
        }
        if let Some(max_discount) = req.max_discount {
            promotion.max_discount = Some(max_discount);
        }
        if let Some(min_order_amount) = req.min_order_amount {
            promotion.min_order_amount = Some(min_order_amount);
        }
        if let Some(start_date) = req.start_date {
            promotion.start_date = start_date;
        }
        if let Some(end_date) = req.end_date {
            promotion.end_date = end_date;
        }
        if let Some(usage_limit) = req.usage_limit {
            promotion.usage_limit = Some(usage_limit);
        }
        if let Some(per_customer_limit) = req.per_customer_limit {
            promotion.per_customer_limit = Some(per_customer_limit);
        }
        if let Some(stackable) = req.stackable {
            promotion.stackable = stackable;
        }
        if let Some(auto_apply) = req.auto_apply {
            promotion.auto_apply = auto_apply;
        }
        if let Some(priority) = req.priority {
            promotion.priority = priority;
        }
        if let Some(applies_to) = req.applies_to {
            promotion.applies_to = applies_to;
        }

        promotion.base.updated_at = Utc::now();
        SqlitePromotionRepository::update(pool, &promotion).await?;
        Ok(promotion)
    }

    pub async fn activate(&self, pool: &SqlitePool, id: Uuid) -> Result<Promotion> {
        let mut promotion = SqlitePromotionRepository::find_by_id(pool, id).await?;
        promotion.status = Status::Active;
        promotion.base.updated_at = Utc::now();
        SqlitePromotionRepository::update(pool, &promotion).await?;
        Ok(promotion)
    }

    pub async fn deactivate(&self, pool: &SqlitePool, id: Uuid) -> Result<Promotion> {
        let mut promotion = SqlitePromotionRepository::find_by_id(pool, id).await?;
        promotion.status = Status::Inactive;
        promotion.base.updated_at = Utc::now();
        SqlitePromotionRepository::update(pool, &promotion).await?;
        Ok(promotion)
    }

    pub async fn calculate_discount(
        &self,
        pool: &SqlitePool,
        promotion_id: Uuid,
        order_amount: i64,
        product_ids: Vec<Uuid>,
    ) -> Result<i64> {
        let promotion = SqlitePromotionRepository::find_by_id(pool, promotion_id).await?;
        
        if promotion.status != Status::Active {
            return Ok(0);
        }

        let now = Utc::now();
        if now < promotion.start_date || now > promotion.end_date {
            return Ok(0);
        }

        if let Some(limit) = promotion.usage_limit {
            if promotion.usage_count >= limit {
                return Ok(0);
            }
        }

        if let Some(min) = promotion.min_order_amount {
            if order_amount < min {
                return Ok(0);
            }
        }

        let has_applicable_products = if !promotion.applies_to.product_ids.is_empty() {
            product_ids.iter().any(|id| promotion.applies_to.product_ids.contains(id))
        } else {
            true
        };

        if !has_applicable_products {
            return Ok(0);
        }

        let discount = match promotion.discount_type {
            DiscountType::Percentage => {
                let pct = promotion.discount_value as f64 / 100.0;
                let mut d = (order_amount as f64 * pct) as i64;
                if let Some(max) = promotion.max_discount {
                    d = d.min(max);
                }
                d
            }
            DiscountType::FixedAmount => promotion.discount_value,
            DiscountType::FixedPrice => order_amount.saturating_sub(promotion.discount_value),
        };

        Ok(discount)
    }

    pub async fn record_usage(
        &self,
        pool: &SqlitePool,
        promotion_id: Uuid,
        order_id: Uuid,
        customer_id: Option<Uuid>,
        discount_amount: i64,
        original_amount: i64,
    ) -> Result<PromotionUsage> {
        SqlitePromotionRepository::increment_usage(pool, promotion_id).await?;

        let usage = PromotionUsage {
            id: Uuid::new_v4(),
            promotion_id: Some(promotion_id),
            coupon_id: None,
            order_id,
            customer_id,
            customer_email: None,
            discount_amount,
            original_amount,
            final_amount: original_amount - discount_amount,
            used_at: Utc::now(),
            status: PromotionUsageStatus::Applied,
        };

        SqlitePromotionUsageRepository::create(pool, &usage).await?;
        Ok(usage)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CreatePromotionRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub promotion_type: PromotionType,
    pub discount_type: DiscountType,
    pub discount_value: i64,
    pub max_discount: Option<i64>,
    pub min_order_amount: Option<i64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub usage_limit: Option<i32>,
    pub per_customer_limit: Option<i32>,
    pub applies_to: Option<PromotionAppliesTo>,
    pub stackable: Option<bool>,
    pub auto_apply: Option<bool>,
    pub priority: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdatePromotionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub discount_value: Option<i64>,
    pub max_discount: Option<i64>,
    pub min_order_amount: Option<i64>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub usage_limit: Option<i32>,
    pub per_customer_limit: Option<i32>,
    pub stackable: Option<bool>,
    pub auto_apply: Option<bool>,
    pub priority: Option<i32>,
    pub applies_to: Option<PromotionAppliesTo>,
}

pub struct CouponService;

impl Default for CouponService {
    fn default() -> Self {
        Self::new()
    }
}

impl CouponService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Coupon> {
        SqliteCouponRepository::find_by_id(pool, id).await
    }

    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Coupon> {
        SqliteCouponRepository::find_by_code(pool, code).await
    }

    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Coupon>> {
        SqliteCouponRepository::find_all(pool, pagination).await
    }

    pub async fn create(&self, pool: &SqlitePool, req: CreateCouponRequest) -> Result<Coupon> {
        if SqliteCouponRepository::find_by_code(pool, &req.code).await.is_ok() {
            return Err(Error::Conflict(format!("Coupon code '{}' already exists", req.code)));
        }

        let coupon = Coupon {
            base: BaseEntity::new(),
            code: req.code,
            promotion_id: req.promotion_id,
            coupon_type: req.coupon_type,
            discount_type: req.discount_type,
            discount_value: req.discount_value,
            max_discount: req.max_discount,
            min_order_amount: req.min_order_amount,
            start_date: req.start_date,
            end_date: req.end_date,
            usage_limit: req.usage_limit,
            usage_count: 0,
            per_customer_limit: req.per_customer_limit,
            customer_email: req.customer_email,
            first_time_only: req.first_time_only.unwrap_or(false),
            status: Status::Active,
        };

        SqliteCouponRepository::create(pool, &coupon).await?;
        Ok(coupon)
    }

    pub async fn validate(
        &self,
        pool: &SqlitePool,
        code: &str,
        order_amount: i64,
        customer_id: Option<Uuid>,
        customer_email: Option<&str>,
        is_first_order: bool,
    ) -> Result<CouponValidation> {
        let coupon = match SqliteCouponRepository::find_by_code(pool, code).await {
            Ok(c) => c,
            Err(_) => {
                return Ok(CouponValidation {
                    valid: false,
                    coupon: None,
                    promotion: None,
                    error_message: Some("Invalid coupon code".to_string()),
                    discount_preview: None,
                });
            }
        };

        if coupon.status != Status::Active {
            return Ok(CouponValidation {
                valid: false,
                coupon: Some(coupon),
                promotion: None,
                error_message: Some("Coupon is no longer valid".to_string()),
                discount_preview: None,
            });
        }

        let now = Utc::now();
        if now < coupon.start_date || now > coupon.end_date {
            return Ok(CouponValidation {
                valid: false,
                coupon: Some(coupon.clone()),
                promotion: None,
                error_message: Some("Coupon has expired".to_string()),
                discount_preview: None,
            });
        }

        if let Some(limit) = coupon.usage_limit {
            if coupon.usage_count >= limit {
                return Ok(CouponValidation {
                    valid: false,
                    coupon: Some(coupon.clone()),
                    promotion: None,
                    error_message: Some("Coupon usage limit reached".to_string()),
                    discount_preview: None,
                });
            }
        }

        if let Some(ref email) = coupon.customer_email {
            if customer_email != Some(email.as_str()) {
                return Ok(CouponValidation {
                    valid: false,
                    coupon: Some(coupon.clone()),
                    promotion: None,
                    error_message: Some("Coupon not valid for this customer".to_string()),
                    discount_preview: None,
                });
            }
        }

        if coupon.first_time_only && !is_first_order {
            return Ok(CouponValidation {
                valid: false,
                coupon: Some(coupon.clone()),
                promotion: None,
                error_message: Some("Coupon valid for first order only".to_string()),
                discount_preview: None,
            });
        }

        if let Some(min) = coupon.min_order_amount {
            if order_amount < min {
                return Ok(CouponValidation {
                    valid: false,
                    coupon: Some(coupon.clone()),
                    promotion: None,
                    error_message: Some(format!("Minimum order amount is {:.2}", min as f64 / 100.0)),
                    discount_preview: None,
                });
            }
        }

        if let (Some(cust_id), Some(limit)) = (customer_id, coupon.per_customer_limit) {
            let usages = SqlitePromotionUsageRepository::find_by_customer(pool, cust_id, coupon.promotion_id).await?;
            if usages.len() >= limit as usize {
                return Ok(CouponValidation {
                    valid: false,
                    coupon: Some(coupon.clone()),
                    promotion: None,
                    error_message: Some("Customer usage limit reached".to_string()),
                    discount_preview: None,
                });
            }
        }

        let discount = Self::calculate_coupon_discount(&coupon, order_amount);

        let promotion = SqlitePromotionRepository::find_by_id(pool, coupon.promotion_id).await.ok();

        Ok(CouponValidation {
            valid: true,
            coupon: Some(coupon.clone()),
            promotion,
            error_message: None,
            discount_preview: Some(DiscountPreview {
                original_amount: order_amount,
                discount_amount: discount,
                final_amount: order_amount - discount,
                applied_rules: vec![format!("Coupon: {}", code)],
            }),
        })
    }

    fn calculate_coupon_discount(coupon: &Coupon, order_amount: i64) -> i64 {
        let mut discount = match coupon.discount_type {
            DiscountType::Percentage => {
                let pct = coupon.discount_value as f64 / 100.0;
                (order_amount as f64 * pct) as i64
            }
            DiscountType::FixedAmount => coupon.discount_value,
            DiscountType::FixedPrice => order_amount.saturating_sub(coupon.discount_value),
        };

        if let Some(max) = coupon.max_discount {
            discount = discount.min(max);
        }

        discount
    }

    pub async fn apply(
        &self,
        pool: &SqlitePool,
        code: &str,
        order_id: Uuid,
        customer_id: Option<Uuid>,
        order_amount: i64,
    ) -> Result<(Coupon, i64)> {
        let coupon = SqliteCouponRepository::find_by_code(pool, code).await?;
        let discount = Self::calculate_coupon_discount(&coupon, order_amount);

        SqliteCouponRepository::increment_usage(pool, coupon.base.id).await?;

        let usage = PromotionUsage {
            id: Uuid::new_v4(),
            promotion_id: Some(coupon.promotion_id),
            coupon_id: Some(coupon.base.id),
            order_id,
            customer_id,
            customer_email: coupon.customer_email.clone(),
            discount_amount: discount,
            original_amount: order_amount,
            final_amount: order_amount - discount,
            used_at: Utc::now(),
            status: PromotionUsageStatus::Applied,
        };

        SqlitePromotionUsageRepository::create(pool, &usage).await?;

        if coupon.coupon_type == CouponType::SingleUse {
            SqliteCouponRepository::delete_by_code(pool, code).await?;
        }

        Ok((coupon, discount))
    }

    pub async fn generate_batch(
        &self,
        pool: &SqlitePool,
        req: GenerateCouponBatchRequest,
    ) -> Result<Vec<Coupon>> {
        let mut coupons = Vec::new();
        for _ in 0..req.quantity {
            let code = {
                let mut rng = rand::thread_rng();
                let suffix: String = (0..req.length)
                    .map(|_| {
                        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                        let idx = rng.gen_range(0..CHARSET.len());
                        CHARSET[idx] as char
                    })
                    .collect();
                format!("{}{}", req.prefix, suffix)
            };

            let coupon = Coupon {
                base: BaseEntity::new(),
                code: code.clone(),
                promotion_id: req.promotion_id,
                coupon_type: req.coupon_type.clone().unwrap_or(CouponType::SingleUse),
                discount_type: req.discount_type.clone(),
                discount_value: req.discount_value,
                max_discount: req.max_discount,
                min_order_amount: req.min_order_amount,
                start_date: req.start_date,
                end_date: req.end_date,
                usage_limit: Some(1),
                usage_count: 0,
                per_customer_limit: Some(1),
                customer_email: None,
                first_time_only: false,
                status: Status::Active,
            };

            SqliteCouponRepository::create(pool, &coupon).await?;
            coupons.push(coupon);
        }

        Ok(coupons)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub promotion_id: Uuid,
    pub coupon_type: CouponType,
    pub discount_type: DiscountType,
    pub discount_value: i64,
    pub max_discount: Option<i64>,
    pub min_order_amount: Option<i64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub usage_limit: Option<i32>,
    pub per_customer_limit: Option<i32>,
    pub customer_email: Option<String>,
    pub first_time_only: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GenerateCouponBatchRequest {
    pub promotion_id: Uuid,
    pub prefix: String,
    pub quantity: i32,
    pub length: i32,
    pub discount_type: DiscountType,
    pub discount_value: i64,
    pub max_discount: Option<i64>,
    pub min_order_amount: Option<i64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub coupon_type: Option<CouponType>,
}

pub struct PromotionReportingService;

impl Default for PromotionReportingService {
    fn default() -> Self {
        Self::new()
    }
}

impl PromotionReportingService {
    pub fn new() -> Self { Self }

    pub async fn get_report(pool: &SqlitePool, promotion_id: Uuid) -> Result<PromotionReport> {
        SqlitePromotionUsageRepository::get_usage_report(pool, promotion_id).await
    }

    pub async fn list_active_promotions_report(pool: &SqlitePool) -> Result<Vec<PromotionReport>> {
        let promotions = SqlitePromotionRepository::find_active(pool).await?;
        let mut reports = Vec::new();

        for promo in promotions {
            if let Ok(report) = SqlitePromotionUsageRepository::get_usage_report(pool, promo.base.id).await {
                reports.push(report);
            }
        }

        Ok(reports)
    }
}
