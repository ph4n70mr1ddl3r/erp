use axum::{extract::State, routing::{get, post}, Json, Router};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/equipment", post(create_equipment).get(list_equipment))
        .route("/equipment/:id", get(get_equipment).put(update_equipment))
        .route("/equipment/:id/meter-readings", post(record_meter_reading).get(list_meter_readings))
        .route("/equipment/:id/failures", get(list_failures))
        .route("/equipment/:id/down", post(record_downtime))
        .route("/work-orders", post(create_work_order).get(list_work_orders))
        .route("/work-orders/:id", get(get_work_order))
        .route("/work-orders/:id/assign", post(assign_work_order))
        .route("/work-orders/:id/start", post(start_work_order))
        .route("/work-orders/:id/complete", post(complete_work_order))
        .route("/work-orders/:id/close", post(close_work_order))
        .route("/work-orders/:id/tasks", post(add_task).get(list_tasks))
        .route("/work-orders/:id/labor", post(add_labor).get(list_labor))
        .route("/work-orders/:id/parts", post(add_part).get(list_parts))
        .route("/pm-schedules", post(create_pm_schedule).get(list_pm_schedules))
        .route("/pm-schedules/:id", get(get_pm_schedule))
        .route("/pm-schedules/:id/tasks", post(add_pm_task).get(list_pm_tasks))
        .route("/pm-schedules/:id/generate", post(generate_pm_work_order))
        .route("/failure-codes", post(create_failure_code).get(list_failure_codes))
        .route("/locations", post(create_location).get(list_locations))
        .route("/spare-parts", post(create_spare_part).get(list_spare_parts))
        .route("/spare-parts/:id", get(get_spare_part))
        .route("/budgets", post(create_budget).get(list_budgets))
        .route("/kpis", get(list_kpis))
        .route("/service-contracts", post(create_contract).get(list_contracts))
}

#[derive(Serialize)]
pub struct EquipmentResponse {
    pub id: Uuid,
    pub asset_number: String,
    pub name: String,
    pub asset_type: String,
    pub criticality: String,
    pub status: String,
    pub location: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateEquipmentRequest {
    pub asset_number: String,
    pub name: String,
    pub asset_type: String,
    pub criticality: String,
    pub acquisition_cost: i64,
}

pub async fn create_equipment(State(_state): State<AppState>, Json(_req): Json<CreateEquipmentRequest>) -> Json<EquipmentResponse> {
    Json(EquipmentResponse {
        id: Uuid::new_v4(),
        asset_number: "EQ-001".to_string(),
        name: "Production Line 1".to_string(),
        asset_type: "Production".to_string(),
        criticality: "Critical".to_string(),
        status: "Running".to_string(),
        location: Some("Building A".to_string()),
    })
}

pub async fn list_equipment(State(_state): State<AppState>) -> Json<Vec<EquipmentResponse>> { Json(vec![]) }

pub async fn get_equipment(State(_state): State<AppState>) -> Json<EquipmentResponse> {
    Json(EquipmentResponse {
        id: Uuid::new_v4(),
        asset_number: "EQ-001".to_string(),
        name: "CNC Machine".to_string(),
        asset_type: "Production".to_string(),
        criticality: "High".to_string(),
        status: "Running".to_string(),
        location: Some("Building A, Floor 2".to_string()),
    })
}

pub async fn update_equipment(State(_state): State<AppState>, Json(_req): Json<CreateEquipmentRequest>) -> Json<EquipmentResponse> {
    Json(EquipmentResponse {
        id: Uuid::new_v4(),
        asset_number: "EQ-001".to_string(),
        name: "Updated Equipment".to_string(),
        asset_type: "Production".to_string(),
        criticality: "Medium".to_string(),
        status: "Running".to_string(),
        location: Some("Building B".to_string()),
    })
}

#[derive(Serialize)]
pub struct MeterReadingResponse { pub id: Uuid, pub reading_value: i64, pub reading_date: String }

#[derive(Deserialize)]
pub struct RecordMeterReadingRequest { pub reading_value: i64, pub reading_type: String }

pub async fn record_meter_reading(State(_state): State<AppState>, Json(_req): Json<RecordMeterReadingRequest>) -> Json<MeterReadingResponse> {
    Json(MeterReadingResponse { id: Uuid::new_v4(), reading_value: 5000, reading_date: chrono::Utc::now().to_rfc3339() })
}

pub async fn list_meter_readings(State(_state): State<AppState>) -> Json<Vec<MeterReadingResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct FailureResponse { pub id: Uuid, pub failure_date: String, pub description: String, pub downtime_hours: f64 }
pub async fn list_failures(State(_state): State<AppState>) -> Json<Vec<FailureResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct DowntimeResponse { pub id: Uuid, pub down_start: String, pub status: String }

#[derive(Deserialize)]
pub struct RecordDowntimeRequest { pub reason: String }

pub async fn record_downtime(State(_state): State<AppState>, Json(_req): Json<RecordDowntimeRequest>) -> Json<DowntimeResponse> {
    Json(DowntimeResponse { id: Uuid::new_v4(), down_start: chrono::Utc::now().to_rfc3339(), status: "Down".to_string() })
}

#[derive(Serialize)]
pub struct WorkOrderResponse {
    pub id: Uuid,
    pub wo_number: String,
    pub description: String,
    pub work_order_type: String,
    pub priority: String,
    pub status: String,
    pub asset_name: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateWorkOrderRequest {
    pub description: String,
    pub work_order_type: String,
    pub priority: String,
    pub asset_id: Option<Uuid>,
}

pub async fn create_work_order(State(_state): State<AppState>, Json(_req): Json<CreateWorkOrderRequest>) -> Json<WorkOrderResponse> {
    Json(WorkOrderResponse {
        id: Uuid::new_v4(),
        wo_number: format!("WO-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
        description: "Repair equipment".to_string(),
        work_order_type: "Corrective".to_string(),
        priority: "High".to_string(),
        status: "Requested".to_string(),
        asset_name: Some("CNC Machine".to_string()),
    })
}

pub async fn list_work_orders(State(_state): State<AppState>) -> Json<Vec<WorkOrderResponse>> { Json(vec![]) }

pub async fn get_work_order(State(_state): State<AppState>) -> Json<WorkOrderResponse> {
    Json(WorkOrderResponse {
        id: Uuid::new_v4(),
        wo_number: "WO-20240101000000".to_string(),
        description: "Scheduled maintenance".to_string(),
        work_order_type: "Preventive".to_string(),
        priority: "Medium".to_string(),
        status: "InProgress".to_string(),
        asset_name: Some("Production Line 1".to_string()),
    })
}

#[derive(Deserialize)]
pub struct AssignRequest { pub assigned_to: Uuid }

pub async fn assign_work_order(State(_state): State<AppState>, Json(_req): Json<AssignRequest>) -> Json<WorkOrderResponse> {
    Json(WorkOrderResponse {
        id: Uuid::new_v4(),
        wo_number: "WO-001".to_string(),
        description: "Repair".to_string(),
        work_order_type: "Corrective".to_string(),
        priority: "High".to_string(),
        status: "Scheduled".to_string(),
        asset_name: None,
    })
}

pub async fn start_work_order(State(_state): State<AppState>) -> Json<WorkOrderResponse> {
    Json(WorkOrderResponse {
        id: Uuid::new_v4(),
        wo_number: "WO-001".to_string(),
        description: "Repair".to_string(),
        work_order_type: "Corrective".to_string(),
        priority: "High".to_string(),
        status: "InProgress".to_string(),
        asset_name: None,
    })
}

#[derive(Deserialize)]
pub struct CompleteRequest { pub completion_notes: Option<String>, pub actual_hours: f64 }

pub async fn complete_work_order(State(_state): State<AppState>, Json(_req): Json<CompleteRequest>) -> Json<WorkOrderResponse> {
    Json(WorkOrderResponse {
        id: Uuid::new_v4(),
        wo_number: "WO-001".to_string(),
        description: "Repair".to_string(),
        work_order_type: "Corrective".to_string(),
        priority: "High".to_string(),
        status: "Completed".to_string(),
        asset_name: None,
    })
}

pub async fn close_work_order(State(_state): State<AppState>) -> Json<WorkOrderResponse> {
    Json(WorkOrderResponse {
        id: Uuid::new_v4(),
        wo_number: "WO-001".to_string(),
        description: "Repair".to_string(),
        work_order_type: "Corrective".to_string(),
        priority: "High".to_string(),
        status: "Closed".to_string(),
        asset_name: None,
    })
}

#[derive(Serialize)]
pub struct TaskResponse { pub id: Uuid, pub task_number: i32, pub description: String, pub completed: bool }

#[derive(Deserialize)]
pub struct AddTaskRequest { pub description: String, pub estimated_hours: f64 }

pub async fn add_task(State(_state): State<AppState>, Json(_req): Json<AddTaskRequest>) -> Json<TaskResponse> {
    Json(TaskResponse { id: Uuid::new_v4(), task_number: 1, description: "Check oil levels".to_string(), completed: false })
}

pub async fn list_tasks(State(_state): State<AppState>) -> Json<Vec<TaskResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct LaborResponse { pub id: Uuid, pub employee_id: Uuid, pub hours: f64, pub cost: i64 }

#[derive(Deserialize)]
pub struct AddLaborRequest { pub employee_id: Uuid, pub hours: f64 }

pub async fn add_labor(State(_state): State<AppState>, Json(_req): Json<AddLaborRequest>) -> Json<LaborResponse> {
    Json(LaborResponse { id: Uuid::new_v4(), employee_id: Uuid::new_v4(), hours: 2.5, cost: 7500 })
}

pub async fn list_labor(State(_state): State<AppState>) -> Json<Vec<LaborResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct PartResponse { pub id: Uuid, pub product_id: Uuid, pub quantity: i64, pub cost: i64 }

#[derive(Deserialize)]
pub struct AddPartRequest { pub product_id: Uuid, pub quantity: i64 }

pub async fn add_part(State(_state): State<AppState>, Json(_req): Json<AddPartRequest>) -> Json<PartResponse> {
    Json(PartResponse { id: Uuid::new_v4(), product_id: Uuid::new_v4(), quantity: 2, cost: 5000 })
}

pub async fn list_parts(State(_state): State<AppState>) -> Json<Vec<PartResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct PMScheduleResponse {
    pub id: Uuid,
    pub pm_number: String,
    pub name: String,
    pub asset_name: String,
    pub frequency: String,
    pub next_due_date: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreatePMScheduleRequest {
    pub name: String,
    pub asset_id: Uuid,
    pub maintenance_strategy: String,
    pub frequency_type: String,
    pub frequency_value: i32,
}

pub async fn create_pm_schedule(State(_state): State<AppState>, Json(_req): Json<CreatePMScheduleRequest>) -> Json<PMScheduleResponse> {
    let next_due = chrono::Utc::now() + chrono::Duration::days(30);
    Json(PMScheduleResponse {
        id: Uuid::new_v4(),
        pm_number: format!("PM-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
        name: "Monthly Inspection".to_string(),
        asset_name: "Production Line 1".to_string(),
        frequency: "Monthly".to_string(),
        next_due_date: next_due.format("%Y-%m-%d").to_string(),
        status: "Active".to_string(),
    })
}

pub async fn list_pm_schedules(State(_state): State<AppState>) -> Json<Vec<PMScheduleResponse>> { Json(vec![]) }

pub async fn get_pm_schedule(State(_state): State<AppState>) -> Json<PMScheduleResponse> {
    let next_due = chrono::Utc::now() + chrono::Duration::days(90);
    Json(PMScheduleResponse {
        id: Uuid::new_v4(),
        pm_number: "PM-001".to_string(),
        name: "Quarterly Service".to_string(),
        asset_name: "HVAC System".to_string(),
        frequency: "Quarterly".to_string(),
        next_due_date: next_due.format("%Y-%m-%d").to_string(),
        status: "Active".to_string(),
    })
}

#[derive(Serialize)]
pub struct PMTaskResponse { pub id: Uuid, pub task_number: i32, pub description: String }

#[derive(Deserialize)]
pub struct AddPMTaskRequest { pub description: String, pub estimated_minutes: i32 }

pub async fn add_pm_task(State(_state): State<AppState>, Json(_req): Json<AddPMTaskRequest>) -> Json<PMTaskResponse> {
    Json(PMTaskResponse { id: Uuid::new_v4(), task_number: 1, description: "Check belt tension".to_string() })
}

pub async fn list_pm_tasks(State(_state): State<AppState>) -> Json<Vec<PMTaskResponse>> { Json(vec![]) }

pub async fn generate_pm_work_order(State(_state): State<AppState>) -> Json<WorkOrderResponse> {
    Json(WorkOrderResponse {
        id: Uuid::new_v4(),
        wo_number: format!("WO-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
        description: "PM Work Order".to_string(),
        work_order_type: "Preventive".to_string(),
        priority: "Scheduled".to_string(),
        status: "Scheduled".to_string(),
        asset_name: Some("Equipment".to_string()),
    })
}

#[derive(Serialize)]
pub struct FailureCodeResponse { pub id: Uuid, pub code: String, pub description: String, pub problem_type: String }

#[derive(Deserialize)]
pub struct CreateFailureCodeRequest { pub code: String, pub description: String, pub problem_type: String }

pub async fn create_failure_code(State(_state): State<AppState>, Json(_req): Json<CreateFailureCodeRequest>) -> Json<FailureCodeResponse> {
    Json(FailureCodeResponse { id: Uuid::new_v4(), code: "FC-001".to_string(), description: "Motor failure".to_string(), problem_type: "Electrical".to_string() })
}

pub async fn list_failure_codes(State(_state): State<AppState>) -> Json<Vec<FailureCodeResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct LocationResponse { pub id: Uuid, pub location_code: String, pub name: String, pub building: Option<String> }

#[derive(Deserialize)]
pub struct CreateLocationRequest { pub location_code: String, pub name: String, pub building: Option<String> }

pub async fn create_location(State(_state): State<AppState>, Json(_req): Json<CreateLocationRequest>) -> Json<LocationResponse> {
    Json(LocationResponse { id: Uuid::new_v4(), location_code: "LOC-001".to_string(), name: "Production Floor".to_string(), building: Some("Building A".to_string()) })
}

pub async fn list_locations(State(_state): State<AppState>) -> Json<Vec<LocationResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct SparePartResponse { pub id: Uuid, pub part_number: String, pub name: String, pub current_stock: i64, pub reorder_point: i64 }

#[derive(Deserialize)]
pub struct CreateSparePartRequest { pub part_number: String, pub name: String, pub unit_of_measure: String, pub warehouse_id: Uuid }

pub async fn create_spare_part(State(_state): State<AppState>, Json(_req): Json<CreateSparePartRequest>) -> Json<SparePartResponse> {
    Json(SparePartResponse { id: Uuid::new_v4(), part_number: "SP-001".to_string(), name: "Bearing".to_string(), current_stock: 10, reorder_point: 5 })
}

pub async fn list_spare_parts(State(_state): State<AppState>) -> Json<Vec<SparePartResponse>> { Json(vec![]) }

pub async fn get_spare_part(State(_state): State<AppState>) -> Json<SparePartResponse> {
    Json(SparePartResponse { id: Uuid::new_v4(), part_number: "SP-001".to_string(), name: "Belt".to_string(), current_stock: 15, reorder_point: 5 })
}

#[derive(Serialize)]
pub struct BudgetResponse { pub id: Uuid, pub name: String, pub fiscal_year: i32, pub total_budget: i64, pub spent_to_date: i64 }

#[derive(Deserialize)]
pub struct CreateBudgetRequest { pub name: String, pub fiscal_year: i32, pub total_budget: i64 }

pub async fn create_budget(State(_state): State<AppState>, Json(_req): Json<CreateBudgetRequest>) -> Json<BudgetResponse> {
    Json(BudgetResponse { id: Uuid::new_v4(), name: "Maintenance Budget".to_string(), fiscal_year: 2024, total_budget: 50000000, spent_to_date: 0 })
}

pub async fn list_budgets(State(_state): State<AppState>) -> Json<Vec<BudgetResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct KPIResponse { pub kpi_type: String, pub value: f64, pub target: f64, pub period: String }
pub async fn list_kpis(State(_state): State<AppState>) -> Json<Vec<KPIResponse>> {
    let current_period = chrono::Utc::now().format("%Y-%m").to_string();
    Json(vec![
        KPIResponse { kpi_type: "MTBF".to_string(), value: 720.0, target: 800.0, period: current_period.clone() },
        KPIResponse { kpi_type: "MTTR".to_string(), value: 4.5, target: 4.0, period: current_period.clone() },
        KPIResponse { kpi_type: "Availability".to_string(), value: 95.5, target: 98.0, period: current_period },
    ])
}

#[derive(Serialize)]
pub struct ContractResponse { pub id: Uuid, pub contract_number: String, pub vendor_name: String, pub end_date: String, pub status: String }

#[derive(Deserialize)]
pub struct CreateContractRequest { pub vendor_id: Uuid, pub contract_type: String, pub start_date: String, pub end_date: String, pub annual_cost: i64 }

pub async fn create_contract(State(_state): State<AppState>, Json(_req): Json<CreateContractRequest>) -> Json<ContractResponse> {
    let end_of_year = chrono::Utc::now().with_month(12).unwrap_or(chrono::Utc::now()).with_day(31).unwrap_or(chrono::Utc::now());
    Json(ContractResponse { id: Uuid::new_v4(), contract_number: "SC-001".to_string(), vendor_name: "Service Co".to_string(), end_date: end_of_year.format("%Y-%m-%d").to_string(), status: "Active".to_string() })
}

pub async fn list_contracts(State(_state): State<AppState>) -> Json<Vec<ContractResponse>> { Json(vec![]) }
