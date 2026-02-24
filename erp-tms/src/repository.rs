use crate::models::*;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait TMSRepository {
    async fn create_vehicle(&self, vehicle: &Vehicle) -> anyhow::Result<()>;
    async fn get_vehicle(&self, id: Uuid) -> anyhow::Result<Option<Vehicle>>;
    async fn list_vehicles(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Vehicle>>;
    async fn update_vehicle(&self, vehicle: &Vehicle) -> anyhow::Result<()>;
    
    async fn create_driver(&self, driver: &Driver) -> anyhow::Result<()>;
    async fn get_driver(&self, id: Uuid) -> anyhow::Result<Option<Driver>>;
    async fn list_drivers(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Driver>>;
    async fn update_driver(&self, driver: &Driver) -> anyhow::Result<()>;
    
    async fn create_carrier_contract(&self, contract: &CarrierContract) -> anyhow::Result<()>;
    async fn get_carrier_contract(&self, id: Uuid) -> anyhow::Result<Option<CarrierContract>>;
    async fn list_carrier_contracts(&self, carrier_id: Uuid) -> anyhow::Result<Vec<CarrierContract>>;
    
    async fn create_freight_rate(&self, rate: &FreightRate) -> anyhow::Result<()>;
    async fn list_freight_rates(&self, contract_id: Uuid) -> anyhow::Result<Vec<FreightRate>>;
    
    async fn create_load(&self, load: &Load) -> anyhow::Result<()>;
    async fn get_load(&self, id: Uuid) -> anyhow::Result<Option<Load>>;
    async fn list_loads(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Load>>;
    async fn update_load(&self, load: &Load) -> anyhow::Result<()>;
    
    async fn create_load_stop(&self, stop: &LoadStop) -> anyhow::Result<()>;
    async fn list_load_stops(&self, load_id: Uuid) -> anyhow::Result<Vec<LoadStop>>;
    
    async fn create_load_item(&self, item: &LoadItem) -> anyhow::Result<()>;
    async fn list_load_items(&self, load_id: Uuid) -> anyhow::Result<Vec<LoadItem>>;
    
    async fn create_route(&self, route: &Route) -> anyhow::Result<()>;
    async fn get_route(&self, id: Uuid) -> anyhow::Result<Option<Route>>;
    async fn list_routes(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Route>>;
    async fn update_route(&self, route: &Route) -> anyhow::Result<()>;
    
    async fn create_route_stop(&self, stop: &RouteStop) -> anyhow::Result<()>;
    async fn list_route_stops(&self, route_id: Uuid) -> anyhow::Result<Vec<RouteStop>>;
    
    async fn create_freight_invoice(&self, invoice: &FreightInvoice) -> anyhow::Result<()>;
    async fn get_freight_invoice(&self, id: Uuid) -> anyhow::Result<Option<FreightInvoice>>;
    async fn list_freight_invoices(&self, status: Option<FreightAuditStatus>) -> anyhow::Result<Vec<FreightInvoice>>;
    async fn update_freight_invoice(&self, invoice: &FreightInvoice) -> anyhow::Result<()>;
    
    async fn create_freight_invoice_line(&self, line: &FreightInvoiceLine) -> anyhow::Result<()>;
    async fn list_freight_invoice_lines(&self, invoice_id: Uuid) -> anyhow::Result<Vec<FreightInvoiceLine>>;
    
    async fn create_fuel_purchase(&self, purchase: &FuelPurchase) -> anyhow::Result<()>;
    async fn list_fuel_purchases(&self, vehicle_id: Option<Uuid>, limit: i32) -> anyhow::Result<Vec<FuelPurchase>>;
    
    async fn create_accessorial_charge(&self, charge: &AccessorialCharge) -> anyhow::Result<()>;
    async fn list_accessorial_charges(&self) -> anyhow::Result<Vec<AccessorialCharge>>;
}

pub struct SqliteTMSRepository {
    pool: SqlitePool,
}

impl SqliteTMSRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TMSRepository for SqliteTMSRepository {
    async fn create_vehicle(&self, vehicle: &Vehicle) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_vehicles (id, vehicle_number, vehicle_type, license_plate, vin, make, model, year,
                capacity_weight, capacity_volume, capacity_unit, fuel_type, status, current_location_lat,
                current_location_lng, odometer, last_maintenance, next_maintenance, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            vehicle.id, vehicle.vehicle_number, vehicle.vehicle_type, vehicle.license_plate,
            vehicle.vin, vehicle.make, vehicle.model, vehicle.year,
            vehicle.capacity_weight, vehicle.capacity_volume, vehicle.capacity_unit,
            vehicle.fuel_type, vehicle.status as _, vehicle.current_location_lat,
            vehicle.current_location_lng, vehicle.odometer, vehicle.last_maintenance,
            vehicle.next_maintenance, vehicle.created_at, vehicle.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_vehicle(&self, id: Uuid) -> anyhow::Result<Option<Vehicle>> {
        let vehicle = sqlx::query_as!(
            Vehicle,
            r#"SELECT id, vehicle_number, vehicle_type, license_plate, vin, make, model, year,
                capacity_weight, capacity_volume, capacity_unit, fuel_type, status as "status: _",
                current_location_lat, current_location_lng, odometer, last_maintenance,
                next_maintenance, created_at, updated_at FROM tms_vehicles WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(vehicle)
    }

    async fn list_vehicles(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Vehicle>> {
        let vehicles = sqlx::query_as!(
            Vehicle,
            r#"SELECT id, vehicle_number, vehicle_type, license_plate, vin, make, model, year,
                capacity_weight, capacity_volume, capacity_unit, fuel_type, status as "status: _",
                current_location_lat, current_location_lng, odometer, last_maintenance,
                next_maintenance, created_at, updated_at FROM tms_vehicles
                ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            limit, offset
        ).fetch_all(&self.pool).await?;
        Ok(vehicles)
    }

    async fn update_vehicle(&self, vehicle: &Vehicle) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tms_vehicles SET vehicle_number = ?, vehicle_type = ?, license_plate = ?,
                vin = ?, make = ?, model = ?, year = ?, capacity_weight = ?, capacity_volume = ?,
                capacity_unit = ?, fuel_type = ?, status = ?, current_location_lat = ?,
                current_location_lng = ?, odometer = ?, last_maintenance = ?, next_maintenance = ?,
                updated_at = ? WHERE id = ?"#,
            vehicle.vehicle_number, vehicle.vehicle_type, vehicle.license_plate,
            vehicle.vin, vehicle.make, vehicle.model, vehicle.year,
            vehicle.capacity_weight, vehicle.capacity_volume, vehicle.capacity_unit,
            vehicle.fuel_type, vehicle.status as _, vehicle.current_location_lat,
            vehicle.current_location_lng, vehicle.odometer, vehicle.last_maintenance,
            vehicle.next_maintenance, vehicle.updated_at, vehicle.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_driver(&self, driver: &Driver) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_drivers (id, employee_id, driver_code, first_name, last_name, phone,
                email, license_number, license_class, license_expiry, status, current_vehicle_id,
                current_location_lat, current_location_lng, hours_driven_today, hours_available,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            driver.id, driver.employee_id, driver.driver_code, driver.first_name, driver.last_name,
            driver.phone, driver.email, driver.license_number, driver.license_class,
            driver.license_expiry, driver.status as _, driver.current_vehicle_id,
            driver.current_location_lat, driver.current_location_lng, driver.hours_driven_today,
            driver.hours_available, driver.created_at, driver.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_driver(&self, id: Uuid) -> anyhow::Result<Option<Driver>> {
        let driver = sqlx::query_as!(
            Driver,
            r#"SELECT id, employee_id, driver_code, first_name, last_name, phone, email,
                license_number, license_class, license_expiry, status as "status: _",
                current_vehicle_id, current_location_lat, current_location_lng,
                hours_driven_today, hours_available, created_at, updated_at
                FROM tms_drivers WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(driver)
    }

    async fn list_drivers(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Driver>> {
        let drivers = sqlx::query_as!(
            Driver,
            r#"SELECT id, employee_id, driver_code, first_name, last_name, phone, email,
                license_number, license_class, license_expiry, status as "status: _",
                current_vehicle_id, current_location_lat, current_location_lng,
                hours_driven_today, hours_available, created_at, updated_at
                FROM tms_drivers ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            limit, offset
        ).fetch_all(&self.pool).await?;
        Ok(drivers)
    }

    async fn update_driver(&self, driver: &Driver) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tms_drivers SET driver_code = ?, first_name = ?, last_name = ?, phone = ?,
                email = ?, license_number = ?, license_class = ?, license_expiry = ?, status = ?,
                current_vehicle_id = ?, current_location_lat = ?, current_location_lng = ?,
                hours_driven_today = ?, hours_available = ?, updated_at = ? WHERE id = ?"#,
            driver.driver_code, driver.first_name, driver.last_name, driver.phone,
            driver.email, driver.license_number, driver.license_class, driver.license_expiry,
            driver.status as _, driver.current_vehicle_id, driver.current_location_lat,
            driver.current_location_lng, driver.hours_driven_today, driver.hours_available,
            driver.updated_at, driver.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_carrier_contract(&self, contract: &CarrierContract) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_carrier_contracts (id, carrier_id, contract_number, contract_name,
                start_date, end_date, payment_terms, fuel_surcharge_percent, accessorial_rates,
                volume_commitment, volume_discount_tiers, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            contract.id, contract.carrier_id, contract.contract_number, contract.contract_name,
            contract.start_date, contract.end_date, contract.payment_terms,
            contract.fuel_surcharge_percent, contract.accessorial_rates,
            contract.volume_commitment, contract.volume_discount_tiers, contract.is_active,
            contract.created_at, contract.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_carrier_contract(&self, id: Uuid) -> anyhow::Result<Option<CarrierContract>> {
        let contract = sqlx::query_as!(
            CarrierContract,
            r#"SELECT id, carrier_id, contract_number, contract_name, start_date, end_date,
                payment_terms, fuel_surcharge_percent, accessorial_rates, volume_commitment,
                volume_discount_tiers, is_active, created_at, updated_at
                FROM tms_carrier_contracts WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(contract)
    }

    async fn list_carrier_contracts(&self, carrier_id: Uuid) -> anyhow::Result<Vec<CarrierContract>> {
        let contracts = sqlx::query_as!(
            CarrierContract,
            r#"SELECT id, carrier_id, contract_number, contract_name, start_date, end_date,
                payment_terms, fuel_surcharge_percent, accessorial_rates, volume_commitment,
                volume_discount_tiers, is_active, created_at, updated_at
                FROM tms_carrier_contracts WHERE carrier_id = ? ORDER BY created_at DESC"#,
            carrier_id
        ).fetch_all(&self.pool).await?;
        Ok(contracts)
    }

    async fn create_freight_rate(&self, rate: &FreightRate) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_freight_rates (id, contract_id, origin_zone, destination_zone,
                service_type, rate_type, base_rate, per_mile_rate, per_kg_rate, per_pallet_rate,
                min_charge, max_charge, currency, effective_date, expiry_date, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rate.id, rate.contract_id, rate.origin_zone, rate.destination_zone,
            rate.service_type, rate.rate_type, rate.base_rate, rate.per_mile_rate,
            rate.per_kg_rate, rate.per_pallet_rate, rate.min_charge, rate.max_charge,
            rate.currency, rate.effective_date, rate.expiry_date, rate.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_freight_rates(&self, contract_id: Uuid) -> anyhow::Result<Vec<FreightRate>> {
        let rates = sqlx::query_as!(
            FreightRate,
            r#"SELECT id, contract_id, origin_zone, destination_zone, service_type, rate_type,
                base_rate, per_mile_rate, per_kg_rate, per_pallet_rate, min_charge, max_charge,
                currency, effective_date, expiry_date, created_at
                FROM tms_freight_rates WHERE contract_id = ?"#,
            contract_id
        ).fetch_all(&self.pool).await?;
        Ok(rates)
    }

    async fn create_load(&self, load: &Load) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_loads (id, load_number, carrier_id, driver_id, vehicle_id, status,
                origin_name, origin_street, origin_city, origin_state, origin_postal_code, origin_country,
                origin_lat, origin_lng, destination_name, destination_street, destination_city,
                destination_state, destination_postal_code, destination_country, destination_lat,
                destination_lng, planned_pickup, actual_pickup, planned_delivery, actual_delivery,
                total_weight, total_volume, total_pallets, total_pieces, freight_charge, fuel_surcharge,
                accessorial_charges, total_charge, currency, distance_miles, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            load.id, load.load_number, load.carrier_id, load.driver_id, load.vehicle_id,
            load.status as _, load.origin_name, load.origin_street, load.origin_city,
            load.origin_state, load.origin_postal_code, load.origin_country, load.origin_lat,
            load.origin_lng, load.destination_name, load.destination_street, load.destination_city,
            load.destination_state, load.destination_postal_code, load.destination_country,
            load.destination_lat, load.destination_lng, load.planned_pickup, load.actual_pickup,
            load.planned_delivery, load.actual_delivery, load.total_weight, load.total_volume,
            load.total_pallets, load.total_pieces, load.freight_charge, load.fuel_surcharge,
            load.accessorial_charges, load.total_charge, load.currency, load.distance_miles,
            load.notes, load.created_at, load.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_load(&self, id: Uuid) -> anyhow::Result<Option<Load>> {
        let load = sqlx::query_as!(
            Load,
            r#"SELECT id, load_number, carrier_id, driver_id, vehicle_id, status as "status: _",
                origin_name, origin_street, origin_city, origin_state, origin_postal_code,
                origin_country, origin_lat, origin_lng, destination_name, destination_street,
                destination_city, destination_state, destination_postal_code, destination_country,
                destination_lat, destination_lng, planned_pickup, actual_pickup, planned_delivery,
                actual_delivery, total_weight, total_volume, total_pallets, total_pieces,
                freight_charge, fuel_surcharge, accessorial_charges, total_charge, currency,
                distance_miles, notes, created_at, updated_at FROM tms_loads WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(load)
    }

    async fn list_loads(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Load>> {
        let loads = sqlx::query_as!(
            Load,
            r#"SELECT id, load_number, carrier_id, driver_id, vehicle_id, status as "status: _",
                origin_name, origin_street, origin_city, origin_state, origin_postal_code,
                origin_country, origin_lat, origin_lng, destination_name, destination_street,
                destination_city, destination_state, destination_postal_code, destination_country,
                destination_lat, destination_lng, planned_pickup, actual_pickup, planned_delivery,
                actual_delivery, total_weight, total_volume, total_pallets, total_pieces,
                freight_charge, fuel_surcharge, accessorial_charges, total_charge, currency,
                distance_miles, notes, created_at, updated_at FROM tms_loads
                ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            limit, offset
        ).fetch_all(&self.pool).await?;
        Ok(loads)
    }

    async fn update_load(&self, load: &Load) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tms_loads SET carrier_id = ?, driver_id = ?, vehicle_id = ?, status = ?,
                actual_pickup = ?, actual_delivery = ?, total_weight = ?, total_volume = ?,
                total_pallets = ?, total_pieces = ?, freight_charge = ?, fuel_surcharge = ?,
                accessorial_charges = ?, total_charge = ?, distance_miles = ?, notes = ?,
                updated_at = ? WHERE id = ?"#,
            load.carrier_id, load.driver_id, load.vehicle_id, load.status as _,
            load.actual_pickup, load.actual_delivery, load.total_weight, load.total_volume,
            load.total_pallets, load.total_pieces, load.freight_charge, load.fuel_surcharge,
            load.accessorial_charges, load.total_charge, load.distance_miles, load.notes,
            load.updated_at, load.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_load_stop(&self, stop: &LoadStop) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_load_stops (id, load_id, stop_number, stop_type, location_name,
                street, city, state, postal_code, country, lat, lng, planned_arrival,
                actual_arrival, planned_departure, actual_departure, appointment_number,
                instructions, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            stop.id, stop.load_id, stop.stop_number, stop.stop_type, stop.location_name,
            stop.street, stop.city, stop.state, stop.postal_code, stop.country, stop.lat,
            stop.lng, stop.planned_arrival, stop.actual_arrival, stop.planned_departure,
            stop.actual_departure, stop.appointment_number, stop.instructions, stop.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_load_stops(&self, load_id: Uuid) -> anyhow::Result<Vec<LoadStop>> {
        let stops = sqlx::query_as!(
            LoadStop,
            r#"SELECT id, load_id, stop_number, stop_type, location_name, street, city, state,
                postal_code, country, lat, lng, planned_arrival, actual_arrival, planned_departure,
                actual_departure, appointment_number, instructions, created_at
                FROM tms_load_stops WHERE load_id = ? ORDER BY stop_number"#,
            load_id
        ).fetch_all(&self.pool).await?;
        Ok(stops)
    }

    async fn create_load_item(&self, item: &LoadItem) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_load_items (id, load_id, stop_id, shipment_id, product_id,
                description, quantity, weight, volume, pallets, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            item.id, item.load_id, item.stop_id, item.shipment_id, item.product_id,
            item.description, item.quantity, item.weight, item.volume, item.pallets, item.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_load_items(&self, load_id: Uuid) -> anyhow::Result<Vec<LoadItem>> {
        let items = sqlx::query_as!(
            LoadItem,
            r#"SELECT id, load_id, stop_id, shipment_id, product_id, description, quantity,
                weight, volume, pallets, created_at FROM tms_load_items WHERE load_id = ?"#,
            load_id
        ).fetch_all(&self.pool).await?;
        Ok(items)
    }

    async fn create_route(&self, route: &Route) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_routes (id, route_number, route_name, driver_id, vehicle_id, status,
                planned_start, actual_start, planned_end, actual_end, total_distance,
                total_duration_minutes, total_stops, completed_stops, total_weight,
                optimization_score, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            route.id, route.route_number, route.route_name, route.driver_id, route.vehicle_id,
            route.status as _, route.planned_start, route.actual_start, route.planned_end,
            route.actual_end, route.total_distance, route.total_duration_minutes,
            route.total_stops, route.completed_stops, route.total_weight, route.optimization_score,
            route.created_at, route.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_route(&self, id: Uuid) -> anyhow::Result<Option<Route>> {
        let route = sqlx::query_as!(
            Route,
            r#"SELECT id, route_number, route_name, driver_id, vehicle_id, status as "status: _",
                planned_start, actual_start, planned_end, actual_end, total_distance,
                total_duration_minutes, total_stops, completed_stops, total_weight,
                optimization_score, created_at, updated_at FROM tms_routes WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(route)
    }

    async fn list_routes(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Route>> {
        let routes = sqlx::query_as!(
            Route,
            r#"SELECT id, route_number, route_name, driver_id, vehicle_id, status as "status: _",
                planned_start, actual_start, planned_end, actual_end, total_distance,
                total_duration_minutes, total_stops, completed_stops, total_weight,
                optimization_score, created_at, updated_at FROM tms_routes
                ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            limit, offset
        ).fetch_all(&self.pool).await?;
        Ok(routes)
    }

    async fn update_route(&self, route: &Route) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tms_routes SET driver_id = ?, vehicle_id = ?, status = ?, actual_start = ?,
                actual_end = ?, total_distance = ?, total_duration_minutes = ?, completed_stops = ?,
                optimization_score = ?, updated_at = ? WHERE id = ?"#,
            route.driver_id, route.vehicle_id, route.status as _, route.actual_start,
            route.actual_end, route.total_distance, route.total_duration_minutes,
            route.completed_stops, route.optimization_score, route.updated_at, route.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_route_stop(&self, stop: &RouteStop) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_route_stops (id, route_id, stop_sequence, stop_type, load_id,
                location_name, street, city, state, postal_code, country, lat, lng,
                planned_arrival, actual_arrival, planned_duration_minutes, status, notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            stop.id, stop.route_id, stop.stop_sequence, stop.stop_type, stop.load_id,
            stop.location_name, stop.street, stop.city, stop.state, stop.postal_code,
            stop.country, stop.lat, stop.lng, stop.planned_arrival, stop.actual_arrival,
            stop.planned_duration_minutes, stop.status, stop.notes, stop.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_route_stops(&self, route_id: Uuid) -> anyhow::Result<Vec<RouteStop>> {
        let stops = sqlx::query_as!(
            RouteStop,
            r#"SELECT id, route_id, stop_sequence, stop_type, load_id, location_name, street,
                city, state, postal_code, country, lat, lng, planned_arrival, actual_arrival,
                planned_duration_minutes, status, notes, created_at
                FROM tms_route_stops WHERE route_id = ? ORDER BY stop_sequence"#,
            route_id
        ).fetch_all(&self.pool).await?;
        Ok(stops)
    }

    async fn create_freight_invoice(&self, invoice: &FreightInvoice) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_freight_invoices (id, invoice_number, carrier_id, load_id,
                invoice_date, due_date, status, base_freight, fuel_surcharge, accessorial_charges,
                tax, total_amount, currency, expected_amount, variance_amount, variance_reason,
                approved_by, approved_at, paid_at, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            invoice.id, invoice.invoice_number, invoice.carrier_id, invoice.load_id,
            invoice.invoice_date, invoice.due_date, invoice.status as _, invoice.base_freight,
            invoice.fuel_surcharge, invoice.accessorial_charges, invoice.tax, invoice.total_amount,
            invoice.currency, invoice.expected_amount, invoice.variance_amount,
            invoice.variance_reason, invoice.approved_by, invoice.approved_at, invoice.paid_at,
            invoice.created_at, invoice.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_freight_invoice(&self, id: Uuid) -> anyhow::Result<Option<FreightInvoice>> {
        let invoice = sqlx::query_as!(
            FreightInvoice,
            r#"SELECT id, invoice_number, carrier_id, load_id, invoice_date, due_date,
                status as "status: _", base_freight, fuel_surcharge, accessorial_charges, tax,
                total_amount, currency, expected_amount, variance_amount, variance_reason,
                approved_by, approved_at, paid_at, created_at, updated_at
                FROM tms_freight_invoices WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(invoice)
    }

    async fn list_freight_invoices(&self, status: Option<FreightAuditStatus>) -> anyhow::Result<Vec<FreightInvoice>> {
        let invoices = if let Some(s) = status {
            sqlx::query_as!(
                FreightInvoice,
                r#"SELECT id, invoice_number, carrier_id, load_id, invoice_date, due_date,
                    status as "status: _", base_freight, fuel_surcharge, accessorial_charges, tax,
                    total_amount, currency, expected_amount, variance_amount, variance_reason,
                    approved_by, approved_at, paid_at, created_at, updated_at
                    FROM tms_freight_invoices WHERE status = ? ORDER BY created_at DESC"#,
                    s as _
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                FreightInvoice,
                r#"SELECT id, invoice_number, carrier_id, load_id, invoice_date, due_date,
                    status as "status: _", base_freight, fuel_surcharge, accessorial_charges, tax,
                    total_amount, currency, expected_amount, variance_amount, variance_reason,
                    approved_by, approved_at, paid_at, created_at, updated_at
                    FROM tms_freight_invoices ORDER BY created_at DESC"#
            ).fetch_all(&self.pool).await?
        };
        Ok(invoices)
    }

    async fn update_freight_invoice(&self, invoice: &FreightInvoice) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tms_freight_invoices SET status = ?, variance_amount = ?, variance_reason = ?,
                approved_by = ?, approved_at = ?, paid_at = ?, updated_at = ? WHERE id = ?"#,
            invoice.status as _, invoice.variance_amount, invoice.variance_reason,
            invoice.approved_by, invoice.approved_at, invoice.paid_at, invoice.updated_at, invoice.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_freight_invoice_line(&self, line: &FreightInvoiceLine) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_freight_invoice_lines (id, invoice_id, line_number, charge_type,
                description, quantity, unit, rate, amount, expected_amount, variance, currency, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            line.id, line.invoice_id, line.line_number, line.charge_type, line.description,
            line.quantity, line.unit, line.rate, line.amount, line.expected_amount, line.variance,
            line.currency, line.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_freight_invoice_lines(&self, invoice_id: Uuid) -> anyhow::Result<Vec<FreightInvoiceLine>> {
        let lines = sqlx::query_as!(
            FreightInvoiceLine,
            r#"SELECT id, invoice_id, line_number, charge_type, description, quantity, unit,
                rate, amount, expected_amount, variance, currency, created_at
                FROM tms_freight_invoice_lines WHERE invoice_id = ? ORDER BY line_number"#,
            invoice_id
        ).fetch_all(&self.pool).await?;
        Ok(lines)
    }

    async fn create_fuel_purchase(&self, purchase: &FuelPurchase) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_fuel_purchases (id, vehicle_id, driver_id, purchase_date,
                vendor_name, location, fuel_type, quantity, unit, price_per_unit, total_amount,
                currency, odometer, receipt_number, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            purchase.id, purchase.vehicle_id, purchase.driver_id, purchase.purchase_date,
            purchase.vendor_name, purchase.location, purchase.fuel_type, purchase.quantity,
            purchase.unit, purchase.price_per_unit, purchase.total_amount, purchase.currency,
            purchase.odometer, purchase.receipt_number, purchase.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_fuel_purchases(&self, vehicle_id: Option<Uuid>, limit: i32) -> anyhow::Result<Vec<FuelPurchase>> {
        let purchases = if let Some(vid) = vehicle_id {
            sqlx::query_as!(
                FuelPurchase,
                r#"SELECT id, vehicle_id, driver_id, purchase_date, vendor_name, location,
                    fuel_type, quantity, unit, price_per_unit, total_amount, currency,
                    odometer, receipt_number, created_at
                    FROM tms_fuel_purchases WHERE vehicle_id = ? ORDER BY purchase_date DESC LIMIT ?"#,
                vid, limit
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                FuelPurchase,
                r#"SELECT id, vehicle_id, driver_id, purchase_date, vendor_name, location,
                    fuel_type, quantity, unit, price_per_unit, total_amount, currency,
                    odometer, receipt_number, created_at
                    FROM tms_fuel_purchases ORDER BY purchase_date DESC LIMIT ?"#,
                limit
            ).fetch_all(&self.pool).await?
        };
        Ok(purchases)
    }

    async fn create_accessorial_charge(&self, charge: &AccessorialCharge) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tms_accessorial_charges (id, code, name, description, default_rate,
                currency, is_active, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            charge.id, charge.code, charge.name, charge.description, charge.default_rate,
            charge.currency, charge.is_active, charge.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_accessorial_charges(&self) -> anyhow::Result<Vec<AccessorialCharge>> {
        let charges = sqlx::query_as!(
            AccessorialCharge,
            r#"SELECT id, code, name, description, default_rate, currency, is_active, created_at
                FROM tms_accessorial_charges WHERE is_active = 1 ORDER BY code"#
        ).fetch_all(&self.pool).await?;
        Ok(charges)
    }
}
