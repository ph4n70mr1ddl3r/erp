use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ServiceOrderStatus {
    Scheduled,
    Dispatched,
    InProgress,
    OnHold,
    Completed,
    Cancelled,
}

impl std::str::FromStr for ServiceOrderStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Scheduled" => Ok(ServiceOrderStatus::Scheduled),
            "Dispatched" => Ok(ServiceOrderStatus::Dispatched),
            "InProgress" => Ok(ServiceOrderStatus::InProgress),
            "OnHold" => Ok(ServiceOrderStatus::OnHold),
            "Completed" => Ok(ServiceOrderStatus::Completed),
            "Cancelled" => Ok(ServiceOrderStatus::Cancelled),
            _ => Ok(ServiceOrderStatus::Scheduled),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TechnicianStatus {
    Available,
    Busy,
    OnBreak,
    OffDuty,
    Traveling,
}

impl std::str::FromStr for TechnicianStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Available" => Ok(TechnicianStatus::Available),
            "Busy" => Ok(TechnicianStatus::Busy),
            "OnBreak" => Ok(TechnicianStatus::OnBreak),
            "OffDuty" => Ok(TechnicianStatus::OffDuty),
            "Traveling" => Ok(TechnicianStatus::Traveling),
            _ => Ok(TechnicianStatus::Available),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

impl std::str::FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(Priority::Low),
            "Medium" => Ok(Priority::Medium),
            "High" => Ok(Priority::High),
            "Critical" => Ok(Priority::Critical),
            "Emergency" => Ok(Priority::Emergency),
            _ => Ok(Priority::Medium),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum WorkType {
    Installation,
    Maintenance,
    Repair,
    Inspection,
    Calibration,
    Training,
    Consultation,
    Other,
}

impl std::str::FromStr for WorkType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "installation" => Ok(WorkType::Installation),
            "maintenance" => Ok(WorkType::Maintenance),
            "repair" => Ok(WorkType::Repair),
            "inspection" => Ok(WorkType::Inspection),
            "calibration" => Ok(WorkType::Calibration),
            "training" => Ok(WorkType::Training),
            "consultation" => Ok(WorkType::Consultation),
            _ => Ok(WorkType::Other),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AppointmentStatus {
    Scheduled,
    Confirmed,
    InProgress,
    Completed,
    Cancelled,
    NoShow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ContractStatus {
    Draft,
    Pending,
    Active,
    Expired,
    Cancelled,
    Suspended,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ContractType {
    Standard,
    Premium,
    Basic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrder {
    pub id: Uuid,
    pub order_number: String,
    pub customer_id: Uuid,
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: Option<String>,
    pub service_address: String,
    pub service_city: String,
    pub service_state: Option<String>,
    pub service_postal_code: String,
    pub service_country: String,
    pub service_lat: Option<f64>,
    pub service_lng: Option<f64>,
    pub work_type: WorkType,
    pub priority: Priority,
    pub status: ServiceOrderStatus,
    pub description: String,
    pub asset_id: Option<Uuid>,
    pub asset_serial: Option<String>,
    pub contract_id: Option<Uuid>,
    pub sla_id: Option<Uuid>,
    pub assigned_technician_id: Option<Uuid>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub travel_time_minutes: Option<i32>,
    pub work_duration_minutes: Option<i32>,
    pub resolution_notes: Option<String>,
    pub customer_signature: Option<String>,
    pub customer_rating: Option<i32>,
    pub customer_feedback: Option<String>,
    pub total_charges: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrderTask {
    pub id: Uuid,
    pub service_order_id: Uuid,
    pub task_number: i32,
    pub task_type: String,
    pub description: String,
    pub estimated_duration_minutes: i32,
    pub actual_duration_minutes: Option<i32>,
    pub status: String,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technician {
    pub id: Uuid,
    pub employee_id: Option<Uuid>,
    pub technician_code: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: Option<String>,
    pub status: TechnicianStatus,
    pub skills: String,
    pub certifications: String,
    pub home_location_lat: Option<f64>,
    pub home_location_lng: Option<f64>,
    pub current_location_lat: Option<f64>,
    pub current_location_lng: Option<f64>,
    pub current_order_id: Option<Uuid>,
    pub service_region: Option<String>,
    pub hourly_rate: i64,
    pub overtime_rate: i64,
    pub currency: String,
    pub work_start_time: Option<String>,
    pub work_end_time: Option<String>,
    pub working_days: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicianAvailability {
    pub id: Uuid,
    pub technician_id: Uuid,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTerritory {
    pub id: Uuid,
    pub territory_code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_territory_id: Option<Uuid>,
    pub boundary_type: String,
    pub boundary_data: String,
    pub manager_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicianTerritory {
    pub id: Uuid,
    pub technician_id: Uuid,
    pub territory_id: Uuid,
    pub is_primary: bool,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAppointment {
    pub id: Uuid,
    pub appointment_number: String,
    pub service_order_id: Uuid,
    pub technician_id: Uuid,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub status: String,
    pub confirmation_status: String,
    pub reminder_sent: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRoute {
    pub id: Uuid,
    pub route_number: String,
    pub technician_id: Uuid,
    pub route_date: String,
    pub status: String,
    pub total_appointments: i32,
    pub completed_appointments: i32,
    pub total_distance: f64,
    pub total_duration_minutes: i32,
    pub optimization_score: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRouteStop {
    pub id: Uuid,
    pub route_id: Uuid,
    pub appointment_id: Uuid,
    pub stop_sequence: i32,
    pub planned_arrival: String,
    pub actual_arrival: Option<String>,
    pub planned_departure: String,
    pub actual_departure: Option<String>,
    pub travel_distance: f64,
    pub travel_time_minutes: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePart {
    pub id: Uuid,
    pub service_order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: i64,
    pub total_price: i64,
    pub currency: String,
    pub disposition: String,
    pub returned: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTimeEntry {
    pub id: Uuid,
    pub service_order_id: Uuid,
    pub technician_id: Uuid,
    pub entry_date: DateTime<Utc>,
    pub start_time: String,
    pub end_time: String,
    pub hours: f64,
    pub overtime_hours: f64,
    pub work_type: String,
    pub billable: bool,
    pub rate: i64,
    pub total_amount: i64,
    pub currency: String,
    pub notes: Option<String>,
    pub approved: bool,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceExpense {
    pub id: Uuid,
    pub service_order_id: Uuid,
    pub technician_id: Uuid,
    pub expense_type: String,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
    pub receipt_url: Option<String>,
    pub approved: bool,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceChecklist {
    pub id: Uuid,
    pub service_order_id: Uuid,
    pub checklist_type: String,
    pub name: String,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceChecklistItem {
    pub id: Uuid,
    pub checklist_id: Uuid,
    pub item_number: i32,
    pub description: String,
    pub is_required: bool,
    pub response_type: String,
    pub response_value: Option<String>,
    pub notes: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceContract {
    pub id: Uuid,
    pub contract_number: String,
    pub customer_id: Uuid,
    pub contract_name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub contract_type: String,
    pub coverage_type: String,
    pub response_time_hours: i32,
    pub resolution_time_hours: i32,
    pub visit_limit: Option<i32>,
    pub visits_used: i32,
    pub coverage_hours: String,
    pub coverage_days: String,
    pub annual_fee: i64,
    pub currency: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchRule {
    pub id: Uuid,
    pub rule_name: String,
    pub description: Option<String>,
    pub priority: i32,
    pub conditions: String,
    pub actions: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicianSkill {
    pub id: Uuid,
    pub skill_code: String,
    pub skill_name: String,
    pub category: String,
    pub description: Option<String>,
    pub proficiency_levels: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicianSkillAssignment {
    pub id: Uuid,
    pub technician_id: Uuid,
    pub skill_id: Uuid,
    pub proficiency_level: i32,
    pub certified: bool,
    pub certified_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileAppSession {
    pub id: Uuid,
    pub technician_id: Uuid,
    pub device_id: String,
    pub device_type: String,
    pub os_version: String,
    pub app_version: String,
    pub login_at: DateTime<Utc>,
    pub logout_at: Option<DateTime<Utc>>,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub push_token: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceReport {
    pub id: Uuid,
    pub service_order_id: Uuid,
    pub report_type: String,
    pub title: String,
    pub content: serde_json::Value,
    pub photos: serde_json::Value,
    pub signature: Option<String>,
    pub generated_at: DateTime<Utc>,
    pub sent_to_customer: bool,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServiceOrderRequest {
    pub customer_id: Uuid,
    pub contact_name: String,
    pub contact_phone: String,
    pub contact_email: Option<String>,
    pub service_address: String,
    pub service_city: String,
    pub service_state: Option<String>,
    pub service_postal_code: String,
    pub service_country: String,
    pub work_type: WorkType,
    pub priority: Priority,
    pub description: String,
    pub asset_id: Option<Uuid>,
    pub contract_id: Option<Uuid>,
    pub scheduled_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchRequest {
    pub service_order_id: Uuid,
    pub technician_id: Option<Uuid>,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizeRouteRequest {
    pub technician_id: Uuid,
    pub route_date: DateTime<Utc>,
    pub service_order_ids: Vec<Uuid>,
    pub start_location_lat: f64,
    pub start_location_lng: f64,
    pub end_location_lat: Option<f64>,
    pub end_location_lng: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteOptimizationResult {
    pub route_id: Uuid,
    pub total_distance: f64,
    pub total_duration_minutes: i32,
    pub stops: Vec<RouteStopResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStopResult {
    pub service_order_id: Uuid,
    pub sequence: i32,
    pub planned_arrival: DateTime<Utc>,
    pub planned_departure: DateTime<Utc>,
    pub distance_from_previous: f64,
}
