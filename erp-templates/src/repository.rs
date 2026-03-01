use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait TemplateRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, template: &Template) -> anyhow::Result<Template>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Template>>;
    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<Template>>;
    async fn list(&self, pool: &SqlitePool, template_type: Option<TemplateType>) -> anyhow::Result<Vec<Template>>;
    async fn update(&self, pool: &SqlitePool, template: &Template) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteTemplateRepository;

#[async_trait]
impl TemplateRepository for SqliteTemplateRepository {
    async fn create(&self, pool: &SqlitePool, template: &Template) -> anyhow::Result<Template> {
        let now = Utc::now();
        sqlx::query_as::<_, Template>(
            r#"
            INSERT INTO templates (
                id, name, code, description, template_type, format, subject,
                body, html_body, variables, default_values, styles, header_template_id,
                footer_template_id, version, parent_id, status, created_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(template.base.id)
        .bind(&template.name)
        .bind(&template.code)
        .bind(&template.description)
        .bind(&template.template_type)
        .bind(&template.format)
        .bind(&template.subject)
        .bind(&template.body)
        .bind(&template.html_body)
        .bind(&template.variables)
        .bind(&template.default_values)
        .bind(&template.styles)
        .bind(template.header_template_id)
        .bind(template.footer_template_id)
        .bind(template.version)
        .bind(template.parent_id)
        .bind(&template.status)
        .bind(template.created_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Template>> {
        sqlx::query_as::<_, Template>("SELECT * FROM templates WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<Template>> {
        sqlx::query_as::<_, Template>("SELECT * FROM templates WHERE code = ?")
            .bind(code)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, template_type: Option<TemplateType>) -> anyhow::Result<Vec<Template>> {
        match template_type {
            Some(t) => sqlx::query_as::<_, Template>("SELECT * FROM templates WHERE template_type = ? ORDER BY name")
                .bind(&t)
                .fetch_all(pool)
                .await
                .map_err(Into::into),
            None => sqlx::query_as::<_, Template>("SELECT * FROM templates ORDER BY name")
                .fetch_all(pool)
                .await
                .map_err(Into::into),
        }
    }

    async fn update(&self, pool: &SqlitePool, template: &Template) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE templates SET
                name = ?, description = ?, subject = ?, body = ?, html_body = ?,
                variables = ?, default_values = ?, styles = ?, header_template_id = ?,
                footer_template_id = ?, version = ?, status = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&template.name)
        .bind(&template.description)
        .bind(&template.subject)
        .bind(&template.body)
        .bind(&template.html_body)
        .bind(&template.variables)
        .bind(&template.default_values)
        .bind(&template.styles)
        .bind(template.header_template_id)
        .bind(template.footer_template_id)
        .bind(template.version)
        .bind(&template.status)
        .bind(now)
        .bind(template.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM templates WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait EmailTemplateRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<EmailTemplate>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailTemplate>>;
    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<EmailTemplate>>;
    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<EmailTemplate>>;
    async fn update(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<()>;
}

pub struct SqliteEmailTemplateRepository;

#[async_trait]
impl EmailTemplateRepository for SqliteEmailTemplateRepository {
    async fn create(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<EmailTemplate> {
        let now = Utc::now();
        sqlx::query_as::<_, EmailTemplate>(
            r#"
            INSERT INTO email_templates (
                id, name, code, description, subject_template, body_text,
                body_html, from_name, from_email, reply_to, cc_addresses,
                bcc_addresses, attachments, variables, status, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(template.base.id)
        .bind(&template.name)
        .bind(&template.code)
        .bind(&template.description)
        .bind(&template.subject_template)
        .bind(&template.body_text)
        .bind(&template.body_html)
        .bind(&template.from_name)
        .bind(&template.from_email)
        .bind(&template.reply_to)
        .bind(&template.cc_addresses)
        .bind(&template.bcc_addresses)
        .bind(&template.attachments)
        .bind(&template.variables)
        .bind(&template.status)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailTemplate>> {
        sqlx::query_as::<_, EmailTemplate>("SELECT * FROM email_templates WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<EmailTemplate>> {
        sqlx::query_as::<_, EmailTemplate>("SELECT * FROM email_templates WHERE code = ?")
            .bind(code)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<EmailTemplate>> {
        sqlx::query_as::<_, EmailTemplate>("SELECT * FROM email_templates ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE email_templates SET
                name = ?, description = ?, subject_template = ?, body_text = ?,
                body_html = ?, from_name = ?, from_email = ?, reply_to = ?,
                cc_addresses = ?, bcc_addresses = ?, attachments = ?, variables = ?,
                status = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&template.name)
        .bind(&template.description)
        .bind(&template.subject_template)
        .bind(&template.body_text)
        .bind(&template.body_html)
        .bind(&template.from_name)
        .bind(&template.from_email)
        .bind(&template.reply_to)
        .bind(&template.cc_addresses)
        .bind(&template.bcc_addresses)
        .bind(&template.attachments)
        .bind(&template.variables)
        .bind(&template.status)
        .bind(now)
        .bind(template.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
pub trait GeneratedDocumentRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, doc: &GeneratedDocument) -> anyhow::Result<GeneratedDocument>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<GeneratedDocument>>;
    async fn list_by_entity(&self, pool: &SqlitePool, entity_type: &str, entity_id: Uuid) -> anyhow::Result<Vec<GeneratedDocument>>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteGeneratedDocumentRepository;

#[async_trait]
impl GeneratedDocumentRepository for SqliteGeneratedDocumentRepository {
    async fn create(&self, pool: &SqlitePool, doc: &GeneratedDocument) -> anyhow::Result<GeneratedDocument> {
        let now = Utc::now();
        sqlx::query_as::<_, GeneratedDocument>(
            r#"
            INSERT INTO generated_documents (
                id, template_id, template_version, name, output_format, content,
                file_path, file_size, variables_used, related_entity_type,
                related_entity_id, generated_by, generated_at, expires_at, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(doc.base.id)
        .bind(doc.template_id)
        .bind(doc.template_version)
        .bind(&doc.name)
        .bind(&doc.output_format)
        .bind(&doc.content)
        .bind(&doc.file_path)
        .bind(doc.file_size)
        .bind(&doc.variables_used)
        .bind(&doc.related_entity_type)
        .bind(doc.related_entity_id)
        .bind(doc.generated_by)
        .bind(doc.generated_at)
        .bind(doc.expires_at)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<GeneratedDocument>> {
        sqlx::query_as::<_, GeneratedDocument>("SELECT * FROM generated_documents WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_entity(&self, pool: &SqlitePool, entity_type: &str, entity_id: Uuid) -> anyhow::Result<Vec<GeneratedDocument>> {
        sqlx::query_as::<_, GeneratedDocument>(
            "SELECT * FROM generated_documents WHERE related_entity_type = ? AND related_entity_id = ? ORDER BY created_at DESC"
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM generated_documents WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
