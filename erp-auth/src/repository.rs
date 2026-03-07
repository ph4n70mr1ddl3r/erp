use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<User>;
    async fn find_by_username(&self, username: &str) -> Result<User>;
    async fn find_by_email(&self, email: &str) -> Result<User>;
    async fn create(&self, user: User) -> Result<User>;
    async fn update_last_login(&self, id: Uuid) -> Result<()>;
    async fn list(&self) -> Result<Vec<User>>;
}

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<User> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at, created_by, updated_by FROM users WHERE id = ?"
        ).bind(id.to_string()).fetch_optional(&self.pool).await?
        .ok_or_else(|| erp_core::Error::not_found("User", &id.to_string()))?;
        
        self.map_user_row(row)
    }

    async fn find_by_username(&self, username: &str) -> Result<User> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at, created_by, updated_by FROM users WHERE username = ?"
        ).bind(username).fetch_optional(&self.pool).await?
        .ok_or_else(|| erp_core::Error::not_found("User", username))?;
        
        self.map_user_row(row)
    }

    async fn find_by_email(&self, email: &str) -> Result<User> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at, created_by, updated_by FROM users WHERE email = ?"
        ).bind(email).fetch_optional(&self.pool).await?
        .ok_or_else(|| erp_core::Error::not_found("User", email))?;
        
        self.map_user_row(row)
    }

    async fn create(&self, user: User) -> Result<User> {
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at, created_by, updated_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(user.base.id.to_string())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.full_name)
        .bind(&user.role)
        .bind(&user.status)
        .bind(user.last_login)
        .bind(user.base.created_at)
        .bind(user.base.updated_at)
        .bind(user.base.created_by.map(|id| id.to_string()))
        .bind(user.base.updated_by.map(|id| id.to_string()))
        .execute(&self.pool).await?;
        Ok(user)
    }

    async fn update_last_login(&self, id: Uuid) -> Result<()> {
        let rows = sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
            .bind(Utc::now()).bind(id.to_string()).execute(&self.pool).await?;
        if rows.rows_affected() == 0 {
            return Err(erp_core::Error::not_found("User", &id.to_string()));
        }
        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>> {
        let rows = sqlx::query(
            "SELECT id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at, created_by, updated_by FROM users ORDER BY username"
        ).fetch_all(&self.pool).await?;
        
        rows.into_iter().map(|r| self.map_user_row(r)).collect()
    }
}

impl SqliteUserRepository {
    fn map_user_row(&self, row: sqlx::sqlite::SqliteRow) -> Result<User> {
        use sqlx::Row;
        use erp_core::BaseEntity;

        let id_str: String = row.try_get("id")?;
        let id = Uuid::parse_str(&id_str).map_err(|_| erp_core::Error::validation("Invalid UUID in database"))?;

        Ok(User {
            base: BaseEntity {
                id,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
                created_by: row.try_get::<Option<String>, _>("created_by")?.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.try_get::<Option<String>, _>("updated_by")?.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            full_name: row.try_get("full_name")?,
            role: row.try_get("role")?,
            status: row.try_get("status")?,
            last_login: row.try_get("last_login")?,
        })
    }
}
