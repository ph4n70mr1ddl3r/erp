use crate::models::*;
use crate::repository::{PCardRepository, SqlitePCardRepository};
use chrono::{NaiveDate, Utc};
use erp_core::{BaseEntity, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct PCardService { repo: SqlitePCardRepository }
impl PCardService {
    pub fn new(pool: SqlitePool) -> Self { Self { repo: SqlitePCardRepository::new(pool) } }

    pub async fn issue_card(&self, pool: &SqlitePool, req: IssueCardRequest) -> Result<CorporateCard> {
        let card_number = generate_card_number();
        let last_four = card_number[card_number.len()-4..].to_string();
        let masked = format!("****{}", last_four);
        let card = CorporateCard {
            base: BaseEntity::new(),
            card_number,
            masked_number: masked,
            card_type: req.card_type,
            cardholder_id: req.cardholder_id,
            department_id: req.department_id,
            issuer: req.issuer,
            card_program: req.card_program,
            credit_limit: req.credit_limit,
            available_credit: req.credit_limit,
            currency: req.currency,
            issue_date: Utc::now().date_naive(),
            expiry_date: Utc::now().date_naive() + chrono::Duration::days(365 * 3),
            last_four,
            embossed_name: req.embossed_name,
            pin_set: false,
            contactless_enabled: true,
            international_enabled: req.international_enabled.unwrap_or(false),
            atm_enabled: req.atm_enabled.unwrap_or(false),
            online_enabled: true,
            mcc_restrictions: req.mcc_restrictions,
            merchant_restrictions: req.merchant_restrictions,
            daily_limit: req.daily_limit,
            transaction_limit: req.transaction_limit,
            status: CardStatus::PendingActivation,
            activated_at: None,
            cancelled_at: None,
            cancellation_reason: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_card(&card).await
    }

    pub async fn record_transaction(&self, pool: &SqlitePool, req: RecordTransactionRequest) -> Result<CardTransaction> {
        let tx = CardTransaction {
            base: BaseEntity::new(),
            transaction_number: format!("CTX-{}", Uuid::new_v4()),
            card_id: req.card_id,
            transaction_date: req.transaction_date,
            posting_date: req.posting_date,
            merchant_name: req.merchant_name,
            merchant_category: req.merchant_category,
            mcc_code: req.mcc_code,
            amount: req.amount,
            currency: req.currency,
            billing_amount: req.billing_amount,
            billing_currency: req.billing_currency,
            transaction_type: req.transaction_type,
            status: CardTransactionStatus::Pending,
            reference_number: req.reference_number,
            authorization_code: req.authorization_code,
            description: req.description,
            receipt_available: false,
            receipt_path: None,
            expense_report_id: None,
            expense_line_id: None,
            reconciled: false,
            reconciled_at: None,
            approved_by: None,
            approved_at: None,
            tax_amount: req.tax_amount,
            tip_amount: req.tip_amount,
            notes: req.notes,
            created_at: Utc::now(),
        };
        self.repo.create_transaction(&tx).await
    }

    pub async fn create_virtual_card(&self, pool: &SqlitePool, req: CreateVirtualCardRequest) -> Result<VirtualCard> {
        let masked = format!("****{}", &Uuid::new_v4().to_string()[..4]);
        let card = VirtualCard {
            base: BaseEntity::new(),
            parent_card_id: req.parent_card_id,
            cardholder_id: req.cardholder_id,
            masked_number: masked,
            credit_limit: req.credit_limit,
            available_credit: req.credit_limit,
            currency: req.currency,
            valid_from: req.valid_from,
            valid_until: req.valid_until,
            single_use: req.single_use.unwrap_or(false),
            merchant_lock: req.merchant_lock,
            usage_limit: req.usage_limit,
            usage_count: 0,
            status: CardStatus::Active,
            created_at: Utc::now(),
        };
        self.repo.create_virtual_card(&card).await
    }

    pub async fn file_dispute(&self, pool: &SqlitePool, req: FileDisputeRequest) -> Result<CardDispute> {
        let dispute = CardDispute {
            base: BaseEntity::new(),
            dispute_number: format!("DSP-{}", Uuid::new_v4()),
            card_id: req.card_id,
            transaction_id: req.transaction_id,
            dispute_type: req.dispute_type,
            dispute_reason: req.dispute_reason,
            disputed_amount: req.disputed_amount,
            currency: req.currency,
            filed_date: Utc::now().date_naive(),
            resolution_date: None,
            resolution: None,
            provisional_credit: None,
            provisional_credit_date: None,
            status: DisputeStatus::Filed,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_dispute(&dispute).await
    }
}

fn generate_card_number() -> String {
    format!("4{}", &Uuid::new_v4().to_string().replace("-", "")[..15])
}

#[derive(Debug, serde::Deserialize)]
pub struct IssueCardRequest {
    pub card_type: CardType,
    pub cardholder_id: Uuid,
    pub department_id: Option<Uuid>,
    pub issuer: String,
    pub card_program: Option<String>,
    pub credit_limit: i64,
    pub currency: String,
    pub embossed_name: Option<String>,
    pub international_enabled: Option<bool>,
    pub atm_enabled: Option<bool>,
    pub mcc_restrictions: Option<String>,
    pub merchant_restrictions: Option<String>,
    pub daily_limit: Option<i64>,
    pub transaction_limit: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RecordTransactionRequest {
    pub card_id: Uuid,
    pub transaction_date: NaiveDate,
    pub posting_date: Option<NaiveDate>,
    pub merchant_name: String,
    pub merchant_category: Option<String>,
    pub mcc_code: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub billing_amount: i64,
    pub billing_currency: String,
    pub transaction_type: CardTransactionType,
    pub reference_number: Option<String>,
    pub authorization_code: Option<String>,
    pub description: Option<String>,
    pub tax_amount: Option<i64>,
    pub tip_amount: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateVirtualCardRequest {
    pub parent_card_id: Option<Uuid>,
    pub cardholder_id: Uuid,
    pub credit_limit: i64,
    pub currency: String,
    pub valid_from: NaiveDate,
    pub valid_until: NaiveDate,
    pub single_use: Option<bool>,
    pub merchant_lock: Option<String>,
    pub usage_limit: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FileDisputeRequest {
    pub card_id: Uuid,
    pub transaction_id: Uuid,
    pub dispute_type: DisputeType,
    pub dispute_reason: String,
    pub disputed_amount: i64,
    pub currency: String,
}
