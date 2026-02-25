use chrono::{DateTime, Utc};
use erp_core::error::{Error, Result};
use erp_core::models::{BaseEntity, Money, Currency, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct GrantService {
    grant_repo: SqliteGrantRepository,
}

impl GrantService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            grant_repo: SqliteGrantRepository::new(pool),
        }
    }

    pub async fn create_grant(
        &self,
        pool: &SqlitePool,
        grant_number: String,
        title: String,
        grant_type: GrantType,
        funding_source: FundingSource,
        funder_name: String,
        total_award: i64,
        indirect_cost_rate: f64,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        principal_investigator_id: Uuid,
    ) -> Result<Grant> {
        if start_date >= end_date {
            return Err(Error::validation("Start date must be before end date".to_string()));
        }
        if indirect_cost_rate < 0.0 || indirect_cost_rate > 100.0 {
            return Err(Error::validation("Invalid indirect cost rate".to_string()));
        }
        let grant = Grant {
            base: BaseEntity::new(),
            grant_number,
            title,
            description: None,
            grant_type,
            status: GrantStatus::Draft,
            funding_source,
            funder_name,
            funder_contact: None,
            total_award_amount: Money::new(total_award, Currency::USD),
            currency: "USD".to_string(),
            indirect_cost_rate,
            start_date,
            end_date,
            principal_investigator_id,
            department_id: None,
            program_id: None,
            cfda_number: None,
            award_number: None,
            is_cost_sharing: false,
            cost_sharing_amount: None,
            reporting_frequency: ReportingFrequency::Quarterly,
            next_report_due: None,
            compliance_requirements: Vec::new(),
            special_conditions: None,
        };
        self.grant_repo.create(&grant).await
    }

    pub async fn get_grant(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<Grant>> {
        self.grant_repo.find_by_id(id).await
    }

    pub async fn submit_grant(&self, pool: &SqlitePool, id: Uuid) -> Result<Grant> {
        let mut grant = self.grant_repo.find_by_id(id).await?
            .ok_or(Error::not_found("grant", &id.to_string()))?;
        if grant.status != GrantStatus::Draft {
            return Err(Error::validation("Only draft grants can be submitted".to_string()));
        }
        grant.status = GrantStatus::Submitted;
        self.grant_repo.update(&grant).await
    }

    pub async fn award_grant(&self, pool: &SqlitePool, id: Uuid, award_number: String) -> Result<Grant> {
        let mut grant = self.grant_repo.find_by_id(id).await?
            .ok_or(Error::not_found("grant", &id.to_string()))?;
        grant.status = GrantStatus::Awarded;
        grant.award_number = Some(award_number);
        self.grant_repo.update(&grant).await
    }

    pub async fn activate_grant(&self, pool: &SqlitePool, id: Uuid) -> Result<Grant> {
        let mut grant = self.grant_repo.find_by_id(id).await?
            .ok_or(Error::not_found("grant", &id.to_string()))?;
        if grant.status != GrantStatus::Awarded {
            return Err(Error::validation("Only awarded grants can be activated".to_string()));
        }
        grant.status = GrantStatus::Active;
        self.grant_repo.update(&grant).await
    }

    pub async fn calculate_indirect_costs(&self, direct_costs: i64, rate: f64) -> i64 {
        ((direct_costs as f64) * rate / 100.0).round() as i64
    }

    pub async fn check_budget_balance(&self, pool: &SqlitePool, grant_id: Uuid) -> Result<BudgetBalance> {
        Ok(BudgetBalance {
            grant_id,
            total_award: 0,
            total_budgeted: 0,
            total_expended: 0,
            total_encumbered: 0,
            available_balance: 0,
            percent_utilized: 0.0,
        })
    }
}

pub struct BudgetBalance {
    pub grant_id: Uuid,
    pub total_award: i64,
    pub total_budgeted: i64,
    pub total_expended: i64,
    pub total_encumbered: i64,
    pub available_balance: i64,
    pub percent_utilized: f64,
}

pub struct GrantBudgetService {
    pool: SqlitePool,
}

impl GrantBudgetService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_budget_line(
        &self,
        grant_id: Uuid,
        category: BudgetCategory,
        description: String,
        approved_amount: i64,
    ) -> Result<GrantBudget> {
        let budget = GrantBudget {
            base: BaseEntity::new(),
            grant_id,
            budget_category: category,
            description,
            approved_amount: Money::new(approved_amount, Currency::USD),
            budgeted_amount: Money::new(approved_amount, Currency::USD),
            expended_amount: Money::new(0, Currency::USD),
            encumbered_amount: Money::new(0, Currency::USD),
            available_balance: Money::new(approved_amount, Currency::USD),
            notes: None,
        };
        Ok(budget)
    }

    pub async fn modify_budget(
        &self,
        budget_id: Uuid,
        new_amount: i64,
        justification: String,
    ) -> Result<GrantBudget> {
        Ok(GrantBudget {
            base: BaseEntity::new(),
            grant_id: Uuid::nil(),
            budget_category: BudgetCategory::OtherDirect,
            description: String::new(),
            approved_amount: Money::new(new_amount, Currency::USD),
            budgeted_amount: Money::new(new_amount, Currency::USD),
            expended_amount: Money::new(0, Currency::USD),
            encumbered_amount: Money::new(0, Currency::USD),
            available_balance: Money::new(new_amount, Currency::USD),
            notes: Some(justification),
        })
    }
}

pub struct GrantTransactionService {
    pool: SqlitePool,
}

impl GrantTransactionService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn record_expenditure(
        &self,
        grant_id: Uuid,
        category: BudgetCategory,
        amount: i64,
        description: String,
        reference_number: Option<String>,
        invoice_id: Option<Uuid>,
    ) -> Result<GrantTransaction> {
        let transaction = GrantTransaction {
            base: BaseEntity::new(),
            grant_id,
            budget_category: category,
            transaction_type: TransactionType::Expenditure,
            transaction_date: Utc::now(),
            amount: Money::new(amount, Currency::USD),
            description,
            reference_number,
            invoice_id,
            journal_entry_id: None,
            approved_by: None,
            cost_sharing_flag: false,
        };
        Ok(transaction)
    }

    pub async fn create_encumbrance(
        &self,
        grant_id: Uuid,
        category: BudgetCategory,
        amount: i64,
        description: String,
    ) -> Result<GrantTransaction> {
        let transaction = GrantTransaction {
            base: BaseEntity::new(),
            grant_id,
            budget_category: category,
            transaction_type: TransactionType::Encumbrance,
            transaction_date: Utc::now(),
            amount: Money::new(amount, Currency::USD),
            description,
            reference_number: None,
            invoice_id: None,
            journal_entry_id: None,
            approved_by: None,
            cost_sharing_flag: false,
        };
        Ok(transaction)
    }

    pub async fn record_cost_transfer(
        &self,
        from_grant_id: Uuid,
        to_grant_id: Uuid,
        amount: i64,
        justification: String,
    ) -> Result<(GrantTransaction, GrantTransaction)> {
        let debit = GrantTransaction {
            base: BaseEntity::new(),
            grant_id: from_grant_id,
            budget_category: BudgetCategory::OtherDirect,
            transaction_type: TransactionType::CostTransfer,
            transaction_date: Utc::now(),
            amount: Money::new(-amount, Currency::USD),
            description: format!("Transfer to {}: {}", to_grant_id, justification),
            reference_number: None,
            invoice_id: None,
            journal_entry_id: None,
            approved_by: None,
            cost_sharing_flag: false,
        };
        let credit = GrantTransaction {
            base: BaseEntity::new(),
            grant_id: to_grant_id,
            budget_category: BudgetCategory::OtherDirect,
            transaction_type: TransactionType::CostTransfer,
            transaction_date: Utc::now(),
            amount: Money::new(amount, Currency::USD),
            description: format!("Transfer from {}: {}", from_grant_id, justification),
            reference_number: None,
            invoice_id: None,
            journal_entry_id: None,
            approved_by: None,
            cost_sharing_flag: false,
        };
        Ok((debit, credit))
    }
}

pub struct GrantReportService {
    pool: SqlitePool,
}

impl GrantReportService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_report(
        &self,
        grant_id: Uuid,
        report_type: ReportType,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        due_date: DateTime<Utc>,
    ) -> Result<GrantReport> {
        let report = GrantReport {
            base: BaseEntity::new(),
            grant_id,
            report_type,
            reporting_period_start: period_start,
            reporting_period_end: period_end,
            due_date,
            submitted_date: None,
            status: ReportStatus::Draft,
            prepared_by: None,
            approved_by: None,
            notes: None,
            attachment_ids: Vec::new(),
        };
        Ok(report)
    }

    pub async fn submit_report(&self, report_id: Uuid, prepared_by: Uuid) -> Result<GrantReport> {
        Ok(GrantReport {
            base: BaseEntity::new(),
            grant_id: Uuid::nil(),
            report_type: ReportType::Financial,
            reporting_period_start: Utc::now(),
            reporting_period_end: Utc::now(),
            due_date: Utc::now(),
            submitted_date: Some(Utc::now()),
            status: ReportStatus::Submitted,
            prepared_by: Some(prepared_by),
            approved_by: None,
            notes: None,
            attachment_ids: Vec::new(),
        })
    }

    pub async fn get_upcoming_reports(&self, days: i32) -> Result<Vec<GrantReport>> {
        Ok(Vec::new())
    }
}

pub struct GrantComplianceService {
    pool: SqlitePool,
}

impl GrantComplianceService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_compliance_requirement(
        &self,
        grant_id: Uuid,
        requirement_type: ComplianceRequirement,
        description: String,
        due_date: Option<DateTime<Utc>>,
        responsible_party: Option<Uuid>,
    ) -> Result<GrantCompliance> {
        let compliance = GrantCompliance {
            base: BaseEntity::new(),
            grant_id,
            requirement_type,
            description,
            is_mandatory: true,
            due_date,
            completed_date: None,
            status: ComplianceStatus::Pending,
            responsible_party,
            documentation_ids: Vec::new(),
        };
        Ok(compliance)
    }

    pub async fn mark_compliant(&self, id: Uuid) -> Result<GrantCompliance> {
        Ok(GrantCompliance {
            base: BaseEntity::new(),
            grant_id: Uuid::nil(),
            requirement_type: ComplianceRequirement::HumanSubjects,
            description: String::new(),
            is_mandatory: true,
            due_date: None,
            completed_date: Some(Utc::now()),
            status: ComplianceStatus::Compliant,
            responsible_party: None,
            documentation_ids: Vec::new(),
        })
    }

    pub async fn check_compliance_status(&self, grant_id: Uuid) -> Result<ComplianceStatusSummary> {
        Ok(ComplianceStatusSummary {
            grant_id,
            total_requirements: 0,
            compliant: 0,
            pending: 0,
            non_compliant: 0,
            overall_status: ComplianceStatus::Compliant,
        })
    }
}

pub struct ComplianceStatusSummary {
    pub grant_id: Uuid,
    pub total_requirements: i32,
    pub compliant: i32,
    pub pending: i32,
    pub non_compliant: i32,
    pub overall_status: ComplianceStatus,
}

pub struct GrantCloseoutService {
    pool: SqlitePool,
}

impl GrantCloseoutService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn initiate_closeout(&self, grant_id: Uuid) -> Result<GrantCloseout> {
        let closeout = GrantCloseout {
            base: BaseEntity::new(),
            grant_id,
            closeout_date: Utc::now(),
            final_expenditure: Money::new(0, Currency::USD),
            unexpended_balance: Money::new(0, Currency::USD),
            final_report_submitted: false,
            equipment_inventory_complete: false,
            inventions_reported: false,
            subawards_closed: false,
            status: CloseoutStatus::Initiated,
            closed_by: None,
            notes: None,
        };
        Ok(closeout)
    }

    pub async fn complete_closeout(&self, closeout_id: Uuid, closed_by: Uuid) -> Result<GrantCloseout> {
        Ok(GrantCloseout {
            base: BaseEntity::new(),
            grant_id: Uuid::nil(),
            closeout_date: Utc::now(),
            final_expenditure: Money::new(0, Currency::USD),
            unexpended_balance: Money::new(0, Currency::USD),
            final_report_submitted: true,
            equipment_inventory_complete: true,
            inventions_reported: true,
            subawards_closed: true,
            status: CloseoutStatus::Completed,
            closed_by: Some(closed_by),
            notes: None,
        })
    }
}
