use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use crate::repository::*;

pub struct EcommercePlatformService { repo: SqliteEcommercePlatformRepository }
impl Default for EcommercePlatformService {
    fn default() -> Self {
        Self::new()
    }
}

impl EcommercePlatformService {
    pub fn new() -> Self { Self { repo: SqliteEcommercePlatformRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<EcommercePlatform> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<EcommercePlatform>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut platform: EcommercePlatform) -> Result<EcommercePlatform> {
        if platform.name.is_empty() || platform.base_url.is_empty() {
            return Err(Error::validation("Platform name and URL are required"));
        }
        platform.base = BaseEntity::new();
        platform.status = erp_core::Status::Active;
        self.repo.create(pool, platform).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, mut platform: EcommercePlatform) -> Result<EcommercePlatform> {
        platform.base.updated_at = Utc::now();
        self.repo.update(pool, platform).await
    }
    
    pub async fn update_sync_time(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE ecommerce_platforms SET last_sync_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

pub struct EcommerceOrderService { repo: SqliteEcommerceOrderRepository }
impl Default for EcommerceOrderService {
    fn default() -> Self {
        Self::new()
    }
}

impl EcommerceOrderService {
    pub fn new() -> Self { Self { repo: SqliteEcommerceOrderRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<EcommerceOrder> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn get_by_external(&self, pool: &SqlitePool, platform_id: Uuid, external_id: &str) -> Result<EcommerceOrder> {
        self.repo.find_by_external_id(pool, platform_id, external_id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, platform_id: Option<Uuid>, pagination: Pagination) -> Result<Paginated<EcommerceOrder>> {
        self.repo.find_all(pool, platform_id, pagination).await
    }
    
    pub async fn import(&self, pool: &SqlitePool, mut order: EcommerceOrder) -> Result<EcommerceOrder> {
        if self.repo.find_by_external_id(pool, order.platform_id, &order.external_order_id).await.is_ok() { return Err(Error::validation("Order already imported")) }
        
        order.base = BaseEntity::new();
        order.sync_status = SyncStatus::Synced;
        order.imported_at = Some(Utc::now());
        
        for line in &mut order.lines {
            line.id = Uuid::new_v4();
            line.order_id = order.base.id;
        }
        
        self.repo.create(pool, order).await
    }
    
    pub async fn link_sales_order(&self, pool: &SqlitePool, id: Uuid, sales_order_id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE ecommerce_orders SET sales_order_id = ?, updated_at = ? WHERE id = ?"
        )
        .bind(sales_order_id.to_string())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
    
    pub async fn update_fulfillment(&self, pool: &SqlitePool, id: Uuid, tracking_number: &str, _carrier: Option<&str>) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE ecommerce_orders SET tracking_number = ?, fulfillment_status = 'Shipped', updated_at = ? WHERE id = ?"
        )
        .bind(tracking_number)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

pub struct ProductListingService { repo: SqliteProductListingRepository }
impl Default for ProductListingService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductListingService {
    pub fn new() -> Self { Self { repo: SqliteProductListingRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductListing> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list_by_product(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<ProductListing>> {
        self.repo.find_by_product(pool, product_id).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut listing: ProductListing) -> Result<ProductListing> {
        if listing.title.is_empty() {
            return Err(Error::validation("Listing title is required"));
        }
        listing.base = BaseEntity::new();
        listing.status = ProductListingStatus::Draft;
        listing.sync_status = SyncStatus::Pending;
        self.repo.create(pool, listing).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, mut listing: ProductListing) -> Result<ProductListing> {
        listing.base.updated_at = Utc::now();
        listing.sync_status = SyncStatus::Pending;
        self.repo.update(pool, listing).await
    }
    
    pub async fn publish(&self, pool: &SqlitePool, id: Uuid) -> Result<ProductListing> {
        let mut listing = self.repo.find_by_id(pool, id).await?;
        listing.status = ProductListingStatus::Active;
        listing.sync_status = SyncStatus::Pending;
        listing.base.updated_at = Utc::now();
        self.repo.update(pool, listing).await
    }
    
    pub async fn mark_synced(&self, pool: &SqlitePool, id: Uuid, external_id: &str) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE product_listings SET external_product_id = ?, sync_status = 'Synced', last_sync_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(external_id)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

pub struct WebhookService;
impl Default for WebhookService {
    fn default() -> Self {
        Self::new()
    }
}

impl WebhookService {
    pub fn new() -> Self { Self }
    
    pub async fn receive(
        pool: &SqlitePool,
        platform_id: Uuid,
        event_type: &str,
        external_id: &str,
        payload: &str,
    ) -> Result<WebhookEvent> {
        let now = Utc::now();
        let event = WebhookEvent {
            base: BaseEntity::new(),
            platform_id,
            event_type: event_type.to_string(),
            external_id: external_id.to_string(),
            payload: payload.to_string(),
            processed: false,
            processed_at: None,
            error: None,
            retry_count: 0,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO webhook_events (id, platform_id, event_type, external_id, payload, processed, processed_at, error, retry_count, created_at)
             VALUES (?, ?, ?, ?, ?, 0, NULL, NULL, 0, ?)"
        )
        .bind(event.base.id.to_string())
        .bind(event.platform_id.to_string())
        .bind(&event.event_type)
        .bind(&event.external_id)
        .bind(&event.payload)
        .bind(event.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(event)
    }
    
    pub async fn mark_processed(pool: &SqlitePool, id: Uuid, error: Option<&str>) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE webhook_events SET processed = 1, processed_at = ?, error = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(error)
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

pub struct InventorySyncService;
impl Default for InventorySyncService {
    fn default() -> Self {
        Self::new()
    }
}

impl InventorySyncService {
    pub fn new() -> Self { Self }
    
    pub async fn sync_to_platform(
        pool: &SqlitePool,
        platform_id: Uuid,
        product_id: Uuid,
        quantity: i64,
        external_product_id: &str,
    ) -> Result<InventorySync> {
        let now = Utc::now();
        let sync = InventorySync {
            base: BaseEntity::new(),
            platform_id,
            product_id,
            warehouse_id: None,
            external_product_id: external_product_id.to_string(),
            external_variant_id: None,
            local_quantity: quantity,
            remote_quantity: quantity,
            reserved_quantity: 0,
            sync_status: SyncStatus::Synced,
            last_sync_at: Some(now),
            sync_error: None,
        };
        
        sqlx::query(
            "INSERT INTO inventory_syncs (id, platform_id, product_id, warehouse_id, external_product_id, external_variant_id, local_quantity, remote_quantity, reserved_quantity, sync_status, last_sync_at, sync_error, created_at, updated_at)
             VALUES (?, ?, ?, NULL, ?, NULL, ?, ?, ?, 'Synced', ?, NULL, ?, ?)"
        )
        .bind(sync.base.id.to_string())
        .bind(sync.platform_id.to_string())
        .bind(sync.product_id.to_string())
        .bind(&sync.external_product_id)
        .bind(sync.local_quantity)
        .bind(sync.remote_quantity)
        .bind(sync.reserved_quantity)
        .bind(sync.last_sync_at.map(|d| d.to_rfc3339()))
        .bind(sync.base.created_at.to_rfc3339())
        .bind(sync.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(sync)
    }
}
