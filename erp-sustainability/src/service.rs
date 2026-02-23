use crate::models::*;
use crate::repository::SqliteSustainabilityRepository;

pub struct SustainabilityService {
    repo: SqliteSustainabilityRepository,
}

impl SustainabilityService {
    pub fn new(repo: SqliteSustainabilityRepository) -> Self {
        Self { repo }
    }
}
