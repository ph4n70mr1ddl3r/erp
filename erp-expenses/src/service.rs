use crate::repository::SqliteExpensesRepository;

#[allow(dead_code)]
pub struct ExpensesService {
    repo: SqliteExpensesRepository,
}

impl ExpensesService {
    pub fn new(repo: SqliteExpensesRepository) -> Self {
        Self { repo }
    }
}
