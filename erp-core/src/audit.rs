use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub action: AuditAction,
    pub old_values: Option<String>,
    pub new_values: Option<String>,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
    Post,
    Approve,
    Confirm,
    Cancel,
}

impl AuditLog {
    pub fn new(
        entity_type: &str,
        entity_id: &str,
        action: AuditAction,
        old_values: Option<String>,
        new_values: Option<String>,
        user_id: Option<Uuid>,
        username: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            action,
            old_values,
            new_values,
            user_id,
            username,
            ip_address: None,
            created_at: Utc::now(),
        }
    }
}

pub async fn log_audit(
    pool: &sqlx::SqlitePool,
    entity_type: &str,
    entity_id: &str,
    action: AuditAction,
    old_values: Option<String>,
    new_values: Option<String>,
    user_id: Option<Uuid>,
    username: Option<String>,
) -> crate::Result<()> {
    let log = AuditLog::new(entity_type, entity_id, action, old_values, new_values, user_id, username);
    
    sqlx::query(
        "INSERT INTO audit_logs (id, entity_type, entity_id, action, old_values, new_values, user_id, username, ip_address, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(log.id.to_string())
    .bind(&log.entity_type)
    .bind(&log.entity_id)
    .bind(format!("{:?}", log.action))
    .bind(&log.old_values)
    .bind(&log.new_values)
    .bind(log.user_id.map(|id| id.to_string()))
    .bind(&log.username)
    .bind(&log.ip_address)
    .bind(log.created_at.to_rfc3339())
    .execute(pool)
    .await
    .map_err(|e| crate::Error::Database(e.into()))?;
    
    Ok(())
}

pub async fn get_audit_logs(
    pool: &sqlx::SqlitePool,
    entity_type: Option<&str>,
    entity_id: Option<&str>,
    page: u32,
    per_page: u32,
) -> crate::Result<crate::Paginated<AuditLog>> {
    let offset = (page.saturating_sub(1)) * per_page;
    
    let (logs, total): (Vec<AuditLogRow>, i64) = if let Some(et) = entity_type {
        if let Some(eid) = entity_id {
            let logs = sqlx::query_as::<_, AuditLogRow>(
                "SELECT * FROM audit_logs WHERE entity_type = ? AND entity_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(et)
            .bind(eid)
            .bind(per_page as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await
            .map_err(|e| crate::Error::Database(e.into()))?;
            
            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM audit_logs WHERE entity_type = ? AND entity_id = ?"
            )
            .bind(et)
            .bind(eid)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::Error::Database(e.into()))?;
            
            (logs, total.0)
        } else {
            let logs = sqlx::query_as::<_, AuditLogRow>(
                "SELECT * FROM audit_logs WHERE entity_type = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(et)
            .bind(per_page as i64)
            .bind(offset as i64)
            .fetch_all(pool)
            .await
            .map_err(|e| crate::Error::Database(e.into()))?;
            
            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM audit_logs WHERE entity_type = ?"
            )
            .bind(et)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::Error::Database(e.into()))?;
            
            (logs, total.0)
        }
    } else {
        let logs = sqlx::query_as::<_, AuditLogRow>(
            "SELECT * FROM audit_logs ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| crate::Error::Database(e.into()))?;
        
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_logs")
            .fetch_one(pool)
            .await
            .map_err(|e| crate::Error::Database(e.into()))?;
        
        (logs, total.0)
    };
    
    let items: Vec<AuditLog> = logs.into_iter().map(|r| r.into()).collect();
    
    Ok(crate::Paginated::new(
        items,
        total as u64,
        crate::Pagination { page, per_page },
    ))
}

#[derive(sqlx::FromRow)]
struct AuditLogRow {
    id: String,
    entity_type: String,
    entity_id: String,
    action: String,
    old_values: Option<String>,
    new_values: Option<String>,
    user_id: Option<String>,
    username: Option<String>,
    ip_address: Option<String>,
    created_at: String,
}

impl From<AuditLogRow> for AuditLog {
    fn from(r: AuditLogRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            entity_type: r.entity_type,
            entity_id: r.entity_id,
            action: match r.action.as_str() {
                "Create" => AuditAction::Create,
                "Update" => AuditAction::Update,
                "Delete" => AuditAction::Delete,
                "Post" => AuditAction::Post,
                "Approve" => AuditAction::Approve,
                "Confirm" => AuditAction::Confirm,
                "Cancel" => AuditAction::Cancel,
                _ => AuditAction::Update,
            },
            old_values: r.old_values,
            new_values: r.new_values,
            user_id: r.user_id.and_then(|id| Uuid::parse_str(&id).ok()),
            username: r.username,
            ip_address: r.ip_address,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}
