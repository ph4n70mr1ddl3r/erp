use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Error, Result, Pagination, Paginated};
use crate::models::*;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Project>;
    async fn find_by_number(&self, number: &str) -> Result<Project>;
    async fn find_all(&self, pagination: Pagination) -> Result<Paginated<Project>>;
    async fn create(&self, project: Project) -> Result<Project>;
    async fn update(&self, project: Project) -> Result<Project>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectRepository {
    pool: SqlitePool,
}

impl SqliteProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for SqliteProjectRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Project> {
        Err(Error::not_found("Project", &id.to_string()))
    }

    async fn find_by_number(&self, number: &str) -> Result<Project> {
        Err(Error::not_found("Project", number))
    }

    async fn find_all(&self, pagination: Pagination) -> Result<Paginated<Project>> {
        Ok(Paginated::new(vec![], 0, pagination))
    }

    async fn create(&self, project: Project) -> Result<Project> {
        Ok(project)
    }

    async fn update(&self, project: Project) -> Result<Project> {
        Ok(project)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("Project", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectTaskRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectTask>;
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<ProjectTask>>;
    async fn create(&self, task: ProjectTask) -> Result<ProjectTask>;
    async fn update(&self, task: ProjectTask) -> Result<ProjectTask>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectTaskRepository {
    pool: SqlitePool,
}

impl SqliteProjectTaskRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectTaskRepository for SqliteProjectTaskRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectTask> {
        Err(Error::not_found("ProjectTask", &id.to_string()))
    }

    async fn find_by_project(&self, _project_id: Uuid) -> Result<Vec<ProjectTask>> {
        Ok(vec![])
    }

    async fn create(&self, task: ProjectTask) -> Result<ProjectTask> {
        Ok(task)
    }

    async fn update(&self, task: ProjectTask) -> Result<ProjectTask> {
        Ok(task)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectTask", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectMilestoneRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectMilestone>;
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<ProjectMilestone>>;
    async fn create(&self, milestone: ProjectMilestone) -> Result<ProjectMilestone>;
    async fn update(&self, milestone: ProjectMilestone) -> Result<ProjectMilestone>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectMilestoneRepository {
    pool: SqlitePool,
}

impl SqliteProjectMilestoneRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectMilestoneRepository for SqliteProjectMilestoneRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectMilestone> {
        Err(Error::not_found("ProjectMilestone", &id.to_string()))
    }

    async fn find_by_project(&self, _project_id: Uuid) -> Result<Vec<ProjectMilestone>> {
        Ok(vec![])
    }

    async fn create(&self, milestone: ProjectMilestone) -> Result<ProjectMilestone> {
        Ok(milestone)
    }

    async fn update(&self, milestone: ProjectMilestone) -> Result<ProjectMilestone> {
        Ok(milestone)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectMilestone", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectExpenseRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectExpense>;
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<ProjectExpense>>;
    async fn create(&self, expense: ProjectExpense) -> Result<ProjectExpense>;
    async fn update(&self, expense: ProjectExpense) -> Result<ProjectExpense>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectExpenseRepository {
    pool: SqlitePool,
}

impl SqliteProjectExpenseRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectExpenseRepository for SqliteProjectExpenseRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectExpense> {
        Err(Error::not_found("ProjectExpense", &id.to_string()))
    }

    async fn find_by_project(&self, _project_id: Uuid) -> Result<Vec<ProjectExpense>> {
        Ok(vec![])
    }

    async fn create(&self, expense: ProjectExpense) -> Result<ProjectExpense> {
        Ok(expense)
    }

    async fn update(&self, expense: ProjectExpense) -> Result<ProjectExpense> {
        Ok(expense)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectExpense", &id.to_string()))
    }
}

#[async_trait]
pub trait TimesheetRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Timesheet>;
    async fn find_by_number(&self, number: &str) -> Result<Timesheet>;
    async fn find_by_employee(&self, employee_id: Uuid) -> Result<Vec<Timesheet>>;
    async fn find_all(&self, pagination: Pagination) -> Result<Paginated<Timesheet>>;
    async fn create(&self, timesheet: Timesheet) -> Result<Timesheet>;
    async fn update(&self, timesheet: Timesheet) -> Result<Timesheet>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteTimesheetRepository {
    pool: SqlitePool,
}

impl SqliteTimesheetRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TimesheetRepository for SqliteTimesheetRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Timesheet> {
        Err(Error::not_found("Timesheet", &id.to_string()))
    }

    async fn find_by_number(&self, number: &str) -> Result<Timesheet> {
        Err(Error::not_found("Timesheet", number))
    }

    async fn find_by_employee(&self, _employee_id: Uuid) -> Result<Vec<Timesheet>> {
        Ok(vec![])
    }

    async fn find_all(&self, pagination: Pagination) -> Result<Paginated<Timesheet>> {
        Ok(Paginated::new(vec![], 0, pagination))
    }

    async fn create(&self, timesheet: Timesheet) -> Result<Timesheet> {
        Ok(timesheet)
    }

    async fn update(&self, timesheet: Timesheet) -> Result<Timesheet> {
        Ok(timesheet)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("Timesheet", &id.to_string()))
    }
}

#[async_trait]
pub trait TimesheetEntryRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<TimesheetEntry>;
    async fn find_by_timesheet(&self, timesheet_id: Uuid) -> Result<Vec<TimesheetEntry>>;
    async fn create(&self, entry: TimesheetEntry) -> Result<TimesheetEntry>;
    async fn update(&self, entry: TimesheetEntry) -> Result<TimesheetEntry>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteTimesheetEntryRepository {
    pool: SqlitePool,
}

impl SqliteTimesheetEntryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TimesheetEntryRepository for SqliteTimesheetEntryRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<TimesheetEntry> {
        Err(Error::not_found("TimesheetEntry", &id.to_string()))
    }

    async fn find_by_timesheet(&self, _timesheet_id: Uuid) -> Result<Vec<TimesheetEntry>> {
        Ok(vec![])
    }

    async fn create(&self, entry: TimesheetEntry) -> Result<TimesheetEntry> {
        Ok(entry)
    }

    async fn update(&self, entry: TimesheetEntry) -> Result<TimesheetEntry> {
        Ok(entry)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("TimesheetEntry", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectBillingRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectBilling>;
    async fn find_by_number(&self, number: &str) -> Result<ProjectBilling>;
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<ProjectBilling>>;
    async fn create(&self, billing: ProjectBilling) -> Result<ProjectBilling>;
    async fn update(&self, billing: ProjectBilling) -> Result<ProjectBilling>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectBillingRepository {
    pool: SqlitePool,
}

impl SqliteProjectBillingRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectBillingRepository for SqliteProjectBillingRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectBilling> {
        Err(Error::not_found("ProjectBilling", &id.to_string()))
    }

    async fn find_by_number(&self, number: &str) -> Result<ProjectBilling> {
        Err(Error::not_found("ProjectBilling", number))
    }

    async fn find_by_project(&self, _project_id: Uuid) -> Result<Vec<ProjectBilling>> {
        Ok(vec![])
    }

    async fn create(&self, billing: ProjectBilling) -> Result<ProjectBilling> {
        Ok(billing)
    }

    async fn update(&self, billing: ProjectBilling) -> Result<ProjectBilling> {
        Ok(billing)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectBilling", &id.to_string()))
    }
}

#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Skill>;
    async fn find_all(&self, pagination: Pagination) -> Result<Paginated<Skill>>;
    async fn create(&self, skill: Skill) -> Result<Skill>;
    async fn update(&self, skill: Skill) -> Result<Skill>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteSkillRepository {
    pool: SqlitePool,
}

impl SqliteSkillRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SkillRepository for SqliteSkillRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Skill> {
        Err(Error::not_found("Skill", &id.to_string()))
    }

    async fn find_all(&self, pagination: Pagination) -> Result<Paginated<Skill>> {
        Ok(Paginated::new(vec![], 0, pagination))
    }

    async fn create(&self, skill: Skill) -> Result<Skill> {
        Ok(skill)
    }

    async fn update(&self, skill: Skill) -> Result<Skill> {
        Ok(skill)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("Skill", &id.to_string()))
    }
}

#[async_trait]
pub trait ResourceSkillRepository: Send + Sync {
    async fn find_by_employee(&self, employee_id: Uuid) -> Result<Vec<ResourceSkill>>;
    async fn create(&self, skill: ResourceSkill) -> Result<ResourceSkill>;
    async fn update(&self, skill: ResourceSkill) -> Result<ResourceSkill>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteResourceSkillRepository {
    pool: SqlitePool,
}

impl SqliteResourceSkillRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ResourceSkillRepository for SqliteResourceSkillRepository {
    async fn find_by_employee(&self, _employee_id: Uuid) -> Result<Vec<ResourceSkill>> {
        Ok(vec![])
    }

    async fn create(&self, skill: ResourceSkill) -> Result<ResourceSkill> {
        Ok(skill)
    }

    async fn update(&self, skill: ResourceSkill) -> Result<ResourceSkill> {
        Ok(skill)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("ResourceSkill", &id.to_string()))
    }
}

#[async_trait]
pub trait ResourceRequestRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ResourceRequest>;
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<ResourceRequest>>;
    async fn create(&self, request: ResourceRequest) -> Result<ResourceRequest>;
    async fn update(&self, request: ResourceRequest) -> Result<ResourceRequest>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteResourceRequestRepository {
    pool: SqlitePool,
}

impl SqliteResourceRequestRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ResourceRequestRepository for SqliteResourceRequestRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ResourceRequest> {
        Err(Error::not_found("ResourceRequest", &id.to_string()))
    }

    async fn find_by_project(&self, _project_id: Uuid) -> Result<Vec<ResourceRequest>> {
        Ok(vec![])
    }

    async fn create(&self, request: ResourceRequest) -> Result<ResourceRequest> {
        Ok(request)
    }

    async fn update(&self, request: ResourceRequest) -> Result<ResourceRequest> {
        Ok(request)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("ResourceRequest", &id.to_string()))
    }
}

#[async_trait]
pub trait ResourceAllocationRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ResourceAllocation>;
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<ResourceAllocation>>;
    async fn find_by_employee(&self, employee_id: Uuid) -> Result<Vec<ResourceAllocation>>;
    async fn create(&self, allocation: ResourceAllocation) -> Result<ResourceAllocation>;
    async fn update(&self, allocation: ResourceAllocation) -> Result<ResourceAllocation>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteResourceAllocationRepository {
    pool: SqlitePool,
}

impl SqliteResourceAllocationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ResourceAllocationRepository for SqliteResourceAllocationRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ResourceAllocation> {
        Err(Error::not_found("ResourceAllocation", &id.to_string()))
    }

    async fn find_by_project(&self, _project_id: Uuid) -> Result<Vec<ResourceAllocation>> {
        Ok(vec![])
    }

    async fn find_by_employee(&self, _employee_id: Uuid) -> Result<Vec<ResourceAllocation>> {
        Ok(vec![])
    }

    async fn create(&self, allocation: ResourceAllocation) -> Result<ResourceAllocation> {
        Ok(allocation)
    }

    async fn update(&self, allocation: ResourceAllocation) -> Result<ResourceAllocation> {
        Ok(allocation)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("ResourceAllocation", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectTemplate>;
    async fn find_all(&self, pagination: Pagination) -> Result<Paginated<ProjectTemplate>>;
    async fn create(&self, template: ProjectTemplate) -> Result<ProjectTemplate>;
    async fn update(&self, template: ProjectTemplate) -> Result<ProjectTemplate>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectTemplateRepository {
    pool: SqlitePool,
}

impl SqliteProjectTemplateRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectTemplateRepository for SqliteProjectTemplateRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<ProjectTemplate> {
        Err(Error::not_found("ProjectTemplate", &id.to_string()))
    }

    async fn find_all(&self, pagination: Pagination) -> Result<Paginated<ProjectTemplate>> {
        Ok(Paginated::new(vec![], 0, pagination))
    }

    async fn create(&self, template: ProjectTemplate) -> Result<ProjectTemplate> {
        Ok(template)
    }

    async fn update(&self, template: ProjectTemplate) -> Result<ProjectTemplate> {
        Ok(template)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectTemplate", &id.to_string()))
    }
}

