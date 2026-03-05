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
type PerformanceGoalRow = (
    String, String, String, String, Option<String>,
    i64, Option<String>, Option<String>, Option<i64>, Option<i64>, Option<i64>, String,
);
type PerformanceReviewRow = (
    String, String, String, String, String,
    Option<i64>, Option<String>, Option<String>, Option<String>, Option<String>, String,
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
pub async fn get_employee_names(pool: &sqlx::SqlitePool, employee_ids: &[String]) -> ApiResult<HashMap<String, String>> {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePerformanceCycleRequest {
    pub name: String,
    pub cycle_type: String,
    pub start_date: String,
    pub end_date: String,
    pub review_due_date: String,
}
#[derive(Serialize)]
pub struct PerformanceCycleResponse {
    pub id: Uuid,
    pub name: String,
    pub cycle_type: String,
    pub start_date: String,
    pub end_date: String,
    pub review_due_date: String,
    pub status: String,
    pub created_at: String,
}
#[derive(Serialize)]
pub struct PerformanceGoalResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub cycle_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub weight: i32,
    pub target_value: Option<String>,
    pub actual_value: Option<String>,
    pub self_rating: Option<i32>,
    pub manager_rating: Option<i32>,
    pub final_rating: Option<i32>,
    pub status: String,
}
#[derive(Serialize)]
pub struct PerformanceReviewResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
    pub cycle_id: Uuid,
    pub review_type: String,
    pub overall_rating: Option<i32>,
    pub strengths: Option<String>,
    pub areas_for_improvement: Option<String>,
    pub comments: Option<String>,
    pub submitted_at: Option<String>,
    pub status: String,
}
type PerformanceCycleRow = (String, String, String, String, String, String, String, String);

pub async fn list_performance_cycles(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<PerformanceCycleResponse>>> {
    let rows: Vec<PerformanceCycleRow> = sqlx::query_as(
        "SELECT id, name, cycle_type, start_date, end_date, review_due_date, status, created_at FROM performance_cycles ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await?;
    
    Ok(Json(rows.into_iter().map(|r| PerformanceCycleResponse {
        id: Uuid::parse_str(&r.0).unwrap_or_default(),
        name: r.1,
        cycle_type: r.2,
        start_date: r.3,
        end_date: r.4,
        review_due_date: r.5,
        status: r.6,
        created_at: r.7,
    }).collect()))
}
pub async fn create_performance_cycle(
    State(state): State<AppState>,
    Json(req): Json<CreatePerformanceCycleRequest>,
) -> ApiResult<Json<PerformanceCycleResponse>> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO performance_cycles (id, name, cycle_type, start_date, end_date, review_due_date, status, created_at) VALUES (?, ?, ?, ?, ?, ?, 'Draft', ?)"
    )
    .bind(id.to_string())
    .bind(&req.name)
    .bind(&req.cycle_type)
    .bind(&req.start_date)
    .bind(&req.end_date)
    .bind(&req.review_due_date)
    .bind(now.to_rfc3339())
    .execute(&state.pool)
    .await?;
    
    Ok(Json(PerformanceCycleResponse {
        id,
        name: req.name,
        cycle_type: req.cycle_type,
        start_date: req.start_date,
        end_date: req.end_date,
        review_due_date: req.review_due_date,
        status: "Draft".to_string(),
        created_at: now.to_rfc3339(),
    }))
}
pub async fn activate_performance_cycle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PerformanceCycleResponse>> {
    sqlx::query("UPDATE performance_cycles SET status = 'Active' WHERE id = ?")
        .bind(id.to_string())
        .execute(&state.pool)
        .await?;
    
    let row: (String, String, String, String, String, String, String, String) = sqlx::query_as(
        "SELECT id, name, cycle_type, start_date, end_date, review_due_date, status, created_at FROM performance_cycles WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_one(&state.pool)
    .await?;
    
    Ok(Json(PerformanceCycleResponse {
        id: Uuid::parse_str(&row.0).unwrap_or_default(),
        name: row.1,
        cycle_type: row.2,
        start_date: row.3,
        end_date: row.4,
        review_due_date: row.5,
        status: row.6,
        created_at: row.7,
    }))
}
pub async fn close_performance_cycle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PerformanceCycleResponse>> {
    sqlx::query("UPDATE performance_cycles SET status = 'Closed' WHERE id = ?")
        .bind(id.to_string())
        .execute(&state.pool)
        .await?;
    
    let row: (String, String, String, String, String, String, String, String) = sqlx::query_as(
        "SELECT id, name, cycle_type, start_date, end_date, review_due_date, status, created_at FROM performance_cycles WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_one(&state.pool)
    .await?;
    
    Ok(Json(PerformanceCycleResponse {
        id: Uuid::parse_str(&row.0).unwrap_or_default(),
        name: row.1,
        cycle_type: row.2,
        start_date: row.3,
        end_date: row.4,
        review_due_date: row.5,
        status: row.6,
        created_at: row.7,
    }))
}
#[derive(Deserialize)]
pub struct CreatePerformanceGoalRequest {
    pub employee_id: Uuid,
    pub cycle_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub weight: i32,
    pub target_value: Option<String>,
}
pub async fn list_performance_goals(
    State(state): State<AppState>,
    Query(cycle_id): Query<Option<Uuid>>,
) -> ApiResult<Json<Vec<PerformanceGoalResponse>>> {
    let sql = if let Some(_cid) = cycle_id {
        "SELECT id, employee_id, cycle_id, title, description, weight, target_value, actual_value, self_rating, manager_rating, final_rating, status FROM performance_goals WHERE cycle_id = ? ORDER BY title"
    } else {
        "SELECT id, employee_id, cycle_id, title, description, weight, target_value, actual_value, self_rating, manager_rating, final_rating, status FROM performance_goals ORDER BY title"
    };
    
    let mut query = sqlx::query_as::<_, PerformanceGoalRow>(sql);
    if let Some(id) = cycle_id {
        query = query.bind(id.to_string());
    }
    
    let rows: Vec<PerformanceGoalRow> = query.fetch_all(&state.pool).await?;
    
    Ok(Json(rows.into_iter().map(|r| PerformanceGoalResponse {
        id: Uuid::parse_str(&r.0).unwrap_or_default(),
        employee_id: Uuid::parse_str(&r.1).unwrap_or_default(),
        cycle_id: Uuid::parse_str(&r.2).unwrap_or_default(),
        title: r.3,
        description: r.4,
        weight: r.5 as i32,
        target_value: r.6,
        actual_value: r.7,
        self_rating: r.8.map(|v| v as i32),
        manager_rating: r.9.map(|v| v as i32),
        final_rating: r.10.map(|v| v as i32),
        status: r.11,
    }).collect()))
}
pub async fn create_performance_goal(
    State(state): State<AppState>,
    Json(req): Json<CreatePerformanceGoalRequest>,
) -> ApiResult<Json<PerformanceGoalResponse>> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO performance_goals (id, employee_id, cycle_id, title, description, weight, target_value, status) VALUES (?, ?, ?, ?, ?, ?, ?, 'Draft')"
    )
    .bind(id.to_string())
    .bind(req.employee_id.to_string())
    .bind(req.cycle_id.to_string())
    .bind(&req.title)
    .bind(&req.description)
    .bind(req.weight)
    .bind(&req.target_value)
    .execute(&state.pool)
    .await?;
    
    Ok(Json(PerformanceGoalResponse {
        id,
        employee_id: req.employee_id,
        cycle_id: req.cycle_id,
        title: req.title,
        description: req.description,
        weight: req.weight,
        target_value: req.target_value,
        actual_value: None,
        self_rating: None,
        manager_rating: None,
        final_rating: None,
        status: "Draft".to_string(),
    }))
}
#[derive(Deserialize)]
pub struct UpdateGoalRatingRequest {
    pub rating_type: String,
    pub rating: i32,
    pub actual_value: Option<String>,
}
pub async fn update_goal_rating(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateGoalRatingRequest>,
) -> ApiResult<Json<PerformanceGoalResponse>> {
    match req.rating_type.as_str() {
        "self" => {
            sqlx::query("UPDATE performance_goals SET self_rating = ?, actual_value = ?, status = 'Pending' WHERE id = ?")
                .bind(req.rating)
                .bind(&req.actual_value)
                .bind(id.to_string())
                .execute(&state.pool)
                .await?;
        }
        "manager" => {
            sqlx::query("UPDATE performance_goals SET manager_rating = ?, status = 'Approved' WHERE id = ?")
                .bind(req.rating)
                .bind(id.to_string())
                .execute(&state.pool)
                .await?;
        }
        _ => return Err(crate::error::ApiError(erp_core::Error::validation("Invalid rating type"))),
    }
    
    let row: PerformanceGoalRow = sqlx::query_as(
        "SELECT id, employee_id, cycle_id, title, description, weight, target_value, actual_value, self_rating, manager_rating, final_rating, status FROM performance_goals WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| erp_core::Error::not_found("PerformanceGoal", &id.to_string()))?;
    
    Ok(Json(PerformanceGoalResponse {
        id: Uuid::parse_str(&row.0).unwrap_or_default(),
        employee_id: Uuid::parse_str(&row.1).unwrap_or_default(),
        cycle_id: Uuid::parse_str(&row.2).unwrap_or_default(),
        title: row.3,
        description: row.4,
        weight: row.5 as i32,
        target_value: row.6,
        actual_value: row.7,
        self_rating: row.8.map(|v| v as i32),
        manager_rating: row.9.map(|v| v as i32),
        final_rating: row.10.map(|v| v as i32),
        status: row.11,
    }))
}
#[derive(Deserialize)]
pub struct CreatePerformanceReviewRequest {
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
    pub cycle_id: Uuid,
    pub review_type: String,
}
pub async fn list_performance_reviews(
    State(state): State<AppState>,
    Query(cycle_id): Query<Option<Uuid>>,
) -> ApiResult<Json<Vec<PerformanceReviewResponse>>> {
    let sql = if let Some(_cid) = cycle_id {
        "SELECT id, employee_id, reviewer_id, cycle_id, review_type, overall_rating, strengths, areas_for_improvement, comments, submitted_at, status FROM performance_reviews WHERE cycle_id = ? ORDER BY created_at DESC"
    } else {
        "SELECT id, employee_id, reviewer_id, cycle_id, review_type, overall_rating, strengths, areas_for_improvement, comments, submitted_at, status FROM performance_reviews ORDER BY created_at DESC"
    };
    
    let mut query = sqlx::query_as::<_, PerformanceReviewRow>(sql);
    if let Some(id) = cycle_id {
        query = query.bind(id.to_string());
    }
    
    let rows = query.fetch_all(&state.pool).await?;
    
    Ok(Json(rows.into_iter().map(|r| PerformanceReviewResponse {
        id: Uuid::parse_str(&r.0).unwrap_or_default(),
        employee_id: Uuid::parse_str(&r.1).unwrap_or_default(),
        reviewer_id: Uuid::parse_str(&r.2).unwrap_or_default(),
        cycle_id: Uuid::parse_str(&r.3).unwrap_or_default(),
        review_type: r.4,
        overall_rating: r.5.map(|v| v as i32),
        strengths: r.6,
        areas_for_improvement: r.7,
        comments: r.8,
        submitted_at: r.9,
        status: r.10,
    }).collect()))
}
pub async fn create_performance_review(
    State(state): State<AppState>,
    Json(req): Json<CreatePerformanceReviewRequest>,
) -> ApiResult<Json<PerformanceReviewResponse>> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO performance_reviews (id, employee_id, reviewer_id, cycle_id, review_type, status) VALUES (?, ?, ?, ?, ?, 'Draft')"
    )
    .bind(id.to_string())
    .bind(req.employee_id.to_string())
    .bind(req.reviewer_id.to_string())
    .bind(req.cycle_id.to_string())
    .bind(&req.review_type)
    .execute(&state.pool)
    .await?;
    
    Ok(Json(PerformanceReviewResponse {
        id,
        employee_id: req.employee_id,
        reviewer_id: req.reviewer_id,
        cycle_id: req.cycle_id,
        review_type: req.review_type,
        overall_rating: None,
        strengths: None,
        areas_for_improvement: None,
        comments: None,
        submitted_at: None,
        status: "Draft".to_string(),
    }))
}
#[derive(Deserialize)]
pub struct SubmitReviewRequest {
    pub overall_rating: i32,
    pub strengths: Option<String>,
    pub areas_for_improvement: Option<String>,
    pub comments: Option<String>,
}
pub async fn submit_performance_review(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<SubmitReviewRequest>,
) -> ApiResult<Json<PerformanceReviewResponse>> {
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE performance_reviews SET overall_rating = ?, strengths = ?, areas_for_improvement = ?, comments = ?, submitted_at = ?, status = 'Submitted' WHERE id = ?"
    )
    .bind(req.overall_rating)
    .bind(&req.strengths)
    .bind(&req.areas_for_improvement)
    .bind(&req.comments)
    .bind(&now)
    .bind(id.to_string())
    .execute(&state.pool)
    .await?;
    
    let row: PerformanceReviewRow = sqlx::query_as(
        "SELECT id, employee_id, reviewer_id, cycle_id, review_type, overall_rating, strengths, areas_for_improvement, comments, submitted_at, status FROM performance_reviews WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| erp_core::Error::not_found("PerformanceReview", &id.to_string()))?;
    
    Ok(Json(PerformanceReviewResponse {
        id: Uuid::parse_str(&row.0).unwrap_or_default(),
        employee_id: Uuid::parse_str(&row.1).unwrap_or_default(),
        reviewer_id: Uuid::parse_str(&row.2).unwrap_or_default(),
        cycle_id: Uuid::parse_str(&row.3).unwrap_or_default(),
        review_type: row.4,
        overall_rating: row.5.map(|v| v as i32),
        strengths: row.6,
        areas_for_improvement: row.7,
        comments: row.8,
        submitted_at: row.9,
        status: row.10,
    }))
}
