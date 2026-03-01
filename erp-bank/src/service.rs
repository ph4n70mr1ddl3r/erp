use crate::models::*;
use crate::repository::{BankRepository, SqliteBankRepository};
use chrono::{NaiveDate, Utc};
use erp_core::{BaseEntity, Result};
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct BankService {
    repo: SqliteBankRepository,
}

impl BankService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { repo: SqliteBankRepository::new(pool) }
    }

    pub async fn create_connection(&self, pool: &SqlitePool, req: CreateConnectionRequest) -> Result<BankConnection> {
        let conn = BankConnection {
            base: BaseEntity::new(),
            connection_number: format!("BCN-{}", Uuid::new_v4()),
            bank_name: req.bank_name,
            bank_code: req.bank_code,
            swift_code: req.swift_code,
            api_endpoint: req.api_endpoint,
            api_key: req.api_key,
            api_secret: req.api_secret,
            certificate_path: req.certificate_path,
            authentication_type: req.authentication_type,
            statement_format: req.statement_format,
            polling_enabled: req.polling_enabled,
            polling_interval_minutes: req.polling_interval_minutes.unwrap_or(60),
            last_poll_at: None,
            last_successful_at: None,
            last_error: None,
            status: BankConnectionStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_connection(&conn).await
    }

    pub async fn create_bank_account(&self, pool: &SqlitePool, req: CreateBankAccountRequest) -> Result<BankAccount> {
        let masked = mask_account_number(&req.account_number);
        let account = BankAccount {
            base: BaseEntity::new(),
            connection_id: req.connection_id,
            account_number: req.account_number,
            masked_account_number: masked,
            account_name: req.account_name,
            account_type: req.account_type,
            currency: req.currency,
            gl_account_id: req.gl_account_id,
            company_id: req.company_id,
            bank_branch: req.bank_branch,
            iban: req.iban,
            routing_number: req.routing_number,
            auto_reconcile: req.auto_reconcile.unwrap_or(false),
            reconciliation_rules: req.reconciliation_rules,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_bank_account(&account).await
    }

    pub async fn import_statement(&self, pool: &SqlitePool, req: ImportStatementRequest) -> Result<BankStatement> {
        let stmt = BankStatement {
            base: BaseEntity::new(),
            statement_number: format!("STMT-{}", Uuid::new_v4()),
            bank_account_id: req.bank_account_id,
            statement_date: req.statement_date,
            currency: req.currency,
            opening_balance: req.opening_balance,
            closing_balance: req.closing_balance,
            total_credits: req.total_credits,
            total_debits: req.total_debits,
            credit_count: req.credit_count,
            debit_count: req.debit_count,
            statement_format: req.format,
            raw_file_path: req.file_path,
            imported_at: Utc::now(),
            imported_by: req.imported_by,
            status: StatementStatus::Imported,
            created_at: Utc::now(),
        };
        self.repo.create_statement(&stmt).await
    }

    pub async fn create_transaction(&self, pool: &SqlitePool, req: CreateTransactionRequest) -> Result<BankTransaction> {
        let tx = BankTransaction {
            base: BaseEntity::new(),
            statement_id: req.statement_id,
            bank_account_id: req.bank_account_id,
            transaction_date: req.transaction_date,
            value_date: req.value_date,
            transaction_type: req.transaction_type,
            amount: req.amount,
            currency: req.currency,
            reference_number: req.reference_number,
            bank_reference: req.bank_reference,
            customer_reference: req.customer_reference,
            description: req.description,
            payee_name: req.payee_name,
            payee_account: req.payee_account,
            check_number: req.check_number,
            additional_info: req.additional_info,
            reconciliation_status: ReconciliationStatus::Unmatched,
            matched_entity_type: None,
            matched_entity_id: None,
            matched_amount: None,
            match_confidence: None,
            match_rule: None,
            journal_entry_id: None,
            notes: None,
            created_at: Utc::now(),
        };
        self.repo.create_transaction(&tx).await
    }

    pub async fn reconcile(&self, pool: &SqlitePool, account_id: Uuid, period_start: NaiveDate, period_end: NaiveDate) -> Result<ReconciliationSession> {
        let session = ReconciliationSession {
            base: BaseEntity::new(),
            session_number: format!("REC-{}", Uuid::new_v4()),
            bank_account_id: account_id,
            period_start,
            period_end,
            total_transactions: 0,
            matched_count: 0,
            unmatched_count: 0,
            exception_count: 0,
            auto_matched_count: 0,
            manual_matched_count: 0,
            opening_balance: 0,
            closing_balance: 0,
            calculated_balance: 0,
            variance: 0,
            status: ReconciliationSessionStatus::InProgress,
            started_at: Utc::now(),
            completed_at: None,
            completed_by: None,
            created_at: Utc::now(),
        };
        self.repo.create_reconciliation_session(&session).await
    }

    pub async fn create_match(&self, pool: &SqlitePool, req: CreateMatchRequest) -> Result<ReconciliationMatch> {
        let match_rec = ReconciliationMatch {
            id: Uuid::new_v4(),
            session_id: req.session_id,
            bank_transaction_id: req.bank_transaction_id,
            entity_type: req.entity_type,
            entity_id: req.entity_id,
            entity_reference: req.entity_reference,
            transaction_amount: req.transaction_amount,
            entity_amount: req.entity_amount,
            match_difference: req.transaction_amount - req.entity_amount,
            match_type: req.match_type,
            match_rule: req.match_rule,
            match_confidence: req.match_confidence.unwrap_or(1.0),
            matched_at: Utc::now(),
            matched_by: req.matched_by,
            status: MatchStatus::Proposed,
            notes: req.notes,
            created_at: Utc::now(),
        };
        self.repo.create_match(&match_rec).await
    }

    pub async fn generate_payment_file(&self, pool: &SqlitePool, req: GeneratePaymentFileRequest) -> Result<PaymentFileGeneration> {
        let file = PaymentFileGeneration {
            base: BaseEntity::new(),
            file_number: format!("PAY-{}", Uuid::new_v4()),
            bank_account_id: req.bank_account_id,
            file_type: req.file_type,
            file_date: Utc::now().date_naive(),
            value_date: req.value_date,
            currency: req.currency,
            total_amount: req.total_amount,
            payment_count: req.payment_count,
            file_content: req.file_content,
            file_path: None,
            status: PaymentFileStatus::Generated,
            generated_at: Utc::now(),
            transmitted_at: None,
            acknowledged_at: None,
            created_at: Utc::now(),
        };
        self.repo.create_payment_file(&file).await
    }

    pub async fn parse_bai2(&self, pool: &SqlitePool, content: &str) -> Result<Vec<BankTransaction>> {
        let mut transactions = Vec::new();
        for line in content.lines() {
            if line.starts_with("16") {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 5 {
                    let tx = BankTransaction {
                        base: BaseEntity::new(),
                        statement_id: Uuid::nil(),
                        bank_account_id: Uuid::nil(),
                        transaction_date: chrono::Utc::now().date_naive(),
                        value_date: None,
                        transaction_type: TransactionType::Credit,
                        amount: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0),
                        currency: "USD".to_string(),
                        reference_number: parts.get(3).map(|s| s.to_string()),
                        bank_reference: None,
                        customer_reference: None,
                        description: parts.get(4).unwrap_or(&"").to_string(),
                        payee_name: None,
                        payee_account: None,
                        check_number: None,
                        additional_info: None,
                        reconciliation_status: ReconciliationStatus::Unmatched,
                        matched_entity_type: None,
                        matched_entity_id: None,
                        matched_amount: None,
                        match_confidence: None,
                        match_rule: None,
                        journal_entry_id: None,
                        notes: None,
                        created_at: Utc::now(),
                    };
                    transactions.push(tx);
                }
            }
        }
        Ok(transactions)
    }
}

fn mask_account_number(account: &str) -> String {
    if account.len() > 4 {
        format!("****{}", &account[account.len()-4..])
    } else {
        "****".to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateConnectionRequest {
    pub bank_name: String,
    pub bank_code: Option<String>,
    pub swift_code: Option<String>,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub certificate_path: Option<String>,
    pub authentication_type: AuthenticationType,
    pub statement_format: StatementFormat,
    pub polling_enabled: bool,
    pub polling_interval_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBankAccountRequest {
    pub connection_id: Uuid,
    pub account_number: String,
    pub account_name: String,
    pub account_type: BankAccountType,
    pub currency: String,
    pub gl_account_id: Option<Uuid>,
    pub company_id: Uuid,
    pub bank_branch: Option<String>,
    pub iban: Option<String>,
    pub routing_number: Option<String>,
    pub auto_reconcile: Option<bool>,
    pub reconciliation_rules: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImportStatementRequest {
    pub bank_account_id: Uuid,
    pub statement_date: NaiveDate,
    pub currency: String,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub total_credits: i64,
    pub total_debits: i64,
    pub credit_count: i32,
    pub debit_count: i32,
    pub format: StatementFormat,
    pub file_path: Option<String>,
    pub imported_by: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub statement_id: Uuid,
    pub bank_account_id: Uuid,
    pub transaction_date: NaiveDate,
    pub value_date: Option<NaiveDate>,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub currency: String,
    pub reference_number: Option<String>,
    pub bank_reference: Option<String>,
    pub customer_reference: Option<String>,
    pub description: String,
    pub payee_name: Option<String>,
    pub payee_account: Option<String>,
    pub check_number: Option<String>,
    pub additional_info: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMatchRequest {
    pub session_id: Uuid,
    pub bank_transaction_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub entity_reference: String,
    pub transaction_amount: i64,
    pub entity_amount: i64,
    pub match_type: MatchType,
    pub match_rule: Option<String>,
    pub match_confidence: Option<f64>,
    pub matched_by: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GeneratePaymentFileRequest {
    pub bank_account_id: Uuid,
    pub file_type: String,
    pub value_date: NaiveDate,
    pub currency: String,
    pub total_amount: i64,
    pub payment_count: i32,
    pub file_content: Option<String>,
}
