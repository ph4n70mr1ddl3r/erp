use chrono::Utc;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct RevRecService<R: RevRecRepository> {
    pub repo: R,
}

impl<R: RevRecRepository> RevRecService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: RevRecRepository> RevRecService<R> {
    pub async fn create_contract(&self, req: CreateContractRequest) -> anyhow::Result<RevenueContract> {
        let contract_number = format!("CTR-{}", Utc::now().format("%Y%m%d%H%M"));
        let mut contract = RevenueContract {
            id: Uuid::new_v4(),
            contract_number,
            customer_id: req.customer_id,
            contract_date: req.contract_date,
            start_date: req.start_date,
            end_date: req.end_date,
            total_value: req.total_value,
            currency: req.currency,
            status: ContractStatus::Draft,
            performance_obligations: req.obligations.into_iter().map(|o| PerformanceObligation {
                id: Uuid::new_v4(),
                contract_id: Uuid::nil(), // Will be set after contract ID is known
                name: o.name,
                description: o.description,
                standalone_price: o.standalone_price,
                allocated_price: 0, // Will be calculated by allocation rule
                recognition_type: o.recognition_type,
                recognition_method: o.recognition_method,
                total_periods: o.total_periods,
                status: ObligationStatus::NotStarted,
                created_at: Utc::now(),
            }).collect(),
            transaction_price: req.total_value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Set the contract_id for each obligation
        for obligation in &mut contract.performance_obligations {
            obligation.contract_id = contract.id;
        }

        // Apply allocation if more than one obligation or if StandalonePrice rule exists
        self.allocate_transaction_price(&mut contract).await?;

        self.repo.create_contract(&contract).await?;
        for obligation in &contract.performance_obligations {
            self.repo.create_obligation(obligation).await?;
        }

        Ok(contract)
    }

    /// Allocates the total transaction price to performance obligations based on their Relative Standalone Selling Price (RSSP).
    /// This is a key requirement for ASC 606 / IFRS 15 compliance in enterprise ERP systems.
    pub async fn allocate_transaction_price(&self, contract: &mut RevenueContract) -> anyhow::Result<()> {
        if contract.performance_obligations.is_empty() {
            return Ok(());
        }

        let total_standalone: i64 = contract.performance_obligations.iter().map(|o| o.standalone_price).sum();
        
        if total_standalone == 0 {
            // If no standalone prices, allocate equally (fallback)
            let count = contract.performance_obligations.len();
            let equal_share = contract.transaction_price / count as i64;
            for (i, obligation) in contract.performance_obligations.iter_mut().enumerate() {
                if i == count - 1 {
                    // Adjust last one for rounding
                    let allocated_so_far: i64 = equal_share * (count as i64 - 1);
                    obligation.allocated_price = contract.transaction_price - allocated_so_far;
                } else {
                    obligation.allocated_price = equal_share;
                }
            }
            return Ok(());
        }

        // Relative Standalone Selling Price (RSSP) Allocation
        let mut total_allocated = 0;
        let count = contract.performance_obligations.len();
        
        for (i, obligation) in contract.performance_obligations.iter_mut().enumerate() {
            if i == count - 1 {
                // Adjust last one for rounding to ensure exact transaction price allocation
                obligation.allocated_price = contract.transaction_price - total_allocated;
            } else {
                let ratio = obligation.standalone_price as f64 / total_standalone as f64;
                let allocated = (contract.transaction_price as f64 * ratio).round() as i64;
                obligation.allocated_price = allocated;
                total_allocated += allocated;
            }
        }

        Ok(())
    }

    /// Calculates revenue to be recognized for a Performance Obligation based on Percent of Completion (POC).
    /// Used for project-based contracts in commercial ERPs.
    pub async fn calculate_poc_revenue(
        &self, 
        obligation_id: Uuid, 
        costs_incurred: i64, 
        total_estimated_costs: i64
    ) -> anyhow::Result<i64> {
        let obligation = self.repo.get_obligation(obligation_id).await?
            .ok_or_else(|| anyhow::anyhow!("Performance obligation not found"))?;

        if obligation.recognition_method != RecognitionMethod::PercentageComplete {
            return Err(anyhow::anyhow!("Obligation does not use PercentageComplete recognition method"));
        }

        if total_estimated_costs == 0 {
            return Err(anyhow::anyhow!("Total estimated costs cannot be zero for POC calculation"));
        }

        let poc = costs_incurred as f64 / total_estimated_costs as f64;
        let poc = poc.min(1.0); // Cannot recognize more than 100%

        let total_revenue_to_date = (obligation.allocated_price as f64 * poc).round() as i64;
        
        // Subtract already recognized revenue
        let events = self.repo.list_events(obligation.contract_id).await?;
        let already_recognized: i64 = events.iter()
            .filter(|e| e.obligation_id == Some(obligation_id))
            .map(|e| e.amount)
            .sum();

        let to_recognize = total_revenue_to_date - already_recognized;
        Ok(to_recognize.max(0))
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
