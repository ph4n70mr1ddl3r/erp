use crate::models::*;
use async_trait::async_trait;
use chrono::Utc;
use erp_core::{Currency, Status, Money};
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait BundleRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, bundle: &ProductBundle) -> crate::service::Result<ProductBundle>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> crate::service::Result<Option<ProductBundle>>;
    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> crate::service::Result<Option<ProductBundle>>;
    async fn list(&self, pool: &SqlitePool, page: u32, per_page: u32, status: Option<Status>) -> crate::service::Result<BundleListResponse>;
    async fn update(&self, pool: &SqlitePool, id: Uuid, req: UpdateBundleRequest) -> crate::service::Result<ProductBundle>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> crate::service::Result<()>;
    async fn add_component(&self, pool: &SqlitePool, component: &BundleComponent) -> crate::service::Result<BundleComponent>;
    async fn remove_component(&self, pool: &SqlitePool, bundle_id: Uuid, component_id: Uuid) -> crate::service::Result<()>;
    async fn get_components(&self, pool: &SqlitePool, bundle_id: Uuid) -> crate::service::Result<Vec<BundleComponent>>;
    async fn get_availability(&self, pool: &SqlitePool, bundle_id: Uuid) -> crate::service::Result<BundleAvailability>;
    async fn add_price_rule(&self, pool: &SqlitePool, rule: &BundlePriceRule) -> crate::service::Result<BundlePriceRule>;
    async fn get_price_rules(&self, pool: &SqlitePool, bundle_id: Uuid) -> crate::service::Result<Vec<BundlePriceRule>>;
    async fn record_usage(&self, pool: &SqlitePool, usage: &BundleUsage) -> crate::service::Result<BundleUsage>;
    async fn get_analytics(&self, pool: &SqlitePool, bundle_id: Uuid, period_start: chrono::DateTime<Utc>, period_end: chrono::DateTime<Utc>) -> crate::service::Result<BundleAnalytics>;
}

pub struct SqliteBundleRepository;

#[async_trait]
impl BundleRepository for SqliteBundleRepository {
    async fn create(&self, pool: &SqlitePool, bundle: &ProductBundle) -> crate::service::Result<ProductBundle> {
        let now = Utc::now();
        let currency_str = format!("{:?}", bundle.list_price.currency);
        let calc_currency_str = format!("{:?}", bundle.calculated_price.currency);
        
        sqlx::query(
            r#"INSERT INTO product_bundles (id, bundle_code, name, description, bundle_type, pricing_method, list_price_amount, list_price_currency, calculated_price_amount, calculated_price_currency, discount_percent, auto_explode, track_inventory, availability_date, expiry_date, max_quantity_per_order, status, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)"#,
        )
        .bind(bundle.base.id.to_string())
        .bind(&bundle.bundle_code)
        .bind(&bundle.name)
        .bind(&bundle.description)
        .bind(format!("{:?}", bundle.bundle_type))
        .bind(format!("{:?}", bundle.pricing_method))
        .bind(bundle.list_price.amount)
        .bind(&currency_str)
        .bind(bundle.calculated_price.amount)
        .bind(&calc_currency_str)
        .bind(bundle.discount_percent)
        .bind(bundle.auto_explode)
        .bind(bundle.track_inventory)
        .bind(bundle.availability_date.map(|d| d.to_rfc3339()))
        .bind(bundle.expiry_date.map(|d| d.to_rfc3339()))
        .bind(bundle.max_quantity_per_order)
        .bind(format!("{:?}", bundle.status))
        .bind(bundle.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool).await?;

        for component in &bundle.components {
            let comp_currency_str = format!("{:?}", component.component_price.currency);
            sqlx::query(
                r#"INSERT INTO bundle_components (id, bundle_id, product_id, quantity, unit_of_measure, is_mandatory, sort_order, component_price_amount, component_price_currency, discount_percent, can_substitute, substitute_group_id, created_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"#,
            )
            .bind(component.id.to_string())
            .bind(bundle.base.id.to_string())
            .bind(component.product_id.to_string())
            .bind(component.quantity)
            .bind(&component.unit_of_measure)
            .bind(component.is_mandatory)
            .bind(component.sort_order)
            .bind(component.component_price.amount)
            .bind(&comp_currency_str)
            .bind(component.discount_percent)
            .bind(component.can_substitute)
            .bind(component.substitute_group_id.map(|id| id.to_string()))
            .bind(component.created_at.to_rfc3339())
            .execute(pool).await?;
        }

        Ok(bundle.clone())
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> crate::service::Result<Option<ProductBundle>> {
        let row = sqlx::query(
            r#"SELECT id, bundle_code, name, description, bundle_type, pricing_method, list_price_amount, list_price_currency, calculated_price_amount, calculated_price_currency, discount_percent, auto_explode, track_inventory, availability_date, expiry_date, max_quantity_per_order, status, created_at, updated_at FROM product_bundles WHERE id = ?1"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;

        if let Some(r) = row {
            let components = self.get_components(pool, id).await?;
            let row_map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(&r).unwrap_or_default()).unwrap_or_default();
            
            let bundle = parse_bundle_from_row(&row_map, components)?;
            Ok(Some(bundle))
        } else {
            Ok(None)
        }
    }

    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> crate::service::Result<Option<ProductBundle>> {
        let row = sqlx::query(
            r#"SELECT id, bundle_code, name, description, bundle_type, pricing_method, list_price_amount, list_price_currency, calculated_price_amount, calculated_price_currency, discount_percent, auto_explode, track_inventory, availability_date, expiry_date, max_quantity_per_order, status, created_at, updated_at FROM product_bundles WHERE bundle_code = ?1"#
        )
        .bind(code)
        .fetch_optional(pool).await?;

        if let Some(r) = row {
            let row_map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(&r).unwrap_or_default()).unwrap_or_default();
            let id_str = row_map.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let id = Uuid::parse_str(id_str).unwrap_or(Uuid::nil());
            let components = self.get_components(pool, id).await?;
            let bundle = parse_bundle_from_row(&row_map, components)?;
            Ok(Some(bundle))
        } else {
            Ok(None)
        }
    }

    async fn list(&self, pool: &SqlitePool, page: u32, per_page: u32, status: Option<Status>) -> crate::service::Result<BundleListResponse> {
        let offset = (page.saturating_sub(1)) * per_page;
        let status_filter = status.map(|s| format!("{:?}", s));

        let count_result = sqlx::query(
            r#"SELECT COUNT(*) as count FROM product_bundles WHERE (?1 IS NULL OR status = ?1)"#
        )
        .bind(&status_filter)
        .fetch_one(pool).await?;

        let count_map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(&count_result).unwrap_or_default()).unwrap_or_default();
        let total: i64 = count_map.get("count").and_then(|v| v.as_i64()).unwrap_or(0);

        let rows = sqlx::query(
            r#"SELECT id, bundle_code, name, bundle_type, list_price_amount, list_price_currency, status, created_at FROM product_bundles WHERE (?1 IS NULL OR status = ?1) ORDER BY created_at DESC LIMIT ?2 OFFSET ?3"#
        )
        .bind(&status_filter)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool).await?;

        let items = rows.iter().filter_map(|r| {
            let row_map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(r).ok()?).ok()?;
            parse_bundle_summary(&row_map)
        }).collect();

        Ok(BundleListResponse {
            items,
            total,
            page,
            per_page,
        })
    }

    async fn update(&self, pool: &SqlitePool, id: Uuid, req: UpdateBundleRequest) -> crate::service::Result<ProductBundle> {
        let now = Utc::now();
        let existing = self.find_by_id(pool, id).await?.ok_or_else(|| crate::service::Error::NotFound(format!("Bundle {} not found", id)))?;

        let name = req.name.unwrap_or(existing.name);
        let description = req.description.or(existing.description);
        let bundle_type = req.bundle_type.unwrap_or(existing.bundle_type);
        let pricing_method = req.pricing_method.unwrap_or(existing.pricing_method);
        let list_price_amount = req.list_price_amount.unwrap_or(existing.list_price.amount);
        let discount_percent = req.discount_percent.or(existing.discount_percent);
        let auto_explode = req.auto_explode.unwrap_or(existing.auto_explode);
        let track_inventory = req.track_inventory.unwrap_or(existing.track_inventory);
        let availability_date = req.availability_date.or(existing.availability_date);
        let expiry_date = req.expiry_date.or(existing.expiry_date);
        let max_quantity_per_order = req.max_quantity_per_order.or(existing.max_quantity_per_order);
        let status = req.status.unwrap_or(existing.status);

        sqlx::query(
            r#"UPDATE product_bundles SET name = ?1, description = ?2, bundle_type = ?3, pricing_method = ?4, list_price_amount = ?5, discount_percent = ?6, auto_explode = ?7, track_inventory = ?8, availability_date = ?9, expiry_date = ?10, max_quantity_per_order = ?11, status = ?12, updated_at = ?13 WHERE id = ?14"#,
        )
        .bind(&name)
        .bind(&description)
        .bind(format!("{:?}", bundle_type))
        .bind(format!("{:?}", pricing_method))
        .bind(list_price_amount)
        .bind(discount_percent)
        .bind(auto_explode)
        .bind(track_inventory)
        .bind(availability_date.map(|d| d.to_rfc3339()))
        .bind(expiry_date.map(|d| d.to_rfc3339()))
        .bind(max_quantity_per_order)
        .bind(format!("{:?}", status))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool).await?;

        self.find_by_id(pool, id).await?.ok_or_else(|| crate::service::Error::NotFound(format!("Bundle {} not found", id)))
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> crate::service::Result<()> {
        sqlx::query(r#"DELETE FROM bundle_components WHERE bundle_id = ?1"#)
            .bind(id.to_string())
            .execute(pool).await?;
        sqlx::query(r#"DELETE FROM bundle_price_rules WHERE bundle_id = ?1"#)
            .bind(id.to_string())
            .execute(pool).await?;
        sqlx::query(r#"DELETE FROM product_bundles WHERE id = ?1"#)
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn add_component(&self, pool: &SqlitePool, component: &BundleComponent) -> crate::service::Result<BundleComponent> {
        let comp_currency_str = format!("{:?}", component.component_price.currency);
        sqlx::query(
            r#"INSERT INTO bundle_components (id, bundle_id, product_id, quantity, unit_of_measure, is_mandatory, sort_order, component_price_amount, component_price_currency, discount_percent, can_substitute, substitute_group_id, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"#,
        )
        .bind(component.id.to_string())
        .bind(component.bundle_id.to_string())
        .bind(component.product_id.to_string())
        .bind(component.quantity)
        .bind(&component.unit_of_measure)
        .bind(component.is_mandatory)
        .bind(component.sort_order)
        .bind(component.component_price.amount)
        .bind(&comp_currency_str)
        .bind(component.discount_percent)
        .bind(component.can_substitute)
        .bind(component.substitute_group_id.map(|id| id.to_string()))
        .bind(component.created_at.to_rfc3339())
        .execute(pool).await?;

        Ok(component.clone())
    }

    async fn remove_component(&self, pool: &SqlitePool, bundle_id: Uuid, component_id: Uuid) -> crate::service::Result<()> {
        sqlx::query(r#"DELETE FROM bundle_components WHERE bundle_id = ?1 AND id = ?2"#)
            .bind(bundle_id.to_string())
            .bind(component_id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn get_components(&self, pool: &SqlitePool, bundle_id: Uuid) -> crate::service::Result<Vec<BundleComponent>> {
        let rows = sqlx::query(
            r#"SELECT id, bundle_id, product_id, quantity, unit_of_measure, is_mandatory, sort_order, component_price_amount, component_price_currency, discount_percent, can_substitute, substitute_group_id, created_at FROM bundle_components WHERE bundle_id = ?1 ORDER BY sort_order"#
        )
        .bind(bundle_id.to_string())
        .fetch_all(pool).await?;

        let components = rows.iter().filter_map(|r| {
            let row_map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(r).ok()?).ok()?;
            parse_component(&row_map)
        }).collect();

        Ok(components)
    }

    async fn get_availability(&self, pool: &SqlitePool, bundle_id: Uuid) -> crate::service::Result<BundleAvailability> {
        let bundle = self.find_by_id(pool, bundle_id).await?.ok_or_else(|| crate::service::Error::NotFound(format!("Bundle {} not found", bundle_id)))?;

        let mut total_available: i64 = i64::MAX;
        let mut component_shortages = Vec::new();

        for component in &bundle.components {
            let stock_result = sqlx::query(
                r#"SELECT COALESCE(SUM(available_quantity), 0) as available FROM stock_levels WHERE product_id = ?1"#
            )
            .bind(component.product_id.to_string())
            .fetch_one(pool).await?;

            let stock_map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(&stock_result).unwrap_or_default()).unwrap_or_default();
            let available: i64 = stock_map.get("available").and_then(|v| v.as_i64()).unwrap_or(0);
            let bundle_qty = available / component.quantity.max(1);
            total_available = total_available.min(bundle_qty);

            if available < component.quantity {
                component_shortages.push(ComponentShortage {
                    product_id: component.product_id,
                    product_code: component.product_code.clone().unwrap_or_default(),
                    product_name: component.product_name.clone().unwrap_or_default(),
                    required_quantity: component.quantity,
                    available_quantity: available,
                    shortage_quantity: component.quantity - available,
                });
            }
        }

        if total_available == i64::MAX {
            total_available = 0;
        }

        Ok(BundleAvailability {
            bundle_id,
            bundle_code: bundle.bundle_code,
            bundle_name: bundle.name,
            total_available,
            warehouse_availability: vec![],
            component_shortages,
            can_fulfill: total_available > 0,
            earliest_available_date: None,
        })
    }

    async fn add_price_rule(&self, pool: &SqlitePool, rule: &BundlePriceRule) -> crate::service::Result<BundlePriceRule> {
        sqlx::query(
            r#"INSERT INTO bundle_price_rules (id, bundle_id, rule_name, rule_type, min_quantity, max_quantity, discount_percent, fixed_price, start_date, end_date, customer_group_id, priority, status, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)"#,
        )
        .bind(rule.id.to_string())
        .bind(rule.bundle_id.to_string())
        .bind(&rule.rule_name)
        .bind(format!("{:?}", rule.rule_type))
        .bind(rule.min_quantity)
        .bind(rule.max_quantity)
        .bind(rule.discount_percent)
        .bind(rule.fixed_price)
        .bind(rule.start_date.map(|d| d.to_rfc3339()))
        .bind(rule.end_date.map(|d| d.to_rfc3339()))
        .bind(rule.customer_group_id.map(|id| id.to_string()))
        .bind(rule.priority)
        .bind(format!("{:?}", rule.status))
        .bind(rule.created_at.to_rfc3339())
        .execute(pool).await?;

        Ok(rule.clone())
    }

    async fn get_price_rules(&self, pool: &SqlitePool, bundle_id: Uuid) -> crate::service::Result<Vec<BundlePriceRule>> {
        let rows = sqlx::query(
            r#"SELECT id, bundle_id, rule_name, rule_type, min_quantity, max_quantity, discount_percent, fixed_price, start_date, end_date, customer_group_id, priority, status, created_at FROM bundle_price_rules WHERE bundle_id = ?1 ORDER BY priority"#
        )
        .bind(bundle_id.to_string())
        .fetch_all(pool).await?;

        let rules = rows.iter().filter_map(|r| {
            let row_map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(r).ok()?).ok()?;
            parse_price_rule(&row_map)
        }).collect();

        Ok(rules)
    }

    async fn record_usage(&self, pool: &SqlitePool, usage: &BundleUsage) -> crate::service::Result<BundleUsage> {
        sqlx::query(
            r#"INSERT INTO bundle_usage (id, bundle_id, order_id, order_line_id, invoice_id, customer_id, quantity, unit_price, total_price, margin_amount, margin_percent, usage_date, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"#,
        )
        .bind(usage.id.to_string())
        .bind(usage.bundle_id.to_string())
        .bind(usage.order_id.map(|id| id.to_string()))
        .bind(usage.order_line_id.map(|id| id.to_string()))
        .bind(usage.invoice_id.map(|id| id.to_string()))
        .bind(usage.customer_id.map(|id| id.to_string()))
        .bind(usage.quantity)
        .bind(usage.unit_price)
        .bind(usage.total_price)
        .bind(usage.margin_amount)
        .bind(usage.margin_percent)
        .bind(usage.usage_date.to_rfc3339())
        .bind(usage.created_at.to_rfc3339())
        .execute(pool).await?;

        Ok(usage.clone())
    }

    async fn get_analytics(&self, pool: &SqlitePool, bundle_id: Uuid, period_start: chrono::DateTime<Utc>, period_end: chrono::DateTime<Utc>) -> crate::service::Result<BundleAnalytics> {
        let bundle = self.find_by_id(pool, bundle_id).await?.ok_or_else(|| crate::service::Error::NotFound(format!("Bundle {} not found", bundle_id)))?;

        let result = sqlx::query(
            r#"SELECT 
                COALESCE(SUM(quantity), 0) as total_sold,
                COALESCE(SUM(total_price), 0) as total_revenue,
                COALESCE(SUM(margin_amount), 0) as total_margin,
                COUNT(DISTINCT order_id) as order_count,
                COUNT(DISTINCT customer_id) as customer_count,
                AVG(margin_percent) as avg_margin_percent
            FROM bundle_usage 
            WHERE bundle_id = ?1 AND usage_date >= ?2 AND usage_date <= ?3"#
        )
        .bind(bundle_id.to_string())
        .bind(period_start.to_rfc3339())
        .bind(period_end.to_rfc3339())
        .fetch_one(pool).await?;

        let result_map: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(serde_json::to_value(&result).unwrap_or_default()).unwrap_or_default();
        
        let total_sold: i64 = result_map.get("total_sold").and_then(|v| v.as_i64()).unwrap_or(0);
        let total_revenue: i64 = result_map.get("total_revenue").and_then(|v| v.as_i64()).unwrap_or(0);
        let total_margin: i64 = result_map.get("total_margin").and_then(|v| v.as_i64()).unwrap_or(0);
        let margin_percent = if total_revenue > 0 { (total_margin as f64 / total_revenue as f64) * 100.0 } else { 0.0 };
        let order_count: i32 = result_map.get("order_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let customer_count: i32 = result_map.get("customer_count").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

        Ok(BundleAnalytics {
            bundle_id,
            bundle_code: bundle.bundle_code,
            bundle_name: bundle.name,
            period_start,
            period_end,
            total_sold,
            total_revenue,
            total_margin,
            margin_percent,
            avg_discount_percent: 0.0,
            order_count,
            customer_count,
            top_warehouses: vec![],
        })
    }
}

fn parse_bundle_from_row(row_map: &std::collections::HashMap<String, serde_json::Value>, components: Vec<BundleComponent>) -> crate::service::Result<ProductBundle> {
    let id_str = row_map.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let id = Uuid::parse_str(id_str).unwrap_or(Uuid::nil());
    let currency_str = row_map.get("list_price_currency").and_then(|v| v.as_str()).unwrap_or("USD");
    let currency: Currency = currency_str.parse().unwrap_or(Currency::USD);
    let calc_currency_str = row_map.get("calculated_price_currency").and_then(|v| v.as_str()).unwrap_or("USD");
    let calc_currency: Currency = calc_currency_str.parse().unwrap_or(Currency::USD);
    let status_str = row_map.get("status").and_then(|v| v.as_str()).unwrap_or("Active");
    let status: Status = status_str.parse().unwrap_or(Status::Active);
    let bundle_type_str = row_map.get("bundle_type").and_then(|v| v.as_str()).unwrap_or("SalesKit");
    let bundle_type: BundleType = bundle_type_str.parse().unwrap_or(BundleType::SalesKit);
    let pricing_method_str = row_map.get("pricing_method").and_then(|v| v.as_str()).unwrap_or("FixedPrice");
    let pricing_method: BundlePricingMethod = pricing_method_str.parse().unwrap_or(BundlePricingMethod::FixedPrice);
    let created_at_str = row_map.get("created_at").and_then(|v| v.as_str()).unwrap_or("");
    let updated_at_str = row_map.get("updated_at").and_then(|v| v.as_str()).unwrap_or("");
    
    Ok(ProductBundle {
        base: erp_core::BaseEntity {
            id,
            created_at: chrono::DateTime::parse_from_rfc3339(created_at_str).map(|d| d.with_timezone(&Utc)).unwrap_or(Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(updated_at_str).map(|d| d.with_timezone(&Utc)).unwrap_or(Utc::now()),
            created_by: None,
            updated_by: None,
        },
        bundle_code: row_map.get("bundle_code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        name: row_map.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        description: row_map.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
        bundle_type,
        pricing_method,
        list_price: Money::new(row_map.get("list_price_amount").and_then(|v| v.as_i64()).unwrap_or(0), currency),
        calculated_price: Money::new(row_map.get("calculated_price_amount").and_then(|v| v.as_i64()).unwrap_or(0), calc_currency),
        discount_percent: row_map.get("discount_percent").and_then(|v| v.as_f64()),
        components,
        auto_explode: row_map.get("auto_explode").and_then(|v| v.as_bool()).unwrap_or(false),
        track_inventory: row_map.get("track_inventory").and_then(|v| v.as_bool()).unwrap_or(true),
        availability_date: row_map.get("availability_date").and_then(|v| v.as_str()).and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&Utc))),
        expiry_date: row_map.get("expiry_date").and_then(|v| v.as_str()).and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&Utc))),
        max_quantity_per_order: row_map.get("max_quantity_per_order").and_then(|v| v.as_i64()),
        status,
    })
}

fn parse_bundle_summary(row_map: &std::collections::HashMap<String, serde_json::Value>) -> Option<ProductBundleSummary> {
    let id_str = row_map.get("id").and_then(|v| v.as_str())?;
    let id = Uuid::parse_str(id_str).ok()?;
    let currency_str = row_map.get("list_price_currency").and_then(|v| v.as_str()).unwrap_or("USD");
    let currency: Currency = currency_str.parse().ok()?;
    let status_str = row_map.get("status").and_then(|v| v.as_str()).unwrap_or("Active");
    let status: Status = status_str.parse().ok()?;
    let bundle_type_str = row_map.get("bundle_type").and_then(|v| v.as_str()).unwrap_or("SalesKit");
    let bundle_type: BundleType = bundle_type_str.parse().ok()?;
    let created_at_str = row_map.get("created_at").and_then(|v| v.as_str()).unwrap_or("");
    
    Some(ProductBundleSummary {
        id,
        bundle_code: row_map.get("bundle_code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        name: row_map.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        bundle_type,
        list_price: Money::new(row_map.get("list_price_amount").and_then(|v| v.as_i64()).unwrap_or(0), currency),
        component_count: 0,
        status,
        created_at: chrono::DateTime::parse_from_rfc3339(created_at_str).map(|d| d.with_timezone(&Utc)).unwrap_or(Utc::now()),
    })
}

fn parse_component(row_map: &std::collections::HashMap<String, serde_json::Value>) -> Option<BundleComponent> {
    let id_str = row_map.get("id").and_then(|v| v.as_str())?;
    let id = Uuid::parse_str(id_str).ok()?;
    let bundle_id_str = row_map.get("bundle_id").and_then(|v| v.as_str())?;
    let bundle_id = Uuid::parse_str(bundle_id_str).ok()?;
    let product_id_str = row_map.get("product_id").and_then(|v| v.as_str())?;
    let product_id = Uuid::parse_str(product_id_str).ok()?;
    let currency_str = row_map.get("component_price_currency").and_then(|v| v.as_str()).unwrap_or("USD");
    let currency: Currency = currency_str.parse().ok()?;
    let created_at_str = row_map.get("created_at").and_then(|v| v.as_str()).unwrap_or("");

    Some(BundleComponent {
        id,
        bundle_id,
        product_id,
        product_code: None,
        product_name: None,
        quantity: row_map.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1),
        unit_of_measure: row_map.get("unit_of_measure").and_then(|v| v.as_str()).unwrap_or("PCS").to_string(),
        is_mandatory: row_map.get("is_mandatory").and_then(|v| v.as_bool()).unwrap_or(true),
        sort_order: row_map.get("sort_order").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
        component_price: Money::new(row_map.get("component_price_amount").and_then(|v| v.as_i64()).unwrap_or(0), currency),
        discount_percent: row_map.get("discount_percent").and_then(|v| v.as_f64()),
        can_substitute: row_map.get("can_substitute").and_then(|v| v.as_bool()).unwrap_or(false),
        substitute_group_id: row_map.get("substitute_group_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()),
        created_at: chrono::DateTime::parse_from_rfc3339(created_at_str).map(|d| d.with_timezone(&Utc)).unwrap_or(Utc::now()),
    })
}

fn parse_price_rule(row_map: &std::collections::HashMap<String, serde_json::Value>) -> Option<BundlePriceRule> {
    let id_str = row_map.get("id").and_then(|v| v.as_str())?;
    let id = Uuid::parse_str(id_str).ok()?;
    let bundle_id_str = row_map.get("bundle_id").and_then(|v| v.as_str())?;
    let bundle_id = Uuid::parse_str(bundle_id_str).ok()?;
    let rule_type_str = row_map.get("rule_type").and_then(|v| v.as_str()).unwrap_or("QuantityBreak");
    let rule_type: BundlePriceRuleType = rule_type_str.parse().ok()?;
    let status_str = row_map.get("status").and_then(|v| v.as_str()).unwrap_or("Active");
    let status: Status = status_str.parse().ok()?;
    let created_at_str = row_map.get("created_at").and_then(|v| v.as_str()).unwrap_or("");

    Some(BundlePriceRule {
        id,
        bundle_id,
        rule_name: row_map.get("rule_name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        rule_type,
        min_quantity: row_map.get("min_quantity").and_then(|v| v.as_i64()).unwrap_or(1),
        max_quantity: row_map.get("max_quantity").and_then(|v| v.as_i64()),
        discount_percent: row_map.get("discount_percent").and_then(|v| v.as_f64()),
        fixed_price: row_map.get("fixed_price").and_then(|v| v.as_i64()),
        start_date: row_map.get("start_date").and_then(|v| v.as_str()).and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&Utc))),
        end_date: row_map.get("end_date").and_then(|v| v.as_str()).and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&Utc))),
        customer_group_id: row_map.get("customer_group_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()),
        priority: row_map.get("priority").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
        status,
        created_at: chrono::DateTime::parse_from_rfc3339(created_at_str).map(|d| d.with_timezone(&Utc)).unwrap_or(Utc::now()),
    })
}
