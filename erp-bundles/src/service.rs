use crate::models::*;
use crate::repository::{BundleRepository, SqliteBundleRepository};
use chrono::Utc;
use erp_core::{BaseEntity, Money, Currency, Status};
use sqlx::SqlitePool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    DuplicateError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct BundleService {
    repo: SqliteBundleRepository,
}

impl BundleService {
    pub fn new() -> Self {
        Self {
            repo: SqliteBundleRepository,
        }
    }

    pub async fn create_bundle(&self, pool: &SqlitePool, req: CreateBundleRequest) -> Result<ProductBundle> {
        if self.repo.find_by_code(pool, &req.bundle_code).await?.is_some() {
            return Err(Error::DuplicateError(format!("Bundle with code {} already exists", req.bundle_code)));
        }

        if req.components.is_empty() {
            return Err(Error::ValidationError("Bundle must have at least one component".to_string()));
        }

        let now = Utc::now();
        let base = BaseEntity::new();

        let components: Vec<BundleComponent> = req.components
            .into_iter()
            .enumerate()
            .map(|(idx, c)| BundleComponent {
                id: Uuid::new_v4(),
                bundle_id: base.id,
                product_id: c.product_id,
                product_code: None,
                product_name: None,
                quantity: c.quantity,
                unit_of_measure: c.unit_of_measure,
                is_mandatory: c.is_mandatory,
                sort_order: idx as i32,
                component_price: Money::new(0, req.currency.clone()),
                discount_percent: c.discount_percent,
                can_substitute: c.can_substitute,
                substitute_group_id: None,
                created_at: now,
            })
            .collect();

        let calculated_price = self.calculate_bundle_price(&components, &req.pricing_method, req.list_price_amount, req.discount_percent);

        let bundle = ProductBundle {
            base: base.clone(),
            bundle_code: req.bundle_code,
            name: req.name,
            description: req.description,
            bundle_type: req.bundle_type,
            pricing_method: req.pricing_method,
            list_price: Money::new(req.list_price_amount, req.currency.clone()),
            calculated_price: Money::new(calculated_price, req.currency),
            discount_percent: req.discount_percent,
            components,
            auto_explode: req.auto_explode,
            track_inventory: req.track_inventory,
            availability_date: req.availability_date,
            expiry_date: req.expiry_date,
            max_quantity_per_order: req.max_quantity_per_order,
            status: Status::Active,
        };

        self.repo.create(pool, &bundle).await
    }

    fn calculate_bundle_price(&self, _components: &[BundleComponent], method: &BundlePricingMethod, list_price: i64, discount: Option<f64>) -> i64 {
        match method {
            BundlePricingMethod::FixedPrice => list_price,
            BundlePricingMethod::ComponentSum => {
                let sum: i64 = _components.iter().map(|c| c.component_price.amount * c.quantity).sum();
                sum
            },
            BundlePricingMethod::ComponentSumLessDiscount => {
                let sum: i64 = _components.iter().map(|c| c.component_price.amount * c.quantity).sum();
                let discount_val = discount.unwrap_or(0.0);
                (sum as f64 * (1.0 - discount_val / 100.0)) as i64
            },
            BundlePricingMethod::MarkupOnCost => {
                let cost: i64 = _components.iter().map(|c| c.component_price.amount * c.quantity).sum();
                let markup = discount.unwrap_or(0.0);
                (cost as f64 * (1.0 + markup / 100.0)) as i64
            },
        }
    }

    pub async fn get_bundle(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductBundle> {
        self.repo.find_by_id(pool, id).await?.ok_or_else(|| Error::NotFound(format!("Bundle {} not found", id)))
    }

    pub async fn get_bundle_by_code(&self, pool: &SqlitePool, code: &str) -> Result<ProductBundle> {
        self.repo.find_by_code(pool, code).await?.ok_or_else(|| Error::NotFound(format!("Bundle {} not found", code)))
    }

    pub async fn list_bundles(&self, pool: &SqlitePool, page: u32, per_page: u32, status: Option<Status>) -> Result<BundleListResponse> {
        self.repo.list(pool, page, per_page, status).await
    }

    pub async fn update_bundle(&self, pool: &SqlitePool, id: Uuid, req: UpdateBundleRequest) -> Result<ProductBundle> {
        self.repo.update(pool, id, req).await
    }

    pub async fn delete_bundle(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let _ = self.get_bundle(pool, id).await?;
        self.repo.delete(pool, id).await
    }

    pub async fn add_component(&self, pool: &SqlitePool, bundle_id: Uuid, req: CreateBundleComponentRequest) -> Result<BundleComponent> {
        let _ = self.get_bundle(pool, bundle_id).await?;

        let component = BundleComponent {
            id: Uuid::new_v4(),
            bundle_id,
            product_id: req.product_id,
            product_code: None,
            product_name: None,
            quantity: req.quantity,
            unit_of_measure: req.unit_of_measure,
            is_mandatory: req.is_mandatory,
            sort_order: 999,
            component_price: Money::new(0, Currency::USD),
            discount_percent: req.discount_percent,
            can_substitute: req.can_substitute,
            substitute_group_id: None,
            created_at: Utc::now(),
        };

        self.repo.add_component(pool, &component).await
    }

    pub async fn remove_component(&self, pool: &SqlitePool, bundle_id: Uuid, component_id: Uuid) -> Result<()> {
        self.repo.remove_component(pool, bundle_id, component_id).await
    }

    pub async fn get_availability(&self, pool: &SqlitePool, bundle_id: Uuid) -> Result<BundleAvailability> {
        self.repo.get_availability(pool, bundle_id).await
    }

    pub async fn add_price_rule(&self, pool: &SqlitePool, bundle_id: Uuid, rule: BundlePriceRule) -> Result<BundlePriceRule> {
        let _ = self.get_bundle(pool, bundle_id).await?;
        self.repo.add_price_rule(pool, &rule).await
    }

    pub async fn get_price_rules(&self, pool: &SqlitePool, bundle_id: Uuid) -> Result<Vec<BundlePriceRule>> {
        let _ = self.get_bundle(pool, bundle_id).await?;
        self.repo.get_price_rules(pool, bundle_id).await
    }

    pub async fn record_usage(&self, pool: &SqlitePool, usage: BundleUsage) -> Result<BundleUsage> {
        self.repo.record_usage(pool, &usage).await
    }

    pub async fn get_analytics(&self, pool: &SqlitePool, bundle_id: Uuid, period_start: chrono::DateTime<Utc>, period_end: chrono::DateTime<Utc>) -> Result<BundleAnalytics> {
        self.repo.get_analytics(pool, bundle_id, period_start, period_end).await
    }

    pub async fn calculate_price_for_quantity(&self, pool: &SqlitePool, bundle_id: Uuid, quantity: i64, customer_group_id: Option<Uuid>) -> Result<i64> {
        let bundle = self.get_bundle(pool, bundle_id).await?;
        let rules = self.repo.get_price_rules(pool, bundle_id).await?;
        let now = Utc::now();

        for rule in rules {
            if rule.status != Status::Active {
                continue;
            }

            let quantity_match = quantity >= rule.min_quantity && 
                (rule.max_quantity.is_none() || quantity <= rule.max_quantity.unwrap());

            let date_match = (rule.start_date.is_none() || now >= rule.start_date.unwrap()) &&
                (rule.end_date.is_none() || now <= rule.end_date.unwrap());

            let customer_match = rule.customer_group_id.is_none() || 
                customer_group_id.map(|cg| Some(cg) == rule.customer_group_id).unwrap_or(false);

            if quantity_match && date_match && customer_match {
                if let Some(fixed) = rule.fixed_price {
                    return Ok(fixed);
                }
                if let Some(discount) = rule.discount_percent {
                    return Ok((bundle.list_price.amount as f64 * (1.0 - discount / 100.0)) as i64);
                }
            }
        }

        Ok(bundle.calculated_price.amount)
    }
}

impl Default for BundleService {
    fn default() -> Self {
        Self::new()
    }
}
