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
        sqlx::query(
            r#"INSERT INTO tms_vehicles (id, vehicle_number, vehicle_type, license_plate, vin, make, model, year,
                capacity_weight, capacity_volume, capacity_unit, fuel_type, status, current_location_lat,
                current_location_lng, odometer, last_maintenance, next_maintenance, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(vehicle.id)
        .bind(&vehicle.vehicle_number)
        .bind(&vehicle.vehicle_type)
        .bind(&vehicle.license_plate)
        .bind(&vehicle.vin)
        .bind(&vehicle.make)
        .bind(&vehicle.model)
        .bind(vehicle.year)
        .bind(vehicle.capacity_weight)
        .bind(vehicle.capacity_volume)
        .bind(&vehicle.capacity_unit)
        .bind(&vehicle.fuel_type)
        .bind(vehicle.status)
        .bind(vehicle.current_location_lat)
        .bind(vehicle.current_location_lng)
        .bind(vehicle.odometer)
        .bind(vehicle.last_maintenance)
        .bind(vehicle.next_maintenance)
        .bind(vehicle.created_at)
        .bind(vehicle.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_vehicle(&self, id: Uuid) -> anyhow::Result<Option<Vehicle>> {
        let vehicle = sqlx::query_as::<_, Vehicle>(
            r#"SELECT id, vehicle_number, vehicle_type, license_plate, vin, make, model, year,
                capacity_weight, capacity_volume, capacity_unit, fuel_type, status,
                current_location_lat, current_location_lng, odometer, last_maintenance,
                next_maintenance, created_at, updated_at FROM tms_vehicles WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool).await?;
        Ok(vehicle)
    }

    async fn list_vehicles(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Vehicle>> {
        let vehicles = sqlx::query_as::<_, Vehicle>(
            r#"SELECT id, vehicle_number, vehicle_type, license_plate, vin, make, model, year,
                capacity_weight, capacity_volume, capacity_unit, fuel_type, status,
                current_location_lat, current_location_lng, odometer, last_maintenance,
                next_maintenance, created_at, updated_at FROM tms_vehicles
                ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool).await?;
        Ok(vehicles)
    }

    async fn update_vehicle(&self, vehicle: &Vehicle) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tms_vehicles SET vehicle_number = ?, vehicle_type = ?, license_plate = ?,
                vin = ?, make = ?, model = ?, year = ?, capacity_weight = ?, capacity_volume = ?,
                capacity_unit = ?, fuel_type = ?, status = ?, current_location_lat = ?,
                current_location_lng = ?, odometer = ?, last_maintenance = ?, next_maintenance = ?,
                updated_at = ? WHERE id = ?"#,
        )
        .bind(&vehicle.vehicle_number)
        .bind(&vehicle.vehicle_type)
        .bind(&vehicle.license_plate)
        .bind(&vehicle.vin)
        .bind(&vehicle.make)
        .bind(&vehicle.model)
        .bind(vehicle.year)
        .bind(vehicle.capacity_weight)
        .bind(vehicle.capacity_volume)
        .bind(&vehicle.capacity_unit)
        .bind(&vehicle.fuel_type)
        .bind(vehicle.status)
        .bind(vehicle.current_location_lat)
        .bind(vehicle.current_location_lng)
        .bind(vehicle.odometer)
        .bind(vehicle.last_maintenance)
        .bind(vehicle.next_maintenance)
        .bind(vehicle.updated_at)
        .bind(vehicle.id)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_driver(&self, driver: &Driver) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_drivers (id, employee_id, driver_code, first_name, last_name, phone,
                email, license_number, license_class, license_expiry, status, current_vehicle_id,
                current_location_lat, current_location_lng, hours_driven_today, hours_available,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(driver.id)
        .bind(driver.employee_id)
        .bind(&driver.driver_code)
        .bind(&driver.first_name)
        .bind(&driver.last_name)
        .bind(&driver.phone)
        .bind(&driver.email)
        .bind(&driver.license_number)
        .bind(&driver.license_class)
        .bind(driver.license_expiry)
        .bind(driver.status)
        .bind(driver.current_vehicle_id)
        .bind(driver.current_location_lat)
        .bind(driver.current_location_lng)
        .bind(driver.hours_driven_today)
        .bind(driver.hours_available)
        .bind(driver.created_at)
        .bind(driver.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_driver(&self, id: Uuid) -> anyhow::Result<Option<Driver>> {
        let driver = sqlx::query_as::<_, Driver>(
            r#"SELECT id, employee_id, driver_code, first_name, last_name, phone, email,
                license_number, license_class, license_expiry, status,
                current_vehicle_id, current_location_lat, current_location_lng,
                hours_driven_today, hours_available, created_at, updated_at
                FROM tms_drivers WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool).await?;
        Ok(driver)
    }

    async fn list_drivers(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Driver>> {
        let drivers = sqlx::query_as::<_, Driver>(
            r#"SELECT id, employee_id, driver_code, first_name, last_name, phone, email,
                license_number, license_class, license_expiry, status,
                current_vehicle_id, current_location_lat, current_location_lng,
                hours_driven_today, hours_available, created_at, updated_at
                FROM tms_drivers ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool).await?;
        Ok(drivers)
    }

    async fn update_driver(&self, driver: &Driver) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tms_drivers SET driver_code = ?, first_name = ?, last_name = ?, phone = ?,
                email = ?, license_number = ?, license_class = ?, license_expiry = ?, status = ?,
                current_vehicle_id = ?, current_location_lat = ?, current_location_lng = ?,
                hours_driven_today = ?, hours_available = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&driver.driver_code)
        .bind(&driver.first_name)
        .bind(&driver.last_name)
        .bind(&driver.phone)
        .bind(&driver.email)
        .bind(&driver.license_number)
        .bind(&driver.license_class)
        .bind(driver.license_expiry)
        .bind(driver.status)
        .bind(driver.current_vehicle_id)
        .bind(driver.current_location_lat)
        .bind(driver.current_location_lng)
        .bind(driver.hours_driven_today)
        .bind(driver.hours_available)
        .bind(driver.updated_at)
        .bind(driver.id)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_carrier_contract(&self, contract: &CarrierContract) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_carrier_contracts (id, carrier_id, contract_number, contract_name,
                start_date, end_date, payment_terms, fuel_surcharge_percent, accessorial_rates,
                volume_commitment, volume_discount_tiers, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(contract.id)
        .bind(contract.carrier_id)
        .bind(&contract.contract_number)
        .bind(&contract.contract_name)
        .bind(contract.start_date)
        .bind(contract.end_date)
        .bind(&contract.payment_terms)
        .bind(contract.fuel_surcharge_percent)
        .bind(&contract.accessorial_rates)
        .bind(contract.volume_commitment)
        .bind(&contract.volume_discount_tiers)
        .bind(contract.is_active)
        .bind(contract.created_at)
        .bind(contract.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_carrier_contract(&self, id: Uuid) -> anyhow::Result<Option<CarrierContract>> {
        let contract = sqlx::query_as::<_, CarrierContract>(
            r#"SELECT id, carrier_id, contract_number, contract_name, start_date, end_date,
                payment_terms, fuel_surcharge_percent, accessorial_rates, volume_commitment,
                volume_discount_tiers, is_active, created_at, updated_at
                FROM tms_carrier_contracts WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool).await?;
        Ok(contract)
    }

    async fn list_carrier_contracts(&self, carrier_id: Uuid) -> anyhow::Result<Vec<CarrierContract>> {
        let contracts = sqlx::query_as::<_, CarrierContract>(
            r#"SELECT id, carrier_id, contract_number, contract_name, start_date, end_date,
                payment_terms, fuel_surcharge_percent, accessorial_rates, volume_commitment,
                volume_discount_tiers, is_active, created_at, updated_at
                FROM tms_carrier_contracts WHERE carrier_id = ? ORDER BY created_at DESC"#,
        )
        .bind(carrier_id)
        .fetch_all(&self.pool).await?;
        Ok(contracts)
    }

    async fn create_freight_rate(&self, rate: &FreightRate) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_freight_rates (id, contract_id, origin_zone, destination_zone,
                service_type, rate_type, base_rate, per_mile_rate, per_kg_rate, per_pallet_rate,
                min_charge, max_charge, currency, effective_date, expiry_date, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(rate.id)
        .bind(rate.contract_id)
        .bind(&rate.origin_zone)
        .bind(&rate.destination_zone)
        .bind(&rate.service_type)
        .bind(&rate.rate_type)
        .bind(rate.base_rate)
        .bind(rate.per_mile_rate)
        .bind(rate.per_kg_rate)
        .bind(rate.per_pallet_rate)
        .bind(rate.min_charge)
        .bind(rate.max_charge)
        .bind(&rate.currency)
        .bind(rate.effective_date)
        .bind(rate.expiry_date)
        .bind(rate.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_freight_rates(&self, contract_id: Uuid) -> anyhow::Result<Vec<FreightRate>> {
        let rates = sqlx::query_as::<_, FreightRate>(
            r#"SELECT id, contract_id, origin_zone, destination_zone, service_type, rate_type,
                base_rate, per_mile_rate, per_kg_rate, per_pallet_rate, min_charge, max_charge,
                currency, effective_date, expiry_date, created_at
                FROM tms_freight_rates WHERE contract_id = ?"#,
        )
        .bind(contract_id)
        .fetch_all(&self.pool).await?;
        Ok(rates)
    }

    async fn create_load(&self, load: &Load) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_loads (id, load_number, carrier_id, driver_id, vehicle_id, status,
                origin_name, origin_street, origin_city, origin_state, origin_postal_code, origin_country,
                origin_lat, origin_lng, destination_name, destination_street, destination_city,
                destination_state, destination_postal_code, destination_country, destination_lat,
                destination_lng, planned_pickup, actual_pickup, planned_delivery, actual_delivery,
                total_weight, total_volume, total_pallets, total_pieces, freight_charge, fuel_surcharge,
                accessorial_charges, total_charge, currency, distance_miles, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(load.id)
        .bind(&load.load_number)
        .bind(load.carrier_id)
        .bind(load.driver_id)
        .bind(load.vehicle_id)
        .bind(load.status)
        .bind(&load.origin_name)
        .bind(&load.origin_street)
        .bind(&load.origin_city)
        .bind(&load.origin_state)
        .bind(&load.origin_postal_code)
        .bind(&load.origin_country)
        .bind(load.origin_lat)
        .bind(load.origin_lng)
        .bind(&load.destination_name)
        .bind(&load.destination_street)
        .bind(&load.destination_city)
        .bind(&load.destination_state)
        .bind(&load.destination_postal_code)
        .bind(&load.destination_country)
        .bind(load.destination_lat)
        .bind(load.destination_lng)
        .bind(load.planned_pickup)
        .bind(load.actual_pickup)
        .bind(load.planned_delivery)
        .bind(load.actual_delivery)
        .bind(load.total_weight)
        .bind(load.total_volume)
        .bind(load.total_pallets)
        .bind(load.total_pieces)
        .bind(load.freight_charge)
        .bind(load.fuel_surcharge)
        .bind(load.accessorial_charges)
        .bind(load.total_charge)
        .bind(&load.currency)
        .bind(load.distance_miles)
        .bind(&load.notes)
        .bind(load.created_at)
        .bind(load.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_load(&self, id: Uuid) -> anyhow::Result<Option<Load>> {
        let load = sqlx::query_as::<_, Load>(
            r#"SELECT id, load_number, carrier_id, driver_id, vehicle_id, status,
                origin_name, origin_street, origin_city, origin_state, origin_postal_code,
                origin_country, origin_lat, origin_lng, destination_name, destination_street,
                destination_city, destination_state, destination_postal_code, destination_country,
                destination_lat, destination_lng, planned_pickup, actual_pickup, planned_delivery,
                actual_delivery, total_weight, total_volume, total_pallets, total_pieces,
                freight_charge, fuel_surcharge, accessorial_charges, total_charge, currency,
                distance_miles, notes, created_at, updated_at FROM tms_loads WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool).await?;
        Ok(load)
    }

    async fn list_loads(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Load>> {
        let loads = sqlx::query_as::<_, Load>(
            r#"SELECT id, load_number, carrier_id, driver_id, vehicle_id, status,
                origin_name, origin_street, origin_city, origin_state, origin_postal_code,
                origin_country, origin_lat, origin_lng, destination_name, destination_street,
                destination_city, destination_state, destination_postal_code, destination_country,
                destination_lat, destination_lng, planned_pickup, actual_pickup, planned_delivery,
                actual_delivery, total_weight, total_volume, total_pallets, total_pieces,
                freight_charge, fuel_surcharge, accessorial_charges, total_charge, currency,
                distance_miles, notes, created_at, updated_at FROM tms_loads
                ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool).await?;
        Ok(loads)
    }

    async fn update_load(&self, load: &Load) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tms_loads SET carrier_id = ?, driver_id = ?, vehicle_id = ?, status = ?,
                actual_pickup = ?, actual_delivery = ?, total_weight = ?, total_volume = ?,
                total_pallets = ?, total_pieces = ?, freight_charge = ?, fuel_surcharge = ?,
                accessorial_charges = ?, total_charge = ?, distance_miles = ?, notes = ?,
                updated_at = ? WHERE id = ?"#,
        )
        .bind(load.carrier_id)
        .bind(load.driver_id)
        .bind(load.vehicle_id)
        .bind(load.status)
        .bind(load.actual_pickup)
        .bind(load.actual_delivery)
        .bind(load.total_weight)
        .bind(load.total_volume)
        .bind(load.total_pallets)
        .bind(load.total_pieces)
        .bind(load.freight_charge)
        .bind(load.fuel_surcharge)
        .bind(load.accessorial_charges)
        .bind(load.total_charge)
        .bind(load.distance_miles)
        .bind(&load.notes)
        .bind(load.updated_at)
        .bind(load.id)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_load_stop(&self, stop: &LoadStop) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_load_stops (id, load_id, stop_number, stop_type, location_name,
                street, city, state, postal_code, country, lat, lng, planned_arrival,
                actual_arrival, planned_departure, actual_departure, appointment_number,
                instructions, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(stop.id)
        .bind(stop.load_id)
        .bind(stop.stop_number)
        .bind(&stop.stop_type)
        .bind(&stop.location_name)
        .bind(&stop.street)
        .bind(&stop.city)
        .bind(&stop.state)
        .bind(&stop.postal_code)
        .bind(&stop.country)
        .bind(stop.lat)
        .bind(stop.lng)
        .bind(stop.planned_arrival)
        .bind(stop.actual_arrival)
        .bind(stop.planned_departure)
        .bind(stop.actual_departure)
        .bind(&stop.appointment_number)
        .bind(&stop.instructions)
        .bind(stop.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_load_stops(&self, load_id: Uuid) -> anyhow::Result<Vec<LoadStop>> {
        let stops = sqlx::query_as::<_, LoadStop>(
            r#"SELECT id, load_id, stop_number, stop_type, location_name, street, city, state,
                postal_code, country, lat, lng, planned_arrival, actual_arrival, planned_departure,
                actual_departure, appointment_number, instructions, created_at
                FROM tms_load_stops WHERE load_id = ? ORDER BY stop_number"#,
        )
        .bind(load_id)
        .fetch_all(&self.pool).await?;
        Ok(stops)
    }

    async fn create_load_item(&self, item: &LoadItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_load_items (id, load_id, stop_id, shipment_id, product_id,
                description, quantity, weight, volume, pallets, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(item.id)
        .bind(item.load_id)
        .bind(item.stop_id)
        .bind(item.shipment_id)
        .bind(item.product_id)
        .bind(&item.description)
        .bind(item.quantity)
        .bind(item.weight)
        .bind(item.volume)
        .bind(item.pallets)
        .bind(item.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_load_items(&self, load_id: Uuid) -> anyhow::Result<Vec<LoadItem>> {
        let items = sqlx::query_as::<_, LoadItem>(
            r#"SELECT id, load_id, stop_id, shipment_id, product_id, description, quantity,
                weight, volume, pallets, created_at FROM tms_load_items WHERE load_id = ?"#,
        )
        .bind(load_id)
        .fetch_all(&self.pool).await?;
        Ok(items)
    }

    async fn create_route(&self, route: &Route) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_routes (id, route_number, route_name, driver_id, vehicle_id, status,
                planned_start, actual_start, planned_end, actual_end, total_distance,
                total_duration_minutes, total_stops, completed_stops, total_weight,
                optimization_score, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(route.id)
        .bind(&route.route_number)
        .bind(&route.route_name)
        .bind(route.driver_id)
        .bind(route.vehicle_id)
        .bind(route.status)
        .bind(route.planned_start)
        .bind(route.actual_start)
        .bind(route.planned_end)
        .bind(route.actual_end)
        .bind(route.total_distance)
        .bind(route.total_duration_minutes)
        .bind(route.total_stops)
        .bind(route.completed_stops)
        .bind(route.total_weight)
        .bind(route.optimization_score)
        .bind(route.created_at)
        .bind(route.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_route(&self, id: Uuid) -> anyhow::Result<Option<Route>> {
        let route = sqlx::query_as::<_, Route>(
            r#"SELECT id, route_number, route_name, driver_id, vehicle_id, status,
                planned_start, actual_start, planned_end, actual_end, total_distance,
                total_duration_minutes, total_stops, completed_stops, total_weight,
                optimization_score, created_at, updated_at FROM tms_routes WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool).await?;
        Ok(route)
    }

    async fn list_routes(&self, limit: i32, offset: i32) -> anyhow::Result<Vec<Route>> {
        let routes = sqlx::query_as::<_, Route>(
            r#"SELECT id, route_number, route_name, driver_id, vehicle_id, status,
                planned_start, actual_start, planned_end, actual_end, total_distance,
                total_duration_minutes, total_stops, completed_stops, total_weight,
                optimization_score, created_at, updated_at FROM tms_routes
                ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool).await?;
        Ok(routes)
    }

    async fn update_route(&self, route: &Route) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tms_routes SET driver_id = ?, vehicle_id = ?, status = ?, actual_start = ?,
                actual_end = ?, total_distance = ?, total_duration_minutes = ?, completed_stops = ?,
                optimization_score = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(route.driver_id)
        .bind(route.vehicle_id)
        .bind(route.status)
        .bind(route.actual_start)
        .bind(route.actual_end)
        .bind(route.total_distance)
        .bind(route.total_duration_minutes)
        .bind(route.completed_stops)
        .bind(route.optimization_score)
        .bind(route.updated_at)
        .bind(route.id)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_route_stop(&self, stop: &RouteStop) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_route_stops (id, route_id, stop_sequence, stop_type, load_id,
                location_name, street, city, state, postal_code, country, lat, lng,
                planned_arrival, actual_arrival, planned_duration_minutes, status, notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(stop.id)
        .bind(stop.route_id)
        .bind(stop.stop_sequence)
        .bind(&stop.stop_type)
        .bind(stop.load_id)
        .bind(&stop.location_name)
        .bind(&stop.street)
        .bind(&stop.city)
        .bind(&stop.state)
        .bind(&stop.postal_code)
        .bind(&stop.country)
        .bind(stop.lat)
        .bind(stop.lng)
        .bind(stop.planned_arrival)
        .bind(stop.actual_arrival)
        .bind(stop.planned_duration_minutes)
        .bind(&stop.status)
        .bind(&stop.notes)
        .bind(stop.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_route_stops(&self, route_id: Uuid) -> anyhow::Result<Vec<RouteStop>> {
        let stops = sqlx::query_as::<_, RouteStop>(
            r#"SELECT id, route_id, stop_sequence, stop_type, load_id, location_name, street,
                city, state, postal_code, country, lat, lng, planned_arrival, actual_arrival,
                planned_duration_minutes, status, notes, created_at
                FROM tms_route_stops WHERE route_id = ? ORDER BY stop_sequence"#,
        )
        .bind(route_id)
        .fetch_all(&self.pool).await?;
        Ok(stops)
    }

    async fn create_freight_invoice(&self, invoice: &FreightInvoice) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_freight_invoices (id, invoice_number, carrier_id, load_id,
                invoice_date, due_date, status, base_freight, fuel_surcharge, accessorial_charges,
                tax, total_amount, currency, expected_amount, variance_amount, variance_reason,
                approved_by, approved_at, paid_at, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(invoice.id)
        .bind(&invoice.invoice_number)
        .bind(invoice.carrier_id)
        .bind(invoice.load_id)
        .bind(invoice.invoice_date)
        .bind(invoice.due_date)
        .bind(invoice.status)
        .bind(invoice.base_freight)
        .bind(invoice.fuel_surcharge)
        .bind(invoice.accessorial_charges)
        .bind(invoice.tax)
        .bind(invoice.total_amount)
        .bind(&invoice.currency)
        .bind(invoice.expected_amount)
        .bind(invoice.variance_amount)
        .bind(&invoice.variance_reason)
        .bind(invoice.approved_by)
        .bind(invoice.approved_at)
        .bind(invoice.paid_at)
        .bind(invoice.created_at)
        .bind(invoice.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_freight_invoice(&self, id: Uuid) -> anyhow::Result<Option<FreightInvoice>> {
        let invoice = sqlx::query_as::<_, FreightInvoice>(
            r#"SELECT id, invoice_number, carrier_id, load_id, invoice_date, due_date,
                status, base_freight, fuel_surcharge, accessorial_charges, tax,
                total_amount, currency, expected_amount, variance_amount, variance_reason,
                approved_by, approved_at, paid_at, created_at, updated_at
                FROM tms_freight_invoices WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool).await?;
        Ok(invoice)
    }

    async fn list_freight_invoices(&self, status: Option<FreightAuditStatus>) -> anyhow::Result<Vec<FreightInvoice>> {
        let invoices = if let Some(s) = status {
            sqlx::query_as::<_, FreightInvoice>(
                r#"SELECT id, invoice_number, carrier_id, load_id, invoice_date, due_date,
                    status, base_freight, fuel_surcharge, accessorial_charges, tax,
                    total_amount, currency, expected_amount, variance_amount, variance_reason,
                    approved_by, approved_at, paid_at, created_at, updated_at
                    FROM tms_freight_invoices WHERE status = ? ORDER BY created_at DESC"#,
            )
            .bind(s)
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, FreightInvoice>(
                r#"SELECT id, invoice_number, carrier_id, load_id, invoice_date, due_date,
                    status, base_freight, fuel_surcharge, accessorial_charges, tax,
                    total_amount, currency, expected_amount, variance_amount, variance_reason,
                    approved_by, approved_at, paid_at, created_at, updated_at
                    FROM tms_freight_invoices ORDER BY created_at DESC"#,
            )
            .fetch_all(&self.pool).await?
        };
        Ok(invoices)
    }

    async fn update_freight_invoice(&self, invoice: &FreightInvoice) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tms_freight_invoices SET status = ?, variance_amount = ?, variance_reason = ?,
                approved_by = ?, approved_at = ?, paid_at = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(invoice.status)
        .bind(invoice.variance_amount)
        .bind(&invoice.variance_reason)
        .bind(invoice.approved_by)
        .bind(invoice.approved_at)
        .bind(invoice.paid_at)
        .bind(invoice.updated_at)
        .bind(invoice.id)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_freight_invoice_line(&self, line: &FreightInvoiceLine) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_freight_invoice_lines (id, invoice_id, line_number, charge_type,
                description, quantity, unit, rate, amount, expected_amount, variance, currency, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(line.id)
        .bind(line.invoice_id)
        .bind(line.line_number)
        .bind(&line.charge_type)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(&line.unit)
        .bind(line.rate)
        .bind(line.amount)
        .bind(line.expected_amount)
        .bind(line.variance)
        .bind(&line.currency)
        .bind(line.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_freight_invoice_lines(&self, invoice_id: Uuid) -> anyhow::Result<Vec<FreightInvoiceLine>> {
        let lines = sqlx::query_as::<_, FreightInvoiceLine>(
            r#"SELECT id, invoice_id, line_number, charge_type, description, quantity, unit,
                rate, amount, expected_amount, variance, currency, created_at
                FROM tms_freight_invoice_lines WHERE invoice_id = ? ORDER BY line_number"#,
        )
        .bind(invoice_id)
        .fetch_all(&self.pool).await?;
        Ok(lines)
    }

    async fn create_fuel_purchase(&self, purchase: &FuelPurchase) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_fuel_purchases (id, vehicle_id, driver_id, purchase_date,
                vendor_name, location, fuel_type, quantity, unit, price_per_unit, total_amount,
                currency, odometer, receipt_number, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(purchase.id)
        .bind(purchase.vehicle_id)
        .bind(purchase.driver_id)
        .bind(purchase.purchase_date)
        .bind(&purchase.vendor_name)
        .bind(&purchase.location)
        .bind(&purchase.fuel_type)
        .bind(purchase.quantity)
        .bind(&purchase.unit)
        .bind(purchase.price_per_unit)
        .bind(purchase.total_amount)
        .bind(&purchase.currency)
        .bind(purchase.odometer)
        .bind(&purchase.receipt_number)
        .bind(purchase.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_fuel_purchases(&self, vehicle_id: Option<Uuid>, limit: i32) -> anyhow::Result<Vec<FuelPurchase>> {
        let purchases = if let Some(vid) = vehicle_id {
            sqlx::query_as::<_, FuelPurchase>(
                r#"SELECT id, vehicle_id, driver_id, purchase_date, vendor_name, location,
                    fuel_type, quantity, unit, price_per_unit, total_amount, currency,
                    odometer, receipt_number, created_at
                    FROM tms_fuel_purchases WHERE vehicle_id = ? ORDER BY purchase_date DESC LIMIT ?"#,
            )
            .bind(vid)
            .bind(limit)
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, FuelPurchase>(
                r#"SELECT id, vehicle_id, driver_id, purchase_date, vendor_name, location,
                    fuel_type, quantity, unit, price_per_unit, total_amount, currency,
                    odometer, receipt_number, created_at
                    FROM tms_fuel_purchases ORDER BY purchase_date DESC LIMIT ?"#,
            )
            .bind(limit)
            .fetch_all(&self.pool).await?
        };
        Ok(purchases)
    }

    async fn create_accessorial_charge(&self, charge: &AccessorialCharge) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tms_accessorial_charges (id, code, name, description, default_rate,
                currency, is_active, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(charge.id)
        .bind(&charge.code)
        .bind(&charge.name)
        .bind(&charge.description)
        .bind(charge.default_rate)
        .bind(&charge.currency)
        .bind(charge.is_active)
        .bind(charge.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_accessorial_charges(&self) -> anyhow::Result<Vec<AccessorialCharge>> {
        let charges = sqlx::query_as::<_, AccessorialCharge>(
            r#"SELECT id, code, name, description, default_rate, currency, is_active, created_at
                FROM tms_accessorial_charges WHERE is_active = 1 ORDER BY code"#,
        )
        .fetch_all(&self.pool).await?;
        Ok(charges)
    }
}
