use crate::repository::SqlitePlanningRepository;

pub struct PlanningService {
    repo: SqlitePlanningRepository,
}

impl PlanningService {
    pub fn new(repo: SqlitePlanningRepository) -> Self {
        Self { repo }
    }
}
