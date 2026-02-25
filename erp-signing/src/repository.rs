use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

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
        sqlx::query_as::<_, SigningDocument>(
            r#"INSERT INTO signing_documents (
                id, name, description, document_type, file_path, file_name, file_size, file_hash,
                pages, status, envelope_id, sender_id, message, expires_at, completed_at, sent_at,
                viewed_at, reminder_count, last_reminder_at, auto_remind, remind_days, sequential_signing,
                current_signer_order, final_signed_file, final_signed_at, audit_trail_file,
                certificate_of_completion, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(doc.base.id)
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
        .bind(doc.sender_id)
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
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<SigningDocument>> {
        sqlx::query_as::<_, SigningDocument>("SELECT * FROM signing_documents WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, sender_id: Option<Uuid>, status: Option<DocumentStatus>) -> anyhow::Result<Vec<SigningDocument>> {
        let mut query = "SELECT * FROM signing_documents WHERE 1=1".to_string();
        if sender_id.is_some() { query.push_str(" AND sender_id = ?"); }
        if status.is_some() { query.push_str(" AND status = ?"); }
        query.push_str(" ORDER BY created_at DESC");
        
        let mut q = sqlx::query_as::<_, SigningDocument>(&query);
        if let Some(sid) = sender_id { q = q.bind(sid); }
        if let Some(s) = status { q = q.bind(s); }
        q.fetch_all(pool).await.map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, doc: &SigningDocument) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signing_documents SET name=?, description=?, status=?, updated_at=? WHERE id=?")
            .bind(&doc.name)
            .bind(&doc.description)
            .bind(&doc.status)
            .bind(now)
            .bind(doc.base.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: DocumentStatus) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signing_documents SET status=?, updated_at=? WHERE id=?")
            .bind(&status)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM signing_documents WHERE id = ?")
            .bind(id)
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
        sqlx::query_as::<_, Signer>(
            r#"INSERT INTO signers (
                id, document_id, name, email, phone, user_id, order_index, role, status,
                authentication_method, access_code, viewed_at, signed_at, declined_at,
                declined_reason, delegated_to, email_sent_at, reminder_sent_at, signature_ip,
                signature_user_agent, signature_location, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(signer.base.id)
        .bind(signer.document_id)
        .bind(&signer.name)
        .bind(&signer.email)
        .bind(&signer.phone)
        .bind(signer.user_id)
        .bind(signer.order_index)
        .bind(&signer.role)
        .bind(&signer.status)
        .bind(&signer.authentication_method)
        .bind(&signer.access_code)
        .bind(signer.viewed_at)
        .bind(signer.signed_at)
        .bind(signer.declined_at)
        .bind(&signer.declined_reason)
        .bind(signer.delegated_to)
        .bind(signer.email_sent_at)
        .bind(signer.reminder_sent_at)
        .bind(&signer.signature_ip)
        .bind(&signer.signature_user_agent)
        .bind(&signer.signature_location)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Signer>> {
        sqlx::query_as::<_, Signer>("SELECT * FROM signers WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_document(&self, pool: &SqlitePool, document_id: Uuid) -> anyhow::Result<Vec<Signer>> {
        sqlx::query_as::<_, Signer>("SELECT * FROM signers WHERE document_id = ? ORDER BY order_index")
            .bind(document_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, signer: &Signer) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signers SET status=?, updated_at=? WHERE id=?")
            .bind(&signer.status)
            .bind(now)
            .bind(signer.base.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: SignerStatus) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signers SET status=?, updated_at=? WHERE id=?")
            .bind(&status)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM signers WHERE id = ?")
            .bind(id)
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
        sqlx::query_as::<_, SignatureField>(
            r#"INSERT INTO signature_fields (
                id, document_id, signer_id, field_type, page, x_position, y_position,
                width, height, required, value, signature_data, signature_type, signed_at, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(field.base.id)
        .bind(field.document_id)
        .bind(field.signer_id)
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
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<SignatureField>> {
        sqlx::query_as::<_, SignatureField>("SELECT * FROM signature_fields WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_document(&self, pool: &SqlitePool, document_id: Uuid) -> anyhow::Result<Vec<SignatureField>> {
        sqlx::query_as::<_, SignatureField>("SELECT * FROM signature_fields WHERE document_id = ? ORDER BY page, y_position, x_position")
            .bind(document_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_signer(&self, pool: &SqlitePool, signer_id: Uuid) -> anyhow::Result<Vec<SignatureField>> {
        sqlx::query_as::<_, SignatureField>("SELECT * FROM signature_fields WHERE signer_id = ? ORDER BY page, y_position, x_position")
            .bind(signer_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn sign(&self, pool: &SqlitePool, id: Uuid, value: String, signature_data: String, signature_type: SignatureType) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE signature_fields SET value=?, signature_data=?, signature_type=?, signed_at=?, updated_at=? WHERE id=?")
            .bind(&value)
            .bind(&signature_data)
            .bind(&signature_type)
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM signature_fields WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
