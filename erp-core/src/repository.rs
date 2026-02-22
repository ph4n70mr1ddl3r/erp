use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::Result;

#[async_trait]
pub trait Repository<T>: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<T>;
    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<T>>;
    async fn create(&self, pool: &SqlitePool, entity: T) -> Result<T>;
    async fn update(&self, pool: &SqlitePool, entity: T) -> Result<T>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}
