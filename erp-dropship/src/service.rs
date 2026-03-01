use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct DropShipOrderService {
    order_repo: SqliteDropShipOrderRepository,
    settings_repo: SqliteVendorDropShipSettingsRepository,
    shipment_repo: SqliteDropShipShipmentRepository,
}

impl DropShipOrderService {
    pub fn new() -> Self {
        Self {
            order_repo: SqliteDropShipOrderRepository,
            settings_repo: SqliteVendorDropShipSettingsRepository,
            shipment_repo: SqliteDropShipShipmentRepository,
        }
    }

    pub async fn create_order(
        &self,
        pool: &SqlitePool,
        sales_order_id: Uuid,
        customer_id: Uuid,
        vendor_id: Uuid,
        ship_to: ShipToInfo,
        lines: Vec<CreateDropShipOrderLine>,
        currency: String,
        priority: i32,
        notes: Option<String>,
        internal_notes: Option<String>,
        created_by: Option<Uuid>,
    ) -> Result<DropShipOrder> {
        let settings = self.settings_repo.find_by_vendor(pool, vendor_id).await
            .map_err(|_| Error::validation("Vendor is not enabled for drop shipping"))?;
        
        if !settings.enabled {
            return Err(Error::validation("Vendor drop shipping is not enabled"));
        }
        
        let subtotal: i64 = lines.iter().map(|l| l.unit_price * l.quantity).sum();
        let shipping_cost = self.calculate_shipping_cost(&settings, subtotal).await;
        let tax_amount: i64 = lines.iter().map(|l| l.tax_amount).sum();
        let total = subtotal + shipping_cost + tax_amount;
        
        if settings.require_approval && total > settings.min_order_value {
            return Err(Error::validation("Order requires approval due to value"));
        }
        
        let order_number = self.generate_order_number();
        let now = Utc::now();
        let order_id = Uuid::new_v4();
        
        let order_lines: Vec<DropShipOrderLine> = lines
            .into_iter()
            .map(|l| DropShipOrderLine {
                id: Uuid::new_v4(),
                drop_ship_order_id: order_id,
                sales_order_line_id: l.sales_order_line_id,
                product_id: l.product_id,
                vendor_sku: l.vendor_sku,
                description: l.description,
                quantity: l.quantity,
                quantity_shipped: 0,
                quantity_cancelled: 0,
                unit_price: l.unit_price,
                line_total: l.unit_price * l.quantity,
                tax_amount: l.tax_amount,
                status: DropShipOrderStatus::Pending,
            })
            .collect();
        
        let initial_status = if settings.auto_confirm {
            DropShipOrderStatus::Confirmed
        } else {
            DropShipOrderStatus::Pending
        };
        
        let order = DropShipOrder {
            base: BaseEntity::new(),
            order_number,
            sales_order_id,
            customer_id,
            vendor_id,
            purchase_order_id: None,
            ship_to_name: ship_to.name,
            ship_to_company: ship_to.company,
            ship_to_address: ship_to.address,
            ship_to_city: ship_to.city,
            ship_to_state: ship_to.state,
            ship_to_postal_code: ship_to.postal_code,
            ship_to_country: ship_to.country,
            ship_to_phone: ship_to.phone,
            ship_to_email: ship_to.email,
            lines: order_lines,
            subtotal,
            shipping_cost,
            tax_amount,
            total,
            currency,
            status: initial_status,
            vendor_confirmation_number: None,
            expected_ship_date: Some(now + chrono::Duration::days(settings.processing_time_days as i64)),
            actual_ship_date: None,
            expected_delivery_date: None,
            actual_delivery_date: None,
            notes,
            internal_notes,
            priority,
            created_by,
            approved_by: None,
            approved_at: None,
            sent_to_vendor_at: None,
        };
        
        self.order_repo.create(pool, order).await
    }

    pub async fn get_order(&self, pool: &SqlitePool, id: Uuid) -> Result<DropShipOrder> {
        self.order_repo.find_by_id(pool, id).await
    }

    pub async fn get_order_by_number(&self, pool: &SqlitePool, order_number: &str) -> Result<DropShipOrder> {
        self.order_repo.find_by_order_number(pool, order_number).await
    }

    pub async fn list_orders(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<DropShipOrder>> {
        self.order_repo.find_all(pool, pagination).await
    }

    pub async fn list_orders_by_vendor(&self, pool: &SqlitePool, vendor_id: Uuid, pagination: Pagination) -> Result<Paginated<DropShipOrder>> {
        self.order_repo.find_by_vendor(pool, vendor_id, pagination).await
    }

    pub async fn list_orders_by_customer(&self, pool: &SqlitePool, customer_id: Uuid, pagination: Pagination) -> Result<Paginated<DropShipOrder>> {
        self.order_repo.find_by_customer(pool, customer_id, pagination).await
    }

    pub async fn send_to_vendor(&self, pool: &SqlitePool, id: Uuid) -> Result<DropShipOrder> {
        let mut order = self.order_repo.find_by_id(pool, id).await?;
        
        if order.status != DropShipOrderStatus::Pending && order.status != DropShipOrderStatus::OnHold {
            return Err(Error::validation("Order cannot be sent to vendor in current status"));
        }
        
        order.status = DropShipOrderStatus::SentToVendor;
        order.sent_to_vendor_at = Some(Utc::now());
        
        self.order_repo.update(pool, order).await
    }

    pub async fn confirm_order(&self, pool: &SqlitePool, id: Uuid, confirmation_number: Option<String>) -> Result<DropShipOrder> {
        let mut order = self.order_repo.find_by_id(pool, id).await?;
        
        if order.status != DropShipOrderStatus::SentToVendor && order.status != DropShipOrderStatus::Pending {
            return Err(Error::validation("Order cannot be confirmed in current status"));
        }
        
        order.status = DropShipOrderStatus::Confirmed;
        order.vendor_confirmation_number = confirmation_number;
        
        self.order_repo.update(pool, order).await
    }

    pub async fn ship_order(
        &self,
        pool: &SqlitePool,
        id: Uuid,
        tracking_number: String,
        carrier: String,
        carrier_service: Option<String>,
        tracking_url: Option<String>,
        shipped_lines: Vec<ShippedLineInfo>,
    ) -> Result<DropShipShipment> {
        let mut order = self.order_repo.find_by_id(pool, id).await?;
        
        if order.status != DropShipOrderStatus::Confirmed && order.status != DropShipOrderStatus::PartiallyShipped {
            return Err(Error::validation("Order cannot be shipped in current status"));
        }
        
        let shipment_id = Uuid::new_v4();
        let shipment_number = format!("DS-{}", Utc::now().format("%Y%m%d%H%M%S"));
        
        let shipment_lines: Vec<DropShipShipmentLine> = shipped_lines
            .iter()
            .map(|l| DropShipShipmentLine {
                id: Uuid::new_v4(),
                shipment_id,
                drop_ship_order_line_id: l.order_line_id,
                product_id: l.product_id,
                quantity: l.quantity,
                condition: l.condition.clone(),
                notes: l.notes.clone(),
            })
            .collect();
        
        for shipped in &shipped_lines {
            for line in &mut order.lines {
                if line.id == shipped.order_line_id {
                    line.quantity_shipped += shipped.quantity;
                    line.status = if line.quantity_shipped >= line.quantity {
                        DropShipOrderStatus::Shipped
                    } else {
                        DropShipOrderStatus::PartiallyShipped
                    };
                }
            }
        }
        
        let all_shipped = order.lines.iter().all(|l| l.quantity_shipped >= l.quantity);
        let any_shipped = order.lines.iter().any(|l| l.quantity_shipped > 0);
        
        order.status = if all_shipped {
            DropShipOrderStatus::Shipped
        } else if any_shipped {
            DropShipOrderStatus::PartiallyShipped
        } else {
            order.status
        };
        order.actual_ship_date = Some(Utc::now());
        
        let shipment = DropShipShipment {
            id: shipment_id,
            shipment_number,
            drop_ship_order_id: order.base.id,
            vendor_id: order.vendor_id,
            carrier,
            carrier_service,
            tracking_number: Some(tracking_number),
            tracking_url,
            shipping_label_url: None,
            status: ShipmentStatus::PickedUp,
            ship_date: Some(Utc::now()),
            estimated_delivery: None,
            actual_delivery: None,
            weight: None,
            weight_unit: None,
            dimensions: None,
            shipped_from_address: None,
            shipped_from_city: None,
            shipped_from_state: None,
            shipped_from_postal: None,
            shipped_from_country: None,
            signature_required: false,
            signed_by: None,
            signed_at: None,
            delivery_notes: None,
            lines: shipment_lines,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.order_repo.update(pool, order).await?;
        self.shipment_repo.create(pool, shipment).await
    }

    pub async fn mark_delivered(&self, pool: &SqlitePool, shipment_id: Uuid, signed_by: Option<String>) -> Result<DropShipShipment> {
        let mut shipment = self.shipment_repo.find_by_id(pool, shipment_id).await?;
        
        shipment.status = ShipmentStatus::Delivered;
        shipment.actual_delivery = Some(Utc::now());
        shipment.signed_by = signed_by;
        shipment.signed_at = Some(Utc::now());
        
        let mut order = self.order_repo.find_by_id(pool, shipment.drop_ship_order_id).await?;
        
        let shipments = self.shipment_repo.find_by_order(pool, order.base.id).await?;
        let all_delivered = shipments.iter().all(|s| s.status == ShipmentStatus::Delivered);
        
        if all_delivered {
            order.status = DropShipOrderStatus::Delivered;
            order.actual_delivery_date = Some(Utc::now());
            self.order_repo.update(pool, order).await?;
        }
        
        self.shipment_repo.update(pool, shipment).await
    }

    pub async fn cancel_order(&self, pool: &SqlitePool, id: Uuid, reason: Option<String>) -> Result<DropShipOrder> {
        let mut order = self.order_repo.find_by_id(pool, id).await?;
        
        if order.status == DropShipOrderStatus::Shipped || order.status == DropShipOrderStatus::Delivered {
            return Err(Error::validation("Cannot cancel shipped or delivered orders"));
        }
        
        order.status = DropShipOrderStatus::Cancelled;
        order.internal_notes = Some(format!(
            "{}\nCancelled: {}",
            order.internal_notes.unwrap_or_default(),
            reason.unwrap_or_else(|| "No reason provided".to_string())
        ));
        
        self.order_repo.update(pool, order).await
    }

    pub async fn hold_order(&self, pool: &SqlitePool, id: Uuid, reason: Option<String>) -> Result<DropShipOrder> {
        let mut order = self.order_repo.find_by_id(pool, id).await?;
        
        order.status = DropShipOrderStatus::OnHold;
        order.internal_notes = Some(format!(
            "{}\nOn Hold: {}",
            order.internal_notes.unwrap_or_default(),
            reason.unwrap_or_else(|| "No reason provided".to_string())
        ));
        
        self.order_repo.update(pool, order).await
    }

    async fn calculate_shipping_cost(&self, settings: &VendorDropShipSettings, subtotal: i64) -> i64 {
        if let Some(threshold) = settings.free_shipping_threshold {
            if subtotal >= threshold {
                return 0;
            }
        }
        settings.handling_fee
    }

    fn generate_order_number(&self) -> String {
        let now = Utc::now();
        let random_suffix: u32 = rand::random::<u32>() % 10000;
        format!("DSO-{}{:04}", now.format("%Y%m%d%H%M%S"), random_suffix)
    }

    pub async fn get_pending_orders(&self, pool: &SqlitePool) -> Result<Vec<DropShipOrder>> {
        self.order_repo.find_by_status(pool, DropShipOrderStatus::Pending).await
    }

    pub async fn get_orders_to_ship(&self, pool: &SqlitePool) -> Result<Vec<DropShipOrder>> {
        let mut orders = self.order_repo.find_by_status(pool, DropShipOrderStatus::Confirmed).await?;
        orders.extend(self.order_repo.find_by_status(pool, DropShipOrderStatus::PartiallyShipped).await?);
        Ok(orders)
    }
}

pub struct VendorDropShipSettingsService {
    repo: SqliteVendorDropShipSettingsRepository,
}

impl VendorDropShipSettingsService {
    pub fn new() -> Self {
        Self {
            repo: SqliteVendorDropShipSettingsRepository,
        }
    }

    pub async fn get_settings(&self, pool: &SqlitePool, vendor_id: Uuid) -> Result<VendorDropShipSettings> {
        self.repo.find_by_vendor(pool, vendor_id).await
    }

    pub async fn list_enabled_vendors(&self, pool: &SqlitePool) -> Result<Vec<VendorDropShipSettings>> {
        self.repo.find_all_enabled(pool).await
    }

    pub async fn enable_drop_shipping(
        &self,
        pool: &SqlitePool,
        vendor_id: Uuid,
        tier: VendorDropShipTier,
        processing_time_days: i32,
        handling_fee: i64,
        return_policy_days: i32,
        auto_confirm: bool,
        require_approval: bool,
        min_order_value: i64,
    ) -> Result<VendorDropShipSettings> {
        let now = Utc::now();
        let settings = VendorDropShipSettings {
            id: Uuid::new_v4(),
            vendor_id,
            enabled: true,
            tier,
            auto_confirm,
            require_approval,
            min_order_value,
            max_order_value: None,
            processing_time_days,
            shipping_carrier: None,
            shipping_method: None,
            free_shipping_threshold: None,
            handling_fee,
            allow_partial_shipment: true,
            return_policy_days,
            notification_email: None,
            api_endpoint: None,
            api_key: None,
            sync_inventory: false,
            inventory_sync_hours: 24,
            product_feed_url: None,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        };
        
        self.repo.create(pool, settings).await
    }

    pub async fn update_settings(&self, pool: &SqlitePool, settings: VendorDropShipSettings) -> Result<VendorDropShipSettings> {
        self.repo.update(pool, settings).await
    }

    pub async fn disable_drop_shipping(&self, pool: &SqlitePool, vendor_id: Uuid) -> Result<VendorDropShipSettings> {
        let mut settings = self.repo.find_by_vendor(pool, vendor_id).await?;
        settings.enabled = false;
        settings.status = Status::Inactive;
        self.repo.update(pool, settings).await
    }
}

pub struct DropShipShipmentService {
    repo: SqliteDropShipShipmentRepository,
}

impl DropShipShipmentService {
    pub fn new() -> Self {
        Self {
            repo: SqliteDropShipShipmentRepository,
        }
    }

    pub async fn get_shipment(&self, pool: &SqlitePool, id: Uuid) -> Result<DropShipShipment> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn get_shipments_by_order(&self, pool: &SqlitePool, order_id: Uuid) -> Result<Vec<DropShipShipment>> {
        self.repo.find_by_order(pool, order_id).await
    }

    pub async fn track_shipment(&self, pool: &SqlitePool, tracking_number: &str) -> Result<DropShipShipment> {
        self.repo.find_by_tracking(pool, tracking_number).await
    }

    pub async fn update_tracking(
        &self,
        pool: &SqlitePool,
        shipment_id: Uuid,
        tracking_number: String,
        tracking_url: Option<String>,
    ) -> Result<DropShipShipment> {
        let mut shipment = self.repo.find_by_id(pool, shipment_id).await?;
        shipment.tracking_number = Some(tracking_number);
        shipment.tracking_url = tracking_url;
        self.repo.update(pool, shipment).await
    }

    pub async fn update_status(&self, pool: &SqlitePool, shipment_id: Uuid, status: ShipmentStatus) -> Result<DropShipShipment> {
        let mut shipment = self.repo.find_by_id(pool, shipment_id).await?;
        shipment.status = status;
        self.repo.update(pool, shipment).await
    }
}

pub struct CreateDropShipOrderLine {
    pub sales_order_line_id: Option<Uuid>,
    pub product_id: Uuid,
    pub vendor_sku: Option<String>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub tax_amount: i64,
}

pub struct ShipToInfo {
    pub name: String,
    pub company: Option<String>,
    pub address: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

pub struct ShippedLineInfo {
    pub order_line_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub condition: Option<String>,
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_number_generation() {
        let svc = DropShipOrderService::new();
        let order_number = svc.generate_order_number();
        
        assert!(order_number.starts_with("DSO-"));
        assert!(order_number.len() > 10);
    }
}
