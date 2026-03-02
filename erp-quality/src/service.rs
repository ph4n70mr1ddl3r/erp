use crate::repository::SqliteQualityRepository;

#[allow(dead_code)]
pub struct QualityService {
    repo: SqliteQualityRepository,
}

impl QualityService {
    pub fn new(repo: SqliteQualityRepository) -> Self {
        Self { repo }
    }
}
