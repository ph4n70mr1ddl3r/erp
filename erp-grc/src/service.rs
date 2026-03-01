use crate::repository::SqliteGRCRepository;

pub struct GRCService {
    repo: SqliteGRCRepository,
}

impl GRCService {
    pub fn new(repo: SqliteGRCRepository) -> Self {
        Self { repo }
    }
}
