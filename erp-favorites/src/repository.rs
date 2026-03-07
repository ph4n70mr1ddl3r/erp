use anyhow::Result;
use async_trait::async_trait;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{Favorite, FavoriteType};

#[async_trait]
pub trait FavoriteRepositoryTrait: Send + Sync {
    async fn create(&self, pool: &SqlitePool, favorite: &Favorite) -> Result<Favorite>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<Favorite>>;
    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<Favorite>>;
    async fn find_by_user_and_type(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
    ) -> Result<Vec<Favorite>>;
    async fn find_by_user_and_entity(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
        entity_id: Uuid,
    ) -> Result<Option<Favorite>>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn delete_by_user_and_entity(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
        entity_id: Uuid,
    ) -> Result<()>;
    async fn count_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<i64>;
    async fn exists(&self, pool: &SqlitePool, user_id: Uuid, favorite_type: &FavoriteType, entity_id: Uuid) -> Result<bool>;
}

pub struct FavoriteRepository;

#[async_trait]
impl FavoriteRepositoryTrait for FavoriteRepository {
    async fn create(&self, pool: &SqlitePool, favorite: &Favorite) -> Result<Favorite> {
        sqlx::query(
            r#"INSERT INTO favorites (
                id, user_id, favorite_type, entity_id, entity_name, entity_code, notes,
                created_at, updated_at, created_by, updated_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(favorite.base.id.to_string())
        .bind(favorite.user_id.to_string())
        .bind(favorite.favorite_type.to_string())
        .bind(favorite.entity_id.map(|id| id.to_string()))
        .bind(&favorite.entity_name)
        .bind(&favorite.entity_code)
        .bind(&favorite.notes)
        .bind(favorite.base.created_at.to_rfc3339())
        .bind(favorite.base.updated_at.to_rfc3339())
        .bind(favorite.base.created_by.map(|id| id.to_string()))
        .bind(favorite.base.updated_by.map(|id| id.to_string()))
        .execute(pool)
        .await?;

        Ok(favorite.clone())
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<Favorite>> {
        let row = sqlx::query_as::<_, FavoriteRow>("SELECT * FROM favorites WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool)
            .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<Favorite>> {
        let rows = sqlx::query_as::<_, FavoriteRow>(
            "SELECT * FROM favorites WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_user_and_type(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
    ) -> Result<Vec<Favorite>> {
        let rows = sqlx::query_as::<_, FavoriteRow>(
            "SELECT * FROM favorites WHERE user_id = ? AND favorite_type = ? ORDER BY created_at DESC"
        )
        .bind(user_id.to_string())
        .bind(favorite_type.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_user_and_entity(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
        entity_id: Uuid,
    ) -> Result<Option<Favorite>> {
        let row = sqlx::query_as::<_, FavoriteRow>(
            "SELECT * FROM favorites WHERE user_id = ? AND favorite_type = ? AND entity_id = ?"
        )
        .bind(user_id.to_string())
        .bind(favorite_type.to_string())
        .bind(entity_id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM favorites WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete_by_user_and_entity(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
        entity_id: Uuid,
    ) -> Result<()> {
        sqlx::query("DELETE FROM favorites WHERE user_id = ? AND favorite_type = ? AND entity_id = ?")
            .bind(user_id.to_string())
            .bind(favorite_type.to_string())
            .bind(entity_id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn count_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM favorites WHERE user_id = ?")
            .bind(user_id.to_string())
            .fetch_one(pool)
            .await?;

        Ok(count.0)
    }

    async fn exists(&self, pool: &SqlitePool, user_id: Uuid, favorite_type: &FavoriteType, entity_id: Uuid) -> Result<bool> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM favorites WHERE user_id = ? AND favorite_type = ? AND entity_id = ?"
        )
        .bind(user_id.to_string())
        .bind(favorite_type.to_string())
        .bind(entity_id.to_string())
        .fetch_one(pool)
        .await?;

        Ok(count.0 > 0)
    }
}

#[derive(sqlx::FromRow)]
struct FavoriteRow {
    id: String,
    user_id: String,
    favorite_type: String,
    entity_id: Option<String>,
    entity_name: String,
    entity_code: Option<String>,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
    created_by: Option<String>,
    updated_by: Option<String>,
}

impl From<FavoriteRow> for Favorite {
    fn from(r: FavoriteRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: parse_datetime(&r.created_at),
                updated_at: parse_datetime(&r.updated_at),
                created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
                updated_by: r.updated_by.and_then(|id| Uuid::parse_str(&id).ok()),
            },
            user_id: Uuid::parse_str(&r.user_id).unwrap_or_default(),
            favorite_type: r.favorite_type.parse().unwrap_or(FavoriteType::Page),
            entity_id: r.entity_id.and_then(|id| Uuid::parse_str(&id).ok()),
            entity_name: r.entity_name,
            entity_code: r.entity_code,
            notes: r.notes,
        }
    }
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}
