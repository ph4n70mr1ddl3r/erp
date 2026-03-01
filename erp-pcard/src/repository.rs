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
        sqlx::query(r#"INSERT INTO corporate_cards (id, card_number, masked_number, card_type,
            cardholder_id, department_id, issuer, card_program, credit_limit, available_credit,
            currency, issue_date, expiry_date, last_four, embossed_name, pin_set, contactless_enabled,
            international_enabled, atm_enabled, online_enabled, mcc_restrictions, merchant_restrictions,
            daily_limit, transaction_limit, status, activated_at, cancelled_at, cancellation_reason,
            created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(c.base.id.to_string())
            .bind(c.card_number)
            .bind(c.masked_number)
            .bind(c.card_type)
            .bind(c.cardholder_id.to_string())
            .bind(c.department_id.map(|id| id.to_string()))
            .bind(c.issuer)
            .bind(c.card_program)
            .bind(c.credit_limit)
            .bind(c.available_credit)
            .bind(c.currency)
            .bind(c.issue_date.to_string())
            .bind(c.expiry_date.to_string())
            .bind(c.last_four)
            .bind(c.embossed_name)
            .bind(c.pin_set)
            .bind(c.contactless_enabled)
            .bind(c.international_enabled)
            .bind(c.atm_enabled)
            .bind(c.online_enabled)
            .bind(c.mcc_restrictions)
            .bind(c.merchant_restrictions)
            .bind(c.daily_limit)
            .bind(c.transaction_limit)
            .bind(c.status)
            .bind(c.activated_at.map(|d| d.to_string()))
            .bind(c.cancelled_at.map(|d| d.to_string()))
            .bind(c.cancellation_reason.clone())
            .bind(c.created_at.to_string())
            .bind(c.updated_at.to_string())
            .execute(&self.pool).await?;
        Ok(card.clone())
    }
    async fn get_card(&self, _id: Uuid) -> Result<Option<CorporateCard>> { Ok(None) }
    async fn list_cards(&self, _cardholder_id: Option<Uuid>) -> Result<Vec<CorporateCard>> { Ok(vec![]) }
    async fn update_card(&self, card: &CorporateCard) -> Result<CorporateCard> { Ok(card.clone()) }
    async fn create_transaction(&self, tx: &CardTransaction) -> Result<CardTransaction> {
        let t = tx.clone();
        sqlx::query(r#"INSERT INTO card_transactions (id, transaction_number, card_id, transaction_date,
            posting_date, merchant_name, merchant_category, mcc_code, amount, currency, billing_amount,
            billing_currency, transaction_type, status, reference_number, authorization_code, description,
            receipt_available, receipt_path, expense_report_id, expense_line_id, reconciled, reconciled_at,
            approved_by, approved_at, tax_amount, tip_amount, notes, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(t.base.id.to_string())
            .bind(t.transaction_number)
            .bind(t.card_id.to_string())
            .bind(t.transaction_date.to_string())
            .bind(t.posting_date.map(|d| d.to_string()))
            .bind(t.merchant_name)
            .bind(t.merchant_category)
            .bind(t.mcc_code)
            .bind(t.amount)
            .bind(t.currency)
            .bind(t.billing_amount)
            .bind(t.billing_currency)
            .bind(t.transaction_type)
            .bind(t.status)
            .bind(t.reference_number)
            .bind(t.authorization_code)
            .bind(t.description)
            .bind(t.receipt_available)
            .bind(t.receipt_path)
            .bind(t.expense_report_id.map(|id| id.to_string()))
            .bind(t.expense_line_id.map(|id| id.to_string()))
            .bind(t.reconciled)
            .bind(t.reconciled_at.map(|d| d.to_string()))
            .bind(t.approved_by.map(|id| id.to_string()))
            .bind(t.approved_at.map(|d| d.to_string()))
            .bind(t.tax_amount)
            .bind(t.tip_amount)
            .bind(t.notes.clone())
            .bind(t.created_at.to_string())
            .execute(&self.pool).await?;
        Ok(tx.clone())
    }
    async fn list_transactions(&self, _card_id: Uuid) -> Result<Vec<CardTransaction>> { Ok(vec![]) }
    async fn create_statement(&self, stmt: &CardStatement) -> Result<CardStatement> {
        let s = stmt.clone();
        sqlx::query(r#"INSERT INTO card_statements (id, statement_number, card_id, statement_date,
            due_date, period_start, period_end, opening_balance, payments, credits, purchases, fees,
            interest, closing_balance, minimum_payment, currency, paid_amount, paid_date, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(s.base.id.to_string())
            .bind(s.statement_number)
            .bind(s.card_id.to_string())
            .bind(s.statement_date.to_string())
            .bind(s.due_date.to_string())
            .bind(s.period_start.to_string())
            .bind(s.period_end.to_string())
            .bind(s.opening_balance)
            .bind(s.payments)
            .bind(s.credits)
            .bind(s.purchases)
            .bind(s.fees)
            .bind(s.interest)
            .bind(s.closing_balance)
            .bind(s.minimum_payment)
            .bind(s.currency)
            .bind(s.paid_amount)
            .bind(s.paid_date.map(|d| d.to_string()))
            .bind(s.status.clone())
            .bind(s.created_at.to_string())
            .execute(&self.pool).await?;
        Ok(stmt.clone())
    }
    async fn create_order(&self, order: &PCardOrder) -> Result<PCardOrder> {
        let o = order.clone();
        sqlx::query(r#"INSERT INTO pcard_orders (id, order_number, card_id, vendor_id, vendor_name,
            order_date, required_date, delivery_date, subtotal, tax, shipping, total, currency,
            po_number, transaction_id, status, approved_by, approved_at, notes, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(o.base.id.to_string())
            .bind(o.order_number)
            .bind(o.card_id.to_string())
            .bind(o.vendor_id.to_string())
            .bind(o.vendor_name)
            .bind(o.order_date.to_string())
            .bind(o.required_date.map(|d| d.to_string()))
            .bind(o.delivery_date.map(|d| d.to_string()))
            .bind(o.subtotal)
            .bind(o.tax)
            .bind(o.shipping)
            .bind(o.total)
            .bind(o.currency)
            .bind(o.po_number)
            .bind(o.transaction_id.map(|id| id.to_string()))
            .bind(o.status)
            .bind(o.approved_by.map(|id| id.to_string()))
            .bind(o.approved_at.map(|d| d.to_string()))
            .bind(o.notes.clone())
            .bind(o.created_at.to_string())
            .bind(o.updated_at.to_string())
            .execute(&self.pool).await?;
        Ok(order.clone())
    }
    async fn create_policy(&self, policy: &CardPolicy) -> Result<CardPolicy> {
        let p = policy.clone();
        sqlx::query(r#"INSERT INTO card_policies (id, policy_number, name, description, card_type,
            default_limit, daily_limit, transaction_limit, mcc_allowed, mcc_blocked, merchant_blocked,
            requires_approval_over, approval_workflow_id, requires_receipt_over, auto_reconcile,
            international_allowed, atm_allowed, online_allowed, contactless_allowed, status,
            created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(p.base.id.to_string())
            .bind(p.policy_number)
            .bind(p.name)
            .bind(p.description)
            .bind(p.card_type)
            .bind(p.default_limit)
            .bind(p.daily_limit)
            .bind(p.transaction_limit)
            .bind(p.mcc_allowed)
            .bind(p.mcc_blocked)
            .bind(p.merchant_blocked)
            .bind(p.requires_approval_over)
            .bind(p.approval_workflow_id.map(|id| id.to_string()))
            .bind(p.requires_receipt_over)
            .bind(p.auto_reconcile)
            .bind(p.international_allowed)
            .bind(p.atm_allowed)
            .bind(p.online_allowed)
            .bind(p.contactless_allowed)
            .bind(p.status.clone())
            .bind(p.created_at.to_string())
            .bind(p.updated_at.to_string())
            .execute(&self.pool).await?;
        Ok(policy.clone())
    }
    async fn create_dispute(&self, dispute: &CardDispute) -> Result<CardDispute> {
        let d = dispute.clone();
        sqlx::query(r#"INSERT INTO card_disputes (id, dispute_number, card_id, transaction_id,
            dispute_type, dispute_reason, disputed_amount, currency, filed_date, resolution_date,
            resolution, provisional_credit, provisional_credit_date, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(d.base.id.to_string())
            .bind(d.dispute_number)
            .bind(d.card_id.to_string())
            .bind(d.transaction_id.to_string())
            .bind(d.dispute_type)
            .bind(d.dispute_reason)
            .bind(d.disputed_amount)
            .bind(d.currency)
            .bind(d.filed_date.to_string())
            .bind(d.resolution_date.map(|d| d.to_string()))
            .bind(d.resolution)
            .bind(d.provisional_credit)
            .bind(d.provisional_credit_date.map(|d| d.to_string()))
            .bind(d.status.clone())
            .bind(d.created_at.to_string())
            .bind(d.updated_at.to_string())
            .execute(&self.pool).await?;
        Ok(dispute.clone())
    }
    async fn create_virtual_card(&self, card: &VirtualCard) -> Result<VirtualCard> {
        let c = card.clone();
        sqlx::query(r#"INSERT INTO virtual_cards (id, parent_card_id, cardholder_id, masked_number,
            credit_limit, available_credit, currency, valid_from, valid_until, single_use,
            merchant_lock, usage_limit, usage_count, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(c.base.id.to_string())
            .bind(c.parent_card_id.map(|id| id.to_string()))
            .bind(c.cardholder_id.to_string())
            .bind(c.masked_number)
            .bind(c.credit_limit)
            .bind(c.available_credit)
            .bind(c.currency)
            .bind(c.valid_from.to_string())
            .bind(c.valid_until.to_string())
            .bind(c.single_use)
            .bind(c.merchant_lock)
            .bind(c.usage_limit)
            .bind(c.usage_count)
            .bind(c.status.clone())
            .bind(c.created_at.to_string())
            .execute(&self.pool).await?;
        Ok(card.clone())
    }
}
