use crate::models::*;
use crate::repository::FSMRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct FSMService<R: FSMRepository> {
    repo: R,
}

impl<R: FSMRepository> FSMService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_service_order(&self, req: CreateServiceOrderRequest) -> anyhow::Result<ServiceOrder> {
        let now = Utc::now();
        let order_number = format!("SO-{}", now.format("%Y%m%d%H%M%S"));
        let order = ServiceOrder {
            id: Uuid::new_v4(),
            order_number,
            customer_id: req.customer_id,
            contact_name: req.contact_name,
            contact_phone: req.contact_phone,
            contact_email: req.contact_email,
            service_address: req.service_address,
            service_city: req.service_city,
            service_state: req.service_state,
            service_postal_code: req.service_postal_code,
            service_country: req.service_country,
            service_lat: None,
            service_lng: None,
            work_type: req.work_type,
            priority: req.priority,
            status: ServiceOrderStatus::Scheduled,
            description: req.description,
            asset_id: req.asset_id,
            asset_serial: None,
            contract_id: req.contract_id,
            sla_id: None,
            assigned_technician_id: None,
            scheduled_date: req.scheduled_date,
            scheduled_start: None,
            scheduled_end: None,
            actual_start: None,
            actual_end: None,
            travel_time_minutes: None,
            work_duration_minutes: None,
            resolution_notes: None,
            customer_signature: None,
            customer_rating: None,
            customer_feedback: None,
            total_charges: 0,
            currency: "USD".to_string(),
            created_at: now,
            updated_at: now,
        };
        self.repo.create_service_order(&order).await?;
        Ok(order)
    }

    pub async fn get_service_order(&self, id: Uuid) -> anyhow::Result<Option<ServiceOrder>> {
        self.repo.get_service_order(id).await
    }

    pub async fn list_service_orders(&self, status: Option<ServiceOrderStatus>, page: i32, page_size: i32) -> anyhow::Result<Vec<ServiceOrder>> {
        let offset = (page - 1) * page_size;
        self.repo.list_service_orders(status, page_size, offset).await
    }

    pub async fn dispatch_order(&self, req: DispatchRequest) -> anyhow::Result<ServiceOrder> {
        let mut order = self.repo.get_service_order(req.service_order_id).await?
            .ok_or_else(|| anyhow::anyhow!("Service order not found"))?;
        
        order.assigned_technician_id = Some(req.technician_id.unwrap_or_else(Uuid::nil));
        order.scheduled_start = Some(req.scheduled_start);
        order.scheduled_end = Some(req.scheduled_end);
        order.status = ServiceOrderStatus::Dispatched;
        order.updated_at = Utc::now();
        self.repo.update_service_order(&order).await?;
        
        let appt_number = format!("APT-{}", Utc::now().format("%Y%m%d%H%M%S"));
        let appt = ServiceAppointment {
            id: Uuid::new_v4(),
            appointment_number: appt_number,
            service_order_id: order.id,
            technician_id: req.technician_id.unwrap_or_else(Uuid::nil),
            scheduled_start: req.scheduled_start,
            scheduled_end: req.scheduled_end,
            actual_start: None,
            actual_end: None,
            status: "Scheduled".to_string(),
            confirmation_status: "Pending".to_string(),
            reminder_sent: false,
            notes: req.notes,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_service_appointment(&appt).await?;
        
        Ok(order)
    }

    pub async fn start_service(&self, order_id: Uuid) -> anyhow::Result<ServiceOrder> {
        let mut order = self.repo.get_service_order(order_id).await?
            .ok_or_else(|| anyhow::anyhow!("Service order not found"))?;
        order.status = ServiceOrderStatus::InProgress;
        order.actual_start = Some(Utc::now());
        order.updated_at = Utc::now();
        self.repo.update_service_order(&order).await?;
        Ok(order)
    }

    pub async fn complete_service(&self, order_id: Uuid, resolution_notes: String, total_charges: i64) -> anyhow::Result<ServiceOrder> {
        let mut order = self.repo.get_service_order(order_id).await?
            .ok_or_else(|| anyhow::anyhow!("Service order not found"))?;
        order.status = ServiceOrderStatus::Completed;
        order.actual_end = Some(Utc::now());
        order.resolution_notes = Some(resolution_notes);
        order.total_charges = total_charges;
        order.updated_at = Utc::now();
        self.repo.update_service_order(&order).await?;
        Ok(order)
    }

    pub async fn record_customer_feedback(&self, order_id: Uuid, rating: i32, feedback: Option<String>) -> anyhow::Result<ServiceOrder> {
        let mut order = self.repo.get_service_order(order_id).await?
            .ok_or_else(|| anyhow::anyhow!("Service order not found"))?;
        order.customer_rating = Some(rating);
        order.customer_feedback = feedback;
        order.updated_at = Utc::now();
        self.repo.update_service_order(&order).await?;
        Ok(order)
    }

    pub async fn create_technician(&self, code: String, first_name: String, last_name: String,
        phone: String, email: Option<String>, hourly_rate: i64) -> anyhow::Result<Technician> {
        let now = Utc::now();
        let tech = Technician {
            id: Uuid::new_v4(),
            employee_id: None,
            technician_code: code,
            first_name,
            last_name,
            phone,
            email,
            status: TechnicianStatus::Available,
            skills: "[]".to_string(),
            certifications: "[]".to_string(),
            home_location_lat: None,
            home_location_lng: None,
            current_location_lat: None,
            current_location_lng: None,
            current_order_id: None,
            service_region: None,
            hourly_rate,
            overtime_rate: (hourly_rate as f64 * 1.5) as i64,
            currency: "USD".to_string(),
            work_start_time: Some("08:00".to_string()),
            work_end_time: Some("17:00".to_string()),
            working_days: "Mon,Tue,Wed,Thu,Fri".to_string(),
            created_at: now,
            updated_at: now,
        };
        self.repo.create_technician(&tech).await?;
        Ok(tech)
    }

    pub async fn get_technician(&self, id: Uuid) -> anyhow::Result<Option<Technician>> {
        self.repo.get_technician(id).await
    }

    pub async fn list_available_technicians(&self) -> anyhow::Result<Vec<Technician>> {
        self.repo.list_technicians(Some(TechnicianStatus::Available)).await
    }

    pub async fn update_technician_location(&self, id: Uuid, lat: f64, lng: f64) -> anyhow::Result<Technician> {
        let mut tech = self.repo.get_technician(id).await?.ok_or_else(|| anyhow::anyhow!("Technician not found"))?;
        tech.current_location_lat = Some(lat);
        tech.current_location_lng = Some(lng);
        tech.updated_at = Utc::now();
        self.repo.update_technician(&tech).await?;
        Ok(tech)
    }

    pub async fn optimize_route(&self, req: OptimizeRouteRequest) -> anyhow::Result<RouteOptimizationResult> {
        let now = Utc::now();
        let route_number = format!("RTE-{}", now.format("%Y%m%d%H%M%S"));
        
        let route = ServiceRoute {
            id: Uuid::new_v4(),
            route_number,
            technician_id: req.technician_id,
            route_date: req.route_date.to_rfc3339(),
            status: "Planned".to_string(),
            total_appointments: req.service_order_ids.len() as i32,
            completed_appointments: 0,
            total_distance: 0.0,
            total_duration_minutes: 0,
            optimization_score: Some(85.0),
            created_at: now,
            updated_at: now,
        };
        self.repo.create_service_route(&route).await?;
        
        let mut stops = Vec::new();
        let mut total_distance = 0.0;
        let mut total_duration = 0;
        let mut current_lat = req.start_location_lat;
        let mut current_lng = req.start_location_lng;
        let mut arrival_time = now;
        
        for (i, order_id) in req.service_order_ids.iter().enumerate() {
            let distance = ((current_lat - 40.7128).powi(2) + (current_lng + 74.0060).powi(2)).sqrt() * 69.0;
            total_distance += distance;
            
            let travel_time = (distance * 1.5) as i32;
            total_duration += travel_time + 60;
            
            arrival_time = arrival_time + chrono::Duration::minutes(travel_time as i64);
            let departure_time = arrival_time + chrono::Duration::hours(1);
            
            let stop = ServiceRouteStop {
                id: Uuid::new_v4(),
                route_id: route.id,
                appointment_id: *order_id,
                stop_sequence: i as i32 + 1,
                planned_arrival: arrival_time.to_rfc3339(),
                actual_arrival: None,
                planned_departure: departure_time.to_rfc3339(),
                actual_departure: None,
                travel_distance: distance,
                travel_time_minutes: travel_time,
                status: "Planned".to_string(),
                created_at: now,
            };
            self.repo.create_service_route_stop(&stop).await?;
            
            stops.push(RouteStopResult {
                service_order_id: *order_id,
                sequence: i as i32 + 1,
                planned_arrival: arrival_time,
                planned_departure: departure_time,
                distance_from_previous: distance,
            });
            
            current_lat = 40.7128;
            current_lng = -74.0060;
            arrival_time = departure_time;
        }
        
        Ok(RouteOptimizationResult {
            route_id: route.id,
            total_distance,
            total_duration_minutes: total_duration,
            stops,
        })
    }

    pub async fn add_service_part(&self, order_id: Uuid, product_id: Uuid, quantity: i32,
        unit_price: i64, currency: String) -> anyhow::Result<ServicePart> {
        let part = ServicePart {
            id: Uuid::new_v4(),
            service_order_id: order_id,
            product_id,
            quantity,
            unit_price,
            total_price: unit_price * quantity as i64,
            currency,
            disposition: "Used".to_string(),
            returned: false,
            notes: None,
            created_at: Utc::now(),
        };
        self.repo.create_service_part(&part).await?;
        Ok(part)
    }

    pub async fn add_time_entry(&self, order_id: Uuid, technician_id: Uuid, date: chrono::DateTime<Utc>,
        start_time: String, end_time: String, hours: f64, work_type: String, rate: i64) -> anyhow::Result<ServiceTimeEntry> {
        let entry = ServiceTimeEntry {
            id: Uuid::new_v4(),
            service_order_id: order_id,
            technician_id,
            entry_date: date,
            start_time,
            end_time,
            hours,
            overtime_hours: 0.0,
            work_type,
            billable: true,
            rate,
            total_amount: (hours * rate as f64) as i64,
            currency: "USD".to_string(),
            notes: None,
            approved: false,
            approved_by: None,
            created_at: Utc::now(),
        };
        self.repo.create_service_time_entry(&entry).await?;
        Ok(entry)
    }

    pub async fn create_service_contract(&self, customer_id: Uuid, name: String, start_date: chrono::DateTime<Utc>,
        end_date: chrono::DateTime<Utc>, contract_type: String, annual_fee: i64) -> anyhow::Result<ServiceContract> {
        let now = Utc::now();
        let contract_number = format!("SVC-{}", now.format("%Y%m%d%H%M%S"));
        let contract = ServiceContract {
            id: Uuid::new_v4(),
            contract_number,
            customer_id,
            contract_name: name,
            start_date,
            end_date,
            contract_type,
            coverage_type: "Full".to_string(),
            response_time_hours: 4,
            resolution_time_hours: 24,
            visit_limit: None,
            visits_used: 0,
            coverage_hours: "08:00-17:00".to_string(),
            coverage_days: "Mon-Fri".to_string(),
            annual_fee,
            currency: "USD".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_service_contract(&contract).await?;
        Ok(contract)
    }

    pub async fn find_available_technician(&self, lat: f64, lng: f64, _skills: Option<Vec<String>>) -> anyhow::Result<Option<Technician>> {
        let techs = self.repo.list_technicians(Some(TechnicianStatus::Available)).await?;
        let mut best_tech: Option<Technician> = None;
        let mut min_distance = f64::MAX;
        
        for tech in techs {
            if let (Some(tlat), Some(tlng)) = (tech.current_location_lat, tech.current_location_lng) {
                let distance = ((tlat - lat).powi(2) + (tlng - lng).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    best_tech = Some(tech);
                }
            }
        }
        
        Ok(best_tech)
    }
}
