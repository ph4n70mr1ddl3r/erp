use crate::models::*;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{FromRow, SqlitePool};
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
    async fn list_technician_availability(&self, tech_id: Uuid, _date: chrono::DateTime<Utc>) -> anyhow::Result<Vec<TechnicianAvailability>>;
    
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
    async fn list_service_routes(&self, tech_id: Option<Uuid>, _date: Option<chrono::DateTime<Utc>>) -> anyhow::Result<Vec<ServiceRoute>>;
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
        sqlx::query(
            r#"INSERT INTO fsm_service_orders (id, order_number, customer_id, contact_name, contact_phone,
                contact_email, service_address, service_city, service_state, service_postal_code,
                service_country, service_lat, service_lng, work_type, priority, status, description,
                asset_id, asset_serial, contract_id, sla_id, assigned_technician_id, scheduled_date,
                scheduled_start, scheduled_end, actual_start, actual_end, travel_time_minutes,
                work_duration_minutes, resolution_notes, customer_signature, customer_rating,
                customer_feedback, total_charges, currency, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(order.id.to_string())
        .bind(&order.order_number)
        .bind(order.customer_id.to_string())
        .bind(&order.contact_name)
        .bind(&order.contact_phone)
        .bind(&order.contact_email)
        .bind(&order.service_address)
        .bind(&order.service_city)
        .bind(&order.service_state)
        .bind(&order.service_postal_code)
        .bind(&order.service_country)
        .bind(order.service_lat)
        .bind(order.service_lng)
        .bind(format!("{:?}", order.work_type))
        .bind(format!("{:?}", order.priority))
        .bind(format!("{:?}", order.status))
        .bind(&order.description)
        .bind(order.asset_id.map(|id| id.to_string()))
        .bind(&order.asset_serial)
        .bind(order.contract_id.map(|id| id.to_string()))
        .bind(order.sla_id.map(|id| id.to_string()))
        .bind(order.assigned_technician_id.map(|id| id.to_string()))
        .bind(order.scheduled_date.map(|d| d.to_rfc3339()))
        .bind(order.scheduled_start.map(|d| d.to_rfc3339()))
        .bind(order.scheduled_end.map(|d| d.to_rfc3339()))
        .bind(order.actual_start.map(|d| d.to_rfc3339()))
        .bind(order.actual_end.map(|d| d.to_rfc3339()))
        .bind(order.travel_time_minutes)
        .bind(order.work_duration_minutes)
        .bind(&order.resolution_notes)
        .bind(&order.customer_signature)
        .bind(order.customer_rating)
        .bind(&order.customer_feedback)
        .bind(order.total_charges)
        .bind(&order.currency)
        .bind(order.created_at.to_rfc3339())
        .bind(order.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_service_order(&self, id: Uuid) -> anyhow::Result<Option<ServiceOrder>> {
        let row: Option<ServiceOrderRow> = sqlx::query_as::<_, ServiceOrderRow>(
            r#"SELECT id, order_number, customer_id, contact_name, contact_phone,
                contact_email, service_address, service_city, service_state, service_postal_code,
                service_country, service_lat, service_lng, work_type, priority, status, description,
                asset_id, asset_serial, contract_id, sla_id, assigned_technician_id, scheduled_date,
                scheduled_start, scheduled_end, actual_start, actual_end, travel_time_minutes,
                work_duration_minutes, resolution_notes, customer_signature, customer_rating,
                customer_feedback, total_charges, currency, created_at, updated_at
                FROM fsm_service_orders WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_service_orders(&self, status: Option<ServiceOrderStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<ServiceOrder>> {
        let rows: Vec<ServiceOrderRow> = if let Some(s) = status {
            sqlx::query_as::<_, ServiceOrderRow>(
                r#"SELECT id, order_number, customer_id, contact_name, contact_phone,
                    contact_email, service_address, service_city, service_state, service_postal_code,
                    service_country, service_lat, service_lng, work_type, priority, status, description,
                    asset_id, asset_serial, contract_id, sla_id, assigned_technician_id, scheduled_date,
                    scheduled_start, scheduled_end, actual_start, actual_end, travel_time_minutes,
                    work_duration_minutes, resolution_notes, customer_signature, customer_rating,
                    customer_feedback, total_charges, currency, created_at, updated_at
                    FROM fsm_service_orders WHERE status = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            )
            .bind(format!("{:?}", s))
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, ServiceOrderRow>(
                r#"SELECT id, order_number, customer_id, contact_name, contact_phone,
                    contact_email, service_address, service_city, service_state, service_postal_code,
                    service_country, service_lat, service_lng, work_type, priority, status, description,
                    asset_id, asset_serial, contract_id, sla_id, assigned_technician_id, scheduled_date,
                    scheduled_start, scheduled_end, actual_start, actual_end, travel_time_minutes,
                    work_duration_minutes, resolution_notes, customer_signature, customer_rating,
                    customer_feedback, total_charges, currency, created_at, updated_at
                    FROM fsm_service_orders ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool).await?
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_service_order(&self, order: &ServiceOrder) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE fsm_service_orders SET status = ?, assigned_technician_id = ?,
                scheduled_date = ?, scheduled_start = ?, scheduled_end = ?, actual_start = ?,
                actual_end = ?, resolution_notes = ?, customer_rating = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(format!("{:?}", order.status))
        .bind(order.assigned_technician_id.map(|id| id.to_string()))
        .bind(order.scheduled_date.map(|d| d.to_rfc3339()))
        .bind(order.scheduled_start.map(|d| d.to_rfc3339()))
        .bind(order.scheduled_end.map(|d| d.to_rfc3339()))
        .bind(order.actual_start.map(|d| d.to_rfc3339()))
        .bind(order.actual_end.map(|d| d.to_rfc3339()))
        .bind(&order.resolution_notes)
        .bind(order.customer_rating)
        .bind(order.updated_at.to_rfc3339())
        .bind(order.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_order_task(&self, task: &ServiceOrderTask) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_order_tasks (id, service_order_id, task_number, task_type, description,
                estimated_duration_minutes, actual_duration_minutes, status, completed_at, notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(task.id.to_string())
        .bind(task.service_order_id.to_string())
        .bind(task.task_number)
        .bind(&task.task_type)
        .bind(&task.description)
        .bind(task.estimated_duration_minutes)
        .bind(task.actual_duration_minutes)
        .bind(&task.status)
        .bind(task.completed_at.map(|d| d.to_rfc3339()))
        .bind(&task.notes)
        .bind(task.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_order_tasks(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceOrderTask>> {
        let rows: Vec<ServiceOrderTaskRow> = sqlx::query_as::<_, ServiceOrderTaskRow>(
            r#"SELECT id, service_order_id, task_number, task_type, description, estimated_duration_minutes, 
                actual_duration_minutes, status, completed_at, notes, created_at
                FROM fsm_service_order_tasks WHERE service_order_id = ? ORDER BY task_number"#,
        )
        .bind(order_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_technician(&self, tech: &Technician) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_technicians (id, employee_id, technician_code, first_name, last_name, phone, email,
                status, skills, certifications, home_location_lat, home_location_lng, current_location_lat, current_location_lng,
                current_order_id, service_region, hourly_rate, overtime_rate, currency, work_start_time, work_end_time,
                working_days, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(tech.id.to_string())
        .bind(tech.employee_id.map(|id| id.to_string()))
        .bind(&tech.technician_code)
        .bind(&tech.first_name)
        .bind(&tech.last_name)
        .bind(&tech.phone)
        .bind(&tech.email)
        .bind(format!("{:?}", tech.status))
        .bind(&tech.skills)
        .bind(&tech.certifications)
        .bind(tech.home_location_lat)
        .bind(tech.home_location_lng)
        .bind(tech.current_location_lat)
        .bind(tech.current_location_lng)
        .bind(tech.current_order_id.map(|id| id.to_string()))
        .bind(&tech.service_region)
        .bind(tech.hourly_rate)
        .bind(tech.overtime_rate)
        .bind(&tech.currency)
        .bind(&tech.work_start_time)
        .bind(&tech.work_end_time)
        .bind(&tech.working_days)
        .bind(tech.created_at.to_rfc3339())
        .bind(tech.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_technician(&self, id: Uuid) -> anyhow::Result<Option<Technician>> {
        let row: Option<TechnicianRow> = sqlx::query_as::<_, TechnicianRow>(
            r#"SELECT id, employee_id, technician_code, first_name, last_name, phone, email,
                status, skills, certifications, home_location_lat, home_location_lng, current_location_lat, current_location_lng,
                current_order_id, service_region, hourly_rate, overtime_rate, currency, work_start_time, work_end_time,
                working_days, created_at, updated_at FROM fsm_technicians WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_technicians(&self, status: Option<TechnicianStatus>) -> anyhow::Result<Vec<Technician>> {
        let rows: Vec<TechnicianRow> = if let Some(s) = status {
            sqlx::query_as::<_, TechnicianRow>(
                r#"SELECT id, employee_id, technician_code, first_name, last_name, phone, email,
                    status, skills, certifications, home_location_lat, home_location_lng, current_location_lat, current_location_lng,
                    current_order_id, service_region, hourly_rate, overtime_rate, currency, work_start_time, work_end_time,
                    working_days, created_at, updated_at FROM fsm_technicians WHERE status = ?"#,
            )
            .bind(format!("{:?}", s))
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, TechnicianRow>(
                r#"SELECT id, employee_id, technician_code, first_name, last_name, phone, email,
                    status, skills, certifications, home_location_lat, home_location_lng, current_location_lat, current_location_lng,
                    current_order_id, service_region, hourly_rate, overtime_rate, currency, work_start_time, work_end_time,
                    working_days, created_at, updated_at FROM fsm_technicians ORDER BY created_at DESC"#,
            )
            .fetch_all(&self.pool).await?
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_technician(&self, tech: &Technician) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE fsm_technicians SET email = ?, phone = ?, status = ?,
                current_location_lat = ?, current_location_lng = ?, current_order_id = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&tech.email)
        .bind(&tech.phone)
        .bind(format!("{:?}", tech.status))
        .bind(tech.current_location_lat)
        .bind(tech.current_location_lng)
        .bind(tech.current_order_id.map(|id| id.to_string()))
        .bind(tech.updated_at.to_rfc3339())
        .bind(tech.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_technician_availability(&self, avail: &TechnicianAvailability) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_technician_availability (id, technician_id, date, start_time, end_time,
                status, reason, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(avail.id.to_string())
        .bind(avail.technician_id.to_string())
        .bind(&avail.date)
        .bind(&avail.start_time)
        .bind(&avail.end_time)
        .bind(&avail.status)
        .bind(&avail.reason)
        .bind(avail.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_technician_availability(&self, tech_id: Uuid, _date: chrono::DateTime<Utc>) -> anyhow::Result<Vec<TechnicianAvailability>> {
        let rows: Vec<TechnicianAvailabilityRow> = sqlx::query_as::<_, TechnicianAvailabilityRow>(
            r#"SELECT id, technician_id, date, start_time, end_time, status, reason, created_at 
                FROM fsm_technician_availability WHERE technician_id = ?"#,
        )
        .bind(tech_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_service_territory(&self, territory: &ServiceTerritory) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_territories (id, territory_code, name, description, parent_territory_id,
                boundary_type, boundary_data, manager_id, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(territory.id.to_string())
        .bind(&territory.territory_code)
        .bind(&territory.name)
        .bind(&territory.description)
        .bind(territory.parent_territory_id.map(|id| id.to_string()))
        .bind(&territory.boundary_type)
        .bind(&territory.boundary_data)
        .bind(territory.manager_id.map(|id| id.to_string()))
        .bind(territory.is_active)
        .bind(territory.created_at.to_rfc3339())
        .bind(territory.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_territories(&self) -> anyhow::Result<Vec<ServiceTerritory>> {
        let rows: Vec<ServiceTerritoryRow> = sqlx::query_as::<_, ServiceTerritoryRow>(
            r#"SELECT id, territory_code, name, description, parent_territory_id, boundary_type,
                boundary_data, manager_id, is_active, created_at, updated_at FROM fsm_service_territories"#,
        )
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_technician_territory(&self, tt: &TechnicianTerritory) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_technician_territories (id, technician_id, territory_id, is_primary, effective_date, expiry_date, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(tt.id.to_string())
        .bind(tt.technician_id.to_string())
        .bind(tt.territory_id.to_string())
        .bind(tt.is_primary)
        .bind(tt.effective_date.to_rfc3339())
        .bind(tt.expiry_date.map(|d| d.to_rfc3339()))
        .bind(tt.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_technician_territories(&self, tech_id: Uuid) -> anyhow::Result<Vec<TechnicianTerritory>> {
        let rows: Vec<TechnicianTerritoryRow> = sqlx::query_as::<_, TechnicianTerritoryRow>(
            r#"SELECT id, technician_id, territory_id, is_primary, effective_date, expiry_date, created_at
                FROM fsm_technician_territories WHERE technician_id = ?"#,
        )
        .bind(tech_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_service_appointment(&self, appt: &ServiceAppointment) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_appointments (id, appointment_number, service_order_id, technician_id,
                scheduled_start, scheduled_end, actual_start, actual_end, status, confirmation_status, reminder_sent, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(appt.id.to_string())
        .bind(&appt.appointment_number)
        .bind(appt.service_order_id.to_string())
        .bind(appt.technician_id.to_string())
        .bind(appt.scheduled_start.to_rfc3339())
        .bind(appt.scheduled_end.to_rfc3339())
        .bind(appt.actual_start.map(|d| d.to_rfc3339()))
        .bind(appt.actual_end.map(|d| d.to_rfc3339()))
        .bind(&appt.status)
        .bind(&appt.confirmation_status)
        .bind(appt.reminder_sent)
        .bind(&appt.notes)
        .bind(appt.created_at.to_rfc3339())
        .bind(appt.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_service_appointment(&self, id: Uuid) -> anyhow::Result<Option<ServiceAppointment>> {
        let row: Option<ServiceAppointmentRow> = sqlx::query_as::<_, ServiceAppointmentRow>(
            r#"SELECT id, appointment_number, service_order_id, technician_id, scheduled_start, scheduled_end,
                actual_start, actual_end, status, confirmation_status, reminder_sent, notes,
                created_at, updated_at FROM fsm_service_appointments WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_service_appointments(&self, order_id: Option<Uuid>, tech_id: Option<Uuid>) -> anyhow::Result<Vec<ServiceAppointment>> {
        let mut query = String::from(
            r#"SELECT id, appointment_number, service_order_id, technician_id, scheduled_start, scheduled_end,
                actual_start, actual_end, status, confirmation_status, reminder_sent, notes,
                created_at, updated_at FROM fsm_service_appointments WHERE 1=1"#
        );
        if order_id.is_some() {
            query.push_str(" AND service_order_id = ?");
        }
        if tech_id.is_some() {
            query.push_str(" AND technician_id = ?");
        }
        query.push_str(" ORDER BY scheduled_start DESC");
        
        let mut q = sqlx::query_as::<_, ServiceAppointmentRow>(&query);
        if let Some(oid) = order_id {
            q = q.bind(oid.to_string());
        }
        if let Some(tid) = tech_id {
            q = q.bind(tid.to_string());
        }
        let rows = q.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_service_appointment(&self, appt: &ServiceAppointment) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE fsm_service_appointments SET status = ?, actual_start = ?, actual_end = ?,
                notes = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&appt.status)
        .bind(appt.actual_start.map(|d| d.to_rfc3339()))
        .bind(appt.actual_end.map(|d| d.to_rfc3339()))
        .bind(&appt.notes)
        .bind(appt.updated_at.to_rfc3339())
        .bind(appt.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_route(&self, route: &ServiceRoute) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_routes (id, route_number, technician_id, route_date, status,
                total_appointments, completed_appointments, total_distance, total_duration_minutes, optimization_score, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(route.id.to_string())
        .bind(&route.route_number)
        .bind(route.technician_id.to_string())
        .bind(&route.route_date)
        .bind(&route.status)
        .bind(route.total_appointments)
        .bind(route.completed_appointments)
        .bind(route.total_distance)
        .bind(route.total_duration_minutes)
        .bind(route.optimization_score)
        .bind(route.created_at.to_rfc3339())
        .bind(route.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_service_route(&self, id: Uuid) -> anyhow::Result<Option<ServiceRoute>> {
        let row: Option<ServiceRouteRow> = sqlx::query_as::<_, ServiceRouteRow>(
            r#"SELECT id, route_number, technician_id, route_date, status, total_appointments,
                completed_appointments, total_distance, total_duration_minutes, optimization_score, created_at, updated_at
                FROM fsm_service_routes WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_service_routes(&self, tech_id: Option<Uuid>, _date: Option<chrono::DateTime<Utc>>) -> anyhow::Result<Vec<ServiceRoute>> {
        let rows: Vec<ServiceRouteRow> = if let Some(tid) = tech_id {
            sqlx::query_as::<_, ServiceRouteRow>(
                r#"SELECT id, route_number, technician_id, route_date, status, total_appointments,
                    completed_appointments, total_distance, total_duration_minutes, optimization_score, created_at, updated_at
                    FROM fsm_service_routes WHERE technician_id = ?"#,
            )
            .bind(tid.to_string())
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, ServiceRouteRow>(
                r#"SELECT id, route_number, technician_id, route_date, status, total_appointments,
                    completed_appointments, total_distance, total_duration_minutes, optimization_score, created_at, updated_at
                    FROM fsm_service_routes ORDER BY route_date DESC"#,
            )
            .fetch_all(&self.pool).await?
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_service_route(&self, route: &ServiceRoute) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE fsm_service_routes SET status = ?, completed_appointments = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&route.status)
        .bind(route.completed_appointments)
        .bind(route.updated_at.to_rfc3339())
        .bind(route.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_route_stop(&self, stop: &ServiceRouteStop) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_route_stops (id, route_id, appointment_id, stop_sequence, planned_arrival,
                actual_arrival, planned_departure, actual_departure, travel_distance, travel_time_minutes, status, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(stop.id.to_string())
        .bind(stop.route_id.to_string())
        .bind(stop.appointment_id.to_string())
        .bind(stop.stop_sequence)
        .bind(&stop.planned_arrival)
        .bind(stop.actual_arrival.as_ref())
        .bind(&stop.planned_departure)
        .bind(stop.actual_departure.as_ref())
        .bind(stop.travel_distance)
        .bind(stop.travel_time_minutes)
        .bind(&stop.status)
        .bind(stop.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_route_stops(&self, route_id: Uuid) -> anyhow::Result<Vec<ServiceRouteStop>> {
        let rows: Vec<ServiceRouteStopRow> = sqlx::query_as::<_, ServiceRouteStopRow>(
            r#"SELECT id, route_id, appointment_id, stop_sequence, planned_arrival, actual_arrival,
                planned_departure, actual_departure, travel_distance, travel_time_minutes, status, created_at
                FROM fsm_service_route_stops WHERE route_id = ? ORDER BY stop_sequence"#,
        )
        .bind(route_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_service_route_stop(&self, stop: &ServiceRouteStop) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE fsm_service_route_stops SET actual_arrival = ?, actual_departure = ?, status = ? WHERE id = ?"#,
        )
        .bind(stop.actual_arrival.as_ref())
        .bind(stop.actual_departure.as_ref())
        .bind(&stop.status)
        .bind(stop.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_part(&self, part: &ServicePart) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_parts (id, service_order_id, product_id, quantity, unit_price,
                total_price, currency, disposition, returned, notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(part.id.to_string())
        .bind(part.service_order_id.to_string())
        .bind(part.product_id.to_string())
        .bind(part.quantity)
        .bind(part.unit_price)
        .bind(part.total_price)
        .bind(&part.currency)
        .bind(&part.disposition)
        .bind(part.returned)
        .bind(&part.notes)
        .bind(part.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_parts(&self, order_id: Uuid) -> anyhow::Result<Vec<ServicePart>> {
        let rows: Vec<ServicePartRow> = sqlx::query_as::<_, ServicePartRow>(
            r#"SELECT id, service_order_id, product_id, quantity, unit_price, total_price,
                currency, disposition, returned, notes, created_at
                FROM fsm_service_parts WHERE service_order_id = ?"#,
        )
        .bind(order_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_service_time_entry(&self, entry: &ServiceTimeEntry) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_time_entries (id, service_order_id, technician_id, entry_date, start_time,
                end_time, hours, overtime_hours, work_type, billable, rate, total_amount, currency, notes, approved, approved_by, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(entry.id.to_string())
        .bind(entry.service_order_id.to_string())
        .bind(entry.technician_id.to_string())
        .bind(entry.entry_date.to_rfc3339())
        .bind(&entry.start_time)
        .bind(&entry.end_time)
        .bind(entry.hours)
        .bind(entry.overtime_hours)
        .bind(&entry.work_type)
        .bind(entry.billable)
        .bind(entry.rate)
        .bind(entry.total_amount)
        .bind(&entry.currency)
        .bind(&entry.notes)
        .bind(entry.approved)
        .bind(entry.approved_by.map(|id| id.to_string()))
        .bind(entry.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_time_entries(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceTimeEntry>> {
        let rows: Vec<ServiceTimeEntryRow> = sqlx::query_as::<_, ServiceTimeEntryRow>(
            r#"SELECT id, service_order_id, technician_id, entry_date, start_time, end_time, hours, overtime_hours,
                work_type, billable, rate, total_amount, currency, notes, approved, approved_by, created_at
                FROM fsm_service_time_entries WHERE service_order_id = ? ORDER BY start_time"#,
        )
        .bind(order_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_service_expense(&self, expense: &ServiceExpense) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_expenses (id, service_order_id, technician_id, expense_type, amount,
                currency, description, receipt_url, approved, approved_by, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(expense.id.to_string())
        .bind(expense.service_order_id.to_string())
        .bind(expense.technician_id.to_string())
        .bind(&expense.expense_type)
        .bind(expense.amount)
        .bind(&expense.currency)
        .bind(&expense.description)
        .bind(&expense.receipt_url)
        .bind(expense.approved)
        .bind(expense.approved_by.map(|id| id.to_string()))
        .bind(expense.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_expenses(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceExpense>> {
        let rows: Vec<ServiceExpenseRow> = sqlx::query_as::<_, ServiceExpenseRow>(
            r#"SELECT id, service_order_id, technician_id, expense_type, amount, currency, description,
                receipt_url, approved, approved_by, created_at
                FROM fsm_service_expenses WHERE service_order_id = ? ORDER BY created_at"#,
        )
        .bind(order_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_service_checklist(&self, checklist: &ServiceChecklist) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_checklists (id, service_order_id, checklist_type, name, completed, completed_at, completed_by, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(checklist.id.to_string())
        .bind(checklist.service_order_id.to_string())
        .bind(&checklist.checklist_type)
        .bind(&checklist.name)
        .bind(checklist.completed)
        .bind(checklist.completed_at.map(|d| d.to_rfc3339()))
        .bind(checklist.completed_by.map(|id| id.to_string()))
        .bind(checklist.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_checklists(&self, order_id: Uuid) -> anyhow::Result<Vec<ServiceChecklist>> {
        let rows: Vec<ServiceChecklistRow> = sqlx::query_as::<_, ServiceChecklistRow>(
            r#"SELECT id, service_order_id, checklist_type, name, completed, completed_at, completed_by, created_at
                FROM fsm_service_checklists WHERE service_order_id = ?"#,
        )
        .bind(order_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_service_checklist_item(&self, item: &ServiceChecklistItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_checklist_items (id, checklist_id, item_number, description,
                is_required, response_type, response_value, notes, completed, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(item.id.to_string())
        .bind(item.checklist_id.to_string())
        .bind(item.item_number)
        .bind(&item.description)
        .bind(item.is_required)
        .bind(&item.response_type)
        .bind(&item.response_value)
        .bind(&item.notes)
        .bind(item.completed)
        .bind(item.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_service_checklist_items(&self, checklist_id: Uuid) -> anyhow::Result<Vec<ServiceChecklistItem>> {
        let rows: Vec<ServiceChecklistItemRow> = sqlx::query_as::<_, ServiceChecklistItemRow>(
            r#"SELECT id, checklist_id, item_number, description, is_required, response_type, response_value,
                notes, completed, created_at FROM fsm_service_checklist_items
                WHERE checklist_id = ? ORDER BY item_number"#,
        )
        .bind(checklist_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_service_checklist_item(&self, item: &ServiceChecklistItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE fsm_service_checklist_items SET response_value = ?, notes = ?, completed = ? WHERE id = ?"#,
        )
        .bind(&item.response_value)
        .bind(&item.notes)
        .bind(item.completed)
        .bind(item.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_service_contract(&self, contract: &ServiceContract) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_service_contracts (id, contract_number, customer_id, contract_name, start_date, end_date,
                contract_type, coverage_type, response_time_hours, resolution_time_hours, visit_limit, visits_used,
                coverage_hours, coverage_days, annual_fee, currency, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(contract.id.to_string())
        .bind(&contract.contract_number)
        .bind(contract.customer_id.to_string())
        .bind(&contract.contract_name)
        .bind(contract.start_date.to_rfc3339())
        .bind(contract.end_date.to_rfc3339())
        .bind(&contract.contract_type)
        .bind(&contract.coverage_type)
        .bind(contract.response_time_hours)
        .bind(contract.resolution_time_hours)
        .bind(contract.visit_limit)
        .bind(contract.visits_used)
        .bind(&contract.coverage_hours)
        .bind(&contract.coverage_days)
        .bind(contract.annual_fee)
        .bind(&contract.currency)
        .bind(contract.is_active)
        .bind(contract.created_at.to_rfc3339())
        .bind(contract.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_service_contract(&self, id: Uuid) -> anyhow::Result<Option<ServiceContract>> {
        let row: Option<ServiceContractRow> = sqlx::query_as::<_, ServiceContractRow>(
            r#"SELECT id, contract_number, customer_id, contract_name, start_date, end_date, contract_type,
                coverage_type, response_time_hours, resolution_time_hours, visit_limit, visits_used,
                coverage_hours, coverage_days, annual_fee, currency, is_active, created_at, updated_at
                FROM fsm_service_contracts WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_service_contracts(&self, customer_id: Uuid) -> anyhow::Result<Vec<ServiceContract>> {
        let rows: Vec<ServiceContractRow> = sqlx::query_as::<_, ServiceContractRow>(
            r#"SELECT id, contract_number, customer_id, contract_name, start_date, end_date, contract_type,
                coverage_type, response_time_hours, resolution_time_hours, visit_limit, visits_used,
                coverage_hours, coverage_days, annual_fee, currency, is_active, created_at, updated_at
                FROM fsm_service_contracts WHERE customer_id = ? ORDER BY created_at DESC"#,
        )
        .bind(customer_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_service_contract(&self, contract: &ServiceContract) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE fsm_service_contracts SET is_active = ?, visits_used = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(contract.is_active)
        .bind(contract.visits_used)
        .bind(contract.updated_at.to_rfc3339())
        .bind(contract.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_dispatch_rule(&self, rule: &DispatchRule) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_dispatch_rules (id, rule_name, description, priority, conditions, actions, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(rule.id.to_string())
        .bind(&rule.rule_name)
        .bind(&rule.description)
        .bind(rule.priority)
        .bind(&rule.conditions)
        .bind(&rule.actions)
        .bind(rule.is_active)
        .bind(rule.created_at.to_rfc3339())
        .bind(rule.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_dispatch_rules(&self) -> anyhow::Result<Vec<DispatchRule>> {
        let rows: Vec<DispatchRuleRow> = sqlx::query_as::<_, DispatchRuleRow>(
            r#"SELECT id, rule_name, description, priority, conditions, actions, is_active, created_at, updated_at
                FROM fsm_dispatch_rules ORDER BY priority"#,
        )
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_technician_skill(&self, skill: &TechnicianSkill) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_technician_skills (id, skill_code, skill_name, category, description, proficiency_levels, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(skill.id.to_string())
        .bind(&skill.skill_code)
        .bind(&skill.skill_name)
        .bind(&skill.category)
        .bind(&skill.description)
        .bind(&skill.proficiency_levels)
        .bind(skill.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_technician_skills(&self) -> anyhow::Result<Vec<TechnicianSkill>> {
        let rows: Vec<TechnicianSkillRow> = sqlx::query_as::<_, TechnicianSkillRow>(
            r#"SELECT id, skill_code, skill_name, category, description, proficiency_levels, created_at FROM fsm_technician_skills"#,
        )
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_technician_skill_assignment(&self, tsa: &TechnicianSkillAssignment) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO fsm_technician_skill_assignments (id, technician_id, skill_id, proficiency_level,
                certified, certified_date, expiry_date, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(tsa.id.to_string())
        .bind(tsa.technician_id.to_string())
        .bind(tsa.skill_id.to_string())
        .bind(tsa.proficiency_level)
        .bind(tsa.certified)
        .bind(tsa.certified_date.map(|d| d.to_rfc3339()))
        .bind(tsa.expiry_date.map(|d| d.to_rfc3339()))
        .bind(tsa.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_technician_skill_assignments(&self, tech_id: Uuid) -> anyhow::Result<Vec<TechnicianSkillAssignment>> {
        let rows: Vec<TechnicianSkillAssignmentRow> = sqlx::query_as::<_, TechnicianSkillAssignmentRow>(
            r#"SELECT id, technician_id, skill_id, proficiency_level, certified, certified_date,
                expiry_date, created_at FROM fsm_technician_skill_assignments WHERE technician_id = ?"#,
        )
        .bind(tech_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

// Row structs for FromRow
#[derive(Debug, FromRow)]
struct ServiceOrderRow {
    id: String,
    order_number: String,
    customer_id: String,
    contact_name: String,
    contact_phone: String,
    contact_email: Option<String>,
    service_address: String,
    service_city: String,
    service_state: Option<String>,
    service_postal_code: String,
    service_country: String,
    service_lat: Option<f64>,
    service_lng: Option<f64>,
    work_type: String,
    priority: String,
    status: String,
    description: String,
    asset_id: Option<String>,
    asset_serial: Option<String>,
    contract_id: Option<String>,
    sla_id: Option<String>,
    assigned_technician_id: Option<String>,
    scheduled_date: Option<String>,
    scheduled_start: Option<String>,
    scheduled_end: Option<String>,
    actual_start: Option<String>,
    actual_end: Option<String>,
    travel_time_minutes: Option<i32>,
    work_duration_minutes: Option<i32>,
    resolution_notes: Option<String>,
    customer_signature: Option<String>,
    customer_rating: Option<i32>,
    customer_feedback: Option<String>,
    total_charges: i64,
    currency: String,
    created_at: String,
    updated_at: String,
}

impl From<ServiceOrderRow> for ServiceOrder {
    fn from(row: ServiceOrderRow) -> Self {
        ServiceOrder {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            order_number: row.order_number,
            customer_id: Uuid::parse_str(&row.customer_id).unwrap_or(Uuid::nil()),
            contact_name: row.contact_name,
            contact_phone: row.contact_phone,
            contact_email: row.contact_email,
            service_address: row.service_address,
            service_city: row.service_city,
            service_state: row.service_state,
            service_postal_code: row.service_postal_code,
            service_country: row.service_country,
            service_lat: row.service_lat,
            service_lng: row.service_lng,
            work_type: row.work_type.parse().unwrap_or(WorkType::Repair),
            priority: row.priority.parse().unwrap_or(Priority::Medium),
            status: row.status.parse().unwrap_or(ServiceOrderStatus::Scheduled),
            description: row.description,
            asset_id: row.asset_id.and_then(|s| Uuid::parse_str(&s).ok()),
            asset_serial: row.asset_serial,
            contract_id: row.contract_id.and_then(|s| Uuid::parse_str(&s).ok()),
            sla_id: row.sla_id.and_then(|s| Uuid::parse_str(&s).ok()),
            assigned_technician_id: row.assigned_technician_id.and_then(|s| Uuid::parse_str(&s).ok()),
            scheduled_date: row.scheduled_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            scheduled_start: row.scheduled_start.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            scheduled_end: row.scheduled_end.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            actual_start: row.actual_start.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            actual_end: row.actual_end.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            travel_time_minutes: row.travel_time_minutes,
            work_duration_minutes: row.work_duration_minutes,
            resolution_notes: row.resolution_notes,
            customer_signature: row.customer_signature,
            customer_rating: row.customer_rating,
            customer_feedback: row.customer_feedback,
            total_charges: row.total_charges,
            currency: row.currency,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceOrderTaskRow {
    id: String,
    service_order_id: String,
    task_number: i32,
    task_type: String,
    description: String,
    estimated_duration_minutes: i32,
    actual_duration_minutes: Option<i32>,
    status: String,
    completed_at: Option<String>,
    notes: Option<String>,
    created_at: String,
}

impl From<ServiceOrderTaskRow> for ServiceOrderTask {
    fn from(row: ServiceOrderTaskRow) -> Self {
        ServiceOrderTask {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            service_order_id: Uuid::parse_str(&row.service_order_id).unwrap_or(Uuid::nil()),
            task_number: row.task_number,
            task_type: row.task_type,
            description: row.description,
            estimated_duration_minutes: row.estimated_duration_minutes,
            actual_duration_minutes: row.actual_duration_minutes,
            status: row.status,
            completed_at: row.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            notes: row.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}


#[derive(Debug, FromRow)]
struct TechnicianAvailabilityRow {
    id: String,
    technician_id: String,
    date: String,
    start_time: String,
    end_time: String,
    status: String,
    reason: Option<String>,
    created_at: String,
}

impl From<TechnicianAvailabilityRow> for TechnicianAvailability {
    fn from(row: TechnicianAvailabilityRow) -> Self {
        TechnicianAvailability {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            technician_id: Uuid::parse_str(&row.technician_id).unwrap_or(Uuid::nil()),
            date: row.date,
            start_time: row.start_time,
            end_time: row.end_time,
            status: row.status,
            reason: row.reason,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceTerritoryRow {
    id: String,
    territory_code: String,
    name: String,
    description: Option<String>,
    parent_territory_id: Option<String>,
    boundary_type: String,
    boundary_data: String,
    manager_id: Option<String>,
    is_active: bool,
    created_at: String,
    updated_at: String,
}

impl From<ServiceTerritoryRow> for ServiceTerritory {
    fn from(row: ServiceTerritoryRow) -> Self {
        ServiceTerritory {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            territory_code: row.territory_code,
            name: row.name,
            description: row.description,
            parent_territory_id: row.parent_territory_id.and_then(|s| Uuid::parse_str(&s).ok()),
            boundary_type: row.boundary_type,
            boundary_data: row.boundary_data,
            manager_id: row.manager_id.and_then(|s| Uuid::parse_str(&s).ok()),
            is_active: row.is_active,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct TechnicianTerritoryRow {
    id: String,
    technician_id: String,
    territory_id: String,
    is_primary: bool,
    effective_date: String,
    expiry_date: Option<String>,
    created_at: String,
}

impl From<TechnicianTerritoryRow> for TechnicianTerritory {
    fn from(row: TechnicianTerritoryRow) -> Self {
        TechnicianTerritory {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            technician_id: Uuid::parse_str(&row.technician_id).unwrap_or(Uuid::nil()),
            territory_id: Uuid::parse_str(&row.territory_id).unwrap_or(Uuid::nil()),
            is_primary: row.is_primary,
            effective_date: chrono::DateTime::parse_from_rfc3339(&row.effective_date).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            expiry_date: row.expiry_date.as_ref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok()).map(|d| d.with_timezone(&chrono::Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceAppointmentRow {
    id: String,
    appointment_number: String,
    service_order_id: String,
    technician_id: String,
    scheduled_start: String,
    scheduled_end: String,
    actual_start: Option<String>,
    actual_end: Option<String>,
    status: String,
    confirmation_status: String,
    reminder_sent: bool,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ServiceAppointmentRow> for ServiceAppointment {
    fn from(row: ServiceAppointmentRow) -> Self {
        ServiceAppointment {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            appointment_number: row.appointment_number,
            service_order_id: Uuid::parse_str(&row.service_order_id).unwrap_or(Uuid::nil()),
            technician_id: Uuid::parse_str(&row.technician_id).unwrap_or(Uuid::nil()),
            scheduled_start: chrono::DateTime::parse_from_rfc3339(&row.scheduled_start).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            scheduled_end: chrono::DateTime::parse_from_rfc3339(&row.scheduled_end).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            actual_start: row.actual_start.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            actual_end: row.actual_end.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            status: row.status,
            confirmation_status: row.confirmation_status,
            reminder_sent: row.reminder_sent,
            notes: row.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceRouteRow {
    id: String,
    route_number: String,
    technician_id: String,
    route_date: String,
    status: String,
    total_appointments: i32,
    completed_appointments: i32,
    total_distance: f64,
    total_duration_minutes: i32,
    optimization_score: Option<f64>,
    created_at: String,
    updated_at: String,
}

impl From<ServiceRouteRow> for ServiceRoute {
    fn from(row: ServiceRouteRow) -> Self {
        ServiceRoute {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            route_number: row.route_number,
            technician_id: Uuid::parse_str(&row.technician_id).unwrap_or(Uuid::nil()),
            route_date: row.route_date,
            status: row.status,
            total_appointments: row.total_appointments,
            completed_appointments: row.completed_appointments,
            total_distance: row.total_distance,
            total_duration_minutes: row.total_duration_minutes,
            optimization_score: row.optimization_score,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceRouteStopRow {
    id: String,
    route_id: String,
    appointment_id: String,
    stop_sequence: i32,
    planned_arrival: String,
    actual_arrival: Option<String>,
    planned_departure: String,
    actual_departure: Option<String>,
    travel_distance: f64,
    travel_time_minutes: i32,
    status: String,
    created_at: String,
}

impl From<ServiceRouteStopRow> for ServiceRouteStop {
    fn from(row: ServiceRouteStopRow) -> Self {
        ServiceRouteStop {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            route_id: Uuid::parse_str(&row.route_id).unwrap_or(Uuid::nil()),
            appointment_id: Uuid::parse_str(&row.appointment_id).unwrap_or(Uuid::nil()),
            stop_sequence: row.stop_sequence,
            planned_arrival: row.planned_arrival,
            actual_arrival: row.actual_arrival,
            planned_departure: row.planned_departure,
            actual_departure: row.actual_departure,
            travel_distance: row.travel_distance,
            travel_time_minutes: row.travel_time_minutes,
            status: row.status,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServicePartRow {
    id: String,
    service_order_id: String,
    product_id: String,
    quantity: i32,
    unit_price: i64,
    total_price: i64,
    currency: String,
    disposition: String,
    returned: bool,
    notes: Option<String>,
    created_at: String,
}

impl From<ServicePartRow> for ServicePart {
    fn from(row: ServicePartRow) -> Self {
        ServicePart {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            service_order_id: Uuid::parse_str(&row.service_order_id).unwrap_or(Uuid::nil()),
            product_id: Uuid::parse_str(&row.product_id).unwrap_or(Uuid::nil()),
            quantity: row.quantity,
            unit_price: row.unit_price,
            total_price: row.total_price,
            currency: row.currency,
            disposition: row.disposition,
            returned: row.returned,
            notes: row.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceTimeEntryRow {
    id: String,
    service_order_id: String,
    technician_id: String,
    entry_date: String,
    start_time: String,
    end_time: String,
    hours: f64,
    overtime_hours: f64,
    work_type: String,
    billable: bool,
    rate: i64,
    total_amount: i64,
    currency: String,
    notes: Option<String>,
    approved: bool,
    approved_by: Option<String>,
    created_at: String,
}

impl From<ServiceTimeEntryRow> for ServiceTimeEntry {
    fn from(row: ServiceTimeEntryRow) -> Self {
        ServiceTimeEntry {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            service_order_id: Uuid::parse_str(&row.service_order_id).unwrap_or(Uuid::nil()),
            technician_id: Uuid::parse_str(&row.technician_id).unwrap_or(Uuid::nil()),
            entry_date: chrono::DateTime::parse_from_rfc3339(&row.entry_date).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            start_time: row.start_time,
            end_time: row.end_time,
            hours: row.hours,
            overtime_hours: row.overtime_hours,
            work_type: row.work_type,
            billable: row.billable,
            rate: row.rate,
            total_amount: row.total_amount,
            currency: row.currency,
            notes: row.notes,
            approved: row.approved,
            approved_by: row.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceExpenseRow {
    id: String,
    service_order_id: String,
    technician_id: String,
    expense_type: String,
    amount: i64,
    currency: String,
    description: Option<String>,
    receipt_url: Option<String>,
    approved: bool,
    approved_by: Option<String>,
    created_at: String,
}

impl From<ServiceExpenseRow> for ServiceExpense {
    fn from(row: ServiceExpenseRow) -> Self {
        ServiceExpense {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            service_order_id: Uuid::parse_str(&row.service_order_id).unwrap_or(Uuid::nil()),
            technician_id: Uuid::parse_str(&row.technician_id).unwrap_or(Uuid::nil()),
            expense_type: row.expense_type,
            amount: row.amount,
            currency: row.currency,
            description: row.description,
            receipt_url: row.receipt_url,
            approved: row.approved,
            approved_by: row.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceChecklistRow {
    id: String,
    service_order_id: String,
    checklist_type: String,
    name: String,
    completed: bool,
    completed_at: Option<String>,
    completed_by: Option<String>,
    created_at: String,
}

impl From<ServiceChecklistRow> for ServiceChecklist {
    fn from(row: ServiceChecklistRow) -> Self {
        ServiceChecklist {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            service_order_id: Uuid::parse_str(&row.service_order_id).unwrap_or(Uuid::nil()),
            checklist_type: row.checklist_type,
            name: row.name,
            completed: row.completed,
            completed_at: row.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            completed_by: row.completed_by.and_then(|s| Uuid::parse_str(&s).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceChecklistItemRow {
    id: String,
    checklist_id: String,
    item_number: i32,
    description: String,
    is_required: bool,
    response_type: String,
    response_value: Option<String>,
    notes: Option<String>,
    completed: bool,
    created_at: String,
}

impl From<ServiceChecklistItemRow> for ServiceChecklistItem {
    fn from(row: ServiceChecklistItemRow) -> Self {
        ServiceChecklistItem {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            checklist_id: Uuid::parse_str(&row.checklist_id).unwrap_or(Uuid::nil()),
            item_number: row.item_number,
            description: row.description,
            is_required: row.is_required,
            response_type: row.response_type,
            response_value: row.response_value,
            notes: row.notes,
            completed: row.completed,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ServiceContractRow {
    id: String,
    contract_number: String,
    customer_id: String,
    contract_name: String,
    start_date: String,
    end_date: String,
    contract_type: String,
    coverage_type: String,
    response_time_hours: i32,
    resolution_time_hours: i32,
    visit_limit: Option<i32>,
    visits_used: i32,
    coverage_hours: String,
    coverage_days: String,
    annual_fee: i64,
    currency: String,
    is_active: bool,
    created_at: String,
    updated_at: String,
}

impl From<ServiceContractRow> for ServiceContract {
    fn from(row: ServiceContractRow) -> Self {
        ServiceContract {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            contract_number: row.contract_number,
            customer_id: Uuid::parse_str(&row.customer_id).unwrap_or(Uuid::nil()),
            contract_name: row.contract_name,
            start_date: chrono::DateTime::parse_from_rfc3339(&row.start_date).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            end_date: chrono::DateTime::parse_from_rfc3339(&row.end_date).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            contract_type: row.contract_type,
            coverage_type: row.coverage_type,
            response_time_hours: row.response_time_hours,
            resolution_time_hours: row.resolution_time_hours,
            visit_limit: row.visit_limit,
            visits_used: row.visits_used,
            coverage_hours: row.coverage_hours,
            coverage_days: row.coverage_days,
            annual_fee: row.annual_fee,
            currency: row.currency,
            is_active: row.is_active,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct DispatchRuleRow {
    id: String,
    rule_name: String,
    description: Option<String>,
    priority: i32,
    conditions: String,
    actions: String,
    is_active: bool,
    created_at: String,
    updated_at: String,
}

impl From<DispatchRuleRow> for DispatchRule {
    fn from(row: DispatchRuleRow) -> Self {
        DispatchRule {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            rule_name: row.rule_name,
            description: row.description,
            priority: row.priority,
            conditions: row.conditions,
            actions: row.actions,
            is_active: row.is_active,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct TechnicianSkillRow {
    id: String,
    skill_code: String,
    skill_name: String,
    category: String,
    description: Option<String>,
    proficiency_levels: String,
    created_at: String,
}

impl From<TechnicianSkillRow> for TechnicianSkill {
    fn from(row: TechnicianSkillRow) -> Self {
        TechnicianSkill {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            skill_code: row.skill_code,
            skill_name: row.skill_name,
            category: row.category,
            description: row.description,
            proficiency_levels: row.proficiency_levels,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct TechnicianSkillAssignmentRow {
    id: String,
    technician_id: String,
    skill_id: String,
    proficiency_level: i32,
    certified: bool,
    certified_date: Option<String>,
    expiry_date: Option<String>,
    created_at: String,
}

impl From<TechnicianSkillAssignmentRow> for TechnicianSkillAssignment {
    fn from(row: TechnicianSkillAssignmentRow) -> Self {
        TechnicianSkillAssignment {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            technician_id: Uuid::parse_str(&row.technician_id).unwrap_or(Uuid::nil()),
            skill_id: Uuid::parse_str(&row.skill_id).unwrap_or(Uuid::nil()),
            proficiency_level: row.proficiency_level,
            certified: row.certified,
            certified_date: row.certified_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            expiry_date: row.expiry_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct TechnicianRow {
    id: String,
    employee_id: Option<String>,
    technician_code: String,
    first_name: String,
    last_name: String,
    phone: String,
    email: Option<String>,
    status: String,
    skills: String,
    certifications: String,
    home_location_lat: Option<f64>,
    home_location_lng: Option<f64>,
    current_location_lat: Option<f64>,
    current_location_lng: Option<f64>,
    current_order_id: Option<String>,
    service_region: Option<String>,
    hourly_rate: i64,
    overtime_rate: i64,
    currency: String,
    work_start_time: Option<String>,
    work_end_time: Option<String>,
    working_days: String,
    created_at: String,
    updated_at: String,
}

impl From<TechnicianRow> for Technician {
    fn from(row: TechnicianRow) -> Self {
        Technician {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            employee_id: row.employee_id.and_then(|s| Uuid::parse_str(&s).ok()),
            technician_code: row.technician_code,
            first_name: row.first_name,
            last_name: row.last_name,
            phone: row.phone,
            email: row.email,
            status: row.status.parse().unwrap_or(TechnicianStatus::Available),
            skills: row.skills,
            certifications: row.certifications,
            home_location_lat: row.home_location_lat,
            home_location_lng: row.home_location_lng,
            current_location_lat: row.current_location_lat,
            current_location_lng: row.current_location_lng,
            current_order_id: row.current_order_id.and_then(|s| Uuid::parse_str(&s).ok()),
            service_region: row.service_region,
            hourly_rate: row.hourly_rate,
            overtime_rate: row.overtime_rate,
            currency: row.currency,
            work_start_time: row.work_start_time,
            work_end_time: row.work_end_time,
            working_days: row.working_days,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
