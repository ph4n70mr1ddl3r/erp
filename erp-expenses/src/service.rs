use crate::models::*;
use crate::repository::SqliteExpensesRepository;

pub struct ExpensesService {
    repo: SqliteExpensesRepository,
}

impl ExpensesService {
    pub fn new(repo: SqliteExpensesRepository) -> Self {
        Self { repo }
    }
}
