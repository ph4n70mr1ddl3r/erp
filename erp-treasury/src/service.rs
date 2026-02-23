use crate::models::*;
use crate::repository::SqliteTreasuryRepository;

pub struct TreasuryService {
    repo: SqliteTreasuryRepository,
}

impl TreasuryService {
    pub fn new(repo: SqliteTreasuryRepository) -> Self {
        Self { repo }
    }
}
