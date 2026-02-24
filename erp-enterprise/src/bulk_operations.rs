use chrono::{DateTime, Utc};
use erp_core::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperation {
    pub id: Uuid,
    pub operation_type: String,
    pub entity_type: String,
    pub total_count: i32,
    pub processed_count: i32,
    pub success_count: i32,
    pub error_count: i32,
    pub status: String,
    pub errors: Option<Vec<BulkOperationError>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationError {
    pub entity_id: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkCreateRequest {
    pub entity_type: String,
    pub records: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdateRequest {
    pub entity_type: String,
    pub ids: Vec<String>,
    pub updates: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDeleteRequest {
    pub entity_type: String,
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResponse {
    pub operation_id: Uuid,
    pub status: String,
    pub total_count: i32,
    pub processed_count: i32,
    pub success_count: i32,
    pub error_count: i32,
}

pub struct BulkOperationsService;

impl BulkOperationsService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_operation(
        &self,
        pool: &SqlitePool,
        operation_type: &str,
        entity_type: &str,
        total_count: i32,
        created_by: Uuid,
    ) -> Result<BulkOperation> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"INSERT INTO bulk_operations 
               (id, operation_type, entity_type, total_count, processed_count, success_count, error_count, status, created_by, created_at)
               VALUES (?, ?, ?, ?, 0, 0, 0, 'pending', ?, ?)"#
        )
        .bind(id.to_string())
        .bind(operation_type)
        .bind(entity_type)
        .bind(total_count)
        .bind(created_by.to_string())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(BulkOperation {
            id,
            operation_type: operation_type.to_string(),
            entity_type: entity_type.to_string(),
            total_count,
            processed_count: 0,
            success_count: 0,
            error_count: 0,
            status: "pending".to_string(),
            errors: None,
            created_by,
            created_at: now,
            completed_at: None,
        })
    }

    pub async fn get_operation(&self, pool: &SqlitePool, id: Uuid) -> Result<BulkOperation> {
        let row = sqlx::query_as::<_, BulkOperationRow>(
            r#"SELECT id, operation_type, entity_type, total_count, processed_count, success_count, error_count, status, errors, created_by, created_at, completed_at 
               FROM bulk_operations WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| erp_core::Error::not_found("BulkOperation", &id.to_string()))?;

        Ok(row.into_model())
    }

    pub async fn list_operations(
        &self,
        pool: &SqlitePool,
        status: Option<&str>,
        limit: i32,
    ) -> Result<Vec<BulkOperation>> {
        let rows = if let Some(status) = status {
            sqlx::query_as::<_, BulkOperationRow>(
                r#"SELECT id, operation_type, entity_type, total_count, processed_count, success_count, error_count, status, errors, created_by, created_at, completed_at 
                   FROM bulk_operations WHERE status = ? ORDER BY created_at DESC LIMIT ?"#
            )
            .bind(status)
            .bind(limit)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, BulkOperationRow>(
                r#"SELECT id, operation_type, entity_type, total_count, processed_count, success_count, error_count, status, errors, created_by, created_at, completed_at 
                   FROM bulk_operations ORDER BY created_at DESC LIMIT ?"#
            )
            .bind(limit)
            .fetch_all(pool)
            .await?
        };

        Ok(rows.into_iter().map(|r| r.into_model()).collect())
    }

    pub async fn update_progress(
        &self,
        pool: &SqlitePool,
        id: Uuid,
        success: bool,
        error: Option<&str>,
        entity_id: Option<&str>,
    ) -> Result<()> {
        let mut op = self.get_operation(pool, id).await?;
        op.processed_count += 1;
        
        if success {
            op.success_count += 1;
        } else {
            op.error_count += 1;
            if let (Some(err), Some(eid)) = (error, entity_id) {
                let mut errors = op.errors.unwrap_or_default();
                errors.push(BulkOperationError {
                    entity_id: eid.to_string(),
                    error: err.to_string(),
                });
                op.errors = Some(errors);
            }
        }

        let errors_json = op.errors.as_ref().map(|e| serde_json::to_string(e).unwrap_or_default());

        sqlx::query(
            r#"UPDATE bulk_operations 
               SET processed_count = ?, success_count = ?, error_count = ?, errors = ?, status = 'processing'
               WHERE id = ?"#
        )
        .bind(op.processed_count)
        .bind(op.success_count)
        .bind(op.error_count)
        .bind(&errors_json)
        .bind(id.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn complete_operation(&self, pool: &SqlitePool, id: Uuid) -> Result<BulkOperation> {
        let now = Utc::now();
        let status = if let Ok(op) = self.get_operation(pool, id).await {
            if op.error_count > 0 && op.success_count == 0 {
                "failed"
            } else if op.error_count > 0 {
                "partial"
            } else {
                "completed"
            }
        } else {
            "completed"
        };

        sqlx::query("UPDATE bulk_operations SET status = ?, completed_at = ? WHERE id = ?")
            .bind(status)
            .bind(now.to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;

        self.get_operation(pool, id).await
    }

    pub async fn cancel_operation(&self, pool: &SqlitePool, id: Uuid) -> Result<BulkOperation> {
        let now = Utc::now();
        sqlx::query("UPDATE bulk_operations SET status = 'cancelled', completed_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;

        self.get_operation(pool, id).await
    }

    pub async fn cleanup_old_operations(&self, pool: &SqlitePool, days_old: i32) -> Result<u64> {
        let cutoff = Utc::now() - chrono::Duration::days(days_old as i64);
        let result = sqlx::query("DELETE FROM bulk_operations WHERE completed_at < ? AND status IN ('completed', 'failed', 'partial', 'cancelled')")
            .bind(cutoff.to_rfc3339())
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct BulkOperationRow {
    id: String,
    operation_type: String,
    entity_type: String,
    total_count: i32,
    processed_count: i32,
    success_count: i32,
    error_count: i32,
    status: String,
    errors: Option<String>,
    created_by: String,
    created_at: String,
    completed_at: Option<String>,
}

impl BulkOperationRow {
    fn into_model(self) -> BulkOperation {
        BulkOperation {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            operation_type: self.operation_type,
            entity_type: self.entity_type,
            total_count: self.total_count,
            processed_count: self.processed_count,
            success_count: self.success_count,
            error_count: self.error_count,
            status: self.status,
            errors: self.errors.and_then(|e| serde_json::from_str(&e).ok()),
            created_by: Uuid::parse_str(&self.created_by).unwrap_or_default(),
            created_at: DateTime::parse_from_rfc3339(&self.created_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            completed_at: self.completed_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
        }
    }
}
