use std::collections::HashMap;

use axum::{extract::{Path, Query, State}, Json};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Pagination, ContactInfo, Address};
use erp_hr::{Employee, Payroll, PayrollRun, EmployeeService, AttendanceService, FullPayrollService};

type PayrollRunRow = (
    String, String, String, String, String,
    i64, i64, i64,
    String, Option<String>, Option<String>, String,
);

type PayrollEntryRow = (
    String, String, String,
    i64, i64, i64,
    String, String,
);

#[derive(Deserialize)]
pub struct CreateEmployeeRequest {
    pub employee_number: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub hire_date: String,
}

#[derive(Serialize)]
pub struct EmployeeResponse {
    pub id: Uuid,
    pub employee_number: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub status: String,
}

impl From<Employee> for EmployeeResponse {
    fn from(e: Employee) -> Self {
        Self {
            id: e.base.id,
            employee_number: e.employee_number,
            first_name: e.first_name,
            last_name: e.last_name,
            email: e.email,
            status: format!("{:?}", e.status),
        }
    }
}

pub async fn list_employees(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<EmployeeResponse>>> {
    let svc = EmployeeService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(EmployeeResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn get_employee(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<EmployeeResponse>> {
    Ok(Json(EmployeeResponse::from(
        EmployeeService::new().get(&state.pool, id).await?,
    )))
}

pub async fn create_employee(
    State(state): State<AppState>,
    Json(req): Json<CreateEmployeeRequest>,
) -> ApiResult<Json<EmployeeResponse>> {
    let svc = EmployeeService::new();
    let hire_date = NaiveDate::parse_from_str(&req.hire_date, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid hire_date format, expected YYYY-MM-DD"))?;
    let e = Employee {
        base: BaseEntity::new(),
        employee_number: req.employee_number,
        first_name: req.first_name,
        last_name: req.last_name,
        email: req.email.clone(),
        contact: ContactInfo {
            email: Some(req.email),
            phone: req.phone,
            fax: None,
            website: None,
        },
        address: Address {
            street: String::new(),
            city: String::new(),
            state: None,
            postal_code: String::new(),
            country: String::new(),
        },
        birth_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_else(|| chrono::Utc::now().date_naive()),
        hire_date,
        termination_date: None,
        department_id: None,
        position_id: None,
        manager_id: None,
        status: Status::Active,
    };
    Ok(Json(EmployeeResponse::from(svc.create(&state.pool, e).await?)))
}

#[derive(Deserialize)]
pub struct AttendanceRequest {
    pub employee_id: Uuid,
}

pub async fn check_in(
    State(state): State<AppState>,
    Json(req): Json<AttendanceRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    AttendanceService::new().check_in(&state.pool, req.employee_id).await?;
    Ok(Json(serde_json::json!({ "status": "checked_in" })))
}

pub async fn check_out(
    State(state): State<AppState>,
    Json(req): Json<AttendanceRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    AttendanceService::new().check_out(&state.pool, req.employee_id).await?;
    Ok(Json(serde_json::json!({ "status": "checked_out" })))
}

pub async fn list_leave_requests(
    State(_state): State<AppState>,
    Query(_pagination): Query<Pagination>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    Ok(Json(vec![]))
}

pub async fn create_leave_request(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({"id": Uuid::new_v4()})))
}

#[derive(Deserialize)]
pub struct CreatePayrollRequest {
    pub employee_id: Uuid,
    pub period_start: String,
    pub period_end: String,
    pub base_salary: i64,
    pub overtime: Option<i64>,
    pub bonuses: Option<i64>,
    pub deductions: Option<i64>,
}

#[derive(Serialize)]
pub struct PayrollResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub period_start: String,
    pub period_end: String,
    pub net_salary: f64,
    pub status: String,
}

impl From<Payroll> for PayrollResponse {
    fn from(p: Payroll) -> Self {
        Self {
            id: p.base.id,
            employee_id: p.employee_id,
            period_start: p.period_start.to_string(),
            period_end: p.period_end.to_string(),
            net_salary: p.net_salary.to_decimal(),
            status: format!("{:?}", p.status),
        }
    }
}

#[derive(Serialize)]
pub struct PayrollRunResponse {
    pub id: Uuid,
    pub run_number: String,
    pub pay_period_start: String,
    pub pay_period_end: String,
    pub pay_date: String,
    pub total_gross: i64,
    pub total_deductions: i64,
    pub total_net: i64,
    pub status: String,
    pub processed_at: Option<String>,
    pub approved_at: Option<String>,
    pub created_at: String,
}

impl From<PayrollRun> for PayrollRunResponse {
    fn from(r: PayrollRun) -> Self {
        Self {
            id: r.id,
            run_number: r.run_number,
            pay_period_start: r.pay_period_start.to_string(),
            pay_period_end: r.pay_period_end.to_string(),
            pay_date: r.pay_date.to_string(),
            total_gross: r.total_gross,
            total_deductions: r.total_deductions,
            total_net: r.total_net,
            status: format!("{:?}", r.status),
            processed_at: r.processed_at.map(|t| t.to_rfc3339()),
            approved_at: r.approved_at.map(|t| t.to_rfc3339()),
            created_at: r.created_at.to_rfc3339(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreatePayrollRunRequest {
    pub pay_period_start: String,
    pub pay_period_end: String,
    pub pay_date: String,
}

#[derive(Serialize)]
pub struct PayrollEntryResponse {
    pub id: Uuid,
    pub payroll_run_id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub gross_pay: i64,
    pub total_deductions: i64,
    pub net_pay: i64,
    pub payment_method: String,
    pub status: String,
}

pub async fn list_payroll_runs(State(state): State<AppState>) -> ApiResult<Json<Vec<PayrollRunResponse>>> {
    let rows: Vec<PayrollRunRow> = sqlx::query_as(
        "SELECT id, run_number, pay_period_start, pay_period_end, pay_date, total_gross, total_deductions, total_net, status, processed_at, approved_at, created_at FROM payroll_runs ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await?;
    
    Ok(Json(rows.into_iter().map(|r| PayrollRunResponse {
        id: Uuid::parse_str(&r.0).unwrap_or_default(),
        run_number: r.1,
        pay_period_start: r.2,
        pay_period_end: r.3,
        pay_date: r.4,
        total_gross: r.5,
        total_deductions: r.6,
        total_net: r.7,
        status: r.8,
        processed_at: r.9,
        approved_at: r.10,
        created_at: r.11,
    }).collect()))
}

pub async fn create_payroll_run(
    State(state): State<AppState>,
    Json(req): Json<CreatePayrollRunRequest>,
) -> ApiResult<Json<PayrollRunResponse>> {
    let run = FullPayrollService::create_payroll_run(&state.pool, &req.pay_period_start, &req.pay_period_end, &req.pay_date).await?;
    Ok(Json(PayrollRunResponse::from(run)))
}

pub async fn get_payroll_run(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PayrollRunResponse>> {
    let run = FullPayrollService::get_payroll_run(&state.pool, id).await?;
    Ok(Json(PayrollRunResponse::from(run)))
}

pub async fn process_payroll_run(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PayrollRunResponse>> {
    let now = Utc::now();
    sqlx::query("UPDATE payroll_runs SET status = 'Processed', processed_at = ? WHERE id = ?")
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(&state.pool)
        .await?;
    get_payroll_run(State(state), Path(id)).await
}

pub async fn approve_payroll_run(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PayrollRunResponse>> {
    let run = FullPayrollService::approve_payroll(&state.pool, id).await?;
    Ok(Json(PayrollRunResponse::from(run)))
}

pub async fn pay_payroll_run(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query("UPDATE payroll_runs SET status = 'Paid' WHERE id = ?")
        .bind(id.to_string())
        .execute(&state.pool)
        .await?;
    sqlx::query("UPDATE payroll_entries SET status = 'Paid' WHERE payroll_run_id = ?")
        .bind(id.to_string())
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({ "status": "Paid" })))
}

pub async fn list_payroll_entries(
    State(state): State<AppState>,
    Path(run_id): Path<Uuid>,
) -> ApiResult<Json<Vec<PayrollEntryResponse>>> {
    let rows: Vec<PayrollEntryRow> = sqlx::query_as(
        "SELECT pe.id, pe.payroll_run_id, pe.employee_id, pe.gross_pay, pe.total_deductions, pe.net_pay, pe.payment_method, pe.status FROM payroll_entries pe WHERE pe.payroll_run_id = ?"
    )
    .bind(run_id.to_string())
    .fetch_all(&state.pool)
    .await?;
    
    let employee_ids: Vec<String> = rows.iter().map(|r| r.2.clone()).collect();
    let employee_names: HashMap<String, String> = get_employee_names(&state.pool, &employee_ids).await?;
    
    Ok(Json(rows.into_iter().map(|r| PayrollEntryResponse {
        id: Uuid::parse_str(&r.0).unwrap_or_default(),
        payroll_run_id: Uuid::parse_str(&r.1).unwrap_or_default(),
        employee_id: Uuid::parse_str(&r.2).unwrap_or_default(),
        employee_name: employee_names.get(&r.2).cloned().unwrap_or_default(),
        gross_pay: r.3,
        total_deductions: r.4,
        net_pay: r.5,
        payment_method: r.6,
        status: r.7,
    }).collect()))
}

async fn get_employee_names(pool: &sqlx::SqlitePool, employee_ids: &[String]) -> ApiResult<HashMap<String, String>> {
    if employee_ids.is_empty() {
        return Ok(HashMap::new());
    }
    
    let placeholders: Vec<&str> = employee_ids.iter().map(|_| "?").collect();
    let sql = format!(
        "SELECT id, first_name, last_name FROM employees WHERE id IN ({})",
        placeholders.join(",")
    );
    
    let mut query = sqlx::query_as::<_, (String, String, String)>(&sql);
    for id in employee_ids {
        query = query.bind(id);
    }
    
    let rows = query.fetch_all(pool).await?;
    let mut names = HashMap::new();
    for (id, first, last) in rows {
        names.insert(id, format!("{} {}", first, last));
    }
    Ok(names)
}
