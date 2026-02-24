use crate::models::*;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait FSMRepository {
    async fn create_service_order(&self, order: &ServiceOrder) -> anyhow::Result<()>;
    async fn get_service_order(&self, id: Uuid) -> anyhow::Result<Option<ServiceOrder>>;
    async fn list_service_orders(&self, status: Option<ServiceOrderStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<ServiceOrder>>;
    async fn update_service_order(&self, order: &ServiceOrder) -> anyhow::Result<()>;
    
    async fn create_service_order_task(&self, task: &ServiceOrderTask) -> anyhow::Result<()>;
    async fn list_service_order_tasks(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceOrderTask>>;
    
    async fn create_technician(&self, tech: &Technician) -> anyhow::Result<()>;
    async fn get_technician(&self, id: Uuid) -> anyhow::Result<Option<Technician>>;
    async fn list_technicians(&self, status: Option<TechnicianStatus>) -> anyhow::Result<Vec<Technician>>;
    async fn update_technician(&self, tech: &Technician) -> anyhow::Result<()>;
    
    async fn create_technician_availability(&self, avail: &TechnicianAvailability) -> anyhow::Result<()>;
    async fn list_technician_availability(&self, tech_id: Uuid, date: chrono::DateTime<Utc>) -> anyhow::Result<Vec<TechnicianAvailability>>;
    
    async fn create_service_territory(&self, territory: &ServiceTerritory) -> anyhow::Result<()>;
    async fn list_service_territories(&self) -> anyhow::Result<Vec<ServiceTerritory>>;
    
    async fn create_technician_territory(&self, tt: &TechnicianTerritory) -> anyhow::Result<()>;
    async fn list_technician_territories(&self, tech_id: Uuid) -> anyhow::Result<Vec<TechnicianTerritory>>;
    
    async fn create_service_appointment(&self, appt: &ServiceAppointment) -> anyhow::Result<()>;
    async fn get_service_appointment(&self, id: Uuid) -> anyhow::Result<Option<ServiceAppointment>>;
    async fn list_service_appointments(&self, order_id: Option<Uuid>, tech_id: Option<Uuid>) -> anyhow::Result<Vec<ServiceAppointment>>;
    async fn update_service_appointment(&self, appt: &ServiceAppointment) -> anyhow::Result<()>;
    
    async fn create_service_route(&self, route: &ServiceRoute) -> anyhow::Result<()>;
    async fn get_service_route(&self, id: Uuid) -> anyhow::Result<Option<ServiceRoute>>;
    async fn list_service_routes(&self, tech_id: Option<Uuid>, date: Option<chrono::DateTime<Utc>>) -> anyhow::Result<Vec<ServiceRoute>>;
    async fn update_service_route(&self, route: &ServiceRoute) -> anyhow::Result<()>;
    
    async fn create_service_route_stop(&self, stop: &ServiceRouteStop) -> anyhow::Result<()>;
    async fn list_service_route_stops(&self, route_id: Uuid) -> anyhow::Result<Vec<ServiceRouteStop>>;
    async fn update_service_route_stop(&self, stop: &ServiceRouteStop) -> anyhow::Result<()>;
    
    async fn create_service_part(&self, part: &ServicePart) -> anyhow::Result<()>;
    async fn list_service_parts(&self, order_id: Uuid) -> anyhow::Result<Vec<ServicePart>>;
    
    async fn create_service_time_entry(&self, entry: &ServiceTimeEntry) -> anyhow::Result<()>;
    async fn list_service_time_entries(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceTimeEntry>>;
    
    async fn create_service_expense(&self, expense: &ServiceExpense) -> anyhow::Result<()>;
    async fn list_service_expenses(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceExpense>>;
    
    async fn create_service_checklist(&self, checklist: &ServiceChecklist) -> anyhow::Result<()>;
    async fn list_service_checklists(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceChecklist>>;
    
    async fn create_service_checklist_item(&self, item: &ServiceChecklistItem) -> anyhow::Result<()>;
    async fn list_service_checklist_items(&self, checklist_id: Uuid) -> anyhow::Result<Vec<ServiceChecklistItem>>;
    async fn update_service_checklist_item(&self, item: &ServiceChecklistItem) -> anyhow::Result<()>;
    
    async fn create_service_contract(&self, contract: &ServiceContract) -> anyhow::Result<()>;
    async fn get_service_contract(&self, id: Uuid) -> anyhow::Result<Option<ServiceContract>>;
    async fn list_service_contracts(&self, customer_id: Uuid) -> anyhow::Result<Vec<ServiceContract>>;
    async fn update_service_contract(&self, contract: &ServiceContract) -> anyhow::Result<()>;
    
    async fn create_dispatch_rule(&self, rule: &DispatchRule) -> anyhow::Result<()>;
    async fn list_dispatch_rules(&self) -> anyhow::Result<Vec<DispatchRule>>;
    
    async fn create_technician_skill(&self, skill: &TechnicianSkill) -> anyhow::Result<()>;
    async fn list_technician_skills(&self) -> anyhow::Result<Vec<TechnicianSkill>>;
    
    async fn create_technician_skill_assignment(&self, tsa: &TechnicianSkillAssignment) -> anyhow::Result<()>;
    async fn list_technician_skill_assignments(&self, tech_id: Uuid) -> anyhow::Result<Vec<TechnicianSkillAssignment>>;
}

pub struct SqliteFSMRepository {
    pool: SqlitePool,
}

impl SqliteFSMRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FSMRepository for SqliteFSMRepository {
    async fn create_service_order(&self, order: &ServiceOrder) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_orders (id, order_number, customer_id, contact_name, contact_phone,
                contact_email, service_address, service_city, service_state, service_postal_code,
                service_country, service_lat, service_lng, work_type, priority, status, description,
                asset_id, asset_serial, contract_id, sla_id, assigned_technician_id, scheduled_date,
                scheduled_start, scheduled_end, actual_start, actual_end, travel_time_minutes,
                work_duration_minutes, resolution_notes, customer_signature, customer_rating,
                customer_feedback, total_charges, currency, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            order.id, order.order_number, order.customer_id, order.contact_name, order.contact_phone,
            order.contact_email, order.service_address, order.service_city, order.service_state,
            order.service_postal_code, order.service_country, order.service_lat, order.service_lng,
            order.work_type as _, order.priority as _, order.status as _, order.description,
            order.asset_id, order.asset_serial, order.contract_id, order.sla_id,
            order.assigned_technician_id, order.scheduled_date, order.scheduled_start, order.scheduled_end,
            order.actual_start, order.actual_end, order.travel_time_minutes, order.work_duration_minutes,
            order.resolution_notes, order.customer_signature, order.customer_rating, order.customer_feedback,
            order.total_charges, order.currency, order.created_at, order.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_service_order(&self, id: Uuid) -> anyhow::Result<Option<ServiceOrder>> {
        let order = sqlx::query_as!(
            ServiceOrder,
            r#"SELECT id, order_number, customer_id, contact_name, contact_phone, contact_email,
                service_address, service_city, service_state, service_postal_code, service_country,
                service_lat, service_lng, work_type as "work_type: _", priority as "priority: _",
                status as "status: _", description, asset_id, asset_serial, contract_id, sla_id,
                assigned_technician_id, scheduled_date, scheduled_start, scheduled_end, actual_start,
                actual_end, travel_time_minutes, work_duration_minutes, resolution_notes,
                customer_signature, customer_rating, customer_feedback, total_charges, currency,
                created_at, updated_at FROM fsm_service_orders WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(order)
    }

    async fn list_service_orders(&self, status: Option<ServiceOrderStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<ServiceOrder>> {
        let orders = if let Some(s) = status {
            sqlx::query_as!(
                ServiceOrder,
                r#"SELECT id, order_number, customer_id, contact_name, contact_phone, contact_email,
                    service_address, service_city, service_state, service_postal_code, service_country,
                    service_lat, service_lng, work_type as "work_type: _", priority as "priority: _",
                    status as "status: _", description, asset_id, asset_serial, contract_id, sla_id,
                    assigned_technician_id, scheduled_date, scheduled_start, scheduled_end, actual_start,
                    actual_end, travel_time_minutes, work_duration_minutes, resolution_notes,
                    customer_signature, customer_rating, customer_feedback, total_charges, currency,
                    created_at, updated_at FROM fsm_service_orders WHERE status = ?
                    ORDER BY priority, created_at DESC LIMIT ? OFFSET ?"#,
                    s as _, limit, offset
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                ServiceOrder,
                r#"SELECT id, order_number, customer_id, contact_name, contact_phone, contact_email,
                    service_address, service_city, service_state, service_postal_code, service_country,
                    service_lat, service_lng, work_type as "work_type: _", priority as "priority: _",
                    status as "status: _", description, asset_id, asset_serial, contract_id, sla_id,
                    assigned_technician_id, scheduled_date, scheduled_start, scheduled_end, actual_start,
                    actual_end, travel_time_minutes, work_duration_minutes, resolution_notes,
                    customer_signature, customer_rating, customer_feedback, total_charges, currency,
                    created_at, updated_at FROM fsm_service_orders
                    ORDER BY priority, created_at DESC LIMIT ? OFFSET ?"#,
                    limit, offset
            ).fetch_all(&self.pool).await?
        };
        Ok(orders)
    }

    async fn update_service_order(&self, order: &ServiceOrder) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE fsm_service_orders SET status = ?, assigned_technician_id = ?, scheduled_date = ?,
                scheduled_start = ?, scheduled_end = ?, actual_start = ?, actual_end = ?,
                travel_time_minutes = ?, work_duration_minutes = ?, resolution_notes = ?,
                customer_signature = ?, customer_rating = ?, customer_feedback = ?, total_charges = ?,
                updated_at = ? WHERE id = ?"#,
            order.status as _, order.assigned_technician_id, order.scheduled_date, order.scheduled_start,
            order.scheduled_end, order.actual_start, order.actual_end, order.travel_time_minutes,
            order.work_duration_minutes, order.resolution_notes, order.customer_signature,
            order.customer_rating, order.customer_feedback, order.total_charges, order.updated_at, order.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_order_task(&self, task: &ServiceOrderTask) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_order_tasks (id, service_order_id, task_number, task_type,
                description, estimated_duration_minutes, actual_duration_minutes, status, completed_at,
                notes, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            task.id, task.service_order_id, task.task_number, task.task_type, task.description,
            task.estimated_duration_minutes, task.actual_duration_minutes, task.status, task.completed_at,
            task.notes, task.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_order_tasks(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceOrderTask>> {
        let tasks = sqlx::query_as!(
            ServiceOrderTask,
            r#"SELECT id, service_order_id, task_number, task_type, description,
                estimated_duration_minutes, actual_duration_minutes, status, completed_at, notes, created_at
                FROM fsm_service_order_tasks WHERE service_order_id = ? ORDER BY task_number"#,
            order_id
        ).fetch_all(&self.pool).await?;
        Ok(tasks)
    }

    async fn create_technician(&self, tech: &Technician) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_technicians (id, employee_id, technician_code, first_name, last_name,
                phone, email, status, skills, certifications, home_location_lat, home_location_lng,
                current_location_lat, current_location_lng, current_order_id, service_region, hourly_rate,
                overtime_rate, currency, work_start_time, work_end_time, working_days, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            tech.id, tech.employee_id, tech.technician_code, tech.first_name, tech.last_name,
            tech.phone, tech.email, tech.status as _, tech.skills, tech.certifications,
            tech.home_location_lat, tech.home_location_lng, tech.current_location_lat, tech.current_location_lng,
            tech.current_order_id, tech.service_region, tech.hourly_rate, tech.overtime_rate,
            tech.currency, tech.work_start_time, tech.work_end_time, tech.working_days,
            tech.created_at, tech.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_technician(&self, id: Uuid) -> anyhow::Result<Option<Technician>> {
        let tech = sqlx::query_as!(
            Technician,
            r#"SELECT id, employee_id, technician_code, first_name, last_name, phone, email,
                status as "status: _", skills, certifications, home_location_lat, home_location_lng,
                current_location_lat, current_location_lng, current_order_id, service_region,
                hourly_rate, overtime_rate, currency, work_start_time, work_end_time, working_days,
                created_at, updated_at FROM fsm_technicians WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(tech)
    }

    async fn list_technicians(&self, status: Option<TechnicianStatus>) -> anyhow::Result<Vec<Technician>> {
        let techs = if let Some(s) = status {
            sqlx::query_as!(
                Technician,
                r#"SELECT id, employee_id, technician_code, first_name, last_name, phone, email,
                    status as "status: _", skills, certifications, home_location_lat, home_location_lng,
                    current_location_lat, current_location_lng, current_order_id, service_region,
                    hourly_rate, overtime_rate, currency, work_start_time, work_end_time, working_days,
                    created_at, updated_at FROM fsm_technicians WHERE status = ?"#,
                    s as _
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                Technician,
                r#"SELECT id, employee_id, technician_code, first_name, last_name, phone, email,
                    status as "status: _", skills, certifications, home_location_lat, home_location_lng,
                    current_location_lat, current_location_lng, current_order_id, service_region,
                    hourly_rate, overtime_rate, currency, work_start_time, work_end_time, working_days,
                    created_at, updated_at FROM fsm_technicians"#
            ).fetch_all(&self.pool).await?
        };
        Ok(techs)
    }

    async fn update_technician(&self, tech: &Technician) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE fsm_technicians SET status = ?, current_location_lat = ?, current_location_lng = ?,
                current_order_id = ?, updated_at = ? WHERE id = ?"#,
            tech.status as _, tech.current_location_lat, tech.current_location_lng,
            tech.current_order_id, tech.updated_at, tech.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_technician_availability(&self, avail: &TechnicianAvailability) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_technician_availability (id, technician_id, date, start_time, end_time,
                status, reason, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            avail.id, avail.technician_id, avail.date, avail.start_time, avail.end_time,
            avail.status, avail.reason, avail.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_technician_availability(&self, tech_id: Uuid, date: chrono::DateTime<Utc>) -> anyhow::Result<Vec<TechnicianAvailability>> {
        let avail = sqlx::query_as!(
            TechnicianAvailability,
            r#"SELECT id, technician_id, date, start_time, end_time, status, reason, created_at
                FROM fsm_technician_availability WHERE technician_id = ? AND date(date) = date(?)"""#,
            tech_id, date
        ).fetch_all(&self.pool).await?;
        Ok(avail)
    }

    async fn create_service_territory(&self, territory: &ServiceTerritory) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_territories (id, territory_code, name, description,
                parent_territory_id, boundary_type, boundary_data, manager_id, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            territory.id, territory.territory_code, territory.name, territory.description,
            territory.parent_territory_id, territory.boundary_type, territory.boundary_data,
            territory.manager_id, territory.is_active, territory.created_at, territory.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_territories(&self) -> anyhow::Result<Vec<ServiceTerritory>> {
        let territories = sqlx::query_as!(
            ServiceTerritory,
            r#"SELECT id, territory_code, name, description, parent_territory_id, boundary_type,
                boundary_data, manager_id, is_active, created_at, updated_at
                FROM fsm_service_territories WHERE is_active = 1 ORDER BY name"#
        ).fetch_all(&self.pool).await?;
        Ok(territories)
    }

    async fn create_technician_territory(&self, tt: &TechnicianTerritory) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_technician_territories (id, technician_id, territory_id, is_primary,
                effective_date, expiry_date, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            tt.id, tt.technician_id, tt.territory_id, tt.is_primary, tt.effective_date, tt.expiry_date, tt.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_technician_territories(&self, tech_id: Uuid) -> anyhow::Result<Vec<TechnicianTerritory>> {
        let tts = sqlx::query_as!(
            TechnicianTerritory,
            r#"SELECT id, technician_id, territory_id, is_primary, effective_date, expiry_date, created_at
                FROM fsm_technician_territories WHERE technician_id = ?"#,
            tech_id
        ).fetch_all(&self.pool).await?;
        Ok(tts)
    }

    async fn create_service_appointment(&self, appt: &ServiceAppointment) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_appointments (id, appointment_number, service_order_id,
                technician_id, scheduled_start, scheduled_end, actual_start, actual_end, status,
                confirmation_status, reminder_sent, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            appt.id, appt.appointment_number, appt.service_order_id, appt.technician_id,
            appt.scheduled_start, appt.scheduled_end, appt.actual_start, appt.actual_end,
            appt.status, appt.confirmation_status, appt.reminder_sent, appt.notes,
            appt.created_at, appt.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_service_appointment(&self, id: Uuid) -> anyhow::Result<Option<ServiceAppointment>> {
        let appt = sqlx::query_as!(
            ServiceAppointment,
            r#"SELECT id, appointment_number, service_order_id, technician_id, scheduled_start,
                scheduled_end, actual_start, actual_end, status, confirmation_status, reminder_sent,
                notes, created_at, updated_at FROM fsm_service_appointments WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(appt)
    }

    async fn list_service_appointments(&self, order_id: Option<Uuid>, tech_id: Option<Uuid>) -> anyhow::Result<Vec<ServiceAppointment>> {
        let appts = sqlx::query_as!(
            ServiceAppointment,
            r#"SELECT id, appointment_number, service_order_id, technician_id, scheduled_start,
                scheduled_end, actual_start, actual_end, status, confirmation_status, reminder_sent,
                notes, created_at, updated_at FROM fsm_service_appointments ORDER BY scheduled_start"#
        ).fetch_all(&self.pool).await?;
        Ok(appts)
    }

    async fn update_service_appointment(&self, appt: &ServiceAppointment) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE fsm_service_appointments SET status = ?, actual_start = ?, actual_end = ?,
                confirmation_status = ?, reminder_sent = ?, notes = ?, updated_at = ? WHERE id = ?"#,
            appt.status, appt.actual_start, appt.actual_end, appt.confirmation_status,
            appt.reminder_sent, appt.notes, appt.updated_at, appt.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_route(&self, route: &ServiceRoute) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_routes (id, route_number, technician_id, route_date, status,
                total_appointments, completed_appointments, total_distance, total_duration_minutes,
                optimization_score, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            route.id, route.route_number, route.technician_id, route.route_date, route.status,
            route.total_appointments, route.completed_appointments, route.total_distance,
            route.total_duration_minutes, route.optimization_score, route.created_at, route.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_service_route(&self, id: Uuid) -> anyhow::Result<Option<ServiceRoute>> {
        let route = sqlx::query_as!(
            ServiceRoute,
            r#"SELECT id, route_number, technician_id, route_date, status, total_appointments,
                completed_appointments, total_distance, total_duration_minutes, optimization_score,
                created_at, updated_at FROM fsm_service_routes WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(route)
    }

    async fn list_service_routes(&self, tech_id: Option<Uuid>, date: Option<chrono::DateTime<Utc>>) -> anyhow::Result<Vec<ServiceRoute>> {
        let routes = sqlx::query_as!(
            ServiceRoute,
            r#"SELECT id, route_number, technician_id, route_date, status, total_appointments,
                completed_appointments, total_distance, total_duration_minutes, optimization_score,
                created_at, updated_at FROM fsm_service_routes ORDER BY route_date DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(routes)
    }

    async fn update_service_route(&self, route: &ServiceRoute) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE fsm_service_routes SET status = ?, completed_appointments = ?, total_distance = ?,
                total_duration_minutes = ?, optimization_score = ?, updated_at = ? WHERE id = ?"#,
            route.status, route.completed_appointments, route.total_distance, route.total_duration_minutes,
            route.optimization_score, route.updated_at, route.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_route_stop(&self, stop: &ServiceRouteStop) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_route_stops (id, route_id, appointment_id, stop_sequence,
                planned_arrival, actual_arrival, planned_departure, actual_departure, travel_distance,
                travel_time_minutes, status, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            stop.id, stop.route_id, stop.appointment_id, stop.stop_sequence, stop.planned_arrival,
            stop.actual_arrival, stop.planned_departure, stop.actual_departure, stop.travel_distance,
            stop.travel_time_minutes, stop.status, stop.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_route_stops(&self, route_id: Uuid) -> anyhow::Result<Vec<ServiceRouteStop>> {
        let stops = sqlx::query_as!(
            ServiceRouteStop,
            r#"SELECT id, route_id, appointment_id, stop_sequence, planned_arrival, actual_arrival,
                planned_departure, actual_departure, travel_distance, travel_time_minutes, status, created_at
                FROM fsm_service_route_stops WHERE route_id = ? ORDER BY stop_sequence"#,
            route_id
        ).fetch_all(&self.pool).await?;
        Ok(stops)
    }

    async fn update_service_route_stop(&self, stop: &ServiceRouteStop) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE fsm_service_route_stops SET actual_arrival = ?, actual_departure = ?, status = ?
                WHERE id = ?"#,
            stop.actual_arrival, stop.actual_departure, stop.status, stop.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_part(&self, part: &ServicePart) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_parts (id, service_order_id, product_id, quantity, unit_price,
                total_price, currency, disposition, returned, notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            part.id, part.service_order_id, part.product_id, part.quantity, part.unit_price,
            part.total_price, part.currency, part.disposition, part.returned, part.notes, part.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_parts(&self, order_id: Uuid) -> anyhow::Result<Vec<ServicePart>> {
        let parts = sqlx::query_as!(
            ServicePart,
            r#"SELECT id, service_order_id, product_id, quantity, unit_price, total_price, currency,
                disposition, returned, notes, created_at FROM fsm_service_parts WHERE service_order_id = ?"#,
            order_id
        ).fetch_all(&self.pool).await?;
        Ok(parts)
    }

    async fn create_service_time_entry(&self, entry: &ServiceTimeEntry) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_time_entries (id, service_order_id, technician_id, entry_date,
                start_time, end_time, hours, overtime_hours, work_type, billable, rate, total_amount,
                currency, notes, approved, approved_by, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            entry.id, entry.service_order_id, entry.technician_id, entry.entry_date, entry.start_time,
            entry.end_time, entry.hours, entry.overtime_hours, entry.work_type, entry.billable,
            entry.rate, entry.total_amount, entry.currency, entry.notes, entry.approved,
            entry.approved_by, entry.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_time_entries(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceTimeEntry>> {
        let entries = sqlx::query_as!(
            ServiceTimeEntry,
            r#"SELECT id, service_order_id, technician_id, entry_date, start_time, end_time, hours,
                overtime_hours, work_type, billable, rate, total_amount, currency, notes, approved,
                approved_by, created_at FROM fsm_service_time_entries WHERE service_order_id = ?"#,
            order_id
        ).fetch_all(&self.pool).await?;
        Ok(entries)
    }

    async fn create_service_expense(&self, expense: &ServiceExpense) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_expenses (id, service_order_id, technician_id, expense_type,
                amount, currency, description, receipt_url, approved, approved_by, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            expense.id, expense.service_order_id, expense.technician_id, expense.expense_type,
            expense.amount, expense.currency, expense.description, expense.receipt_url,
            expense.approved, expense.approved_by, expense.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_expenses(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceExpense>> {
        let expenses = sqlx::query_as!(
            ServiceExpense,
            r#"SELECT id, service_order_id, technician_id, expense_type, amount, currency,
                description, receipt_url, approved, approved_by, created_at
                FROM fsm_service_expenses WHERE service_order_id = ?"#,
            order_id
        ).fetch_all(&self.pool).await?;
        Ok(expenses)
    }

    async fn create_service_checklist(&self, checklist: &ServiceChecklist) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_checklists (id, service_order_id, checklist_type, name,
                completed, completed_at, completed_by, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            checklist.id, checklist.service_order_id, checklist.checklist_type, checklist.name,
            checklist.completed, checklist.completed_at, checklist.completed_by, checklist.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_checklists(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceChecklist>> {
        let checklists = sqlx::query_as!(
            ServiceChecklist,
            r#"SELECT id, service_order_id, checklist_type, name, completed, completed_at,
                completed_by, created_at FROM fsm_service_checklists WHERE service_order_id = ?"#,
            order_id
        ).fetch_all(&self.pool).await?;
        Ok(checklists)
    }

    async fn create_service_checklist_item(&self, item: &ServiceChecklistItem) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_checklist_items (id, checklist_id, item_number, description,
                is_required, response_type, response_value, notes, completed, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            item.id, item.checklist_id, item.item_number, item.description, item.is_required,
            item.response_type, item.response_value, item.notes, item.completed, item.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_checklist_items(&self, checklist_id: Uuid) -> anyhow::Result<Vec<ServiceChecklistItem>> {
        let items = sqlx::query_as!(
            ServiceChecklistItem,
            r#"SELECT id, checklist_id, item_number, description, is_required, response_type,
                response_value, notes, completed, created_at FROM fsm_service_checklist_items
                WHERE checklist_id = ? ORDER BY item_number"#,
            checklist_id
        ).fetch_all(&self.pool).await?;
        Ok(items)
    }

    async fn update_service_checklist_item(&self, item: &ServiceChecklistItem) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE fsm_service_checklist_items SET response_value = ?, notes = ?, completed = ?
                WHERE id = ?"#,
            item.response_value, item.notes, item.completed, item.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_contract(&self, contract: &ServiceContract) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_service_contracts (id, contract_number, customer_id, contract_name,
                start_date, end_date, contract_type, coverage_type, response_time_hours,
                resolution_time_hours, visit_limit, visits_used, coverage_hours, coverage_days,
                annual_fee, currency, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            contract.id, contract.contract_number, contract.customer_id, contract.contract_name,
            contract.start_date, contract.end_date, contract.contract_type, contract.coverage_type,
            contract.response_time_hours, contract.resolution_time_hours, contract.visit_limit,
            contract.visits_used, contract.coverage_hours, contract.coverage_days, contract.annual_fee,
            contract.currency, contract.is_active, contract.created_at, contract.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_service_contract(&self, id: Uuid) -> anyhow::Result<Option<ServiceContract>> {
        let contract = sqlx::query_as!(
            ServiceContract,
            r#"SELECT id, contract_number, customer_id, contract_name, start_date, end_date,
                contract_type, coverage_type, response_time_hours, resolution_time_hours,
                visit_limit, visits_used, coverage_hours, coverage_days, annual_fee, currency,
                is_active, created_at, updated_at FROM fsm_service_contracts WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(contract)
    }

    async fn list_service_contracts(&self, customer_id: Uuid) -> anyhow::Result<Vec<ServiceContract>> {
        let contracts = sqlx::query_as!(
            ServiceContract,
            r#"SELECT id, contract_number, customer_id, contract_name, start_date, end_date,
                contract_type, coverage_type, response_time_hours, resolution_time_hours,
                visit_limit, visits_used, coverage_hours, coverage_days, annual_fee, currency,
                is_active, created_at, updated_at FROM fsm_service_contracts
                WHERE customer_id = ? AND is_active = 1"#,
            customer_id
        ).fetch_all(&self.pool).await?;
        Ok(contracts)
    }

    async fn update_service_contract(&self, contract: &ServiceContract) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE fsm_service_contracts SET visits_used = ?, is_active = ?, updated_at = ? WHERE id = ?"#,
            contract.visits_used, contract.is_active, contract.updated_at, contract.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_dispatch_rule(&self, rule: &DispatchRule) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_dispatch_rules (id, rule_name, description, priority, conditions,
                actions, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.id, rule.rule_name, rule.description, rule.priority, rule.conditions,
            rule.actions, rule.is_active, rule.created_at, rule.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_dispatch_rules(&self) -> anyhow::Result<Vec<DispatchRule>> {
        let rules = sqlx::query_as!(
            DispatchRule,
            r#"SELECT id, rule_name, description, priority, conditions, actions, is_active,
                created_at, updated_at FROM fsm_dispatch_rules WHERE is_active = 1 ORDER BY priority"#
        ).fetch_all(&self.pool).await?;
        Ok(rules)
    }

    async fn create_technician_skill(&self, skill: &TechnicianSkill) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_technician_skills (id, skill_code, skill_name, category, description,
                proficiency_levels, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            skill.id, skill.skill_code, skill.skill_name, skill.category, skill.description,
            skill.proficiency_levels, skill.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_technician_skills(&self) -> anyhow::Result<Vec<TechnicianSkill>> {
        let skills = sqlx::query_as!(
            TechnicianSkill,
            r#"SELECT id, skill_code, skill_name, category, description, proficiency_levels, created_at
                FROM fsm_technician_skills ORDER BY category, skill_name"#
        ).fetch_all(&self.pool).await?;
        Ok(skills)
    }

    async fn create_technician_skill_assignment(&self, tsa: &TechnicianSkillAssignment) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO fsm_technician_skill_assignments (id, technician_id, skill_id,
                proficiency_level, certified, certified_date, expiry_date, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            tsa.id, tsa.technician_id, tsa.skill_id, tsa.proficiency_level, tsa.certified,
            tsa.certified_date, tsa.expiry_date, tsa.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_technician_skill_assignments(&self, tech_id: Uuid) -> anyhow::Result<Vec<TechnicianSkillAssignment>> {
        let tsas = sqlx::query_as!(
            TechnicianSkillAssignment,
            r#"SELECT id, technician_id, skill_id, proficiency_level, certified, certified_date,
                expiry_date, created_at FROM fsm_technician_skill_assignments WHERE technician_id = ?"#,
            tech_id
        ).fetch_all(&self.pool).await?;
        Ok(tsas)
    }
}
