use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct ClmService {
    repo: SqliteClmRepository,
}

impl Default for ClmService {
    fn default() -> Self {
        Self::new()
    }
}

impl ClmService {
    pub fn new() -> Self {
        Self {
            repo: SqliteClmRepository::new(),
        }
    }
    
    pub async fn create_contract(&self, _pool: &SqlitePool, mut contract: ContractRecord) -> Result<ContractRecord> {
        contract.id = Uuid::new_v4();
        contract.contract_number = format!("CTR-{}", Utc::now().format("%Y%m%d%H%M%S"));
        contract.created_at = Utc::now();
        contract.updated_at = Utc::now();
        contract.status = ContractStatus::Draft;
        
        self.repo.create_contract(&contract).await?;
        Ok(contract)
    }
    
    pub async fn get_contract(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<ContractRecord>> {
        self.repo.get_contract(id).await
    }
    
    pub async fn list_contracts(&self, _pool: &SqlitePool) -> Result<Vec<ContractRecord>> {
        self.repo.list_contracts().await
    }
    
    pub async fn submit_for_approval(&self, _pool: &SqlitePool, contract_id: Uuid, approver_id: Uuid) -> Result<ContractApproval> {
        self.repo.update_contract_status(contract_id, ContractStatus::InReview).await?;
        
        let approval = ContractApproval {
            id: Uuid::new_v4(),
            contract_id,
            approver_id,
            approval_level: 1,
            status: ApprovalStatus::Pending,
            comments: None,
            due_date: None,
            approved_at: None,
            created_at: Utc::now(),
        };
        
        self.repo.create_contract_approval(&approval).await?;
        Ok(approval)
    }
    
    pub async fn approve_contract(&self, _pool: &SqlitePool, _approval_id: Uuid) -> Result<()> {
        Ok(())
    }
    
    pub async fn list_pending_approvals(&self, _pool: &SqlitePool, approver_id: Uuid) -> Result<Vec<ContractApproval>> {
        self.repo.list_pending_approvals(approver_id).await
    }
    
    pub async fn activate_contract(&self, _pool: &SqlitePool, contract_id: Uuid) -> Result<()> {
        self.repo.update_contract_status(contract_id, ContractStatus::Active).await
    }
    
    pub async fn terminate_contract(&self, _pool: &SqlitePool, contract_id: Uuid, _reason: String) -> Result<()> {
        self.repo.update_contract_status(contract_id, ContractStatus::Terminated).await
    }
    
    pub async fn create_renewal(&self, _pool: &SqlitePool, mut renewal: ContractRenewal) -> Result<ContractRenewal> {
        renewal.id = Uuid::new_v4();
        renewal.status = "Pending".to_string();
        renewal.initiated_at = Utc::now();
        
        self.repo.create_contract_renewal(&renewal).await?;
        Ok(renewal)
    }
    
    pub async fn list_expiring_contracts(&self, _pool: &SqlitePool, days: i32) -> Result<Vec<ContractRecord>> {
        self.repo.list_expiring_contracts(days).await
    }
    
    pub async fn assess_contract_risk(&self, _pool: &SqlitePool, contract_id: Uuid) -> Result<ContractRiskAssessment> {
        let assessment = ContractRiskAssessment {
            id: Uuid::new_v4(),
            contract_id,
            assessment_date: Utc::now(),
            financial_risk: "Low".to_string(),
            legal_risk: "Low".to_string(),
            operational_risk: "Low".to_string(),
            compliance_risk: "Low".to_string(),
            overall_risk: ContractRisk::Low,
            mitigation_notes: None,
            assessed_by: None,
        };
        
        Ok(assessment)
    }
}
