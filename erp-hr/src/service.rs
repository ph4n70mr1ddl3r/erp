use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency};
use chrono::NaiveDate;
use crate::models::*;
use crate::repository::*;

pub struct EmployeeService { repo: SqliteEmployeeRepository }
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
impl AttendanceService {
    pub fn new() -> Self { Self { repo: SqliteAttendanceRepository } }
    pub async fn check_in(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<()> {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let date = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap();
        self.repo.record_check_in(pool, employee_id, date).await
    }
    pub async fn check_out(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<()> {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let date = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap();
        self.repo.record_check_out(pool, employee_id, date).await
    }
}

pub struct PayrollService { repo: SqlitePayrollRepository }
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
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(lt)
    }

    pub async fn list_leave_types(pool: &SqlitePool) -> Result<Vec<LeaveTypeDef>> {
        let rows = sqlx::query_as::<_, LeaveTypeRow>(
            "SELECT id, name, code, days_per_year, carry_over, status FROM leave_types WHERE status = 'Active'"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
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
        .map_err(|e| Error::Database(e.into()))?
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
        .map_err(|e| Error::Database(e.into()))?;
        
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
        .map_err(|e| Error::Database(e.into()))?;
        
        Self::get_leave_request(pool, id).await
    }

    pub async fn reject_leave_request(pool: &SqlitePool, id: Uuid, reason: &str) -> Result<LeaveRequestExtended> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE leave_requests SET status = 'Rejected', rejection_reason = ? WHERE id = ?"
        )
        .bind(reason)
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
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
        .map_err(|e| Error::Database(e.into()))?
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
        .map_err(|e| Error::Database(e.into()))?;
        
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
        let report_number = format!("EXP-{}", now.format("%Y%m%d%H%M%S"));
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
        .map_err(|e| Error::Database(e.into()))?;
        
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
            .map_err(|e| Error::Database(e.into()))?;
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
        .map_err(|e| Error::Database(e.into()))?
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
        .map_err(|e| Error::Database(e.into()))?;
        
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
        .map_err(|e| Error::Database(e.into()))?;
        
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
        .map_err(|e| Error::Database(e.into()))?;
        
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
            .map_err(|e| Error::Database(e.into()))?
        } else {
            sqlx::query_as::<_, ExpenseReportRow>(
                "SELECT id, report_number, employee_id, title, description, total_amount, status, submitted_at, approved_by, approved_at, rejected_at, rejection_reason, created_at, updated_at
                 FROM expense_reports ORDER BY created_at DESC"
            )
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?
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
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(cat)
    }

    pub async fn list_expense_categories(pool: &SqlitePool) -> Result<Vec<ExpenseCategory>> {
        let rows = sqlx::query_as::<_, ExpenseCategoryRow>(
            "SELECT id, name, code, description, status FROM expense_categories WHERE status = 'Active' ORDER BY name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
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
