use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;

pub struct SqliteDropShipOrderRepository;

#[async_trait]
pub trait DropShipOrderRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<DropShipOrder>;
    async fn find_by_order_number(&self, pool: &SqlitePool, order_number: &str) -> Result<DropShipOrder>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<DropShipOrder>>;
    async fn find_by_vendor(&self, pool: &SqlitePool, vendor_id: Uuid, pagination: Pagination) -> Result<Paginated<DropShipOrder>>;
    async fn find_by_customer(&self, pool: &SqlitePool, customer_id: Uuid, pagination: Pagination) -> Result<Paginated<DropShipOrder>>;
    async fn find_by_status(&self, pool: &SqlitePool, status: DropShipOrderStatus) -> Result<Vec<DropShipOrder>>;
    async fn create(&self, pool: &SqlitePool, order: DropShipOrder) -> Result<DropShipOrder>;
    async fn update(&self, pool: &SqlitePool, order: DropShipOrder) -> Result<DropShipOrder>;
}

#[async_trait]
impl DropShipOrderRepository for SqliteDropShipOrderRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<DropShipOrder> {
        let row = sqlx::query_as::<_, DropShipOrderRow>(
            "SELECT id, order_number, sales_order_id, customer_id, vendor_id, purchase_order_id,
                    ship_to_name, ship_to_company, ship_to_address, ship_to_city, ship_to_state,
                    ship_to_postal_code, ship_to_country, ship_to_phone, ship_to_email,
                    subtotal, shipping_cost, tax_amount, total, currency, status,
                    vendor_confirmation_number, expected_ship_date, actual_ship_date,
                    expected_delivery_date, actual_delivery_date, notes, internal_notes, priority,
                    created_by, approved_by, approved_at, sent_to_vendor_at,
                    created_at, updated_at, created_by as order_created_by, updated_by
             FROM drop_ship_orders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("DropShipOrder", &id.to_string()))?;
        
        let lines = self.get_order_lines(pool, id).await?;
        Ok(row.into_order(lines)?)
    }

    async fn find_by_order_number(&self, pool: &SqlitePool, order_number: &str) -> Result<DropShipOrder> {
        let row = sqlx::query_as::<_, DropShipOrderRow>(
            "SELECT id, order_number, sales_order_id, customer_id, vendor_id, purchase_order_id,
                    ship_to_name, ship_to_company, ship_to_address, ship_to_city, ship_to_state,
                    ship_to_postal_code, ship_to_country, ship_to_phone, ship_to_email,
                    subtotal, shipping_cost, tax_amount, total, currency, status,
                    vendor_confirmation_number, expected_ship_date, actual_ship_date,
                    expected_delivery_date, actual_delivery_date, notes, internal_notes, priority,
                    created_by, approved_by, approved_at, sent_to_vendor_at,
                    created_at, updated_at, created_by as order_created_by, updated_by
             FROM drop_ship_orders WHERE order_number = ?"
        )
        .bind(order_number)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("DropShipOrder", order_number))?;
        
        let id = Uuid::parse_str(&row.id).unwrap_or_default();
        let lines = self.get_order_lines(pool, id).await?;
        Ok(row.into_order(lines)?)
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<DropShipOrder>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM drop_ship_orders")
            .fetch_one(pool)
            .await?;
        
        let rows = sqlx::query_as::<_, DropShipOrderRow>(
            "SELECT id, order_number, sales_order_id, customer_id, vendor_id, purchase_order_id,
                    ship_to_name, ship_to_company, ship_to_address, ship_to_city, ship_to_state,
                    ship_to_postal_code, ship_to_country, ship_to_phone, ship_to_email,
                    subtotal, shipping_cost, tax_amount, total, currency, status,
                    vendor_confirmation_number, expected_ship_date, actual_ship_date,
                    expected_delivery_date, actual_delivery_date, notes, internal_notes, priority,
                    created_by, approved_by, approved_at, sent_to_vendor_at,
                    created_at, updated_at, created_by as order_created_by, updated_by
             FROM drop_ship_orders ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;
        
        let mut orders = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = self.get_order_lines(pool, id).await?;
            orders.push(row.into_order(lines)?);
        }
        
        Ok(Paginated::new(orders, count.0 as u64, pagination))
    }

    async fn find_by_vendor(&self, pool: &SqlitePool, vendor_id: Uuid, pagination: Pagination) -> Result<Paginated<DropShipOrder>> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM drop_ship_orders WHERE vendor_id = ?"
        )
        .bind(vendor_id.to_string())
        .fetch_one(pool)
        .await?;
        
        let rows = sqlx::query_as::<_, DropShipOrderRow>(
            "SELECT id, order_number, sales_order_id, customer_id, vendor_id, purchase_order_id,
                    ship_to_name, ship_to_company, ship_to_address, ship_to_city, ship_to_state,
                    ship_to_postal_code, ship_to_country, ship_to_phone, ship_to_email,
                    subtotal, shipping_cost, tax_amount, total, currency, status,
                    vendor_confirmation_number, expected_ship_date, actual_ship_date,
                    expected_delivery_date, actual_delivery_date, notes, internal_notes, priority,
                    created_by, approved_by, approved_at, sent_to_vendor_at,
                    created_at, updated_at, created_by as order_created_by, updated_by
             FROM drop_ship_orders WHERE vendor_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(vendor_id.to_string())
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;
        
        let mut orders = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = self.get_order_lines(pool, id).await?;
            orders.push(row.into_order(lines)?);
        }
        
        Ok(Paginated::new(orders, count.0 as u64, pagination))
    }

    async fn find_by_customer(&self, pool: &SqlitePool, customer_id: Uuid, pagination: Pagination) -> Result<Paginated<DropShipOrder>> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM drop_ship_orders WHERE customer_id = ?"
        )
        .bind(customer_id.to_string())
        .fetch_one(pool)
        .await?;
        
        let rows = sqlx::query_as::<_, DropShipOrderRow>(
            "SELECT id, order_number, sales_order_id, customer_id, vendor_id, purchase_order_id,
                    ship_to_name, ship_to_company, ship_to_address, ship_to_city, ship_to_state,
                    ship_to_postal_code, ship_to_country, ship_to_phone, ship_to_email,
                    subtotal, shipping_cost, tax_amount, total, currency, status,
                    vendor_confirmation_number, expected_ship_date, actual_ship_date,
                    expected_delivery_date, actual_delivery_date, notes, internal_notes, priority,
                    created_by, approved_by, approved_at, sent_to_vendor_at,
                    created_at, updated_at, created_by as order_created_by, updated_by
             FROM drop_ship_orders WHERE customer_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(customer_id.to_string())
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;
        
        let mut orders = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = self.get_order_lines(pool, id).await?;
            orders.push(row.into_order(lines)?);
        }
        
        Ok(Paginated::new(orders, count.0 as u64, pagination))
    }

    async fn find_by_status(&self, pool: &SqlitePool, status: DropShipOrderStatus) -> Result<Vec<DropShipOrder>> {
        let status_str = format!("{:?}", status);
        let rows = sqlx::query_as::<_, DropShipOrderRow>(
            "SELECT id, order_number, sales_order_id, customer_id, vendor_id, purchase_order_id,
                    ship_to_name, ship_to_company, ship_to_address, ship_to_city, ship_to_state,
                    ship_to_postal_code, ship_to_country, ship_to_phone, ship_to_email,
                    subtotal, shipping_cost, tax_amount, total, currency, status,
                    vendor_confirmation_number, expected_ship_date, actual_ship_date,
                    expected_delivery_date, actual_delivery_date, notes, internal_notes, priority,
                    created_by, approved_by, approved_at, sent_to_vendor_at,
                    created_at, updated_at, created_by as order_created_by, updated_by
             FROM drop_ship_orders WHERE status = ? ORDER BY priority DESC, created_at ASC"
        )
        .bind(&status_str)
        .fetch_all(pool)
        .await?;
        
        let mut orders = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = self.get_order_lines(pool, id).await?;
            orders.push(row.into_order(lines)?);
        }
        
        Ok(orders)
    }

    async fn create(&self, pool: &SqlitePool, order: DropShipOrder) -> Result<DropShipOrder> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO drop_ship_orders (id, order_number, sales_order_id, customer_id, vendor_id,
             purchase_order_id, ship_to_name, ship_to_company, ship_to_address, ship_to_city,
             ship_to_state, ship_to_postal_code, ship_to_country, ship_to_phone, ship_to_email,
             subtotal, shipping_cost, tax_amount, total, currency, status, vendor_confirmation_number,
             expected_ship_date, actual_ship_date, expected_delivery_date, actual_delivery_date,
             notes, internal_notes, priority, created_by, approved_by, approved_at, sent_to_vendor_at,
             created_at, updated_at, updated_by)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, NULL, NULL, ?, ?, ?, ?, NULL, NULL, NULL, ?, ?, NULL)"
        )
        .bind(order.base.id.to_string())
        .bind(&order.order_number)
        .bind(order.sales_order_id.to_string())
        .bind(order.customer_id.to_string())
        .bind(order.vendor_id.to_string())
        .bind(order.purchase_order_id.map(|id| id.to_string()))
        .bind(&order.ship_to_name)
        .bind(&order.ship_to_company)
        .bind(&order.ship_to_address)
        .bind(&order.ship_to_city)
        .bind(&order.ship_to_state)
        .bind(&order.ship_to_postal_code)
        .bind(&order.ship_to_country)
        .bind(&order.ship_to_phone)
        .bind(&order.ship_to_email)
        .bind(order.subtotal)
        .bind(order.shipping_cost)
        .bind(order.tax_amount)
        .bind(order.total)
        .bind(&order.currency)
        .bind(format!("{:?}", order.status))
        .bind(&order.notes)
        .bind(&order.internal_notes)
        .bind(order.priority)
        .bind(order.created_by.map(|id| id.to_string()))
        .bind(order.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        
        for line in &order.lines {
            sqlx::query(
                "INSERT INTO drop_ship_order_lines (id, drop_ship_order_id, sales_order_line_id,
                 product_id, vendor_sku, description, quantity, quantity_shipped, quantity_cancelled,
                 unit_price, line_total, tax_amount, status)
                 VALUES (?, ?, ?, ?, ?, ?, ?, 0, 0, ?, ?, ?, ?)"
            )
            .bind(line.id.to_string())
            .bind(line.drop_ship_order_id.to_string())
            .bind(line.sales_order_line_id.map(|id| id.to_string()))
            .bind(line.product_id.to_string())
            .bind(&line.vendor_sku)
            .bind(&line.description)
            .bind(line.quantity)
            .bind(line.unit_price)
            .bind(line.line_total)
            .bind(line.tax_amount)
            .bind(format!("{:?}", line.status))
            .execute(pool)
            .await?;
        }
        
        Ok(order)
    }

    async fn update(&self, pool: &SqlitePool, order: DropShipOrder) -> Result<DropShipOrder> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE drop_ship_orders SET status = ?, vendor_confirmation_number = ?,
             expected_ship_date = ?, actual_ship_date = ?, expected_delivery_date = ?, actual_delivery_date = ?,
             notes = ?, internal_notes = ?, priority = ?, approved_by = ?, approved_at = ?,
             sent_to_vendor_at = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(format!("{:?}", order.status))
        .bind(&order.vendor_confirmation_number)
        .bind(order.expected_ship_date.map(|d| d.to_rfc3339()))
        .bind(order.actual_ship_date.map(|d| d.to_rfc3339()))
        .bind(order.expected_delivery_date.map(|d| d.to_rfc3339()))
        .bind(order.actual_delivery_date.map(|d| d.to_rfc3339()))
        .bind(&order.notes)
        .bind(&order.internal_notes)
        .bind(order.priority)
        .bind(order.approved_by.map(|id| id.to_string()))
        .bind(order.approved_at.map(|d| d.to_rfc3339()))
        .bind(order.sent_to_vendor_at.map(|d| d.to_rfc3339()))
        .bind(now.to_rfc3339())
        .bind(order.base.id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("DropShipOrder", &order.base.id.to_string()));
        }
        
        Ok(order)
    }
}

impl SqliteDropShipOrderRepository {
    async fn get_order_lines(&self, pool: &SqlitePool, order_id: Uuid) -> Result<Vec<DropShipOrderLine>> {
        let rows = sqlx::query_as::<_, DropShipOrderLineRow>(
            "SELECT id, drop_ship_order_id, sales_order_line_id, product_id, vendor_sku,
                    description, quantity, quantity_shipped, quantity_cancelled, unit_price,
                    line_total, tax_amount, status
             FROM drop_ship_order_lines WHERE drop_ship_order_id = ? ORDER BY id"
        )
        .bind(order_id.to_string())
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct DropShipOrderRow {
    id: String,
    order_number: String,
    sales_order_id: String,
    customer_id: String,
    vendor_id: String,
    purchase_order_id: Option<String>,
    ship_to_name: String,
    ship_to_company: Option<String>,
    ship_to_address: String,
    ship_to_city: String,
    ship_to_state: Option<String>,
    ship_to_postal_code: String,
    ship_to_country: String,
    ship_to_phone: Option<String>,
    ship_to_email: Option<String>,
    subtotal: i64,
    shipping_cost: i64,
    tax_amount: i64,
    total: i64,
    currency: String,
    status: String,
    vendor_confirmation_number: Option<String>,
    expected_ship_date: Option<String>,
    actual_ship_date: Option<String>,
    expected_delivery_date: Option<String>,
    actual_delivery_date: Option<String>,
    notes: Option<String>,
    internal_notes: Option<String>,
    priority: i64,
    created_by: Option<String>,
    approved_by: Option<String>,
    approved_at: Option<String>,
    sent_to_vendor_at: Option<String>,
    created_at: String,
    updated_at: String,
    order_created_by: Option<String>,
    updated_by: Option<String>,
}

impl DropShipOrderRow {
    fn into_order(self, lines: Vec<DropShipOrderLine>) -> Result<DropShipOrder> {
        let id = Uuid::parse_str(&self.id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid order UUID: {}", e)))?;
        let sales_order_id = Uuid::parse_str(&self.sales_order_id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid sales_order_id UUID: {}", e)))?;
        let customer_id = Uuid::parse_str(&self.customer_id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid customer_id UUID: {}", e)))?;
        let vendor_id = Uuid::parse_str(&self.vendor_id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid vendor_id UUID: {}", e)))?;
        
        Ok(DropShipOrder {
            base: BaseEntity {
                id,
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                created_by: self.order_created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: self.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            order_number: self.order_number,
            sales_order_id,
            customer_id,
            vendor_id,
            purchase_order_id: self.purchase_order_id.and_then(|s| Uuid::parse_str(&s).ok()),
            ship_to_name: self.ship_to_name,
            ship_to_company: self.ship_to_company,
            ship_to_address: self.ship_to_address,
            ship_to_city: self.ship_to_city,
            ship_to_state: self.ship_to_state,
            ship_to_postal_code: self.ship_to_postal_code,
            ship_to_country: self.ship_to_country,
            ship_to_phone: self.ship_to_phone,
            ship_to_email: self.ship_to_email,
            lines,
            subtotal: self.subtotal,
            shipping_cost: self.shipping_cost,
            tax_amount: self.tax_amount,
            total: self.total,
            currency: self.currency,
            status: parse_drop_ship_status(&self.status),
            vendor_confirmation_number: self.vendor_confirmation_number,
            expected_ship_date: self.expected_ship_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            actual_ship_date: self.actual_ship_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            expected_delivery_date: self.expected_delivery_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            actual_delivery_date: self.actual_delivery_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            notes: self.notes,
            internal_notes: self.internal_notes,
            priority: self.priority as i32,
            created_by: self.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
            approved_by: self.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            approved_at: self.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            sent_to_vendor_at: self.sent_to_vendor_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
        })
    }
}

#[derive(sqlx::FromRow)]
struct DropShipOrderLineRow {
    id: String,
    drop_ship_order_id: String,
    sales_order_line_id: Option<String>,
    product_id: String,
    vendor_sku: Option<String>,
    description: String,
    quantity: i64,
    quantity_shipped: i64,
    quantity_cancelled: i64,
    unit_price: i64,
    line_total: i64,
    tax_amount: i64,
    status: String,
}

impl From<DropShipOrderLineRow> for DropShipOrderLine {
    fn from(r: DropShipOrderLineRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            drop_ship_order_id: Uuid::parse_str(&r.drop_ship_order_id).unwrap_or_default(),
            sales_order_line_id: r.sales_order_line_id.and_then(|s| Uuid::parse_str(&s).ok()),
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            vendor_sku: r.vendor_sku,
            description: r.description,
            quantity: r.quantity,
            quantity_shipped: r.quantity_shipped,
            quantity_cancelled: r.quantity_cancelled,
            unit_price: r.unit_price,
            line_total: r.line_total,
            tax_amount: r.tax_amount,
            status: parse_drop_ship_status(&r.status),
        }
    }
}

fn parse_drop_ship_status(s: &str) -> DropShipOrderStatus {
    match s {
        "SentToVendor" => DropShipOrderStatus::SentToVendor,
        "Confirmed" => DropShipOrderStatus::Confirmed,
        "PartiallyShipped" => DropShipOrderStatus::PartiallyShipped,
        "Shipped" => DropShipOrderStatus::Shipped,
        "Delivered" => DropShipOrderStatus::Delivered,
        "Cancelled" => DropShipOrderStatus::Cancelled,
        "OnHold" => DropShipOrderStatus::OnHold,
        _ => DropShipOrderStatus::Pending,
    }
}

pub struct SqliteVendorDropShipSettingsRepository;

#[async_trait]
pub trait VendorDropShipSettingsRepository: Send + Sync {
    async fn find_by_vendor(&self, pool: &SqlitePool, vendor_id: Uuid) -> Result<VendorDropShipSettings>;
    async fn find_all_enabled(&self, pool: &SqlitePool) -> Result<Vec<VendorDropShipSettings>>;
    async fn create(&self, pool: &SqlitePool, settings: VendorDropShipSettings) -> Result<VendorDropShipSettings>;
    async fn update(&self, pool: &SqlitePool, settings: VendorDropShipSettings) -> Result<VendorDropShipSettings>;
}

#[async_trait]
impl VendorDropShipSettingsRepository for SqliteVendorDropShipSettingsRepository {
    async fn find_by_vendor(&self, pool: &SqlitePool, vendor_id: Uuid) -> Result<VendorDropShipSettings> {
        let row = sqlx::query_as::<_, VendorDropShipSettingsRow>(
            "SELECT id, vendor_id, enabled, tier, auto_confirm, require_approval, min_order_value,
                    max_order_value, processing_time_days, shipping_carrier, shipping_method,
                    free_shipping_threshold, handling_fee, allow_partial_shipment, return_policy_days,
                    notification_email, api_endpoint, api_key, sync_inventory, inventory_sync_hours,
                    product_feed_url, status, created_at, updated_at
             FROM vendor_dropship_settings WHERE vendor_id = ?"
        )
        .bind(vendor_id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("VendorDropShipSettings", &vendor_id.to_string()))?;
        
        Ok(row.into())
    }

    async fn find_all_enabled(&self, pool: &SqlitePool) -> Result<Vec<VendorDropShipSettings>> {
        let rows = sqlx::query_as::<_, VendorDropShipSettingsRow>(
            "SELECT id, vendor_id, enabled, tier, auto_confirm, require_approval, min_order_value,
                    max_order_value, processing_time_days, shipping_carrier, shipping_method,
                    free_shipping_threshold, handling_fee, allow_partial_shipment, return_policy_days,
                    notification_email, api_endpoint, api_key, sync_inventory, inventory_sync_hours,
                    product_feed_url, status, created_at, updated_at
             FROM vendor_dropship_settings WHERE enabled = 1 AND status = 'Active'"
        )
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn create(&self, pool: &SqlitePool, settings: VendorDropShipSettings) -> Result<VendorDropShipSettings> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO vendor_dropship_settings (id, vendor_id, enabled, tier, auto_confirm,
             require_approval, min_order_value, max_order_value, processing_time_days,
             shipping_carrier, shipping_method, free_shipping_threshold, handling_fee,
             allow_partial_shipment, return_policy_days, notification_email, api_endpoint, api_key,
             sync_inventory, inventory_sync_hours, product_feed_url, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(settings.id.to_string())
        .bind(settings.vendor_id.to_string())
        .bind(settings.enabled)
        .bind(format!("{:?}", settings.tier))
        .bind(settings.auto_confirm)
        .bind(settings.require_approval)
        .bind(settings.min_order_value)
        .bind(settings.max_order_value)
        .bind(settings.processing_time_days)
        .bind(&settings.shipping_carrier)
        .bind(&settings.shipping_method)
        .bind(settings.free_shipping_threshold)
        .bind(settings.handling_fee)
        .bind(settings.allow_partial_shipment)
        .bind(settings.return_policy_days)
        .bind(&settings.notification_email)
        .bind(&settings.api_endpoint)
        .bind(&settings.api_key)
        .bind(settings.sync_inventory)
        .bind(settings.inventory_sync_hours)
        .bind(&settings.product_feed_url)
        .bind(format!("{:?}", settings.status))
        .bind(settings.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        
        Ok(settings)
    }

    async fn update(&self, pool: &SqlitePool, settings: VendorDropShipSettings) -> Result<VendorDropShipSettings> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE vendor_dropship_settings SET enabled = ?, tier = ?, auto_confirm = ?,
             require_approval = ?, min_order_value = ?, max_order_value = ?, processing_time_days = ?,
             shipping_carrier = ?, shipping_method = ?, free_shipping_threshold = ?, handling_fee = ?,
             allow_partial_shipment = ?, return_policy_days = ?, notification_email = ?,
             api_endpoint = ?, api_key = ?, sync_inventory = ?, inventory_sync_hours = ?,
             product_feed_url = ?, status = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(settings.enabled)
        .bind(format!("{:?}", settings.tier))
        .bind(settings.auto_confirm)
        .bind(settings.require_approval)
        .bind(settings.min_order_value)
        .bind(settings.max_order_value)
        .bind(settings.processing_time_days)
        .bind(&settings.shipping_carrier)
        .bind(&settings.shipping_method)
        .bind(settings.free_shipping_threshold)
        .bind(settings.handling_fee)
        .bind(settings.allow_partial_shipment)
        .bind(settings.return_policy_days)
        .bind(&settings.notification_email)
        .bind(&settings.api_endpoint)
        .bind(&settings.api_key)
        .bind(settings.sync_inventory)
        .bind(settings.inventory_sync_hours)
        .bind(&settings.product_feed_url)
        .bind(format!("{:?}", settings.status))
        .bind(now.to_rfc3339())
        .bind(settings.id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("VendorDropShipSettings", &settings.id.to_string()));
        }
        
        Ok(settings)
    }
}

#[derive(sqlx::FromRow)]
struct VendorDropShipSettingsRow {
    id: String,
    vendor_id: String,
    enabled: i32,
    tier: String,
    auto_confirm: i32,
    require_approval: i32,
    min_order_value: i64,
    max_order_value: Option<i64>,
    processing_time_days: i32,
    shipping_carrier: Option<String>,
    shipping_method: Option<String>,
    free_shipping_threshold: Option<i64>,
    handling_fee: i64,
    allow_partial_shipment: i32,
    return_policy_days: i32,
    notification_email: Option<String>,
    api_endpoint: Option<String>,
    api_key: Option<String>,
    sync_inventory: i32,
    inventory_sync_hours: i32,
    product_feed_url: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<VendorDropShipSettingsRow> for VendorDropShipSettings {
    fn from(r: VendorDropShipSettingsRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            vendor_id: Uuid::parse_str(&r.vendor_id).unwrap_or_default(),
            enabled: r.enabled == 1,
            tier: match r.tier.as_str() {
                "Preferred" => VendorDropShipTier::Preferred,
                "Premium" => VendorDropShipTier::Premium,
                _ => VendorDropShipTier::Standard,
            },
            auto_confirm: r.auto_confirm == 1,
            require_approval: r.require_approval == 1,
            min_order_value: r.min_order_value,
            max_order_value: r.max_order_value,
            processing_time_days: r.processing_time_days,
            shipping_carrier: r.shipping_carrier,
            shipping_method: r.shipping_method,
            free_shipping_threshold: r.free_shipping_threshold,
            handling_fee: r.handling_fee,
            allow_partial_shipment: r.allow_partial_shipment == 1,
            return_policy_days: r.return_policy_days,
            notification_email: r.notification_email,
            api_endpoint: r.api_endpoint,
            api_key: r.api_key,
            sync_inventory: r.sync_inventory == 1,
            inventory_sync_hours: r.inventory_sync_hours,
            product_feed_url: r.product_feed_url,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

pub struct SqliteDropShipShipmentRepository;

#[async_trait]
pub trait DropShipShipmentRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<DropShipShipment>;
    async fn find_by_order(&self, pool: &SqlitePool, order_id: Uuid) -> Result<Vec<DropShipShipment>>;
    async fn find_by_tracking(&self, pool: &SqlitePool, tracking_number: &str) -> Result<DropShipShipment>;
    async fn create(&self, pool: &SqlitePool, shipment: DropShipShipment) -> Result<DropShipShipment>;
    async fn update(&self, pool: &SqlitePool, shipment: DropShipShipment) -> Result<DropShipShipment>;
}

#[async_trait]
impl DropShipShipmentRepository for SqliteDropShipShipmentRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<DropShipShipment> {
        let row = sqlx::query_as::<_, DropShipShipmentRow>(
            "SELECT id, shipment_number, drop_ship_order_id, vendor_id, carrier, carrier_service,
                    tracking_number, tracking_url, shipping_label_url, status, ship_date,
                    estimated_delivery, actual_delivery, weight, weight_unit, dimensions,
                    shipped_from_address, shipped_from_city, shipped_from_state,
                    shipped_from_postal, shipped_from_country, signature_required, signed_by,
                    signed_at, delivery_notes, created_at, updated_at
             FROM drop_ship_shipments WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("DropShipShipment", &id.to_string()))?;
        
        let lines = self.get_shipment_lines(pool, id).await?;
        Ok(row.into_shipment(lines))
    }

    async fn find_by_order(&self, pool: &SqlitePool, order_id: Uuid) -> Result<Vec<DropShipShipment>> {
        let rows = sqlx::query_as::<_, DropShipShipmentRow>(
            "SELECT id, shipment_number, drop_ship_order_id, vendor_id, carrier, carrier_service,
                    tracking_number, tracking_url, shipping_label_url, status, ship_date,
                    estimated_delivery, actual_delivery, weight, weight_unit, dimensions,
                    shipped_from_address, shipped_from_city, shipped_from_state,
                    shipped_from_postal, shipped_from_country, signature_required, signed_by,
                    signed_at, delivery_notes, created_at, updated_at
             FROM drop_ship_shipments WHERE drop_ship_order_id = ? ORDER BY created_at DESC"
        )
        .bind(order_id.to_string())
        .fetch_all(pool)
        .await?;
        
        let mut shipments = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = self.get_shipment_lines(pool, id).await?;
            shipments.push(row.into_shipment(lines));
        }
        
        Ok(shipments)
    }

    async fn find_by_tracking(&self, pool: &SqlitePool, tracking_number: &str) -> Result<DropShipShipment> {
        let row = sqlx::query_as::<_, DropShipShipmentRow>(
            "SELECT id, shipment_number, drop_ship_order_id, vendor_id, carrier, carrier_service,
                    tracking_number, tracking_url, shipping_label_url, status, ship_date,
                    estimated_delivery, actual_delivery, weight, weight_unit, dimensions,
                    shipped_from_address, shipped_from_city, shipped_from_state,
                    shipped_from_postal, shipped_from_country, signature_required, signed_by,
                    signed_at, delivery_notes, created_at, updated_at
             FROM drop_ship_shipments WHERE tracking_number = ?"
        )
        .bind(tracking_number)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("DropShipShipment", tracking_number))?;
        
        let id = Uuid::parse_str(&row.id).unwrap_or_default();
        let lines = self.get_shipment_lines(pool, id).await?;
        Ok(row.into_shipment(lines))
    }

    async fn create(&self, pool: &SqlitePool, shipment: DropShipShipment) -> Result<DropShipShipment> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO drop_ship_shipments (id, shipment_number, drop_ship_order_id, vendor_id,
             carrier, carrier_service, tracking_number, tracking_url, shipping_label_url, status,
             ship_date, estimated_delivery, actual_delivery, weight, weight_unit, dimensions,
             shipped_from_address, shipped_from_city, shipped_from_state, shipped_from_postal,
             shipped_from_country, signature_required, signed_by, signed_at, delivery_notes,
             created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, ?, ?, ?)"
        )
        .bind(shipment.id.to_string())
        .bind(&shipment.shipment_number)
        .bind(shipment.drop_ship_order_id.to_string())
        .bind(shipment.vendor_id.to_string())
        .bind(&shipment.carrier)
        .bind(&shipment.carrier_service)
        .bind(&shipment.tracking_number)
        .bind(&shipment.tracking_url)
        .bind(&shipment.shipping_label_url)
        .bind(format!("{:?}", shipment.status))
        .bind(shipment.ship_date.map(|d| d.to_rfc3339()))
        .bind(shipment.estimated_delivery.map(|d| d.to_rfc3339()))
        .bind(shipment.actual_delivery.map(|d| d.to_rfc3339()))
        .bind(shipment.weight)
        .bind(&shipment.weight_unit)
        .bind(&shipment.dimensions)
        .bind(&shipment.shipped_from_address)
        .bind(&shipment.shipped_from_city)
        .bind(&shipment.shipped_from_state)
        .bind(&shipment.shipped_from_postal)
        .bind(&shipment.shipped_from_country)
        .bind(shipment.signature_required)
        .bind(&shipment.delivery_notes)
        .bind(shipment.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        
        for line in &shipment.lines {
            sqlx::query(
                "INSERT INTO drop_ship_shipment_lines (id, shipment_id, drop_ship_order_line_id,
                 product_id, quantity, condition, notes)
                 VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(line.id.to_string())
            .bind(line.shipment_id.to_string())
            .bind(line.drop_ship_order_line_id.to_string())
            .bind(line.product_id.to_string())
            .bind(line.quantity)
            .bind(&line.condition)
            .bind(&line.notes)
            .execute(pool)
            .await?;
        }
        
        Ok(shipment)
    }

    async fn update(&self, pool: &SqlitePool, shipment: DropShipShipment) -> Result<DropShipShipment> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE drop_ship_shipments SET status = ?, tracking_number = ?, tracking_url = ?,
             shipping_label_url = ?, ship_date = ?, estimated_delivery = ?, actual_delivery = ?,
             weight = ?, weight_unit = ?, dimensions = ?, signature_required = ?, signed_by = ?,
             signed_at = ?, delivery_notes = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(format!("{:?}", shipment.status))
        .bind(&shipment.tracking_number)
        .bind(&shipment.tracking_url)
        .bind(&shipment.shipping_label_url)
        .bind(shipment.ship_date.map(|d| d.to_rfc3339()))
        .bind(shipment.estimated_delivery.map(|d| d.to_rfc3339()))
        .bind(shipment.actual_delivery.map(|d| d.to_rfc3339()))
        .bind(shipment.weight)
        .bind(&shipment.weight_unit)
        .bind(&shipment.dimensions)
        .bind(shipment.signature_required)
        .bind(&shipment.signed_by)
        .bind(shipment.signed_at.map(|d| d.to_rfc3339()))
        .bind(&shipment.delivery_notes)
        .bind(now.to_rfc3339())
        .bind(shipment.id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("DropShipShipment", &shipment.id.to_string()));
        }
        
        Ok(shipment)
    }
}

impl SqliteDropShipShipmentRepository {
    async fn get_shipment_lines(&self, pool: &SqlitePool, shipment_id: Uuid) -> Result<Vec<DropShipShipmentLine>> {
        let rows = sqlx::query_as::<_, DropShipShipmentLineRow>(
            "SELECT id, shipment_id, drop_ship_order_line_id, product_id, quantity, condition, notes
             FROM drop_ship_shipment_lines WHERE shipment_id = ?"
        )
        .bind(shipment_id.to_string())
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct DropShipShipmentRow {
    id: String,
    shipment_number: String,
    drop_ship_order_id: String,
    vendor_id: String,
    carrier: String,
    carrier_service: Option<String>,
    tracking_number: Option<String>,
    tracking_url: Option<String>,
    shipping_label_url: Option<String>,
    status: String,
    ship_date: Option<String>,
    estimated_delivery: Option<String>,
    actual_delivery: Option<String>,
    weight: Option<f64>,
    weight_unit: Option<String>,
    dimensions: Option<String>,
    shipped_from_address: Option<String>,
    shipped_from_city: Option<String>,
    shipped_from_state: Option<String>,
    shipped_from_postal: Option<String>,
    shipped_from_country: Option<String>,
    signature_required: i32,
    signed_by: Option<String>,
    signed_at: Option<String>,
    delivery_notes: Option<String>,
    created_at: String,
    updated_at: String,
}

impl DropShipShipmentRow {
    fn into_shipment(self, lines: Vec<DropShipShipmentLine>) -> DropShipShipment {
        DropShipShipment {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            shipment_number: self.shipment_number,
            drop_ship_order_id: Uuid::parse_str(&self.drop_ship_order_id).unwrap_or_default(),
            vendor_id: Uuid::parse_str(&self.vendor_id).unwrap_or_default(),
            carrier: self.carrier,
            carrier_service: self.carrier_service,
            tracking_number: self.tracking_number,
            tracking_url: self.tracking_url,
            shipping_label_url: self.shipping_label_url,
            status: match self.status.as_str() {
                "LabelCreated" => ShipmentStatus::LabelCreated,
                "PickedUp" => ShipmentStatus::PickedUp,
                "InTransit" => ShipmentStatus::InTransit,
                "OutForDelivery" => ShipmentStatus::OutForDelivery,
                "Delivered" => ShipmentStatus::Delivered,
                "FailedDelivery" => ShipmentStatus::FailedDelivery,
                "Returned" => ShipmentStatus::Returned,
                _ => ShipmentStatus::Pending,
            },
            ship_date: self.ship_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            estimated_delivery: self.estimated_delivery.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            actual_delivery: self.actual_delivery.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            weight: self.weight,
            weight_unit: self.weight_unit,
            dimensions: self.dimensions,
            shipped_from_address: self.shipped_from_address,
            shipped_from_city: self.shipped_from_city,
            shipped_from_state: self.shipped_from_state,
            shipped_from_postal: self.shipped_from_postal,
            shipped_from_country: self.shipped_from_country,
            signature_required: self.signature_required == 1,
            signed_by: self.signed_by,
            signed_at: self.signed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            delivery_notes: self.delivery_notes,
            lines,
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct DropShipShipmentLineRow {
    id: String,
    shipment_id: String,
    drop_ship_order_line_id: String,
    product_id: String,
    quantity: i64,
    condition: Option<String>,
    notes: Option<String>,
}

impl From<DropShipShipmentLineRow> for DropShipShipmentLine {
    fn from(r: DropShipShipmentLineRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            shipment_id: Uuid::parse_str(&r.shipment_id).unwrap_or_default(),
            drop_ship_order_line_id: Uuid::parse_str(&r.drop_ship_order_line_id).unwrap_or_default(),
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            quantity: r.quantity,
            condition: r.condition,
            notes: r.notes,
        }
    }
}
