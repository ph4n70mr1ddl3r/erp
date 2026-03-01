use crate::repository::SqliteLearningRepository;

pub struct LearningService {
    repo: SqliteLearningRepository,
}

impl LearningService {
    pub fn new(repo: SqliteLearningRepository) -> Self {
        Self { repo }
    }
}
