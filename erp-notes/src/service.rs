use anyhow::Result;
use chrono::Utc;
use erp_core::Error;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{CreateNoteRequest, Note, NoteType, UpdateNoteRequest};
use crate::repository::{NoteRepository, NoteRepositoryTrait};

pub struct NoteService {
    repo: NoteRepository,
}

impl Default for NoteService {
    fn default() -> Self {
        Self::new()
    }
}

impl NoteService {
    pub fn new() -> Self {
        Self {
            repo: NoteRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        req: CreateNoteRequest,
        user_id: Uuid,
    ) -> Result<Note> {
        if req.content.trim().is_empty() {
            return Err(Error::validation("Note content cannot be empty").into());
        }

        if req.entity_type.trim().is_empty() {
            return Err(Error::validation("Entity type cannot be empty").into());
        }

        let note_type = req
            .note_type
            .as_ref()
            .and_then(|t| t.parse().ok())
            .unwrap_or(NoteType::General);

        let mut note = Note::new(
            &req.entity_type,
            req.entity_id,
            note_type,
            req.content,
            Some(user_id),
        );

        note.title = req.title;
        note.is_private = req.is_private.unwrap_or(false);
        note.reminder_at = req.reminder_at;

        self.repo.create(pool, &note).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Note> {
        self.repo
            .find_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow::anyhow!(Error::not_found("Note", &id.to_string())))
    }

    pub async fn list_for_entity(
        &self,
        pool: &SqlitePool,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Vec<Note>> {
        self.repo.find_by_entity(pool, entity_type, entity_id).await
    }

    pub async fn list_for_user(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<Note>> {
        let offset = (page - 1) * page_size;
        self.repo.find_by_user(pool, user_id, page_size, offset).await
    }

    pub async fn update(
        &self,
        pool: &SqlitePool,
        id: Uuid,
        req: UpdateNoteRequest,
        user_id: Uuid,
    ) -> Result<Note> {
        let mut note = self.get(pool, id).await?;

        if let Some(note_type) = req.note_type {
            note.note_type = note_type.parse().unwrap_or(NoteType::General);
        }
        if let Some(title) = req.title {
            note.title = Some(title);
        }
        if let Some(content) = req.content {
            if content.trim().is_empty() {
                return Err(Error::validation("Note content cannot be empty").into());
            }
            note.content = content;
        }
        if let Some(is_private) = req.is_private {
            note.is_private = is_private;
        }
        if let Some(is_pinned) = req.is_pinned {
            note.is_pinned = is_pinned;
        }
        if let Some(reminder_at) = req.reminder_at {
            note.reminder_at = Some(reminder_at);
        }

        note.base.updated_at = Utc::now();
        note.base.updated_by = Some(user_id);

        self.repo.update(pool, &note).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }

    pub async fn pin(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<Note> {
        let mut note = self.get(pool, id).await?;
        note.is_pinned = true;
        note.base.updated_at = Utc::now();
        note.base.updated_by = Some(user_id);
        self.repo.update(pool, &note).await
    }

    pub async fn unpin(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<Note> {
        let mut note = self.get(pool, id).await?;
        note.is_pinned = false;
        note.base.updated_at = Utc::now();
        note.base.updated_by = Some(user_id);
        self.repo.update(pool, &note).await
    }

    pub async fn mark_reminded(&self, pool: &SqlitePool, id: Uuid) -> Result<Note> {
        let mut note = self.get(pool, id).await?;
        note.reminded_at = Some(Utc::now());
        note.base.updated_at = Utc::now();
        self.repo.update(pool, &note).await
    }

    pub async fn get_pending_reminders(&self, pool: &SqlitePool) -> Result<Vec<Note>> {
        self.repo.find_with_reminders(pool).await
    }

    pub async fn count_for_entity(
        &self,
        pool: &SqlitePool,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<i64> {
        self.repo.count_by_entity(pool, entity_type, entity_id).await
    }
}
