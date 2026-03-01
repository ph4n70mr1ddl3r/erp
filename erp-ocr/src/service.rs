use anyhow::Result;
use uuid::Uuid;
use erp_core::models::BaseEntity;
use crate::models::*;
use crate::repository::{OcrRepository, SqliteOcrRepository};
use chrono::Utc;
use regex::Regex;

pub struct OcrService {
    repo: SqliteOcrRepository,
}

impl Default for OcrService {
    fn default() -> Self {
        Self::new()
    }
}

impl OcrService {
    pub fn new() -> Self {
        Self {
            repo: SqliteOcrRepository::new(),
        }
    }
    
    pub async fn upload_document(&self, request: UploadDocumentRequest) -> Result<OcrDocument> {
        let content_bytes = base64::decode(request.content.replace("data:", "").split(",").last().unwrap_or(""))?;
        
        let doc = OcrDocument {
            base: BaseEntity::new(),
            document_type: request.document_type,
            original_filename: request.filename,
            file_path: format!("ocr/{}", Uuid::new_v4()),
            file_size: content_bytes.len() as i64,
            mime_type: "application/pdf".to_string(),
            status: if request.auto_process { OcrStatus::Processing } else { OcrStatus::Pending },
            processing_started_at: if request.auto_process { Some(Utc::now()) } else { None },
            processing_completed_at: None,
            confidence_score: None,
            raw_text: None,
            extracted_data: None,
            validation_errors: Vec::new(),
            reviewed_by: None,
            reviewed_at: None,
        };
        
        self.repo.create_document(&doc).await?;
        
        if request.auto_process {
            let doc_id = doc.base.id;
            tokio::spawn(async move {
                let service = OcrService::new();
                let _ = service.process_document(doc_id).await;
            });
        }
        
        Ok(doc)
    }
    
    pub async fn process_document(&self, document_id: Uuid) -> Result<OcrResult> {
        let mut doc = self.repo.get_document(document_id).await?
            .ok_or_else(|| anyhow::anyhow!("Document not found"))?;
        
        doc.status = OcrStatus::Processing;
        doc.processing_started_at = Some(Utc::now());
        self.repo.update_document(&doc).await?;
        
        let (extracted_data, raw_text, confidence) = self.perform_ocr(&doc).await?;
        
        let validation_errors = self.validate_extracted_data(&extracted_data, &doc.document_type)?;
        
        doc.raw_text = Some(raw_text);
        doc.extracted_data = Some(extracted_data.clone());
        doc.confidence_score = Some(confidence);
        doc.processing_completed_at = Some(Utc::now());
        
        if validation_errors.is_empty() && confidence >= 0.8 {
            doc.status = OcrStatus::Completed;
        } else if confidence < 0.5 {
            doc.status = OcrStatus::Failed;
        } else {
            doc.status = OcrStatus::RequiresReview;
        }
        
        doc.validation_errors = validation_errors.clone();
        self.repo.update_document(&doc).await?;
        
        Ok(OcrResult {
            document_id,
            status: doc.status.clone(),
            extracted_data: Some(extracted_data),
            raw_text: doc.raw_text.clone(),
            confidence,
            validation_errors,
            suggestions: self.generate_suggestions(&doc),
        })
    }
    
    async fn perform_ocr(&self, doc: &OcrDocument) -> Result<(serde_json::Value, String, f64)> {
        let simulated_text = match doc.document_type {
            DocumentType::Invoice => {
                "Invoice Number: INV-2024-001\nDate: 2024-01-15\nVendor: ABC Corp\nTotal: $1,250.00"
            }
            DocumentType::Receipt => {
                "Store: Main Street Shop\nDate: 2024-01-15\nTotal: $45.99"
            }
            _ => "Document content extracted"
        };
        
        let extracted = match doc.document_type {
            DocumentType::Invoice => {
                self.extract_invoice_data(simulated_text)?
            }
            DocumentType::Receipt => {
                self.extract_receipt_data(simulated_text)?
            }
            _ => serde_json::json!({"text": simulated_text})
        };
        
        Ok((extracted, simulated_text.to_string(), 0.92))
    }
    
    fn extract_invoice_data(&self, text: &str) -> Result<serde_json::Value> {
        let invoice_number = Regex::new(r"Invoice\s*(?:Number|No\.?)?\s*:?\s*([A-Z0-9-]+)")?
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_default();
        
        let date = Regex::new(r"Date\s*:?\s*(\d{4}-\d{2}-\d{2})")?
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_default();
        
        let vendor = Regex::new(r"Vendor\s*:?\s*(.+)")?
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .unwrap_or_default();
        
        let total = Regex::new(r"Total\s*:?\s*\$?([\d,]+\.?\d*)")?
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().replace(",", "").parse::<f64>().unwrap_or(0.0)))
            .unwrap_or(0.0);
        
        Ok(serde_json::json!({
            "invoice_number": invoice_number,
            "invoice_date": date,
            "vendor_name": vendor,
            "total_amount_cents": (total * 100.0) as i64,
            "currency": "USD",
            "line_items": []
        }))
    }
    
    fn extract_receipt_data(&self, text: &str) -> Result<serde_json::Value> {
        let store = Regex::new(r"Store\s*:?\s*(.+)")?
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
            .unwrap_or_default();
        
        let date = Regex::new(r"Date\s*:?\s*(\d{4}-\d{2}-\d{2})")?
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .unwrap_or_default();
        
        let total = Regex::new(r"Total\s*:?\s*\$?([\d,]+\.?\d*)")?
            .captures(text)
            .and_then(|c| c.get(1).map(|m| m.as_str().replace(",", "").parse::<f64>().unwrap_or(0.0)))
            .unwrap_or(0.0);
        
        Ok(serde_json::json!({
            "merchant_name": store,
            "receipt_date": date,
            "total_amount_cents": (total * 100.0) as i64,
            "currency": "USD"
        }))
    }
    
    fn validate_extracted_data(&self, data: &serde_json::Value, doc_type: &DocumentType) -> Result<Vec<String>> {
        let mut errors = Vec::new();
        
        match doc_type {
            DocumentType::Invoice => {
                if data.get("invoice_number").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    errors.push("Invoice number is missing".to_string());
                }
                if data.get("total_amount_cents").and_then(|v| v.as_i64()).unwrap_or(0) <= 0 {
                    errors.push("Invalid total amount".to_string());
                }
            }
            DocumentType::Receipt => {
                if data.get("merchant_name").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
                    errors.push("Merchant name is missing".to_string());
                }
            }
            _ => {}
        }
        
        Ok(errors)
    }
    
    fn generate_suggestions(&self, doc: &OcrDocument) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        match doc.status {
            OcrStatus::RequiresReview => {
                suggestions.push("Manual review recommended due to low confidence".to_string());
            }
            OcrStatus::Completed => {
                if doc.document_type == DocumentType::Invoice {
                    suggestions.push("Ready to create vendor invoice".to_string());
                }
            }
            _ => {}
        }
        
        suggestions
    }
    
    pub async fn get_document(&self, id: Uuid) -> Result<Option<OcrDocument>> {
        self.repo.get_document(id).await
    }
    
    pub async fn list_documents(&self, status: Option<OcrStatus>, limit: i64, offset: i64) -> Result<Vec<OcrDocument>> {
        self.repo.list_documents(status, limit, offset).await
    }
    
    pub async fn review_document(&self, id: Uuid, reviewer_id: Uuid, corrections: Option<serde_json::Value>) -> Result<OcrDocument> {
        let mut doc = self.repo.get_document(id).await?
            .ok_or_else(|| anyhow::anyhow!("Document not found"))?;
        
        if let Some(corr) = corrections {
            doc.extracted_data = Some(corr);
        }
        
        doc.reviewed_by = Some(reviewer_id);
        doc.reviewed_at = Some(Utc::now());
        doc.status = OcrStatus::Validated;
        
        self.repo.update_document(&doc).await?;
        Ok(doc)
    }
    
    pub async fn create_template(&self, template: OcrTemplate) -> Result<OcrTemplate> {
        self.repo.create_template(&template).await?;
        Ok(template)
    }
    
    pub async fn list_templates(&self, document_type: Option<DocumentType>) -> Result<Vec<OcrTemplate>> {
        self.repo.list_templates(document_type).await
    }
    
    pub async fn create_batch_job(&self, name: String, document_ids: Vec<Uuid>, template_id: Option<Uuid>) -> Result<OcrBatchJob> {
        let job = OcrBatchJob {
            base: BaseEntity::new(),
            name,
            document_ids: document_ids.clone(),
            template_id,
            status: BatchJobStatus::Pending,
            total_documents: document_ids.len() as i32,
            processed_documents: 0,
            successful_documents: 0,
            failed_documents: 0,
            started_at: None,
            completed_at: None,
            error_details: serde_json::json!({}),
        };
        
        self.repo.create_batch_job(&job).await?;
        Ok(job)
    }
    
    pub async fn get_settings(&self) -> Result<OcrSettings> {
        self.repo.get_settings().await
    }
    
    pub async fn update_settings(&self, settings: OcrSettings) -> Result<()> {
        self.repo.update_settings(&settings).await
    }
}

mod base64 {
    pub fn decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.decode(input)
    }
}
