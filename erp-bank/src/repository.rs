use crate::models::*;
use async_trait::async_trait;
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
        sqlx::query(
            r#"INSERT INTO bank_connections (id, connection_number, bank_name, bank_code, swift_code,
               api_endpoint, api_key, api_secret, certificate_path, authentication_type,
               statement_format, polling_enabled, polling_interval_minutes, last_poll_at,
               last_successful_at, last_error, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(conn.base.id.to_string())
        .bind(&conn.connection_number)
        .bind(&conn.bank_name)
        .bind(&conn.bank_code)
        .bind(&conn.swift_code)
        .bind(&conn.api_endpoint)
        .bind(&conn.api_key)
        .bind(&conn.api_secret)
        .bind(&conn.certificate_path)
        .bind(conn.authentication_type.clone())
        .bind(conn.statement_format.clone())
        .bind(conn.polling_enabled)
        .bind(conn.polling_interval_minutes)
        .bind(conn.last_poll_at)
        .bind(conn.last_successful_at)
        .bind(&conn.last_error)
        .bind(conn.status.clone())
        .bind(conn.created_at)
        .bind(conn.updated_at)
        .execute(&self.pool).await?;
        Ok(conn.clone())
    }

    async fn get_connection(&self, _id: Uuid) -> Result<Option<BankConnection>> {
        Ok(None)
    }

    async fn list_connections(&self) -> Result<Vec<BankConnection>> {
        Ok(vec![])
    }

    async fn create_bank_account(&self, account: &BankAccount) -> Result<BankAccount> {
        sqlx::query(
            r#"INSERT INTO bank_accounts (id, connection_id, account_number, masked_account_number,
               account_name, account_type, currency, gl_account_id, company_id, bank_branch,
               iban, routing_number, auto_reconcile, reconciliation_rules, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(account.base.id.to_string())
        .bind(account.connection_id.to_string())
        .bind(&account.account_number)
        .bind(&account.masked_account_number)
        .bind(&account.account_name)
        .bind(account.account_type.clone())
        .bind(&account.currency)
        .bind(account.gl_account_id.map(|id| id.to_string()))
        .bind(account.company_id.to_string())
        .bind(&account.bank_branch)
        .bind(&account.iban)
        .bind(&account.routing_number)
        .bind(account.auto_reconcile)
        .bind(&account.reconciliation_rules)
        .bind(account.status.clone())
        .bind(account.created_at)
        .bind(account.updated_at)
        .execute(&self.pool).await?;
        Ok(account.clone())
    }

    async fn get_bank_account(&self, _id: Uuid) -> Result<Option<BankAccount>> {
        Ok(None)
    }

    async fn list_bank_accounts(&self, _connection_id: Option<Uuid>) -> Result<Vec<BankAccount>> {
        Ok(vec![])
    }

    async fn create_statement(&self, stmt: &BankStatement) -> Result<BankStatement> {
        sqlx::query(
            r#"INSERT INTO bank_statements (id, statement_number, bank_account_id, statement_date,
               currency, opening_balance, closing_balance, total_credits, total_debits,
               credit_count, debit_count, statement_format, raw_file_path, imported_at,
               imported_by, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(stmt.base.id.to_string())
        .bind(&stmt.statement_number)
        .bind(stmt.bank_account_id.to_string())
        .bind(stmt.statement_date)
        .bind(&stmt.currency)
        .bind(stmt.opening_balance)
        .bind(stmt.closing_balance)
        .bind(stmt.total_credits)
        .bind(stmt.total_debits)
        .bind(stmt.credit_count)
        .bind(stmt.debit_count)
        .bind(stmt.statement_format.clone())
        .bind(&stmt.raw_file_path)
        .bind(stmt.imported_at)
        .bind(stmt.imported_by.map(|id| id.to_string()))
        .bind(stmt.status.clone())
        .bind(stmt.created_at)
        .execute(&self.pool).await?;
        Ok(stmt.clone())
    }

    async fn get_statement(&self, _id: Uuid) -> Result<Option<BankStatement>> {
        Ok(None)
    }

    async fn list_statements(&self, _account_id: Uuid) -> Result<Vec<BankStatement>> {
        Ok(vec![])
    }

    async fn create_transaction(&self, tx: &BankTransaction) -> Result<BankTransaction> {
        sqlx::query(
            r#"INSERT INTO bank_transactions (id, statement_id, bank_account_id, transaction_date,
               value_date, transaction_type, amount, currency, reference_number, bank_reference,
               customer_reference, description, payee_name, payee_account, check_number,
               additional_info, reconciliation_status, matched_entity_type, matched_entity_id,
               matched_amount, match_confidence, match_rule, journal_entry_id, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(tx.base.id.to_string())
        .bind(tx.statement_id.to_string())
        .bind(tx.bank_account_id.to_string())
        .bind(tx.transaction_date)
        .bind(tx.value_date)
        .bind(tx.transaction_type.clone())
        .bind(tx.amount)
        .bind(&tx.currency)
        .bind(&tx.reference_number)
        .bind(&tx.bank_reference)
        .bind(&tx.customer_reference)
        .bind(&tx.description)
        .bind(&tx.payee_name)
        .bind(&tx.payee_account)
        .bind(&tx.check_number)
        .bind(&tx.additional_info)
        .bind(tx.reconciliation_status.clone())
        .bind(&tx.matched_entity_type)
        .bind(tx.matched_entity_id.map(|id| id.to_string()))
        .bind(tx.matched_amount)
        .bind(tx.match_confidence)
        .bind(&tx.match_rule)
        .bind(tx.journal_entry_id.map(|id| id.to_string()))
        .bind(&tx.notes)
        .bind(tx.created_at)
        .execute(&self.pool).await?;
        Ok(tx.clone())
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
        sqlx::query(
            r#"INSERT INTO reconciliation_rules (id, rule_name, description, bank_account_id,
               match_criteria, tolerance_type, tolerance_value, date_tolerance_days,
               reference_patterns, auto_match, priority, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(rule.base.id.to_string())
        .bind(&rule.rule_name)
        .bind(&rule.description)
        .bind(rule.bank_account_id.map(|id| id.to_string()))
        .bind(&rule.match_criteria)
        .bind(rule.tolerance_type.clone())
        .bind(rule.tolerance_value)
        .bind(rule.date_tolerance_days)
        .bind(&rule.reference_patterns)
        .bind(rule.auto_match)
        .bind(rule.priority)
        .bind(rule.status.clone())
        .bind(rule.created_at)
        .bind(rule.updated_at)
        .execute(&self.pool).await?;
        Ok(rule.clone())
    }

    async fn list_reconciliation_rules(&self) -> Result<Vec<ReconciliationRule>> {
        Ok(vec![])
    }

    async fn create_reconciliation_session(&self, session: &ReconciliationSession) -> Result<ReconciliationSession> {
        sqlx::query(
            r#"INSERT INTO reconciliation_sessions (id, session_number, bank_account_id,
               period_start, period_end, total_transactions, matched_count, unmatched_count,
               exception_count, auto_matched_count, manual_matched_count, opening_balance,
               closing_balance, calculated_balance, variance, status, started_at,
               completed_at, completed_by, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(session.base.id.to_string())
        .bind(&session.session_number)
        .bind(session.bank_account_id.to_string())
        .bind(session.period_start)
        .bind(session.period_end)
        .bind(session.total_transactions)
        .bind(session.matched_count)
        .bind(session.unmatched_count)
        .bind(session.exception_count)
        .bind(session.auto_matched_count)
        .bind(session.manual_matched_count)
        .bind(session.opening_balance)
        .bind(session.closing_balance)
        .bind(session.calculated_balance)
        .bind(session.variance)
        .bind(session.status.clone())
        .bind(session.started_at)
        .bind(session.completed_at)
        .bind(session.completed_by.map(|id| id.to_string()))
        .bind(session.created_at)
        .execute(&self.pool).await?;
        Ok(session.clone())
    }

    async fn create_match(&self, match_rec: &ReconciliationMatch) -> Result<ReconciliationMatch> {
        sqlx::query(
            r#"INSERT INTO reconciliation_matches (id, session_id, bank_transaction_id,
               entity_type, entity_id, entity_reference, transaction_amount, entity_amount,
               match_difference, match_type, match_rule, match_confidence, matched_at,
               matched_by, status, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(match_rec.id.to_string())
        .bind(match_rec.session_id.to_string())
        .bind(match_rec.bank_transaction_id.to_string())
        .bind(&match_rec.entity_type)
        .bind(match_rec.entity_id.to_string())
        .bind(&match_rec.entity_reference)
        .bind(match_rec.transaction_amount)
        .bind(match_rec.entity_amount)
        .bind(match_rec.match_difference)
        .bind(match_rec.match_type.clone())
        .bind(&match_rec.match_rule)
        .bind(match_rec.match_confidence)
        .bind(match_rec.matched_at)
        .bind(match_rec.matched_by.map(|id| id.to_string()))
        .bind(match_rec.status.clone())
        .bind(&match_rec.notes)
        .bind(match_rec.created_at)
        .execute(&self.pool).await?;
        Ok(match_rec.clone())
    }

    async fn create_payment_file(&self, file: &PaymentFileGeneration) -> Result<PaymentFileGeneration> {
        sqlx::query(
            r#"INSERT INTO payment_file_generations (id, file_number, bank_account_id, file_type,
               file_date, value_date, currency, total_amount, payment_count, file_content,
               file_path, status, generated_at, transmitted_at, acknowledged_at, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(file.base.id.to_string())
        .bind(&file.file_number)
        .bind(file.bank_account_id.to_string())
        .bind(&file.file_type)
        .bind(file.file_date)
        .bind(file.value_date)
        .bind(&file.currency)
        .bind(file.total_amount)
        .bind(file.payment_count)
        .bind(&file.file_content)
        .bind(&file.file_path)
        .bind(file.status.clone())
        .bind(file.generated_at)
        .bind(file.transmitted_at)
        .bind(file.acknowledged_at)
        .bind(file.created_at)
        .execute(&self.pool).await?;
        Ok(file.clone())
    }

    async fn create_fee(&self, fee: &BankFee) -> Result<BankFee> {
        sqlx::query(
            r#"INSERT INTO bank_fees (id, bank_account_id, fee_date, fee_type, description,
               amount, currency, transaction_id, gl_account_id, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(fee.base.id.to_string())
        .bind(fee.bank_account_id.to_string())
        .bind(fee.fee_date)
        .bind(fee.fee_type.clone())
        .bind(&fee.description)
        .bind(fee.amount)
        .bind(&fee.currency)
        .bind(fee.transaction_id.map(|id| id.to_string()))
        .bind(fee.gl_account_id.map(|id| id.to_string()))
        .bind(fee.status.clone())
        .bind(fee.created_at)
        .execute(&self.pool).await?;
        Ok(fee.clone())
    }
}
