use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Result, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct PricingService {
    repo: SqlitePricingRepository,
}

impl PricingService {
    pub fn new() -> Self {
        Self { repo: SqlitePricingRepository }
    }

    pub async fn create_price_book(&self, pool: &SqlitePool, name: String, code: String, currency: String) -> Result<PriceBook> {
        let pb = PriceBook {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            currency,
            is_default: false,
            is_active: true,
            valid_from: None,
            valid_to: None,
            parent_id: None,
        };
        self.repo.create_price_book(pool, pb).await
    }

    pub async fn get_price_book(&self, pool: &SqlitePool, id: Uuid) -> Result<PriceBook> {
        self.repo.get_price_book(pool, id).await
    }

    pub async fn list_price_books(&self, pool: &SqlitePool) -> Result<Vec<PriceBook>> {
        self.repo.list_price_books(pool).await
    }

    pub async fn set_product_price(&self, pool: &SqlitePool, price_book_id: Uuid, product_id: Uuid, unit_price: i64, currency: String) -> Result<PriceBookEntry> {
        let entry = PriceBookEntry {
            base: BaseEntity::new(),
            price_book_id,
            product_id,
            unit_price,
            currency,
            min_quantity: 1,
            max_quantity: None,
            valid_from: None,
            valid_to: None,
            status: Status::Active,
        };
        self.repo.create_price_book_entry(pool, entry).await
    }

    pub async fn get_product_price(&self, pool: &SqlitePool, price_book_id: Uuid, product_id: Uuid) -> Result<Option<PriceBookEntry>> {
        self.repo.get_product_price(pool, price_book_id, product_id).await
    }

    pub async fn calculate_price(&self, pool: &SqlitePool, price_book_id: Uuid, product_id: Uuid, quantity: i32) -> Result<i64> {
        if let Some(entry) = self.repo.get_product_price(pool, price_book_id, product_id).await? {
            let tiers = self.repo.get_price_tiers(pool, product_id).await?;
            for tier in tiers {
                if quantity >= tier.min_quantity && (tier.max_quantity.is_none() || quantity <= tier.max_quantity.unwrap()) {
                    return Ok(tier.unit_price * quantity as i64);
                }
            }
            Ok(entry.unit_price * quantity as i64)
        } else {
            Ok(0)
        }
    }

    pub async fn create_discount(&self, pool: &SqlitePool, discount: Discount) -> Result<Discount> {
        self.repo.create_discount(pool, discount).await
    }

    pub async fn get_discount(&self, pool: &SqlitePool, id: Uuid) -> Result<Discount> {
        self.repo.get_discount(pool, id).await
    }

    pub async fn validate_discount(&self, pool: &SqlitePool, code: &str) -> Result<Option<Discount>> {
        if let Some(discount) = self.repo.get_discount_by_code(pool, code).await? {
            let now = chrono::Utc::now();
            let valid_from_ok = discount.valid_from.map_or(true, |v| v <= now);
            let valid_to_ok = discount.valid_to.map_or(true, |v| v >= now);
            let usage_ok = discount.usage_limit.map_or(true, |l| discount.current_usage < l);
            
            if valid_from_ok && valid_to_ok && usage_ok && discount.is_active {
                return Ok(Some(discount));
            }
        }
        Ok(None)
    }

    pub async fn apply_discount(&self, pool: &SqlitePool, discount_id: Uuid, order_value: i64) -> Result<i64> {
        let discount = self.repo.get_discount(pool, discount_id).await?;
        
        if let Some(min) = discount.min_order_value {
            if order_value < min {
                return Ok(0);
            }
        }

        let discount_amount = match discount.discount_type {
            DiscountType::Percentage => (order_value as f64 * discount.value / 100.0) as i64,
            DiscountType::FixedAmount => discount.value as i64,
            _ => 0,
        };

        if let Some(max) = discount.max_discount {
            Ok(discount_amount.min(max))
        } else {
            Ok(discount_amount)
        }
    }

    pub async fn create_coupon(&self, pool: &SqlitePool, code: String, discount_id: Uuid) -> Result<Coupon> {
        let coupon = Coupon {
            base: BaseEntity::new(),
            code,
            discount_id,
            promotion_id: None,
            customer_id: None,
            is_used: false,
            used_at: None,
            order_id: None,
            valid_from: None,
            valid_to: None,
            status: Status::Active,
        };
        self.repo.create_coupon(pool, coupon).await
    }

    pub async fn use_coupon(&self, pool: &SqlitePool, code: &str, order_id: Uuid) -> Result<()> {
        if let Some(coupon) = self.repo.get_coupon(pool, code).await? {
            if !coupon.is_used {
                self.repo.use_coupon(pool, coupon.base.id, order_id).await?;
            }
        }
        Ok(())
    }

    pub async fn create_promotion(&self, pool: &SqlitePool, promo: Promotion) -> Result<Promotion> {
        self.repo.create_promotion(pool, promo).await
    }

    pub async fn get_promotion(&self, pool: &SqlitePool, id: Uuid) -> Result<Promotion> {
        self.repo.get_promotion(pool, id).await
    }

    pub async fn list_promotions(&self, pool: &SqlitePool) -> Result<Vec<Promotion>> {
        self.repo.list_promotions(pool).await
    }

    pub async fn create_price_rule(&self, pool: &SqlitePool, rule: PriceRule) -> Result<PriceRule> {
        self.repo.create_price_rule(pool, rule).await
    }

    pub async fn list_price_rules(&self, pool: &SqlitePool) -> Result<Vec<PriceRule>> {
        self.repo.list_price_rules(pool).await
    }

    pub async fn create_price_tier(&self, pool: &SqlitePool, product_id: Uuid, min_qty: i32, max_qty: Option<i32>, unit_price: i64, currency: String) -> Result<PriceTier> {
        let tier = PriceTier {
            base: BaseEntity::new(),
            price_book_entry_id: None,
            product_id: Some(product_id),
            min_quantity: min_qty,
            max_quantity: max_qty,
            unit_price,
            discount_percent: None,
            currency,
        };
        self.repo.create_price_tier(pool, tier).await
    }

    pub async fn create_customer_price_group(&self, pool: &SqlitePool, name: String, code: String, price_book_id: Option<Uuid>) -> Result<CustomerPriceGroup> {
        let group = CustomerPriceGroup {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            price_book_id,
            discount_id: None,
            status: Status::Active,
        };
        self.repo.create_customer_price_group(pool, group).await
    }

    pub async fn add_customer_to_group(&self, pool: &SqlitePool, group_id: Uuid, customer_id: Uuid) -> Result<CustomerPriceGroupMember> {
        let member = CustomerPriceGroupMember {
            base: BaseEntity::new(),
            group_id,
            customer_id,
            joined_at: chrono::Utc::now(),
        };
        self.repo.add_customer_to_group(pool, member).await
    }
}
