use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use erp_core::{Error, Result, Pagination, Paginated};
use crate::models::*;
use crate::repository::*;

pub struct ProjectService<
    P: ProjectRepository = SqliteProjectRepository,
    T: ProjectTaskRepository = SqliteProjectTaskRepository,
    M: ProjectMilestoneRepository = SqliteProjectMilestoneRepository,
    E: ProjectExpenseRepository = SqliteProjectExpenseRepository,
    TP: ProjectTemplateRepository = SqliteProjectTemplateRepository,
> {
    repo: P,
    task_repo: T,
    milestone_repo: M,
    expense_repo: E,
    template_repo: TP,
}

impl Default for ProjectService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectService {
    pub fn new() -> Self {
        Self {
            repo: SqliteProjectRepository,
            task_repo: SqliteProjectTaskRepository,
            milestone_repo: SqliteProjectMilestoneRepository,
            expense_repo: SqliteProjectExpenseRepository,
            template_repo: SqliteProjectTemplateRepository,
        }
    }
}

impl<P, T, M, E, TP> ProjectService<P, T, M, E, TP>
where
    P: ProjectRepository,
    T: ProjectTaskRepository,
    M: ProjectMilestoneRepository,
    E: ProjectExpenseRepository,
    TP: ProjectTemplateRepository,
{
    pub fn with_repos(repo: P, task_repo: T, milestone_repo: M, expense_repo: E, template_repo: TP) -> Self {
        Self {
            repo,
            task_repo,
            milestone_repo,
            expense_repo,
            template_repo,
        }
    }

    pub async fn get_project(&self, pool: &SqlitePool, id: Uuid) -> Result<Project> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn get_project_by_number(&self, pool: &SqlitePool, number: &str) -> Result<Project> {
        self.repo.find_by_number(pool, number).await
    }

    pub async fn list_projects(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Project>> {
        self.repo.find_all(pool, pagination).await
    }

    pub async fn create_project(&self, pool: &SqlitePool, mut project: Project) -> Result<Project> {
        project.id = Uuid::new_v4();
        project.project_number = format!("PRJ-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        project.created_at = Utc::now();
        self.repo.create(pool, project).await
    }

    pub async fn create_project_from_template(
        &self,
        pool: &SqlitePool,
        template_id: Uuid,
        name: String,
        customer_id: Option<Uuid>,
        start_date: DateTime<Utc>,
    ) -> Result<Project> {
        let template = self.template_repo.find_by_id(pool, template_id).await?;
        
        let project = Project {
            id: Uuid::new_v4(),
            project_number: format!("PRJ-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
            name,
            description: template.description,
            customer_id,
            project_type: template.project_type,
            start_date,
            end_date: None,
            budget: 0,
            billable: template.billable,
            billing_method: template.billing_method,
            project_manager: None,
            status: ProjectStatus::Planning,
            percent_complete: 0,
            created_at: Utc::now(),
        };

        let created_project = self.repo.create(pool, project).await?;

        // Create tasks from template
        for t_task in template.tasks {
            let task = ProjectTask {
                id: Uuid::new_v4(),
                project_id: created_project.id,
                task_number: 0, // Should be incremented
                name: t_task.name,
                description: t_task.description,
                parent_task_id: None,
                assigned_to: None,
                start_date: start_date + chrono::Duration::days(t_task.relative_start_day as i64),
                end_date: Some(start_date + chrono::Duration::days((t_task.relative_start_day + t_task.duration_days) as i64)),
                estimated_hours: t_task.estimated_hours,
                actual_hours: 0.0,
                percent_complete: 0,
                priority: t_task.priority,
                status: TaskStatus::NotStarted,
                billable: t_task.billable,
            };
            self.task_repo.create(pool, task).await?;
        }

        // Create milestones from template
        for t_milestone in template.milestones {
            let milestone = ProjectMilestone {
                id: Uuid::new_v4(),
                project_id: created_project.id,
                name: t_milestone.name,
                description: t_milestone.description,
                planned_date: start_date + chrono::Duration::days(t_milestone.relative_day as i64),
                actual_date: None,
                billing_amount: t_milestone.billing_amount,
                billing_status: BillingStatus::NotBilled,
                status: MilestoneStatus::Planned,
            };
            self.milestone_repo.create(pool, milestone).await?;
        }

        Ok(created_project)
    }

    pub async fn update_project(&self, pool: &SqlitePool, project: Project) -> Result<Project> {
        let _ = self.repo.find_by_id(pool, project.id).await?;
        self.repo.update(pool, project).await
    }

    pub async fn delete_project(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }

    pub async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ProjectStatus) -> Result<Project> {
        let mut project = self.repo.find_by_id(pool, id).await?;
        project.status = status;
        if project.status == ProjectStatus::Completed {
            project.end_date = Some(Utc::now());
        }
        self.repo.update(pool, project).await
    }

    pub async fn calculate_progress(&self, pool: &SqlitePool, id: Uuid) -> Result<i32> {
        let tasks = self.task_repo.find_by_project(pool, id).await?;
        if tasks.is_empty() {
            return Ok(0);
        }
        let total_progress: i32 = tasks.iter().map(|t| t.percent_complete).sum();
        let avg_progress = total_progress / tasks.len() as i32;
        let mut project = self.repo.find_by_id(pool, id).await?;
        project.percent_complete = avg_progress;
        self.repo.update(pool, project).await?;
        Ok(avg_progress)
    }

    pub async fn add_task(&self, pool: &SqlitePool, task: ProjectTask) -> Result<ProjectTask> {
        self.task_repo.create(pool, task).await
    }

    pub async fn update_task(&self, pool: &SqlitePool, task: ProjectTask) -> Result<ProjectTask> {
        self.task_repo.update(pool, task).await
    }

    pub async fn complete_task(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectTask> {
        let mut task = self.task_repo.find_by_id(pool, id).await?;
        task.status = TaskStatus::Completed;
        task.percent_complete = 100;
        let task = self.task_repo.update(pool, task).await?;
        let _ = self.calculate_progress(pool, task.project_id).await?;
        Ok(task)
    }

    pub async fn add_milestone(&self, pool: &SqlitePool, milestone: ProjectMilestone) -> Result<ProjectMilestone> {
        self.milestone_repo.create(pool, milestone).await
    }

    pub async fn complete_milestone(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectMilestone> {
        let mut milestone = self.milestone_repo.find_by_id(pool, id).await?;
        milestone.status = MilestoneStatus::Completed;
        milestone.actual_date = Some(Utc::now());
        self.milestone_repo.update(pool, milestone).await
    }

    pub async fn add_expense(&self, pool: &SqlitePool, expense: ProjectExpense) -> Result<ProjectExpense> {
        self.expense_repo.create(pool, expense).await
    }
}

#[allow(dead_code)]
pub struct ProjectTaskService {
    repo: SqliteProjectTaskRepository,
}

impl Default for ProjectTaskService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectTaskService {
    pub fn new() -> Self {
        Self { repo: SqliteProjectTaskRepository }
    }

    pub async fn get_task(&self, _pool: &SqlitePool, id: Uuid) -> Result<ProjectTask> {
        Err(Error::not_found("ProjectTask", &id.to_string()))
    }

    pub async fn list_tasks_by_project(&self, _pool: &SqlitePool, _project_id: Uuid) -> Result<Vec<ProjectTask>> {
        Ok(vec![])
    }

    pub async fn create_task(&self, _pool: &SqlitePool, _task: ProjectTask) -> Result<ProjectTask> {
        Err(Error::validation("Not implemented"))
    }

    pub async fn update_task(&self, _pool: &SqlitePool, task: ProjectTask) -> Result<ProjectTask> {
        Err(Error::not_found("ProjectTask", &task.id.to_string()))
    }

    pub async fn delete_task(&self, _pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectTask", &id.to_string()))
    }
}

#[allow(dead_code)]
pub struct ProjectMilestoneService {
    repo: SqliteProjectMilestoneRepository,
}

impl Default for ProjectMilestoneService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectMilestoneService {
    pub fn new() -> Self {
        Self { repo: SqliteProjectMilestoneRepository }
    }

    pub async fn get_milestone(&self, _pool: &SqlitePool, id: Uuid) -> Result<ProjectMilestone> {
        Err(Error::not_found("ProjectMilestone", &id.to_string()))
    }

    pub async fn list_milestones_by_project(&self, _pool: &SqlitePool, _project_id: Uuid) -> Result<Vec<ProjectMilestone>> {
        Ok(vec![])
    }

    pub async fn create_milestone(&self, _pool: &SqlitePool, _milestone: ProjectMilestone) -> Result<ProjectMilestone> {
        Err(Error::validation("Not implemented"))
    }

    pub async fn update_milestone(&self, _pool: &SqlitePool, milestone: ProjectMilestone) -> Result<ProjectMilestone> {
        Err(Error::not_found("ProjectMilestone", &milestone.id.to_string()))
    }

    pub async fn delete_milestone(&self, _pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectMilestone", &id.to_string()))
    }
}

#[allow(dead_code)]
pub struct ProjectExpenseService {
    repo: SqliteProjectExpenseRepository,
}

impl Default for ProjectExpenseService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectExpenseService {
    pub fn new() -> Self {
        Self { repo: SqliteProjectExpenseRepository }
    }

    pub async fn get_expense(&self, _pool: &SqlitePool, id: Uuid) -> Result<ProjectExpense> {
        Err(Error::not_found("ProjectExpense", &id.to_string()))
    }

    pub async fn list_expenses_by_project(&self, _pool: &SqlitePool, _project_id: Uuid) -> Result<Vec<ProjectExpense>> {
        Ok(vec![])
    }

    pub async fn create_expense(&self, _pool: &SqlitePool, _expense: ProjectExpense) -> Result<ProjectExpense> {
        Err(Error::validation("Not implemented"))
    }

    pub async fn update_expense(&self, _pool: &SqlitePool, expense: ProjectExpense) -> Result<ProjectExpense> {
        Err(Error::not_found("ProjectExpense", &expense.id.to_string()))
    }

    pub async fn delete_expense(&self, _pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectExpense", &id.to_string()))
    }
}

pub struct TimesheetService {
    repo: SqliteTimesheetRepository,
    entry_repo: SqliteTimesheetEntryRepository,
}

impl Default for TimesheetService {
    fn default() -> Self {
        Self::new()
    }
}

impl TimesheetService {
    pub fn new() -> Self {
        Self {
            repo: SqliteTimesheetRepository,
            entry_repo: SqliteTimesheetEntryRepository,
        }
    }

    pub async fn create_timesheet(&self, pool: &SqlitePool, mut timesheet: Timesheet) -> Result<Timesheet> {
        timesheet.id = Uuid::new_v4();
        timesheet.timesheet_number = format!("TS-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        timesheet.created_at = Utc::now();
        timesheet.status = TimesheetStatus::Draft;
        self.repo.create(pool, timesheet).await
    }

    pub async fn get_timesheet(&self, pool: &SqlitePool, id: Uuid) -> Result<Timesheet> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn get_timesheet_by_number(&self, pool: &SqlitePool, number: &str) -> Result<Timesheet> {
        self.repo.find_by_number(pool, number).await
    }

    pub async fn list_timesheets(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Timesheet>> {
        self.repo.find_all(pool, pagination).await
    }

    pub async fn get_employee_timesheets(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<Timesheet>> {
        self.repo.find_by_employee(pool, employee_id).await
    }

    pub async fn add_entry(&self, pool: &SqlitePool, mut entry: TimesheetEntry) -> Result<TimesheetEntry> {
        let timesheet = self.repo.find_by_id(pool, entry.timesheet_id).await?;
        if timesheet.status != TimesheetStatus::Draft {
            return Err(Error::validation("Cannot add entries to a non-draft timesheet"));
        }
        entry.id = Uuid::new_v4();
        let entry = self.entry_repo.create(pool, entry).await?;
        self.recalculate_hours(pool, entry.timesheet_id).await?;
        Ok(entry)
    }

    pub async fn remove_entry(&self, pool: &SqlitePool, entry_id: Uuid) -> Result<()> {
        let entry = self.entry_repo.find_by_id(pool, entry_id).await?;
        let timesheet = self.repo.find_by_id(pool, entry.timesheet_id).await?;
        if timesheet.status != TimesheetStatus::Draft {
            return Err(Error::validation("Cannot remove entries from a non-draft timesheet"));
        }
        let timesheet_id = entry.timesheet_id;
        self.entry_repo.delete(pool, entry_id).await?;
        self.recalculate_hours(pool, timesheet_id).await?;
        Ok(())
    }

    pub async fn submit_timesheet(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let mut timesheet = self.repo.find_by_id(pool, id).await?;
        if timesheet.status != TimesheetStatus::Draft {
            return Err(Error::validation("Only draft timesheets can be submitted"));
        }
        timesheet.status = TimesheetStatus::Submitted;
        timesheet.submitted_at = Some(Utc::now());
        self.repo.update(pool, timesheet).await?;
        Ok(())
    }

    pub async fn approve_timesheet(&self, pool: &SqlitePool, id: Uuid, approver_id: Uuid) -> Result<()> {
        let mut timesheet = self.repo.find_by_id(pool, id).await?;
        if timesheet.status != TimesheetStatus::Submitted {
            return Err(Error::validation("Only submitted timesheets can be approved"));
        }
        timesheet.status = TimesheetStatus::Approved;
        timesheet.approved_at = Some(Utc::now());
        timesheet.approved_by = Some(approver_id);
        self.repo.update(pool, timesheet).await?;
        Ok(())
    }

    pub async fn reject_timesheet(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let mut timesheet = self.repo.find_by_id(pool, id).await?;
        if timesheet.status != TimesheetStatus::Submitted {
            return Err(Error::validation("Only submitted timesheets can be rejected"));
        }
        timesheet.status = TimesheetStatus::Rejected;
        self.repo.update(pool, timesheet).await?;
        Ok(())
    }

    async fn recalculate_hours(&self, pool: &SqlitePool, timesheet_id: Uuid) -> Result<()> {
        let entries = self.entry_repo.find_by_timesheet(pool, timesheet_id).await?;
        let total: f64 = entries.iter().map(|e| e.hours).sum();
        let mut timesheet = self.repo.find_by_id(pool, timesheet_id).await?;
        timesheet.total_hours = total;
        self.repo.update(pool, timesheet).await?;
        Ok(())
    }
}

#[allow(dead_code)]
pub struct TimesheetEntryService {
    repo: SqliteTimesheetEntryRepository,
}

impl Default for TimesheetEntryService {
    fn default() -> Self {
        Self::new()
    }
}

impl TimesheetEntryService {
    pub fn new() -> Self {
        Self { repo: SqliteTimesheetEntryRepository }
    }

    pub async fn get_entry(&self, _pool: &SqlitePool, id: Uuid) -> Result<TimesheetEntry> {
        Err(Error::not_found("TimesheetEntry", &id.to_string()))
    }

    pub async fn list_entries_by_timesheet(&self, _pool: &SqlitePool, _timesheet_id: Uuid) -> Result<Vec<TimesheetEntry>> {
        Ok(vec![])
    }

    pub async fn create_entry(&self, _pool: &SqlitePool, _entry: TimesheetEntry) -> Result<TimesheetEntry> {
        Err(Error::validation("Not implemented"))
    }

    pub async fn update_entry(&self, _pool: &SqlitePool, entry: TimesheetEntry) -> Result<TimesheetEntry> {
        Err(Error::not_found("TimesheetEntry", &entry.id.to_string()))
    }

    pub async fn delete_entry(&self, _pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("TimesheetEntry", &id.to_string()))
    }
}

pub struct ProjectBillingService {
    repo: SqliteProjectBillingRepository,
    milestone_repo: SqliteProjectMilestoneRepository,
    timesheet_repo: SqliteTimesheetRepository,
    entry_repo: SqliteTimesheetEntryRepository,
}

impl Default for ProjectBillingService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectBillingService {
    pub fn new() -> Self {
        Self {
            repo: SqliteProjectBillingRepository,
            milestone_repo: SqliteProjectMilestoneRepository,
            timesheet_repo: SqliteTimesheetRepository,
            entry_repo: SqliteTimesheetEntryRepository,
        }
    }

    pub async fn get_billing(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectBilling> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn get_billing_by_number(&self, pool: &SqlitePool, number: &str) -> Result<ProjectBilling> {
        self.repo.find_by_number(pool, number).await
    }

    pub async fn list_billings_by_project(&self, pool: &SqlitePool, project_id: Uuid) -> Result<Vec<ProjectBilling>> {
        self.repo.find_by_project(pool, project_id).await
    }

    pub async fn create_billing(&self, pool: &SqlitePool, mut billing: ProjectBilling) -> Result<ProjectBilling> {
        billing.id = Uuid::new_v4();
        billing.billing_number = format!("BL-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        billing.created_at = Utc::now();
        billing.status = ProjectBillingStatus::Draft;
        self.repo.create(pool, billing).await
    }

    pub async fn generate_from_milestone(&self, pool: &SqlitePool, milestone_id: Uuid) -> Result<ProjectBilling> {
        let milestone = self.milestone_repo.find_by_id(pool, milestone_id).await?;
        if milestone.status != MilestoneStatus::Completed {
            return Err(Error::validation("Can only generate billing from completed milestones"));
        }
        let billing = ProjectBilling {
            id: Uuid::new_v4(),
            billing_number: format!("BL-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
            project_id: milestone.project_id,
            billing_type: ProjectBillingType::Milestone,
            milestone_id: Some(milestone_id),
            period_start: None,
            period_end: None,
            amount: milestone.billing_amount,
            invoice_id: None,
            status: ProjectBillingStatus::Draft,
            created_at: Utc::now(),
        };
        self.repo.create(pool, billing).await
    }

    pub async fn generate_from_timesheet(&self, pool: &SqlitePool, timesheet_id: Uuid) -> Result<ProjectBilling> {
        let timesheet = self.timesheet_repo.find_by_id(pool, timesheet_id).await?;
        if timesheet.status != TimesheetStatus::Approved {
            return Err(Error::validation("Can only generate billing from approved timesheets"));
        }
        let entries = self.entry_repo.find_by_timesheet(pool, timesheet_id).await?;
        let mut total_amount: i64 = 0;
        let mut project_id: Option<Uuid> = None;
        for entry in &entries {
            if entry.billable {
                if let Some(rate) = entry.hourly_rate {
                    total_amount += (entry.hours * rate as f64) as i64;
                }
                if project_id.is_none() {
                    project_id = entry.project_id;
                }
            }
        }
        let project_id = project_id.ok_or_else(|| Error::validation("No billable entries found"))?;
        let billing = ProjectBilling {
            id: Uuid::new_v4(),
            billing_number: format!("BL-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
            project_id,
            billing_type: ProjectBillingType::TimeBased,
            milestone_id: None,
            period_start: Some(timesheet.period_start),
            period_end: Some(timesheet.period_end),
            amount: total_amount,
            invoice_id: None,
            status: ProjectBillingStatus::Draft,
            created_at: Utc::now(),
        };
        self.repo.create(pool, billing).await
    }

    pub async fn mark_as_invoiced(&self, pool: &SqlitePool, id: Uuid, invoice_id: Uuid) -> Result<ProjectBilling> {
        let mut billing = self.repo.find_by_id(pool, id).await?;
        if billing.status == ProjectBillingStatus::Invoiced || billing.status == ProjectBillingStatus::Paid {
            return Err(Error::validation("Billing is already invoiced or paid"));
        }
        billing.status = ProjectBillingStatus::Invoiced;
        billing.invoice_id = Some(invoice_id);
        self.repo.update(pool, billing).await
    }

    pub async fn update_billing(&self, pool: &SqlitePool, billing: ProjectBilling) -> Result<ProjectBilling> {
        let _ = self.repo.find_by_id(pool, billing.id).await?;
        self.repo.update(pool, billing).await
    }

    pub async fn delete_billing(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }
}

pub struct ResourceService<
    S: SkillRepository = SqliteSkillRepository,
    RS: ResourceSkillRepository = SqliteResourceSkillRepository,
    RQ: ResourceRequestRepository = SqliteResourceRequestRepository,
    A: ResourceAllocationRepository = SqliteResourceAllocationRepository,
> {
    skill_repo: S,
    resource_skill_repo: RS,
    request_repo: RQ,
    allocation_repo: A,
}

impl Default for ResourceService {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceService {
    pub fn new() -> Self {
        Self {
            skill_repo: SqliteSkillRepository,
            resource_skill_repo: SqliteResourceSkillRepository,
            request_repo: SqliteResourceRequestRepository,
            allocation_repo: SqliteResourceAllocationRepository,
        }
    }
}

impl<S, RS, RQ, A> ResourceService<S, RS, RQ, A>
where
    S: SkillRepository,
    RS: ResourceSkillRepository,
    RQ: ResourceRequestRepository,
    A: ResourceAllocationRepository,
{
    pub fn with_repos(skill_repo: S, resource_skill_repo: RS, request_repo: RQ, allocation_repo: A) -> Self {
        Self {
            skill_repo,
            resource_skill_repo,
            request_repo,
            allocation_repo,
        }
    }

    pub async fn create_request(&self, pool: &SqlitePool, mut request: ResourceRequest) -> Result<ResourceRequest> {
        request.id = Uuid::new_v4();
        request.created_at = Utc::now();
        request.status = ResourceRequestStatus::Draft;
        self.request_repo.create(pool, request).await
    }

    pub async fn submit_request(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let mut request = self.request_repo.find_by_id(pool, id).await?;
        if request.status != ResourceRequestStatus::Draft {
            return Err(Error::validation("Only draft requests can be submitted"));
        }
        request.status = ResourceRequestStatus::Pending;
        self.request_repo.update(pool, request).await?;
        Ok(())
    }

    pub async fn allocate_resource(&self, pool: &SqlitePool, mut allocation: ResourceAllocation) -> Result<ResourceAllocation> {
        allocation.id = Uuid::new_v4();
        allocation.created_at = Utc::now();
        
        if let Some(request_id) = allocation.request_id {
            let mut request = self.request_repo.find_by_id(pool, request_id).await?;
            request.status = ResourceRequestStatus::Fulfilled;
            self.request_repo.update(pool, request).await?;
        }
        
        self.allocation_repo.create(pool, allocation).await
    }

    pub async fn list_allocations_by_project(&self, pool: &SqlitePool, project_id: Uuid) -> Result<Vec<ResourceAllocation>> {
        self.allocation_repo.find_by_project(pool, project_id).await
    }

    pub async fn list_allocations_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<ResourceAllocation>> {
        self.allocation_repo.find_by_employee(pool, employee_id).await
    }

    pub async fn list_skills(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Skill>> {
        self.skill_repo.find_all(pool, pagination).await
    }

    pub async fn get_employee_skills(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<ResourceSkill>> {
        self.resource_skill_repo.find_by_employee(pool, employee_id).await
    }
}

pub struct ProjectTemplateService {
    repo: SqliteProjectTemplateRepository,
}

impl Default for ProjectTemplateService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectTemplateService {
    pub fn new() -> Self {
        Self { repo: SqliteProjectTemplateRepository }
    }

    pub async fn get_template(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectTemplate> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn list_templates(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ProjectTemplate>> {
        self.repo.find_all(pool, pagination).await
    }

    pub async fn create_template(&self, pool: &SqlitePool, mut template: ProjectTemplate) -> Result<ProjectTemplate> {
        template.id = Uuid::new_v4();
        template.created_at = Utc::now();
        template.updated_at = Utc::now();
        self.repo.create(pool, template).await
    }

    pub async fn update_template(&self, pool: &SqlitePool, mut template: ProjectTemplate) -> Result<ProjectTemplate> {
        template.updated_at = Utc::now();
        self.repo.update(pool, template).await
    }

    pub async fn delete_template(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }
}
