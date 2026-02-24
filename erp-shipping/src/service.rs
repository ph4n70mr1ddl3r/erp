use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct CarrierService;

impl CarrierService {
    pub async fn create(pool: &SqlitePool, code: String, name: String, carrier_type: CarrierType, tracking_url_template: Option<String>) -> Result<Carrier> {
        let now = Utc::now();
        let carrier = Carrier {
            id: Uuid::new_v4(),
            code,
            name,
            carrier_type,
            api_key: None,
            api_secret: None,
            account_number: None,
            tracking_url_template,
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        CarrierRepository::create(pool, &carrier).await?;
        Ok(carrier)
    }

    pub async fn list_active(pool: &SqlitePool) -> Result<Vec<Carrier>> {
        CarrierRepository::list_active(pool).await
    }
}

pub struct ShipmentService;

impl ShipmentService {
    pub async fn create(pool: &SqlitePool, req: CreateShipmentRequest) -> Result<Shipment> {
        let now = Utc::now();
        let shipment_number = format!("SHP-{}", now.format("%Y%m%d%H%M%S"));
        
        let (shipping_cost, estimated_delivery) = Self::calculate_rate(pool, &req).await?;
        
        let shipment = Shipment {
            id: Uuid::new_v4(),
            shipment_number,
            order_id: req.order_id,
            carrier_id: req.carrier_id,
            carrier_service_id: req.carrier_service_id,
            status: ShipmentStatus::Pending,
            ship_from_name: String::new(),
            ship_from_street: String::new(),
            ship_from_city: String::new(),
            ship_from_state: None,
            ship_from_postal_code: String::new(),
            ship_from_country: String::new(),
            ship_to_name: req.ship_to_name,
            ship_to_street: req.ship_to_street,
            ship_to_city: req.ship_to_city,
            ship_to_state: req.ship_to_state,
            ship_to_postal_code: req.ship_to_postal_code,
            ship_to_country: req.ship_to_country,
            ship_to_phone: req.ship_to_phone,
            weight: req.weight,
            weight_unit: req.weight_unit,
            length: req.length,
            width: req.width,
            height: req.height,
            dimension_unit: req.dimension_unit,
            shipping_cost,
            insurance_cost: 0,
            currency: "USD".to_string(),
            tracking_number: None,
            tracking_url: None,
            label_url: None,
            shipped_at: None,
            estimated_delivery,
            delivered_at: None,
            notes: req.notes,
            created_at: now,
            updated_at: now,
        };
        ShipmentRepository::create(pool, &shipment).await?;
        
        for item in req.items {
            sqlx::query(
                r#"INSERT INTO shipment_items (id, shipment_id, product_id, quantity, weight, declared_value, created_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?)"#
            )
            .bind(Uuid::new_v4().to_string())
            .bind(shipment.id.to_string())
            .bind(item.product_id.to_string())
            .bind(item.quantity)
            .bind(item.weight)
            .bind(item.declared_value)
            .bind(now.to_rfc3339())
            .execute(pool).await?;
        }
        
        Ok(shipment)
    }

    async fn calculate_rate(pool: &SqlitePool, req: &CreateShipmentRequest) -> Result<(i64, Option<chrono::DateTime<Utc>>)> {
        let carriers = CarrierRepository::list_active(pool).await?;
        let carrier = carriers.iter().find(|c| c.id == req.carrier_id);
        
        let base_rate = match carrier.map(|c| c.carrier_type) {
            Some(CarrierType::FedEx) | Some(CarrierType::UPS) | Some(CarrierType::DHL) => 1500i64,
            _ => 800i64,
        };
        
        let weight_rate = (req.weight * 50.0) as i64;
        let dim_weight = (req.length * req.width * req.height) / 139.0;
        let dim_rate = (dim_weight * 50.0) as i64;
        
        let total_rate = base_rate + weight_rate.max(dim_rate);
        let estimated_delivery = Some(Utc::now() + chrono::Duration::days(5));
        
        Ok((total_rate, estimated_delivery))
    }

    pub async fn get_rates(pool: &SqlitePool, req: GetRatesRequest) -> Result<Vec<RateQuote>> {
        let carriers = CarrierRepository::list_active(pool).await?;
        let mut quotes = Vec::new();
        
        for carrier in carriers {
            let base_rate = match carrier.carrier_type {
                CarrierType::FedEx | CarrierType::UPS | CarrierType::DHL => 1500i64,
                _ => 800i64,
            };
            
            let weight_rate = (req.weight * 50.0) as i64;
            let dim_weight = (req.length * req.width * req.height) / 139.0;
            let dim_rate = (dim_weight * 50.0) as i64;
            let total_rate = base_rate + weight_rate.max(dim_rate);
            
            quotes.push(RateQuote {
                carrier_id: carrier.id,
                carrier_name: carrier.name.clone(),
                service_code: "STD".to_string(),
                service_name: "Standard".to_string(),
                total_charge: total_rate,
                currency: req.currency.clone(),
                estimated_days: 5,
            });
        }
        
        Ok(quotes)
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<Option<Shipment>> {
        ShipmentRepository::get_by_id(pool, id).await
    }

    pub async fn update_status(pool: &SqlitePool, id: Uuid, status: ShipmentStatus) -> Result<Shipment> {
        let now = Utc::now();
        let status_str = format!("{:?}", status);
        let shipped_at = if matches!(status, ShipmentStatus::Shipped) { Some(now.to_rfc3339()) } else { None };
        let delivered_at = if matches!(status, ShipmentStatus::Delivered) { Some(now.to_rfc3339()) } else { None };
        
        sqlx::query(
            r#"UPDATE shipments SET status = ?, shipped_at = COALESCE(?, shipped_at), delivered_at = COALESCE(?, delivered_at), updated_at = ? WHERE id = ?"#
        )
        .bind(&status_str)
        .bind(shipped_at)
        .bind(delivered_at)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool).await?;
        
        ShipmentRepository::get_by_id(pool, id).await?.ok_or_else(|| anyhow::anyhow!("Shipment not found"))
    }

    pub async fn add_tracking_event(pool: &SqlitePool, shipment_id: Uuid, event_type: String, description: String, location: Option<String>) -> Result<TrackingEvent> {
        let now = Utc::now();
        let event = TrackingEvent {
            id: Uuid::new_v4(),
            shipment_id,
            event_type,
            description,
            location,
            occurred_at: now,
            created_at: now,
        };
        sqlx::query(
            r#"INSERT INTO tracking_events (id, shipment_id, event_type, description, location, occurred_at, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(event.id.to_string())
        .bind(event.shipment_id.to_string())
        .bind(&event.event_type)
        .bind(&event.description)
        .bind(&event.location)
        .bind(event.occurred_at.to_rfc3339())
        .bind(event.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(event)
    }

    pub async fn list_pending(pool: &SqlitePool) -> Result<Vec<Shipment>> {
        ShipmentRepository::list_by_status(pool, "Pending").await
    }

    pub async fn generate_label(pool: &SqlitePool, id: Uuid) -> Result<String> {
        let shipment = ShipmentRepository::get_by_id(pool, id).await?.ok_or_else(|| anyhow::anyhow!("Shipment not found"))?;
        let label_url = format!("https://labels.example.com/{}.pdf", shipment.shipment_number);
        
        sqlx::query("UPDATE shipments SET label_url = ? WHERE id = ?")
            .bind(&label_url)
            .bind(id.to_string())
            .execute(pool).await?;
        
        Ok(label_url)
    }
}
