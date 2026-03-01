use chrono::Utc;
use erp_core::BaseEntity;
use handlebars::Handlebars;
use regex::Regex;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct TemplateService {
    template_repo: SqliteTemplateRepository,
    document_repo: SqliteGeneratedDocumentRepository,
}

impl Default for TemplateService {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateService {
    pub fn new() -> Self {
        Self {
            template_repo: SqliteTemplateRepository,
            document_repo: SqliteGeneratedDocumentRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        code: String,
        description: Option<String>,
        template_type: TemplateType,
        format: TemplateFormat,
        subject: Option<String>,
        body: String,
        html_body: Option<String>,
        variables: Option<serde_json::Value>,
        created_by: Uuid,
    ) -> anyhow::Result<Template> {
        let template = Template {
            base: BaseEntity::new(),
            name,
            code,
            description,
            template_type,
            format,
            subject,
            body,
            html_body,
            variables,
            default_values: None,
            styles: None,
            header_template_id: None,
            footer_template_id: None,
            version: 1,
            parent_id: None,
            status: erp_core::Status::Active,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.template_repo.create(pool, &template).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Template>> {
        self.template_repo.get_by_id(pool, id).await
    }

    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<Template>> {
        self.template_repo.get_by_code(pool, code).await
    }

    pub async fn list(&self, pool: &SqlitePool, template_type: Option<TemplateType>) -> anyhow::Result<Vec<Template>> {
        self.template_repo.list(pool, template_type).await
    }

    pub async fn update(&self, pool: &SqlitePool, template: &Template) -> anyhow::Result<()> {
        self.template_repo.update(pool, template).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.template_repo.delete(pool, id).await
    }

    pub fn render(&self, template: &Template, variables: &serde_json::Value) -> anyhow::Result<RenderedTemplate> {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(handlebars::no_escape);
        
        let mut merged_vars = variables.clone();
        if let Some(defaults) = &template.default_values {
            if let (serde_json::Value::Object(vars), serde_json::Value::Object(defs)) = (&mut merged_vars, defaults) {
                for (key, value) in defs {
                    if !vars.contains_key(key) {
                        vars.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        
        let subject = if let Some(subj_template) = &template.subject {
            Some(render_string(subj_template, &merged_vars)?)
        } else {
            None
        };
        
        let body = render_string(&template.body, &merged_vars)?;
        
        let html_body = if let Some(html_template) = &template.html_body {
            Some(render_string(html_template, &merged_vars)?)
        } else {
            None
        };
        
        Ok(RenderedTemplate {
            subject,
            body,
            html_body,
        })
    }

    pub async fn generate_document(
        &self,
        pool: &SqlitePool,
        template_id: Uuid,
        name: String,
        variables: serde_json::Value,
        related_entity_type: Option<String>,
        related_entity_id: Option<Uuid>,
        generated_by: Uuid,
    ) -> anyhow::Result<GeneratedDocument> {
        let template = self.template_repo.get_by_id(pool, template_id).await?
            .ok_or_else(|| anyhow::anyhow!("Template not found"))?;
        
        let rendered = self.render(&template, &variables)?;
        
        let doc = GeneratedDocument {
            base: BaseEntity::new(),
            template_id,
            template_version: template.version,
            name,
            output_format: template.format,
            content: Some(rendered.body),
            file_path: None,
            file_size: None,
            variables_used: Some(variables),
            related_entity_type,
            related_entity_id,
            generated_by,
            generated_at: Utc::now(),
            expires_at: None,
            created_at: Utc::now(),
        };
        
        self.document_repo.create(pool, &doc).await
    }

    pub async fn get_document(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<GeneratedDocument>> {
        self.document_repo.get_by_id(pool, id).await
    }

    pub async fn list_documents_for_entity(&self, pool: &SqlitePool, entity_type: &str, entity_id: Uuid) -> anyhow::Result<Vec<GeneratedDocument>> {
        self.document_repo.list_by_entity(pool, entity_type, entity_id).await
    }

    pub fn extract_variables(&self, template_body: &str) -> Vec<String> {
        let re = Regex::new(r"\{\{\s*([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*)\s*\}\}").unwrap();
        re.captures_iter(template_body)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }
}

pub struct RenderedTemplate {
    pub subject: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
}

fn render_string(template: &str, variables: &serde_json::Value) -> anyhow::Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);
    
    handlebars.render_template(template, variables)
        .map_err(|e| anyhow::anyhow!("Template rendering error: {}", e))
}

pub struct EmailTemplateService {
    email_template_repo: SqliteEmailTemplateRepository,
}

impl Default for EmailTemplateService {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailTemplateService {
    pub fn new() -> Self {
        Self {
            email_template_repo: SqliteEmailTemplateRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        code: String,
        description: Option<String>,
        subject_template: String,
        body_text: Option<String>,
        body_html: Option<String>,
        from_name: Option<String>,
        from_email: Option<String>,
        variables: Option<serde_json::Value>,
    ) -> anyhow::Result<EmailTemplate> {
        let template = EmailTemplate {
            base: BaseEntity::new(),
            name,
            code,
            description,
            subject_template,
            body_text,
            body_html,
            from_name,
            from_email,
            reply_to: None,
            cc_addresses: None,
            bcc_addresses: None,
            attachments: None,
            variables,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.email_template_repo.create(pool, &template).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailTemplate>> {
        self.email_template_repo.get_by_id(pool, id).await
    }

    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<EmailTemplate>> {
        self.email_template_repo.get_by_code(pool, code).await
    }

    pub async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<EmailTemplate>> {
        self.email_template_repo.list(pool).await
    }

    pub fn render(&self, template: &EmailTemplate, variables: &serde_json::Value) -> anyhow::Result<RenderedEmail> {
        let subject = render_string(&template.subject_template, variables)?;
        
        let body_text = if let Some(text) = &template.body_text {
            Some(render_string(text, variables)?)
        } else {
            None
        };
        
        let body_html = if let Some(html) = &template.body_html {
            Some(render_string(html, variables)?)
        } else {
            None
        };
        
        Ok(RenderedEmail {
            subject,
            body_text,
            body_html,
            from_name: template.from_name.clone(),
            from_email: template.from_email.clone(),
            reply_to: template.reply_to.clone(),
            cc: template.cc_addresses.clone(),
            bcc: template.bcc_addresses.clone(),
        })
    }

    pub async fn update(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<()> {
        self.email_template_repo.update(pool, template).await
    }
}

pub struct RenderedEmail {
    pub subject: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub from_name: Option<String>,
    pub from_email: Option<String>,
    pub reply_to: Option<String>,
    pub cc: Option<String>,
    pub bcc: Option<String>,
}
