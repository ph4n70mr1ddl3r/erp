use crate::models::*;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait TPMRepository {
    async fn create_promotion(&self, promo: &TradePromotion) -> anyhow::Result<()>;
    async fn get_promotion(&self, id: Uuid) -> anyhow::Result<Option<TradePromotion>>;
    async fn list_promotions(&self, status: Option<PromotionStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<TradePromotion>>;
    async fn update_promotion(&self, promo: &TradePromotion) -> anyhow::Result<()>;
    
    async fn create_promotion_product(&self, pp: &PromotionProduct) -> anyhow::Result<()>;
    async fn list_promotion_products(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionProduct>>;
    
    async fn create_promotion_customer(&self, pc: &PromotionCustomer) -> anyhow::Result<()>;
    async fn list_promotion_customers(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionCustomer>>;
    
    async fn create_trade_fund(&self, fund: &TradeFund) -> anyhow::Result<()>;
    async fn get_trade_fund(&self, id: Uuid) -> anyhow::Result<Option<TradeFund>>;
    async fn list_trade_funds(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<TradeFund>>;
    async fn update_trade_fund(&self, fund: &TradeFund) -> anyhow::Result<()>;
    
    async fn create_fund_transaction(&self, txn: &TradeFundTransaction) -> anyhow::Result<()>;
    async fn list_fund_transactions(&self, fund_id: Uuid) -> anyhow::Result<Vec<TradeFundTransaction>>;
    
    async fn create_rebate_agreement(&self, agreement: &RebateAgreement) -> anyhow::Result<()>;
    async fn get_rebate_agreement(&self, id: Uuid) -> anyhow::Result<Option<RebateAgreement>>;
    async fn list_rebate_agreements(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<RebateAgreement>>;
    async fn update_rebate_agreement(&self, agreement: &RebateAgreement) -> anyhow::Result<()>;
    
    async fn create_rebate_tier(&self, tier: &RebateTier) -> anyhow::Result<()>;
    async fn list_rebate_tiers(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateTier>>;
    
    async fn create_rebate_product(&self, rp: &RebateProduct) -> anyhow::Result<()>;
    async fn list_rebate_products(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateProduct>>;
    
    async fn create_rebate_accrual(&self, accrual: &RebateAccrual) -> anyhow::Result<()>;
    async fn list_rebate_accruals(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateAccrual>>;
    async fn update_rebate_accrual(&self, accrual: &RebateAccrual) -> anyhow::Result<()>;
    
    async fn create_rebate_payment(&self, payment: &RebatePayment) -> anyhow::Result<()>;
    async fn get_rebate_payment(&self, id: Uuid) -> anyhow::Result<Option<RebatePayment>>;
    async fn list_rebate_payments(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebatePayment>>;
    async fn update_rebate_payment(&self, payment: &RebatePayment) -> anyhow::Result<()>;
    
    async fn create_rebate_payment_line(&self, line: &RebatePaymentLine) -> anyhow::Result<()>;
    async fn list_rebate_payment_lines(&self, payment_id: Uuid) -> anyhow::Result<Vec<RebatePaymentLine>>;
    
    async fn create_chargeback(&self, cb: &Chargeback) -> anyhow::Result<()>;
    async fn get_chargeback(&self, id: Uuid) -> anyhow::Result<Option<Chargeback>>;
    async fn list_chargebacks(&self, customer_id: Option<Uuid>, status: Option<ClaimStatus>) -> anyhow::Result<Vec<Chargeback>>;
    async fn update_chargeback(&self, cb: &Chargeback) -> anyhow::Result<()>;
    
    async fn create_chargeback_line(&self, line: &ChargebackLine) -> anyhow::Result<()>;
    async fn list_chargeback_lines(&self, chargeback_id: Uuid) -> anyhow::Result<Vec<ChargebackLine>>;
    
    async fn create_chargeback_document(&self, doc: &ChargebackDocument) -> anyhow::Result<()>;
    async fn list_chargeback_documents(&self, chargeback_id: Uuid) -> anyhow::Result<Vec<ChargebackDocument>>;
    
    async fn create_promotion_performance(&self, perf: &PromotionPerformance) -> anyhow::Result<()>;
    async fn list_promotion_performance(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionPerformance>>;
    
    async fn create_promotion_plan(&self, plan: &PromotionPlan) -> anyhow::Result<()>;
    async fn get_promotion_plan(&self, id: Uuid) -> anyhow::Result<Option<PromotionPlan>>;
    async fn list_promotion_plans(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<PromotionPlan>>;
    async fn update_promotion_plan(&self, plan: &PromotionPlan) -> anyhow::Result<()>;
    
    async fn create_customer_trade_profile(&self, profile: &CustomerTradeProfile) -> anyhow::Result<()>;
    async fn get_customer_trade_profile(&self, customer_id: Uuid) -> anyhow::Result<Option<CustomerTradeProfile>>;
    async fn update_customer_trade_profile(&self, profile: &CustomerTradeProfile) -> anyhow::Result<()>;
    
    async fn create_ship_and_debit(&self, sad: &ShipAndDebit) -> anyhow::Result<()>;
    async fn list_ship_and_debits(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<ShipAndDebit>>;
    async fn update_ship_and_debit(&self, sad: &ShipAndDebit) -> anyhow::Result<()>;
    
    async fn create_price_protection(&self, pp: &PriceProtection) -> anyhow::Result<()>;
    async fn list_price_protections(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<PriceProtection>>;
    async fn update_price_protection(&self, pp: &PriceProtection) -> anyhow::Result<()>;
}

pub struct SqliteTPMRepository {
    pool: SqlitePool,
}

impl SqliteTPMRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TPMRepository for SqliteTPMRepository {
    async fn create_promotion(&self, promo: &TradePromotion) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_promotions (id, promotion_number, name, description, promotion_type,
                status, customer_id, customer_group_id, product_id, product_group_id, start_date, end_date,
                planned_budget, committed_budget, spent_budget, accrued_budget, currency, discount_percent,
                discount_amount, buy_quantity, get_quantity, max_redemptions, redemptions_count,
                forecasted_sales, actual_sales, roi, owner_id, approval_status, approved_by, approved_at,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            promo.id, promo.promotion_number, promo.name, promo.description, promo.promotion_type as _,
            promo.status as _, promo.customer_id, promo.customer_group_id, promo.product_id, promo.product_group_id,
            promo.start_date, promo.end_date, promo.planned_budget, promo.committed_budget, promo.spent_budget,
            promo.accrued_budget, promo.currency, promo.discount_percent, promo.discount_amount, promo.buy_quantity,
            promo.get_quantity, promo.max_redemptions, promo.redemptions_count, promo.forecasted_sales,
            promo.actual_sales, promo.roi, promo.owner_id, promo.approval_status, promo.approved_by,
            promo.approved_at, promo.created_at, promo.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_promotion(&self, id: Uuid) -> anyhow::Result<Option<TradePromotion>> {
        let promo = sqlx::query_as!(
            TradePromotion,
            r#"SELECT id, promotion_number, name, description, promotion_type as "promotion_type: _",
                status as "status: _", customer_id, customer_group_id, product_id, product_group_id,
                start_date, end_date, planned_budget, committed_budget, spent_budget, accrued_budget,
                currency, discount_percent, discount_amount, buy_quantity, get_quantity, max_redemptions,
                redemptions_count, forecasted_sales, actual_sales, roi, owner_id, approval_status,
                approved_by, approved_at, created_at, updated_at FROM tpm_promotions WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(promo)
    }

    async fn list_promotions(&self, status: Option<PromotionStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<TradePromotion>> {
        let promos = if let Some(s) = status {
            sqlx::query_as!(
                TradePromotion,
                r#"SELECT id, promotion_number, name, description, promotion_type as "promotion_type: _",
                    status as "status: _", customer_id, customer_group_id, product_id, product_group_id,
                    start_date, end_date, planned_budget, committed_budget, spent_budget, accrued_budget,
                    currency, discount_percent, discount_amount, buy_quantity, get_quantity, max_redemptions,
                    redemptions_count, forecasted_sales, actual_sales, roi, owner_id, approval_status,
                    approved_by, approved_at, created_at, updated_at FROM tpm_promotions
                    WHERE status = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
                    s as _, limit, offset
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                TradePromotion,
                r#"SELECT id, promotion_number, name, description, promotion_type as "promotion_type: _",
                    status as "status: _", customer_id, customer_group_id, product_id, product_group_id,
                    start_date, end_date, planned_budget, committed_budget, spent_budget, accrued_budget,
                    currency, discount_percent, discount_amount, buy_quantity, get_quantity, max_redemptions,
                    redemptions_count, forecasted_sales, actual_sales, roi, owner_id, approval_status,
                    approved_by, approved_at, created_at, updated_at FROM tpm_promotions
                    ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
                    limit, offset
            ).fetch_all(&self.pool).await?
        };
        Ok(promos)
    }

    async fn update_promotion(&self, promo: &TradePromotion) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_promotions SET status = ?, committed_budget = ?, spent_budget = ?,
                accrued_budget = ?, redemptions_count = ?, actual_sales = ?, roi = ?,
                approval_status = ?, approved_by = ?, approved_at = ?, updated_at = ? WHERE id = ?"#,
            promo.status as _, promo.committed_budget, promo.spent_budget, promo.accrued_budget,
            promo.redemptions_count, promo.actual_sales, promo.roi, promo.approval_status,
            promo.approved_by, promo.approved_at, promo.updated_at, promo.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_promotion_product(&self, pp: &PromotionProduct) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_promotion_products (id, promotion_id, product_id, discount_percent,
                discount_amount, buy_qty, get_qty, max_qty, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            pp.id, pp.promotion_id, pp.product_id, pp.discount_percent, pp.discount_amount,
            pp.buy_qty, pp.get_qty, pp.max_qty, pp.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_promotion_products(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionProduct>> {
        let products = sqlx::query_as!(
            PromotionProduct,
            r#"SELECT id, promotion_id, product_id, discount_percent, discount_amount, buy_qty, get_qty,
                max_qty, created_at FROM tpm_promotion_products WHERE promotion_id = ?"#,
            promotion_id
        ).fetch_all(&self.pool).await?;
        Ok(products)
    }

    async fn create_promotion_customer(&self, pc: &PromotionCustomer) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_promotion_customers (id, promotion_id, customer_id, customer_group_id,
                territory_id, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
            pc.id, pc.promotion_id, pc.customer_id, pc.customer_group_id, pc.territory_id, pc.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_promotion_customers(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionCustomer>> {
        let customers = sqlx::query_as!(
            PromotionCustomer,
            r#"SELECT id, promotion_id, customer_id, customer_group_id, territory_id, created_at
                FROM tpm_promotion_customers WHERE promotion_id = ?"#,
            promotion_id
        ).fetch_all(&self.pool).await?;
        Ok(customers)
    }

    async fn create_trade_fund(&self, fund: &TradeFund) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_trade_funds (id, fund_number, name, fund_type, customer_id, fiscal_year,
                total_budget, committed_amount, spent_amount, available_amount, currency, start_date,
                end_date, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            fund.id, fund.fund_number, fund.name, fund.fund_type as _, fund.customer_id, fund.fiscal_year,
            fund.total_budget, fund.committed_amount, fund.spent_amount, fund.available_amount,
            fund.currency, fund.start_date, fund.end_date, fund.is_active, fund.created_at, fund.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_trade_fund(&self, id: Uuid) -> anyhow::Result<Option<TradeFund>> {
        let fund = sqlx::query_as!(
            TradeFund,
            r#"SELECT id, fund_number, name, fund_type as "fund_type: _", customer_id, fiscal_year,
                total_budget, committed_amount, spent_amount, available_amount, currency, start_date,
                end_date, is_active, created_at, updated_at FROM tpm_trade_funds WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(fund)
    }

    async fn list_trade_funds(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<TradeFund>> {
        let funds = sqlx::query_as!(
            TradeFund,
            r#"SELECT id, fund_number, name, fund_type as "fund_type: _", customer_id, fiscal_year,
                total_budget, committed_amount, spent_amount, available_amount, currency, start_date,
                end_date, is_active, created_at, updated_at FROM tpm_trade_funds ORDER BY created_at DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(funds)
    }

    async fn update_trade_fund(&self, fund: &TradeFund) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_trade_funds SET committed_amount = ?, spent_amount = ?, available_amount = ?,
                is_active = ?, updated_at = ? WHERE id = ?"#,
            fund.committed_amount, fund.spent_amount, fund.available_amount, fund.is_active,
            fund.updated_at, fund.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_fund_transaction(&self, txn: &TradeFundTransaction) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_fund_transactions (id, fund_id, promotion_id, transaction_type, amount,
                currency, reference_number, description, transaction_date, created_by, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            txn.id, txn.fund_id, txn.promotion_id, txn.transaction_type, txn.amount, txn.currency,
            txn.reference_number, txn.description, txn.transaction_date, txn.created_by, txn.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_fund_transactions(&self, fund_id: Uuid) -> anyhow::Result<Vec<TradeFundTransaction>> {
        let txns = sqlx::query_as!(
            TradeFundTransaction,
            r#"SELECT id, fund_id, promotion_id, transaction_type, amount, currency, reference_number,
                description, transaction_date, created_by, created_at FROM tpm_fund_transactions
                WHERE fund_id = ? ORDER BY transaction_date DESC"#,
            fund_id
        ).fetch_all(&self.pool).await?;
        Ok(txns)
    }

    async fn create_rebate_agreement(&self, agreement: &RebateAgreement) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_rebate_agreements (id, agreement_number, name, customer_id, agreement_type,
                start_date, end_date, basis, calculation_method, payment_terms, status, total_eligible_sales,
                total_rebate_earned, total_rebate_paid, currency, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            agreement.id, agreement.agreement_number, agreement.name, agreement.customer_id,
            agreement.agreement_type, agreement.start_date, agreement.end_date, agreement.basis,
            agreement.calculation_method, agreement.payment_terms, agreement.status as _,
            agreement.total_eligible_sales, agreement.total_rebate_earned, agreement.total_rebate_paid,
            agreement.currency, agreement.notes, agreement.created_at, agreement.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_rebate_agreement(&self, id: Uuid) -> anyhow::Result<Option<RebateAgreement>> {
        let agreement = sqlx::query_as!(
            RebateAgreement,
            r#"SELECT id, agreement_number, name, customer_id, agreement_type, start_date, end_date,
                basis, calculation_method, payment_terms, status as "status: _", total_eligible_sales,
                total_rebate_earned, total_rebate_paid, currency, notes, created_at, updated_at
                FROM tpm_rebate_agreements WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(agreement)
    }

    async fn list_rebate_agreements(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<RebateAgreement>> {
        let agreements = sqlx::query_as!(
            RebateAgreement,
            r#"SELECT id, agreement_number, name, customer_id, agreement_type, start_date, end_date,
                basis, calculation_method, payment_terms, status as "status: _", total_eligible_sales,
                total_rebate_earned, total_rebate_paid, currency, notes, created_at, updated_at
                FROM tpm_rebate_agreements ORDER BY created_at DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(agreements)
    }

    async fn update_rebate_agreement(&self, agreement: &RebateAgreement) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_rebate_agreements SET total_eligible_sales = ?, total_rebate_earned = ?,
                total_rebate_paid = ?, status = ?, updated_at = ? WHERE id = ?"#,
            agreement.total_eligible_sales, agreement.total_rebate_earned, agreement.total_rebate_paid,
            agreement.status as _, agreement.updated_at, agreement.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_rebate_tier(&self, tier: &RebateTier) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_rebate_tiers (id, agreement_id, tier_number, min_quantity, max_quantity,
                min_value, max_value, rebate_percent, rebate_amount, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            tier.id, tier.agreement_id, tier.tier_number, tier.min_quantity, tier.max_quantity,
            tier.min_value, tier.max_value, tier.rebate_percent, tier.rebate_amount, tier.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_rebate_tiers(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateTier>> {
        let tiers = sqlx::query_as!(
            RebateTier,
            r#"SELECT id, agreement_id, tier_number, min_quantity, max_quantity, min_value, max_value,
                rebate_percent, rebate_amount, created_at FROM tpm_rebate_tiers WHERE agreement_id = ?
                ORDER BY tier_number"#,
            agreement_id
        ).fetch_all(&self.pool).await?;
        Ok(tiers)
    }

    async fn create_rebate_product(&self, rp: &RebateProduct) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_rebate_products (id, agreement_id, product_id, product_group_id,
                specific_rate, specific_amount, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            rp.id, rp.agreement_id, rp.product_id, rp.product_group_id, rp.specific_rate,
            rp.specific_amount, rp.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_rebate_products(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateProduct>> {
        let products = sqlx::query_as!(
            RebateProduct,
            r#"SELECT id, agreement_id, product_id, product_group_id, specific_rate, specific_amount,
                created_at FROM tpm_rebate_products WHERE agreement_id = ?"#,
            agreement_id
        ).fetch_all(&self.pool).await?;
        Ok(products)
    }

    async fn create_rebate_accrual(&self, accrual: &RebateAccrual) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_rebate_accruals (id, agreement_id, sales_order_id, invoice_id,
                product_id, sales_amount, rebate_rate, rebate_amount, currency, accrual_date, status,
                paid_amount, remaining_amount, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            accrual.id, accrual.agreement_id, accrual.sales_order_id, accrual.invoice_id, accrual.product_id,
            accrual.sales_amount, accrual.rebate_rate, accrual.rebate_amount, accrual.currency,
            accrual.accrual_date, accrual.status, accrual.paid_amount, accrual.remaining_amount, accrual.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_rebate_accruals(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateAccrual>> {
        let accruals = sqlx::query_as!(
            RebateAccrual,
            r#"SELECT id, agreement_id, sales_order_id, invoice_id, product_id, sales_amount,
                rebate_rate, rebate_amount, currency, accrual_date, status, paid_amount, remaining_amount,
                created_at FROM tpm_rebate_accruals WHERE agreement_id = ? ORDER BY accrual_date DESC"#,
            agreement_id
        ).fetch_all(&self.pool).await?;
        Ok(accruals)
    }

    async fn update_rebate_accrual(&self, accrual: &RebateAccrual) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_rebate_accruals SET status = ?, paid_amount = ?, remaining_amount = ? WHERE id = ?"#,
            accrual.status, accrual.paid_amount, accrual.remaining_amount, accrual.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_rebate_payment(&self, payment: &RebatePayment) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_rebate_payments (id, payment_number, agreement_id, customer_id,
                payment_date, period_start, period_end, total_amount, currency, payment_method,
                reference_number, status, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            payment.id, payment.payment_number, payment.agreement_id, payment.customer_id,
            payment.payment_date, payment.period_start, payment.period_end, payment.total_amount,
            payment.currency, payment.payment_method, payment.reference_number, payment.status,
            payment.notes, payment.created_at, payment.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_rebate_payment(&self, id: Uuid) -> anyhow::Result<Option<RebatePayment>> {
        let payment = sqlx::query_as!(
            RebatePayment,
            r#"SELECT id, payment_number, agreement_id, customer_id, payment_date, period_start,
                period_end, total_amount, currency, payment_method, reference_number, status, notes,
                created_at, updated_at FROM tpm_rebate_payments WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(payment)
    }

    async fn list_rebate_payments(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebatePayment>> {
        let payments = sqlx::query_as!(
            RebatePayment,
            r#"SELECT id, payment_number, agreement_id, customer_id, payment_date, period_start,
                period_end, total_amount, currency, payment_method, reference_number, status, notes,
                created_at, updated_at FROM tpm_rebate_payments WHERE agreement_id = ?
                ORDER BY payment_date DESC"#,
            agreement_id
        ).fetch_all(&self.pool).await?;
        Ok(payments)
    }

    async fn update_rebate_payment(&self, payment: &RebatePayment) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_rebate_payments SET status = ?, notes = ?, updated_at = ? WHERE id = ?"#,
            payment.status, payment.notes, payment.updated_at, payment.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_rebate_payment_line(&self, line: &RebatePaymentLine) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_rebate_payment_lines (id, payment_id, accrual_id, amount, currency, created_at)
                VALUES (?, ?, ?, ?, ?, ?)"#,
            line.id, line.payment_id, line.accrual_id, line.amount, line.currency, line.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_rebate_payment_lines(&self, payment_id: Uuid) -> anyhow::Result<Vec<RebatePaymentLine>> {
        let lines = sqlx::query_as!(
            RebatePaymentLine,
            r#"SELECT id, payment_id, accrual_id, amount, currency, created_at
                FROM tpm_rebate_payment_lines WHERE payment_id = ?"#,
            payment_id
        ).fetch_all(&self.pool).await?;
        Ok(lines)
    }

    async fn create_chargeback(&self, cb: &Chargeback) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_chargebacks (id, chargeback_number, customer_id, invoice_id, promotion_id,
                chargeback_date, amount_claimed, amount_approved, amount_rejected, currency, status,
                claim_type, description, rejection_reason, submitted_by, reviewed_by, reviewed_at,
                paid_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            cb.id, cb.chargeback_number, cb.customer_id, cb.invoice_id, cb.promotion_id, cb.chargeback_date,
            cb.amount_claimed, cb.amount_approved, cb.amount_rejected, cb.currency, cb.status as _,
            cb.claim_type, cb.description, cb.rejection_reason, cb.submitted_by, cb.reviewed_by,
            cb.reviewed_at, cb.paid_at, cb.created_at, cb.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_chargeback(&self, id: Uuid) -> anyhow::Result<Option<Chargeback>> {
        let cb = sqlx::query_as!(
            Chargeback,
            r#"SELECT id, chargeback_number, customer_id, invoice_id, promotion_id, chargeback_date,
                amount_claimed, amount_approved, amount_rejected, currency, status as "status: _",
                claim_type, description, rejection_reason, submitted_by, reviewed_by, reviewed_at,
                paid_at, created_at, updated_at FROM tpm_chargebacks WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(cb)
    }

    async fn list_chargebacks(&self, customer_id: Option<Uuid>, status: Option<ClaimStatus>) -> anyhow::Result<Vec<Chargeback>> {
        let cbs = sqlx::query_as!(
            Chargeback,
            r#"SELECT id, chargeback_number, customer_id, invoice_id, promotion_id, chargeback_date,
                amount_claimed, amount_approved, amount_rejected, currency, status as "status: _",
                claim_type, description, rejection_reason, submitted_by, reviewed_by, reviewed_at,
                paid_at, created_at, updated_at FROM tpm_chargebacks ORDER BY created_at DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(cbs)
    }

    async fn update_chargeback(&self, cb: &Chargeback) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_chargebacks SET amount_approved = ?, amount_rejected = ?, status = ?,
                rejection_reason = ?, reviewed_by = ?, reviewed_at = ?, paid_at = ?, updated_at = ?
                WHERE id = ?"#,
            cb.amount_approved, cb.amount_rejected, cb.status as _, cb.rejection_reason,
            cb.reviewed_by, cb.reviewed_at, cb.paid_at, cb.updated_at, cb.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_chargeback_line(&self, line: &ChargebackLine) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_chargeback_lines (id, chargeback_id, product_id, quantity, unit_price,
                claimed_amount, approved_amount, rejected_amount, currency, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            line.id, line.chargeback_id, line.product_id, line.quantity, line.unit_price,
            line.claimed_amount, line.approved_amount, line.rejected_amount, line.currency, line.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_chargeback_lines(&self, chargeback_id: Uuid) -> anyhow::Result<Vec<ChargebackLine>> {
        let lines = sqlx::query_as!(
            ChargebackLine,
            r#"SELECT id, chargeback_id, product_id, quantity, unit_price, claimed_amount,
                approved_amount, rejected_amount, currency, created_at FROM tpm_chargeback_lines
                WHERE chargeback_id = ?"#,
            chargeback_id
        ).fetch_all(&self.pool).await?;
        Ok(lines)
    }

    async fn create_chargeback_document(&self, doc: &ChargebackDocument) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_chargeback_documents (id, chargeback_id, document_type, file_name,
                file_path, uploaded_by, uploaded_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            doc.id, doc.chargeback_id, doc.document_type, doc.file_name, doc.file_path,
            doc.uploaded_by, doc.uploaded_at, doc.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_chargeback_documents(&self, chargeback_id: Uuid) -> anyhow::Result<Vec<ChargebackDocument>> {
        let docs = sqlx::query_as!(
            ChargebackDocument,
            r#"SELECT id, chargeback_id, document_type, file_name, file_path, uploaded_by,
                uploaded_at, created_at FROM tpm_chargeback_documents WHERE chargeback_id = ?"#,
            chargeback_id
        ).fetch_all(&self.pool).await?;
        Ok(docs)
    }

    async fn create_promotion_performance(&self, perf: &PromotionPerformance) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_promotion_performance (id, promotion_id, period_start, period_end,
                baseline_sales, incremental_sales, total_sales, units_sold, promotion_cost, roi_percent,
                lift_percent, cannibalization, forward_buy, currency, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            perf.id, perf.promotion_id, perf.period_start, perf.period_end, perf.baseline_sales,
            perf.incremental_sales, perf.total_sales, perf.units_sold, perf.promotion_cost, perf.roi_percent,
            perf.lift_percent, perf.cannibalization, perf.forward_buy, perf.currency, perf.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_promotion_performance(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionPerformance>> {
        let perfs = sqlx::query_as!(
            PromotionPerformance,
            r#"SELECT id, promotion_id, period_start, period_end, baseline_sales, incremental_sales,
                total_sales, units_sold, promotion_cost, roi_percent, lift_percent, cannibalization,
                forward_buy, currency, created_at FROM tpm_promotion_performance WHERE promotion_id = ?
                ORDER BY period_start DESC"#,
            promotion_id
        ).fetch_all(&self.pool).await?;
        Ok(perfs)
    }

    async fn create_promotion_plan(&self, plan: &PromotionPlan) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_promotion_plans (id, plan_number, name, fiscal_year, customer_id,
                customer_group_id, total_budget, allocated_budget, spent_budget, remaining_budget,
                currency, status, owner_id, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            plan.id, plan.plan_number, plan.name, plan.fiscal_year, plan.customer_id, plan.customer_group_id,
            plan.total_budget, plan.allocated_budget, plan.spent_budget, plan.remaining_budget,
            plan.currency, plan.status, plan.owner_id, plan.created_at, plan.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_promotion_plan(&self, id: Uuid) -> anyhow::Result<Option<PromotionPlan>> {
        let plan = sqlx::query_as!(
            PromotionPlan,
            r#"SELECT id, plan_number, name, fiscal_year, customer_id, customer_group_id, total_budget,
                allocated_budget, spent_budget, remaining_budget, currency, status, owner_id,
                created_at, updated_at FROM tpm_promotion_plans WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(plan)
    }

    async fn list_promotion_plans(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<PromotionPlan>> {
        let plans = sqlx::query_as!(
            PromotionPlan,
            r#"SELECT id, plan_number, name, fiscal_year, customer_id, customer_group_id, total_budget,
                allocated_budget, spent_budget, remaining_budget, currency, status, owner_id,
                created_at, updated_at FROM tpm_promotion_plans ORDER BY fiscal_year DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(plans)
    }

    async fn update_promotion_plan(&self, plan: &PromotionPlan) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_promotion_plans SET allocated_budget = ?, spent_budget = ?, remaining_budget = ?,
                status = ?, updated_at = ? WHERE id = ?"#,
            plan.allocated_budget, plan.spent_budget, plan.remaining_budget, plan.status,
            plan.updated_at, plan.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_customer_trade_profile(&self, profile: &CustomerTradeProfile) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_customer_trade_profiles (id, customer_id, trade_class, annual_volume,
                growth_rate, avg_promotion_response, preferred_promotion_type, credit_limit,
                payment_terms, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            profile.id, profile.customer_id, profile.trade_class, profile.annual_volume, profile.growth_rate,
            profile.avg_promotion_response, profile.preferred_promotion_type, profile.credit_limit,
            profile.payment_terms, profile.notes, profile.created_at, profile.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_customer_trade_profile(&self, customer_id: Uuid) -> anyhow::Result<Option<CustomerTradeProfile>> {
        let profile = sqlx::query_as!(
            CustomerTradeProfile,
            r#"SELECT id, customer_id, trade_class, annual_volume, growth_rate, avg_promotion_response,
                preferred_promotion_type, credit_limit, payment_terms, notes, created_at, updated_at
                FROM tpm_customer_trade_profiles WHERE customer_id = ?"#,
            customer_id
        ).fetch_optional(&self.pool).await?;
        Ok(profile)
    }

    async fn update_customer_trade_profile(&self, profile: &CustomerTradeProfile) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_customer_trade_profiles SET trade_class = ?, annual_volume = ?, growth_rate = ?,
                avg_promotion_response = ?, preferred_promotion_type = ?, credit_limit = ?, payment_terms = ?,
                notes = ?, updated_at = ? WHERE id = ?"#,
            profile.trade_class, profile.annual_volume, profile.growth_rate, profile.avg_promotion_response,
            profile.preferred_promotion_type, profile.credit_limit, profile.payment_terms, profile.notes,
            profile.updated_at, profile.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_ship_and_debit(&self, sad: &ShipAndDebit) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_ship_and_debits (id, sad_number, customer_id, product_id, authorized_price,
                list_price, authorized_discount, quantity_authorized, quantity_shipped, quantity_debited,
                currency, start_date, end_date, status, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            sad.id, sad.sad_number, sad.customer_id, sad.product_id, sad.authorized_price, sad.list_price,
            sad.authorized_discount, sad.quantity_authorized, sad.quantity_shipped, sad.quantity_debited,
            sad.currency, sad.start_date, sad.end_date, sad.status, sad.created_at, sad.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_ship_and_debits(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<ShipAndDebit>> {
        let sads = sqlx::query_as!(
            ShipAndDebit,
            r#"SELECT id, sad_number, customer_id, product_id, authorized_price, list_price,
                authorized_discount, quantity_authorized, quantity_shipped, quantity_debited,
                currency, start_date, end_date, status, created_at, updated_at
                FROM tpm_ship_and_debits ORDER BY created_at DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(sads)
    }

    async fn update_ship_and_debit(&self, sad: &ShipAndDebit) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_ship_and_debits SET quantity_shipped = ?, quantity_debited = ?, status = ?,
                updated_at = ? WHERE id = ?"#,
            sad.quantity_shipped, sad.quantity_debited, sad.status, sad.updated_at, sad.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_price_protection(&self, pp: &PriceProtection) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO tpm_price_protections (id, pp_number, customer_id, product_id, product_group_id,
                old_price, new_price, price_reduction, effective_date, inventory_on_hand, claim_amount,
                approved_amount, currency, status, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            pp.id, pp.pp_number, pp.customer_id, pp.product_id, pp.product_group_id, pp.old_price,
            pp.new_price, pp.price_reduction, pp.effective_date, pp.inventory_on_hand, pp.claim_amount,
            pp.approved_amount, pp.currency, pp.status as _, pp.notes, pp.created_at, pp.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_price_protections(&self, customer_id: Option<Uuid>) -> anyhow::Result<Vec<PriceProtection>> {
        let pps = sqlx::query_as!(
            PriceProtection,
            r#"SELECT id, pp_number, customer_id, product_id, product_group_id, old_price, new_price,
                price_reduction, effective_date, inventory_on_hand, claim_amount, approved_amount,
                currency, status as "status: _", notes, created_at, updated_at
                FROM tpm_price_protections ORDER BY created_at DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(pps)
    }

    async fn update_price_protection(&self, pp: &PriceProtection) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE tpm_price_protections SET approved_amount = ?, status = ?, notes = ?, updated_at = ?
                WHERE id = ?"#,
            pp.approved_amount, pp.status as _, pp.notes, pp.updated_at, pp.id
        ).execute(&self.pool).await?;
        Ok(())
    }
}
