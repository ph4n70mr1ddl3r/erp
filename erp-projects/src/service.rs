use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use erp_core::{BaseEntity, Error, Result, Pagination, Paginated};
use crate::models::*;
use crate::repository::*;
use tracing::info;

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

impl ProjectService<
    SqliteProjectRepository,
    SqliteProjectTaskRepository,
    SqliteProjectMilestoneRepository,
    SqliteProjectExpenseRepository,
    SqliteProjectTemplateRepository,
> {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteProjectRepository::new(pool.clone()),
            task_repo: SqliteProjectTaskRepository::new(pool.clone()),
            milestone_repo: SqliteProjectMilestoneRepository::new(pool.clone()),
            expense_repo: SqliteProjectExpenseRepository::new(pool.clone()),
            template_repo: SqliteProjectTemplateRepository::new(pool),
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

    pub async fn get_project(&self, id: Uuid) -> Result<Project> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_project_by_number(&self, number: &str) -> Result<Project> {
        self.repo.find_by_number(number).await
    }

    pub async fn list_projects(&self, pagination: Pagination) -> Result<Paginated<Project>> {
        self.repo.find_all(pagination).await
    }

    pub async fn create_project(&self, mut project: Project, created_by: Option<Uuid>) -> Result<Project> {
        project.base = BaseEntity::new();
        if let Some(uid) = created_by {
            project.base.created_by = Some(uid);
        }
        project.project_number = format!("PRJ-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        let project = self.repo.create(project).await?;
        info!("Created project {} ({})", project.name, project.project_number);
        Ok(project)
    }

    pub async fn create_project_from_template(
        &self,
        template_id: Uuid,
        name: String,
        customer_id: Option<Uuid>,
        start_date: DateTime<Utc>,
        created_by: Option<Uuid>,
    ) -> Result<Project> {
        let template = self.template_repo.find_by_id(template_id).await?;
        
        let mut project = Project {
            base: BaseEntity::new(),
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
        };

        if let Some(uid) = created_by {
            project.base.created_by = Some(uid);
        }

        let created_project = self.repo.create(project).await?;

        // Create tasks from template
        for t_task in template.tasks {
            let mut task = ProjectTask {
                base: BaseEntity::new(),
                project_id: created_project.base.id,
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
            if let Some(uid) = created_by {
                task.base.created_by = Some(uid);
            }
            self.task_repo.create(task).await?;
        }

        // Create milestones from template
        for t_milestone in template.milestones {
            let mut milestone = ProjectMilestone {
                base: BaseEntity::new(),
                project_id: created_project.base.id,
                name: t_milestone.name,
                description: t_milestone.description,
                planned_date: start_date + chrono::Duration::days(t_milestone.relative_day as i64),
                actual_date: None,
                billing_amount: t_milestone.billing_amount,
                billing_status: BillingStatus::NotBilled,
                status: MilestoneStatus::Planned,
            };
            if let Some(uid) = created_by {
                milestone.base.created_by = Some(uid);
            }
            self.milestone_repo.create(milestone).await?;
        }

        info!("Created project {} from template {}", created_project.project_number, template_id);
        Ok(created_project)
    }

    pub async fn update_project(&self, mut project: Project, updated_by: Option<Uuid>) -> Result<Project> {
        let existing = self.repo.find_by_id(project.base.id).await?;
        project.base.created_at = existing.base.created_at;
        project.base.created_by = existing.base.created_by;
        project.base.updated_at = Utc::now();
        project.base.updated_by = updated_by;
        let project = self.repo.update(project).await?;
        info!("Updated project {}", project.project_number);
        Ok(project)
    }

    pub async fn delete_project(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await?;
        info!("Deleted project {}", id);
        Ok(())
    }

    pub async fn update_status(&self, id: Uuid, status: ProjectStatus, updated_by: Option<Uuid>) -> Result<Project> {
        let mut project = self.repo.find_by_id(id).await?;
        project.status = status;
        if project.status == ProjectStatus::Completed {
            project.end_date = Some(Utc::now());
        }
        project.base.updated_at = Utc::now();
        project.base.updated_by = updated_by;
        let project = self.repo.update(project).await?;
        info!("Updated project {} status to {:?}", project.project_number, project.status);
        Ok(project)
    }

    pub async fn calculate_progress(&self, id: Uuid) -> Result<i32> {
        let tasks = self.task_repo.find_by_project(id).await?;
        if tasks.is_empty() {
            return Ok(0);
        }
        let total_progress: i32 = tasks.iter().map(|t| t.percent_complete).sum();
        let avg_progress = total_progress / tasks.len() as i32;
        let mut project = self.repo.find_by_id(id).await?;
        project.percent_complete = avg_progress;
        project.base.updated_at = Utc::now();
        self.repo.update(project).await?;
        Ok(avg_progress)
    }

    pub async fn add_task(&self, mut task: ProjectTask, created_by: Option<Uuid>) -> Result<ProjectTask> {
        task.base = BaseEntity::new();
        task.base.created_by = created_by;
        let task = self.task_repo.create(task).await?;
        info!("Added task {} to project {}", task.name, task.project_id);
        Ok(task)
    }

    pub async fn update_task(&self, mut task: ProjectTask, updated_by: Option<Uuid>) -> Result<ProjectTask> {
        task.base.updated_at = Utc::now();
        task.base.updated_by = updated_by;
        let task = self.task_repo.update(task).await?;
        info!("Updated task {} in project {}", task.base.id, task.project_id);
        Ok(task)
    }

    pub async fn complete_task(&self, id: Uuid, updated_by: Option<Uuid>) -> Result<ProjectTask> {
        let mut task = self.task_repo.find_by_id(id).await?;
        task.status = TaskStatus::Completed;
        task.percent_complete = 100;
        task.base.updated_at = Utc::now();
        task.base.updated_by = updated_by;
        let task = self.task_repo.update(task).await?;
        let _ = self.calculate_progress(task.project_id).await?;
        info!("Completed task {} in project {}", id, task.project_id);
        Ok(task)
    }

    pub async fn add_milestone(&self, mut milestone: ProjectMilestone, created_by: Option<Uuid>) -> Result<ProjectMilestone> {
        milestone.base = BaseEntity::new();
        milestone.base.created_by = created_by;
        let milestone = self.milestone_repo.create(milestone).await?;
        info!("Added milestone {} to project {}", milestone.name, milestone.project_id);
        Ok(milestone)
    }

    pub async fn complete_milestone(&self, id: Uuid, updated_by: Option<Uuid>) -> Result<ProjectMilestone> {
        let mut milestone = self.milestone_repo.find_by_id(id).await?;
        milestone.status = MilestoneStatus::Completed;
        milestone.actual_date = Some(Utc::now());
        milestone.base.updated_at = Utc::now();
        milestone.base.updated_by = updated_by;
        let milestone = self.milestone_repo.update(milestone).await?;
        info!("Completed milestone {} in project {}", id, milestone.project_id);
        Ok(milestone)
    }

    pub async fn add_expense(&self, mut expense: ProjectExpense, created_by: Option<Uuid>) -> Result<ProjectExpense> {
        expense.base = BaseEntity::new();
        expense.base.created_by = created_by;
        let expense = self.expense_repo.create(expense).await?;
        info!("Added expense of amount {} to project {}", expense.amount, expense.project_id);
        Ok(expense)
    }
}

pub struct TimesheetService {
    repo: SqliteTimesheetRepository,
    entry_repo: SqliteTimesheetEntryRepository,
}

impl TimesheetService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteTimesheetRepository::new(pool.clone()),
            entry_repo: SqliteTimesheetEntryRepository::new(pool),
        }
    }

    pub async fn create_timesheet(&self, mut timesheet: Timesheet, created_by: Option<Uuid>) -> Result<Timesheet> {
        timesheet.base = BaseEntity::new();
        timesheet.base.created_by = created_by;
        timesheet.timesheet_number = format!("TS-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        timesheet.status = TimesheetStatus::Draft;
        let timesheet = self.repo.create(timesheet).await?;
        info!("Created timesheet {} for employee {}", timesheet.timesheet_number, timesheet.employee_id);
        Ok(timesheet)
    }

    pub async fn get_timesheet(&self, id: Uuid) -> Result<Timesheet> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_timesheet_by_number(&self, number: &str) -> Result<Timesheet> {
        self.repo.find_by_number(number).await
    }

    pub async fn list_timesheets(&self, pagination: Pagination) -> Result<Paginated<Timesheet>> {
        self.repo.find_all(pagination).await
    }

    pub async fn get_employee_timesheets(&self, employee_id: Uuid) -> Result<Vec<Timesheet>> {
        self.repo.find_by_employee(employee_id).await
    }

    pub async fn add_entry(&self, mut entry: TimesheetEntry, created_by: Option<Uuid>) -> Result<TimesheetEntry> {
        let timesheet = self.repo.find_by_id(entry.timesheet_id).await?;
        if timesheet.status != TimesheetStatus::Draft {
            return Err(Error::validation("Cannot add entries to a non-draft timesheet"));
        }
        entry.base = BaseEntity::new();
        entry.base.created_by = created_by;
        let entry = self.entry_repo.create(entry).await?;
        self.recalculate_hours(entry.timesheet_id).await?;
        Ok(entry)
    }

    pub async fn remove_entry(&self, entry_id: Uuid) -> Result<()> {
        let entry = self.entry_repo.find_by_id(entry_id).await?;
        let timesheet = self.repo.find_by_id(entry.timesheet_id).await?;
        if timesheet.status != TimesheetStatus::Draft {
            return Err(Error::validation("Cannot remove entries from a non-draft timesheet"));
        }
        let timesheet_id = entry.timesheet_id;
        self.entry_repo.delete(entry_id).await?;
        self.recalculate_hours(timesheet_id).await?;
        Ok(())
    }

    pub async fn submit_timesheet(&self, id: Uuid, updated_by: Option<Uuid>) -> Result<()> {
        let mut timesheet = self.repo.find_by_id(id).await?;
        if timesheet.status != TimesheetStatus::Draft {
            return Err(Error::validation("Only draft timesheets can be submitted"));
        }
        timesheet.status = TimesheetStatus::Submitted;
        timesheet.submitted_at = Some(Utc::now());
        timesheet.base.updated_at = Utc::now();
        timesheet.base.updated_by = updated_by;
        self.repo.update(timesheet).await?;
        info!("Submitted timesheet {}", id);
        Ok(())
    }

    pub async fn approve_timesheet(&self, id: Uuid, approver_id: Uuid) -> Result<()> {
        let mut timesheet = self.repo.find_by_id(id).await?;
        if timesheet.status != TimesheetStatus::Submitted {
            return Err(Error::validation("Only submitted timesheets can be approved"));
        }
        timesheet.status = TimesheetStatus::Approved;
        timesheet.approved_at = Some(Utc::now());
        timesheet.approved_by = Some(approver_id);
        timesheet.base.updated_at = Utc::now();
        timesheet.base.updated_by = Some(approver_id);
        self.repo.update(timesheet).await?;
        info!("Approved timesheet {} by {}", id, approver_id);
        Ok(())
    }

    pub async fn reject_timesheet(&self, id: Uuid, rejected_by: Option<Uuid>) -> Result<()> {
        let mut timesheet = self.repo.find_by_id(id).await?;
        if timesheet.status != TimesheetStatus::Submitted {
            return Err(Error::validation("Only submitted timesheets can be rejected"));
        }
        timesheet.status = TimesheetStatus::Rejected;
        timesheet.base.updated_at = Utc::now();
        timesheet.base.updated_by = rejected_by;
        self.repo.update(timesheet).await?;
        info!("Rejected timesheet {}", id);
        Ok(())
    }

    async fn recalculate_hours(&self, timesheet_id: Uuid) -> Result<()> {
        let entries = self.entry_repo.find_by_timesheet(timesheet_id).await?;
        let total: f64 = entries.iter().map(|e| e.hours).sum();
        let mut timesheet = self.repo.find_by_id(timesheet_id).await?;
        timesheet.total_hours = total;
        timesheet.base.updated_at = Utc::now();
        self.repo.update(timesheet).await?;
        Ok(())
    }
}

pub struct ProjectBillingService {
    repo: SqliteProjectBillingRepository,
    milestone_repo: SqliteProjectMilestoneRepository,
    timesheet_repo: SqliteTimesheetRepository,
    entry_repo: SqliteTimesheetEntryRepository,
}

impl ProjectBillingService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteProjectBillingRepository::new(pool.clone()),
            milestone_repo: SqliteProjectMilestoneRepository::new(pool.clone()),
            timesheet_repo: SqliteTimesheetRepository::new(pool.clone()),
            entry_repo: SqliteTimesheetEntryRepository::new(pool),
        }
    }

    pub async fn get_billing(&self, id: Uuid) -> Result<ProjectBilling> {
        self.repo.find_by_id(id).await
    }

    pub async fn get_billing_by_number(&self, number: &str) -> Result<ProjectBilling> {
        self.repo.find_by_number(number).await
    }

    pub async fn list_billings_by_project(&self, project_id: Uuid) -> Result<Vec<ProjectBilling>> {
        self.repo.find_by_project(project_id).await
    }

    pub async fn create_billing(&self, mut billing: ProjectBilling, created_by: Option<Uuid>) -> Result<ProjectBilling> {
        billing.base = BaseEntity::new();
        billing.base.created_by = created_by;
        billing.billing_number = format!("BL-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        billing.status = ProjectBillingStatus::Draft;
        let billing = self.repo.create(billing).await?;
        info!("Created billing {} for project {}", billing.billing_number, billing.project_id);
        Ok(billing)
    }

    pub async fn generate_from_milestone(&self, milestone_id: Uuid, created_by: Option<Uuid>) -> Result<ProjectBilling> {
        let milestone = self.milestone_repo.find_by_id(milestone_id).await?;
        if milestone.status != MilestoneStatus::Completed {
            return Err(Error::validation("Can only generate billing from completed milestones"));
        }
        let mut billing = ProjectBilling {
            base: BaseEntity::new(),
            billing_number: format!("BL-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
            project_id: milestone.project_id,
            billing_type: ProjectBillingType::Milestone,
            milestone_id: Some(milestone_id),
            period_start: None,
            period_end: None,
            amount: milestone.billing_amount,
            invoice_id: None,
            status: ProjectBillingStatus::Draft,
        };
        billing.base.created_by = created_by;
        let billing = self.repo.create(billing).await?;
        info!("Generated billing {} from milestone {}", billing.billing_number, milestone_id);
        Ok(billing)
    }

    pub async fn generate_from_timesheet(&self, timesheet_id: Uuid, created_by: Option<Uuid>) -> Result<ProjectBilling> {
        let timesheet = self.timesheet_repo.find_by_id(timesheet_id).await?;
        if timesheet.status != TimesheetStatus::Approved {
            return Err(Error::validation("Can only generate billing from approved timesheets"));
        }
        let entries = self.entry_repo.find_by_timesheet(timesheet_id).await?;
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
        let mut billing = ProjectBilling {
            base: BaseEntity::new(),
            billing_number: format!("BL-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
            project_id,
            billing_type: ProjectBillingType::TimeBased,
            milestone_id: None,
            period_start: Some(timesheet.period_start),
            period_end: Some(timesheet.period_end),
            amount: total_amount,
            invoice_id: None,
            status: ProjectBillingStatus::Draft,
        };
        billing.base.created_by = created_by;
        let billing = self.repo.create(billing).await?;
        info!("Generated billing {} from timesheet {}", billing.billing_number, timesheet_id);
        Ok(billing)
    }

    pub async fn mark_as_invoiced(&self, id: Uuid, invoice_id: Uuid, updated_by: Option<Uuid>) -> Result<ProjectBilling> {
        let mut billing = self.repo.find_by_id(id).await?;
        if billing.status == ProjectBillingStatus::Invoiced || billing.status == ProjectBillingStatus::Paid {
            return Err(Error::validation("Billing is already invoiced or paid"));
        }
        billing.status = ProjectBillingStatus::Invoiced;
        billing.invoice_id = Some(invoice_id);
        billing.base.updated_at = Utc::now();
        billing.base.updated_by = updated_by;
        let billing = self.repo.update(billing).await?;
        info!("Marked billing {} as invoiced", id);
        Ok(billing)
    }

    pub async fn update_billing(&self, mut billing: ProjectBilling, updated_by: Option<Uuid>) -> Result<ProjectBilling> {
        billing.base.updated_at = Utc::now();
        billing.base.updated_by = updated_by;
        let billing = self.repo.update(billing).await?;
        info!("Updated billing {}", billing.base.id);
        Ok(billing)
    }

    pub async fn delete_billing(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await?;
        info!("Deleted billing {}", id);
        Ok(())
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

impl ResourceService<
    SqliteSkillRepository,
    SqliteResourceSkillRepository,
    SqliteResourceRequestRepository,
    SqliteResourceAllocationRepository,
> {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            skill_repo: SqliteSkillRepository::new(pool.clone()),
            resource_skill_repo: SqliteResourceSkillRepository::new(pool.clone()),
            request_repo: SqliteResourceRequestRepository::new(pool.clone()),
            allocation_repo: SqliteResourceAllocationRepository::new(pool),
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

    pub async fn create_request(&self, mut request: ResourceRequest, created_by: Option<Uuid>) -> Result<ResourceRequest> {
        request.base = BaseEntity::new();
        request.base.created_by = created_by;
        request.status = ResourceRequestStatus::Draft;
        let request = self.request_repo.create(request).await?;
        info!("Created resource request for skill {}", request.skill_id);
        Ok(request)
    }

    pub async fn submit_request(&self, id: Uuid, updated_by: Option<Uuid>) -> Result<()> {
        let mut request = self.request_repo.find_by_id(id).await?;
        if request.status != ResourceRequestStatus::Draft {
            return Err(Error::validation("Only draft requests can be submitted"));
        }
        request.status = ResourceRequestStatus::Pending;
        request.base.updated_at = Utc::now();
        request.base.updated_by = updated_by;
        self.request_repo.update(request).await?;
        info!("Submitted resource request {}", id);
        Ok(())
    }

    pub async fn allocate_resource(&self, mut allocation: ResourceAllocation, created_by: Option<Uuid>) -> Result<ResourceAllocation> {
        allocation.base = BaseEntity::new();
        allocation.base.created_by = created_by;
        
        if let Some(request_id) = allocation.request_id {
            let mut request = self.request_repo.find_by_id(request_id).await?;
            request.status = ResourceRequestStatus::Fulfilled;
            request.base.updated_at = Utc::now();
            request.base.updated_by = created_by;
            self.request_repo.update(request).await?;
        }
        
        let allocation = self.allocation_repo.create(allocation).await?;
        info!("Allocated resource {} to project {}", allocation.employee_id, allocation.project_id);
        Ok(allocation)
    }

    pub async fn list_allocations_by_project(&self, project_id: Uuid) -> Result<Vec<ResourceAllocation>> {
        self.allocation_repo.find_by_project(project_id).await
    }

    pub async fn list_allocations_by_employee(&self, employee_id: Uuid) -> Result<Vec<ResourceAllocation>> {
        self.allocation_repo.find_by_employee(employee_id).await
    }

    pub async fn list_skills(&self, pagination: Pagination) -> Result<Paginated<Skill>> {
        self.skill_repo.find_all(pagination).await
    }

    pub async fn get_employee_skills(&self, employee_id: Uuid) -> Result<Vec<ResourceSkill>> {
        self.resource_skill_repo.find_by_employee(employee_id).await
    }
}

pub struct ProjectTemplateService {
    repo: SqliteProjectTemplateRepository,
}

impl ProjectTemplateService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { repo: SqliteProjectTemplateRepository::new(pool) }
    }

    pub async fn get_template(&self, id: Uuid) -> Result<ProjectTemplate> {
        self.repo.find_by_id(id).await
    }

    pub async fn list_templates(&self, pagination: Pagination) -> Result<Paginated<ProjectTemplate>> {
        self.repo.find_all(pagination).await
    }

    pub async fn create_template(&self, mut template: ProjectTemplate, created_by: Option<Uuid>) -> Result<ProjectTemplate> {
        template.base = BaseEntity::new();
        template.base.created_by = created_by;
        template.updated_at = Utc::now();
        let template = self.repo.create(template).await?;
        info!("Created project template {}", template.name);
        Ok(template)
    }

    pub async fn update_template(&self, mut template: ProjectTemplate, updated_by: Option<Uuid>) -> Result<ProjectTemplate> {
        template.base.updated_at = Utc::now();
        template.base.updated_by = updated_by;
        template.updated_at = Utc::now();
        let template = self.repo.update(template).await?;
        info!("Updated project template {}", template.base.id);
        Ok(template)
    }

    pub async fn delete_template(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await?;
        info!("Deleted project template {}", id);
        Ok(())
    }
}
