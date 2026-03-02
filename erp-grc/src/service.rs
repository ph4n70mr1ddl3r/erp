use crate::repository::SqliteGRCRepository;

#[allow(dead_code)]
pub struct GRCService {
    repo: SqliteGRCRepository,
}

impl GRCService {
    pub fn new(repo: SqliteGRCRepository) -> Self {
        Self { repo }
    }
}
