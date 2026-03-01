use crate::models::*;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

fn parse_uuid(s: Option<String>) -> Option<Uuid> {
    s.and_then(|s| Uuid::parse_str(&s).ok())
}

fn parse_uuid_opt(s: Option<String>) -> Option<Uuid> {
    s.and_then(|s| Uuid::parse_str(&s).ok())
}

fn parse_uuid_req(s: String) -> Uuid {
    Uuid::parse_str(&s).unwrap_or_default()
}

fn parse_datetime(s: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn parse_datetime_opt(s: Option<String>) -> Option<DateTime<Utc>> {
    s.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc)))
}

fn to_i32(v: Option<i64>) -> Option<i32> {
    v.map(|v| v as i32)
}

fn to_i32_req(v: i64) -> i32 {
    v as i32
}

fn to_i32_opt_req(v: Option<i64>) -> i32 {
    v.unwrap_or(0) as i32
}

fn to_i64(v: Option<i64>) -> i64 {
    v.unwrap_or(0)
}

fn to_bool(v: Option<i64>) -> bool {
    v.unwrap_or(0) != 0
}

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
        sqlx::query(
            r#"INSERT INTO tpm_promotions (id, promotion_number, name, description, promotion_type,
                status, customer_id, customer_group_id, product_id, product_group_id, start_date, end_date,
                planned_budget, committed_budget, spent_budget, accrued_budget, currency, discount_percent,
                discount_amount, buy_quantity, get_quantity, max_redemptions, redemptions_count,
                forecasted_sales, actual_sales, roi, owner_id, approval_status, approved_by, approved_at,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(promo.id.to_string())
        .bind(&promo.promotion_number)
        .bind(&promo.name)
        .bind(&promo.description)
        .bind(promo.promotion_type.to_string())
        .bind(promo.status.to_string())
        .bind(promo.customer_id.map(|id| id.to_string()))
        .bind(promo.customer_group_id.map(|id| id.to_string()))
        .bind(promo.product_id.map(|id| id.to_string()))
        .bind(promo.product_group_id.map(|id| id.to_string()))
        .bind(promo.start_date.to_rfc3339())
        .bind(promo.end_date.to_rfc3339())
        .bind(promo.planned_budget)
        .bind(promo.committed_budget)
        .bind(promo.spent_budget)
        .bind(promo.accrued_budget)
        .bind(&promo.currency)
        .bind(promo.discount_percent)
        .bind(promo.discount_amount)
        .bind(promo.buy_quantity)
        .bind(promo.get_quantity)
        .bind(promo.max_redemptions)
        .bind(promo.redemptions_count)
        .bind(promo.forecasted_sales)
        .bind(promo.actual_sales)
        .bind(promo.roi)
        .bind(promo.owner_id.map(|id| id.to_string()))
        .bind(&promo.approval_status)
        .bind(promo.approved_by.map(|id| id.to_string()))
        .bind(promo.approved_at.map(|dt| dt.to_rfc3339()))
        .bind(promo.created_at.to_rfc3339())
        .bind(promo.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_promotion(&self, id: Uuid) -> anyhow::Result<Option<TradePromotion>> {
        let row = sqlx::query(
            r#"SELECT id, promotion_number, name, description, promotion_type,
                status, customer_id, customer_group_id, product_id, product_group_id,
                start_date, end_date, planned_budget, committed_budget, spent_budget, accrued_budget,
                currency, discount_percent, discount_amount, buy_quantity, get_quantity, max_redemptions,
                redemptions_count, forecasted_sales, actual_sales, roi, owner_id, approval_status,
                approved_by, approved_at, created_at, updated_at FROM tpm_promotions WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        
        Ok(row.map(|r| TradePromotion {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            promotion_number: r.get(1),
            name: r.get(2),
            description: r.get(3),
            promotion_type: r.get::<String, _>(4).parse().unwrap_or(PromotionType::OffInvoice),
            status: r.get::<String, _>(5).parse().unwrap_or(PromotionStatus::Draft),
            customer_id: parse_uuid(r.get(6)),
            customer_group_id: parse_uuid(r.get(7)),
            product_id: parse_uuid(r.get(8)),
            product_group_id: parse_uuid(r.get(9)),
            start_date: parse_datetime(r.get(10)),
            end_date: parse_datetime(r.get(11)),
            planned_budget: r.get(12),
            committed_budget: to_i64(r.get(13)),
            spent_budget: to_i64(r.get(14)),
            accrued_budget: to_i64(r.get(15)),
            currency: r.get(16),
            discount_percent: r.get(17),
            discount_amount: r.get(18),
            buy_quantity: to_i32(r.get(19)),
            get_quantity: to_i32(r.get(20)),
            max_redemptions: to_i32(r.get(21)),
            redemptions_count: to_i32_opt_req(r.get(22)),
            forecasted_sales: r.get(23),
            actual_sales: r.get(24),
            roi: r.get(25),
            owner_id: parse_uuid(r.get(26)),
            approval_status: r.get(27),
            approved_by: parse_uuid(r.get(28)),
            approved_at: parse_datetime_opt(r.get(29)),
            created_at: parse_datetime(r.get(30)),
            updated_at: parse_datetime(r.get(31)),
        }))
    }

    async fn list_promotions(&self, status: Option<PromotionStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<TradePromotion>> {
        let limit_i64 = limit as i64;
        let offset_i64 = offset as i64;
        
        if let Some(s) = status {
            let status_str = s.to_string();
            let rows = sqlx::query(
                r#"SELECT id, promotion_number, name, description, promotion_type,
                    status, customer_id, customer_group_id, product_id, product_group_id,
                    start_date, end_date, planned_budget, committed_budget, spent_budget, accrued_budget,
                    currency, discount_percent, discount_amount, buy_quantity, get_quantity, max_redemptions,
                    redemptions_count, forecasted_sales, actual_sales, roi, owner_id, approval_status,
                    approved_by, approved_at, created_at, updated_at FROM tpm_promotions
                    WHERE status = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"#
            )
            .bind(status_str)
            .bind(limit_i64)
            .bind(offset_i64)
            .fetch_all(&self.pool).await?;
            
            return Ok(rows.into_iter().map(|r| TradePromotion {
                id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
                promotion_number: r.get(1),
                name: r.get(2),
                description: r.get(3),
                promotion_type: r.get::<String, _>(4).parse().unwrap_or(PromotionType::OffInvoice),
                status: r.get::<String, _>(5).parse().unwrap_or(PromotionStatus::Draft),
                customer_id: parse_uuid(r.get(6)),
                customer_group_id: parse_uuid(r.get(7)),
                product_id: parse_uuid(r.get(8)),
                product_group_id: parse_uuid(r.get(9)),
                start_date: parse_datetime(r.get(10)),
                end_date: parse_datetime(r.get(11)),
                planned_budget: r.get(12),
                committed_budget: to_i64(r.get(13)),
                spent_budget: to_i64(r.get(14)),
                accrued_budget: to_i64(r.get(15)),
                currency: r.get(16),
                discount_percent: r.get(17),
                discount_amount: r.get(18),
                buy_quantity: to_i32(r.get(19)),
                get_quantity: to_i32(r.get(20)),
                max_redemptions: to_i32(r.get(21)),
                redemptions_count: to_i32_opt_req(r.get(22)),
                forecasted_sales: r.get(23),
                actual_sales: r.get(24),
                roi: r.get(25),
                owner_id: parse_uuid(r.get(26)),
                approval_status: r.get(27),
                approved_by: parse_uuid(r.get(28)),
                approved_at: parse_datetime_opt(r.get(29)),
                created_at: parse_datetime(r.get(30)),
                updated_at: parse_datetime(r.get(31)),
            }).collect());
        }
        
        let rows = sqlx::query(
            r#"SELECT id, promotion_number, name, description, promotion_type,
                status, customer_id, customer_group_id, product_id, product_group_id,
                start_date, end_date, planned_budget, committed_budget, spent_budget, accrued_budget,
                currency, discount_percent, discount_amount, buy_quantity, get_quantity, max_redemptions,
                redemptions_count, forecasted_sales, actual_sales, roi, owner_id, approval_status,
                approved_by, approved_at, created_at, updated_at FROM tpm_promotions
                ORDER BY created_at DESC LIMIT ? OFFSET ?"#
        )
        .bind(limit_i64)
        .bind(offset_i64)
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| TradePromotion {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            promotion_number: r.get(1),
            name: r.get(2),
            description: r.get(3),
            promotion_type: r.get::<String, _>(4).parse().unwrap_or(PromotionType::OffInvoice),
            status: r.get::<String, _>(5).parse().unwrap_or(PromotionStatus::Draft),
            customer_id: parse_uuid(r.get(6)),
            customer_group_id: parse_uuid(r.get(7)),
            product_id: parse_uuid(r.get(8)),
            product_group_id: parse_uuid(r.get(9)),
            start_date: parse_datetime(r.get(10)),
            end_date: parse_datetime(r.get(11)),
            planned_budget: r.get(12),
            committed_budget: to_i64(r.get(13)),
            spent_budget: to_i64(r.get(14)),
            accrued_budget: to_i64(r.get(15)),
            currency: r.get(16),
            discount_percent: r.get(17),
            discount_amount: r.get(18),
            buy_quantity: to_i32(r.get(19)),
            get_quantity: to_i32(r.get(20)),
            max_redemptions: to_i32(r.get(21)),
            redemptions_count: to_i32_opt_req(r.get(22)),
            forecasted_sales: r.get(23),
            actual_sales: r.get(24),
            roi: r.get(25),
            owner_id: parse_uuid(r.get(26)),
            approval_status: r.get(27),
            approved_by: parse_uuid(r.get(28)),
            approved_at: parse_datetime_opt(r.get(29)),
            created_at: parse_datetime(r.get(30)),
            updated_at: parse_datetime(r.get(31)),
        }).collect())
    }

    async fn update_promotion(&self, promo: &TradePromotion) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_promotions SET status = ?, committed_budget = ?, spent_budget = ?,
                accrued_budget = ?, redemptions_count = ?, actual_sales = ?, roi = ?,
                approval_status = ?, approved_by = ?, approved_at = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(promo.status.to_string())
        .bind(promo.committed_budget)
        .bind(promo.spent_budget)
        .bind(promo.accrued_budget)
        .bind(promo.redemptions_count)
        .bind(promo.actual_sales)
        .bind(promo.roi)
        .bind(&promo.approval_status)
        .bind(promo.approved_by.map(|id| id.to_string()))
        .bind(promo.approved_at.map(|dt| dt.to_rfc3339()))
        .bind(promo.updated_at.to_rfc3339())
        .bind(promo.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_promotion_product(&self, pp: &PromotionProduct) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_promotion_products (id, promotion_id, product_id, discount_percent,
                discount_amount, buy_qty, get_qty, max_qty, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(pp.id.to_string())
        .bind(pp.promotion_id.to_string())
        .bind(pp.product_id.to_string())
        .bind(pp.discount_percent)
        .bind(pp.discount_amount)
        .bind(pp.buy_qty)
        .bind(pp.get_qty)
        .bind(pp.max_qty)
        .bind(pp.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_promotion_products(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionProduct>> {
        let rows = sqlx::query(
            r#"SELECT id, promotion_id, product_id, discount_percent, discount_amount, buy_qty, get_qty,
                max_qty, created_at FROM tpm_promotion_products WHERE promotion_id = ?"#
        )
        .bind(promotion_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| PromotionProduct {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            promotion_id: parse_uuid_req(r.get(1)),
            product_id: parse_uuid_req(r.get(2)),
            discount_percent: r.get(3),
            discount_amount: r.get(4),
            buy_qty: to_i32(r.get(5)),
            get_qty: to_i32(r.get(6)),
            max_qty: to_i32(r.get(7)),
            created_at: parse_datetime(r.get(8)),
        }).collect())
    }

    async fn create_promotion_customer(&self, pc: &PromotionCustomer) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_promotion_customers (id, promotion_id, customer_id, customer_group_id,
                territory_id, created_at) VALUES (?, ?, ?, ?, ?, ?)"#
        )
        .bind(pc.id.to_string())
        .bind(pc.promotion_id.to_string())
        .bind(pc.customer_id.map(|id| id.to_string()))
        .bind(pc.customer_group_id.map(|id| id.to_string()))
        .bind(pc.territory_id.map(|id| id.to_string()))
        .bind(pc.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_promotion_customers(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionCustomer>> {
        let rows = sqlx::query(
            r#"SELECT id, promotion_id, customer_id, customer_group_id, territory_id, created_at
                FROM tpm_promotion_customers WHERE promotion_id = ?"#
        )
        .bind(promotion_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| PromotionCustomer {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            promotion_id: parse_uuid_req(r.get(1)),
            customer_id: parse_uuid(r.get(2)),
            customer_group_id: parse_uuid(r.get(3)),
            territory_id: parse_uuid(r.get(4)),
            created_at: parse_datetime(r.get(5)),
        }).collect())
    }

    async fn create_trade_fund(&self, fund: &TradeFund) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_trade_funds (id, fund_number, name, fund_type, customer_id, fiscal_year,
                total_budget, committed_amount, spent_amount, available_amount, currency, start_date,
                end_date, is_active, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(fund.id.to_string())
        .bind(&fund.fund_number)
        .bind(&fund.name)
        .bind(fund.fund_type.to_string())
        .bind(fund.customer_id.map(|id| id.to_string()))
        .bind(fund.fiscal_year)
        .bind(fund.total_budget)
        .bind(fund.committed_amount)
        .bind(fund.spent_amount)
        .bind(fund.available_amount)
        .bind(&fund.currency)
        .bind(fund.start_date.to_rfc3339())
        .bind(fund.end_date.to_rfc3339())
        .bind(fund.is_active as i64)
        .bind(fund.created_at.to_rfc3339())
        .bind(fund.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_trade_fund(&self, id: Uuid) -> anyhow::Result<Option<TradeFund>> {
        let row = sqlx::query(
            r#"SELECT id, fund_number, name, fund_type, customer_id, fiscal_year,
                total_budget, committed_amount, spent_amount, available_amount, currency, start_date,
                end_date, is_active, created_at, updated_at FROM tpm_trade_funds WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        
        Ok(row.map(|r| TradeFund {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            fund_number: r.get(1),
            name: r.get(2),
            fund_type: r.get::<String, _>(3).parse().unwrap_or(FundType::Other),
            customer_id: parse_uuid(r.get(4)),
            fiscal_year: to_i32_req(r.get(5)),
            total_budget: r.get(6),
            committed_amount: to_i64(r.get(7)),
            spent_amount: to_i64(r.get(8)),
            available_amount: r.get(9),
            currency: r.get(10),
            start_date: parse_datetime(r.get(11)),
            end_date: parse_datetime(r.get(12)),
            is_active: to_bool(r.get(13)),
            created_at: parse_datetime(r.get(14)),
            updated_at: parse_datetime(r.get(15)),
        }))
    }

    async fn list_trade_funds(&self, _customer_id: Option<Uuid>) -> anyhow::Result<Vec<TradeFund>> {
        let rows = sqlx::query(
            r#"SELECT id, fund_number, name, fund_type, customer_id, fiscal_year,
                total_budget, committed_amount, spent_amount, available_amount, currency, start_date,
                end_date, is_active, created_at, updated_at FROM tpm_trade_funds ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| TradeFund {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            fund_number: r.get(1),
            name: r.get(2),
            fund_type: r.get::<String, _>(3).parse().unwrap_or(FundType::Other),
            customer_id: parse_uuid(r.get(4)),
            fiscal_year: to_i32_req(r.get(5)),
            total_budget: r.get(6),
            committed_amount: to_i64(r.get(7)),
            spent_amount: to_i64(r.get(8)),
            available_amount: r.get(9),
            currency: r.get(10),
            start_date: parse_datetime(r.get(11)),
            end_date: parse_datetime(r.get(12)),
            is_active: to_bool(r.get(13)),
            created_at: parse_datetime(r.get(14)),
            updated_at: parse_datetime(r.get(15)),
        }).collect())
    }

    async fn update_trade_fund(&self, fund: &TradeFund) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_trade_funds SET committed_amount = ?, spent_amount = ?, available_amount = ?,
                is_active = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(fund.committed_amount)
        .bind(fund.spent_amount)
        .bind(fund.available_amount)
        .bind(fund.is_active as i64)
        .bind(fund.updated_at.to_rfc3339())
        .bind(fund.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_fund_transaction(&self, txn: &TradeFundTransaction) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_fund_transactions (id, fund_id, promotion_id, transaction_type, amount,
                currency, reference_number, description, transaction_date, created_by, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(txn.id.to_string())
        .bind(txn.fund_id.to_string())
        .bind(txn.promotion_id.map(|id| id.to_string()))
        .bind(&txn.transaction_type)
        .bind(txn.amount)
        .bind(&txn.currency)
        .bind(&txn.reference_number)
        .bind(&txn.description)
        .bind(txn.transaction_date.to_rfc3339())
        .bind(txn.created_by.map(|id| id.to_string()))
        .bind(txn.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_fund_transactions(&self, fund_id: Uuid) -> anyhow::Result<Vec<TradeFundTransaction>> {
        let rows = sqlx::query(
            r#"SELECT id, fund_id, promotion_id, transaction_type, amount, currency, reference_number,
                description, transaction_date, created_by, created_at FROM tpm_fund_transactions
                WHERE fund_id = ? ORDER BY transaction_date DESC"#
        )
        .bind(fund_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| TradeFundTransaction {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            fund_id: parse_uuid_req(r.get(1)),
            promotion_id: parse_uuid(r.get(2)),
            transaction_type: r.get(3),
            amount: r.get(4),
            currency: r.get(5),
            reference_number: r.get(6),
            description: r.get(7),
            transaction_date: parse_datetime(r.get(8)),
            created_by: parse_uuid(r.get(9)),
            created_at: parse_datetime(r.get(10)),
        }).collect())
    }

    async fn create_rebate_agreement(&self, agreement: &RebateAgreement) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_rebate_agreements (id, agreement_number, name, customer_id, agreement_type,
                start_date, end_date, basis, calculation_method, payment_terms, status, total_eligible_sales,
                total_rebate_earned, total_rebate_paid, currency, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(agreement.id.to_string())
        .bind(&agreement.agreement_number)
        .bind(&agreement.name)
        .bind(agreement.customer_id.to_string())
        .bind(&agreement.agreement_type)
        .bind(agreement.start_date.to_rfc3339())
        .bind(agreement.end_date.to_rfc3339())
        .bind(&agreement.basis)
        .bind(&agreement.calculation_method)
        .bind(&agreement.payment_terms)
        .bind(agreement.status.to_string())
        .bind(agreement.total_eligible_sales)
        .bind(agreement.total_rebate_earned)
        .bind(agreement.total_rebate_paid)
        .bind(&agreement.currency)
        .bind(&agreement.notes)
        .bind(agreement.created_at.to_rfc3339())
        .bind(agreement.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_rebate_agreement(&self, id: Uuid) -> anyhow::Result<Option<RebateAgreement>> {
        let row = sqlx::query(
            r#"SELECT id, agreement_number, name, customer_id, agreement_type, start_date, end_date,
                basis, calculation_method, payment_terms, status, total_eligible_sales,
                total_rebate_earned, total_rebate_paid, currency, notes, created_at, updated_at
                FROM tpm_rebate_agreements WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        
        Ok(row.map(|r| RebateAgreement {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            agreement_number: r.get(1),
            name: r.get(2),
            customer_id: parse_uuid_req(r.get(3)),
            agreement_type: r.get(4),
            start_date: parse_datetime(r.get(5)),
            end_date: parse_datetime(r.get(6)),
            basis: r.get(7),
            calculation_method: r.get(8),
            payment_terms: r.get(9),
            status: r.get::<String, _>(10).parse().unwrap_or(RebateStatus::Pending),
            total_eligible_sales: to_i64(r.get(11)),
            total_rebate_earned: to_i64(r.get(12)),
            total_rebate_paid: to_i64(r.get(13)),
            currency: r.get(14),
            notes: r.get(15),
            created_at: parse_datetime(r.get(16)),
            updated_at: parse_datetime(r.get(17)),
        }))
    }

    async fn list_rebate_agreements(&self, _customer_id: Option<Uuid>) -> anyhow::Result<Vec<RebateAgreement>> {
        let rows = sqlx::query(
            r#"SELECT id, agreement_number, name, customer_id, agreement_type, start_date, end_date,
                basis, calculation_method, payment_terms, status, total_eligible_sales,
                total_rebate_earned, total_rebate_paid, currency, notes, created_at, updated_at
                FROM tpm_rebate_agreements ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| RebateAgreement {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            agreement_number: r.get(1),
            name: r.get(2),
            customer_id: parse_uuid_req(r.get(3)),
            agreement_type: r.get(4),
            start_date: parse_datetime(r.get(5)),
            end_date: parse_datetime(r.get(6)),
            basis: r.get(7),
            calculation_method: r.get(8),
            payment_terms: r.get(9),
            status: r.get::<String, _>(10).parse().unwrap_or(RebateStatus::Pending),
            total_eligible_sales: to_i64(r.get(11)),
            total_rebate_earned: to_i64(r.get(12)),
            total_rebate_paid: to_i64(r.get(13)),
            currency: r.get(14),
            notes: r.get(15),
            created_at: parse_datetime(r.get(16)),
            updated_at: parse_datetime(r.get(17)),
        }).collect())
    }

    async fn update_rebate_agreement(&self, agreement: &RebateAgreement) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_rebate_agreements SET total_eligible_sales = ?, total_rebate_earned = ?,
                total_rebate_paid = ?, status = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(agreement.total_eligible_sales)
        .bind(agreement.total_rebate_earned)
        .bind(agreement.total_rebate_paid)
        .bind(agreement.status.to_string())
        .bind(agreement.updated_at.to_rfc3339())
        .bind(agreement.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_rebate_tier(&self, tier: &RebateTier) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_rebate_tiers (id, agreement_id, tier_number, min_quantity, max_quantity,
                min_value, max_value, rebate_percent, rebate_amount, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(tier.id.to_string())
        .bind(tier.agreement_id.to_string())
        .bind(tier.tier_number)
        .bind(tier.min_quantity)
        .bind(tier.max_quantity)
        .bind(tier.min_value)
        .bind(tier.max_value)
        .bind(tier.rebate_percent)
        .bind(tier.rebate_amount)
        .bind(tier.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_rebate_tiers(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateTier>> {
        let rows = sqlx::query(
            r#"SELECT id, agreement_id, tier_number, min_quantity, max_quantity, min_value, max_value,
                rebate_percent, rebate_amount, created_at FROM tpm_rebate_tiers WHERE agreement_id = ?
                ORDER BY tier_number"#
        )
        .bind(agreement_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| RebateTier {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            agreement_id: parse_uuid_req(r.get(1)),
            tier_number: to_i32_req(r.get(2)),
            min_quantity: r.get(3),
            max_quantity: r.get(4),
            min_value: r.get(5),
            max_value: r.get(6),
            rebate_percent: r.get(7),
            rebate_amount: r.get(8),
            created_at: parse_datetime(r.get(9)),
        }).collect())
    }

    async fn create_rebate_product(&self, rp: &RebateProduct) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_rebate_products (id, agreement_id, product_id, product_group_id,
                specific_rate, specific_amount, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(rp.id.to_string())
        .bind(rp.agreement_id.to_string())
        .bind(rp.product_id.map(|id| id.to_string()))
        .bind(rp.product_group_id.map(|id| id.to_string()))
        .bind(rp.specific_rate)
        .bind(rp.specific_amount)
        .bind(rp.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_rebate_products(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateProduct>> {
        let rows = sqlx::query(
            r#"SELECT id, agreement_id, product_id, product_group_id, specific_rate, specific_amount,
                created_at FROM tpm_rebate_products WHERE agreement_id = ?"#
        )
        .bind(agreement_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| RebateProduct {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            agreement_id: parse_uuid_req(r.get(1)),
            product_id: parse_uuid(r.get(2)),
            product_group_id: parse_uuid(r.get(3)),
            specific_rate: r.get(4),
            specific_amount: r.get(5),
            created_at: parse_datetime(r.get(6)),
        }).collect())
    }

    async fn create_rebate_accrual(&self, accrual: &RebateAccrual) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_rebate_accruals (id, agreement_id, sales_order_id, invoice_id,
                product_id, sales_amount, rebate_rate, rebate_amount, currency, accrual_date, status,
                paid_amount, remaining_amount, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(accrual.id.to_string())
        .bind(accrual.agreement_id.to_string())
        .bind(accrual.sales_order_id.map(|id| id.to_string()))
        .bind(accrual.invoice_id.map(|id| id.to_string()))
        .bind(accrual.product_id.map(|id| id.to_string()))
        .bind(accrual.sales_amount)
        .bind(accrual.rebate_rate)
        .bind(accrual.rebate_amount)
        .bind(&accrual.currency)
        .bind(accrual.accrual_date.to_rfc3339())
        .bind(&accrual.status)
        .bind(accrual.paid_amount)
        .bind(accrual.remaining_amount)
        .bind(accrual.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_rebate_accruals(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebateAccrual>> {
        let rows = sqlx::query(
            r#"SELECT id, agreement_id, sales_order_id, invoice_id, product_id, sales_amount,
                rebate_rate, rebate_amount, currency, accrual_date, status, paid_amount, remaining_amount,
                created_at FROM tpm_rebate_accruals WHERE agreement_id = ? ORDER BY accrual_date DESC"#
        )
        .bind(agreement_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| RebateAccrual {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            agreement_id: parse_uuid_req(r.get(1)),
            sales_order_id: parse_uuid(r.get(2)),
            invoice_id: parse_uuid(r.get(3)),
            product_id: parse_uuid(r.get(4)),
            sales_amount: r.get(5),
            rebate_rate: r.get(6),
            rebate_amount: r.get(7),
            currency: r.get(8),
            accrual_date: parse_datetime(r.get(9)),
            status: r.get(10),
            paid_amount: to_i64(r.get(11)),
            remaining_amount: r.get(12),
            created_at: parse_datetime(r.get(13)),
        }).collect())
    }

    async fn update_rebate_accrual(&self, accrual: &RebateAccrual) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_rebate_accruals SET status = ?, paid_amount = ?, remaining_amount = ? WHERE id = ?"#
        )
        .bind(&accrual.status)
        .bind(accrual.paid_amount)
        .bind(accrual.remaining_amount)
        .bind(accrual.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_rebate_payment(&self, payment: &RebatePayment) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_rebate_payments (id, payment_number, agreement_id, customer_id,
                payment_date, period_start, period_end, total_amount, currency, payment_method,
                reference_number, status, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(payment.id.to_string())
        .bind(&payment.payment_number)
        .bind(payment.agreement_id.to_string())
        .bind(payment.customer_id.to_string())
        .bind(payment.payment_date.to_rfc3339())
        .bind(payment.period_start.to_rfc3339())
        .bind(payment.period_end.to_rfc3339())
        .bind(payment.total_amount)
        .bind(&payment.currency)
        .bind(&payment.payment_method)
        .bind(&payment.reference_number)
        .bind(&payment.status)
        .bind(&payment.notes)
        .bind(payment.created_at.to_rfc3339())
        .bind(payment.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_rebate_payment(&self, id: Uuid) -> anyhow::Result<Option<RebatePayment>> {
        let row = sqlx::query(
            r#"SELECT id, payment_number, agreement_id, customer_id, payment_date, period_start,
                period_end, total_amount, currency, payment_method, reference_number, status, notes,
                created_at, updated_at FROM tpm_rebate_payments WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        
        Ok(row.map(|r| RebatePayment {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            payment_number: r.get(1),
            agreement_id: parse_uuid_req(r.get(2)),
            customer_id: parse_uuid_req(r.get(3)),
            payment_date: parse_datetime(r.get(4)),
            period_start: parse_datetime(r.get(5)),
            period_end: parse_datetime(r.get(6)),
            total_amount: r.get(7),
            currency: r.get(8),
            payment_method: r.get(9),
            reference_number: r.get(10),
            status: r.get(11),
            notes: r.get(12),
            created_at: parse_datetime(r.get(13)),
            updated_at: parse_datetime(r.get(14)),
        }))
    }

    async fn list_rebate_payments(&self, agreement_id: Uuid) -> anyhow::Result<Vec<RebatePayment>> {
        let rows = sqlx::query(
            r#"SELECT id, payment_number, agreement_id, customer_id, payment_date, period_start,
                period_end, total_amount, currency, payment_method, reference_number, status, notes,
                created_at, updated_at FROM tpm_rebate_payments WHERE agreement_id = ?
                ORDER BY payment_date DESC"#
        )
        .bind(agreement_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| RebatePayment {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            payment_number: r.get(1),
            agreement_id: parse_uuid_req(r.get(2)),
            customer_id: parse_uuid_req(r.get(3)),
            payment_date: parse_datetime(r.get(4)),
            period_start: parse_datetime(r.get(5)),
            period_end: parse_datetime(r.get(6)),
            total_amount: r.get(7),
            currency: r.get(8),
            payment_method: r.get(9),
            reference_number: r.get(10),
            status: r.get(11),
            notes: r.get(12),
            created_at: parse_datetime(r.get(13)),
            updated_at: parse_datetime(r.get(14)),
        }).collect())
    }

    async fn update_rebate_payment(&self, payment: &RebatePayment) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_rebate_payments SET status = ?, notes = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&payment.status)
        .bind(&payment.notes)
        .bind(payment.updated_at.to_rfc3339())
        .bind(payment.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_rebate_payment_line(&self, line: &RebatePaymentLine) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_rebate_payment_lines (id, payment_id, accrual_id, amount, currency, created_at)
                VALUES (?, ?, ?, ?, ?, ?)"#
        )
        .bind(line.id.to_string())
        .bind(line.payment_id.to_string())
        .bind(line.accrual_id.to_string())
        .bind(line.amount)
        .bind(&line.currency)
        .bind(line.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_rebate_payment_lines(&self, payment_id: Uuid) -> anyhow::Result<Vec<RebatePaymentLine>> {
        let rows = sqlx::query(
            r#"SELECT id, payment_id, accrual_id, amount, currency, created_at
                FROM tpm_rebate_payment_lines WHERE payment_id = ?"#
        )
        .bind(payment_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| RebatePaymentLine {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            payment_id: parse_uuid_req(r.get(1)),
            accrual_id: parse_uuid_req(r.get(2)),
            amount: r.get(3),
            currency: r.get(4),
            created_at: parse_datetime(r.get(5)),
        }).collect())
    }

    async fn create_chargeback(&self, cb: &Chargeback) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_chargebacks (id, chargeback_number, customer_id, invoice_id, promotion_id,
                chargeback_date, amount_claimed, amount_approved, amount_rejected, currency, status,
                claim_type, description, rejection_reason, submitted_by, reviewed_by, reviewed_at,
                paid_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(cb.id.to_string())
        .bind(&cb.chargeback_number)
        .bind(cb.customer_id.to_string())
        .bind(cb.invoice_id.map(|id| id.to_string()))
        .bind(cb.promotion_id.map(|id| id.to_string()))
        .bind(cb.chargeback_date.to_rfc3339())
        .bind(cb.amount_claimed)
        .bind(cb.amount_approved)
        .bind(cb.amount_rejected)
        .bind(&cb.currency)
        .bind(cb.status.to_string())
        .bind(&cb.claim_type)
        .bind(&cb.description)
        .bind(&cb.rejection_reason)
        .bind(cb.submitted_by.map(|id| id.to_string()))
        .bind(cb.reviewed_by.map(|id| id.to_string()))
        .bind(cb.reviewed_at.map(|dt| dt.to_rfc3339()))
        .bind(cb.paid_at.map(|dt| dt.to_rfc3339()))
        .bind(cb.created_at.to_rfc3339())
        .bind(cb.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_chargeback(&self, id: Uuid) -> anyhow::Result<Option<Chargeback>> {
        let row = sqlx::query(
            r#"SELECT id, chargeback_number, customer_id, invoice_id, promotion_id, chargeback_date,
                amount_claimed, amount_approved, amount_rejected, currency, status,
                claim_type, description, rejection_reason, submitted_by, reviewed_by, reviewed_at,
                paid_at, created_at, updated_at FROM tpm_chargebacks WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        
        Ok(row.map(|r| Chargeback {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            chargeback_number: r.get(1),
            customer_id: parse_uuid_req(r.get(2)),
            invoice_id: parse_uuid(r.get(3)),
            promotion_id: parse_uuid(r.get(4)),
            chargeback_date: parse_datetime(r.get(5)),
            amount_claimed: r.get(6),
            amount_approved: to_i64(r.get(7)),
            amount_rejected: to_i64(r.get(8)),
            currency: r.get(9),
            status: r.get::<String, _>(10).parse().unwrap_or(ClaimStatus::Submitted),
            claim_type: r.get(11),
            description: r.get(12),
            rejection_reason: r.get(13),
            submitted_by: parse_uuid(r.get(14)),
            reviewed_by: parse_uuid(r.get(15)),
            reviewed_at: parse_datetime_opt(r.get(16)),
            paid_at: parse_datetime_opt(r.get(17)),
            created_at: parse_datetime(r.get(18)),
            updated_at: parse_datetime(r.get(19)),
        }))
    }

    async fn list_chargebacks(&self, _customer_id: Option<Uuid>, _status: Option<ClaimStatus>) -> anyhow::Result<Vec<Chargeback>> {
        let rows = sqlx::query(
            r#"SELECT id, chargeback_number, customer_id, invoice_id, promotion_id, chargeback_date,
                amount_claimed, amount_approved, amount_rejected, currency, status,
                claim_type, description, rejection_reason, submitted_by, reviewed_by, reviewed_at,
                paid_at, created_at, updated_at FROM tpm_chargebacks ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| Chargeback {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            chargeback_number: r.get(1),
            customer_id: parse_uuid_req(r.get(2)),
            invoice_id: parse_uuid(r.get(3)),
            promotion_id: parse_uuid(r.get(4)),
            chargeback_date: parse_datetime(r.get(5)),
            amount_claimed: r.get(6),
            amount_approved: to_i64(r.get(7)),
            amount_rejected: to_i64(r.get(8)),
            currency: r.get(9),
            status: r.get::<String, _>(10).parse().unwrap_or(ClaimStatus::Submitted),
            claim_type: r.get(11),
            description: r.get(12),
            rejection_reason: r.get(13),
            submitted_by: parse_uuid(r.get(14)),
            reviewed_by: parse_uuid(r.get(15)),
            reviewed_at: parse_datetime_opt(r.get(16)),
            paid_at: parse_datetime_opt(r.get(17)),
            created_at: parse_datetime(r.get(18)),
            updated_at: parse_datetime(r.get(19)),
        }).collect())
    }

    async fn update_chargeback(&self, cb: &Chargeback) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_chargebacks SET amount_approved = ?, amount_rejected = ?, status = ?,
                rejection_reason = ?, reviewed_by = ?, reviewed_at = ?, paid_at = ?, updated_at = ?
                WHERE id = ?"#
        )
        .bind(cb.amount_approved)
        .bind(cb.amount_rejected)
        .bind(cb.status.to_string())
        .bind(&cb.rejection_reason)
        .bind(cb.reviewed_by.map(|id| id.to_string()))
        .bind(cb.reviewed_at.map(|dt| dt.to_rfc3339()))
        .bind(cb.paid_at.map(|dt| dt.to_rfc3339()))
        .bind(cb.updated_at.to_rfc3339())
        .bind(cb.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_chargeback_line(&self, line: &ChargebackLine) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_chargeback_lines (id, chargeback_id, product_id, quantity, unit_price,
                claimed_amount, approved_amount, rejected_amount, currency, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(line.id.to_string())
        .bind(line.chargeback_id.to_string())
        .bind(line.product_id.to_string())
        .bind(line.quantity)
        .bind(line.unit_price)
        .bind(line.claimed_amount)
        .bind(line.approved_amount)
        .bind(line.rejected_amount)
        .bind(&line.currency)
        .bind(line.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_chargeback_lines(&self, chargeback_id: Uuid) -> anyhow::Result<Vec<ChargebackLine>> {
        let rows = sqlx::query(
            r#"SELECT id, chargeback_id, product_id, quantity, unit_price, claimed_amount,
                approved_amount, rejected_amount, currency, created_at FROM tpm_chargeback_lines
                WHERE chargeback_id = ?"#
        )
        .bind(chargeback_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| ChargebackLine {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            chargeback_id: parse_uuid_req(r.get(1)),
            product_id: parse_uuid_req(r.get(2)),
            quantity: to_i32_req(r.get(3)),
            unit_price: r.get(4),
            claimed_amount: r.get(5),
            approved_amount: to_i64(r.get(6)),
            rejected_amount: to_i64(r.get(7)),
            currency: r.get(8),
            created_at: parse_datetime(r.get(9)),
        }).collect())
    }

    async fn create_chargeback_document(&self, doc: &ChargebackDocument) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_chargeback_documents (id, chargeback_id, document_type, file_name,
                file_path, uploaded_by, uploaded_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(doc.id.to_string())
        .bind(doc.chargeback_id.to_string())
        .bind(&doc.document_type)
        .bind(&doc.file_name)
        .bind(&doc.file_path)
        .bind(doc.uploaded_by.map(|id| id.to_string()))
        .bind(doc.uploaded_at.to_rfc3339())
        .bind(doc.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_chargeback_documents(&self, chargeback_id: Uuid) -> anyhow::Result<Vec<ChargebackDocument>> {
        let rows = sqlx::query(
            r#"SELECT id, chargeback_id, document_type, file_name, file_path, uploaded_by,
                uploaded_at, created_at FROM tpm_chargeback_documents WHERE chargeback_id = ?"#
        )
        .bind(chargeback_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| ChargebackDocument {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            chargeback_id: parse_uuid_req(r.get(1)),
            document_type: r.get(2),
            file_name: r.get(3),
            file_path: r.get(4),
            uploaded_by: parse_uuid(r.get(5)),
            uploaded_at: parse_datetime(r.get(6)),
            created_at: parse_datetime(r.get(7)),
        }).collect())
    }

    async fn create_promotion_performance(&self, perf: &PromotionPerformance) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_promotion_performance (id, promotion_id, period_start, period_end,
                baseline_sales, incremental_sales, total_sales, units_sold, promotion_cost, roi_percent,
                lift_percent, cannibalization, forward_buy, currency, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(perf.id.to_string())
        .bind(perf.promotion_id.to_string())
        .bind(perf.period_start.to_rfc3339())
        .bind(perf.period_end.to_rfc3339())
        .bind(perf.baseline_sales)
        .bind(perf.incremental_sales)
        .bind(perf.total_sales)
        .bind(perf.units_sold)
        .bind(perf.promotion_cost)
        .bind(perf.roi_percent)
        .bind(perf.lift_percent)
        .bind(perf.cannibalization)
        .bind(perf.forward_buy)
        .bind(&perf.currency)
        .bind(perf.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_promotion_performance(&self, promotion_id: Uuid) -> anyhow::Result<Vec<PromotionPerformance>> {
        let rows = sqlx::query(
            r#"SELECT id, promotion_id, period_start, period_end, baseline_sales, incremental_sales,
                total_sales, units_sold, promotion_cost, roi_percent, lift_percent, cannibalization,
                forward_buy, currency, created_at FROM tpm_promotion_performance WHERE promotion_id = ?
                ORDER BY period_start DESC"#
        )
        .bind(promotion_id.to_string())
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| PromotionPerformance {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            promotion_id: parse_uuid_req(r.get(1)),
            period_start: parse_datetime(r.get(2)),
            period_end: parse_datetime(r.get(3)),
            baseline_sales: r.get(4),
            incremental_sales: r.get(5),
            total_sales: r.get(6),
            units_sold: to_i32_req(r.get(7)),
            promotion_cost: r.get(8),
            roi_percent: r.get(9),
            lift_percent: r.get(10),
            cannibalization: r.get(11),
            forward_buy: r.get(12),
            currency: r.get(13),
            created_at: parse_datetime(r.get(14)),
        }).collect())
    }

    async fn create_promotion_plan(&self, plan: &PromotionPlan) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_promotion_plans (id, plan_number, name, fiscal_year, customer_id,
                customer_group_id, total_budget, allocated_budget, spent_budget, remaining_budget,
                currency, status, owner_id, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(plan.id.to_string())
        .bind(&plan.plan_number)
        .bind(&plan.name)
        .bind(plan.fiscal_year)
        .bind(plan.customer_id.map(|id| id.to_string()))
        .bind(plan.customer_group_id.map(|id| id.to_string()))
        .bind(plan.total_budget)
        .bind(plan.allocated_budget)
        .bind(plan.spent_budget)
        .bind(plan.remaining_budget)
        .bind(&plan.currency)
        .bind(&plan.status)
        .bind(plan.owner_id.map(|id| id.to_string()))
        .bind(plan.created_at.to_rfc3339())
        .bind(plan.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_promotion_plan(&self, id: Uuid) -> anyhow::Result<Option<PromotionPlan>> {
        let row = sqlx::query(
            r#"SELECT id, plan_number, name, fiscal_year, customer_id, customer_group_id, total_budget,
                allocated_budget, spent_budget, remaining_budget, currency, status, owner_id,
                created_at, updated_at FROM tpm_promotion_plans WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        
        Ok(row.map(|r| PromotionPlan {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            plan_number: r.get(1),
            name: r.get(2),
            fiscal_year: to_i32_req(r.get(3)),
            customer_id: parse_uuid(r.get(4)),
            customer_group_id: parse_uuid(r.get(5)),
            total_budget: r.get(6),
            allocated_budget: to_i64(r.get(7)),
            spent_budget: to_i64(r.get(8)),
            remaining_budget: r.get(9),
            currency: r.get(10),
            status: r.get(11),
            owner_id: parse_uuid(r.get(12)),
            created_at: parse_datetime(r.get(13)),
            updated_at: parse_datetime(r.get(14)),
        }))
    }

    async fn list_promotion_plans(&self, _customer_id: Option<Uuid>) -> anyhow::Result<Vec<PromotionPlan>> {
        let rows = sqlx::query(
            r#"SELECT id, plan_number, name, fiscal_year, customer_id, customer_group_id, total_budget,
                allocated_budget, spent_budget, remaining_budget, currency, status, owner_id,
                created_at, updated_at FROM tpm_promotion_plans ORDER BY fiscal_year DESC"#
        )
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| PromotionPlan {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            plan_number: r.get(1),
            name: r.get(2),
            fiscal_year: to_i32_req(r.get(3)),
            customer_id: parse_uuid(r.get(4)),
            customer_group_id: parse_uuid(r.get(5)),
            total_budget: r.get(6),
            allocated_budget: to_i64(r.get(7)),
            spent_budget: to_i64(r.get(8)),
            remaining_budget: r.get(9),
            currency: r.get(10),
            status: r.get(11),
            owner_id: parse_uuid(r.get(12)),
            created_at: parse_datetime(r.get(13)),
            updated_at: parse_datetime(r.get(14)),
        }).collect())
    }

    async fn update_promotion_plan(&self, plan: &PromotionPlan) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_promotion_plans SET allocated_budget = ?, spent_budget = ?, remaining_budget = ?,
                status = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(plan.allocated_budget)
        .bind(plan.spent_budget)
        .bind(plan.remaining_budget)
        .bind(&plan.status)
        .bind(plan.updated_at.to_rfc3339())
        .bind(plan.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_customer_trade_profile(&self, profile: &CustomerTradeProfile) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_customer_trade_profiles (id, customer_id, trade_class, annual_volume,
                growth_rate, avg_promotion_response, preferred_promotion_type, credit_limit,
                payment_terms, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(profile.id.to_string())
        .bind(profile.customer_id.to_string())
        .bind(&profile.trade_class)
        .bind(profile.annual_volume)
        .bind(profile.growth_rate)
        .bind(profile.avg_promotion_response)
        .bind(&profile.preferred_promotion_type)
        .bind(profile.credit_limit)
        .bind(&profile.payment_terms)
        .bind(&profile.notes)
        .bind(profile.created_at.to_rfc3339())
        .bind(profile.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_customer_trade_profile(&self, customer_id: Uuid) -> anyhow::Result<Option<CustomerTradeProfile>> {
        let row = sqlx::query(
            r#"SELECT id, customer_id, trade_class, annual_volume, growth_rate, avg_promotion_response,
                preferred_promotion_type, credit_limit, payment_terms, notes, created_at, updated_at
                FROM tpm_customer_trade_profiles WHERE customer_id = ?"#
        )
        .bind(customer_id.to_string())
        .fetch_optional(&self.pool).await?;
        
        Ok(row.map(|r| CustomerTradeProfile {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            customer_id: parse_uuid_req(r.get(1)),
            trade_class: r.get(2),
            annual_volume: r.get(3),
            growth_rate: r.get(4),
            avg_promotion_response: r.get(5),
            preferred_promotion_type: r.get(6),
            credit_limit: r.get(7),
            payment_terms: r.get(8),
            notes: r.get(9),
            created_at: parse_datetime(r.get(10)),
            updated_at: parse_datetime(r.get(11)),
        }))
    }

    async fn update_customer_trade_profile(&self, profile: &CustomerTradeProfile) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_customer_trade_profiles SET trade_class = ?, annual_volume = ?, growth_rate = ?,
                avg_promotion_response = ?, preferred_promotion_type = ?, credit_limit = ?, payment_terms = ?,
                notes = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&profile.trade_class)
        .bind(profile.annual_volume)
        .bind(profile.growth_rate)
        .bind(profile.avg_promotion_response)
        .bind(&profile.preferred_promotion_type)
        .bind(profile.credit_limit)
        .bind(&profile.payment_terms)
        .bind(&profile.notes)
        .bind(profile.updated_at.to_rfc3339())
        .bind(profile.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_ship_and_debit(&self, sad: &ShipAndDebit) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_ship_and_debits (id, sad_number, customer_id, product_id, authorized_price,
                list_price, authorized_discount, quantity_authorized, quantity_shipped, quantity_debited,
                currency, start_date, end_date, status, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(sad.id.to_string())
        .bind(&sad.sad_number)
        .bind(sad.customer_id.to_string())
        .bind(sad.product_id.to_string())
        .bind(sad.authorized_price)
        .bind(sad.list_price)
        .bind(sad.authorized_discount)
        .bind(sad.quantity_authorized)
        .bind(sad.quantity_shipped)
        .bind(sad.quantity_debited)
        .bind(&sad.currency)
        .bind(sad.start_date.to_rfc3339())
        .bind(sad.end_date.to_rfc3339())
        .bind(&sad.status)
        .bind(sad.created_at.to_rfc3339())
        .bind(sad.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_ship_and_debits(&self, _customer_id: Option<Uuid>) -> anyhow::Result<Vec<ShipAndDebit>> {
        let rows = sqlx::query(
            r#"SELECT id, sad_number, customer_id, product_id, authorized_price, list_price,
                authorized_discount, quantity_authorized, quantity_shipped, quantity_debited,
                currency, start_date, end_date, status, created_at, updated_at
                FROM tpm_ship_and_debits ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| ShipAndDebit {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            sad_number: r.get(1),
            customer_id: parse_uuid_req(r.get(2)),
            product_id: parse_uuid_req(r.get(3)),
            authorized_price: r.get(4),
            list_price: r.get(5),
            authorized_discount: r.get(6),
            quantity_authorized: to_i32_req(r.get(7)),
            quantity_shipped: to_i32_opt_req(r.get(8)),
            quantity_debited: to_i32_opt_req(r.get(9)),
            currency: r.get(10),
            start_date: parse_datetime(r.get(11)),
            end_date: parse_datetime(r.get(12)),
            status: r.get(13),
            created_at: parse_datetime(r.get(14)),
            updated_at: parse_datetime(r.get(15)),
        }).collect())
    }

    async fn update_ship_and_debit(&self, sad: &ShipAndDebit) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_ship_and_debits SET quantity_shipped = ?, quantity_debited = ?, status = ?,
                updated_at = ? WHERE id = ?"#
        )
        .bind(sad.quantity_shipped)
        .bind(sad.quantity_debited)
        .bind(&sad.status)
        .bind(sad.updated_at.to_rfc3339())
        .bind(sad.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_price_protection(&self, pp: &PriceProtection) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO tpm_price_protections (id, pp_number, customer_id, product_id, product_group_id,
                old_price, new_price, price_reduction, effective_date, inventory_on_hand, claim_amount,
                approved_amount, currency, status, notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(pp.id.to_string())
        .bind(&pp.pp_number)
        .bind(pp.customer_id.to_string())
        .bind(pp.product_id.map(|id| id.to_string()))
        .bind(pp.product_group_id.map(|id| id.to_string()))
        .bind(pp.old_price)
        .bind(pp.new_price)
        .bind(pp.price_reduction)
        .bind(pp.effective_date.to_rfc3339())
        .bind(pp.inventory_on_hand)
        .bind(pp.claim_amount)
        .bind(pp.approved_amount)
        .bind(&pp.currency)
        .bind(pp.status.to_string())
        .bind(&pp.notes)
        .bind(pp.created_at.to_rfc3339())
        .bind(pp.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_price_protections(&self, _customer_id: Option<Uuid>) -> anyhow::Result<Vec<PriceProtection>> {
        let rows = sqlx::query(
            r#"SELECT id, pp_number, customer_id, product_id, product_group_id, old_price, new_price,
                price_reduction, effective_date, inventory_on_hand, claim_amount, approved_amount,
                currency, status, notes, created_at, updated_at
                FROM tpm_price_protections ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool).await?;
        
        Ok(rows.into_iter().map(|r| PriceProtection {
            id: parse_uuid_opt(r.get::<Option<String>, _>(0)).unwrap_or_default(),
            pp_number: r.get(1),
            customer_id: parse_uuid_req(r.get(2)),
            product_id: parse_uuid(r.get(3)),
            product_group_id: parse_uuid(r.get(4)),
            old_price: r.get(5),
            new_price: r.get(6),
            price_reduction: r.get(7),
            effective_date: parse_datetime(r.get(8)),
            inventory_on_hand: to_i32_req(r.get(9)),
            claim_amount: r.get(10),
            approved_amount: to_i64(r.get(11)),
            currency: r.get(12),
            status: r.get::<String, _>(13).parse().unwrap_or(ClaimStatus::Submitted),
            notes: r.get(14),
            created_at: parse_datetime(r.get(15)),
            updated_at: parse_datetime(r.get(16)),
        }).collect())
    }

    async fn update_price_protection(&self, pp: &PriceProtection) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE tpm_price_protections SET approved_amount = ?, status = ?, notes = ?, updated_at = ?
                WHERE id = ?"#
        )
        .bind(pp.approved_amount)
        .bind(pp.status.to_string())
        .bind(&pp.notes)
        .bind(pp.updated_at.to_rfc3339())
        .bind(pp.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }
}
