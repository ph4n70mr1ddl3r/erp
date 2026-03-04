use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::Pagination;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/accounts", post(create_bank_account).get(list_bank_accounts))
        .route("/accounts/:id", get(get_bank_account))
        .route("/statements", post(import_statement).get(list_statements))
        .route("/statements/:id", get(get_statement))
        .route("/statements/:id/transactions", get(list_statement_transactions))
        .route("/transactions", get(list_transactions))
        .route("/transactions/:id/reconcile", post(reconcile_transaction))
        .route("/transactions/:id/unreconcile", post(unreconcile_transaction))
        .route("/reconciliations", post(start_reconciliation).get(list_reconciliations))
        .route("/reconciliations/:id", get(get_reconciliation))
        .route("/reconciliations/:id/complete", post(complete_reconciliation))
        .route("/reconciliations/:id/matches", get(list_reconciliation_matches))
        .route("/reconciliations/:id/auto-match", post(auto_match_transactions))
        .route("/rules", post(create_reconciliation_rule).get(list_reconciliation_rules))
        .route("/rules/:id", get(get_reconciliation_rule).put(update_reconciliation_rule))
        .route("/unreconciled", get(list_unreconciled_transactions))
        .route("/summary/:account_id", get(get_reconciliation_summary))
}

#[derive(Debug, Serialize)]
pub struct BankAccountResponse {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub account_number: String,
    pub masked_account_number: String,
    pub account_name: String,
    pub account_type: String,
    pub currency: String,
    pub gl_account_id: Option<Uuid>,
    pub company_id: Uuid,
    pub bank_branch: Option<String>,
    pub iban: Option<String>,
    pub routing_number: Option<String>,
    pub auto_reconcile: bool,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBankAccountRequest {
    pub connection_id: Uuid,
    pub account_number: String,
    pub account_name: String,
    pub account_type: String,
    pub currency: String,
    pub gl_account_id: Option<Uuid>,
    pub company_id: Uuid,
    pub bank_branch: Option<String>,
    pub iban: Option<String>,
    pub routing_number: Option<String>,
}

pub async fn create_bank_account(
    State(state): State<AppState>,
    Json(req): Json<CreateBankAccountRequest>,
) -> ApiResult<Json<BankAccountResponse>> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let masked = mask_account_number(&req.account_number);
    
    sqlx::query(
        "INSERT INTO bank_accounts (id, connection_id, account_number, masked_account_number, 
         account_name, account_type, currency, gl_account_id, company_id, bank_branch, iban, 
         routing_number, auto_reconcile, status, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 'Active', ?, ?)"
    )
    .bind(id.to_string())
    .bind(req.connection_id.to_string())
    .bind(&req.account_number)
    .bind(&masked)
    .bind(&req.account_name)
    .bind(&req.account_type)
    .bind(&req.currency)
    .bind(req.gl_account_id.map(|id| id.to_string()))
    .bind(req.company_id.to_string())
    .bind(&req.bank_branch)
    .bind(&req.iban)
    .bind(&req.routing_number)
    .bind(now.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(&state.pool)
    .await?;

    Ok(Json(BankAccountResponse {
        id,
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
        auto_reconcile: false,
        status: "Active".to_string(),
        created_at: now.to_rfc3339(),
        updated_at: now.to_rfc3339(),
    }))
}

pub async fn list_bank_accounts(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<Vec<BankAccountResponse>>> {
    let rows = sqlx::query_as::<_, BankAccountRow>(
        "SELECT id, connection_id, account_number, masked_account_number, account_name, 
         account_type, currency, gl_account_id, company_id, bank_branch, iban, routing_number, 
         auto_reconcile, status, created_at, updated_at
         FROM bank_accounts WHERE status = 'Active'
         ORDER BY account_name
         LIMIT ? OFFSET ?"
    )
    .bind(pagination.limit() as i64)
    .bind(pagination.offset() as i64)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

pub async fn get_bank_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<BankAccountResponse>> {
    let row = sqlx::query_as::<_, BankAccountRow>(
        "SELECT id, connection_id, account_number, masked_account_number, account_name, 
         account_type, currency, gl_account_id, company_id, bank_branch, iban, routing_number, 
         auto_reconcile, status, created_at, updated_at
         FROM bank_accounts WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| erp_core::Error::not_found("BankAccount", &id.to_string()))?;

    Ok(Json(row.into()))
}

#[derive(Debug, Serialize)]
pub struct BankStatementResponse {
    pub id: Uuid,
    pub statement_number: String,
    pub bank_account_id: Uuid,
    pub statement_date: String,
    pub currency: String,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub total_credits: i64,
    pub total_debits: i64,
    pub credit_count: i32,
    pub debit_count: i32,
    pub status: String,
    pub imported_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportStatementRequest {
    pub bank_account_id: Uuid,
    pub statement_date: String,
    pub currency: String,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub transactions: Vec<TransactionImport>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionImport {
    pub transaction_date: String,
    pub value_date: Option<String>,
    pub transaction_type: String,
    pub amount: i64,
    pub reference_number: Option<String>,
    pub description: String,
    pub payee_name: Option<String>,
}

pub async fn import_statement(
    State(state): State<AppState>,
    Json(req): Json<ImportStatementRequest>,
) -> ApiResult<Json<BankStatementResponse>> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let statement_number = format!("BS-{}", now.format("%Y%m%d%H%M%S"));
    
    let total_credits: i64 = req.transactions.iter().filter(|t| t.amount > 0).map(|t| t.amount).sum();
    let total_debits: i64 = req.transactions.iter().filter(|t| t.amount < 0).map(|t| t.amount.abs()).sum();
    let credit_count = req.transactions.iter().filter(|t| t.amount > 0).count() as i32;
    let debit_count = req.transactions.iter().filter(|t| t.amount < 0).count() as i32;

    let mut tx = state.pool.begin().await?;
    
    sqlx::query(
        "INSERT INTO bank_statements (id, statement_number, bank_account_id, statement_date, 
         currency, opening_balance, closing_balance, total_credits, total_debits, credit_count, 
         debit_count, statement_format, imported_at, status, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Manual', ?, 'Draft', ?)"
    )
    .bind(id.to_string())
    .bind(&statement_number)
    .bind(req.bank_account_id.to_string())
    .bind(&req.statement_date)
    .bind(&req.currency)
    .bind(req.opening_balance)
    .bind(req.closing_balance)
    .bind(total_credits)
    .bind(total_debits)
    .bind(credit_count)
    .bind(debit_count)
    .bind(now.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(&mut *tx)
    .await?;

    for txn in &req.transactions {
        let txn_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO bank_transactions (id, statement_id, bank_account_id, transaction_date, 
             value_date, transaction_type, amount, currency, reference_number, description, 
             payee_name, reconciliation_status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Unreconciled', ?)"
        )
        .bind(txn_id.to_string())
        .bind(id.to_string())
        .bind(req.bank_account_id.to_string())
        .bind(&txn.transaction_date)
        .bind(&txn.value_date)
        .bind(&txn.transaction_type)
        .bind(txn.amount)
        .bind(&req.currency)
        .bind(&txn.reference_number)
        .bind(&txn.description)
        .bind(&txn.payee_name)
        .bind(now.to_rfc3339())
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(Json(BankStatementResponse {
        id,
        statement_number,
        bank_account_id: req.bank_account_id,
        statement_date: req.statement_date,
        currency: req.currency,
        opening_balance: req.opening_balance,
        closing_balance: req.closing_balance,
        total_credits,
        total_debits,
        credit_count,
        debit_count,
        status: "Draft".to_string(),
        imported_at: now.to_rfc3339(),
    }))
}

pub async fn list_statements(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
    Query(params): Query<ListStatementsParams>,
) -> ApiResult<Json<Vec<BankStatementResponse>>> {
    let rows = if let Some(account_id) = params.account_id {
        sqlx::query_as::<_, StatementRow>(
            "SELECT id, statement_number, bank_account_id, statement_date, currency, 
             opening_balance, closing_balance, total_credits, total_debits, credit_count, 
             debit_count, status, imported_at
             FROM bank_statements WHERE bank_account_id = ?
             ORDER BY statement_date DESC
             LIMIT ? OFFSET ?"
        )
        .bind(account_id.to_string())
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, StatementRow>(
            "SELECT id, statement_number, bank_account_id, statement_date, currency, 
             opening_balance, closing_balance, total_credits, total_debits, credit_count, 
             debit_count, status, imported_at
             FROM bank_statements
             ORDER BY statement_date DESC
             LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(&state.pool)
        .await?
    };

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

#[derive(Debug, Deserialize)]
pub struct ListStatementsParams {
    account_id: Option<Uuid>,
}

pub async fn get_statement(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<BankStatementResponse>> {
    let row = sqlx::query_as::<_, StatementRow>(
        "SELECT id, statement_number, bank_account_id, statement_date, currency, 
         opening_balance, closing_balance, total_credits, total_debits, credit_count, 
         debit_count, status, imported_at
         FROM bank_statements WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| erp_core::Error::not_found("BankStatement", &id.to_string()))?;

    Ok(Json(row.into()))
}

#[derive(Debug, Serialize)]
pub struct BankTransactionResponse {
    pub id: Uuid,
    pub statement_id: Uuid,
    pub bank_account_id: Uuid,
    pub transaction_date: String,
    pub value_date: Option<String>,
    pub transaction_type: String,
    pub amount: i64,
    pub currency: String,
    pub reference_number: Option<String>,
    pub description: String,
    pub payee_name: Option<String>,
    pub reconciliation_status: String,
    pub matched_entity_type: Option<String>,
    pub matched_entity_id: Option<Uuid>,
    pub match_confidence: Option<f64>,
    pub journal_entry_id: Option<Uuid>,
}

pub async fn list_statement_transactions(
    State(state): State<AppState>,
    Path(statement_id): Path<Uuid>,
) -> ApiResult<Json<Vec<BankTransactionResponse>>> {
    let rows = sqlx::query_as::<_, TransactionRow>(
        "SELECT id, statement_id, bank_account_id, transaction_date, value_date, transaction_type,
         amount, currency, reference_number, description, payee_name, reconciliation_status,
         matched_entity_type, matched_entity_id, match_confidence, journal_entry_id
         FROM bank_transactions WHERE statement_id = ?
         ORDER BY transaction_date"
    )
    .bind(statement_id.to_string())
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

pub async fn list_transactions(
    State(state): State<AppState>,
    Query(params): Query<ListTransactionsParams>,
) -> ApiResult<Json<Vec<BankTransactionResponse>>> {
    let rows = if let Some(account_id) = params.account_id {
        sqlx::query_as::<_, TransactionRow>(
            "SELECT id, statement_id, bank_account_id, transaction_date, value_date, transaction_type,
             amount, currency, reference_number, description, payee_name, reconciliation_status,
             matched_entity_type, matched_entity_id, match_confidence, journal_entry_id
             FROM bank_transactions WHERE bank_account_id = ?
             ORDER BY transaction_date DESC
             LIMIT 100"
        )
        .bind(account_id.to_string())
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, TransactionRow>(
            "SELECT id, statement_id, bank_account_id, transaction_date, value_date, transaction_type,
             amount, currency, reference_number, description, payee_name, reconciliation_status,
             matched_entity_type, matched_entity_id, match_confidence, journal_entry_id
             FROM bank_transactions
             ORDER BY transaction_date DESC
             LIMIT 100"
        )
        .fetch_all(&state.pool)
        .await?
    };

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

#[derive(Debug, Deserialize)]
pub struct ListTransactionsParams {
    account_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ReconcileTransactionRequest {
    pub matched_entity_type: Option<String>,
    pub matched_entity_id: Option<Uuid>,
    pub journal_entry_id: Option<Uuid>,
}

pub async fn reconcile_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ReconcileTransactionRequest>,
) -> ApiResult<Json<BankTransactionResponse>> {
    sqlx::query(
        "UPDATE bank_transactions SET reconciliation_status = 'Reconciled', 
         matched_entity_type = ?, matched_entity_id = ?, journal_entry_id = ?,
         match_confidence = 1.0 WHERE id = ?"
    )
    .bind(&req.matched_entity_type)
    .bind(req.matched_entity_id.map(|id| id.to_string()))
    .bind(req.journal_entry_id.map(|id| id.to_string()))
    .bind(id.to_string())
    .execute(&state.pool)
    .await?;

    let row = sqlx::query_as::<_, TransactionRow>(
        "SELECT id, statement_id, bank_account_id, transaction_date, value_date, transaction_type,
         amount, currency, reference_number, description, payee_name, reconciliation_status,
         matched_entity_type, matched_entity_id, match_confidence, journal_entry_id
         FROM bank_transactions WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(row.into()))
}

pub async fn unreconcile_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<BankTransactionResponse>> {
    sqlx::query(
        "UPDATE bank_transactions SET reconciliation_status = 'Unreconciled', 
         matched_entity_type = NULL, matched_entity_id = NULL, journal_entry_id = NULL,
         match_confidence = NULL WHERE id = ?"
    )
    .bind(id.to_string())
    .execute(&state.pool)
    .await?;

    let row = sqlx::query_as::<_, TransactionRow>(
        "SELECT id, statement_id, bank_account_id, transaction_date, value_date, transaction_type,
         amount, currency, reference_number, description, payee_name, reconciliation_status,
         matched_entity_type, matched_entity_id, match_confidence, journal_entry_id
         FROM bank_transactions WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(row.into()))
}

#[derive(Debug, Serialize)]
pub struct ReconciliationSessionResponse {
    pub id: Uuid,
    pub session_number: String,
    pub bank_account_id: Uuid,
    pub period_start: String,
    pub period_end: String,
    pub total_transactions: i32,
    pub matched_count: i32,
    pub unmatched_count: i32,
    pub exception_count: i32,
    pub auto_matched_count: i32,
    pub manual_matched_count: i32,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub calculated_balance: i64,
    pub variance: i64,
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StartReconciliationRequest {
    pub bank_account_id: Uuid,
    pub period_start: String,
    pub period_end: String,
    pub opening_balance: i64,
    pub closing_balance: i64,
}

pub async fn start_reconciliation(
    State(state): State<AppState>,
    Json(req): Json<StartReconciliationRequest>,
) -> ApiResult<Json<ReconciliationSessionResponse>> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let session_number = format!("REC-{}", now.format("%Y%m%d%H%M%S"));

    let (total_txns,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM bank_transactions WHERE bank_account_id = ? 
         AND transaction_date >= ? AND transaction_date <= ?"
    )
    .bind(req.bank_account_id.to_string())
    .bind(&req.period_start)
    .bind(&req.period_end)
    .fetch_one(&state.pool)
    .await?;

    sqlx::query(
        "INSERT INTO reconciliation_sessions (id, session_number, bank_account_id, period_start, 
         period_end, total_transactions, matched_count, unmatched_count, exception_count, 
         auto_matched_count, manual_matched_count, opening_balance, closing_balance, 
         calculated_balance, variance, status, started_at, created_at)
         VALUES (?, ?, ?, ?, ?, ?, 0, ?, 0, 0, 0, ?, ?, 0, ?, 'InProgress', ?, ?)"
    )
    .bind(id.to_string())
    .bind(&session_number)
    .bind(req.bank_account_id.to_string())
    .bind(&req.period_start)
    .bind(&req.period_end)
    .bind(total_txns)
    .bind(total_txns)
    .bind(req.opening_balance)
    .bind(req.closing_balance)
    .bind(req.closing_balance - req.opening_balance)
    .bind(now.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(&state.pool)
    .await?;

    Ok(Json(ReconciliationSessionResponse {
        id,
        session_number,
        bank_account_id: req.bank_account_id,
        period_start: req.period_start,
        period_end: req.period_end,
        total_transactions: total_txns as i32,
        matched_count: 0,
        unmatched_count: total_txns as i32,
        exception_count: 0,
        auto_matched_count: 0,
        manual_matched_count: 0,
        opening_balance: req.opening_balance,
        closing_balance: req.closing_balance,
        calculated_balance: 0,
        variance: req.closing_balance - req.opening_balance,
        status: "InProgress".to_string(),
        started_at: now.to_rfc3339(),
        completed_at: None,
    }))
}

pub async fn list_reconciliations(
    State(state): State<AppState>,
    Query(params): Query<ListReconciliationsParams>,
) -> ApiResult<Json<Vec<ReconciliationSessionResponse>>> {
    let rows = if let Some(account_id) = params.account_id {
        sqlx::query_as::<_, ReconciliationRow>(
            "SELECT id, session_number, bank_account_id, period_start, period_end, 
             total_transactions, matched_count, unmatched_count, exception_count,
             auto_matched_count, manual_matched_count, opening_balance, closing_balance,
             calculated_balance, variance, status, started_at, completed_at
             FROM reconciliation_sessions WHERE bank_account_id = ?
             ORDER BY started_at DESC"
        )
        .bind(account_id.to_string())
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, ReconciliationRow>(
            "SELECT id, session_number, bank_account_id, period_start, period_end, 
             total_transactions, matched_count, unmatched_count, exception_count,
             auto_matched_count, manual_matched_count, opening_balance, closing_balance,
             calculated_balance, variance, status, started_at, completed_at
             FROM reconciliation_sessions
             ORDER BY started_at DESC"
        )
        .fetch_all(&state.pool)
        .await?
    };

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

#[derive(Debug, Deserialize)]
pub struct ListReconciliationsParams {
    account_id: Option<Uuid>,
}

pub async fn get_reconciliation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ReconciliationSessionResponse>> {
    let row = sqlx::query_as::<_, ReconciliationRow>(
        "SELECT id, session_number, bank_account_id, period_start, period_end, 
         total_transactions, matched_count, unmatched_count, exception_count,
         auto_matched_count, manual_matched_count, opening_balance, closing_balance,
         calculated_balance, variance, status, started_at, completed_at
         FROM reconciliation_sessions WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| erp_core::Error::not_found("ReconciliationSession", &id.to_string()))?;

    Ok(Json(row.into()))
}

#[derive(Debug, Deserialize)]
pub struct CompleteReconciliationRequest {
    pub completed_by: Option<Uuid>,
}

pub async fn complete_reconciliation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(_req): Json<CompleteReconciliationRequest>,
) -> ApiResult<Json<ReconciliationSessionResponse>> {
    let now = Utc::now();
    
    sqlx::query(
        "UPDATE reconciliation_sessions SET status = 'Completed', completed_at = ? WHERE id = ?"
    )
    .bind(now.to_rfc3339())
    .bind(id.to_string())
    .execute(&state.pool)
    .await?;

    get_reconciliation(State(state), Path(id)).await
}

#[derive(Debug, Serialize)]
pub struct ReconciliationMatchResponse {
    pub id: Uuid,
    pub session_id: Uuid,
    pub bank_transaction_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub entity_reference: String,
    pub transaction_amount: i64,
    pub entity_amount: i64,
    pub match_difference: i64,
    pub match_type: String,
    pub match_confidence: f64,
    pub matched_at: String,
}

pub async fn list_reconciliation_matches(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<Vec<ReconciliationMatchResponse>>> {
    let rows = sqlx::query_as::<_, MatchRow>(
        "SELECT id, session_id, bank_transaction_id, entity_type, entity_id, entity_reference,
         transaction_amount, entity_amount, match_difference, match_type, match_confidence, matched_at
         FROM reconciliation_matches WHERE session_id = ?
         ORDER BY matched_at DESC"
    )
    .bind(session_id.to_string())
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

#[derive(Debug, Serialize)]
pub struct AutoMatchResult {
    pub matched_count: i32,
    pub matches: Vec<ReconciliationMatchResponse>,
}

pub async fn auto_match_transactions(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<AutoMatchResult>> {
    let session: ReconciliationRow = sqlx::query_as(
        "SELECT id, session_number, bank_account_id, period_start, period_end, 
         total_transactions, matched_count, unmatched_count, exception_count,
         auto_matched_count, manual_matched_count, opening_balance, closing_balance,
         calculated_balance, variance, status, started_at, completed_at
         FROM reconciliation_sessions WHERE id = ?"
    )
    .bind(session_id.to_string())
    .fetch_one(&state.pool)
    .await?;

    let unreconciled_txns = sqlx::query_as::<_, TransactionRow>(
        "SELECT id, statement_id, bank_account_id, transaction_date, value_date, transaction_type,
         amount, currency, reference_number, description, payee_name, reconciliation_status,
         matched_entity_type, matched_entity_id, match_confidence, journal_entry_id
         FROM bank_transactions WHERE bank_account_id = ? 
         AND reconciliation_status = 'Unreconciled'
         AND transaction_date >= ? AND transaction_date <= ?"
    )
    .bind(session.bank_account_id.to_string())
    .bind(&session.period_start)
    .bind(&session.period_end)
    .fetch_all(&state.pool)
    .await?;

    let rules = sqlx::query_as::<_, RuleRow>(
        "SELECT id, rule_name, bank_account_id, match_criteria, tolerance_type, tolerance_value,
         date_tolerance_days, auto_match, priority
         FROM reconciliation_rules WHERE (bank_account_id = ? OR bank_account_id IS NULL) 
         AND auto_match = 1 AND status = 'Active'
         ORDER BY priority"
    )
    .bind(session.bank_account_id.to_string())
    .fetch_all(&state.pool)
    .await?;

    let mut matches = Vec::new();
    let now = Utc::now();

    for txn in &unreconciled_txns {
        if let Some(match_result) = find_match(&state.pool, txn, &rules).await? {
            let match_id = Uuid::new_v4();
            
            sqlx::query(
                "INSERT INTO reconciliation_matches (id, session_id, bank_transaction_id, 
                 entity_type, entity_id, entity_reference, transaction_amount, entity_amount,
                 match_difference, match_type, match_confidence, matched_at, status, created_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'Auto', ?, ?, 'Active', ?)"
            )
            .bind(match_id.to_string())
            .bind(session_id.to_string())
            .bind(txn.id.to_string())
            .bind(&match_result.entity_type)
            .bind(match_result.entity_id.to_string())
            .bind(&match_result.entity_reference)
            .bind(txn.amount)
            .bind(match_result.entity_amount)
            .bind(txn.amount - match_result.entity_amount)
            .bind(match_result.confidence)
            .bind(now.to_rfc3339())
            .bind(now.to_rfc3339())
            .execute(&state.pool)
            .await?;

            sqlx::query(
                "UPDATE bank_transactions SET reconciliation_status = 'Reconciled',
                 matched_entity_type = ?, matched_entity_id = ?, match_confidence = ?
                 WHERE id = ?"
            )
            .bind(&match_result.entity_type)
            .bind(match_result.entity_id.to_string())
            .bind(match_result.confidence)
            .bind(txn.id.to_string())
            .execute(&state.pool)
            .await?;

            matches.push(ReconciliationMatchResponse {
                id: match_id,
                session_id,
                bank_transaction_id: Uuid::parse_str(&txn.id).unwrap_or_default(),
                entity_type: match_result.entity_type,
                entity_id: match_result.entity_id,
                entity_reference: match_result.entity_reference,
                transaction_amount: txn.amount,
                entity_amount: match_result.entity_amount,
                match_difference: txn.amount - match_result.entity_amount,
                match_type: "Auto".to_string(),
                match_confidence: match_result.confidence,
                matched_at: now.to_rfc3339(),
            });
        }
    }

    let matched_count = matches.len() as i32;
    let total_matched: i64 = matches.iter().map(|m| m.transaction_amount).sum();

    sqlx::query(
        "UPDATE reconciliation_sessions SET 
         matched_count = matched_count + ?,
         unmatched_count = unmatched_count - ?,
         auto_matched_count = auto_matched_count + ?,
         calculated_balance = calculated_balance + ?
         WHERE id = ?"
    )
    .bind(matched_count)
    .bind(matched_count)
    .bind(matched_count)
    .bind(total_matched)
    .bind(session_id.to_string())
    .execute(&state.pool)
    .await?;

    Ok(Json(AutoMatchResult {
        matched_count,
        matches,
    }))
}

struct MatchResult {
    entity_type: String,
    entity_id: Uuid,
    entity_reference: String,
    entity_amount: i64,
    confidence: f64,
}

async fn find_match(
    pool: &sqlx::SqlitePool,
    txn: &TransactionRow,
    _rules: &[RuleRow],
) -> ApiResult<Option<MatchResult>> {
    let gl_match: Option<(String, String, i64)> = sqlx::query_as(
        "SELECT je.id, je.entry_number, jl.debit - jl.credit as amount
         FROM journal_lines jl
         JOIN journal_entries je ON jl.journal_entry_id = je.id
         JOIN bank_accounts ba ON ba.gl_account_id = jl.account_id
         WHERE ba.id = ?
         AND ABS(jl.debit - jl.credit) = ABS(?)
         AND je.status = 'Posted'
         AND date(je.date) = date(?)
         LIMIT 1"
    )
    .bind(txn.bank_account_id.to_string())
    .bind(txn.amount.abs())
    .bind(&txn.transaction_date)
    .fetch_optional(pool)
    .await?;

    if let Some((id, ref_num, amount)) = gl_match {
        return Ok(Some(MatchResult {
            entity_type: "JournalEntry".to_string(),
            entity_id: Uuid::parse_str(&id).unwrap_or_default(),
            entity_reference: ref_num,
            entity_amount: amount,
            confidence: 0.95,
        }));
    }

    let amount_match: Option<(String, String, i64)> = sqlx::query_as(
        "SELECT je.id, je.entry_number, jl.debit - jl.credit as amount
         FROM journal_lines jl
         JOIN journal_entries je ON jl.journal_entry_id = je.id
         JOIN bank_accounts ba ON ba.gl_account_id = jl.account_id
         WHERE ba.id = ?
         AND ABS(jl.debit - jl.credit) = ABS(?)
         AND je.status = 'Posted'
         LIMIT 1"
    )
    .bind(txn.bank_account_id.to_string())
    .bind(txn.amount.abs())
    .fetch_optional(pool)
    .await?;

    if let Some((id, ref_num, amount)) = amount_match {
        return Ok(Some(MatchResult {
            entity_type: "JournalEntry".to_string(),
            entity_id: Uuid::parse_str(&id).unwrap_or_default(),
            entity_reference: ref_num,
            entity_amount: amount,
            confidence: 0.8,
        }));
    }

    Ok(None)
}

#[derive(Debug, Serialize)]
pub struct ReconciliationRuleResponse {
    pub id: Uuid,
    pub rule_name: String,
    pub description: Option<String>,
    pub bank_account_id: Option<Uuid>,
    pub match_criteria: String,
    pub tolerance_type: String,
    pub tolerance_value: f64,
    pub date_tolerance_days: i32,
    pub auto_match: bool,
    pub priority: i32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReconciliationRuleRequest {
    pub rule_name: String,
    pub description: Option<String>,
    pub bank_account_id: Option<Uuid>,
    pub match_criteria: String,
    pub tolerance_type: String,
    pub tolerance_value: f64,
    pub date_tolerance_days: i32,
    pub auto_match: bool,
    pub priority: i32,
}

pub async fn create_reconciliation_rule(
    State(state): State<AppState>,
    Json(req): Json<CreateReconciliationRuleRequest>,
) -> ApiResult<Json<ReconciliationRuleResponse>> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO reconciliation_rules (id, rule_name, description, bank_account_id, 
         match_criteria, tolerance_type, tolerance_value, date_tolerance_days, auto_match, 
         priority, status, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Active', ?, ?)"
    )
    .bind(id.to_string())
    .bind(&req.rule_name)
    .bind(&req.description)
    .bind(req.bank_account_id.map(|id| id.to_string()))
    .bind(&req.match_criteria)
    .bind(&req.tolerance_type)
    .bind(req.tolerance_value)
    .bind(req.date_tolerance_days)
    .bind(req.auto_match as i32)
    .bind(req.priority)
    .bind(now.to_rfc3339())
    .bind(now.to_rfc3339())
    .execute(&state.pool)
    .await?;

    Ok(Json(ReconciliationRuleResponse {
        id,
        rule_name: req.rule_name,
        description: req.description,
        bank_account_id: req.bank_account_id,
        match_criteria: req.match_criteria,
        tolerance_type: req.tolerance_type,
        tolerance_value: req.tolerance_value,
        date_tolerance_days: req.date_tolerance_days,
        auto_match: req.auto_match,
        priority: req.priority,
        status: "Active".to_string(),
    }))
}

pub async fn list_reconciliation_rules(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<ReconciliationRuleResponse>>> {
    let rows = sqlx::query_as::<_, RuleRow>(
        "SELECT id, rule_name, description, bank_account_id, match_criteria, tolerance_type,
         tolerance_value, date_tolerance_days, auto_match, priority, status
         FROM reconciliation_rules WHERE status = 'Active'
         ORDER BY priority"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

pub async fn get_reconciliation_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ReconciliationRuleResponse>> {
    let row = sqlx::query_as::<_, RuleRow>(
        "SELECT id, rule_name, description, bank_account_id, match_criteria, tolerance_type,
         tolerance_value, date_tolerance_days, auto_match, priority, status
         FROM reconciliation_rules WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| erp_core::Error::not_found("ReconciliationRule", &id.to_string()))?;

    Ok(Json(row.into()))
}

pub async fn update_reconciliation_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateReconciliationRuleRequest>,
) -> ApiResult<Json<ReconciliationRuleResponse>> {
    let now = Utc::now();

    sqlx::query(
        "UPDATE reconciliation_rules SET rule_name = ?, description = ?, bank_account_id = ?,
         match_criteria = ?, tolerance_type = ?, tolerance_value = ?, date_tolerance_days = ?,
         auto_match = ?, priority = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&req.rule_name)
    .bind(&req.description)
    .bind(req.bank_account_id.map(|id| id.to_string()))
    .bind(&req.match_criteria)
    .bind(&req.tolerance_type)
    .bind(req.tolerance_value)
    .bind(req.date_tolerance_days)
    .bind(req.auto_match as i32)
    .bind(req.priority)
    .bind(now.to_rfc3339())
    .bind(id.to_string())
    .execute(&state.pool)
    .await?;

    get_reconciliation_rule(State(state), Path(id)).await
}

pub async fn list_unreconciled_transactions(
    State(state): State<AppState>,
    Query(params): Query<ListTransactionsParams>,
) -> ApiResult<Json<Vec<BankTransactionResponse>>> {
    let rows = if let Some(account_id) = params.account_id {
        sqlx::query_as::<_, TransactionRow>(
            "SELECT id, statement_id, bank_account_id, transaction_date, value_date, transaction_type,
             amount, currency, reference_number, description, payee_name, reconciliation_status,
             matched_entity_type, matched_entity_id, match_confidence, journal_entry_id
             FROM bank_transactions WHERE bank_account_id = ? AND reconciliation_status = 'Unreconciled'
             ORDER BY transaction_date DESC"
        )
        .bind(account_id.to_string())
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query_as::<_, TransactionRow>(
            "SELECT id, statement_id, bank_account_id, transaction_date, value_date, transaction_type,
             amount, currency, reference_number, description, payee_name, reconciliation_status,
             matched_entity_type, matched_entity_id, match_confidence, journal_entry_id
             FROM bank_transactions WHERE reconciliation_status = 'Unreconciled'
             ORDER BY transaction_date DESC"
        )
        .fetch_all(&state.pool)
        .await?
    };

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

#[derive(Debug, Serialize)]
pub struct ReconciliationSummaryResponse {
    pub bank_account_id: Uuid,
    pub account_name: String,
    pub currency: String,
    pub gl_balance: i64,
    pub bank_balance: Option<i64>,
    pub unreconciled_count: i32,
    pub unreconciled_debits: i64,
    pub unreconciled_credits: i64,
    pub deposits_in_transit: i64,
    pub outstanding_checks: i64,
    pub adjusted_balance: i64,
    pub variance: i64,
    pub last_reconciled_at: Option<String>,
}

pub async fn get_reconciliation_summary(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
) -> ApiResult<Json<ReconciliationSummaryResponse>> {
    let account: (String, String, Option<String>) = sqlx::query_as(
        "SELECT account_name, currency, gl_account_id FROM bank_accounts WHERE id = ?"
    )
    .bind(account_id.to_string())
    .fetch_one(&state.pool)
    .await
    .map_err(|_| erp_core::Error::not_found("BankAccount", &account_id.to_string()))?;

    let gl_balance: i64 = if let Some(gl_account_id) = account.2 {
        let result: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(debit) - SUM(credit), 0) FROM journal_lines 
             WHERE account_id = ?"
        )
        .bind(&gl_account_id)
        .fetch_one(&state.pool)
        .await?;
        result.0
    } else {
        0
    };

    let (unreconciled_count, unreconciled_debits, unreconciled_credits): (i64, i64, i64) = sqlx::query_as(
        "SELECT COUNT(*), 
         COALESCE(SUM(CASE WHEN amount < 0 THEN ABS(amount) ELSE 0 END), 0),
         COALESCE(SUM(CASE WHEN amount > 0 THEN amount ELSE 0 END), 0)
         FROM bank_transactions WHERE bank_account_id = ? AND reconciliation_status = 'Unreconciled'"
    )
    .bind(account_id.to_string())
    .fetch_one(&state.pool)
    .await?;

    let last_reconciled: Option<(String,)> = sqlx::query_as(
        "SELECT completed_at FROM reconciliation_sessions 
         WHERE bank_account_id = ? AND status = 'Completed'
         ORDER BY completed_at DESC LIMIT 1"
    )
    .bind(account_id.to_string())
    .fetch_optional(&state.pool)
    .await?;

    let bank_balance: Option<i64> = sqlx::query_as(
        "SELECT closing_balance FROM bank_statements 
         WHERE bank_account_id = ? ORDER BY statement_date DESC LIMIT 1"
    )
    .bind(account_id.to_string())
    .fetch_optional(&state.pool)
    .await?
    .map(|(v,)| v);

    let adjusted_balance = gl_balance + unreconciled_credits - unreconciled_debits;
    let variance = bank_balance.map(|bb| adjusted_balance - bb).unwrap_or(0);

    Ok(Json(ReconciliationSummaryResponse {
        bank_account_id: account_id,
        account_name: account.0,
        currency: account.1,
        gl_balance,
        bank_balance,
        unreconciled_count: unreconciled_count as i32,
        unreconciled_debits,
        unreconciled_credits,
        deposits_in_transit: unreconciled_credits,
        outstanding_checks: unreconciled_debits,
        adjusted_balance,
        variance,
        last_reconciled_at: last_reconciled.map(|(d,)| d),
    }))
}

fn mask_account_number(account_number: &str) -> String {
    if account_number.len() > 4 {
        format!("****{}", &account_number[account_number.len()-4..])
    } else {
        "****".to_string()
    }
}

#[derive(sqlx::FromRow)]
struct BankAccountRow {
    id: String,
    connection_id: String,
    account_number: String,
    masked_account_number: String,
    account_name: String,
    account_type: String,
    currency: String,
    gl_account_id: Option<String>,
    company_id: String,
    bank_branch: Option<String>,
    iban: Option<String>,
    routing_number: Option<String>,
    auto_reconcile: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<BankAccountRow> for BankAccountResponse {
    fn from(r: BankAccountRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            connection_id: Uuid::parse_str(&r.connection_id).unwrap_or_default(),
            account_number: r.account_number,
            masked_account_number: r.masked_account_number,
            account_name: r.account_name,
            account_type: r.account_type,
            currency: r.currency,
            gl_account_id: r.gl_account_id.and_then(|s| Uuid::parse_str(&s).ok()),
            company_id: Uuid::parse_str(&r.company_id).unwrap_or_default(),
            bank_branch: r.bank_branch,
            iban: r.iban,
            routing_number: r.routing_number,
            auto_reconcile: r.auto_reconcile != 0,
            status: r.status,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct StatementRow {
    id: String,
    statement_number: String,
    bank_account_id: String,
    statement_date: String,
    currency: String,
    opening_balance: i64,
    closing_balance: i64,
    total_credits: i64,
    total_debits: i64,
    credit_count: i64,
    debit_count: i64,
    status: String,
    imported_at: String,
}

impl From<StatementRow> for BankStatementResponse {
    fn from(r: StatementRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            statement_number: r.statement_number,
            bank_account_id: Uuid::parse_str(&r.bank_account_id).unwrap_or_default(),
            statement_date: r.statement_date,
            currency: r.currency,
            opening_balance: r.opening_balance,
            closing_balance: r.closing_balance,
            total_credits: r.total_credits,
            total_debits: r.total_debits,
            credit_count: r.credit_count as i32,
            debit_count: r.debit_count as i32,
            status: r.status,
            imported_at: r.imported_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct TransactionRow {
    id: String,
    statement_id: String,
    bank_account_id: String,
    transaction_date: String,
    value_date: Option<String>,
    transaction_type: String,
    amount: i64,
    currency: String,
    reference_number: Option<String>,
    description: String,
    payee_name: Option<String>,
    reconciliation_status: String,
    matched_entity_type: Option<String>,
    matched_entity_id: Option<String>,
    match_confidence: Option<f64>,
    journal_entry_id: Option<String>,
}

impl From<TransactionRow> for BankTransactionResponse {
    fn from(r: TransactionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            statement_id: Uuid::parse_str(&r.statement_id).unwrap_or_default(),
            bank_account_id: Uuid::parse_str(&r.bank_account_id).unwrap_or_default(),
            transaction_date: r.transaction_date,
            value_date: r.value_date,
            transaction_type: r.transaction_type,
            amount: r.amount,
            currency: r.currency,
            reference_number: r.reference_number,
            description: r.description,
            payee_name: r.payee_name,
            reconciliation_status: r.reconciliation_status,
            matched_entity_type: r.matched_entity_type,
            matched_entity_id: r.matched_entity_id.and_then(|s| Uuid::parse_str(&s).ok()),
            match_confidence: r.match_confidence,
            journal_entry_id: r.journal_entry_id.and_then(|s| Uuid::parse_str(&s).ok()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ReconciliationRow {
    id: String,
    session_number: String,
    bank_account_id: String,
    period_start: String,
    period_end: String,
    total_transactions: i64,
    matched_count: i64,
    unmatched_count: i64,
    exception_count: i64,
    auto_matched_count: i64,
    manual_matched_count: i64,
    opening_balance: i64,
    closing_balance: i64,
    calculated_balance: i64,
    variance: i64,
    status: String,
    started_at: String,
    completed_at: Option<String>,
}

impl From<ReconciliationRow> for ReconciliationSessionResponse {
    fn from(r: ReconciliationRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            session_number: r.session_number,
            bank_account_id: Uuid::parse_str(&r.bank_account_id).unwrap_or_default(),
            period_start: r.period_start,
            period_end: r.period_end,
            total_transactions: r.total_transactions as i32,
            matched_count: r.matched_count as i32,
            unmatched_count: r.unmatched_count as i32,
            exception_count: r.exception_count as i32,
            auto_matched_count: r.auto_matched_count as i32,
            manual_matched_count: r.manual_matched_count as i32,
            opening_balance: r.opening_balance,
            closing_balance: r.closing_balance,
            calculated_balance: r.calculated_balance,
            variance: r.variance,
            status: r.status,
            started_at: r.started_at,
            completed_at: r.completed_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct MatchRow {
    id: String,
    session_id: String,
    bank_transaction_id: String,
    entity_type: String,
    entity_id: String,
    entity_reference: String,
    transaction_amount: i64,
    entity_amount: i64,
    match_difference: i64,
    match_type: String,
    match_confidence: f64,
    matched_at: String,
}

impl From<MatchRow> for ReconciliationMatchResponse {
    fn from(r: MatchRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            session_id: Uuid::parse_str(&r.session_id).unwrap_or_default(),
            bank_transaction_id: Uuid::parse_str(&r.bank_transaction_id).unwrap_or_default(),
            entity_type: r.entity_type,
            entity_id: Uuid::parse_str(&r.entity_id).unwrap_or_default(),
            entity_reference: r.entity_reference,
            transaction_amount: r.transaction_amount,
            entity_amount: r.entity_amount,
            match_difference: r.match_difference,
            match_type: r.match_type,
            match_confidence: r.match_confidence,
            matched_at: r.matched_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct RuleRow {
    id: String,
    rule_name: String,
    description: Option<String>,
    bank_account_id: Option<String>,
    match_criteria: String,
    tolerance_type: String,
    tolerance_value: f64,
    date_tolerance_days: i64,
    auto_match: i64,
    priority: i64,
    status: String,
}

impl From<RuleRow> for ReconciliationRuleResponse {
    fn from(r: RuleRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            rule_name: r.rule_name,
            description: r.description,
            bank_account_id: r.bank_account_id.and_then(|s| Uuid::parse_str(&s).ok()),
            match_criteria: r.match_criteria,
            tolerance_type: r.tolerance_type,
            tolerance_value: r.tolerance_value,
            date_tolerance_days: r.date_tolerance_days as i32,
            auto_match: r.auto_match != 0,
            priority: r.priority as i32,
            status: r.status,
        }
    }
}
