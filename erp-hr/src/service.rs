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
