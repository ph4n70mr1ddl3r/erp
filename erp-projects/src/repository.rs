use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Error, Result, Pagination, Paginated};
use crate::models::*;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Project>;
    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<Project>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Project>>;
    async fn create(&self, pool: &SqlitePool, project: Project) -> Result<Project>;
    async fn update(&self, pool: &SqlitePool, project: Project) -> Result<Project>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectRepository;

#[async_trait]
impl ProjectRepository for SqliteProjectRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Project> {
        Err(Error::not_found("Project", &id.to_string()))
    }

    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<Project> {
        Err(Error::not_found("Project", number))
    }

    async fn find_all(&self, pool: &SqlitePool, _pagination: Pagination) -> Result<Paginated<Project>> {
        Ok(Paginated::new(vec![], 0, _pagination))
    }

    async fn create(&self, _pool: &SqlitePool, project: Project) -> Result<Project> {
        Ok(project)
    }

    async fn update(&self, _pool: &SqlitePool, project: Project) -> Result<Project> {
        Ok(project)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("Project", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectTaskRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectTask>;
    async fn find_by_project(&self, pool: &SqlitePool, project_id: Uuid) -> Result<Vec<ProjectTask>>;
    async fn create(&self, pool: &SqlitePool, task: ProjectTask) -> Result<ProjectTask>;
    async fn update(&self, pool: &SqlitePool, task: ProjectTask) -> Result<ProjectTask>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectTaskRepository;

#[async_trait]
impl ProjectTaskRepository for SqliteProjectTaskRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectTask> {
        Err(Error::not_found("ProjectTask", &id.to_string()))
    }

    async fn find_by_project(&self, _pool: &SqlitePool, _project_id: Uuid) -> Result<Vec<ProjectTask>> {
        Ok(vec![])
    }

    async fn create(&self, _pool: &SqlitePool, task: ProjectTask) -> Result<ProjectTask> {
        Ok(task)
    }

    async fn update(&self, _pool: &SqlitePool, task: ProjectTask) -> Result<ProjectTask> {
        Ok(task)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectTask", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectMilestoneRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectMilestone>;
    async fn find_by_project(&self, pool: &SqlitePool, project_id: Uuid) -> Result<Vec<ProjectMilestone>>;
    async fn create(&self, pool: &SqlitePool, milestone: ProjectMilestone) -> Result<ProjectMilestone>;
    async fn update(&self, pool: &SqlitePool, milestone: ProjectMilestone) -> Result<ProjectMilestone>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectMilestoneRepository;

#[async_trait]
impl ProjectMilestoneRepository for SqliteProjectMilestoneRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectMilestone> {
        Err(Error::not_found("ProjectMilestone", &id.to_string()))
    }

    async fn find_by_project(&self, _pool: &SqlitePool, _project_id: Uuid) -> Result<Vec<ProjectMilestone>> {
        Ok(vec![])
    }

    async fn create(&self, _pool: &SqlitePool, milestone: ProjectMilestone) -> Result<ProjectMilestone> {
        Ok(milestone)
    }

    async fn update(&self, _pool: &SqlitePool, milestone: ProjectMilestone) -> Result<ProjectMilestone> {
        Ok(milestone)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectMilestone", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectExpenseRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectExpense>;
    async fn find_by_project(&self, pool: &SqlitePool, project_id: Uuid) -> Result<Vec<ProjectExpense>>;
    async fn create(&self, pool: &SqlitePool, expense: ProjectExpense) -> Result<ProjectExpense>;
    async fn update(&self, pool: &SqlitePool, expense: ProjectExpense) -> Result<ProjectExpense>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectExpenseRepository;

#[async_trait]
impl ProjectExpenseRepository for SqliteProjectExpenseRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectExpense> {
        Err(Error::not_found("ProjectExpense", &id.to_string()))
    }

    async fn find_by_project(&self, _pool: &SqlitePool, _project_id: Uuid) -> Result<Vec<ProjectExpense>> {
        Ok(vec![])
    }

    async fn create(&self, _pool: &SqlitePool, expense: ProjectExpense) -> Result<ProjectExpense> {
        Ok(expense)
    }

    async fn update(&self, _pool: &SqlitePool, expense: ProjectExpense) -> Result<ProjectExpense> {
        Ok(expense)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectExpense", &id.to_string()))
    }
}

#[async_trait]
pub trait TimesheetRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Timesheet>;
    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<Timesheet>;
    async fn find_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<Timesheet>>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Timesheet>>;
    async fn create(&self, pool: &SqlitePool, timesheet: Timesheet) -> Result<Timesheet>;
    async fn update(&self, pool: &SqlitePool, timesheet: Timesheet) -> Result<Timesheet>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteTimesheetRepository;

#[async_trait]
impl TimesheetRepository for SqliteTimesheetRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Timesheet> {
        Err(Error::not_found("Timesheet", &id.to_string()))
    }

    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<Timesheet> {
        Err(Error::not_found("Timesheet", number))
    }

    async fn find_by_employee(&self, _pool: &SqlitePool, _employee_id: Uuid) -> Result<Vec<Timesheet>> {
        Ok(vec![])
    }

    async fn find_all(&self, _pool: &SqlitePool, _pagination: Pagination) -> Result<Paginated<Timesheet>> {
        Ok(Paginated::new(vec![], 0, _pagination))
    }

    async fn create(&self, _pool: &SqlitePool, timesheet: Timesheet) -> Result<Timesheet> {
        Ok(timesheet)
    }

    async fn update(&self, _pool: &SqlitePool, timesheet: Timesheet) -> Result<Timesheet> {
        Ok(timesheet)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("Timesheet", &id.to_string()))
    }
}

#[async_trait]
pub trait TimesheetEntryRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TimesheetEntry>;
    async fn find_by_timesheet(&self, pool: &SqlitePool, timesheet_id: Uuid) -> Result<Vec<TimesheetEntry>>;
    async fn create(&self, pool: &SqlitePool, entry: TimesheetEntry) -> Result<TimesheetEntry>;
    async fn update(&self, pool: &SqlitePool, entry: TimesheetEntry) -> Result<TimesheetEntry>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteTimesheetEntryRepository;

#[async_trait]
impl TimesheetEntryRepository for SqliteTimesheetEntryRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TimesheetEntry> {
        Err(Error::not_found("TimesheetEntry", &id.to_string()))
    }

    async fn find_by_timesheet(&self, _pool: &SqlitePool, _timesheet_id: Uuid) -> Result<Vec<TimesheetEntry>> {
        Ok(vec![])
    }

    async fn create(&self, _pool: &SqlitePool, entry: TimesheetEntry) -> Result<TimesheetEntry> {
        Ok(entry)
    }

    async fn update(&self, _pool: &SqlitePool, entry: TimesheetEntry) -> Result<TimesheetEntry> {
        Ok(entry)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("TimesheetEntry", &id.to_string()))
    }
}

#[async_trait]
pub trait ProjectBillingRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectBilling>;
    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<ProjectBilling>;
    async fn find_by_project(&self, pool: &SqlitePool, project_id: Uuid) -> Result<Vec<ProjectBilling>>;
    async fn create(&self, pool: &SqlitePool, billing: ProjectBilling) -> Result<ProjectBilling>;
    async fn update(&self, pool: &SqlitePool, billing: ProjectBilling) -> Result<ProjectBilling>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteProjectBillingRepository;

#[async_trait]
impl ProjectBillingRepository for SqliteProjectBillingRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ProjectBilling> {
        Err(Error::not_found("ProjectBilling", &id.to_string()))
    }

    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<ProjectBilling> {
        Err(Error::not_found("ProjectBilling", number))
    }

    async fn find_by_project(&self, _pool: &SqlitePool, _project_id: Uuid) -> Result<Vec<ProjectBilling>> {
        Ok(vec![])
    }

    async fn create(&self, _pool: &SqlitePool, billing: ProjectBilling) -> Result<ProjectBilling> {
        Ok(billing)
    }

    async fn update(&self, _pool: &SqlitePool, billing: ProjectBilling) -> Result<ProjectBilling> {
        Ok(billing)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        Err(Error::not_found("ProjectBilling", &id.to_string()))
    }
}
