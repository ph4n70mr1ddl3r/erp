use crate::repository::SqlitePlanningRepository;

#[allow(dead_code)]
pub struct PlanningService {
    repo: SqlitePlanningRepository,
}

impl PlanningService {
    pub fn new(repo: SqlitePlanningRepository) -> Self {
        Self { repo }
    }
}
