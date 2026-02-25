use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use erp_core::Result;

use async_trait::async_trait;

#[async_trait]
pub trait ProcessRepository: Send + Sync {
    async fn create_definition(&self, pool: &SqlitePool, def: &ProcessDefinition) -> Result<ProcessDefinition>;
    async fn get_definition(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<ProcessDefinition>>;
    async fn list_definitions(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<ProcessDefinition>>;
}

pub struct SqliteProcessRepository;

#[async_trait]
impl ProcessRepository for SqliteProcessRepository {
    async fn create_definition(&self, pool: &SqlitePool, def: &ProcessDefinition) -> Result<ProcessDefinition> {
        sqlx::query(r#"
            INSERT INTO bpm_process_definitions (id, name, code, description, category, version, 
                status, bpmn_xml, diagram_data, variables, forms, owner_id, published_at, published_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(def.id.to_string())
        .bind(&def.name)
        .bind(&def.code)
        .bind(&def.description)
        .bind(&def.category)
        .bind(def.version)
        .bind(&def.status)
        .bind(&def.bpmn_xml)
        .bind(&def.diagram_data)
        .bind(&def.variables)
        .bind(&def.forms)
        .bind(def.owner_id.to_string())
        .bind(&def.published_at.map(|d| d.to_rfc3339()))
        .bind(&def.published_by.map(|id| id.to_string()))
        .bind(def.created_at.to_rfc3339())
        .bind(def.updated_at.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(def.clone())
    }

    async fn get_definition(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<ProcessDefinition>> {
        let row = sqlx::query_as::<_, (String, String, String, Option<String>, String, i32, String, Option<String>, Option<String>, Option<String>, Option<String>, String, Option<String>, Option<String>, String, String)>(
            "SELECT id, name, code, description, category, version, status, bpmn_xml, diagram_data, variables, forms, owner_id, published_at, published_by, created_at, updated_at FROM bpm_process_definitions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| ProcessDefinition {
            id: r.0.parse().unwrap_or_default(),
            name: r.1,
            code: r.2,
            description: r.3,
            category: r.4,
            version: r.5,
            status: r.6,
            bpmn_xml: r.7,
            diagram_data: r.8,
            variables: r.9,
            forms: r.10,
            owner_id: r.11.parse().unwrap_or_default(),
            published_at: r.12.and_then(|s| s.parse().ok()),
            published_by: r.13.and_then(|s| s.parse().ok()),
            created_at: r.14.parse().unwrap_or_default(),
            updated_at: r.15.parse().unwrap_or_default(),
        }))
    }

    async fn list_definitions(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<ProcessDefinition>> {
        let rows = if let Some(cat) = category {
            sqlx::query_as::<_, (String, String, String, Option<String>, String, i32, String, Option<String>, Option<String>, Option<String>, Option<String>, String, Option<String>, Option<String>, String, String)>(
                "SELECT id, name, code, description, category, version, status, bpmn_xml, diagram_data, variables, forms, owner_id, published_at, published_by, created_at, updated_at FROM bpm_process_definitions WHERE category = ? AND status = 'Published'"
            )
            .bind(cat)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, (String, String, String, Option<String>, String, i32, String, Option<String>, Option<String>, Option<String>, Option<String>, String, Option<String>, Option<String>, String, String)>(
                "SELECT id, name, code, description, category, version, status, bpmn_xml, diagram_data, variables, forms, owner_id, published_at, published_by, created_at, updated_at FROM bpm_process_definitions WHERE status = 'Published'"
            )
            .fetch_all(pool)
            .await?
        };

        Ok(rows.into_iter().map(|r| ProcessDefinition {
            id: r.0.parse().unwrap_or_default(),
            name: r.1,
            code: r.2,
            description: r.3,
            category: r.4,
            version: r.5,
            status: r.6,
            bpmn_xml: r.7,
            diagram_data: r.8,
            variables: r.9,
            forms: r.10,
            owner_id: r.11.parse().unwrap_or_default(),
            published_at: r.12.and_then(|s| s.parse().ok()),
            published_by: r.13.and_then(|s| s.parse().ok()),
            created_at: r.14.parse().unwrap_or_default(),
            updated_at: r.15.parse().unwrap_or_default(),
        }).collect())
    }
}
