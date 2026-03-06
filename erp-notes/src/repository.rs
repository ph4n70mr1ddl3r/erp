use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{Note, NoteType};

#[async_trait]
pub trait NoteRepositoryTrait: Send + Sync {
    async fn create(&self, pool: &SqlitePool, note: &Note) -> Result<Note>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<Note>>;
    async fn find_by_entity(&self, pool: &SqlitePool, entity_type: &str, entity_id: Uuid) -> Result<Vec<Note>>;
    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Note>>;
    async fn find_with_reminders(&self, pool: &SqlitePool) -> Result<Vec<Note>>;
    async fn update(&self, pool: &SqlitePool, note: &Note) -> Result<Note>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn count_by_entity(&self, pool: &SqlitePool, entity_type: &str, entity_id: Uuid) -> Result<i64>;
}

pub struct NoteRepository;

#[async_trait]
impl NoteRepositoryTrait for NoteRepository {
    async fn create(&self, pool: &SqlitePool, note: &Note) -> Result<Note> {
        sqlx::query(
            r#"INSERT INTO notes (
                id, entity_type, entity_id, note_type, title, content, is_private, is_pinned,
                reminder_at, reminded_at, created_at, updated_at, created_by, updated_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(note.base.id.to_string())
        .bind(&note.entity_type)
        .bind(note.entity_id.to_string())
        .bind(note.note_type.to_string())
        .bind(&note.title)
        .bind(&note.content)
        .bind(note.is_private as i32)
        .bind(note.is_pinned as i32)
        .bind(note.reminder_at.map(|d| d.to_rfc3339()))
        .bind(note.reminded_at.map(|d| d.to_rfc3339()))
        .bind(note.base.created_at.to_rfc3339())
        .bind(note.base.updated_at.to_rfc3339())
        .bind(note.base.created_by.map(|id| id.to_string()))
        .bind(note.base.updated_by.map(|id| id.to_string()))
        .execute(pool)
        .await?;

        Ok(note.clone())
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<Note>> {
        let row = sqlx::query_as::<_, NoteRow>(
            "SELECT * FROM notes WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_entity(&self, pool: &SqlitePool, entity_type: &str, entity_id: Uuid) -> Result<Vec<Note>> {
        let rows = sqlx::query_as::<_, NoteRow>(
            r#"SELECT * FROM notes 
               WHERE entity_type = ? AND entity_id = ? 
               ORDER BY is_pinned DESC, created_at DESC"#
        )
        .bind(entity_type)
        .bind(entity_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Note>> {
        let rows = sqlx::query_as::<_, NoteRow>(
            r#"SELECT * FROM notes 
               WHERE created_by = ? OR is_private = 0
               ORDER BY is_pinned DESC, created_at DESC 
               LIMIT ? OFFSET ?"#
        )
        .bind(user_id.to_string())
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_with_reminders(&self, pool: &SqlitePool) -> Result<Vec<Note>> {
        let now = Utc::now();
        let rows = sqlx::query_as::<_, NoteRow>(
            r#"SELECT * FROM notes 
               WHERE reminder_at IS NOT NULL 
               AND reminder_at <= ? 
               AND reminded_at IS NULL
               ORDER BY reminder_at ASC"#
        )
        .bind(now.to_rfc3339())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update(&self, pool: &SqlitePool, note: &Note) -> Result<Note> {
        sqlx::query(
            r#"UPDATE notes SET 
                note_type = ?, title = ?, content = ?, is_private = ?, is_pinned = ?,
                reminder_at = ?, reminded_at = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#
        )
        .bind(note.note_type.to_string())
        .bind(&note.title)
        .bind(&note.content)
        .bind(note.is_private as i32)
        .bind(note.is_pinned as i32)
        .bind(note.reminder_at.map(|d| d.to_rfc3339()))
        .bind(note.reminded_at.map(|d| d.to_rfc3339()))
        .bind(note.base.updated_at.to_rfc3339())
        .bind(note.base.updated_by.map(|id| id.to_string()))
        .bind(note.base.id.to_string())
        .execute(pool)
        .await?;

        Ok(note.clone())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn count_by_entity(&self, pool: &SqlitePool, entity_type: &str, entity_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM notes WHERE entity_type = ? AND entity_id = ?"
        )
        .bind(entity_type)
        .bind(entity_id.to_string())
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }
}

#[derive(sqlx::FromRow)]
struct NoteRow {
    id: String,
    entity_type: String,
    entity_id: String,
    note_type: String,
    title: Option<String>,
    content: String,
    is_private: i32,
    is_pinned: i32,
    reminder_at: Option<String>,
    reminded_at: Option<String>,
    created_at: String,
    updated_at: String,
    created_by: Option<String>,
    updated_by: Option<String>,
}

impl From<NoteRow> for Note {
    fn from(r: NoteRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: parse_datetime(&r.created_at),
                updated_at: parse_datetime(&r.updated_at),
                created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
                updated_by: r.updated_by.and_then(|id| Uuid::parse_str(&id).ok()),
            },
            entity_type: r.entity_type,
            entity_id: Uuid::parse_str(&r.entity_id).unwrap_or_default(),
            note_type: r.note_type.parse().unwrap_or(NoteType::General),
            title: r.title,
            content: r.content,
            is_private: r.is_private != 0,
            is_pinned: r.is_pinned != 0,
            reminder_at: r.reminder_at.and_then(|d| parse_datetime_opt(&d)),
            reminded_at: r.reminded_at.and_then(|d| parse_datetime_opt(&d)),
        }
    }
}

fn parse_datetime(s: &str) -> DateTime<Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn parse_datetime_opt(s: &str) -> Option<DateTime<Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .ok()
}
