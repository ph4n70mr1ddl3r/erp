use crate::models::*;
use async_trait::async_trait;
use chrono::NaiveDate;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait LeaseRepository: Send + Sync {
    async fn create_lease(&self, lease: &Lease) -> Result<Lease>;
    async fn get_lease(&self, id: Uuid) -> Result<Option<Lease>>;
    async fn list_leases(&self, status: Option<LeaseStatus>) -> Result<Vec<Lease>>;
    async fn update_lease(&self, lease: &Lease) -> Result<Lease>;
    async fn create_payment(&self, payment: &LeasePayment) -> Result<LeasePayment>;
    async fn list_payments(&self, lease_id: Uuid) -> Result<Vec<LeasePayment>>;
    async fn create_payment_schedule(&self, schedule: &LeasePaymentSchedule) -> Result<LeasePaymentSchedule>;
    async fn create_rou_asset(&self, asset: &RightOfUseAsset) -> Result<RightOfUseAsset>;
    async fn get_rou_asset(&self, lease_id: Uuid) -> Result<Option<RightOfUseAsset>>;
    async fn create_liability(&self, liability: &LeaseLiability) -> Result<LeaseLiability>;
    async fn get_liability(&self, lease_id: Uuid) -> Result<Option<LeaseLiability>>;
    async fn create_amortization_schedule(&self, schedule: &[LeaseAmortizationSchedule]) -> Result<()>;
    async fn get_amortization_schedule(&self, lease_id: Uuid) -> Result<Vec<LeaseAmortizationSchedule>>;
    async fn create_modification(&self, modification: &LeaseModification) -> Result<LeaseModification>;
    async fn create_expense(&self, expense: &LeaseExpense) -> Result<LeaseExpense>;
    async fn list_expenses(&self, lease_id: Uuid, from: NaiveDate, to: NaiveDate) -> Result<Vec<LeaseExpense>>;
    async fn create_disclosure(&self, disclosure: &LeaseDisclosure) -> Result<LeaseDisclosure>;
}

pub struct SqliteLeaseRepository {
    pool: SqlitePool,
}

impl SqliteLeaseRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LeaseRepository for SqliteLeaseRepository {
    async fn create_lease(&self, lease: &Lease) -> Result<Lease> {
        let lease = lease.clone();
        sqlx::query!(
            r#"INSERT INTO leases (id, lease_number, name, description, lease_type, lessor_id, lessee_id,
               asset_id, commencement_date, end_date, lease_term_months, renewal_option, renewal_term_months,
               termination_option, termination_notice_days, purchase_option, purchase_option_price,
               fair_value_at_commencement, residual_value_guarantee, currency, discount_rate, implicit_rate,
               incremental_borrowing_rate, initial_direct_costs, lease_incentives, decommissioning_provision,
               status, classification_date, classification_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            lease.base.id, lease.lease_number, lease.name, lease.description,
            lease.lease_type, lease.lessor_id, lease.lessee_id, lease.asset_id,
            lease.commencement_date, lease.end_date, lease.lease_term_months,
            lease.renewal_option, lease.renewal_term_months, lease.termination_option,
            lease.termination_notice_days, lease.purchase_option, lease.purchase_option_price,
            lease.fair_value_at_commencement, lease.residual_value_guarantee, lease.currency,
            lease.discount_rate, lease.implicit_rate, lease.incremental_borrowing_rate,
            lease.initial_direct_costs, lease.lease_incentives, lease.decommissioning_provision,
            lease.status, lease.classification_date, lease.classification_by,
            lease.created_at, lease.updated_at
        ).execute(&self.pool).await?;
        Ok(lease)
    }

    async fn get_lease(&self, id: Uuid) -> Result<Option<Lease>> {
        let row = sqlx::query_as!(
            Lease,
            r#"SELECT id as "base: BaseEntity", lease_number, name, description, lease_type, lessor_id,
               lessee_id, asset_id, commencement_date, end_date, lease_term_months, renewal_option,
               renewal_term_months, termination_option, termination_notice_days, purchase_option,
               purchase_option_price, fair_value_at_commencement, residual_value_guarantee, currency,
               discount_rate, implicit_rate, incremental_borrowing_rate, initial_direct_costs,
               lease_incentives, decommissioning_provision, status, classification_date, classification_by,
               created_at, updated_at FROM leases WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(row)
    }

    async fn list_leases(&self, status: Option<LeaseStatus>) -> Result<Vec<Lease>> {
        let rows = match status {
            Some(s) => sqlx::query_as!(
                Lease,
                r#"SELECT id as "base: BaseEntity", lease_number, name, description, lease_type, lessor_id,
                   lessee_id, asset_id, commencement_date, end_date, lease_term_months, renewal_option,
                   renewal_term_months, termination_option, termination_notice_days, purchase_option,
                   purchase_option_price, fair_value_at_commencement, residual_value_guarantee, currency,
                   discount_rate, implicit_rate, incremental_borrowing_rate, initial_direct_costs,
                   lease_incentives, decommissioning_provision, status, classification_date, classification_by,
                   created_at, updated_at FROM leases WHERE status = ?"#,
                   s
            ).fetch_all(&self.pool).await?,
            None => sqlx::query_as!(
                Lease,
                r#"SELECT id as "base: BaseEntity", lease_number, name, description, lease_type, lessor_id,
                   lessee_id, asset_id, commencement_date, end_date, lease_term_months, renewal_option,
                   renewal_term_months, termination_option, termination_notice_days, purchase_option,
                   purchase_option_price, fair_value_at_commencement, residual_value_guarantee, currency,
                   discount_rate, implicit_rate, incremental_borrowing_rate, initial_direct_costs,
                   lease_incentives, decommissioning_provision, status, classification_date, classification_by,
                   created_at, updated_at FROM leases"#
            ).fetch_all(&self.pool).await?,
        };
        Ok(rows)
    }

    async fn update_lease(&self, lease: &Lease) -> Result<Lease> {
        let lease = lease.clone();
        sqlx::query!(
            r#"UPDATE leases SET name = ?, description = ?, status = ?, updated_at = ? WHERE id = ?"#,
            lease.name, lease.description, lease.status, lease.updated_at, lease.base.id
        ).execute(&self.pool).await?;
        Ok(lease)
    }

    async fn create_payment(&self, payment: &LeasePayment) -> Result<LeasePayment> {
        let payment = payment.clone();
        sqlx::query!(
            r#"INSERT INTO lease_payments (id, lease_id, payment_number, payment_date, due_date,
               period_start, period_end, period_number, fixed_payment, variable_payment,
               escalation_amount, total_payment, currency, payment_status, paid_date, paid_amount, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            payment.base.id, payment.lease_id, payment.payment_number, payment.payment_date,
            payment.due_date, payment.period_start, payment.period_end, payment.period_number,
            payment.fixed_payment, payment.variable_payment, payment.escalation_amount,
            payment.total_payment, payment.currency, payment.payment_status, payment.paid_date,
            payment.paid_amount, payment.notes, payment.created_at
        ).execute(&self.pool).await?;
        Ok(payment)
    }

    async fn list_payments(&self, lease_id: Uuid) -> Result<Vec<LeasePayment>> {
        let rows = sqlx::query_as!(
            LeasePayment,
            r#"SELECT id as "base: BaseEntity", lease_id, payment_number, payment_date, due_date,
               period_start, period_end, period_number, fixed_payment, variable_payment,
               escalation_amount, total_payment, currency, payment_status, paid_date, paid_amount, notes, created_at
               FROM lease_payments WHERE lease_id = ? ORDER BY period_number"#,
            lease_id
        ).fetch_all(&self.pool).await?;
        Ok(rows)
    }

    async fn create_payment_schedule(&self, schedule: &LeasePaymentSchedule) -> Result<LeasePaymentSchedule> {
        let schedule = schedule.clone();
        sqlx::query!(
            r#"INSERT INTO lease_payment_schedules (id, lease_id, schedule_number, effective_date,
               payment_frequency, payment_timing, payment_day, base_payment, escalation_type,
               escalation_rate, escalation_frequency_months, first_escalation_date, cap_amount,
               floor_amount, currency, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            schedule.base.id, schedule.lease_id, schedule.schedule_number, schedule.effective_date,
            schedule.payment_frequency, schedule.payment_timing, schedule.payment_day,
            schedule.base_payment, schedule.escalation_type, schedule.escalation_rate,
            schedule.escalation_frequency_months, schedule.first_escalation_date, schedule.cap_amount,
            schedule.floor_amount, schedule.currency, schedule.status, schedule.created_at, schedule.updated_at
        ).execute(&self.pool).await?;
        Ok(schedule)
    }

    async fn create_rou_asset(&self, asset: &RightOfUseAsset) -> Result<RightOfUseAsset> {
        let asset = asset.clone();
        sqlx::query!(
            r#"INSERT INTO right_of_use_assets (id, lease_id, asset_number, name, initial_cost,
               accumulated_depreciation, impairment_loss, net_book_value, depreciation_method,
               useful_life_months, residual_value, depreciation_start_date, depreciation_end_date,
               currency, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            asset.base.id, asset.lease_id, asset.asset_number, asset.name, asset.initial_cost,
            asset.accumulated_depreciation, asset.impairment_loss, asset.net_book_value,
            asset.depreciation_method, asset.useful_life_months, asset.residual_value,
            asset.depreciation_start_date, asset.depreciation_end_date, asset.currency,
            asset.status, asset.created_at, asset.updated_at
        ).execute(&self.pool).await?;
        Ok(asset)
    }

    async fn get_rou_asset(&self, lease_id: Uuid) -> Result<Option<RightOfUseAsset>> {
        let row = sqlx::query_as!(
            RightOfUseAsset,
            r#"SELECT id as "base: BaseEntity", lease_id, asset_number, name, initial_cost,
               accumulated_depreciation, impairment_loss, net_book_value, depreciation_method,
               useful_life_months, residual_value, depreciation_start_date, depreciation_end_date,
               currency, status, created_at, updated_at FROM right_of_use_assets WHERE lease_id = ?"#,
            lease_id
        ).fetch_optional(&self.pool).await?;
        Ok(row)
    }

    async fn create_liability(&self, liability: &LeaseLiability) -> Result<LeaseLiability> {
        let liability = liability.clone();
        sqlx::query!(
            r#"INSERT INTO lease_liabilities (id, lease_id, liability_number, initial_liability,
               outstanding_balance, interest_accrued, principal_paid, currency, calculation_date,
               amortization_method, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            liability.base.id, liability.lease_id, liability.liability_number, liability.initial_liability,
            liability.outstanding_balance, liability.interest_accrued, liability.principal_paid,
            liability.currency, liability.calculation_date, liability.amortization_method,
            liability.status, liability.created_at, liability.updated_at
        ).execute(&self.pool).await?;
        Ok(liability)
    }

    async fn get_liability(&self, lease_id: Uuid) -> Result<Option<LeaseLiability>> {
        let row = sqlx::query_as!(
            LeaseLiability,
            r#"SELECT id as "base: BaseEntity", lease_id, liability_number, initial_liability,
               outstanding_balance, interest_accrued, principal_paid, currency, calculation_date,
               amortization_method, status, created_at, updated_at FROM lease_liabilities WHERE lease_id = ?"#,
            lease_id
        ).fetch_optional(&self.pool).await?;
        Ok(row)
    }

    async fn create_amortization_schedule(&self, schedule: &[LeaseAmortizationSchedule]) -> Result<()> {
        for item in schedule {
            sqlx::query!(
                r#"INSERT INTO lease_amortization_schedule (id, lease_id, period_number, period_start,
                   period_end, opening_liability, payment, interest_expense, principal_reduction,
                   closing_liability, opening_rou_asset, depreciation_expense, closing_rou_asset,
                   total_expense, currency, created_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
                item.id, item.lease_id, item.period_number, item.period_start, item.period_end,
                item.opening_liability, item.payment, item.interest_expense, item.principal_reduction,
                item.closing_liability, item.opening_rou_asset, item.depreciation_expense,
                item.closing_rou_asset, item.total_expense, item.currency, item.created_at
            ).execute(&self.pool).await?;
        }
        Ok(())
    }

    async fn get_amortization_schedule(&self, lease_id: Uuid) -> Result<Vec<LeaseAmortizationSchedule>> {
        let rows = sqlx::query_as!(
            LeaseAmortizationSchedule,
            r#"SELECT id, lease_id, period_number, period_start, period_end, opening_liability,
               payment, interest_expense, principal_reduction, closing_liability, opening_rou_asset,
               depreciation_expense, closing_rou_asset, total_expense, currency, created_at
               FROM lease_amortization_schedule WHERE lease_id = ? ORDER BY period_number"#,
            lease_id
        ).fetch_all(&self.pool).await?;
        Ok(rows)
    }

    async fn create_modification(&self, modification: &LeaseModification) -> Result<LeaseModification> {
        let modification = modification.clone();
        sqlx::query!(
            r#"INSERT INTO lease_modifications (id, modification_number, lease_id, modification_date,
               effective_date, modification_type, reason, original_term_months, new_term_months,
               original_payment, new_payment, original_discount_rate, new_discount_rate,
               remeasurement_gain_loss, rou_adjustment, liability_adjustment, currency,
               approved_by, approved_at, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            modification.base.id, modification.modification_number, modification.lease_id,
            modification.modification_date, modification.effective_date, modification.modification_type,
            modification.reason, modification.original_term_months, modification.new_term_months,
            modification.original_payment, modification.new_payment, modification.original_discount_rate,
            modification.new_discount_rate, modification.remeasurement_gain_loss, modification.rou_adjustment,
            modification.liability_adjustment, modification.currency, modification.approved_by,
            modification.approved_at, modification.status, modification.created_at
        ).execute(&self.pool).await?;
        Ok(modification)
    }

    async fn create_expense(&self, expense: &LeaseExpense) -> Result<LeaseExpense> {
        let expense = expense.clone();
        sqlx::query!(
            r#"INSERT INTO lease_expenses (id, lease_id, expense_date, period_start, period_end,
               depreciation_expense, interest_expense, variable_lease_expense, short_term_lease_expense,
               low_value_lease_expense, total_expense, currency, journal_entry_id, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            expense.base.id, expense.lease_id, expense.expense_date, expense.period_start,
            expense.period_end, expense.depreciation_expense, expense.interest_expense,
            expense.variable_lease_expense, expense.short_term_lease_expense, expense.low_value_lease_expense,
            expense.total_expense, expense.currency, expense.journal_entry_id, expense.status, expense.created_at
        ).execute(&self.pool).await?;
        Ok(expense)
    }

    async fn list_expenses(&self, lease_id: Uuid, from: NaiveDate, to: NaiveDate) -> Result<Vec<LeaseExpense>> {
        let rows = sqlx::query_as!(
            LeaseExpense,
            r#"SELECT id as "base: BaseEntity", lease_id, expense_date, period_start, period_end,
               depreciation_expense, interest_expense, variable_lease_expense, short_term_lease_expense,
               low_value_lease_expense, total_expense, currency, journal_entry_id, status, created_at
               FROM lease_expenses WHERE lease_id = ? AND expense_date >= ? AND expense_date <= ?"#,
            lease_id, from, to
        ).fetch_all(&self.pool).await?;
        Ok(rows)
    }

    async fn create_disclosure(&self, disclosure: &LeaseDisclosure) -> Result<LeaseDisclosure> {
        let disclosure = disclosure.clone();
        sqlx::query!(
            r#"INSERT INTO lease_disclosures (id, reporting_period, period_start, period_end,
               total_finance_leases, total_operating_leases, total_rou_assets, total_lease_liabilities,
               total_depreciation, total_interest, total_lease_payments, maturities_within_1_year,
               maturities_1_to_5_years, maturities_after_5_years, total_undiscounted_payments,
               weighted_avg_lease_term, weighted_avg_discount_rate, currency, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            disclosure.base.id, disclosure.reporting_period, disclosure.period_start, disclosure.period_end,
            disclosure.total_finance_leases, disclosure.total_operating_leases, disclosure.total_rou_assets,
            disclosure.total_lease_liabilities, disclosure.total_depreciation, disclosure.total_interest,
            disclosure.total_lease_payments, disclosure.maturities_within_1_year, disclosure.maturities_1_to_5_years,
            disclosure.maturities_after_5_years, disclosure.total_undiscounted_payments,
            disclosure.weighted_avg_lease_term, disclosure.weighted_avg_discount_rate,
            disclosure.currency, disclosure.created_at
        ).execute(&self.pool).await?;
        Ok(disclosure)
    }
}
