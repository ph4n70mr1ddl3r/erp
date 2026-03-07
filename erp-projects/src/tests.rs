#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::repository::*;
    use crate::service::ResourceService;
    use anyhow::Result;
    use async_trait::async_trait;
    use chrono::Utc;
    use erp_core::{Pagination, Paginated};
    use sqlx::SqlitePool;
    use uuid::Uuid;
    use std::sync::{Arc, Mutex};

    struct MockSkillRepository {
        _skills: Mutex<Vec<Skill>>,
    }
    #[async_trait]
    impl SkillRepository for MockSkillRepository {
        async fn find_by_id(&self, _: &SqlitePool, id: Uuid) -> erp_core::Result<Skill> {
            Ok(Skill { id, name: "Test".to_string(), category: "Test".to_string(), description: None })
        }
        async fn find_all(&self, _: &SqlitePool, p: Pagination) -> erp_core::Result<Paginated<Skill>> {
            Ok(Paginated::new(vec![], 0, p))
        }
        async fn create(&self, _: &SqlitePool, s: Skill) -> erp_core::Result<Skill> { Ok(s) }
        async fn update(&self, _: &SqlitePool, s: Skill) -> erp_core::Result<Skill> { Ok(s) }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockResourceSkillRepository {
        _skills: Mutex<Vec<ResourceSkill>>,
    }
    #[async_trait]
    impl ResourceSkillRepository for MockResourceSkillRepository {
        async fn find_by_employee(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<Vec<ResourceSkill>> { Ok(vec![]) }
        async fn create(&self, _: &SqlitePool, s: ResourceSkill) -> erp_core::Result<ResourceSkill> { Ok(s) }
        async fn update(&self, _: &SqlitePool, s: ResourceSkill) -> erp_core::Result<ResourceSkill> { Ok(s) }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockResourceRequestRepository {
        requests: Mutex<Vec<ResourceRequest>>,
    }
    #[async_trait]
    impl ResourceRequestRepository for MockResourceRequestRepository {
        async fn find_by_id(&self, _: &SqlitePool, id: Uuid) -> erp_core::Result<ResourceRequest> {
            self.requests.lock().unwrap().iter().find(|r| r.id == id).cloned()
                .ok_or_else(|| erp_core::Error::not_found("ResourceRequest", &id.to_string()))
        }
        async fn find_by_project(&self, _: &SqlitePool, id: Uuid) -> erp_core::Result<Vec<ResourceRequest>> {
            Ok(self.requests.lock().unwrap().iter().filter(|r| r.project_id == id).cloned().collect())
        }
        async fn create(&self, _: &SqlitePool, r: ResourceRequest) -> erp_core::Result<ResourceRequest> {
            self.requests.lock().unwrap().push(r.clone());
            Ok(r)
        }
        async fn update(&self, _: &SqlitePool, r: ResourceRequest) -> erp_core::Result<ResourceRequest> {
            let mut reqs = self.requests.lock().unwrap();
            if let Some(pos) = reqs.iter().position(|x| x.id == r.id) {
                reqs[pos] = r.clone();
            }
            Ok(r)
        }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockResourceAllocationRepository {
        allocations: Mutex<Vec<ResourceAllocation>>,
    }
    #[async_trait]
    impl ResourceAllocationRepository for MockResourceAllocationRepository {
        async fn find_by_id(&self, _: &SqlitePool, id: Uuid) -> erp_core::Result<ResourceAllocation> {
            self.allocations.lock().unwrap().iter().find(|a| a.id == id).cloned()
                .ok_or_else(|| erp_core::Error::not_found("ResourceAllocation", &id.to_string()))
        }
        async fn find_by_project(&self, _: &SqlitePool, id: Uuid) -> erp_core::Result<Vec<ResourceAllocation>> {
            Ok(self.allocations.lock().unwrap().iter().filter(|a| a.project_id == id).cloned().collect())
        }
        async fn find_by_employee(&self, _: &SqlitePool, id: Uuid) -> erp_core::Result<Vec<ResourceAllocation>> {
            Ok(self.allocations.lock().unwrap().iter().filter(|a| a.employee_id == id).cloned().collect())
        }
        async fn create(&self, _: &SqlitePool, a: ResourceAllocation) -> erp_core::Result<ResourceAllocation> {
            self.allocations.lock().unwrap().push(a.clone());
            Ok(a)
        }
        async fn update(&self, _: &SqlitePool, a: ResourceAllocation) -> erp_core::Result<ResourceAllocation> {
            let mut allocs = self.allocations.lock().unwrap();
            if let Some(pos) = allocs.iter().position(|x| x.id == a.id) {
                allocs[pos] = a.clone();
            }
            Ok(a)
        }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    #[tokio::test]
    async fn test_resource_request_lifecycle() -> Result<()> {
        let pool = SqlitePool::connect("sqlite::memory:").await?; 
        let skill_repo = MockSkillRepository { _skills: Mutex::new(Vec::new()) };
        let rs_repo = MockResourceSkillRepository { _skills: Mutex::new(Vec::new()) };
        let rq_repo = Arc::new(MockResourceRequestRepository { requests: Mutex::new(Vec::new()) });
        let a_repo = MockResourceAllocationRepository { allocations: Mutex::new(Vec::new()) };
        
        // I need RQ to be Arc to share it with the service and the test check
        // But the service takes the repo by value or Clone. 
        // If I make the service take the repo by Arc, it works.
        // Or I implement the trait for Arc<Mock...>.
        
        struct ArcRQ(Arc<MockResourceRequestRepository>);
        #[async_trait]
        impl ResourceRequestRepository for ArcRQ {
            async fn find_by_id(&self, p: &SqlitePool, id: Uuid) -> erp_core::Result<ResourceRequest> { self.0.find_by_id(p, id).await }
            async fn find_by_project(&self, p: &SqlitePool, id: Uuid) -> erp_core::Result<Vec<ResourceRequest>> { self.0.find_by_project(p, id).await }
            async fn create(&self, p: &SqlitePool, r: ResourceRequest) -> erp_core::Result<ResourceRequest> { self.0.create(p, r).await }
            async fn update(&self, p: &SqlitePool, r: ResourceRequest) -> erp_core::Result<ResourceRequest> { self.0.update(p, r).await }
            async fn delete(&self, p: &SqlitePool, id: Uuid) -> erp_core::Result<()> { self.0.delete(p, id).await }
        }

        let service = ResourceService::with_repos(
            skill_repo,
            rs_repo,
            ArcRQ(rq_repo.clone()),
            a_repo,
        );

        let project_id = Uuid::new_v4();
        let request = ResourceRequest {
            id: Uuid::new_v4(),
            project_id,
            task_id: None,
            skill_id: Uuid::new_v4(),
            min_proficiency: 3,
            start_date: Utc::now(),
            end_date: Utc::now(),
            hours_required: 40.0,
            status: ResourceRequestStatus::Draft,
            requested_by: Uuid::new_v4(),
            created_at: Utc::now(),
        };

        let created = service.create_request(&pool, request).await?;
        assert_eq!(created.status, ResourceRequestStatus::Draft);

        service.submit_request(&pool, created.id).await?;
        
        let req = rq_repo.find_by_id(&pool, created.id).await?;
        assert_eq!(req.status, ResourceRequestStatus::Pending);

        let allocation = ResourceAllocation {
            id: Uuid::new_v4(),
            project_id,
            employee_id: Uuid::new_v4(),
            request_id: Some(created.id),
            start_date: Utc::now(),
            end_date: Utc::now(),
            allocation_percent: 100,
            billable_rate: Some(100),
            created_at: Utc::now(),
        };

        let allocated = service.allocate_resource(&pool, allocation).await?;
        assert_eq!(allocated.project_id, project_id);

        let req = rq_repo.find_by_id(&pool, created.id).await?;
        assert_eq!(req.status, ResourceRequestStatus::Fulfilled);

        Ok(())
    }

    struct MockProjectRepository {
        projects: Mutex<Vec<Project>>,
    }
    #[async_trait]
    impl ProjectRepository for MockProjectRepository {
        async fn find_by_id(&self, _: &SqlitePool, id: Uuid) -> erp_core::Result<Project> {
            self.projects.lock().unwrap().iter().find(|p| p.id == id).cloned()
                .ok_or_else(|| erp_core::Error::not_found("Project", &id.to_string()))
        }
        async fn find_by_number(&self, _: &SqlitePool, number: &str) -> erp_core::Result<Project> {
            self.projects.lock().unwrap().iter().find(|p| p.project_number == number).cloned()
                .ok_or_else(|| erp_core::Error::not_found("Project", number))
        }
        async fn find_all(&self, _: &SqlitePool, p: Pagination) -> erp_core::Result<Paginated<Project>> {
            let projects = self.projects.lock().unwrap().clone();
            let total = projects.len() as u64;
            Ok(Paginated::new(projects, total, p))
        }
        async fn create(&self, _: &SqlitePool, p: Project) -> erp_core::Result<Project> {
            self.projects.lock().unwrap().push(p.clone());
            Ok(p)
        }
        async fn update(&self, _: &SqlitePool, p: Project) -> erp_core::Result<Project> {
            let mut projs = self.projects.lock().unwrap();
            if let Some(pos) = projs.iter().position(|x| x.id == p.id) {
                projs[pos] = p.clone();
            }
            Ok(p)
        }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockProjectTaskRepository;
    #[async_trait]
    impl ProjectTaskRepository for MockProjectTaskRepository {
        async fn find_by_id(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<ProjectTask> { todo!() }
        async fn find_by_project(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<Vec<ProjectTask>> { Ok(vec![]) }
        async fn create(&self, _: &SqlitePool, t: ProjectTask) -> erp_core::Result<ProjectTask> { Ok(t) }
        async fn update(&self, _: &SqlitePool, t: ProjectTask) -> erp_core::Result<ProjectTask> { Ok(t) }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockProjectMilestoneRepository;
    #[async_trait]
    impl ProjectMilestoneRepository for MockProjectMilestoneRepository {
        async fn find_by_id(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<ProjectMilestone> { todo!() }
        async fn find_by_project(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<Vec<ProjectMilestone>> { Ok(vec![]) }
        async fn create(&self, _: &SqlitePool, m: ProjectMilestone) -> erp_core::Result<ProjectMilestone> { Ok(m) }
        async fn update(&self, _: &SqlitePool, m: ProjectMilestone) -> erp_core::Result<ProjectMilestone> { Ok(m) }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockProjectExpenseRepository;
    #[async_trait]
    impl ProjectExpenseRepository for MockProjectExpenseRepository {
        async fn find_by_id(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<ProjectExpense> { todo!() }
        async fn find_by_project(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<Vec<ProjectExpense>> { Ok(vec![]) }
        async fn create(&self, _: &SqlitePool, e: ProjectExpense) -> erp_core::Result<ProjectExpense> { Ok(e) }
        async fn update(&self, _: &SqlitePool, e: ProjectExpense) -> erp_core::Result<ProjectExpense> { Ok(e) }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockProjectTemplateRepository;
    #[async_trait]
    impl ProjectTemplateRepository for MockProjectTemplateRepository {
        async fn find_by_id(&self, _: &SqlitePool, id: Uuid) -> erp_core::Result<ProjectTemplate> {
            Ok(ProjectTemplate {
                id,
                name: "Test Template".to_string(),
                description: None,
                project_type: ProjectType::Internal,
                billable: true,
                billing_method: BillingMethod::FixedPrice,
                tasks: vec![],
                milestones: vec![],
                status: erp_core::Status::Active,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }
        async fn find_all(&self, _: &SqlitePool, p: Pagination) -> erp_core::Result<Paginated<ProjectTemplate>> {
            Ok(Paginated::new(vec![], 0, p))
        }
        async fn create(&self, _: &SqlitePool, t: ProjectTemplate) -> erp_core::Result<ProjectTemplate> { Ok(t) }
        async fn update(&self, _: &SqlitePool, t: ProjectTemplate) -> erp_core::Result<ProjectTemplate> { Ok(t) }
        async fn delete(&self, _: &SqlitePool, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    #[tokio::test]
    async fn test_create_project_from_template_logic() -> Result<()> {
        use crate::service::ProjectService;
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        let service = ProjectService::with_repos(
            MockProjectRepository { projects: Mutex::new(Vec::new()) },
            MockProjectTaskRepository,
            MockProjectMilestoneRepository,
            MockProjectExpenseRepository,
            MockProjectTemplateRepository,
        );

        let project = service.create_project_from_template(
            &pool,
            Uuid::new_v4(),
            "New Project".to_string(),
            None,
            Utc::now(),
        ).await?;

        assert_eq!(project.name, "New Project");
        assert_eq!(project.status, ProjectStatus::Planning);
        
        Ok(())
    }
}
