use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::BaseEntity;

use crate::models::*;

// Row structs for sqlx::FromRow - these match the database schema
#[derive(sqlx::FromRow)]
struct SigningDocumentRow {
    id: String,
    name: String,
    description: Option<String>,
    document_type: String,
    file_path: String,
    file_name: String,
    file_size: i64,
    file_hash: String,
    pages: i32,
    status: DocumentStatus,
    envelope_id: Option<String>,
    sender_id: String,
    message: Option<String>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
    sent_at: Option<chrono::DateTime<chrono::Utc>>,
    viewed_at: Option<chrono::DateTime<chrono::Utc>>,
    reminder_count: i32,
    last_reminder_at: Option<chrono::DateTime<chrono::Utc>>,
    auto_remind: bool,
    remind_days: i32,
    sequential_signing: bool,
    current_signer_order: Option<i32>,
    final_signed_file: Option<String>,
    final_signed_at: Option<chrono::DateTime<chrono::Utc>>,
    audit_trail_file: Option<String>,
    certificate_of_completion: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<SigningDocumentRow> for SigningDocument {
    type Error = anyhow::Error;

    fn try_from(row: SigningDocumentRow) -> Result<Self, Self::Error> {
        Ok(SigningDocument {
            base: BaseEntity {
                id: Uuid::parse_str(&row.id)?,
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            description: row.description,
            document_type: row.document_type,
            file_path: row.file_path,
            file_name: row.file_name,
            file_size: row.file_size,
            file_hash: row.file_hash,
            pages: row.pages,
            status: row.status,
            envelope_id: row.envelope_id,
            sender_id: Uuid::parse_str(&row.sender_id)?,
            message: row.message,
            expires_at: row.expires_at,
            completed_at: row.completed_at,
            sent_at: row.sent_at,
            viewed_at: row.viewed_at,
            reminder_count: row.reminder_count,
            last_reminder_at: row.last_reminder_at,
            auto_remind: row.auto_remind,
            remind_days: row.remind_days,
            sequential_signing: row.sequential_signing,
            current_signer_order: row.current_signer_order,
            final_signed_file: row.final_signed_file,
            final_signed_at: row.final_signed_at,
            audit_trail_file: row.audit_trail_file,
            certificate_of_completion: row.certificate_of_completion,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[derive(sqlx::FromRow)]
struct SignerRow {
    id: String,
    document_id: String,
    name: String,
    email: String,
    phone: Option<String>,
    user_id: Option<String>,
    order_index: i32,
    role: SignerRole,
    status: SignerStatus,
    authentication_method: AuthenticationMethod,
    access_code: Option<String>,
    viewed_at: Option<chrono::DateTime<chrono::Utc>>,
    signed_at: Option<chrono::DateTime<chrono::Utc>>,
    declined_at: Option<chrono::DateTime<chrono::Utc>>,
    declined_reason: Option<String>,
    delegated_to: Option<String>,
    email_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    reminder_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    signature_ip: Option<String>,
    signature_user_agent: Option<String>,
    signature_location: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<SignerRow> for Signer {
    type Error = anyhow::Error;

    fn try_from(row: SignerRow) -> Result<Self, Self::Error> {
        Ok(Signer {
            base: BaseEntity {
                id: Uuid::parse_str(&row.id)?,
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: None,
                updated_by: None,
            },
            document_id: Uuid::parse_str(&row.document_id)?,
            name: row.name,
            email: row.email,
            phone: row.phone,
            user_id: row.user_id.as_ref().and_then(|s| Uuid::parse_str(s).ok()),
            order_index: row.order_index,
            role: row.role,
            status: row.status,
            authentication_method: row.authentication_method,
            access_code: row.access_code,
            viewed_at: row.viewed_at,
            signed_at: row.signed_at,
            declined_at: row.declined_at,
            declined_reason: row.declined_reason,
            delegated_to: row.delegated_to.as_ref().and_then(|s| Uuid::parse_str(s).ok()),
            email_sent_at: row.email_sent_at,
            reminder_sent_at: row.reminder_sent_at,
            signature_ip: row.signature_ip,
            signature_user_agent: row.signature_user_agent,
            signature_location: row.signature_location,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[derive(sqlx::FromRow)]
struct SignatureFieldRow {
    id: String,
    document_id: String,
    signer_id: String,
    field_type: SignatureFieldType,
    page: i32,
    x_position: f64,
    y_position: f64,
    width: f64,
    height: f64,
    required: bool,
    value: Option<String>,
    signature_data: Option<String>,
    signature_type: Option<SignatureType>,
    signed_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<SignatureFieldRow> for SignatureField {
    type Error = anyhow::Error;

    fn try_from(row: SignatureFieldRow) -> Result<Self, Self::Error> {
        Ok(SignatureField {
            base: BaseEntity {
                id: Uuid::parse_str(&row.id)?,
                created_at: row.created_at,
                updated_at: row.updated_at,
                created_by: None,
                updated_by: None,
            },
            document_id: Uuid::parse_str(&row.document_id)?,
            signer_id: Uuid::parse_str(&row.signer_id)?,
            field_type: row.field_type,
            page: row.page,
            x_position: row.x_position,
            y_position: row.y_position,
            width: row.width,
            height: row.height,
            required: row.required,
            value: row.value,
            signature_data: row.signature_data,
            signature_type: row.signature_type,
            signed_at: row.signed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[async_trait]
pub trait SigningDocumentRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, doc: &SigningDocument) -> anyhow::Result<SigningDocument>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<SigningDocument>>;
    async fn list(&self, pool: &SqlitePool, sender_id: Option<Uuid>, status: Option<DocumentStatus>) -> anyhow::Result<Vec<SigningDocument>>;
    async fn update(&self, pool: &SqlitePool, doc: &SigningDocument) -> anyhow::Result<()>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: DocumentStatus) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteSigningDocumentRepository;

#[async_trait]
impl SigningDocumentRepository for SqliteSigningDocumentRepository {
    async fn create(&self, pool: &SqlitePool, doc: &SigningDocument) -> anyhow::Result<SigningDocument> {
        let now = Utc::now();
        let id_str = doc.base.id.to_string();
        let sender_id_str = doc.sender_id.to_string();
        sqlx::query(
            r#"INSERT INTO signing_documents (
                id, name, description, document_type, file_path, file_name, file_size, file_hash,
                pages, status, envelope_id, sender_id, message, expires_at, completed_at, sent_at,
                viewed_at, reminder_count, last_reminder_at, auto_remind, remind_days, sequential_signing,
                current_signer_order, final_signed_file, final_signed_at, audit_trail_file,
                certificate_of_completion, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id_str)
        .bind(&doc.name)
        .bind(&doc.description)
        .bind(&doc.document_type)
        .bind(&doc.file_path)
        .bind(&doc.file_name)
        .bind(doc.file_size)
        .bind(&doc.file_hash)
        .bind(doc.pages)
        .bind(&doc.status)
        .bind(&doc.envelope_id)
        .bind(&sender_id_str)
        .bind(&doc.message)
        .bind(doc.expires_at)
        .bind(doc.completed_at)
        .bind(doc.sent_at)
        .bind(doc.viewed_at)
        .bind(doc.reminder_count)
        .bind(doc.last_reminder_at)
        .bind(doc.auto_remind)
        .bind(doc.remind_days)
        .bind(doc.sequential_signing)
        .bind(doc.current_signer_order)
        .bind(&doc.final_signed_file)
        .bind(doc.final_signed_at)
        .bind(&doc.audit_trail_file)
        .bind(&doc.certificate_of_completion)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        self.get_by_id(pool, doc.base.id).await?.ok_or_else(|| anyhow::anyhow!("Failed to create document"))
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<SigningDocument>> {
        let row: Option<SigningDocumentRow> = sqlx::query_as::<_, SigningDocumentRow>(
            "SELECT * FROM signing_documents WHERE id = ?"
        )
            .bind(id.to_string())
            .fetch_optional(pool)
            .await?;
        
        match row {
            Some(r) => Ok(Some(r.try_into()?)),
            None => Ok(None),
        }
    }

    async fn list(&self, pool: &SqlitePool, sender_id: Option<Uuid>, status: Option<DocumentStatus>) -> anyhow::Result<Vec<SigningDocument>> {
        let mut query = "SELECT * FROM signing_documents WHERE 1=1".to_string();
        if sender_id.is_some() { query.push_str(" AND sender_id = ?"); }
        if status.is_some() { query.push_str(" AND status = ?"); }
        query.push_str(" ORDER BY created_at DESC");
        
        let mut q = sqlx::query_as::<_, SigningDocumentRow>(&query);
        if let Some(sid) = sender_id { q = q.bind(sid.to_string()); }
        if let Some(s) = status { q = q.bind(s); }
        let rows: Vec<SigningDocumentRow> = q.fetch_all(pool).await?;
        rows.into_iter().map(|r| r.try_into()).collect()
    }

    async fn update(&self, pool: &SqlitePool, doc: &SigningDocument) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signing_documents SET name=?, description=?, status=?, updated_at=? WHERE id=?")
            .bind(&doc.name)
            .bind(&doc.description)
            .bind(&doc.status)
            .bind(now)
            .bind(doc.base.id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: DocumentStatus) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signing_documents SET status=?, updated_at=? WHERE id=?")
            .bind(&status)
            .bind(now)
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM signing_documents WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait SignerRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, signer: &Signer) -> anyhow::Result<Signer>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Signer>>;
    async fn list_by_document(&self, pool: &SqlitePool, document_id: Uuid) -> anyhow::Result<Vec<Signer>>;
    async fn update(&self, pool: &SqlitePool, signer: &Signer) -> anyhow::Result<()>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: SignerStatus) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteSignerRepository;

#[async_trait]
impl SignerRepository for SqliteSignerRepository {
    async fn create(&self, pool: &SqlitePool, signer: &Signer) -> anyhow::Result<Signer> {
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO signers (
                id, document_id, name, email, phone, user_id, order_index, role, status,
                authentication_method, access_code, viewed_at, signed_at, declined_at,
                declined_reason, delegated_to, email_sent_at, reminder_sent_at, signature_ip,
                signature_user_agent, signature_location, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(signer.base.id.to_string())
        .bind(signer.document_id.to_string())
        .bind(&signer.name)
        .bind(&signer.email)
        .bind(&signer.phone)
        .bind(signer.user_id.map(|id| id.to_string()))
        .bind(signer.order_index)
        .bind(&signer.role)
        .bind(&signer.status)
        .bind(&signer.authentication_method)
        .bind(&signer.access_code)
        .bind(signer.viewed_at)
        .bind(signer.signed_at)
        .bind(signer.declined_at)
        .bind(&signer.declined_reason)
        .bind(signer.delegated_to.map(|id| id.to_string()))
        .bind(signer.email_sent_at)
        .bind(signer.reminder_sent_at)
        .bind(&signer.signature_ip)
        .bind(&signer.signature_user_agent)
        .bind(&signer.signature_location)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        self.get_by_id(pool, signer.base.id).await?.ok_or_else(|| anyhow::anyhow!("Failed to create signer"))
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Signer>> {
        let row: Option<SignerRow> = sqlx::query_as::<_, SignerRow>("SELECT * FROM signers WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool)
            .await?;
        
        match row {
            Some(r) => Ok(Some(r.try_into()?)),
            None => Ok(None),
        }
    }

    async fn list_by_document(&self, pool: &SqlitePool, document_id: Uuid) -> anyhow::Result<Vec<Signer>> {
        let rows: Vec<SignerRow> = sqlx::query_as::<_, SignerRow>("SELECT * FROM signers WHERE document_id = ? ORDER BY order_index")
            .bind(document_id.to_string())
            .fetch_all(pool)
            .await?;
        rows.into_iter().map(|r| r.try_into()).collect()
    }

    async fn update(&self, pool: &SqlitePool, signer: &Signer) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signers SET status=?, updated_at=? WHERE id=?")
            .bind(&signer.status)
            .bind(now)
            .bind(signer.base.id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: SignerStatus) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signers SET status=?, updated_at=? WHERE id=?")
            .bind(&status)
            .bind(now)
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM signers WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait SignatureFieldRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, field: &SignatureField) -> anyhow::Result<SignatureField>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<SignatureField>>;
    async fn list_by_document(&self, pool: &SqlitePool, document_id: Uuid) -> anyhow::Result<Vec<SignatureField>>;
    async fn list_by_signer(&self, pool: &SqlitePool, signer_id: Uuid) -> anyhow::Result<Vec<SignatureField>>;
    async fn sign(&self, pool: &SqlitePool, id: Uuid, value: String, signature_data: String, signature_type: SignatureType) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteSignatureFieldRepository;

#[async_trait]
impl SignatureFieldRepository for SqliteSignatureFieldRepository {
    async fn create(&self, pool: &SqlitePool, field: &SignatureField) -> anyhow::Result<SignatureField> {
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO signature_fields (
                id, document_id, signer_id, field_type, page, x_position, y_position,
                width, height, required, value, signature_data, signature_type, signed_at, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(field.base.id.to_string())
        .bind(field.document_id.to_string())
        .bind(field.signer_id.to_string())
        .bind(&field.field_type)
        .bind(field.page)
        .bind(field.x_position)
        .bind(field.y_position)
        .bind(field.width)
        .bind(field.height)
        .bind(field.required)
        .bind(&field.value)
        .bind(&field.signature_data)
        .bind(&field.signature_type)
        .bind(field.signed_at)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        self.get_by_id(pool, field.base.id).await?.ok_or_else(|| anyhow::anyhow!("Failed to create field"))
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<SignatureField>> {
        let row: Option<SignatureFieldRow> = sqlx::query_as::<_, SignatureFieldRow>("SELECT * FROM signature_fields WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool)
            .await?;
        
        match row {
            Some(r) => Ok(Some(r.try_into()?)),
            None => Ok(None),
        }
    }

    async fn list_by_document(&self, pool: &SqlitePool, document_id: Uuid) -> anyhow::Result<Vec<SignatureField>> {
        let rows: Vec<SignatureFieldRow> = sqlx::query_as::<_, SignatureFieldRow>("SELECT * FROM signature_fields WHERE document_id = ? ORDER BY page, y_position, x_position")
            .bind(document_id.to_string())
            .fetch_all(pool)
            .await?;
        rows.into_iter().map(|r| r.try_into()).collect()
    }

    async fn list_by_signer(&self, pool: &SqlitePool, signer_id: Uuid) -> anyhow::Result<Vec<SignatureField>> {
        let rows: Vec<SignatureFieldRow> = sqlx::query_as::<_, SignatureFieldRow>("SELECT * FROM signature_fields WHERE signer_id = ? ORDER BY page, y_position, x_position")
            .bind(signer_id.to_string())
            .fetch_all(pool)
            .await?;
        rows.into_iter().map(|r| r.try_into()).collect()
    }

    async fn sign(&self, pool: &SqlitePool, id: Uuid, value: String, signature_data: String, signature_type: SignatureType) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signature_fields SET value=?, signature_data=?, signature_type=?, signed_at=?, updated_at=? WHERE id=?")
            .bind(&value)
            .bind(&signature_data)
            .bind(&signature_type)
            .bind(now)
            .bind(now)
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM signature_fields WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }
}
