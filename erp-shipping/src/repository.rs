use crate::models::*;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct CarrierRepository;

impl CarrierRepository {
    pub async fn create(pool: &SqlitePool, carrier: &Carrier) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO carriers (id, code, name, carrier_type, api_key, api_secret, account_number, tracking_url_template, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(carrier.id.to_string())
        .bind(&carrier.code)
        .bind(&carrier.name)
        .bind(format!("{:?}", carrier.carrier_type))
        .bind(&carrier.api_key)
        .bind(&carrier.api_secret)
        .bind(&carrier.account_number)
        .bind(&carrier.tracking_url_template)
        .bind(carrier.is_active)
        .bind(carrier.created_at.to_rfc3339())
        .bind(carrier.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }

    pub async fn list_active(pool: &SqlitePool) -> Result<Vec<Carrier>> {
        let rows = sqlx::query(
            r#"SELECT id, code, name, carrier_type, api_key, api_secret, account_number, tracking_url_template, is_active, created_at, updated_at FROM carriers WHERE is_active = 1 ORDER BY name"#
        )
        .fetch_all(pool).await?;
        Ok(rows.iter().map(|r| Carrier {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str()).unwrap(),
            code: r.get("code"),
            name: r.get("name"),
            carrier_type: match r.get::<String, _>("carrier_type").as_str() {
                "FedEx" => CarrierType::FedEx,
                "UPS" => CarrierType::UPS,
                "DHL" => CarrierType::DHL,
                "USPS" => CarrierType::USPS,
                "CanadaPost" => CarrierType::CanadaPost,
                "RoyalMail" => CarrierType::RoyalMail,
                "DPD" => CarrierType::DPD,
                _ => CarrierType::Other,
            },
            api_key: r.get("api_key"),
            api_secret: r.get("api_secret"),
            account_number: r.get("account_number"),
            tracking_url_template: r.get("tracking_url_template"),
            is_active: r.get("is_active"),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
        }).collect())
    }
}

pub struct ShipmentRepository;

impl ShipmentRepository {
    pub async fn create(pool: &SqlitePool, shipment: &Shipment) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO shipments (id, shipment_number, order_id, carrier_id, carrier_service_id, status, ship_from_name, ship_from_street, ship_from_city, ship_from_state, ship_from_postal_code, ship_from_country, ship_to_name, ship_to_street, ship_to_city, ship_to_state, ship_to_postal_code, ship_to_country, ship_to_phone, weight, weight_unit, length, width, height, dimension_unit, shipping_cost, insurance_cost, currency, tracking_number, tracking_url, label_url, shipped_at, estimated_delivery, delivered_at, notes, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(shipment.id.to_string())
        .bind(&shipment.shipment_number)
        .bind(shipment.order_id.map(|id| id.to_string()))
        .bind(shipment.carrier_id.to_string())
        .bind(shipment.carrier_service_id.to_string())
        .bind(format!("{:?}", shipment.status))
        .bind(&shipment.ship_from_name)
        .bind(&shipment.ship_from_street)
        .bind(&shipment.ship_from_city)
        .bind(&shipment.ship_from_state)
        .bind(&shipment.ship_from_postal_code)
        .bind(&shipment.ship_from_country)
        .bind(&shipment.ship_to_name)
        .bind(&shipment.ship_to_street)
        .bind(&shipment.ship_to_city)
        .bind(&shipment.ship_to_state)
        .bind(&shipment.ship_to_postal_code)
        .bind(&shipment.ship_to_country)
        .bind(&shipment.ship_to_phone)
        .bind(shipment.weight)
        .bind(&shipment.weight_unit)
        .bind(shipment.length)
        .bind(shipment.width)
        .bind(shipment.height)
        .bind(&shipment.dimension_unit)
        .bind(shipment.shipping_cost)
        .bind(shipment.insurance_cost)
        .bind(&shipment.currency)
        .bind(&shipment.tracking_number)
        .bind(&shipment.tracking_url)
        .bind(&shipment.label_url)
        .bind(shipment.shipped_at.map(|d| d.to_rfc3339()))
        .bind(shipment.estimated_delivery.map(|d| d.to_rfc3339()))
        .bind(shipment.delivered_at.map(|d| d.to_rfc3339()))
        .bind(&shipment.notes)
        .bind(shipment.created_at.to_rfc3339())
        .bind(shipment.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }

    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Shipment>> {
        let row = sqlx::query(
            r#"SELECT id, shipment_number, order_id, carrier_id, carrier_service_id, status, ship_from_name, ship_from_street, ship_from_city, ship_from_state, ship_from_postal_code, ship_from_country, ship_to_name, ship_to_street, ship_to_city, ship_to_state, ship_to_postal_code, ship_to_country, ship_to_phone, weight, weight_unit, length, width, height, dimension_unit, shipping_cost, insurance_cost, currency, tracking_number, tracking_url, label_url, shipped_at, estimated_delivery, delivered_at, notes, created_at, updated_at FROM shipments WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| Self::row_to_shipment(&r)))
    }

    pub async fn list_by_status(pool: &SqlitePool, status: &str) -> Result<Vec<Shipment>> {
        let rows = sqlx::query(
            r#"SELECT id, shipment_number, order_id, carrier_id, carrier_service_id, status, ship_from_name, ship_from_street, ship_from_city, ship_from_state, ship_from_postal_code, ship_from_country, ship_to_name, ship_to_street, ship_to_city, ship_to_state, ship_to_postal_code, ship_to_country, ship_to_phone, weight, weight_unit, length, width, height, dimension_unit, shipping_cost, insurance_cost, currency, tracking_number, tracking_url, label_url, shipped_at, estimated_delivery, delivered_at, notes, created_at, updated_at FROM shipments WHERE status = ? ORDER BY created_at DESC"#
        )
        .bind(status)
        .fetch_all(pool).await?;
        Ok(rows.iter().map(Self::row_to_shipment).collect())
    }

    fn row_to_shipment(r: &sqlx::sqlite::SqliteRow) -> Shipment {
        Shipment {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str()).unwrap(),
            shipment_number: r.get("shipment_number"),
            order_id: r.get::<Option<String>, _>("order_id").and_then(|s| Uuid::parse_str(&s).ok()),
            carrier_id: Uuid::parse_str(r.get::<String, _>("carrier_id").as_str()).unwrap(),
            carrier_service_id: Uuid::parse_str(r.get::<String, _>("carrier_service_id").as_str()).unwrap(),
            status: match r.get::<String, _>("status").as_str() {
                "Picked" => ShipmentStatus::Picked,
                "Packed" => ShipmentStatus::Packed,
                "Shipped" => ShipmentStatus::Shipped,
                "InTransit" => ShipmentStatus::InTransit,
                "OutForDelivery" => ShipmentStatus::OutForDelivery,
                "Delivered" => ShipmentStatus::Delivered,
                "Returned" => ShipmentStatus::Returned,
                "Cancelled" => ShipmentStatus::Cancelled,
                _ => ShipmentStatus::Pending,
            },
            ship_from_name: r.get("ship_from_name"),
            ship_from_street: r.get("ship_from_street"),
            ship_from_city: r.get("ship_from_city"),
            ship_from_state: r.get("ship_from_state"),
            ship_from_postal_code: r.get("ship_from_postal_code"),
            ship_from_country: r.get("ship_from_country"),
            ship_to_name: r.get("ship_to_name"),
            ship_to_street: r.get("ship_to_street"),
            ship_to_city: r.get("ship_to_city"),
            ship_to_state: r.get("ship_to_state"),
            ship_to_postal_code: r.get("ship_to_postal_code"),
            ship_to_country: r.get("ship_to_country"),
            ship_to_phone: r.get("ship_to_phone"),
            weight: r.get("weight"),
            weight_unit: r.get("weight_unit"),
            length: r.get("length"),
            width: r.get("width"),
            height: r.get("height"),
            dimension_unit: r.get("dimension_unit"),
            shipping_cost: r.get("shipping_cost"),
            insurance_cost: r.get("insurance_cost"),
            currency: r.get("currency"),
            tracking_number: r.get("tracking_number"),
            tracking_url: r.get("tracking_url"),
            label_url: r.get("label_url"),
            shipped_at: r.get::<Option<String>, _>("shipped_at").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            estimated_delivery: r.get::<Option<String>, _>("estimated_delivery").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            delivered_at: r.get::<Option<String>, _>("delivered_at").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            notes: r.get("notes"),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
        }
    }
}
