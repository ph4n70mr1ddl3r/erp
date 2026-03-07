use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency};
use chrono::NaiveDate;
use crate::models::*;
use crate::repository::*;

fn rand_digits() -> u16 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u16)
        .unwrap_or(0)
}

pub struct EmployeeService { repo: SqliteEmployeeRepository }
impl Default for EmployeeService {
    fn default() -> Self {
        Self::new()
    }
}

impl EmployeeService {
    pub fn new() -> Self { Self { repo: SqliteEmployeeRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Employee> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Employee>> { self.repo.find_all(pool, pagination).await }
    
    pub async fn create(&self, pool: &SqlitePool, emp: Employee) -> Result<Employee> {
        if emp.employee_number.is_empty() { return Err(Error::validation("Employee number is required")); }
        if emp.first_name.is_empty() || emp.last_name.is_empty() { return Err(Error::validation("Employee name is required")); }
        if emp.email.is_empty() { return Err(Error::validation("Email is required")); }
        self.repo.create(pool, emp).await
    }
    
    pub async fn terminate(&self, pool: &SqlitePool, id: Uuid, date: NaiveDate) -> Result<()> {
        self.repo.terminate(pool, id, date).await
    }
}

pub struct AttendanceService { repo: SqliteAttendanceRepository }
impl Default for AttendanceService {
    fn default() -> Self {
        Self::new()
    }
}

impl AttendanceService {
    pub fn new() -> Self { Self { repo: SqliteAttendanceRepository } }
    pub async fn check_in(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<()> {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let date = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d")
            .map_err(|e| Error::internal(format!("Failed to parse date: {}", e)))?;
        self.repo.record_check_in(pool, employee_id, date).await
    }
    pub async fn check_out(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<()> {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let date = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d")
            .map_err(|e| Error::internal(format!("Failed to parse date: {}", e)))?;
        self.repo.record_check_out(pool, employee_id, date).await
    }
}

pub struct PayrollService { repo: SqlitePayrollRepository }
impl Default for PayrollService {
    fn default() -> Self {
        Self::new()
    }
}

impl PayrollService {
    pub fn new() -> Self { Self { repo: SqlitePayrollRepository } }
    pub async fn list_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<Payroll>> {
        self.repo.find_by_employee(pool, employee_id).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, payroll: Payroll) -> Result<Payroll> {
        let net = payroll.base_salary.amount + payroll.overtime.amount + payroll.bonuses.amount - payroll.deductions.amount;
        let mut p = payroll;
        p.net_salary = Money::new(net, Currency::USD);
        p.base = BaseEntity::new();
        p.status = Status::Draft;
        self.repo.create(pool, p).await
    }
    
    pub async fn approve(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.approve(pool, id).await
    }
}

pub struct LeaveService;

impl Default for LeaveService {
    fn default() -> Self {
        Self::new()
    }
}

impl LeaveService {
    pub fn new() -> Self { Self }

    pub async fn create_leave_type(pool: &SqlitePool, name: &str, code: &str, days_per_year: i64, carry_over: bool) -> Result<LeaveTypeDef> {
        let lt = LeaveTypeDef {
            id: Uuid::new_v4(),
            name: name.to_string(),
            code: code.to_string(),
            days_per_year,
            carry_over,
            status: Status::Active,
        };
        
        sqlx::query(
            "INSERT INTO leave_types (id, name, code, days_per_year, carry_over, status)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(lt.id.to_string())
        .bind(&lt.name)
        .bind(&lt.code)
        .bind(lt.days_per_year)
        .bind(lt.carry_over as i64)
        .bind("Active")
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(lt)
    }

    pub async fn list_leave_types(pool: &SqlitePool) -> Result<Vec<LeaveTypeDef>> {
        let rows = sqlx::query_as::<_, LeaveTypeRow>(
            "SELECT id, name, code, days_per_year, carry_over, status FROM leave_types WHERE status = 'Active'"
        )
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_leave_balance(pool: &SqlitePool, employee_id: Uuid, leave_type_id: Uuid, year: i32) -> Result<LeaveBalance> {
        let row = sqlx::query_as::<_, LeaveBalanceRow>(
            "SELECT id, employee_id, leave_type_id, year, entitled, used, remaining, carried_over
             FROM leave_balances WHERE employee_id = ? AND leave_type_id = ? AND year = ?"
        )
        .bind(employee_id.to_string())
        .bind(leave_type_id.to_string())
        .bind(year)
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("LeaveBalance", &format!("{}:{}:{}", employee_id, leave_type_id, year)))?;
        
        Ok(row.into())
    }

    pub async fn create_leave_request(
        pool: &SqlitePool,
        employee_id: Uuid,
        leave_type_id: Uuid,
        start_date: &str,
        end_date: &str,
        days: i64,
        reason: Option<&str>,
    ) -> Result<LeaveRequestExtended> {
        let now = chrono::Utc::now();
        let req = LeaveRequestExtended {
            id: Uuid::new_v4(),
            employee_id,
            leave_type_id,
            start_date: chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
                .map_err(|_| Error::validation("Invalid start date"))?,
            end_date: chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
                .map_err(|_| Error::validation("Invalid end date"))?,
            days,
            reason: reason.map(|s| s.to_string()),
            status: LeaveRequestStatus::Pending,
            approved_by: None,
            approved_at: None,
            rejection_reason: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO leave_requests (id, employee_id, leave_type_id, start_date, end_date, days, reason, status, approved_by, approved_at, rejection_reason, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, 'Pending', NULL, NULL, NULL, ?)"
        )
        .bind(req.id.to_string())
        .bind(req.employee_id.to_string())
        .bind(req.leave_type_id.to_string())
        .bind(req.start_date.to_string())
        .bind(req.end_date.to_string())
        .bind(req.days)
        .bind(&req.reason)
        .bind(req.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(req)
    }

    pub async fn approve_leave_request(pool: &SqlitePool, id: Uuid, approver_id: Uuid) -> Result<LeaveRequestExtended> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE leave_requests SET status = 'Approved', approved_by = ?, approved_at = ? WHERE id = ?"
        )
        .bind(approver_id.to_string())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_leave_request(pool, id).await
    }

    pub async fn reject_leave_request(pool: &SqlitePool, id: Uuid, reason: &str) -> Result<LeaveRequestExtended> {
        let _now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE leave_requests SET status = 'Rejected', rejection_reason = ? WHERE id = ?"
        )
        .bind(reason)
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_leave_request(pool, id).await
    }

    pub async fn get_leave_request(pool: &SqlitePool, id: Uuid) -> Result<LeaveRequestExtended> {
        let row = sqlx::query_as::<_, LeaveRequestRow>(
            "SELECT id, employee_id, leave_type_id, start_date, end_date, days, reason, status, approved_by, approved_at, rejection_reason, created_at
             FROM leave_requests WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("LeaveRequest", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list_pending_leave_requests(pool: &SqlitePool) -> Result<Vec<LeaveRequestExtended>> {
        let rows = sqlx::query_as::<_, LeaveRequestRow>(
            "SELECT id, employee_id, leave_type_id, start_date, end_date, days, reason, status, approved_by, approved_at, rejection_reason, created_at
             FROM leave_requests WHERE status = 'Pending' ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct LeaveTypeRow {
    id: String,
    name: String,
    code: String,
    days_per_year: i64,
    carry_over: i64,
    status: String,
}

impl From<LeaveTypeRow> for LeaveTypeDef {
    fn from(r: LeaveTypeRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            code: r.code,
            days_per_year: r.days_per_year,
            carry_over: r.carry_over != 0,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
        }
    }
}

#[derive(sqlx::FromRow)]
struct LeaveBalanceRow {
    id: String,
    employee_id: String,
    leave_type_id: String,
    year: i64,
    entitled: i64,
    used: i64,
    remaining: i64,
    carried_over: i64,
}

impl From<LeaveBalanceRow> for LeaveBalance {
    fn from(r: LeaveBalanceRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            employee_id: Uuid::parse_str(&r.employee_id).unwrap_or_default(),
            leave_type_id: Uuid::parse_str(&r.leave_type_id).unwrap_or_default(),
            year: r.year as i32,
            entitled: r.entitled,
            used: r.used,
            remaining: r.remaining,
            carried_over: r.carried_over,
        }
    }
}

#[derive(sqlx::FromRow)]
struct LeaveRequestRow {
    id: String,
    employee_id: String,
    leave_type_id: String,
    start_date: String,
    end_date: String,
    days: i64,
    reason: Option<String>,
    status: String,
    approved_by: Option<String>,
    approved_at: Option<String>,
    rejection_reason: Option<String>,
    created_at: String,
}

impl From<LeaveRequestRow> for LeaveRequestExtended {
    fn from(r: LeaveRequestRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            employee_id: Uuid::parse_str(&r.employee_id).unwrap_or_default(),
            leave_type_id: Uuid::parse_str(&r.leave_type_id).unwrap_or_default(),
            start_date: chrono::NaiveDate::parse_from_str(&r.start_date, "%Y-%m-%d").unwrap_or_default(),
            end_date: chrono::NaiveDate::parse_from_str(&r.end_date, "%Y-%m-%d").unwrap_or_default(),
            days: r.days,
            reason: r.reason,
            status: match r.status.as_str() {
                "Approved" => LeaveRequestStatus::Approved,
                "Rejected" => LeaveRequestStatus::Rejected,
                "Cancelled" => LeaveRequestStatus::Cancelled,
                _ => LeaveRequestStatus::Pending,
            },
            approved_by: r.approved_by.and_then(|id| Uuid::parse_str(&id).ok()),
            approved_at: r.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            rejection_reason: r.rejection_reason,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct ExpenseService;

impl Default for ExpenseService {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpenseService {
    pub fn new() -> Self { Self }

    pub async fn create_expense_report(
        pool: &SqlitePool,
        employee_id: Uuid,
        title: &str,
        description: Option<&str>,
        lines: Vec<(Uuid, &str, &str, i64)>,
    ) -> Result<ExpenseReport> {
        let now = chrono::Utc::now();
        let report_number = format!("EXP-{}-{:04x}", now.format("%Y%m%d%H%M%S"), rand_digits());
        let id = Uuid::new_v4();
        
        let total: i64 = lines.iter().map(|(_, _, _, amt)| *amt).sum();
        
        sqlx::query(
            "INSERT INTO expense_reports (id, report_number, employee_id, title, description, total_amount, status, submitted_at, approved_by, approved_at, rejected_at, rejection_reason, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, 'Draft', NULL, NULL, NULL, NULL, NULL, ?, ?)"
        )
        .bind(id.to_string())
        .bind(&report_number)
        .bind(employee_id.to_string())
        .bind(title)
        .bind(description)
        .bind(total)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        for (category_id, expense_date, desc, amount) in lines {
            let line_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO expense_lines (id, expense_report_id, category_id, expense_date, description, amount, currency, receipt_path)
                 VALUES (?, ?, ?, ?, ?, ?, 'USD', NULL)"
            )
            .bind(line_id.to_string())
            .bind(id.to_string())
            .bind(category_id.to_string())
            .bind(expense_date)
            .bind(desc)
            .bind(amount)
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        }
        
        Self::get_expense_report(pool, id).await
    }

    pub async fn get_expense_report(pool: &SqlitePool, id: Uuid) -> Result<ExpenseReport> {
        let row = sqlx::query_as::<_, ExpenseReportRow>(
            "SELECT id, report_number, employee_id, title, description, total_amount, status, submitted_at, approved_by, approved_at, rejected_at, rejection_reason, created_at, updated_at
             FROM expense_reports WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("ExpenseReport", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn submit_expense_report(pool: &SqlitePool, id: Uuid) -> Result<ExpenseReport> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE expense_reports SET status = 'Submitted', submitted_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_expense_report(pool, id).await
    }

    pub async fn approve_expense_report(pool: &SqlitePool, id: Uuid, approver_id: Uuid) -> Result<ExpenseReport> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE expense_reports SET status = 'Approved', approved_by = ?, approved_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(approver_id.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_expense_report(pool, id).await
    }

    pub async fn reject_expense_report(pool: &SqlitePool, id: Uuid, reason: &str) -> Result<ExpenseReport> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE expense_reports SET status = 'Rejected', rejected_at = ?, rejection_reason = ?, updated_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(reason)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_expense_report(pool, id).await
    }

    pub async fn list_expense_reports(pool: &SqlitePool, employee_id: Option<Uuid>) -> Result<Vec<ExpenseReport>> {
        let rows = if let Some(eid) = employee_id {
            sqlx::query_as::<_, ExpenseReportRow>(
                "SELECT id, report_number, employee_id, title, description, total_amount, status, submitted_at, approved_by, approved_at, rejected_at, rejection_reason, created_at, updated_at
                 FROM expense_reports WHERE employee_id = ? ORDER BY created_at DESC"
            )
            .bind(eid.to_string())
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?
        } else {
            sqlx::query_as::<_, ExpenseReportRow>(
                "SELECT id, report_number, employee_id, title, description, total_amount, status, submitted_at, approved_by, approved_at, rejected_at, rejection_reason, created_at, updated_at
                 FROM expense_reports ORDER BY created_at DESC"
            )
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?
        };
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn create_expense_category(pool: &SqlitePool, name: &str, code: &str, description: Option<&str>) -> Result<ExpenseCategory> {
        let cat = ExpenseCategory {
            id: Uuid::new_v4(),
            name: name.to_string(),
            code: code.to_string(),
            description: description.map(|s| s.to_string()),
            status: Status::Active,
        };
        
        sqlx::query(
            "INSERT INTO expense_categories (id, name, code, description, status)
             VALUES (?, ?, ?, ?, 'Active')"
        )
        .bind(cat.id.to_string())
        .bind(&cat.name)
        .bind(&cat.code)
        .bind(&cat.description)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(cat)
    }

    pub async fn list_expense_categories(pool: &SqlitePool) -> Result<Vec<ExpenseCategory>> {
        let rows = sqlx::query_as::<_, ExpenseCategoryRow>(
            "SELECT id, name, code, description, status FROM expense_categories WHERE status = 'Active' ORDER BY name"
        )
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct ExpenseReportRow {
    id: String,
    report_number: String,
    employee_id: String,
    title: String,
    description: Option<String>,
    total_amount: i64,
    status: String,
    submitted_at: Option<String>,
    approved_by: Option<String>,
    approved_at: Option<String>,
    rejected_at: Option<String>,
    rejection_reason: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ExpenseReportRow> for ExpenseReport {
    fn from(r: ExpenseReportRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            report_number: r.report_number,
            employee_id: Uuid::parse_str(&r.employee_id).unwrap_or_default(),
            title: r.title,
            description: r.description,
            total_amount: r.total_amount,
            status: match r.status.as_str() {
                "Submitted" => ExpenseReportStatus::Submitted,
                "Approved" => ExpenseReportStatus::Approved,
                "Rejected" => ExpenseReportStatus::Rejected,
                "Paid" => ExpenseReportStatus::Paid,
                _ => ExpenseReportStatus::Draft,
            },
            submitted_at: r.submitted_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            approved_by: r.approved_by.and_then(|id| Uuid::parse_str(&id).ok()),
            approved_at: r.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            rejected_at: r.rejected_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            rejection_reason: r.rejection_reason,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ExpenseCategoryRow {
    id: String,
    name: String,
    code: String,
    description: Option<String>,
    status: String,
}

impl From<ExpenseCategoryRow> for ExpenseCategory {
    fn from(r: ExpenseCategoryRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            code: r.code,
            description: r.description,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
        }
    }
}

pub struct FullPayrollService;

impl FullPayrollService {
    pub async fn create_pay_grade(pool: &SqlitePool, code: &str, name: &str, min_salary: i64, max_salary: i64) -> Result<PayGrade> {
        let pg = PayGrade {
            id: Uuid::new_v4(),
            grade_code: code.to_string(),
            name: name.to_string(),
            description: None,
            min_salary,
            max_salary,
            midpoint: Some((min_salary + max_salary) / 2),
            currency: "USD".to_string(),
            status: Status::Active,
        };
        sqlx::query("INSERT INTO pay_grades (id, grade_code, name, description, min_salary, max_salary, midpoint, currency, status) VALUES (?, ?, ?, NULL, ?, ?, ?, 'USD', 'Active')")
            .bind(pg.id.to_string()).bind(&pg.grade_code).bind(&pg.name)
            .bind(pg.min_salary).bind(pg.max_salary).bind(pg.midpoint)
            .execute(pool).await.map_err(Error::Database)?;
        Ok(pg)
    }

    pub async fn create_pay_component(pool: &SqlitePool, code: &str, name: &str, comp_type: ComponentType, calc_type: CalculationType) -> Result<PayComponent> {
        let pc = PayComponent {
            id: Uuid::new_v4(),
            component_code: code.to_string(),
            name: name.to_string(),
            component_type: comp_type.clone(),
            calculation_type: calc_type,
            default_value: None,
            taxable: true,
            affects_gross: true,
            status: Status::Active,
        };
        sqlx::query("INSERT INTO pay_components (id, component_code, name, component_type, calculation_type, default_value, taxable, affects_gross, status) VALUES (?, ?, ?, ?, ?, NULL, 1, 1, 'Active')")
            .bind(pc.id.to_string()).bind(&pc.component_code).bind(&pc.name)
            .bind(format!("{:?}", pc.component_type)).bind(format!("{:?}", pc.calculation_type))
            .execute(pool).await.map_err(Error::Database)?;
        Ok(pc)
    }

    pub async fn create_payroll_run(pool: &SqlitePool, period_start: &str, period_end: &str, pay_date: &str) -> Result<PayrollRun> {
        let now = chrono::Utc::now();
        let run = PayrollRun {
            id: Uuid::new_v4(),
            run_number: format!("PR-{}-{:04x}", now.format("%Y%m%d%H%M%S"), rand_digits()),
            pay_period_start: chrono::NaiveDate::parse_from_str(period_start, "%Y-%m-%d").map_err(|e| Error::validation(format!("Invalid period_start date: {}", e)))?,
            pay_period_end: chrono::NaiveDate::parse_from_str(period_end, "%Y-%m-%d").map_err(|e| Error::validation(format!("Invalid period_end date: {}", e)))?,
            pay_date: chrono::NaiveDate::parse_from_str(pay_date, "%Y-%m-%d").map_err(|e| Error::validation(format!("Invalid pay_date: {}", e)))?,
            total_gross: 0,
            total_deductions: 0,
            total_net: 0,
            status: PayrollRunStatus::Draft,
            processed_at: None,
            approved_at: None,
            created_at: now,
        };
        sqlx::query("INSERT INTO payroll_runs (id, run_number, pay_period_start, pay_period_end, pay_date, total_gross, total_deductions, total_net, status, processed_at, approved_at, created_at) VALUES (?, ?, ?, ?, ?, 0, 0, 0, 'Draft', NULL, NULL, ?)")
            .bind(run.id.to_string()).bind(&run.run_number)
            .bind(run.pay_period_start.to_string()).bind(run.pay_period_end.to_string()).bind(run.pay_date.to_string())
            .bind(run.created_at.to_rfc3339())
            .execute(pool).await.map_err(Error::Database)?;
        Ok(run)
    }

    pub async fn get_payroll_run(pool: &SqlitePool, id: Uuid) -> Result<PayrollRun> {
        let row: PayrollRunRow = sqlx::query_as("SELECT id, run_number, pay_period_start, pay_period_end, pay_date, total_gross, total_deductions, total_net, status, processed_at, approved_at, created_at FROM payroll_runs WHERE id = ?")
            .bind(id.to_string()).fetch_optional(pool).await.map_err(Error::Database)?
            .ok_or_else(|| Error::not_found("PayrollRun", &id.to_string()))?;
        Ok(row.into())
    }

    pub async fn approve_payroll(pool: &SqlitePool, id: Uuid) -> Result<PayrollRun> {
        let now = chrono::Utc::now();
        sqlx::query("UPDATE payroll_runs SET status = 'Approved', approved_at = ? WHERE id = ?")
            .bind(now.to_rfc3339()).bind(id.to_string()).execute(pool).await.map_err(Error::Database)?;
        Self::get_payroll_run(pool, id).await
    }
}

#[derive(sqlx::FromRow)]
struct PayrollRunRow {
    id: String, run_number: String, pay_period_start: String, pay_period_end: String, pay_date: String,
    total_gross: i64, total_deductions: i64, total_net: i64, status: String,
    processed_at: Option<String>, approved_at: Option<String>, created_at: String,
}

impl From<PayrollRunRow> for PayrollRun {
    fn from(r: PayrollRunRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            run_number: r.run_number,
            pay_period_start: chrono::NaiveDate::parse_from_str(&r.pay_period_start, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            pay_period_end: chrono::NaiveDate::parse_from_str(&r.pay_period_end, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            pay_date: chrono::NaiveDate::parse_from_str(&r.pay_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            total_gross: r.total_gross, total_deductions: r.total_deductions, total_net: r.total_net,
            status: match r.status.as_str() { "Approved" => PayrollRunStatus::Approved, "Processed" => PayrollRunStatus::Processed, _ => PayrollRunStatus::Draft },
            processed_at: r.processed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&chrono::Utc)),
            approved_at: r.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&chrono::Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct JobPostingRow {
    id: String,
    job_code: String,
    title: String,
    department_id: Option<String>,
    location: Option<String>,
    employment_type: String,
    min_salary: Option<i64>,
    max_salary: Option<i64>,
    description: String,
    requirements: Option<String>,
    posted_date: Option<String>,
    closing_date: Option<String>,
    openings: i64,
    filled: i64,
    status: String,
    hiring_manager: Option<String>,
    created_at: String,
}

impl From<JobPostingRow> for JobPosting {
    fn from(r: JobPostingRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            job_code: r.job_code,
            title: r.title,
            department_id: r.department_id.and_then(|id| Uuid::parse_str(&id).ok()),
            location: r.location,
            employment_type: match r.employment_type.as_str() {
                "PartTime" => EmploymentType::PartTime,
                "Contract" => EmploymentType::Contract,
                "Temporary" => EmploymentType::Temporary,
                _ => EmploymentType::FullTime,
            },
            min_salary: r.min_salary,
            max_salary: r.max_salary,
            description: r.description,
            requirements: r.requirements,
            posted_date: r.posted_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            closing_date: r.closing_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            openings: r.openings as i32,
            filled: r.filled as i32,
            status: match r.status.as_str() {
                "Published" => JobPostingStatus::Published,
                "Closed" => JobPostingStatus::Closed,
                _ => JobPostingStatus::Draft,
            },
            hiring_manager: r.hiring_manager,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct EmployeeCostRateService {
    repo: SqliteEmployeeCostRateRepository,
}

impl Default for EmployeeCostRateService {
    fn default() -> Self {
        Self::new()
    }
}

impl EmployeeCostRateService {
    pub fn new() -> Self {
        Self { repo: SqliteEmployeeCostRateRepository }
    }

    pub async fn add_cost_rate(
        &self,
        pool: &SqlitePool,
        employee_id: Uuid,
        base_rate: i64,
        burden_percent: f64,
        burden_amount: i64,
        currency: String,
        effective_date: NaiveDate,
    ) -> Result<EmployeeCostRate> {
        let total_cost_rate = (base_rate as f64 * (1.0 + burden_percent)) as i64 + burden_amount;
        
        let rate = EmployeeCostRate {
            id: Uuid::new_v4(),
            employee_id,
            effective_date,
            base_rate,
            burden_percent,
            burden_amount,
            total_cost_rate,
            currency,
            status: Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.repo.create(pool, rate).await
    }

    pub async fn get_current_rate(
        &self,
        pool: &SqlitePool,
        employee_id: Uuid,
        date: NaiveDate,
    ) -> Result<Option<EmployeeCostRate>> {
        self.repo.find_current(pool, employee_id, date).await
    }
}

pub struct SuccessionService {
    repo: SqliteSuccessionRepository,
}

impl Default for SuccessionService {
    fn default() -> Self {
        Self::new()
    }
}

impl SuccessionService {
    pub fn new() -> Self {
        Self {
            repo: SqliteSuccessionRepository,
        }
    }

    pub async fn create_plan(
        &self,
        pool: &SqlitePool,
        position_id: Uuid,
        incumbent_id: Option<Uuid>,
        criticality: ReadinessLevel,
    ) -> Result<SuccessionPlan> {
        let plan = SuccessionPlan {
            id: Uuid::new_v4(),
            position_id,
            incumbent_id,
            status: SuccessionPlanStatus::Draft,
            criticality,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_plan(pool, plan).await
    }

    pub async fn add_successor(
        &self,
        pool: &SqlitePool,
        plan_id: Uuid,
        employee_id: Uuid,
        readiness: ReadinessLevel,
        ranking: i32,
    ) -> Result<Successor> {
        let successor = Successor {
            id: Uuid::new_v4(),
            plan_id,
            employee_id,
            readiness,
            development_needs: None,
            ranking,
        };
        self.repo.add_successor(pool, successor).await
    }

    pub async fn list_successors(&self, pool: &SqlitePool, plan_id: Uuid) -> Result<Vec<Successor>> {
        self.repo.find_successors(pool, plan_id).await
    }
}

pub struct ChecklistService {
    repo: SqliteEmployeeRepository,
}

impl Default for ChecklistService {
    fn default() -> Self {
        Self::new()
    }
}

impl ChecklistService {
    pub fn new() -> Self {
        Self {
            repo: SqliteEmployeeRepository,
        }
    }

    pub async fn create_template(
        &self,
        pool: &SqlitePool,
        req: CreateChecklistTemplateRequest,
    ) -> Result<ChecklistTemplate> {
        let now = Utc::now();
        let template = ChecklistTemplate {
            id: Uuid::new_v4(),
            name: req.name,
            description: req.description,
            checklist_type: req.checklist_type,
            department_id: req.department_id,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        let created_template = self.repo.create_checklist_template(pool, template).await?;

        for task_req in req.tasks {
            let task = ChecklistTemplateTask {
                id: Uuid::new_v4(),
                template_id: created_template.id,
                task_name: task_req.task_name,
                description: task_req.description,
                assignee_role: task_req.assignee_role,
                relative_due_days: task_req.relative_due_days,
                is_required: task_req.is_required,
                sort_order: task_req.sort_order,
            };
            self.repo.create_checklist_task_template(pool, task).await?;
        }

        Ok(created_template)
    }

    pub async fn assign_checklist(
        &self,
        pool: &SqlitePool,
        employee_id: Uuid,
        template_id: Uuid,
    ) -> Result<EmployeeChecklist> {
        let template = self.repo.get_checklist_template(pool, template_id).await?;
        let tasks = self.repo.list_checklist_tasks_by_template(pool, template_id).await?;
        
        let now = Utc::now();
        let checklist = EmployeeChecklist {
            id: Uuid::new_v4(),
            employee_id,
            template_id,
            checklist_type: template.checklist_type,
            status: ChecklistStatus::InProgress,
            started_at: now,
            completed_at: None,
        };

        let created_checklist = self.repo.create_employee_checklist(pool, checklist).await?;

        for task_template in tasks {
            let task = EmployeeChecklistTask {
                id: Uuid::new_v4(),
                employee_checklist_id: created_checklist.id,
                task_name: task_template.task_name,
                description: task_template.description,
                assigned_to: None, // Can be set later
                due_date: now + chrono::Duration::days(task_template.relative_due_days as i64),
                completed_at: None,
                completed_by: None,
                status: ChecklistTaskStatus::Pending,
                notes: None,
            };
            self.repo.create_employee_checklist_task(pool, task).await?;
        }

        Ok(created_checklist)
    }

    pub async fn complete_task(
        &self,
        pool: &SqlitePool,
        task_id: Uuid,
        completed_by: Uuid,
        notes: Option<String>,
    ) -> Result<EmployeeChecklistTask> {
        let tasks = self.repo.list_employee_checklist_tasks(pool, Uuid::nil()).await?; // Stub workaround
        let mut task = tasks.into_iter().find(|t| t.id == task_id)
            .ok_or_else(|| Error::not_found("EmployeeChecklistTask", &task_id.to_string()))?;
        
        task.status = ChecklistTaskStatus::Completed;
        task.completed_at = Some(Utc::now());
        task.completed_by = Some(completed_by);
        task.notes = notes;
        
        self.repo.update_employee_checklist_task(pool, task).await
    }
}
