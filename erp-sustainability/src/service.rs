use crate::repository::SqliteSustainabilityRepository;

#[allow(dead_code)]
pub struct SustainabilityService {
    repo: SqliteSustainabilityRepository,
}

impl SustainabilityService {
    pub fn new(repo: SqliteSustainabilityRepository) -> Self {
        Self { repo }
    }
}
