use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Pagination, Money, Currency, ContactInfo, Address};
use erp_hr::{Employee, Payroll, EmployeeService, AttendanceService, PayrollService};

#[derive(Deserialize)] pub struct CreateEmployeeRequest { pub employee_number: String, pub first_name: String, pub last_name: String, pub email: String, pub phone: Option<String>, pub hire_date: String }
#[derive(Serialize)] pub struct EmployeeResponse { pub id: Uuid, pub employee_number: String, pub first_name: String, pub last_name: String, pub email: String, pub status: String }

impl From<Employee> for EmployeeResponse {
    fn from(e: Employee) -> Self { Self { id: e.base.id, employee_number: e.employee_number, first_name: e.first_name, last_name: e.last_name, email: e.email, status: format!("{:?}", e.status) } }
}

pub async fn list_employees(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<EmployeeResponse>>> {
    let svc = EmployeeService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(res.items.into_iter().map(EmployeeResponse::from).collect(), res.total, Pagination { page: res.page, per_page: res.per_page })))
}

pub async fn get_employee(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<EmployeeResponse>> {
    Ok(Json(EmployeeResponse::from(EmployeeService::new().get(&state.pool, id).await?)))
}

pub async fn create_employee(State(state): State<AppState>, Json(req): Json<CreateEmployeeRequest>) -> ApiResult<Json<EmployeeResponse>> {
    let svc = EmployeeService::new();
    let hire_date = NaiveDate::parse_from_str(&req.hire_date, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid hire_date format, expected YYYY-MM-DD"))?;
    let e = Employee {
        base: BaseEntity::new(), employee_number: req.employee_number, first_name: req.first_name, last_name: req.last_name, email: req.email.clone(),
        contact: ContactInfo { email: Some(req.email), phone: req.phone, fax: None, website: None },
        address: Address { street: String::new(), city: String::new(), state: None, postal_code: String::new(), country: String::new() },
        birth_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
        hire_date,
        termination_date: None, department_id: None, position_id: None, manager_id: None, status: Status::Active,
    };
    Ok(Json(EmployeeResponse::from(svc.create(&state.pool, e).await?)))
}

#[derive(Deserialize)] pub struct AttendanceRequest { pub employee_id: Uuid }

pub async fn check_in(State(state): State<AppState>, Json(req): Json<AttendanceRequest>) -> ApiResult<Json<serde_json::Value>> {
    AttendanceService::new().check_in(&state.pool, req.employee_id).await?;
    Ok(Json(serde_json::json!({ "status": "checked_in" })))
}

pub async fn check_out(State(state): State<AppState>, Json(req): Json<AttendanceRequest>) -> ApiResult<Json<serde_json::Value>> {
    AttendanceService::new().check_out(&state.pool, req.employee_id).await?;
    Ok(Json(serde_json::json!({ "status": "checked_out" })))
}

pub async fn list_leave_requests(State(_state): State<AppState>, Query(_pagination): Query<Pagination>) -> ApiResult<Json<Vec<serde_json::Value>>> { Ok(Json(vec![])) }
pub async fn create_leave_request(State(_state): State<AppState>, Json(_req): Json<serde_json::Value>) -> ApiResult<Json<serde_json::Value>> { Ok(Json(serde_json::json!({"id": Uuid::new_v4()}))) }

#[derive(Deserialize)] pub struct CreatePayrollRequest { pub employee_id: Uuid, pub period_start: String, pub period_end: String, pub base_salary: i64, pub overtime: Option<i64>, pub bonuses: Option<i64>, pub deductions: Option<i64> }
#[derive(Serialize)] pub struct PayrollResponse { pub id: Uuid, pub employee_id: Uuid, pub period_start: String, pub period_end: String, pub net_salary: f64, pub status: String }

impl From<Payroll> for PayrollResponse {
    fn from(p: Payroll) -> Self { Self { id: p.base.id, employee_id: p.employee_id, period_start: p.period_start.to_string(), period_end: p.period_end.to_string(), net_salary: p.net_salary.to_decimal(), status: format!("{:?}", p.status) } }
}

pub async fn list_payroll(State(_state): State<AppState>, Query(_pagination): Query<Pagination>) -> ApiResult<Json<Vec<PayrollResponse>>> {
    Ok(Json(vec![]))
}

pub async fn create_payroll(State(state): State<AppState>, Json(req): Json<CreatePayrollRequest>) -> ApiResult<Json<PayrollResponse>> {
    let svc = PayrollService::new();
    let period_start = NaiveDate::parse_from_str(&req.period_start, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid period_start format, expected YYYY-MM-DD"))?;
    let period_end = NaiveDate::parse_from_str(&req.period_end, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid period_end format, expected YYYY-MM-DD"))?;
    let p = Payroll {
        base: BaseEntity::new(), employee_id: req.employee_id,
        period_start,
        period_end,
        base_salary: Money::new(req.base_salary, Currency::USD), overtime: Money::new(req.overtime.unwrap_or(0), Currency::USD),
        bonuses: Money::new(req.bonuses.unwrap_or(0), Currency::USD), deductions: Money::new(req.deductions.unwrap_or(0), Currency::USD),
        net_salary: Money::zero(Currency::USD), status: Status::Draft,
    };
    Ok(Json(PayrollResponse::from(svc.create(&state.pool, p).await?)))
}
