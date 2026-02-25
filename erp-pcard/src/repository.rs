use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait PCardRepository: Send + Sync {
    async fn create_card(&self, card: &CorporateCard) -> Result<CorporateCard>;
    async fn get_card(&self, id: Uuid) -> Result<Option<CorporateCard>>;
    async fn list_cards(&self, cardholder_id: Option<Uuid>) -> Result<Vec<CorporateCard>>;
    async fn update_card(&self, card: &CorporateCard) -> Result<CorporateCard>;
    async fn create_transaction(&self, tx: &CardTransaction) -> Result<CardTransaction>;
    async fn list_transactions(&self, card_id: Uuid) -> Result<Vec<CardTransaction>>;
    async fn create_statement(&self, stmt: &CardStatement) -> Result<CardStatement>;
    async fn create_order(&self, order: &PCardOrder) -> Result<PCardOrder>;
    async fn create_policy(&self, policy: &CardPolicy) -> Result<CardPolicy>;
    async fn create_dispute(&self, dispute: &CardDispute) -> Result<CardDispute>;
    async fn create_virtual_card(&self, card: &VirtualCard) -> Result<VirtualCard>;
}

pub struct SqlitePCardRepository { pool: SqlitePool }
impl SqlitePCardRepository { pub fn new(pool: SqlitePool) -> Self { Self { pool } } }

#[async_trait]
impl PCardRepository for SqlitePCardRepository {
    async fn create_card(&self, card: &CorporateCard) -> Result<CorporateCard> {
        let c = card.clone();
        sqlx::query!(r#"INSERT INTO corporate_cards (id, card_number, masked_number, card_type,
            cardholder_id, department_id, issuer, card_program, credit_limit, available_credit,
            currency, issue_date, expiry_date, last_four, embossed_name, pin_set, contactless_enabled,
            international_enabled, atm_enabled, online_enabled, mcc_restrictions, merchant_restrictions,
            daily_limit, transaction_limit, status, activated_at, cancelled_at, cancellation_reason,
            created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            c.base.id, c.card_number, c.masked_number, c.card_type, c.cardholder_id, c.department_id,
            c.issuer, c.card_program, c.credit_limit, c.available_credit, c.currency, c.issue_date,
            c.expiry_date, c.last_four, c.embossed_name, c.pin_set, c.contactless_enabled,
            c.international_enabled, c.atm_enabled, c.online_enabled, c.mcc_restrictions, c.merchant_restrictions,
            c.daily_limit, c.transaction_limit, c.status, c.activated_at, c.cancelled_at, c.cancellation_reason,
            c.created_at, c.updated_at).execute(&self.pool).await?;
        Ok(c)
    }
    async fn get_card(&self, _id: Uuid) -> Result<Option<CorporateCard>> { Ok(None) }
    async fn list_cards(&self, _cardholder_id: Option<Uuid>) -> Result<Vec<CorporateCard>> { Ok(vec![]) }
    async fn update_card(&self, card: &CorporateCard) -> Result<CorporateCard> { Ok(card.clone()) }
    async fn create_transaction(&self, tx: &CardTransaction) -> Result<CardTransaction> {
        let t = tx.clone();
        sqlx::query!(r#"INSERT INTO card_transactions (id, transaction_number, card_id, transaction_date,
            posting_date, merchant_name, merchant_category, mcc_code, amount, currency, billing_amount,
            billing_currency, transaction_type, status, reference_number, authorization_code, description,
            receipt_available, receipt_path, expense_report_id, expense_line_id, reconciled, reconciled_at,
            approved_by, approved_at, tax_amount, tip_amount, notes, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            t.base.id, t.transaction_number, t.card_id, t.transaction_date, t.posting_date, t.merchant_name,
            t.merchant_category, t.mcc_code, t.amount, t.currency, t.billing_amount, t.billing_currency,
            t.transaction_type, t.status, t.reference_number, t.authorization_code, t.description,
            t.receipt_available, t.receipt_path, t.expense_report_id, t.expense_line_id, t.reconciled,
            t.reconciled_at, t.approved_by, t.approved_at, t.tax_amount, t.tip_amount, t.notes, t.created_at).execute(&self.pool).await?;
        Ok(t)
    }
    async fn list_transactions(&self, _card_id: Uuid) -> Result<Vec<CardTransaction>> { Ok(vec![]) }
    async fn create_statement(&self, stmt: &CardStatement) -> Result<CardStatement> {
        let s = stmt.clone();
        sqlx::query!(r#"INSERT INTO card_statements (id, statement_number, card_id, statement_date,
            due_date, period_start, period_end, opening_balance, payments, credits, purchases, fees,
            interest, closing_balance, minimum_payment, currency, paid_amount, paid_date, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            s.base.id, s.statement_number, s.card_id, s.statement_date, s.due_date, s.period_start,
            s.period_end, s.opening_balance, s.payments, s.credits, s.purchases, s.fees, s.interest,
            s.closing_balance, s.minimum_payment, s.currency, s.paid_amount, s.paid_date, s.status, s.created_at).execute(&self.pool).await?;
        Ok(s)
    }
    async fn create_order(&self, order: &PCardOrder) -> Result<PCardOrder> {
        let o = order.clone();
        sqlx::query!(r#"INSERT INTO pcard_orders (id, order_number, card_id, vendor_id, vendor_name,
            order_date, required_date, delivery_date, subtotal, tax, shipping, total, currency,
            po_number, transaction_id, status, approved_by, approved_at, notes, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            o.base.id, o.order_number, o.card_id, o.vendor_id, o.vendor_name, o.order_date,
            o.required_date, o.delivery_date, o.subtotal, o.tax, o.shipping, o.total, o.currency,
            o.po_number, o.transaction_id, o.status, o.approved_by, o.approved_at, o.notes, o.created_at, o.updated_at).execute(&self.pool).await?;
        Ok(o)
    }
    async fn create_policy(&self, policy: &CardPolicy) -> Result<CardPolicy> {
        let p = policy.clone();
        sqlx::query!(r#"INSERT INTO card_policies (id, policy_number, name, description, card_type,
            default_limit, daily_limit, transaction_limit, mcc_allowed, mcc_blocked, merchant_blocked,
            requires_approval_over, approval_workflow_id, requires_receipt_over, auto_reconcile,
            international_allowed, atm_allowed, online_allowed, contactless_allowed, status,
            created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            p.base.id, p.policy_number, p.name, p.description, p.card_type, p.default_limit,
            p.daily_limit, p.transaction_limit, p.mcc_allowed, p.mcc_blocked, p.merchant_blocked,
            p.requires_approval_over, p.approval_workflow_id, p.requires_receipt_over, p.auto_reconcile,
            p.international_allowed, p.atm_allowed, p.online_allowed, p.contactless_allowed, p.status,
            p.created_at, p.updated_at).execute(&self.pool).await?;
        Ok(p)
    }
    async fn create_dispute(&self, dispute: &CardDispute) -> Result<CardDispute> {
        let d = dispute.clone();
        sqlx::query!(r#"INSERT INTO card_disputes (id, dispute_number, card_id, transaction_id,
            dispute_type, dispute_reason, disputed_amount, currency, filed_date, resolution_date,
            resolution, provisional_credit, provisional_credit_date, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            d.base.id, d.dispute_number, d.card_id, d.transaction_id, d.dispute_type, d.dispute_reason,
            d.disputed_amount, d.currency, d.filed_date, d.resolution_date, d.resolution,
            d.provisional_credit, d.provisional_credit_date, d.status, d.created_at, d.updated_at).execute(&self.pool).await?;
        Ok(d)
    }
    async fn create_virtual_card(&self, card: &VirtualCard) -> Result<VirtualCard> {
        let c = card.clone();
        sqlx::query!(r#"INSERT INTO virtual_cards (id, parent_card_id, cardholder_id, masked_number,
            credit_limit, available_credit, currency, valid_from, valid_until, single_use,
            merchant_lock, usage_limit, usage_count, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            c.base.id, c.parent_card_id, c.cardholder_id, c.masked_number, c.credit_limit, c.available_credit,
            c.currency, c.valid_from, c.valid_until, c.single_use, c.merchant_lock, c.usage_limit,
            c.usage_count, c.status, c.created_at).execute(&self.pool).await?;
        Ok(c)
    }
}
