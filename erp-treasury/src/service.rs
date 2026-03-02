use crate::repository::SqliteTreasuryRepository;

#[allow(dead_code)]
pub struct TreasuryService {
    repo: SqliteTreasuryRepository,
}

impl TreasuryService {
    pub fn new(repo: SqliteTreasuryRepository) -> Self {
        Self { repo }
    }
}
