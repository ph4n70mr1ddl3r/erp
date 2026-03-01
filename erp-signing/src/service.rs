use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct SigningService {
    document_repo: SqliteSigningDocumentRepository,
    signer_repo: SqliteSignerRepository,
    field_repo: SqliteSignatureFieldRepository,
}

impl Default for SigningService {
    fn default() -> Self {
        Self::new()
    }
}

impl SigningService {
    pub fn new() -> Self {
        Self {
            document_repo: SqliteSigningDocumentRepository,
            signer_repo: SqliteSignerRepository,
            field_repo: SqliteSignatureFieldRepository,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_document(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        document_type: String,
        file_path: String,
        file_name: String,
        file_size: i64,
        file_hash: String,
        pages: i32,
        sender_id: Uuid,
        message: Option<String>,
    ) -> anyhow::Result<SigningDocument> {
        let doc = SigningDocument {
            base: BaseEntity::new(),
            name,
            description,
            document_type,
            file_path,
            file_name,
            file_size,
            file_hash,
            pages,
            status: DocumentStatus::Draft,
            envelope_id: Some(format!("ENV-{}", Uuid::new_v4())),
            sender_id,
            message,
            expires_at: Some(Utc::now() + chrono::Duration::days(30)),
            completed_at: None,
            sent_at: None,
            viewed_at: None,
            reminder_count: 0,
            last_reminder_at: None,
            auto_remind: true,
            remind_days: 3,
            sequential_signing: false,
            current_signer_order: None,
            final_signed_file: None,
            final_signed_at: None,
            audit_trail_file: None,
            certificate_of_completion: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.document_repo.create(pool, &doc).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<SigningDocument>> {
        self.document_repo.get_by_id(pool, id).await
    }

    pub async fn list(&self, pool: &SqlitePool, sender_id: Option<Uuid>, status: Option<DocumentStatus>) -> anyhow::Result<Vec<SigningDocument>> {
        self.document_repo.list(pool, sender_id, status).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn add_signer(
        &self,
        pool: &SqlitePool,
        document_id: Uuid,
        name: String,
        email: String,
        phone: Option<String>,
        user_id: Option<Uuid>,
        role: SignerRole,
        order_index: i32,
    ) -> anyhow::Result<Signer> {
        let access_code = Self::generate_access_code();
        
        let signer = Signer {
            base: BaseEntity::new(),
            document_id,
            name,
            email,
            phone,
            user_id,
            order_index,
            role,
            status: SignerStatus::Pending,
            authentication_method: AuthenticationMethod::Email,
            access_code: Some(access_code),
            viewed_at: None,
            signed_at: None,
            declined_at: None,
            declined_reason: None,
            delegated_to: None,
            email_sent_at: None,
            reminder_sent_at: None,
            signature_ip: None,
            signature_user_agent: None,
            signature_location: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.signer_repo.create(pool, &signer).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn add_signature_field(
        &self,
        pool: &SqlitePool,
        document_id: Uuid,
        signer_id: Uuid,
        field_type: SignatureFieldType,
        page: i32,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        required: bool,
    ) -> anyhow::Result<SignatureField> {
        let field = SignatureField {
            base: BaseEntity::new(),
            document_id,
            signer_id,
            field_type,
            page,
            x_position: x,
            y_position: y,
            width,
            height,
            required,
            value: None,
            signature_data: None,
            signature_type: None,
            signed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.field_repo.create(pool, &field).await
    }

    pub async fn send_for_signature(&self, pool: &SqlitePool, document_id: Uuid) -> anyhow::Result<()> {
        if let Some(mut doc) = self.document_repo.get_by_id(pool, document_id).await? {
            doc.status = DocumentStatus::Sent;
            doc.sent_at = Some(Utc::now());
            self.document_repo.update(pool, &doc).await?;
            
            let signers = self.signer_repo.list_by_document(pool, document_id).await?;
            for signer in signers {
                self.log_audit(pool, document_id, Some(signer.base.id), "email_sent", None).await?;
            }
        }
        Ok(())
    }

    pub async fn view_document(&self, pool: &SqlitePool, document_id: Uuid, signer_id: Uuid) -> anyhow::Result<()> {
        if let Some(mut doc) = self.document_repo.get_by_id(pool, document_id).await? {
            if doc.viewed_at.is_none() {
                doc.viewed_at = Some(Utc::now());
                doc.status = DocumentStatus::Viewed;
                self.document_repo.update(pool, &doc).await?;
            }
        }
        
        if let Some(mut signer) = self.signer_repo.get_by_id(pool, signer_id).await? {
            signer.status = SignerStatus::Viewed;
            signer.viewed_at = Some(Utc::now());
            self.signer_repo.update(pool, &signer).await?;
        }
        
        self.log_audit(pool, document_id, Some(signer_id), "viewed", None).await
    }

    pub async fn sign_field(
        &self,
        pool: &SqlitePool,
        field_id: Uuid,
        value: String,
        signature_data: String,
        signature_type: SignatureType,
    ) -> anyhow::Result<()> {
        self.field_repo.sign(pool, field_id, value, signature_data, signature_type.clone()).await?;
        Ok(())
    }

    pub async fn complete_signing(&self, pool: &SqlitePool, document_id: Uuid, signer_id: Uuid) -> anyhow::Result<()> {
        if let Some(mut signer) = self.signer_repo.get_by_id(pool, signer_id).await? {
            signer.status = SignerStatus::Signed;
            signer.signed_at = Some(Utc::now());
            self.signer_repo.update(pool, &signer).await?;
        }
        
        self.log_audit(pool, document_id, Some(signer_id), "signed", None).await?;
        
        let signers = self.signer_repo.list_by_document(pool, document_id).await?;
        let all_signed = signers.iter().all(|s| s.status == SignerStatus::Signed);
        
        if all_signed {
            if let Some(mut doc) = self.document_repo.get_by_id(pool, document_id).await? {
                doc.status = DocumentStatus::Completed;
                doc.completed_at = Some(Utc::now());
                doc.final_signed_file = Some(format!("/documents/{}/signed.pdf", document_id));
                doc.final_signed_at = Some(Utc::now());
                self.document_repo.update(pool, &doc).await?;
            }
        }
        
        Ok(())
    }

    pub async fn decline(&self, pool: &SqlitePool, document_id: Uuid, signer_id: Uuid, reason: String) -> anyhow::Result<()> {
        if let Some(mut signer) = self.signer_repo.get_by_id(pool, signer_id).await? {
            signer.status = SignerStatus::Declined;
            signer.declined_at = Some(Utc::now());
            signer.declined_reason = Some(reason.clone());
            self.signer_repo.update(pool, &signer).await?;
        }
        
        if let Some(mut doc) = self.document_repo.get_by_id(pool, document_id).await? {
            doc.status = DocumentStatus::Declined;
            self.document_repo.update(pool, &doc).await?;
        }
        
        self.log_audit(pool, document_id, Some(signer_id), "declined", Some(serde_json::json!({"reason": reason}))).await
    }

    fn generate_access_code() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.gen_range(0..999999))
    }

    async fn log_audit(&self, pool: &SqlitePool, document_id: Uuid, signer_id: Option<Uuid>, action: &str, details: Option<serde_json::Value>) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO signing_audit (id, document_id, signer_id, action, details, timestamp) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4())
        .bind(document_id)
        .bind(signer_id)
        .bind(action)
        .bind(details)
        .bind(Utc::now())
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_audit_trail(&self, pool: &SqlitePool, document_id: Uuid) -> anyhow::Result<Vec<SigningAudit>> {
        sqlx::query_as::<_, SigningAudit>("SELECT * FROM signing_audit WHERE document_id = ? ORDER BY timestamp")
            .bind(document_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }
}

pub struct SigningTemplateService;

impl Default for SigningTemplateService {
    fn default() -> Self {
        Self::new()
    }
}

impl SigningTemplateService {
    pub fn new() -> Self {
        Self
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        category: Option<String>,
        template_type: String,
        field_config: serde_json::Value,
        signer_config: serde_json::Value,
        created_by: Uuid,
    ) -> anyhow::Result<SigningTemplate> {
        let now = Utc::now();
        sqlx::query_as::<_, SigningTemplate>(
            r#"INSERT INTO signing_templates (
                id, name, description, category, template_type, document_path, field_config,
                signer_config, message_template, auto_expire_days, remind_days, sequential_signing,
                status, usage_count, created_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(Uuid::new_v4())
        .bind(&name)
        .bind(&description)
        .bind(&category)
        .bind(&template_type)
        .bind(None::<String>)
        .bind(&field_config)
        .bind(&signer_config)
        .bind(None::<String>)
        .bind(30)
        .bind(3)
        .bind(false)
        .bind(erp_core::Status::Active)
        .bind(0i64)
        .bind(created_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<SigningTemplate>> {
        sqlx::query_as::<_, SigningTemplate>("SELECT * FROM signing_templates WHERE status = 'Active' ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }
}
