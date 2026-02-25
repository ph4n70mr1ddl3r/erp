use crate::models::*;
use async_trait::async_trait;
use chrono::NaiveDate;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait BankRepository: Send + Sync {
    async fn create_connection(&self, conn: &BankConnection) -> Result<BankConnection>;
    async fn get_connection(&self, id: Uuid) -> Result<Option<BankConnection>>;
    async fn list_connections(&self) -> Result<Vec<BankConnection>>;
    async fn create_bank_account(&self, account: &BankAccount) -> Result<BankAccount>;
    async fn get_bank_account(&self, id: Uuid) -> Result<Option<BankAccount>>;
    async fn list_bank_accounts(&self, connection_id: Option<Uuid>) -> Result<Vec<BankAccount>>;
    async fn create_statement(&self, stmt: &BankStatement) -> Result<BankStatement>;
    async fn get_statement(&self, id: Uuid) -> Result<Option<BankStatement>>;
    async fn list_statements(&self, account_id: Uuid) -> Result<Vec<BankStatement>>;
    async fn create_transaction(&self, tx: &BankTransaction) -> Result<BankTransaction>;
    async fn list_transactions(&self, statement_id: Uuid) -> Result<Vec<BankTransaction>>;
    async fn list_unreconciled(&self, account_id: Uuid) -> Result<Vec<BankTransaction>>;
    async fn update_transaction(&self, tx: &BankTransaction) -> Result<BankTransaction>;
    async fn create_reconciliation_rule(&self, rule: &ReconciliationRule) -> Result<ReconciliationRule>;
    async fn list_reconciliation_rules(&self) -> Result<Vec<ReconciliationRule>>;
    async fn create_reconciliation_session(&self, session: &ReconciliationSession) -> Result<ReconciliationSession>;
    async fn create_match(&self, match_rec: &ReconciliationMatch) -> Result<ReconciliationMatch>;
    async fn create_payment_file(&self, file: &PaymentFileGeneration) -> Result<PaymentFileGeneration>;
    async fn create_fee(&self, fee: &BankFee) -> Result<BankFee>;
}

pub struct SqliteBankRepository {
    pool: SqlitePool,
}

impl SqliteBankRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BankRepository for SqliteBankRepository {
    async fn create_connection(&self, conn: &BankConnection) -> Result<BankConnection> {
        let conn = conn.clone();
        sqlx::query!(
            r#"INSERT INTO bank_connections (id, connection_number, bank_name, bank_code, swift_code,
               api_endpoint, api_key, api_secret, certificate_path, authentication_type,
               statement_format, polling_enabled, polling_interval_minutes, last_poll_at,
               last_successful_at, last_error, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            conn.base.id, conn.connection_number, conn.bank_name, conn.bank_code, conn.swift_code,
            conn.api_endpoint, conn.api_key, conn.api_secret, conn.certificate_path, conn.authentication_type,
            conn.statement_format, conn.polling_enabled, conn.polling_interval_minutes, conn.last_poll_at,
            conn.last_successful_at, conn.last_error, conn.status, conn.created_at, conn.updated_at
        ).execute(&self.pool).await?;
        Ok(conn)
    }

    async fn get_connection(&self, id: Uuid) -> Result<Option<BankConnection>> {
        Ok(None)
    }

    async fn list_connections(&self) -> Result<Vec<BankConnection>> {
        Ok(vec![])
    }

    async fn create_bank_account(&self, account: &BankAccount) -> Result<BankAccount> {
        let account = account.clone();
        sqlx::query!(
            r#"INSERT INTO bank_accounts (id, connection_id, account_number, masked_account_number,
               account_name, account_type, currency, gl_account_id, company_id, bank_branch,
               iban, routing_number, auto_reconcile, reconciliation_rules, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            account.base.id, account.connection_id, account.account_number, account.masked_account_number,
            account.account_name, account.account_type, account.currency, account.gl_account_id,
            account.company_id, account.bank_branch, account.iban, account.routing_number,
            account.auto_reconcile, account.reconciliation_rules, account.status,
            account.created_at, account.updated_at
        ).execute(&self.pool).await?;
        Ok(account)
    }

    async fn get_bank_account(&self, id: Uuid) -> Result<Option<BankAccount>> {
        Ok(None)
    }

    async fn list_bank_accounts(&self, _connection_id: Option<Uuid>) -> Result<Vec<BankAccount>> {
        Ok(vec![])
    }

    async fn create_statement(&self, stmt: &BankStatement) -> Result<BankStatement> {
        let stmt = stmt.clone();
        sqlx::query!(
            r#"INSERT INTO bank_statements (id, statement_number, bank_account_id, statement_date,
               currency, opening_balance, closing_balance, total_credits, total_debits,
               credit_count, debit_count, statement_format, raw_file_path, imported_at,
               imported_by, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            stmt.base.id, stmt.statement_number, stmt.bank_account_id, stmt.statement_date,
            stmt.currency, stmt.opening_balance, stmt.closing_balance, stmt.total_credits,
            stmt.total_debits, stmt.credit_count, stmt.debit_count, stmt.statement_format,
            stmt.raw_file_path, stmt.imported_at, stmt.imported_by, stmt.status, stmt.created_at
        ).execute(&self.pool).await?;
        Ok(stmt)
    }

    async fn get_statement(&self, id: Uuid) -> Result<Option<BankStatement>> {
        Ok(None)
    }

    async fn list_statements(&self, _account_id: Uuid) -> Result<Vec<BankStatement>> {
        Ok(vec![])
    }

    async fn create_transaction(&self, tx: &BankTransaction) -> Result<BankTransaction> {
        let tx = tx.clone();
        sqlx::query!(
            r#"INSERT INTO bank_transactions (id, statement_id, bank_account_id, transaction_date,
               value_date, transaction_type, amount, currency, reference_number, bank_reference,
               customer_reference, description, payee_name, payee_account, check_number,
               additional_info, reconciliation_status, matched_entity_type, matched_entity_id,
               matched_amount, match_confidence, match_rule, journal_entry_id, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            tx.base.id, tx.statement_id, tx.bank_account_id, tx.transaction_date, tx.value_date,
            tx.transaction_type, tx.amount, tx.currency, tx.reference_number, tx.bank_reference,
            tx.customer_reference, tx.description, tx.payee_name, tx.payee_account, tx.check_number,
            tx.additional_info, tx.reconciliation_status, tx.matched_entity_type, tx.matched_entity_id,
            tx.matched_amount, tx.match_confidence, tx.match_rule, tx.journal_entry_id, tx.notes, tx.created_at
        ).execute(&self.pool).await?;
        Ok(tx)
    }

    async fn list_transactions(&self, _statement_id: Uuid) -> Result<Vec<BankTransaction>> {
        Ok(vec![])
    }

    async fn list_unreconciled(&self, _account_id: Uuid) -> Result<Vec<BankTransaction>> {
        Ok(vec![])
    }

    async fn update_transaction(&self, tx: &BankTransaction) -> Result<BankTransaction> {
        Ok(tx.clone())
    }

    async fn create_reconciliation_rule(&self, rule: &ReconciliationRule) -> Result<ReconciliationRule> {
        let rule = rule.clone();
        sqlx::query!(
            r#"INSERT INTO reconciliation_rules (id, rule_name, description, bank_account_id,
               match_criteria, tolerance_type, tolerance_value, date_tolerance_days,
               reference_patterns, auto_match, priority, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.base.id, rule.rule_name, rule.description, rule.bank_account_id,
            rule.match_criteria, rule.tolerance_type, rule.tolerance_value, rule.date_tolerance_days,
            rule.reference_patterns, rule.auto_match, rule.priority, rule.status,
            rule.created_at, rule.updated_at
        ).execute(&self.pool).await?;
        Ok(rule)
    }

    async fn list_reconciliation_rules(&self) -> Result<Vec<ReconciliationRule>> {
        Ok(vec![])
    }

    async fn create_reconciliation_session(&self, session: &ReconciliationSession) -> Result<ReconciliationSession> {
        let session = session.clone();
        sqlx::query!(
            r#"INSERT INTO reconciliation_sessions (id, session_number, bank_account_id,
               period_start, period_end, total_transactions, matched_count, unmatched_count,
               exception_count, auto_matched_count, manual_matched_count, opening_balance,
               closing_balance, calculated_balance, variance, status, started_at,
               completed_at, completed_by, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            session.base.id, session.session_number, session.bank_account_id,
            session.period_start, session.period_end, session.total_transactions,
            session.matched_count, session.unmatched_count, session.exception_count,
            session.auto_matched_count, session.manual_matched_count, session.opening_balance,
            session.closing_balance, session.calculated_balance, session.variance, session.status,
            session.started_at, session.completed_at, session.completed_by, session.created_at
        ).execute(&self.pool).await?;
        Ok(session)
    }

    async fn create_match(&self, match_rec: &ReconciliationMatch) -> Result<ReconciliationMatch> {
        let match_rec = match_rec.clone();
        sqlx::query!(
            r#"INSERT INTO reconciliation_matches (id, session_id, bank_transaction_id,
               entity_type, entity_id, entity_reference, transaction_amount, entity_amount,
               match_difference, match_type, match_rule, match_confidence, matched_at,
               matched_by, status, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            match_rec.id, match_rec.session_id, match_rec.bank_transaction_id,
            match_rec.entity_type, match_rec.entity_id, match_rec.entity_reference,
            match_rec.transaction_amount, match_rec.entity_amount, match_rec.match_difference,
            match_rec.match_type, match_rec.match_rule, match_rec.match_confidence,
            match_rec.matched_at, match_rec.matched_by, match_rec.status, match_rec.notes, match_rec.created_at
        ).execute(&self.pool).await?;
        Ok(match_rec)
    }

    async fn create_payment_file(&self, file: &PaymentFileGeneration) -> Result<PaymentFileGeneration> {
        let file = file.clone();
        sqlx::query!(
            r#"INSERT INTO payment_file_generations (id, file_number, bank_account_id, file_type,
               file_date, value_date, currency, total_amount, payment_count, file_content,
               file_path, status, generated_at, transmitted_at, acknowledged_at, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            file.base.id, file.file_number, file.bank_account_id, file.file_type,
            file.file_date, file.value_date, file.currency, file.total_amount, file.payment_count,
            file.file_content, file.file_path, file.status, file.generated_at,
            file.transmitted_at, file.acknowledged_at, file.created_at
        ).execute(&self.pool).await?;
        Ok(file)
    }

    async fn create_fee(&self, fee: &BankFee) -> Result<BankFee> {
        let fee = fee.clone();
        sqlx::query!(
            r#"INSERT INTO bank_fees (id, bank_account_id, fee_date, fee_type, description,
               amount, currency, transaction_id, gl_account_id, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            fee.base.id, fee.bank_account_id, fee.fee_date, fee.fee_type, fee.description,
            fee.amount, fee.currency, fee.transaction_id, fee.gl_account_id, fee.status, fee.created_at
        ).execute(&self.pool).await?;
        Ok(fee)
    }
}
