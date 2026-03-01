use crate::models::*;
use async_trait::async_trait;
use chrono::NaiveDate;
use erp_core::Result;
use sqlx::{FromRow, SqlitePool};
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

#[derive(Debug, FromRow)]
struct LeaseRow {
    id: String,
    lease_number: String,
    name: String,
    description: Option<String>,
    lease_type: String,
    lessor_id: String,
    lessee_id: String,
    asset_id: Option<String>,
    commencement_date: String,
    end_date: String,
    lease_term_months: i32,
    renewal_option: bool,
    renewal_term_months: Option<i32>,
    termination_option: bool,
    termination_notice_days: Option<i32>,
    purchase_option: bool,
    purchase_option_price: Option<i64>,
    fair_value_at_commencement: Option<i64>,
    residual_value_guarantee: Option<i64>,
    currency: String,
    discount_rate: f64,
    implicit_rate: Option<f64>,
    incremental_borrowing_rate: Option<f64>,
    initial_direct_costs: Option<i64>,
    lease_incentives: Option<i64>,
    decommissioning_provision: Option<i64>,
    status: String,
    classification_date: String,
    classification_by: String,
    created_at: String,
    updated_at: String,
}

impl From<LeaseRow> for Lease {
    fn from(row: LeaseRow) -> Self {
        Lease {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            lease_number: row.lease_number,
            name: row.name,
            description: row.description,
            lease_type: row.lease_type.parse().unwrap_or(LeaseType::Operating),
            lessor_id: Uuid::parse_str(&row.lessor_id).unwrap_or(Uuid::nil()),
            lessee_id: Uuid::parse_str(&row.lessee_id).unwrap_or(Uuid::nil()),
            asset_id: row.asset_id.and_then(|s| Uuid::parse_str(&s).ok()),
            commencement_date: chrono::NaiveDate::parse_from_str(&row.commencement_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            end_date: chrono::NaiveDate::parse_from_str(&row.end_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            lease_term_months: row.lease_term_months,
            renewal_option: row.renewal_option,
            renewal_term_months: row.renewal_term_months,
            termination_option: row.termination_option,
            termination_notice_days: row.termination_notice_days,
            purchase_option: row.purchase_option,
            purchase_option_price: row.purchase_option_price,
            fair_value_at_commencement: row.fair_value_at_commencement.unwrap_or(0),
            residual_value_guarantee: row.residual_value_guarantee,
            currency: row.currency,
            discount_rate: row.discount_rate,
            implicit_rate: row.implicit_rate,
            incremental_borrowing_rate: row.incremental_borrowing_rate,
            initial_direct_costs: row.initial_direct_costs.unwrap_or(0),
            lease_incentives: row.lease_incentives.unwrap_or(0),
            decommissioning_provision: row.decommissioning_provision,
            status: row.status.parse().unwrap_or(LeaseStatus::Draft),
            classification_date: chrono::NaiveDate::parse_from_str(&row.classification_date, "%Y-%m-%d").ok(),
            classification_by: Uuid::parse_str(&row.classification_by).ok(),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[async_trait]
impl LeaseRepository for SqliteLeaseRepository {
    async fn create_lease(&self, lease: &Lease) -> Result<Lease> {
        let lease = lease.clone();
        sqlx::query(
            r#"INSERT INTO leases (id, lease_number, name, description, lease_type, lessor_id, lessee_id,
               asset_id, commencement_date, end_date, lease_term_months, renewal_option, renewal_term_months,
               termination_option, termination_notice_days, purchase_option, purchase_option_price,
               fair_value_at_commencement, residual_value_guarantee, currency, discount_rate, implicit_rate,
               incremental_borrowing_rate, initial_direct_costs, lease_incentives, decommissioning_provision,
               status, classification_date, classification_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(lease.base.id.to_string())
        .bind(&lease.lease_number)
        .bind(&lease.name)
        .bind(&lease.description)
        .bind(&lease.lease_type)
        .bind(lease.lessor_id.to_string())
        .bind(lease.lessee_id.to_string())
        .bind(lease.asset_id.map(|id| id.to_string()))
        .bind(lease.commencement_date.to_string())
        .bind(lease.end_date.to_string())
        .bind(lease.lease_term_months)
        .bind(lease.renewal_option)
        .bind(lease.renewal_term_months)
        .bind(lease.termination_option)
        .bind(lease.termination_notice_days)
        .bind(lease.purchase_option)
        .bind(lease.purchase_option_price)
        .bind(lease.fair_value_at_commencement)
        .bind(lease.residual_value_guarantee)
        .bind(&lease.currency)
        .bind(lease.discount_rate)
        .bind(lease.implicit_rate)
        .bind(lease.incremental_borrowing_rate)
        .bind(lease.initial_direct_costs)
        .bind(lease.lease_incentives)
        .bind(lease.decommissioning_provision)
        .bind(&lease.status)
        .bind(lease.classification_date.map(|d| d.to_string()))
        .bind(lease.classification_by.map(|id| id.to_string()))
        .bind(lease.created_at.to_rfc3339())
        .bind(lease.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(lease)
    }

    async fn get_lease(&self, id: Uuid) -> Result<Option<Lease>> {
        let row: Option<LeaseRow> = sqlx::query_as::<_, LeaseRow>(
            r#"SELECT id, lease_number, name, description, lease_type, lessor_id,
               lessee_id, asset_id, commencement_date, end_date, lease_term_months, renewal_option,
               renewal_term_months, termination_option, termination_notice_days, purchase_option,
               purchase_option_price, fair_value_at_commencement, residual_value_guarantee, currency,
               discount_rate, implicit_rate, incremental_borrowing_rate, initial_direct_costs,
               lease_incentives, decommissioning_provision, status, classification_date, classification_by,
               created_at, updated_at FROM leases WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_leases(&self, status: Option<LeaseStatus>) -> Result<Vec<Lease>> {
        let rows: Vec<LeaseRow> = match status {
            Some(s) => sqlx::query_as::<_, LeaseRow>(
                r#"SELECT id, lease_number, name, description, lease_type, lessor_id,
                   lessee_id, asset_id, commencement_date, end_date, lease_term_months, renewal_option,
                   renewal_term_months, termination_option, termination_notice_days, purchase_option,
                   purchase_option_price, fair_value_at_commencement, residual_value_guarantee, currency,
                   discount_rate, implicit_rate, incremental_borrowing_rate, initial_direct_costs,
                   lease_incentives, decommissioning_provision, status, classification_date, classification_by,
                   created_at, updated_at FROM leases WHERE status = ?"#,
            )
            .bind(format!("{:?}", s))
            .fetch_all(&self.pool).await?,
            None => sqlx::query_as::<_, LeaseRow>(
                r#"SELECT id, lease_number, name, description, lease_type, lessor_id,
                   lessee_id, asset_id, commencement_date, end_date, lease_term_months, renewal_option,
                   renewal_term_months, termination_option, termination_notice_days, purchase_option,
                   purchase_option_price, fair_value_at_commencement, residual_value_guarantee, currency,
                   discount_rate, implicit_rate, incremental_borrowing_rate, initial_direct_costs,
                   lease_incentives, decommissioning_provision, status, classification_date, classification_by,
                   created_at, updated_at FROM leases"#,
            )
            .fetch_all(&self.pool).await?,
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_lease(&self, lease: &Lease) -> Result<Lease> {
        let lease = lease.clone();
        sqlx::query(
            r#"UPDATE leases SET name = ?, description = ?, status = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&lease.name)
        .bind(&lease.description)
        .bind(&lease.status)
        .bind(lease.updated_at.to_rfc3339())
        .bind(lease.base.id.to_string())
        .execute(&self.pool).await?;
        Ok(lease)
    }

    async fn create_payment(&self, payment: &LeasePayment) -> Result<LeasePayment> {
        let payment = payment.clone();
        sqlx::query(
            r#"INSERT INTO lease_payments (id, lease_id, payment_number, payment_date, due_date,
               period_start, period_end, period_number, fixed_payment, variable_payment,
               escalation_amount, total_payment, currency, payment_status, paid_date, paid_amount, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(payment.base.id.to_string())
        .bind(payment.lease_id.to_string())
        .bind(&payment.payment_number)
        .bind(payment.payment_date.to_string())
        .bind(payment.due_date.to_string())
        .bind(payment.period_start.to_string())
        .bind(payment.period_end.to_string())
        .bind(payment.period_number)
        .bind(payment.fixed_payment)
        .bind(payment.variable_payment)
        .bind(payment.escalation_amount)
        .bind(payment.total_payment)
        .bind(&payment.currency)
        .bind(&payment.payment_status)
        .bind(payment.paid_date.map(|d| d.to_string()))
        .bind(payment.paid_amount)
        .bind(&payment.notes)
        .bind(payment.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(payment)
    }

    async fn list_payments(&self, lease_id: Uuid) -> Result<Vec<LeasePayment>> {
        let rows: Vec<LeasePaymentRow> = sqlx::query_as::<_, LeasePaymentRow>(
            r#"SELECT id, lease_id, payment_number, payment_date, due_date,
               period_start, period_end, period_number, fixed_payment, variable_payment,
               escalation_amount, total_payment, currency, payment_status, paid_date, paid_amount, notes, created_at
               FROM lease_payments WHERE lease_id = ? ORDER BY period_number"#,
        )
        .bind(lease_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_payment_schedule(&self, schedule: &LeasePaymentSchedule) -> Result<LeasePaymentSchedule> {
        let schedule = schedule.clone();
        sqlx::query(
            r#"INSERT INTO lease_payment_schedules (id, lease_id, schedule_number, effective_date,
               payment_frequency, payment_timing, payment_day, base_payment, escalation_type,
               escalation_rate, escalation_frequency_months, first_escalation_date, cap_amount,
               floor_amount, currency, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(schedule.base.id.to_string())
        .bind(schedule.lease_id.to_string())
        .bind(&schedule.schedule_number)
        .bind(schedule.effective_date.to_string())
        .bind(&schedule.payment_frequency)
        .bind(&schedule.payment_timing)
        .bind(schedule.payment_day)
        .bind(schedule.base_payment)
        .bind(&schedule.escalation_type)
        .bind(schedule.escalation_rate)
        .bind(schedule.escalation_frequency_months)
        .bind(schedule.first_escalation_date.map(|d| d.to_string()))
        .bind(schedule.cap_amount)
        .bind(schedule.floor_amount)
        .bind(&schedule.currency)
        .bind(&schedule.status)
        .bind(schedule.created_at.to_rfc3339())
        .bind(schedule.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(schedule)
    }

    async fn create_rou_asset(&self, asset: &RightOfUseAsset) -> Result<RightOfUseAsset> {
        let asset = asset.clone();
        sqlx::query(
            r#"INSERT INTO right_of_use_assets (id, lease_id, asset_number, name, initial_cost,
               accumulated_depreciation, impairment_loss, net_book_value, depreciation_method,
               useful_life_months, residual_value, depreciation_start_date, depreciation_end_date,
               currency, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(asset.base.id.to_string())
        .bind(asset.lease_id.to_string())
        .bind(&asset.asset_number)
        .bind(&asset.name)
        .bind(asset.initial_cost)
        .bind(asset.accumulated_depreciation)
        .bind(asset.impairment_loss)
        .bind(asset.net_book_value)
        .bind(&asset.depreciation_method)
        .bind(asset.useful_life_months)
        .bind(asset.residual_value)
        .bind(Some(asset.depreciation_start_date.to_string()))
        .bind(Some(asset.depreciation_end_date.to_string()))
        .bind(&asset.currency)
        .bind(&asset.status)
        .bind(asset.created_at.to_rfc3339())
        .bind(asset.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(asset)
    }

    async fn get_rou_asset(&self, lease_id: Uuid) -> Result<Option<RightOfUseAsset>> {
        let row: Option<ROUAssetRow> = sqlx::query_as::<_, ROUAssetRow>(
            r#"SELECT id, lease_id, asset_number, name, initial_cost,
               accumulated_depreciation, impairment_loss, net_book_value, depreciation_method,
               useful_life_months, residual_value, depreciation_start_date, depreciation_end_date,
               currency, status, created_at, updated_at FROM right_of_use_assets WHERE lease_id = ?"#,
        )
        .bind(lease_id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn create_liability(&self, liability: &LeaseLiability) -> Result<LeaseLiability> {
        let liability = liability.clone();
        sqlx::query(
            r#"INSERT INTO lease_liabilities (id, lease_id, liability_number, initial_liability,
               outstanding_balance, interest_accrued, principal_paid, currency, calculation_date,
               amortization_method, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(liability.base.id.to_string())
        .bind(liability.lease_id.to_string())
        .bind(&liability.liability_number)
        .bind(liability.initial_liability)
        .bind(liability.outstanding_balance)
        .bind(liability.interest_accrued)
        .bind(liability.principal_paid)
        .bind(&liability.currency)
        .bind(liability.calculation_date.to_string())
        .bind(&liability.amortization_method)
        .bind(&liability.status)
        .bind(liability.created_at.to_rfc3339())
        .bind(liability.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(liability)
    }

    async fn get_liability(&self, lease_id: Uuid) -> Result<Option<LeaseLiability>> {
        let row: Option<LeaseLiabilityRow> = sqlx::query_as::<_, LeaseLiabilityRow>(
            r#"SELECT id, lease_id, liability_number, initial_liability,
               outstanding_balance, interest_accrued, principal_paid, currency, calculation_date,
               amortization_method, status, created_at, updated_at FROM lease_liabilities WHERE lease_id = ?"#,
        )
        .bind(lease_id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn create_amortization_schedule(&self, schedule: &[LeaseAmortizationSchedule]) -> Result<()> {
        for item in schedule {
            sqlx::query(
                r#"INSERT INTO lease_amortization_schedule (id, lease_id, period_number, period_start,
                   period_end, opening_liability, payment, interest_expense, principal_reduction,
                   closing_liability, opening_rou_asset, depreciation_expense, closing_rou_asset,
                   total_expense, currency, created_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            )
            .bind(item.id.to_string())
            .bind(item.lease_id.to_string())
            .bind(item.period_number)
            .bind(item.period_start.to_string())
            .bind(item.period_end.to_string())
            .bind(item.opening_liability)
            .bind(item.payment)
            .bind(item.interest_expense)
            .bind(item.principal_reduction)
            .bind(item.closing_liability)
            .bind(item.opening_rou_asset)
            .bind(item.depreciation_expense)
            .bind(item.closing_rou_asset)
            .bind(item.total_expense)
            .bind(&item.currency)
            .bind(item.created_at.to_rfc3339())
            .execute(&self.pool).await?;
        }
        Ok(())
    }

    async fn get_amortization_schedule(&self, lease_id: Uuid) -> Result<Vec<LeaseAmortizationSchedule>> {
        let rows: Vec<LeaseAmortizationScheduleRow> = sqlx::query_as::<_, LeaseAmortizationScheduleRow>(
            r#"SELECT id, lease_id, period_number, period_start, period_end, opening_liability,
               payment, interest_expense, principal_reduction, closing_liability, opening_rou_asset,
               depreciation_expense, closing_rou_asset, total_expense, currency, created_at
               FROM lease_amortization_schedule WHERE lease_id = ? ORDER BY period_number"#,
        )
        .bind(lease_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_modification(&self, modification: &LeaseModification) -> Result<LeaseModification> {
        let modification = modification.clone();
        sqlx::query(
            r#"INSERT INTO lease_modifications (id, modification_number, lease_id, modification_date,
               effective_date, modification_type, reason, original_term_months, new_term_months,
               original_payment, new_payment, original_discount_rate, new_discount_rate,
               remeasurement_gain_loss, rou_adjustment, liability_adjustment, currency,
               approved_by, approved_at, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(modification.base.id.to_string())
        .bind(&modification.modification_number)
        .bind(modification.lease_id.to_string())
        .bind(modification.modification_date.to_string())
        .bind(modification.effective_date.to_string())
        .bind(&modification.modification_type)
        .bind(&modification.reason)
        .bind(modification.original_term_months)
        .bind(modification.new_term_months)
        .bind(modification.original_payment)
        .bind(modification.new_payment)
        .bind(modification.original_discount_rate)
        .bind(modification.new_discount_rate)
        .bind(modification.remeasurement_gain_loss)
        .bind(modification.rou_adjustment)
        .bind(modification.liability_adjustment)
        .bind(&modification.currency)
        .bind(modification.approved_by.map(|id| id.to_string()))
        .bind(modification.approved_at.map(|d| d.to_rfc3339()))
        .bind(&modification.status)
        .bind(modification.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(modification)
    }

    async fn create_expense(&self, expense: &LeaseExpense) -> Result<LeaseExpense> {
        let expense = expense.clone();
        sqlx::query(
            r#"INSERT INTO lease_expenses (id, lease_id, expense_date, period_start, period_end,
               depreciation_expense, interest_expense, variable_lease_expense, short_term_lease_expense,
               low_value_lease_expense, total_expense, currency, journal_entry_id, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(expense.base.id.to_string())
        .bind(expense.lease_id.to_string())
        .bind(expense.expense_date.to_string())
        .bind(expense.period_start.to_string())
        .bind(expense.period_end.to_string())
        .bind(expense.depreciation_expense)
        .bind(expense.interest_expense)
        .bind(expense.variable_lease_expense)
        .bind(expense.short_term_lease_expense)
        .bind(expense.low_value_lease_expense)
        .bind(expense.total_expense)
        .bind(&expense.currency)
        .bind(expense.journal_entry_id.map(|id| id.to_string()))
        .bind(&expense.status)
        .bind(expense.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(expense)
    }

    async fn list_expenses(&self, lease_id: Uuid, from: NaiveDate, to: NaiveDate) -> Result<Vec<LeaseExpense>> {
        let rows: Vec<LeaseExpenseRow> = sqlx::query_as::<_, LeaseExpenseRow>(
            r#"SELECT id, lease_id, expense_date, period_start, period_end,
               depreciation_expense, interest_expense, variable_lease_expense, short_term_lease_expense,
               low_value_lease_expense, total_expense, currency, journal_entry_id, status, created_at
               FROM lease_expenses WHERE lease_id = ? AND expense_date >= ? AND expense_date <= ?"#,
        )
        .bind(lease_id.to_string())
        .bind(from.to_string())
        .bind(to.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_disclosure(&self, disclosure: &LeaseDisclosure) -> Result<LeaseDisclosure> {
        let disclosure = disclosure.clone();
        sqlx::query(
            r#"INSERT INTO lease_disclosures (id, reporting_period, period_start, period_end,
               total_finance_leases, total_operating_leases, total_rou_assets, total_lease_liabilities,
               total_depreciation, total_interest, total_lease_payments, maturities_within_1_year,
               maturities_1_to_5_years, maturities_after_5_years, total_undiscounted_payments,
               weighted_avg_lease_term, weighted_avg_discount_rate, currency, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(disclosure.base.id.to_string())
        .bind(&disclosure.reporting_period)
        .bind(disclosure.period_start.to_string())
        .bind(disclosure.period_end.to_string())
        .bind(disclosure.total_finance_leases)
        .bind(disclosure.total_operating_leases)
        .bind(disclosure.total_rou_assets)
        .bind(disclosure.total_lease_liabilities)
        .bind(disclosure.total_depreciation)
        .bind(disclosure.total_interest)
        .bind(disclosure.total_lease_payments)
        .bind(disclosure.maturities_within_1_year)
        .bind(disclosure.maturities_1_to_5_years)
        .bind(disclosure.maturities_after_5_years)
        .bind(disclosure.total_undiscounted_payments)
        .bind(disclosure.weighted_avg_lease_term)
        .bind(disclosure.weighted_avg_discount_rate)
        .bind(&disclosure.currency)
        .bind(disclosure.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(disclosure)
    }
}

// Row structs for FromRow
#[derive(Debug, FromRow)]
struct LeasePaymentRow {
    id: String,
    lease_id: String,
    payment_number: i32,
    payment_date: String,
    due_date: String,
    period_start: String,
    period_end: String,
    period_number: i32,
    fixed_payment: i64,
    variable_payment: i64,
    escalation_amount: i64,
    total_payment: i64,
    currency: String,
    payment_status: String,
    paid_date: Option<String>,
    paid_amount: Option<i64>,
    notes: Option<String>,
    created_at: String,
}

impl From<LeasePaymentRow> for LeasePayment {
    fn from(row: LeasePaymentRow) -> Self {
        LeasePayment {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            lease_id: Uuid::parse_str(&row.lease_id).unwrap_or(Uuid::nil()),
            payment_number: row.payment_number.to_string(),
            payment_date: chrono::NaiveDate::parse_from_str(&row.payment_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            due_date: chrono::NaiveDate::parse_from_str(&row.due_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            period_start: chrono::NaiveDate::parse_from_str(&row.period_start, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            period_end: chrono::NaiveDate::parse_from_str(&row.period_end, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            period_number: row.period_number,
            fixed_payment: row.fixed_payment,
            variable_payment: row.variable_payment,
            escalation_amount: row.escalation_amount,
            total_payment: row.total_payment,
            currency: row.currency,
            payment_status: row.payment_status.parse().unwrap_or(PaymentStatus::Scheduled),
            paid_date: row.paid_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            paid_amount: row.paid_amount,
            notes: row.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ROUAssetRow {
    id: String,
    lease_id: String,
    asset_number: String,
    name: String,
    initial_cost: i64,
    accumulated_depreciation: i64,
    impairment_loss: Option<i64>,
    net_book_value: i64,
    depreciation_method: String,
    useful_life_months: i32,
    residual_value: i64,
    depreciation_start_date: Option<String>,
    depreciation_end_date: Option<String>,
    currency: String,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<ROUAssetRow> for RightOfUseAsset {
    fn from(row: ROUAssetRow) -> Self {
        RightOfUseAsset {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            lease_id: Uuid::parse_str(&row.lease_id).unwrap_or(Uuid::nil()),
            asset_number: row.asset_number,
            name: row.name,
            initial_cost: row.initial_cost,
            accumulated_depreciation: row.accumulated_depreciation,
            impairment_loss: row.impairment_loss.unwrap_or(0),
            net_book_value: row.net_book_value,
            depreciation_method: row.depreciation_method.parse().unwrap_or(DepreciationMethod::StraightLine),
            useful_life_months: row.useful_life_months,
            residual_value: row.residual_value,
            depreciation_start_date: row.depreciation_start_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()).unwrap_or_else(|| chrono::Utc::now().date_naive()),
            depreciation_end_date: row.depreciation_end_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()).unwrap_or_else(|| chrono::Utc::now().date_naive()),
            currency: row.currency,
            status: row.status.parse().unwrap_or(erp_core::Status::Active),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct LeaseLiabilityRow {
    id: String,
    lease_id: String,
    liability_number: String,
    initial_liability: i64,
    outstanding_balance: i64,
    interest_accrued: i64,
    principal_paid: i64,
    currency: String,
    calculation_date: String,
    amortization_method: String,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<LeaseLiabilityRow> for LeaseLiability {
    fn from(row: LeaseLiabilityRow) -> Self {
        LeaseLiability {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            lease_id: Uuid::parse_str(&row.lease_id).unwrap_or(Uuid::nil()),
            liability_number: row.liability_number,
            initial_liability: row.initial_liability,
            outstanding_balance: row.outstanding_balance,
            interest_accrued: row.interest_accrued,
            principal_paid: row.principal_paid,
            currency: row.currency,
            calculation_date: chrono::NaiveDate::parse_from_str(&row.calculation_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            amortization_method: row.amortization_method.parse().unwrap_or(AmortizationMethod::EffectiveInterest),
            status: row.status.parse().unwrap_or(erp_core::Status::Active),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct LeaseAmortizationScheduleRow {
    id: String,
    lease_id: String,
    period_number: i32,
    period_start: String,
    period_end: String,
    opening_liability: i64,
    payment: i64,
    interest_expense: i64,
    principal_reduction: i64,
    closing_liability: i64,
    opening_rou_asset: i64,
    depreciation_expense: i64,
    closing_rou_asset: i64,
    total_expense: i64,
    currency: String,
    created_at: String,
}

impl From<LeaseAmortizationScheduleRow> for LeaseAmortizationSchedule {
    fn from(row: LeaseAmortizationScheduleRow) -> Self {
        LeaseAmortizationSchedule {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            lease_id: Uuid::parse_str(&row.lease_id).unwrap_or(Uuid::nil()),
            period_number: row.period_number,
            period_start: chrono::NaiveDate::parse_from_str(&row.period_start, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            period_end: chrono::NaiveDate::parse_from_str(&row.period_end, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            opening_liability: row.opening_liability,
            payment: row.payment,
            interest_expense: row.interest_expense,
            principal_reduction: row.principal_reduction,
            closing_liability: row.closing_liability,
            opening_rou_asset: row.opening_rou_asset,
            depreciation_expense: row.depreciation_expense,
            closing_rou_asset: row.closing_rou_asset,
            total_expense: row.total_expense,
            currency: row.currency,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct LeaseExpenseRow {
    id: String,
    lease_id: String,
    expense_date: String,
    period_start: String,
    period_end: String,
    depreciation_expense: i64,
    interest_expense: i64,
    variable_lease_expense: i64,
    short_term_lease_expense: i64,
    low_value_lease_expense: i64,
    total_expense: i64,
    currency: String,
    journal_entry_id: Option<String>,
    status: String,
    created_at: String,
}

impl From<LeaseExpenseRow> for LeaseExpense {
    fn from(row: LeaseExpenseRow) -> Self {
        LeaseExpense {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            lease_id: Uuid::parse_str(&row.lease_id).unwrap_or(Uuid::nil()),
            expense_date: chrono::NaiveDate::parse_from_str(&row.expense_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            period_start: chrono::NaiveDate::parse_from_str(&row.period_start, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            period_end: chrono::NaiveDate::parse_from_str(&row.period_end, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            depreciation_expense: row.depreciation_expense,
            interest_expense: row.interest_expense,
            variable_lease_expense: row.variable_lease_expense,
            short_term_lease_expense: row.short_term_lease_expense,
            low_value_lease_expense: row.low_value_lease_expense,
            total_expense: row.total_expense,
            currency: row.currency,
            journal_entry_id: row.journal_entry_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: row.status.parse().unwrap_or(erp_core::Status::Active),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
