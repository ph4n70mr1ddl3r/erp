use chrono::Utc;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct IntercompanyService<R: IntercompanyRepository> {
    pub repo: R,
}

impl IntercompanyService<SqliteIntercompanyRepository> {
    pub fn new(repo: SqliteIntercompanyRepository) -> Self {
        Self { repo }
    }
}

impl<R: IntercompanyRepository> IntercompanyService<R> {
    pub async fn create_entity(&self, req: CreateEntityRequest) -> anyhow::Result<IntercompanyEntity> {
        let entity = IntercompanyEntity {
            id: Uuid::new_v4(),
            code: req.code,
            name: req.name,
            legal_entity_id: req.legal_entity_id,
            currency: req.currency,
            timezone: "UTC".to_string(),
            tax_id: None,
            status: EntityStatus::Active,
            created_at: Utc::now(),
        };
        self.repo.create_entity(&entity).await?;
        Ok(entity)
    }

    pub async fn create_transaction(&self, req: CreateICTransactionRequest) -> anyhow::Result<IntercompanyTransaction> {
        let exchange_rate = req.exchange_rate.unwrap_or(1.0);
        let target_amount = (req.source_amount as f64 * exchange_rate) as i64;
        let transaction_number = format!("IC-{}", Utc::now().format("%Y%m%d%H%M%S"));

        let txn = IntercompanyTransaction {
            id: Uuid::new_v4(),
            transaction_number,
            source_entity_id: req.source_entity_id,
            target_entity_id: req.target_entity_id,
            transaction_type: req.transaction_type,
            source_amount: req.source_amount,
            source_currency: req.source_currency,
            target_amount,
            target_currency: req.target_currency,
            exchange_rate,
            status: ICTransactionStatus::Draft,
            source_document_id: None,
            target_document_id: None,
            due_date: req.due_date,
            created_at: Utc::now(),
            settled_at: None,
        };
        self.repo.create_transaction(&txn).await?;
        Ok(txn)
    }

    pub async fn settle_transaction(&self, id: Uuid) -> anyhow::Result<IntercompanyTransaction> {
        self.repo.update_transaction_status(id, ICTransactionStatus::Settled).await?;
        let txn = self.repo.get_transaction(id).await?.ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

        let due_to = DueToFrom {
            id: Uuid::new_v4(),
            source_entity_id: txn.source_entity_id,
            target_entity_id: txn.target_entity_id,
            account_type: DueToFromType::DueTo,
            amount: txn.target_amount,
            currency: txn.target_currency.clone(),
            as_of_date: Utc::now().date_naive(),
            created_at: Utc::now(),
        };
        self.repo.create_due_to_from(&due_to).await?;

        Ok(txn)
    }

    pub async fn create_transfer_price(&self, req: CreateTransferPriceRequest) -> anyhow::Result<TransferPrice> {
        let price = TransferPrice {
            id: Uuid::new_v4(),
            product_id: req.product_id,
            source_entity_id: req.source_entity_id,
            target_entity_id: req.target_entity_id,
            price: req.price,
            currency: req.currency,
            method: req.method,
            effective_from: req.effective_from,
            effective_to: None,
            approved_by: None,
            created_at: Utc::now(),
        };
        self.repo.create_transfer_price(&price).await?;
        Ok(price)
    }

    pub async fn run_consolidation(&self, req: RunConsolidationRequest) -> anyhow::Result<ConsolidationResult> {
        let consolidation_number = format!("CONS-{}", Utc::now().format("%Y%m%d%H%M%S"));
        let consolidation = Consolidation {
            id: Uuid::new_v4(),
            consolidation_number,
            period: req.period,
            status: ConsolidationStatus::InProgress,
            entities: req.entity_ids.clone(),
            elimination_entries: vec![],
            started_at: Utc::now(),
            completed_at: None,
            created_by: Uuid::nil(),
        };
        self.repo.create_consolidation(&consolidation).await?;

        let mut elimination_count = 0;
        let mut total_eliminations = 0i64;

        for entity_id in &req.entity_ids {
            let txns = self.repo.list_transactions(Some(*entity_id), None).await?;
            for txn in txns {
                if matches!(txn.status, ICTransactionStatus::Settled) {
                    let entry = EliminationEntry {
                        id: Uuid::new_v4(),
                        consolidation_id: consolidation.id,
                        source_transaction_id: txn.id,
                        debit_entity_id: txn.target_entity_id,
                        credit_entity_id: txn.source_entity_id,
                        account_code: "IC-ELIM".to_string(),
                        amount: txn.target_amount,
                        currency: txn.target_currency,
                        description: format!("Elimination for {}", txn.transaction_number),
                        created_at: Utc::now(),
                    };
                    self.repo.create_elimination(&entry).await?;
                    elimination_count += 1;
                    total_eliminations += entry.amount;
                }
            }
        }

        self.repo.update_consolidation_status(consolidation.id, ConsolidationStatus::Completed).await?;

        Ok(ConsolidationResult {
            consolidation_id: consolidation.id,
            elimination_count,
            total_eliminations,
            status: ConsolidationStatus::Completed,
        })
    }

    pub async fn create_agreement(&self, source: Uuid, target: Uuid, name: String, agreement_type: AgreementType) -> anyhow::Result<IntercompanyAgreement> {
        let agreement_number = format!("ICA-{}", Utc::now().format("%Y%m%d%H%M"));
        let agreement = IntercompanyAgreement {
            id: Uuid::new_v4(),
            agreement_number,
            name,
            source_entity_id: source,
            target_entity_id: target,
            agreement_type,
            start_date: Utc::now().date_naive(),
            end_date: None,
            terms: serde_json::json!({}),
            status: AgreementStatus::Active,
            created_at: Utc::now(),
        };
        self.repo.create_agreement(&agreement).await?;
        Ok(agreement)
    }
}
