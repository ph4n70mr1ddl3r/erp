use crate::models::*;
use crate::repository::TMSRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct TMSService<R: TMSRepository> {
    repo: R,
}

impl<R: TMSRepository> TMSService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_vehicle(&self, req: CreateVehicleRequest) -> anyhow::Result<Vehicle> {
        let now = Utc::now();
        let vehicle = Vehicle {
            id: Uuid::new_v4(),
            vehicle_number: req.vehicle_number,
            vehicle_type: req.vehicle_type,
            license_plate: req.license_plate,
            vin: req.vin,
            make: req.make,
            model: req.model,
            year: req.year,
            capacity_weight: req.capacity_weight,
            capacity_volume: req.capacity_volume,
            capacity_unit: req.capacity_unit,
            fuel_type: req.fuel_type,
            status: VehicleStatus::Available,
            current_location_lat: None,
            current_location_lng: None,
            odometer: 0.0,
            last_maintenance: None,
            next_maintenance: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_vehicle(&vehicle).await?;
        Ok(vehicle)
    }

    pub async fn get_vehicle(&self, id: Uuid) -> anyhow::Result<Option<Vehicle>> {
        self.repo.get_vehicle(id).await
    }

    pub async fn list_vehicles(&self, page: i32, page_size: i32) -> anyhow::Result<Vec<Vehicle>> {
        let offset = (page - 1) * page_size;
        self.repo.list_vehicles(page_size, offset).await
    }

    pub async fn update_vehicle_status(&self, id: Uuid, status: VehicleStatus) -> anyhow::Result<Vehicle> {
        let mut vehicle = self.repo.get_vehicle(id).await?.ok_or_else(|| anyhow::anyhow!("Vehicle not found"))?;
        vehicle.status = status;
        vehicle.updated_at = Utc::now();
        self.repo.update_vehicle(&vehicle).await?;
        Ok(vehicle)
    }

    pub async fn create_driver(&self, req: CreateDriverRequest) -> anyhow::Result<Driver> {
        let now = Utc::now();
        let driver = Driver {
            id: Uuid::new_v4(),
            employee_id: req.employee_id,
            driver_code: req.driver_code,
            first_name: req.first_name,
            last_name: req.last_name,
            phone: req.phone,
            email: req.email,
            license_number: req.license_number,
            license_class: req.license_class,
            license_expiry: req.license_expiry,
            status: DriverStatus::Available,
            current_vehicle_id: None,
            current_location_lat: None,
            current_location_lng: None,
            hours_driven_today: 0.0,
            hours_available: 11.0,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_driver(&driver).await?;
        Ok(driver)
    }

    pub async fn get_driver(&self, id: Uuid) -> anyhow::Result<Option<Driver>> {
        self.repo.get_driver(id).await
    }

    pub async fn list_drivers(&self, page: i32, page_size: i32) -> anyhow::Result<Vec<Driver>> {
        let offset = (page - 1) * page_size;
        self.repo.list_drivers(page_size, offset).await
    }

    pub async fn create_load(&self, req: CreateLoadRequest) -> anyhow::Result<Load> {
        let now = Utc::now();
        let load_number = format!("LD-{}", now.format("%Y%m%d%H%M%S"));
        let load = Load {
            id: Uuid::new_v4(),
            load_number,
            carrier_id: req.carrier_id,
            driver_id: None,
            vehicle_id: None,
            status: LoadStatus::Planned,
            origin_name: req.origin_name,
            origin_street: req.origin_street,
            origin_city: req.origin_city,
            origin_state: req.origin_state,
            origin_postal_code: req.origin_postal_code,
            origin_country: req.origin_country,
            origin_lat: None,
            origin_lng: None,
            destination_name: req.destination_name,
            destination_street: req.destination_street,
            destination_city: req.destination_city,
            destination_state: req.destination_state,
            destination_postal_code: req.destination_postal_code,
            destination_country: req.destination_country,
            destination_lat: None,
            destination_lng: None,
            planned_pickup: req.planned_pickup,
            actual_pickup: None,
            planned_delivery: req.planned_delivery,
            actual_delivery: None,
            total_weight: 0.0,
            total_volume: 0.0,
            total_pallets: 0,
            total_pieces: 0,
            freight_charge: 0,
            fuel_surcharge: 0,
            accessorial_charges: 0,
            total_charge: 0,
            currency: "USD".to_string(),
            distance_miles: None,
            notes: req.notes,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_load(&load).await?;
        Ok(load)
    }

    pub async fn get_load(&self, id: Uuid) -> anyhow::Result<Option<Load>> {
        self.repo.get_load(id).await
    }

    pub async fn list_loads(&self, page: i32, page_size: i32) -> anyhow::Result<Vec<Load>> {
        let offset = (page - 1) * page_size;
        self.repo.list_loads(page_size, offset).await
    }

    pub async fn assign_load(&self, load_id: Uuid, driver_id: Uuid, vehicle_id: Uuid) -> anyhow::Result<Load> {
        let mut load = self.repo.get_load(load_id).await?.ok_or_else(|| anyhow::anyhow!("Load not found"))?;
        load.driver_id = Some(driver_id);
        load.vehicle_id = Some(vehicle_id);
        load.status = LoadStatus::Ready;
        load.updated_at = Utc::now();
        self.repo.update_load(&load).await?;
        Ok(load)
    }

    pub async fn dispatch_load(&self, load_id: Uuid) -> anyhow::Result<Load> {
        let mut load = self.repo.get_load(load_id).await?.ok_or_else(|| anyhow::anyhow!("Load not found"))?;
        if load.status != LoadStatus::Ready {
            return Err(anyhow::anyhow!("Load must be ready before dispatch"));
        }
        load.status = LoadStatus::InTransit;
        load.actual_pickup = Some(Utc::now());
        load.updated_at = Utc::now();
        self.repo.update_load(&load).await?;
        Ok(load)
    }

    pub async fn deliver_load(&self, load_id: Uuid) -> anyhow::Result<Load> {
        let mut load = self.repo.get_load(load_id).await?.ok_or_else(|| anyhow::anyhow!("Load not found"))?;
        load.status = LoadStatus::Delivered;
        load.actual_delivery = Some(Utc::now());
        load.updated_at = Utc::now();
        self.repo.update_load(&load).await?;
        Ok(load)
    }

    pub async fn optimize_route(&self, req: RouteOptimizationRequest) -> anyhow::Result<RouteOptimizationResult> {
        let mut stops = req.stops.clone();
        stops.sort_by(|a, b| {
            let dist_a = ((a.lat - req.origin_lat).powi(2) + (a.lng - req.origin_lng).powi(2)).sqrt();
            let dist_b = ((b.lat - req.origin_lat).powi(2) + (b.lng - req.origin_lng).powi(2)).sqrt();
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        let mut optimized = Vec::new();
        let mut total_distance = 0.0;
        let mut current_lat = req.origin_lat;
        let mut current_lng = req.origin_lng;
        let mut arrival_time = Utc::now();

        for (i, stop) in stops.iter().enumerate() {
            let dist = ((stop.lat - current_lat).powi(2) + (stop.lng - current_lng).powi(2)).sqrt();
            total_distance += dist * 69.0;
            
            let travel_minutes = (dist * 69.0 * 1.5) as i32;
            arrival_time += chrono::Duration::minutes(travel_minutes as i64);
            let departure_time = arrival_time + chrono::Duration::minutes(stop.duration_minutes as i64);

            optimized.push(OptimizedStop {
                stop_id: stop.id,
                sequence: i as i32 + 1,
                arrival_time,
                departure_time,
                distance_from_previous: dist * 69.0,
            });

            current_lat = stop.lat;
            current_lng = stop.lng;
            arrival_time = departure_time;
        }

        let total_duration = optimized.iter().map(|s| {
            let stop = stops.iter().find(|st| st.id == s.stop_id).unwrap();
            (s.distance_from_previous * 1.5) as i32 + stop.duration_minutes
        }).sum::<i32>();

        Ok(RouteOptimizationResult {
            total_distance,
            total_duration_minutes: total_duration,
            optimized_stops: optimized,
            total_weight: stops.iter().map(|s| s.weight).sum(),
            total_volume: stops.iter().map(|s| s.volume).sum(),
        })
    }

    pub async fn audit_freight_invoice(&self, invoice_id: Uuid) -> anyhow::Result<FreightInvoice> {
        let mut invoice = self.repo.get_freight_invoice(invoice_id).await?
            .ok_or_else(|| anyhow::anyhow!("Invoice not found"))?;
        
        let variance = invoice.total_amount - invoice.expected_amount;
        invoice.variance_amount = variance;
        
        if variance.abs() > invoice.expected_amount / 10 {
            invoice.status = FreightAuditStatus::Disputed;
            invoice.variance_reason = Some("Variance exceeds 10% threshold".to_string());
        } else {
            invoice.status = FreightAuditStatus::Validated;
        }
        
        invoice.updated_at = Utc::now();
        self.repo.update_freight_invoice(&invoice).await?;
        Ok(invoice)
    }

    pub async fn approve_freight_invoice(&self, invoice_id: Uuid, approved_by: Uuid) -> anyhow::Result<FreightInvoice> {
        let mut invoice = self.repo.get_freight_invoice(invoice_id).await?
            .ok_or_else(|| anyhow::anyhow!("Invoice not found"))?;
        
        if invoice.status != FreightAuditStatus::Validated {
            return Err(anyhow::anyhow!("Invoice must be validated before approval"));
        }
        
        invoice.status = FreightAuditStatus::Approved;
        invoice.approved_by = Some(approved_by);
        invoice.approved_at = Some(Utc::now());
        invoice.updated_at = Utc::now();
        self.repo.update_freight_invoice(&invoice).await?;
        Ok(invoice)
    }

    pub async fn record_fuel_purchase(&self, vehicle_id: Uuid, driver_id: Option<Uuid>,
        vendor: String, location: String, fuel_type: String, quantity: f64, unit: String,
        price_per_unit: i64, currency: String, odometer: f64, receipt: Option<String>) -> anyhow::Result<FuelPurchase> {
        let purchase = FuelPurchase {
            id: Uuid::new_v4(),
            vehicle_id: Some(vehicle_id),
            driver_id,
            purchase_date: Utc::now(),
            vendor_name: vendor,
            location,
            fuel_type,
            quantity,
            unit,
            price_per_unit,
            total_amount: (quantity * price_per_unit as f64) as i64,
            currency,
            odometer,
            receipt_number: receipt,
            created_at: Utc::now(),
        };
        self.repo.create_fuel_purchase(&purchase).await?;
        Ok(purchase)
    }
}
