use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use uuid::Uuid;

#[async_trait]
pub trait EcommercePlatformRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<EcommercePlatform>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<EcommercePlatform>>;
    async fn create(&self, pool: &SqlitePool, platform: EcommercePlatform) -> Result<EcommercePlatform>;
    async fn update(&self, pool: &SqlitePool, platform: EcommercePlatform) -> Result<EcommercePlatform>;
}

pub struct SqliteEcommercePlatformRepository;

#[async_trait]
impl EcommercePlatformRepository for SqliteEcommercePlatformRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<EcommercePlatform> {
        let row = sqlx::query_as::<_, EcommercePlatformRow>(
            "SELECT * FROM ecommerce_platforms WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("EcommercePlatform", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<EcommercePlatform>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ecommerce_platforms")
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?;
        
        let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
        let rows = sqlx::query_as::<_, EcommercePlatformRow>(
            "SELECT * FROM ecommerce_platforms ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, platform: EcommercePlatform) -> Result<EcommercePlatform> {
        sqlx::query(
            "INSERT INTO ecommerce_platforms (id, name, platform_type, base_url, api_key, api_secret, access_token, webhook_secret, store_id, status, sync_direction, last_sync_at, sync_interval_minutes, auto_sync, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(platform.base.id.to_string())
        .bind(&platform.name)
        .bind(format!("{:?}", platform.platform_type))
        .bind(&platform.base_url)
        .bind(&platform.api_key)
        .bind(&platform.api_secret)
        .bind(&platform.access_token)
        .bind(&platform.webhook_secret)
        .bind(&platform.store_id)
        .bind(format!("{:?}", platform.status))
        .bind(format!("{:?}", platform.sync_direction))
        .bind(platform.last_sync_at.map(|d| d.to_rfc3339()))
        .bind(platform.sync_interval_minutes)
        .bind(platform.auto_sync as i32)
        .bind(platform.base.created_at.to_rfc3339())
        .bind(platform.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(platform)
    }
    
    async fn update(&self, pool: &SqlitePool, platform: EcommercePlatform) -> Result<EcommercePlatform> {
        sqlx::query(
            "UPDATE ecommerce_platforms SET name = ?, status = ?, auto_sync = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&platform.name)
        .bind(format!("{:?}", platform.status))
        .bind(platform.auto_sync as i32)
        .bind(platform.base.updated_at.to_rfc3339())
        .bind(platform.base.id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(platform)
    }
}

#[derive(sqlx::FromRow)]
struct EcommercePlatformRow {
    id: String,
    name: String,
    platform_type: String,
    base_url: String,
    api_key: Option<String>,
    api_secret: Option<String>,
    access_token: Option<String>,
    webhook_secret: Option<String>,
    store_id: Option<String>,
    status: String,
    sync_direction: String,
    last_sync_at: Option<String>,
    sync_interval_minutes: i32,
    auto_sync: i32,
    created_at: String,
    updated_at: String,
}

impl From<EcommercePlatformRow> for EcommercePlatform {
    fn from(r: EcommercePlatformRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            platform_type: match r.platform_type.as_str() {
                "WooCommerce" => PlatformType::WooCommerce,
                "Magento" => PlatformType::Magento,
                "BigCommerce" => PlatformType::BigCommerce,
                "Amazon" => PlatformType::Amazon,
                "EBay" => PlatformType::EBay,
                "Etsy" => PlatformType::Etsy,
                "Walmart" => PlatformType::Walmart,
                "Custom" => PlatformType::Custom,
                _ => PlatformType::Shopify,
            },
            base_url: r.base_url,
            api_key: r.api_key,
            api_secret: r.api_secret,
            access_token: r.access_token,
            webhook_secret: r.webhook_secret,
            store_id: r.store_id,
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            sync_direction: match r.sync_direction.as_str() {
                "Export" => SyncDirection::Export,
                "Bidirectional" => SyncDirection::Bidirectional,
                _ => SyncDirection::Import,
            },
            last_sync_at: r.last_sync_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            sync_interval_minutes: r.sync_interval_minutes,
            auto_sync: r.auto_sync != 0,
        }
    }
}

#[async_trait]
pub trait EcommerceOrderRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<EcommerceOrder>;
    async fn find_by_external_id(&self, pool: &SqlitePool, platform_id: Uuid, external_id: &str) -> Result<EcommerceOrder>;
    async fn find_all(&self, pool: &SqlitePool, platform_id: Option<Uuid>, pagination: Pagination) -> Result<Paginated<EcommerceOrder>>;
    async fn create(&self, pool: &SqlitePool, order: EcommerceOrder) -> Result<EcommerceOrder>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: EcommerceOrderStatus, sync_status: SyncStatus) -> Result<()>;
}

pub struct SqliteEcommerceOrderRepository;

#[async_trait]
impl EcommerceOrderRepository for SqliteEcommerceOrderRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<EcommerceOrder> {
        let row = sqlx::query_as::<_, EcommerceOrderRow>(
            "SELECT * FROM ecommerce_orders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("EcommerceOrder", &id.to_string()))?;
        
        let lines = self.get_lines(pool, id).await?;
        Ok(row.into_order(lines))
    }
    
    async fn find_by_external_id(&self, pool: &SqlitePool, platform_id: Uuid, external_id: &str) -> Result<EcommerceOrder> {
        let row = sqlx::query_as::<_, EcommerceOrderRow>(
            "SELECT * FROM ecommerce_orders WHERE platform_id = ? AND external_order_id = ?"
        )
        .bind(platform_id.to_string())
        .bind(external_id)
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("EcommerceOrder", external_id))?;
        
        let id = Uuid::parse_str(&row.id).unwrap_or_default();
        let lines = self.get_lines(pool, id).await?;
        Ok(row.into_order(lines))
    }
    
    async fn find_all(&self, pool: &SqlitePool, platform_id: Option<Uuid>, pagination: Pagination) -> Result<Paginated<EcommerceOrder>> {
        let (count, rows) = match platform_id {
            Some(pid) => {
                let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ecommerce_orders WHERE platform_id = ?")
                    .bind(pid.to_string())
                    .fetch_one(pool)
                    .await
                    .map_err(Error::Database)?;
                
                let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
                let rows = sqlx::query_as::<_, EcommerceOrderRow>(
                    "SELECT * FROM ecommerce_orders WHERE platform_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(pid.to_string())
                .bind(pagination.per_page as i64)
                .bind(offset as i64)
                .fetch_all(pool)
                .await
                .map_err(Error::Database)?;
                
                (count, rows)
            }
            None => {
                let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ecommerce_orders")
                    .fetch_one(pool)
                    .await
                    .map_err(Error::Database)?;
                
                let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
                let rows = sqlx::query_as::<_, EcommerceOrderRow>(
                    "SELECT * FROM ecommerce_orders ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(pagination.per_page as i64)
                .bind(offset as i64)
                .fetch_all(pool)
                .await
                .map_err(Error::Database)?;
                
                (count, rows)
            }
        };
        
        let mut orders = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = self.get_lines(pool, id).await?;
            orders.push(row.into_order(lines));
        }
        
        Ok(Paginated::new(orders, count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, order: EcommerceOrder) -> Result<EcommerceOrder> {
        sqlx::query(
            "INSERT INTO ecommerce_orders (id, platform_id, external_order_id, order_number, customer_id, external_customer_id, sales_order_id, order_date, status, fulfillment_status, payment_status, subtotal, shipping_amount, tax_amount, discount_amount, total, currency, billing_name, billing_address, billing_city, billing_state, billing_postal_code, billing_country, shipping_name, shipping_address, shipping_city, shipping_state, shipping_postal_code, shipping_country, shipping_method, tracking_number, customer_email, customer_phone, notes, sync_status, imported_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(order.base.id.to_string())
        .bind(order.platform_id.to_string())
        .bind(&order.external_order_id)
        .bind(&order.order_number)
        .bind(order.customer_id.map(|id| id.to_string()))
        .bind(&order.external_customer_id)
        .bind(order.sales_order_id.map(|id| id.to_string()))
        .bind(order.order_date.to_rfc3339())
        .bind(format!("{:?}", order.status))
        .bind(format!("{:?}", order.fulfillment_status))
        .bind(format!("{:?}", order.payment_status))
        .bind(order.subtotal.amount)
        .bind(order.shipping_amount.amount)
        .bind(order.tax_amount.amount)
        .bind(order.discount_amount.amount)
        .bind(order.total.amount)
        .bind(&order.currency)
        .bind(&order.billing_name)
        .bind(&order.billing_address)
        .bind(&order.billing_city)
        .bind(&order.billing_state)
        .bind(&order.billing_postal_code)
        .bind(&order.billing_country)
        .bind(&order.shipping_name)
        .bind(&order.shipping_address)
        .bind(&order.shipping_city)
        .bind(&order.shipping_state)
        .bind(&order.shipping_postal_code)
        .bind(&order.shipping_country)
        .bind(&order.shipping_method)
        .bind(&order.tracking_number)
        .bind(&order.customer_email)
        .bind(&order.customer_phone)
        .bind(&order.notes)
        .bind(format!("{:?}", order.sync_status))
        .bind(order.imported_at.map(|d| d.to_rfc3339()))
        .bind(order.base.created_at.to_rfc3339())
        .bind(order.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        for line in &order.lines {
            self.create_line(pool, line).await?;
        }
        
        Ok(order)
    }
    
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: EcommerceOrderStatus, sync_status: SyncStatus) -> Result<()> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE ecommerce_orders SET status = ?, sync_status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(format!("{:?}", sync_status))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

impl SqliteEcommerceOrderRepository {
    async fn get_lines(&self, pool: &SqlitePool, order_id: Uuid) -> Result<Vec<EcommerceOrderLine>> {
        let rows = sqlx::query_as::<_, EcommerceOrderLineRow>(
            "SELECT * FROM ecommerce_order_lines WHERE order_id = ?"
        )
        .bind(order_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create_line(&self, pool: &SqlitePool, line: &EcommerceOrderLine) -> Result<()> {
        sqlx::query(
            "INSERT INTO ecommerce_order_lines (id, order_id, external_line_id, product_id, listing_id, sku, title, variant_title, quantity, unit_price, tax_amount, discount_amount, line_total, fulfillment_status, quantity_fulfilled)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(line.id.to_string())
        .bind(line.order_id.to_string())
        .bind(&line.external_line_id)
        .bind(line.product_id.map(|id| id.to_string()))
        .bind(line.listing_id.map(|id| id.to_string()))
        .bind(&line.sku)
        .bind(&line.title)
        .bind(&line.variant_title)
        .bind(line.quantity)
        .bind(line.unit_price.amount)
        .bind(line.tax_amount.amount)
        .bind(line.discount_amount.amount)
        .bind(line.line_total.amount)
        .bind(format!("{:?}", line.fulfillment_status))
        .bind(line.quantity_fulfilled)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct EcommerceOrderRow {
    id: String,
    platform_id: String,
    external_order_id: String,
    order_number: String,
    customer_id: Option<String>,
    external_customer_id: Option<String>,
    sales_order_id: Option<String>,
    order_date: String,
    status: String,
    fulfillment_status: String,
    payment_status: String,
    subtotal: i64,
    shipping_amount: i64,
    tax_amount: i64,
    discount_amount: i64,
    total: i64,
    currency: String,
    billing_name: String,
    billing_address: String,
    billing_city: String,
    billing_state: String,
    billing_postal_code: String,
    billing_country: String,
    shipping_name: String,
    shipping_address: String,
    shipping_city: String,
    shipping_state: String,
    shipping_postal_code: String,
    shipping_country: String,
    shipping_method: Option<String>,
    tracking_number: Option<String>,
    customer_email: Option<String>,
    customer_phone: Option<String>,
    notes: Option<String>,
    sync_status: String,
    imported_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl EcommerceOrderRow {
    fn into_order(self, lines: Vec<EcommerceOrderLine>) -> EcommerceOrder {
        EcommerceOrder {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            platform_id: Uuid::parse_str(&self.platform_id).unwrap_or_default(),
            external_order_id: self.external_order_id,
            order_number: self.order_number,
            customer_id: self.customer_id.and_then(|id| Uuid::parse_str(&id).ok()),
            external_customer_id: self.external_customer_id,
            sales_order_id: self.sales_order_id.and_then(|id| Uuid::parse_str(&id).ok()),
            order_date: chrono::DateTime::parse_from_rfc3339(&self.order_date)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            status: match self.status.as_str() {
                "Confirmed" => EcommerceOrderStatus::Confirmed,
                "Processing" => EcommerceOrderStatus::Processing,
                "OnHold" => EcommerceOrderStatus::OnHold,
                "Cancelled" => EcommerceOrderStatus::Cancelled,
                "Refunded" => EcommerceOrderStatus::Refunded,
                "Completed" => EcommerceOrderStatus::Completed,
                _ => EcommerceOrderStatus::Pending,
            },
            fulfillment_status: match self.fulfillment_status.as_str() {
                "PartiallyFulfilled" => FulfillmentStatus::PartiallyFulfilled,
                "Fulfilled" => FulfillmentStatus::Fulfilled,
                "Shipped" => FulfillmentStatus::Shipped,
                "Delivered" => FulfillmentStatus::Delivered,
                "Returned" => FulfillmentStatus::Returned,
                _ => FulfillmentStatus::Unfulfilled,
            },
            payment_status: match self.payment_status.as_str() {
                "Authorized" => PaymentStatus::Authorized,
                "Paid" => PaymentStatus::Paid,
                "PartiallyRefunded" => PaymentStatus::PartiallyRefunded,
                "Refunded" => PaymentStatus::Refunded,
                "Voided" => PaymentStatus::Voided,
                _ => PaymentStatus::Pending,
            },
            subtotal: erp_core::Money::new(self.subtotal, erp_core::Currency::USD),
            shipping_amount: erp_core::Money::new(self.shipping_amount, erp_core::Currency::USD),
            tax_amount: erp_core::Money::new(self.tax_amount, erp_core::Currency::USD),
            discount_amount: erp_core::Money::new(self.discount_amount, erp_core::Currency::USD),
            total: erp_core::Money::new(self.total, erp_core::Currency::USD),
            currency: self.currency,
            billing_name: self.billing_name,
            billing_address: self.billing_address,
            billing_city: self.billing_city,
            billing_state: self.billing_state,
            billing_postal_code: self.billing_postal_code,
            billing_country: self.billing_country,
            shipping_name: self.shipping_name,
            shipping_address: self.shipping_address,
            shipping_city: self.shipping_city,
            shipping_state: self.shipping_state,
            shipping_postal_code: self.shipping_postal_code,
            shipping_country: self.shipping_country,
            shipping_method: self.shipping_method,
            tracking_number: self.tracking_number,
            customer_email: self.customer_email,
            customer_phone: self.customer_phone,
            notes: self.notes,
            lines,
            sync_status: match self.sync_status.as_str() {
                "Syncing" => SyncStatus::Syncing,
                "Synced" => SyncStatus::Synced,
                "Failed" => SyncStatus::Failed,
                "Conflict" => SyncStatus::Conflict,
                _ => SyncStatus::Pending,
            },
            imported_at: self.imported_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
        }
    }
}

#[derive(sqlx::FromRow)]
struct EcommerceOrderLineRow {
    id: String,
    order_id: String,
    external_line_id: Option<String>,
    product_id: Option<String>,
    listing_id: Option<String>,
    sku: Option<String>,
    title: String,
    variant_title: Option<String>,
    quantity: i64,
    unit_price: i64,
    tax_amount: i64,
    discount_amount: i64,
    line_total: i64,
    fulfillment_status: String,
    quantity_fulfilled: i64,
}

impl From<EcommerceOrderLineRow> for EcommerceOrderLine {
    fn from(r: EcommerceOrderLineRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            order_id: Uuid::parse_str(&r.order_id).unwrap_or_default(),
            external_line_id: r.external_line_id,
            product_id: r.product_id.and_then(|id| Uuid::parse_str(&id).ok()),
            listing_id: r.listing_id.and_then(|id| Uuid::parse_str(&id).ok()),
            sku: r.sku,
            title: r.title,
            variant_title: r.variant_title,
            quantity: r.quantity,
            unit_price: erp_core::Money::new(r.unit_price, erp_core::Currency::USD),
            tax_amount: erp_core::Money::new(r.tax_amount, erp_core::Currency::USD),
            discount_amount: erp_core::Money::new(r.discount_amount, erp_core::Currency::USD),
            line_total: erp_core::Money::new(r.line_total, erp_core::Currency::USD),
            fulfillment_status: match r.fulfillment_status.as_str() {
                "PartiallyFulfilled" => FulfillmentStatus::PartiallyFulfilled,
                "Fulfilled" => FulfillmentStatus::Fulfilled,
                "Shipped" => FulfillmentStatus::Shipped,
                "Delivered" => FulfillmentStatus::Delivered,
                "Returned" => FulfillmentStatus::Returned,
                _ => FulfillmentStatus::Unfulfilled,
            },
            quantity_fulfilled: r.quantity_fulfilled,
        }
    }
}

#[async_trait]
pub trait ProductListingRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductListing>;
    async fn find_by_product(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<ProductListing>>;
    async fn create(&self, pool: &SqlitePool, listing: ProductListing) -> Result<ProductListing>;
    async fn update(&self, pool: &SqlitePool, listing: ProductListing) -> Result<ProductListing>;
}

pub struct SqliteProductListingRepository;

#[async_trait]
impl ProductListingRepository for SqliteProductListingRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductListing> {
        let row = sqlx::query_as::<_, ProductListingRow>(
            "SELECT * FROM product_listings WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("ProductListing", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_product(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<ProductListing>> {
        let rows = sqlx::query_as::<_, ProductListingRow>(
            "SELECT * FROM product_listings WHERE product_id = ?"
        )
        .bind(product_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create(&self, pool: &SqlitePool, listing: ProductListing) -> Result<ProductListing> {
        sqlx::query(
            "INSERT INTO product_listings (id, platform_id, product_id, external_product_id, external_variant_id, title, description, price, compare_at_price, quantity, sku, barcode, status, visibility, seo_title, seo_description, tags, category, images, sync_status, last_sync_at, sync_error, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(listing.base.id.to_string())
        .bind(listing.platform_id.to_string())
        .bind(listing.product_id.to_string())
        .bind(&listing.external_product_id)
        .bind(&listing.external_variant_id)
        .bind(&listing.title)
        .bind(&listing.description)
        .bind(listing.price.amount)
        .bind(listing.compare_at_price.as_ref().map(|p| p.amount))
        .bind(listing.quantity)
        .bind(&listing.sku)
        .bind(&listing.barcode)
        .bind(format!("{:?}", listing.status))
        .bind(format!("{:?}", listing.visibility))
        .bind(&listing.seo_title)
        .bind(&listing.seo_description)
        .bind(&listing.tags)
        .bind(&listing.category)
        .bind(&listing.images)
        .bind(format!("{:?}", listing.sync_status))
        .bind(listing.last_sync_at.as_ref().map(|d| d.to_rfc3339()))
        .bind(&listing.sync_error)
        .bind(listing.base.created_at.to_rfc3339())
        .bind(listing.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(listing)
    }
    
    async fn update(&self, pool: &SqlitePool, listing: ProductListing) -> Result<ProductListing> {
        sqlx::query(
            "UPDATE product_listings SET title = ?, price = ?, quantity = ?, status = ?, sync_status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&listing.title)
        .bind(listing.price.amount)
        .bind(listing.quantity)
        .bind(format!("{:?}", listing.status))
        .bind(format!("{:?}", listing.sync_status))
        .bind(listing.base.updated_at.to_rfc3339())
        .bind(listing.base.id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(listing)
    }
}

#[derive(sqlx::FromRow)]
struct ProductListingRow {
    id: String,
    platform_id: String,
    product_id: String,
    external_product_id: String,
    external_variant_id: Option<String>,
    title: String,
    description: Option<String>,
    price: i64,
    compare_at_price: Option<i64>,
    quantity: i64,
    sku: Option<String>,
    barcode: Option<String>,
    status: String,
    visibility: String,
    seo_title: Option<String>,
    seo_description: Option<String>,
    tags: Option<String>,
    category: Option<String>,
    images: Option<String>,
    sync_status: String,
    last_sync_at: Option<String>,
    sync_error: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ProductListingRow> for ProductListing {
    fn from(r: ProductListingRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            platform_id: Uuid::parse_str(&r.platform_id).unwrap_or_default(),
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            external_product_id: r.external_product_id,
            external_variant_id: r.external_variant_id,
            title: r.title,
            description: r.description,
            price: erp_core::Money::new(r.price, erp_core::Currency::USD),
            compare_at_price: r.compare_at_price.map(|p| erp_core::Money::new(p, erp_core::Currency::USD)),
            quantity: r.quantity,
            sku: r.sku,
            barcode: r.barcode,
            status: match r.status.as_str() {
                "Draft" => ProductListingStatus::Draft,
                "Archived" => ProductListingStatus::Archived,
                "Pending" => ProductListingStatus::Pending,
                _ => ProductListingStatus::Active,
            },
            visibility: match r.visibility.as_str() {
                "Hidden" => ProductVisibility::Hidden,
                "SearchOnly" => ProductVisibility::SearchOnly,
                _ => ProductVisibility::Visible,
            },
            seo_title: r.seo_title,
            seo_description: r.seo_description,
            tags: r.tags,
            category: r.category,
            images: r.images,
            sync_status: match r.sync_status.as_str() {
                "Syncing" => SyncStatus::Syncing,
                "Synced" => SyncStatus::Synced,
                "Failed" => SyncStatus::Failed,
                "Conflict" => SyncStatus::Conflict,
                _ => SyncStatus::Pending,
            },
            last_sync_at: r.last_sync_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            sync_error: r.sync_error,
        }
    }
}
