use chrono::Utc;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct EdiService<R: EdiRepository> {
    pub repo: R,
}

impl EdiService<SqliteEdiRepository> {
    pub fn new(repo: SqliteEdiRepository) -> Self {
        Self { repo }
    }
}

impl<R: EdiRepository> EdiService<R> {
    pub async fn create_partner(&self, req: CreatePartnerRequest) -> anyhow::Result<EdiPartner> {
        let partner = EdiPartner {
            id: Uuid::new_v4(),
            partner_code: req.partner_code,
            partner_name: req.partner_name,
            partner_type: req.partner_type,
            qualifier: req.qualifier,
            interchange_id: req.interchange_id,
            communication_type: req.communication_type,
            endpoint: req.endpoint,
            encryption: req.encryption,
            is_active: true,
            created_at: Utc::now(),
        };
        self.repo.create_partner(&partner).await?;
        Ok(partner)
    }

    pub async fn process_inbound(&self, req: ProcessEdiRequest) -> anyhow::Result<EdiTransaction> {
        let control_number = format!("IC-{}", Utc::now().format("%Y%m%d%H%M%S"));
        let txn = EdiTransaction {
            id: Uuid::new_v4(),
            partner_id: req.partner_id,
            transaction_type: EdiTransactionType::X12_850,
            direction: EdiDirection::Inbound,
            control_number,
            status: EdiStatus::Received,
            raw_content: Some(req.raw_content),
            parsed_data: None,
            error_message: None,
            processed_at: None,
            created_at: Utc::now(),
        };
        self.repo.create_transaction(&txn).await?;
        Ok(txn)
    }

    pub async fn generate_outbound(&self, req: GenerateEdiRequest) -> anyhow::Result<EdiTransmissionResult> {
        let control_number = format!("OC-{}", Utc::now().format("%Y%m%d%H%M%S"));
        let raw_content = format!("ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *{}*1234*U*00401*000000001*0*P*>~", Utc::now().format("%y%m%d"));
        
        let txn = EdiTransaction {
            id: Uuid::new_v4(),
            partner_id: req.partner_id,
            transaction_type: req.transaction_type,
            direction: EdiDirection::Outbound,
            control_number: control_number.clone(),
            status: EdiStatus::Processed,
            raw_content: Some(raw_content.clone()),
            parsed_data: None,
            error_message: None,
            processed_at: Some(Utc::now()),
            created_at: Utc::now(),
        };
        self.repo.create_transaction(&txn).await?;

        Ok(EdiTransmissionResult {
            transaction_id: txn.id,
            control_number,
            raw_content,
            sent_at: Utc::now(),
        })
    }

    pub async fn list_partners(&self, partner_type: Option<PartnerType>) -> anyhow::Result<Vec<EdiPartner>> {
        self.repo.list_partners(partner_type).await
    }

    pub async fn list_transactions(&self, partner_id: Option<Uuid>, txn_type: Option<EdiTransactionType>) -> anyhow::Result<Vec<EdiTransaction>> {
        self.repo.list_transactions(partner_id, txn_type).await
    }
}
