use crate::models::*;
use crate::repository::TPMRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct TPMService<R: TPMRepository> {
    repo: R,
}

impl<R: TPMRepository> TPMService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_promotion(&self, req: CreatePromotionRequest) -> anyhow::Result<TradePromotion> {
        let now = Utc::now();
        let promo_number = format!("PROMO-{}", now.format("%Y%m%d%H%M%S"));
        let promo = TradePromotion {
            id: Uuid::new_v4(),
            promotion_number: promo_number,
            name: req.name,
            description: req.description,
            promotion_type: req.promotion_type,
            status: PromotionStatus::Draft,
            customer_id: req.customer_id,
            customer_group_id: req.customer_group_id,
            product_id: req.product_id,
            product_group_id: req.product_group_id,
            start_date: req.start_date,
            end_date: req.end_date,
            planned_budget: req.planned_budget,
            committed_budget: 0,
            spent_budget: 0,
            accrued_budget: 0,
            currency: req.currency,
            discount_percent: req.discount_percent,
            discount_amount: req.discount_amount,
            buy_quantity: None,
            get_quantity: None,
            max_redemptions: None,
            redemptions_count: 0,
            forecasted_sales: None,
            actual_sales: None,
            roi: None,
            owner_id: None,
            approval_status: "Pending".to_string(),
            approved_by: None,
            approved_at: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_promotion(&promo).await?;
        Ok(promo)
    }

    pub async fn get_promotion(&self, id: Uuid) -> anyhow::Result<Option<TradePromotion>> {
        self.repo.get_promotion(id).await
    }

    pub async fn list_promotions(&self, status: Option<PromotionStatus>, page: i32, page_size: i32) -> anyhow::Result<Vec<TradePromotion>> {
        let offset = (page - 1) * page_size;
        self.repo.list_promotions(status, page_size, offset).await
    }

    pub async fn activate_promotion(&self, id: Uuid) -> anyhow::Result<TradePromotion> {
        let mut promo = self.repo.get_promotion(id).await?.ok_or_else(|| anyhow::anyhow!("Promotion not found"))?;
        promo.status = PromotionStatus::Active;
        promo.approval_status = "Approved".to_string();
        promo.approved_at = Some(Utc::now());
        promo.updated_at = Utc::now();
        self.repo.update_promotion(&promo).await?;
        Ok(promo)
    }

    pub async fn calculate_promotion_performance(&self, id: Uuid) -> anyhow::Result<PromotionPerformance> {
        let promo = self.repo.get_promotion(id).await?.ok_or_else(|| anyhow::anyhow!("Promotion not found"))?;
        
        let baseline = promo.planned_budget / 2;
        let incremental = promo.actual_sales.unwrap_or(0) - baseline;
        let cost = promo.spent_budget;
        let roi = if cost > 0 { (incremental as f64 / cost as f64) * 100.0 } else { 0.0 };
        let lift = if baseline > 0 { (incremental as f64 / baseline as f64) * 100.0 } else { 0.0 };
        
        let perf = PromotionPerformance {
            id: Uuid::new_v4(),
            promotion_id: id,
            period_start: promo.start_date,
            period_end: promo.end_date,
            baseline_sales: baseline,
            incremental_sales: incremental,
            total_sales: promo.actual_sales.unwrap_or(0),
            units_sold: promo.redemptions_count,
            promotion_cost: cost,
            roi_percent: roi,
            lift_percent: lift,
            cannibalization: None,
            forward_buy: None,
            currency: promo.currency.clone(),
            created_at: Utc::now(),
        };
        self.repo.create_promotion_performance(&perf).await?;
        Ok(perf)
    }

    pub async fn create_rebate_agreement(&self, req: CreateRebateAgreementRequest) -> anyhow::Result<RebateAgreement> {
        let now = Utc::now();
        let agreement_number = format!("REB-{}", now.format("%Y%m%d%H%M%S"));
        let agreement = RebateAgreement {
            id: Uuid::new_v4(),
            agreement_number,
            name: req.name,
            customer_id: req.customer_id,
            agreement_type: req.agreement_type,
            start_date: req.start_date,
            end_date: req.end_date,
            basis: req.basis,
            calculation_method: req.calculation_method,
            payment_terms: req.payment_terms,
            status: RebateStatus::Pending,
            total_eligible_sales: 0,
            total_rebate_earned: 0,
            total_rebate_paid: 0,
            currency: "USD".to_string(),
            notes: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_rebate_agreement(&agreement).await?;
        
        for tier in req.tiers {
            let rebate_tier = RebateTier {
                id: Uuid::new_v4(),
                agreement_id: agreement.id,
                tier_number: tier.tier_number,
                min_quantity: tier.min_quantity,
                max_quantity: tier.max_quantity,
                min_value: tier.min_value,
                max_value: tier.max_value,
                rebate_percent: tier.rebate_percent,
                rebate_amount: None,
                created_at: now,
            };
            self.repo.create_rebate_tier(&rebate_tier).await?;
        }
        
        for product_id in req.products {
            let rp = RebateProduct {
                id: Uuid::new_v4(),
                agreement_id: agreement.id,
                product_id: Some(product_id),
                product_group_id: None,
                specific_rate: None,
                specific_amount: None,
                created_at: now,
            };
            self.repo.create_rebate_product(&rp).await?;
        }
        
        Ok(agreement)
    }

    pub async fn get_rebate_agreement(&self, id: Uuid) -> anyhow::Result<Option<RebateAgreement>> {
        self.repo.get_rebate_agreement(id).await
    }

    pub async fn calculate_rebate(&self, agreement_id: Uuid, sales_amount: i64, product_id: Uuid) -> anyhow::Result<RebateAccrual> {
        let agreement = self.repo.get_rebate_agreement(agreement_id).await?.ok_or_else(|| anyhow::anyhow!("Agreement not found"))?;
        let tiers = self.repo.list_rebate_tiers(agreement_id).await?;
        
        let mut rebate_rate = 0.0;
        for tier in tiers {
            if sales_amount >= tier.min_value && (tier.max_value.is_none() || sales_amount <= tier.max_value.unwrap()) {
                rebate_rate = tier.rebate_percent;
                break;
            }
        }
        
        let rebate_amount = (sales_amount as f64 * rebate_rate / 100.0) as i64;
        
        let accrual = RebateAccrual {
            id: Uuid::new_v4(),
            agreement_id,
            sales_order_id: None,
            invoice_id: None,
            product_id: Some(product_id),
            sales_amount,
            rebate_rate,
            rebate_amount,
            currency: agreement.currency.clone(),
            accrual_date: Utc::now(),
            status: "Accrued".to_string(),
            paid_amount: 0,
            remaining_amount: rebate_amount,
            created_at: Utc::now(),
        };
        self.repo.create_rebate_accrual(&accrual).await?;
        
        let mut agr = agreement;
        agr.total_eligible_sales += sales_amount;
        agr.total_rebate_earned += rebate_amount;
        agr.updated_at = Utc::now();
        self.repo.update_rebate_agreement(&agr).await?;
        
        Ok(accrual)
    }

    pub async fn process_rebate_payment(&self, agreement_id: Uuid, customer_id: Uuid,
        period_start: chrono::DateTime<Utc>, period_end: chrono::DateTime<Utc>) -> anyhow::Result<RebatePayment> {
        let accruals = self.repo.list_rebate_accruals(agreement_id).await?;
        let unpaid_accruals: Vec<_> = accruals.iter().filter(|a| a.remaining_amount > 0).collect();
        
        let total_amount: i64 = unpaid_accruals.iter().map(|a| a.remaining_amount).sum();
        
        let payment = RebatePayment {
            id: Uuid::new_v4(),
            payment_number: format!("PAY-{}", Utc::now().format("%Y%m%d%H%M%S")),
            agreement_id,
            customer_id,
            payment_date: Utc::now(),
            period_start,
            period_end,
            total_amount,
            currency: "USD".to_string(),
            payment_method: "Credit".to_string(),
            reference_number: None,
            status: "Pending".to_string(),
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_rebate_payment(&payment).await?;
        
        for accrual in unpaid_accruals {
            let mut line = RebatePaymentLine {
                id: Uuid::new_v4(),
                payment_id: payment.id,
                accrual_id: accrual.id,
                amount: accrual.remaining_amount,
                currency: accrual.currency.clone(),
                created_at: Utc::now(),
            };
            self.repo.create_rebate_payment_line(&line).await?;
            
            let mut acc = accrual.clone();
            acc.paid_amount = acc.remaining_amount;
            acc.remaining_amount = 0;
            acc.status = "Paid".to_string();
            self.repo.update_rebate_accrual(&acc).await?;
        }
        
        Ok(payment)
    }

    pub async fn submit_chargeback(&self, req: SubmitChargebackRequest) -> anyhow::Result<Chargeback> {
        let now = Utc::now();
        let chargeback_number = format!("CB-{}", now.format("%Y%m%d%H%M%S"));
        
        let total_claimed: i64 = req.lines.iter().map(|l| l.claimed_amount).sum();
        
        let cb = Chargeback {
            id: Uuid::new_v4(),
            chargeback_number,
            customer_id: req.customer_id,
            invoice_id: req.invoice_id,
            promotion_id: req.promotion_id,
            chargeback_date: now,
            amount_claimed: total_claimed,
            amount_approved: 0,
            amount_rejected: 0,
            currency: "USD".to_string(),
            status: ClaimStatus::Submitted,
            claim_type: req.claim_type,
            description: req.description,
            rejection_reason: None,
            submitted_by: None,
            reviewed_by: None,
            reviewed_at: None,
            paid_at: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_chargeback(&cb).await?;
        
        for line_req in req.lines {
            let line = ChargebackLine {
                id: Uuid::new_v4(),
                chargeback_id: cb.id,
                product_id: line_req.product_id,
                quantity: line_req.quantity,
                unit_price: line_req.unit_price,
                claimed_amount: line_req.claimed_amount,
                approved_amount: 0,
                rejected_amount: 0,
                currency: "USD".to_string(),
                created_at: now,
            };
            self.repo.create_chargeback_line(&line).await?;
        }
        
        Ok(cb)
    }

    pub async fn review_chargeback(&self, id: Uuid, approved_amount: i64, reviewed_by: Uuid,
        rejection_reason: Option<String>) -> anyhow::Result<Chargeback> {
        let mut cb = self.repo.get_chargeback(id).await?.ok_or_else(|| anyhow::anyhow!("Chargeback not found"))?;
        
        cb.amount_approved = approved_amount;
        cb.amount_rejected = cb.amount_claimed - approved_amount;
        cb.reviewed_by = Some(reviewed_by);
        cb.reviewed_at = Some(Utc::now());
        
        if approved_amount > 0 {
            cb.status = ClaimStatus::Approved;
        } else {
            cb.status = ClaimStatus::Rejected;
            cb.rejection_reason = rejection_reason;
        }
        
        cb.updated_at = Utc::now();
        self.repo.update_chargeback(&cb).await?;
        Ok(cb)
    }

    pub async fn create_trade_fund(&self, customer_id: Option<Uuid>, name: String,
        fund_type: FundType, fiscal_year: i32, total_budget: i64) -> anyhow::Result<TradeFund> {
        let now = Utc::now();
        let fund_number = format!("FUND-{}", now.format("%Y%m%d%H%M%S"));
        let fund = TradeFund {
            id: Uuid::new_v4(),
            fund_number,
            name,
            fund_type,
            customer_id,
            fiscal_year,
            total_budget,
            committed_amount: 0,
            spent_amount: 0,
            available_amount: total_budget,
            currency: "USD".to_string(),
            start_date: now,
            end_date: now + chrono::Duration::days(365),
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_trade_fund(&fund).await?;
        Ok(fund)
    }

    pub async fn commit_fund(&self, fund_id: Uuid, promotion_id: Uuid, amount: i64) -> anyhow::Result<TradeFund> {
        let mut fund = self.repo.get_trade_fund(fund_id).await?.ok_or_else(|| anyhow::anyhow!("Fund not found"))?;
        
        if amount > fund.available_amount {
            return Err(anyhow::anyhow!("Insufficient fund balance"));
        }
        
        fund.committed_amount += amount;
        fund.available_amount -= amount;
        fund.updated_at = Utc::now();
        self.repo.update_trade_fund(&fund).await?;
        
        let txn = TradeFundTransaction {
            id: Uuid::new_v4(),
            fund_id,
            promotion_id: Some(promotion_id),
            transaction_type: "Commitment".to_string(),
            amount,
            currency: fund.currency.clone(),
            reference_number: None,
            description: None,
            transaction_date: Utc::now(),
            created_by: None,
            created_at: Utc::now(),
        };
        self.repo.create_fund_transaction(&txn).await?;
        
        Ok(fund)
    }

    pub async fn create_ship_and_debit(&self, customer_id: Uuid, product_id: Uuid,
        authorized_price: i64, list_price: i64, quantity: i32) -> anyhow::Result<ShipAndDebit> {
        let now = Utc::now();
        let sad_number = format!("SAD-{}", now.format("%Y%m%d%H%M%S"));
        let authorized_discount = list_price - authorized_price;
        
        let sad = ShipAndDebit {
            id: Uuid::new_v4(),
            sad_number,
            customer_id,
            product_id,
            authorized_price,
            list_price,
            authorized_discount,
            quantity_authorized: quantity,
            quantity_shipped: 0,
            quantity_debited: 0,
            currency: "USD".to_string(),
            start_date: now,
            end_date: now + chrono::Duration::days(90),
            status: "Active".to_string(),
            created_at: now,
            updated_at: now,
        };
        self.repo.create_ship_and_debit(&sad).await?;
        Ok(sad)
    }

    pub async fn record_shipment(&self, sad_id: Uuid, quantity: i32) -> anyhow::Result<ShipAndDebit> {
        let mut sad = self.repo.list_ship_and_debits(None).await?
            .into_iter().find(|s| s.id == sad_id)
            .ok_or_else(|| anyhow::anyhow!("SAD not found"))?;
        
        sad.quantity_shipped += quantity;
        sad.quantity_debited += quantity;
        sad.updated_at = Utc::now();
        self.repo.update_ship_and_debit(&sad).await?;
        Ok(sad)
    }

    pub async fn create_price_protection(&self, customer_id: Uuid, product_id: Uuid,
        old_price: i64, new_price: i64, inventory: i32) -> anyhow::Result<PriceProtection> {
        let now = Utc::now();
        let pp_number = format!("PP-{}", now.format("%Y%m%d%H%M%S"));
        let price_reduction = old_price - new_price;
        let claim_amount = price_reduction * inventory as i64;
        
        let pp = PriceProtection {
            id: Uuid::new_v4(),
            pp_number,
            customer_id,
            product_id: Some(product_id),
            product_group_id: None,
            old_price,
            new_price,
            price_reduction,
            effective_date: now,
            inventory_on_hand: inventory,
            claim_amount,
            approved_amount: 0,
            currency: "USD".to_string(),
            status: ClaimStatus::Submitted,
            notes: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_price_protection(&pp).await?;
        Ok(pp)
    }

    pub async fn approve_price_protection(&self, pp_id: Uuid, approved_amount: i64) -> anyhow::Result<PriceProtection> {
        let mut pps = self.repo.list_price_protections(None).await?;
        let mut pp = pps.iter_mut().find(|p| p.id == pp_id)
            .ok_or_else(|| anyhow::anyhow!("Price protection not found"))?.clone();
        
        pp.approved_amount = approved_amount;
        pp.status = ClaimStatus::Approved;
        pp.updated_at = Utc::now();
        self.repo.update_price_protection(&pp).await?;
        Ok(pp)
    }
}
