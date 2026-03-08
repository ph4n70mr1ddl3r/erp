#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::models::*;
    use crate::repository::*;
    use crate::service::{ResourceService, ProjectService};
    use anyhow::Result;
    use async_trait::async_trait;
    use chrono::Utc;
    use erp_core::{Pagination, Paginated, BaseEntity, Error};
    use uuid::Uuid;
    use std::sync::{Arc, Mutex};

    struct MockSkillRepository;
    #[async_trait]
    impl SkillRepository for MockSkillRepository {
        async fn find_by_id(&self, id: Uuid) -> erp_core::Result<Skill> {
            Ok(Skill { base: BaseEntity::new_with_id(id), name: "Test".to_string(), category: "Test".to_string(), description: None })
        }
        async fn find_all(&self, p: Pagination) -> erp_core::Result<Paginated<Skill>> {
            Ok(Paginated::new(vec![], 0, p))
        }
        async fn create(&self, s: Skill) -> erp_core::Result<Skill> { Ok(s) }
        async fn update(&self, s: Skill) -> erp_core::Result<Skill> { Ok(s) }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockResourceSkillRepository;
    #[async_trait]
    impl ResourceSkillRepository for MockResourceSkillRepository {
        async fn find_by_employee(&self, _id: Uuid) -> erp_core::Result<Vec<ResourceSkill>> { Ok(vec![]) }
        async fn create(&self, s: ResourceSkill) -> erp_core::Result<ResourceSkill> { Ok(s) }
        async fn update(&self, s: ResourceSkill) -> erp_core::Result<ResourceSkill> { Ok(s) }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockResourceRequestRepository {
        requests: Mutex<Vec<ResourceRequest>>,
    }
    #[async_trait]
    impl ResourceRequestRepository for MockResourceRequestRepository {
        async fn find_by_id(&self, id: Uuid) -> erp_core::Result<ResourceRequest> {
            self.requests.lock().unwrap().iter().find(|r| r.base.id == id).cloned()
                .ok_or_else(|| Error::not_found("ResourceRequest", &id.to_string()))
        }
        async fn find_by_project(&self, id: Uuid) -> erp_core::Result<Vec<ResourceRequest>> {
            Ok(self.requests.lock().unwrap().iter().filter(|r| r.project_id == id).cloned().collect())
        }
        async fn create(&self, r: ResourceRequest) -> erp_core::Result<ResourceRequest> {
            self.requests.lock().unwrap().push(r.clone());
            Ok(r)
        }
        async fn update(&self, r: ResourceRequest) -> erp_core::Result<ResourceRequest> {
            let mut reqs = self.requests.lock().unwrap();
            if let Some(pos) = reqs.iter().position(|x| x.base.id == r.base.id) {
                reqs[pos] = r.clone();
            }
            Ok(r)
        }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockResourceAllocationRepository {
        allocations: Mutex<Vec<ResourceAllocation>>,
    }
    #[async_trait]
    impl ResourceAllocationRepository for MockResourceAllocationRepository {
        async fn find_by_id(&self, id: Uuid) -> erp_core::Result<ResourceAllocation> {
            self.allocations.lock().unwrap().iter().find(|a| a.base.id == id).cloned()
                .ok_or_else(|| Error::not_found("ResourceAllocation", &id.to_string()))
        }
        async fn find_by_project(&self, id: Uuid) -> erp_core::Result<Vec<ResourceAllocation>> {
            Ok(self.allocations.lock().unwrap().iter().filter(|a| a.project_id == id).cloned().collect())
        }
        async fn find_by_employee(&self, id: Uuid) -> erp_core::Result<Vec<ResourceAllocation>> {
            Ok(self.allocations.lock().unwrap().iter().filter(|a| a.employee_id == id).cloned().collect())
        }
        async fn create(&self, a: ResourceAllocation) -> erp_core::Result<ResourceAllocation> {
            self.allocations.lock().unwrap().push(a.clone());
            Ok(a)
        }
        async fn update(&self, a: ResourceAllocation) -> erp_core::Result<ResourceAllocation> {
            let mut allocs = self.allocations.lock().unwrap();
            if let Some(pos) = allocs.iter().position(|x| x.base.id == a.base.id) {
                allocs[pos] = a.clone();
            }
            Ok(a)
        }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    #[tokio::test]
    async fn test_resource_request_lifecycle() -> Result<()> {
        let skill_repo = MockSkillRepository;
        let rs_repo = MockResourceSkillRepository;
        let rq_repo = Arc::new(MockResourceRequestRepository { requests: Mutex::new(Vec::new()) });
        let a_repo = MockResourceAllocationRepository { allocations: Mutex::new(Vec::new()) };
        
        struct ArcRQ(Arc<MockResourceRequestRepository>);
        #[async_trait]
        impl ResourceRequestRepository for ArcRQ {
            async fn find_by_id(&self, id: Uuid) -> erp_core::Result<ResourceRequest> { self.0.find_by_id(id).await }
            async fn find_by_project(&self, id: Uuid) -> erp_core::Result<Vec<ResourceRequest>> { self.0.find_by_project(id).await }
            async fn create(&self, r: ResourceRequest) -> erp_core::Result<ResourceRequest> { self.0.create(r).await }
            async fn update(&self, r: ResourceRequest) -> erp_core::Result<ResourceRequest> { self.0.update(r).await }
            async fn delete(&self, id: Uuid) -> erp_core::Result<()> { self.0.delete(id).await }
        }

        let service = ResourceService::with_repos(
            skill_repo,
            rs_repo,
            ArcRQ(rq_repo.clone()),
            a_repo,
        );

        let project_id = Uuid::new_v4();
        let request = ResourceRequest {
            base: BaseEntity::new(),
            project_id,
            task_id: None,
            skill_id: Uuid::new_v4(),
            min_proficiency: 3,
            start_date: Utc::now(),
            end_date: Utc::now(),
            hours_required: 40.0,
            status: ResourceRequestStatus::Draft,
            requested_by: Uuid::new_v4(),
        };

        let created = service.create_request(request, None).await?;
        assert_eq!(created.status, ResourceRequestStatus::Draft);

        service.submit_request(created.base.id, None).await?;
        
        let req = rq_repo.find_by_id(created.base.id).await?;
        assert_eq!(req.status, ResourceRequestStatus::Pending);

        let allocation = ResourceAllocation {
            base: BaseEntity::new(),
            project_id,
            employee_id: Uuid::new_v4(),
            request_id: Some(created.base.id),
            start_date: Utc::now(),
            end_date: Utc::now(),
            allocation_percent: 100,
            billable_rate: Some(100),
        };

        let allocated = service.allocate_resource(allocation, None).await?;
        assert_eq!(allocated.project_id, project_id);

        let req = rq_repo.find_by_id(created.base.id).await?;
        assert_eq!(req.status, ResourceRequestStatus::Fulfilled);

        Ok(())
    }

    struct MockProjectRepository {
        projects: Mutex<Vec<Project>>,
    }
    #[async_trait]
    impl ProjectRepository for MockProjectRepository {
        async fn find_by_id(&self, id: Uuid) -> erp_core::Result<Project> {
            self.projects.lock().unwrap().iter().find(|p| p.base.id == id).cloned()
                .ok_or_else(|| Error::not_found("Project", &id.to_string()))
        }
        async fn find_by_number(&self, number: &str) -> erp_core::Result<Project> {
            self.projects.lock().unwrap().iter().find(|p| p.project_number == number).cloned()
                .ok_or_else(|| Error::not_found("Project", number))
        }
        async fn find_all(&self, p: Pagination) -> erp_core::Result<Paginated<Project>> {
            let projects = self.projects.lock().unwrap().clone();
            let total = projects.len() as u64;
            Ok(Paginated::new(projects, total, p))
        }
        async fn create(&self, p: Project) -> erp_core::Result<Project> {
            self.projects.lock().unwrap().push(p.clone());
            Ok(p)
        }
        async fn update(&self, p: Project) -> erp_core::Result<Project> {
            let mut projs = self.projects.lock().unwrap();
            if let Some(pos) = projs.iter().position(|x| x.base.id == p.base.id) {
                projs[pos] = p.clone();
            }
            Ok(p)
        }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockProjectTaskRepository;
    #[async_trait]
    impl ProjectTaskRepository for MockProjectTaskRepository {
        async fn find_by_id(&self, id: Uuid) -> erp_core::Result<ProjectTask> { Err(Error::not_found("Task", &id.to_string())) }
        async fn find_by_project(&self, _id: Uuid) -> erp_core::Result<Vec<ProjectTask>> { Ok(vec![]) }
        async fn create(&self, t: ProjectTask) -> erp_core::Result<ProjectTask> { Ok(t) }
        async fn update(&self, t: ProjectTask) -> erp_core::Result<ProjectTask> { Ok(t) }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockProjectMilestoneRepository;
    #[async_trait]
    impl ProjectMilestoneRepository for MockProjectMilestoneRepository {
        async fn find_by_id(&self, id: Uuid) -> erp_core::Result<ProjectMilestone> { Err(Error::not_found("Milestone", &id.to_string())) }
        async fn find_by_project(&self, _id: Uuid) -> erp_core::Result<Vec<ProjectMilestone>> { Ok(vec![]) }
        async fn create(&self, m: ProjectMilestone) -> erp_core::Result<ProjectMilestone> { Ok(m) }
        async fn update(&self, m: ProjectMilestone) -> erp_core::Result<ProjectMilestone> { Ok(m) }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockProjectExpenseRepository;
    #[async_trait]
    impl ProjectExpenseRepository for MockProjectExpenseRepository {
        async fn find_by_id(&self, id: Uuid) -> erp_core::Result<ProjectExpense> { Err(Error::not_found("Expense", &id.to_string())) }
        async fn find_by_project(&self, _id: Uuid) -> erp_core::Result<Vec<ProjectExpense>> { Ok(vec![]) }
        async fn create(&self, e: ProjectExpense) -> erp_core::Result<ProjectExpense> { Ok(e) }
        async fn update(&self, e: ProjectExpense) -> erp_core::Result<ProjectExpense> { Ok(e) }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    struct MockProjectTemplateRepository;
    #[async_trait]
    impl ProjectTemplateRepository for MockProjectTemplateRepository {
        async fn find_by_id(&self, id: Uuid) -> erp_core::Result<ProjectTemplate> {
            Ok(ProjectTemplate {
                base: BaseEntity::new_with_id(id),
                name: "Test Template".to_string(),
                description: None,
                project_type: ProjectType::Internal,
                billable: true,
                billing_method: BillingMethod::FixedPrice,
                tasks: vec![],
                milestones: vec![],
                status: erp_core::Status::Active,
                updated_at: Utc::now(),
            })
        }
        async fn find_all(&self, p: Pagination) -> erp_core::Result<Paginated<ProjectTemplate>> {
            Ok(Paginated::new(vec![], 0, p))
        }
        async fn create(&self, t: ProjectTemplate) -> erp_core::Result<ProjectTemplate> { Ok(t) }
        async fn update(&self, t: ProjectTemplate) -> erp_core::Result<ProjectTemplate> { Ok(t) }
        async fn delete(&self, _id: Uuid) -> erp_core::Result<()> { Ok(()) }
    }

    #[tokio::test]
    async fn test_create_project_from_template_logic() -> Result<()> {
        let service = ProjectService::with_repos(
            MockProjectRepository { projects: Mutex::new(Vec::new()) },
            MockProjectTaskRepository,
            MockProjectMilestoneRepository,
            MockProjectExpenseRepository,
            MockProjectTemplateRepository,
        );

        let project = service.create_project_from_template(
            Uuid::new_v4(),
            "New Project".to_string(),
            None,
            Utc::now(),
            None,
        ).await?;

        assert_eq!(project.name, "New Project");
        assert_eq!(project.status, ProjectStatus::Planning);
        
        Ok(())
    }
}
