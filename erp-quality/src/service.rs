use crate::repository::SqliteQualityRepository;

pub struct QualityService {
    repo: SqliteQualityRepository,
}

impl QualityService {
    pub fn new(repo: SqliteQualityRepository) -> Self {
        Self { repo }
    }
}
