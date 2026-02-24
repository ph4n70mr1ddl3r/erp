use chrono::Utc;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct RevRecService<R: RevRecRepository> {
    pub repo: R,
}

impl RevRecService<SqliteRevRecRepository> {
    pub fn new(repo: SqliteRevRecRepository) -> Self {
        Self { repo }
    }
}

impl<R: RevRecRepository> RevRecService<R> {
    pub async fn create_contract(&self, req: CreateContractRequest) -> anyhow::Result<RevenueContract> {
        let contract_number = format!("CTR-{}", Utc::now().format("%Y%m%d%H%M"));
        let contract = RevenueContract {
            id: Uuid::new_v4(),
            contract_number,
            customer_id: req.customer_id,
            contract_date: req.contract_date,
            start_date: req.start_date,
            end_date: req.end_date,
            total_value: req.total_value,
            currency: req.currency,
            status: ContractStatus::Draft,
            performance_obligations: vec![],
            transaction_price: req.total_value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_contract(&contract).await?;
        Ok(contract)
    }

    pub async fn get_contract(&self, id: Uuid) -> anyhow::Result<Option<RevenueContract>> {
        self.repo.get_contract(id).await
    }

    pub async fn recognize_revenue(&self, req: RecognizeRevenueRequest) -> anyhow::Result<RevenueEvent> {
        let event = RevenueEvent {
            id: Uuid::new_v4(),
            contract_id: req.contract_id,
            obligation_id: req.obligation_id,
            event_type: req.event_type,
            amount: req.amount,
            currency: "USD".to_string(),
            event_date: req.event_date,
            period: req.event_date,
            description: req.description,
            journal_entry_id: None,
            created_at: Utc::now(),
        };
        self.repo.create_event(&event).await?;
        Ok(event)
    }

    pub async fn get_waterfall(&self, contract_id: Uuid) -> anyhow::Result<RevenueWaterfall> {
        let contract = self.repo.get_contract(contract_id).await?.ok_or_else(|| anyhow::anyhow!("Contract not found"))?;
        let events = self.repo.list_events(contract_id).await?;

        let periods: Vec<WaterfallPeriod> = events.iter().map(|e| WaterfallPeriod {
            period: e.period,
            beginning_deferred: 0,
            new_revenue: 0,
            recognized: e.amount,
            ending_deferred: 0,
        }).collect();

        Ok(RevenueWaterfall {
            contract_id,
            periods,
            total_contract_value: contract.total_value,
            total_recognized: 0,
            total_deferred: contract.total_value,
        })
    }

    pub async fn create_allocation_rule(&self, name: String, method: AllocationMethod, basis: AllocationBasis) -> anyhow::Result<AllocationRule> {
        let rule = AllocationRule {
            id: Uuid::new_v4(),
            name,
            description: None,
            method,
            basis,
            is_active: true,
            created_at: Utc::now(),
        };
        self.repo.create_allocation_rule(&rule).await?;
        Ok(rule)
    }
}
