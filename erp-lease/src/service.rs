use crate::models::*;
use crate::repository::{LeaseRepository, SqliteLeaseRepository};
use chrono::{Datelike, NaiveDate, Utc};
use erp_core::{BaseEntity, Result};
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct LeaseService {
    repo: SqliteLeaseRepository,
}

impl LeaseService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { repo: SqliteLeaseRepository::new(pool) }
    }

    pub async fn create_lease(&self, pool: &SqlitePool, req: CreateLeaseRequest) -> Result<Lease> {
        let now = Utc::now();
        let lease = Lease {
            base: BaseEntity::new(),
            lease_number: generate_lease_number(),
            name: req.name,
            description: req.description,
            lease_type: req.lease_type,
            lessor_id: req.lessor_id,
            lessee_id: req.lessee_id,
            asset_id: req.asset_id,
            commencement_date: req.commencement_date,
            end_date: req.end_date,
            lease_term_months: calculate_term_months(req.commencement_date, req.end_date),
            renewal_option: req.renewal_option,
            renewal_term_months: req.renewal_term_months,
            termination_option: req.termination_option,
            termination_notice_days: req.termination_notice_days,
            purchase_option: req.purchase_option,
            purchase_option_price: req.purchase_option_price,
            fair_value_at_commencement: req.fair_value_at_commencement,
            residual_value_guarantee: req.residual_value_guarantee,
            currency: req.currency,
            discount_rate: req.discount_rate,
            implicit_rate: req.implicit_rate,
            incremental_borrowing_rate: req.incremental_borrowing_rate,
            initial_direct_costs: req.initial_direct_costs,
            lease_incentives: req.lease_incentives,
            decommissioning_provision: req.decommissioning_provision,
            status: LeaseStatus::Draft,
            classification_date: None,
            classification_by: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_lease(&lease).await
    }

    pub async fn get_lease(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<Lease>> {
        self.repo.get_lease(id).await
    }

    pub async fn list_leases(&self, pool: &SqlitePool, status: Option<LeaseStatus>) -> Result<Vec<Lease>> {
        self.repo.list_leases(status).await
    }

    pub async fn calculate_amortization(&self, pool: &SqlitePool, lease_id: Uuid) -> Result<Vec<LeaseAmortizationSchedule>> {
        let lease = self.repo.get_lease(lease_id).await?.ok_or_else(|| anyhow::anyhow!("Lease not found"))?;
        let schedule = self.repo.create_payment_schedule(&LeasePaymentSchedule {
            base: BaseEntity::new(),
            lease_id,
            schedule_number: format!("SCH-{}", uuid::Uuid::new_v4()),
            effective_date: lease.commencement_date,
            payment_frequency: PaymentFrequency::Monthly,
            payment_timing: PaymentTiming::InArrears,
            payment_day: None,
            base_payment: lease.fair_value_at_commencement / lease.lease_term_months as i64,
            escalation_type: EscalationType::None,
            escalation_rate: None,
            escalation_frequency_months: None,
            first_escalation_date: None,
            cap_amount: None,
            floor_amount: None,
            currency: lease.currency.clone(),
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }).await?;
        
        let monthly_payment = schedule.base_payment as f64;
        let rate = lease.discount_rate / 100.0 / 12.0;
        let mut amort_schedule = Vec::new();
        let mut opening_liability = lease.fair_value_at_commencement as f64;
        let initial_rou = lease.fair_value_at_commencement as f64;
        let monthly_depreciation = initial_rou / lease.lease_term_months as f64;
        let mut opening_rou = initial_rou;
        
        for period in 1..=lease.lease_term_months {
            let interest = opening_liability * rate;
            let principal = monthly_payment - interest;
            let closing_liability = opening_liability - principal;
            let closing_rou = (opening_rou - monthly_depreciation).max(0.0);
            
            amort_schedule.push(LeaseAmortizationSchedule {
                id: Uuid::new_v4(),
                lease_id,
                period_number: period,
                period_start: add_months(lease.commencement_date, period - 1),
                period_end: add_months(lease.commencement_date, period),
                opening_liability: opening_liability as i64,
                payment: monthly_payment as i64,
                interest_expense: interest as i64,
                principal_reduction: principal as i64,
                closing_liability: closing_liability as i64,
                opening_rou_asset: opening_rou as i64,
                depreciation_expense: monthly_depreciation as i64,
                closing_rou_asset: closing_rou as i64,
                total_expense: (interest + monthly_depreciation) as i64,
                currency: lease.currency.clone(),
                created_at: Utc::now(),
            });
            
            opening_liability = closing_liability;
            opening_rou = closing_rou;
        }
        
        self.repo.create_amortization_schedule(&amort_schedule).await?;
        Ok(amort_schedule)
    }

    pub async fn create_rou_asset(&self, pool: &SqlitePool, lease_id: Uuid) -> Result<RightOfUseAsset> {
        let lease = self.repo.get_lease(lease_id).await?.ok_or_else(|| anyhow::anyhow!("Lease not found"))?;
        let rou = RightOfUseAsset {
            base: BaseEntity::new(),
            lease_id,
            asset_number: format!("ROU-{}", lease.lease_number),
            name: format!("Right of Use Asset - {}", lease.name),
            initial_cost: lease.fair_value_at_commencement,
            accumulated_depreciation: 0,
            impairment_loss: 0,
            net_book_value: lease.fair_value_at_commencement,
            depreciation_method: DepreciationMethod::StraightLine,
            useful_life_months: lease.lease_term_months,
            residual_value: 0,
            depreciation_start_date: lease.commencement_date,
            depreciation_end_date: lease.end_date,
            currency: lease.currency,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_rou_asset(&rou).await
    }

    pub async fn create_liability(&self, pool: &SqlitePool, lease_id: Uuid) -> Result<LeaseLiability> {
        let lease = self.repo.get_lease(lease_id).await?.ok_or_else(|| anyhow::anyhow!("Lease not found"))?;
        let liability = LeaseLiability {
            base: BaseEntity::new(),
            lease_id,
            liability_number: format!("LIA-{}", lease.lease_number),
            initial_liability: lease.fair_value_at_commencement,
            outstanding_balance: lease.fair_value_at_commencement,
            interest_accrued: 0,
            principal_paid: 0,
            currency: lease.currency,
            calculation_date: Utc::now().date_naive(),
            amortization_method: AmortizationMethod::EffectiveInterest,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_liability(&liability).await
    }

    pub async fn record_payment(&self, pool: &SqlitePool, lease_id: Uuid, req: RecordPaymentRequest) -> Result<LeasePayment> {
        let payment = LeasePayment {
            base: BaseEntity::new(),
            lease_id,
            payment_number: format!("PAY-{}", uuid::Uuid::new_v4()),
            payment_date: req.payment_date,
            due_date: req.due_date,
            period_start: req.period_start,
            period_end: req.period_end,
            period_number: req.period_number,
            fixed_payment: req.amount,
            variable_payment: 0,
            escalation_amount: 0,
            total_payment: req.amount,
            currency: req.currency,
            payment_status: PaymentStatus::Paid,
            paid_date: Some(req.payment_date),
            paid_amount: Some(req.amount),
            notes: req.notes,
            created_at: Utc::now(),
        };
        self.repo.create_payment(&payment).await
    }

    pub async fn modify_lease(&self, pool: &SqlitePool, lease_id: Uuid, req: ModifyLeaseRequest) -> Result<LeaseModification> {
        let lease = self.repo.get_lease(lease_id).await?.ok_or_else(|| anyhow::anyhow!("Lease not found"))?;
        let modification = LeaseModification {
            base: BaseEntity::new(),
            modification_number: format!("MOD-{}", uuid::Uuid::new_v4()),
            lease_id,
            modification_date: Utc::now().date_naive(),
            effective_date: req.effective_date,
            modification_type: req.modification_type,
            reason: req.reason,
            original_term_months: lease.lease_term_months,
            new_term_months: req.new_term_months.unwrap_or(lease.lease_term_months),
            original_payment: lease.fair_value_at_commencement,
            new_payment: req.new_payment.unwrap_or(lease.fair_value_at_commencement),
            original_discount_rate: lease.discount_rate,
            new_discount_rate: req.new_discount_rate,
            remeasurement_gain_loss: 0,
            rou_adjustment: 0,
            liability_adjustment: 0,
            currency: lease.currency,
            approved_by: None,
            approved_at: None,
            status: ModificationStatus::Draft,
            created_at: Utc::now(),
        };
        self.repo.create_modification(&modification).await
    }

    pub async fn generate_disclosure(&self, pool: &SqlitePool, period_start: NaiveDate, period_end: NaiveDate) -> Result<LeaseDisclosure> {
        let leases = self.repo.list_leases(Some(LeaseStatus::Active)).await?;
        let mut total_rou = 0i64;
        let mut total_liability = 0i64;
        let mut finance_count = 0i32;
        let mut operating_count = 0i32;
        let mut total_term = 0.0;
        let mut total_rate = 0.0;
        
        for lease in &leases {
            if let Some(rou) = self.repo.get_rou_asset(lease.base.id).await? {
                total_rou += rou.net_book_value;
            }
            if let Some(lia) = self.repo.get_liability(lease.base.id).await? {
                total_liability += lia.outstanding_balance;
            }
            match lease.lease_type {
                LeaseType::Finance => finance_count += 1,
                LeaseType::Operating => operating_count += 1,
                _ => {}
            }
            total_term += lease.lease_term_months as f64;
            total_rate += lease.discount_rate;
        }
        
        let count = leases.len().max(1) as f64;
        let disclosure = LeaseDisclosure {
            base: BaseEntity::new(),
            reporting_period: format!("{} to {}", period_start, period_end),
            period_start,
            period_end,
            total_finance_leases: finance_count,
            total_operating_leases: operating_count,
            total_rou_assets: total_rou,
            total_lease_liabilities: total_liability,
            total_depreciation: 0,
            total_interest: 0,
            total_lease_payments: 0,
            maturities_within_1_year: 0,
            maturities_1_to_5_years: 0,
            maturities_after_5_years: 0,
            total_undiscounted_payments: 0,
            weighted_avg_lease_term: total_term / count,
            weighted_avg_discount_rate: total_rate / count,
            currency: "USD".to_string(),
            created_at: Utc::now(),
        };
        self.repo.create_disclosure(&disclosure).await
    }
}

fn generate_lease_number() -> String {
    format!("LSE-{}", uuid::Uuid::new_v4())
}

fn calculate_term_months(start: NaiveDate, end: NaiveDate) -> i32 {
    ((end.year() - start.year()) * 12 + (end.month() as i32 - start.month() as i32)).max(1)
}

fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let months = months as u32;
    let year = date.year() + ((date.month() + months) / 13) as i32;
    let month = ((date.month() + months - 1) % 12) + 1;
    NaiveDate::from_ymd_opt(year, month, date.day()).unwrap_or(date)
}

#[derive(Debug, Deserialize)]
pub struct CreateLeaseRequest {
    pub name: String,
    pub description: Option<String>,
    pub lease_type: LeaseType,
    pub lessor_id: Uuid,
    pub lessee_id: Uuid,
    pub asset_id: Option<Uuid>,
    pub commencement_date: NaiveDate,
    pub end_date: NaiveDate,
    pub renewal_option: bool,
    pub renewal_term_months: Option<i32>,
    pub termination_option: bool,
    pub termination_notice_days: Option<i32>,
    pub purchase_option: bool,
    pub purchase_option_price: Option<i64>,
    pub fair_value_at_commencement: i64,
    pub residual_value_guarantee: Option<i64>,
    pub currency: String,
    pub discount_rate: f64,
    pub implicit_rate: Option<f64>,
    pub incremental_borrowing_rate: Option<f64>,
    pub initial_direct_costs: i64,
    pub lease_incentives: i64,
    pub decommissioning_provision: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RecordPaymentRequest {
    pub payment_date: NaiveDate,
    pub due_date: NaiveDate,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub period_number: i32,
    pub amount: i64,
    pub currency: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifyLeaseRequest {
    pub effective_date: NaiveDate,
    pub modification_type: ModificationType,
    pub reason: String,
    pub new_term_months: Option<i32>,
    pub new_payment: Option<i64>,
    pub new_discount_rate: Option<f64>,
}
