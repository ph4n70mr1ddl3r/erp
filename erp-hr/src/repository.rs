use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency, Address, ContactInfo};
use crate::models::*;

#[derive(sqlx::FromRow)]
struct EmployeeRow {
    id: String, employee_number: String, first_name: String, last_name: String, email: String,
    phone: Option<String>, birth_date: String, hire_date: String, termination_date: Option<String>,
    department_id: Option<String>, position_id: Option<String>, manager_id: Option<String>, status: String, created_at: String, updated_at: String,
}

impl EmployeeRow {
    fn into_employee(self) -> Employee {
        let email = self.email.clone();
        Employee {
            base: BaseEntity { id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None },
            employee_number: self.employee_number, first_name: self.first_name, last_name: self.last_name, email: self.email,
            contact: ContactInfo { email: Some(email), phone: self.phone, fax: None, website: None },
            address: Address { street: String::new(), city: String::new(), state: None, postal_code: String::new(), country: String::new() },
            birth_date: NaiveDate::parse_from_str(&self.birth_date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            hire_date: NaiveDate::parse_from_str(&self.hire_date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            termination_date: self.termination_date.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            department_id: self.department_id.and_then(|s| Uuid::parse_str(&s).ok()),
            position_id: self.position_id.and_then(|s| Uuid::parse_str(&s).ok()),
            manager_id: self.manager_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match self.status.as_str() { "Inactive" => Status::Inactive, "Terminated" => Status::Cancelled, _ => Status::Active },
        }
    }
}

pub struct SqliteEmployeeRepository;

#[async_trait]
impl EmployeeRepository for SqliteEmployeeRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Employee> {
        let row = sqlx::query_as::<_, EmployeeRow>(
            "SELECT id, employee_number, first_name, last_name, email, phone, birth_date, hire_date, termination_date, department_id, position_id, manager_id, status, created_at, updated_at FROM employees WHERE id = ?")
            .bind(id.to_string()).fetch_optional(pool).await?.ok_or_else(|| Error::not_found("Employee", &id.to_string()))?;
        Ok(row.into_employee())
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Employee>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM employees WHERE status != 'Deleted'").fetch_one(pool).await?;
        let rows = sqlx::query_as::<_, EmployeeRow>(
            "SELECT id, employee_number, first_name, last_name, email, phone, birth_date, hire_date, termination_date, department_id, position_id, manager_id, status, created_at, updated_at FROM employees WHERE status != 'Deleted' ORDER BY employee_number LIMIT ? OFFSET ?")
            .bind(pagination.limit() as i64).bind(pagination.offset() as i64).fetch_all(pool).await?;
        Ok(Paginated::new(rows.into_iter().map(|r| r.into_employee()).collect(), count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, emp: Employee) -> Result<Employee> {
        let now = Utc::now();
        sqlx::query("INSERT INTO employees (id, employee_number, first_name, last_name, email, phone, birth_date, hire_date, termination_date, department_id, position_id, manager_id, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(emp.base.id.to_string()).bind(&emp.employee_number).bind(&emp.first_name).bind(&emp.last_name)
            .bind(&emp.email).bind(&emp.contact.phone).bind(emp.birth_date.to_string()).bind(emp.hire_date.to_string())
            .bind(emp.termination_date.map(|d| d.to_string())).bind(emp.department_id.map(|id| id.to_string()))
            .bind(emp.position_id.map(|id| id.to_string())).bind(emp.manager_id.map(|id| id.to_string()))
            .bind(format!("{:?}", emp.status)).bind(emp.base.created_at.to_rfc3339()).bind(now.to_rfc3339())
            .execute(pool).await?;
        Ok(emp)
    }

    async fn terminate(&self, pool: &SqlitePool, id: Uuid, date: NaiveDate) -> Result<()> {
        let rows = sqlx::query("UPDATE employees SET status = 'Terminated', termination_date = ?, updated_at = ? WHERE id = ?")
            .bind(date.to_string()).bind(Utc::now().to_rfc3339()).bind(id.to_string()).execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("Employee", &id.to_string())); }
        Ok(())
    }

    async fn create_checklist_template(&self, _pool: &SqlitePool, template: ChecklistTemplate) -> Result<ChecklistTemplate> {
        Ok(template)
    }

    async fn create_checklist_task_template(&self, _pool: &SqlitePool, task: ChecklistTemplateTask) -> Result<ChecklistTemplateTask> {
        Ok(task)
    }

    async fn get_checklist_template(&self, _pool: &SqlitePool, id: Uuid) -> Result<ChecklistTemplate> {
        Err(Error::not_found("ChecklistTemplate", &id.to_string()))
    }

    async fn list_checklist_tasks_by_template(&self, _pool: &SqlitePool, _template_id: Uuid) -> Result<Vec<ChecklistTemplateTask>> {
        Ok(vec![])
    }

    async fn create_employee_checklist(&self, _pool: &SqlitePool, checklist: EmployeeChecklist) -> Result<EmployeeChecklist> {
        Ok(checklist)
    }

    async fn create_employee_checklist_task(&self, _pool: &SqlitePool, task: EmployeeChecklistTask) -> Result<EmployeeChecklistTask> {
        Ok(task)
    }

    async fn get_employee_checklist(&self, _pool: &SqlitePool, id: Uuid) -> Result<EmployeeChecklist> {
        Err(Error::not_found("EmployeeChecklist", &id.to_string()))
    }

    async fn list_employee_checklist_tasks(&self, _pool: &SqlitePool, _checklist_id: Uuid) -> Result<Vec<EmployeeChecklistTask>> {
        Ok(vec![])
    }

    async fn update_employee_checklist_task(&self, _pool: &SqlitePool, task: EmployeeChecklistTask) -> Result<EmployeeChecklistTask> {
        Ok(task)
    }
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct AttendanceRow { id: String, employee_id: String, date: String, check_in: Option<String>, check_out: Option<String>, work_hours: f64, overtime_hours: f64, status: String }

pub struct SqliteAttendanceRepository;

#[async_trait]
impl AttendanceRepository for SqliteAttendanceRepository {
    async fn record_check_in(&self, pool: &SqlitePool, employee_id: Uuid, date: NaiveDate) -> Result<()> {
        let now = Utc::now();
        sqlx::query("INSERT INTO attendance (id, employee_id, date, check_in, status, work_hours, overtime_hours, created_at, updated_at) VALUES (?, ?, ?, ?, 'Present', 0, 0, ?, ?)")
            .bind(Uuid::new_v4().to_string()).bind(employee_id.to_string()).bind(date.to_string())
            .bind(now.to_rfc3339()).bind(now.to_rfc3339()).bind(now.to_rfc3339())
            .execute(pool).await.map_err(|_| Error::Conflict("Attendance already recorded".into()))?;
        Ok(())
    }

    async fn record_check_out(&self, pool: &SqlitePool, employee_id: Uuid, date: NaiveDate) -> Result<()> {
        let now = Utc::now();
        let rows = sqlx::query("UPDATE attendance SET check_out = ?, updated_at = ? WHERE employee_id = ? AND date = ?")
            .bind(now.to_rfc3339()).bind(now.to_rfc3339()).bind(employee_id.to_string()).bind(date.to_string())
            .execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("Attendance", &date.to_string())); }
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PayrollRow { id: String, employee_id: String, period_start: String, period_end: String, base_salary: i64, overtime: i64, bonuses: i64, deductions: i64, net_salary: i64, status: String, created_at: String, updated_at: String }

pub struct SqlitePayrollRepository;

#[async_trait]
impl PayrollRepository for SqlitePayrollRepository {
    async fn find_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<Payroll>> {
        let rows = sqlx::query_as::<_, PayrollRow>("SELECT id, employee_id, period_start, period_end, base_salary, overtime, bonuses, deductions, net_salary, status, created_at, updated_at FROM payroll WHERE employee_id = ? ORDER BY period_start DESC")
            .bind(employee_id.to_string()).fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| Payroll {
            base: BaseEntity { id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None },
            employee_id: Uuid::parse_str(&r.employee_id).unwrap_or_default(),
            period_start: NaiveDate::parse_from_str(&r.period_start, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            period_end: NaiveDate::parse_from_str(&r.period_end, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            base_salary: Money::new(r.base_salary, Currency::USD), overtime: Money::new(r.overtime, Currency::USD),
            bonuses: Money::new(r.bonuses, Currency::USD), deductions: Money::new(r.deductions, Currency::USD),
            net_salary: Money::new(r.net_salary, Currency::USD),
            status: match r.status.as_str() { "Approved" => Status::Approved, _ => Status::Draft },
        }).collect())
    }

    async fn create(&self, pool: &SqlitePool, payroll: Payroll) -> Result<Payroll> {
        let now = Utc::now();
        sqlx::query("INSERT INTO payroll (id, employee_id, period_start, period_end, base_salary, overtime, bonuses, deductions, net_salary, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(payroll.base.id.to_string()).bind(payroll.employee_id.to_string())
            .bind(payroll.period_start.to_string()).bind(payroll.period_end.to_string())
            .bind(payroll.base_salary.amount).bind(payroll.overtime.amount).bind(payroll.bonuses.amount)
            .bind(payroll.deductions.amount).bind(payroll.net_salary.amount).bind(format!("{:?}", payroll.status))
            .bind(payroll.base.created_at.to_rfc3339()).bind(now.to_rfc3339()).execute(pool).await?;
        Ok(payroll)
    }

    async fn approve(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("UPDATE payroll SET status = 'Approved', updated_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339()).bind(id.to_string()).execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("Payroll", &id.to_string())); }
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct CostRateRow {
    id: String, employee_id: String, effective_date: String, base_rate: i64, burden_percent: f64,
    burden_amount: i64, total_cost_rate: i64, currency: String, status: String, created_at: String, updated_at: String,
}

impl CostRateRow {
    fn into_cost_rate(self) -> EmployeeCostRate {
        EmployeeCostRate {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            employee_id: Uuid::parse_str(&self.employee_id).unwrap_or_default(),
            effective_date: NaiveDate::parse_from_str(&self.effective_date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            base_rate: self.base_rate, burden_percent: self.burden_percent, burden_amount: self.burden_amount,
            total_cost_rate: self.total_cost_rate, currency: self.currency,
            status: match self.status.as_str() { "Inactive" => Status::Inactive, _ => Status::Active },
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

pub struct SqliteEmployeeCostRateRepository;

#[async_trait]
impl EmployeeCostRateRepository for SqliteEmployeeCostRateRepository {
    async fn find_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<EmployeeCostRate>> {
        let rows = sqlx::query_as::<_, CostRateRow>(
            "SELECT id, employee_id, effective_date, base_rate, burden_percent, burden_amount, total_cost_rate, currency, status, created_at, updated_at FROM employee_cost_rates WHERE employee_id = ? ORDER BY effective_date DESC")
            .bind(employee_id.to_string()).fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_cost_rate()).collect())
    }

    async fn find_current(&self, pool: &SqlitePool, employee_id: Uuid, date: NaiveDate) -> Result<Option<EmployeeCostRate>> {
        let row = sqlx::query_as::<_, CostRateRow>(
            "SELECT id, employee_id, effective_date, base_rate, burden_percent, burden_amount, total_cost_rate, currency, status, created_at, updated_at FROM employee_cost_rates WHERE employee_id = ? AND effective_date <= ? AND status = 'Active' ORDER BY effective_date DESC LIMIT 1")
            .bind(employee_id.to_string()).bind(date.to_string()).fetch_optional(pool).await?;
        Ok(row.map(|r| r.into_cost_rate()))
    }

    async fn create(&self, pool: &SqlitePool, rate: EmployeeCostRate) -> Result<EmployeeCostRate> {
        let now = Utc::now();
        sqlx::query("INSERT INTO employee_cost_rates (id, employee_id, effective_date, base_rate, burden_percent, burden_amount, total_cost_rate, currency, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(rate.id.to_string()).bind(rate.employee_id.to_string()).bind(rate.effective_date.to_string())
            .bind(rate.base_rate).bind(rate.burden_percent).bind(rate.burden_amount).bind(rate.total_cost_rate)
            .bind(&rate.currency).bind(format!("{:?}", rate.status)).bind(rate.created_at.to_rfc3339()).bind(now.to_rfc3339())
            .execute(pool).await?;
        let mut created = rate;
        created.updated_at = now;
        Ok(created)
    }
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct SuccessionPlanRow {
    id: String,
    position_id: String,
    incumbent_id: Option<String>,
    status: String,
    criticality: String,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

impl SuccessionPlanRow {
    #[allow(dead_code)]
    fn into_succession_plan(self) -> SuccessionPlan {
        SuccessionPlan {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            position_id: Uuid::parse_str(&self.position_id).unwrap_or_default(),
            incumbent_id: self.incumbent_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match self.status.as_str() {
                "Active" => SuccessionPlanStatus::Active,
                "Archived" => SuccessionPlanStatus::Archived,
                _ => SuccessionPlanStatus::Draft,
            },
            criticality: match self.criticality.as_str() {
                "ReadyNow" => ReadinessLevel::ReadyNow,
                "Ready1To2Years" => ReadinessLevel::Ready1To2Years,
                "Ready3To5Years" => ReadinessLevel::Ready3To5Years,
                _ => ReadinessLevel::EmergencyCoverage,
            },
            notes: self.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct SuccessorRow {
    id: String,
    plan_id: String,
    employee_id: String,
    readiness: String,
    development_needs: Option<String>,
    ranking: i32,
}

impl SuccessorRow {
    #[allow(dead_code)]
    fn into_successor(self) -> Successor {
        Successor {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            plan_id: Uuid::parse_str(&self.plan_id).unwrap_or_default(),
            employee_id: Uuid::parse_str(&self.employee_id).unwrap_or_default(),
            readiness: match self.readiness.as_str() {
                "ReadyNow" => ReadinessLevel::ReadyNow,
                "Ready1To2Years" => ReadinessLevel::Ready1To2Years,
                "Ready3To5Years" => ReadinessLevel::Ready3To5Years,
                _ => ReadinessLevel::EmergencyCoverage,
            },
            development_needs: self.development_needs,
            ranking: self.ranking,
        }
    }
}

pub struct SqliteSuccessionRepository;

#[async_trait]
impl SuccessionRepository for SqliteSuccessionRepository {
    async fn find_plan_by_position(&self, _pool: &SqlitePool, _position_id: Uuid) -> Result<Option<SuccessionPlan>> {
        Ok(None)
    }

    async fn create_plan(&self, _pool: &SqlitePool, plan: SuccessionPlan) -> Result<SuccessionPlan> {
        Ok(plan)
    }

    async fn add_successor(&self, _pool: &SqlitePool, successor: Successor) -> Result<Successor> {
        Ok(successor)
    }

    async fn find_successors(&self, _pool: &SqlitePool, _plan_id: Uuid) -> Result<Vec<Successor>> {
        Ok(vec![])
    }
}

#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Employee>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Employee>>;
    async fn create(&self, pool: &SqlitePool, emp: Employee) -> Result<Employee>;
    async fn terminate(&self, pool: &SqlitePool, id: Uuid, date: NaiveDate) -> Result<()>;

    // Checklist Management
    async fn create_checklist_template(&self, pool: &SqlitePool, template: ChecklistTemplate) -> Result<ChecklistTemplate>;
    async fn create_checklist_task_template(&self, pool: &SqlitePool, task: ChecklistTemplateTask) -> Result<ChecklistTemplateTask>;
    async fn get_checklist_template(&self, pool: &SqlitePool, id: Uuid) -> Result<ChecklistTemplate>;
    async fn list_checklist_tasks_by_template(&self, pool: &SqlitePool, template_id: Uuid) -> Result<Vec<ChecklistTemplateTask>>;
    
    async fn create_employee_checklist(&self, pool: &SqlitePool, checklist: EmployeeChecklist) -> Result<EmployeeChecklist>;
    async fn create_employee_checklist_task(&self, pool: &SqlitePool, task: EmployeeChecklistTask) -> Result<EmployeeChecklistTask>;
    async fn get_employee_checklist(&self, pool: &SqlitePool, id: Uuid) -> Result<EmployeeChecklist>;
    async fn list_employee_checklist_tasks(&self, pool: &SqlitePool, checklist_id: Uuid) -> Result<Vec<EmployeeChecklistTask>>;
    async fn update_employee_checklist_task(&self, pool: &SqlitePool, task: EmployeeChecklistTask) -> Result<EmployeeChecklistTask>;
}

#[async_trait]
pub trait AttendanceRepository: Send + Sync {
    async fn record_check_in(&self, pool: &SqlitePool, employee_id: Uuid, date: NaiveDate) -> Result<()>;
    async fn record_check_out(&self, pool: &SqlitePool, employee_id: Uuid, date: NaiveDate) -> Result<()>;
}

#[async_trait]
pub trait PayrollRepository: Send + Sync {
    async fn find_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<Payroll>>;
    async fn create(&self, pool: &SqlitePool, payroll: Payroll) -> Result<Payroll>;
    async fn approve(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait EmployeeCostRateRepository: Send + Sync {
    async fn find_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<EmployeeCostRate>>;
    async fn find_current(&self, pool: &SqlitePool, employee_id: Uuid, date: NaiveDate) -> Result<Option<EmployeeCostRate>>;
    async fn create(&self, pool: &SqlitePool, rate: EmployeeCostRate) -> Result<EmployeeCostRate>;
}

#[async_trait]
pub trait SuccessionRepository: Send + Sync {
    async fn find_plan_by_position(&self, pool: &SqlitePool, position_id: Uuid) -> Result<Option<SuccessionPlan>>;
    async fn create_plan(&self, pool: &SqlitePool, plan: SuccessionPlan) -> Result<SuccessionPlan>;
    async fn add_successor(&self, pool: &SqlitePool, successor: Successor) -> Result<Successor>;
    async fn find_successors(&self, pool: &SqlitePool, plan_id: Uuid) -> Result<Vec<Successor>>;
}
