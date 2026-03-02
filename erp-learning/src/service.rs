use crate::repository::SqliteLearningRepository;

#[allow(dead_code)]
pub struct LearningService {
    repo: SqliteLearningRepository,
}

impl LearningService {
    pub fn new(repo: SqliteLearningRepository) -> Self {
        Self { repo }
    }
}
