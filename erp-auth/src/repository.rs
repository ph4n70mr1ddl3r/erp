use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<User>;
    async fn find_by_username(&self, pool: &SqlitePool, username: &str) -> Result<User>;
    async fn find_by_email(&self, pool: &SqlitePool, email: &str) -> Result<User>;
    async fn create(&self, pool: &SqlitePool, user: User) -> Result<User>;
    async fn update_last_login(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn list(&self, pool: &SqlitePool) -> Result<Vec<User>>;
}

pub struct SqliteUserRepository;

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<User> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at FROM users WHERE id = ?"
        ).bind(id.to_string()).fetch_optional(pool).await?
        .ok_or_else(|| erp_core::Error::not_found("User", &id.to_string()))?;
        Ok(row.into_user())
    }

    async fn find_by_username(&self, pool: &SqlitePool, username: &str) -> Result<User> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at FROM users WHERE username = ?"
        ).bind(username).fetch_optional(pool).await?
        .ok_or_else(|| erp_core::Error::not_found("User", username))?;
        Ok(row.into_user())
    }

    async fn find_by_email(&self, pool: &SqlitePool, email: &str) -> Result<User> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at FROM users WHERE email = ?"
        ).bind(email).fetch_optional(pool).await?
        .ok_or_else(|| erp_core::Error::not_found("User", email))?;
        Ok(row.into_user())
    }

    async fn create(&self, pool: &SqlitePool, user: User) -> Result<User> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        ).bind(user.id.to_string()).bind(&user.username).bind(&user.email).bind(&user.password_hash)
        .bind(&user.full_name).bind(user.role.as_str()).bind(user.status.as_str())
        .bind(user.last_login.map(|d| d.to_rfc3339())).bind(user.created_at.to_rfc3339()).bind(now.to_rfc3339())
        .execute(pool).await?;
        Ok(user)
    }

    async fn update_last_login(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339()).bind(id.to_string()).execute(pool).await?;
        if rows.rows_affected() == 0 {
            return Err(erp_core::Error::not_found("User", &id.to_string()));
        }
        Ok(())
    }

    async fn list(&self, pool: &SqlitePool) -> Result<Vec<User>> {
        let rows = sqlx::query_as::<_, UserRow>(
            "SELECT id, username, email, password_hash, full_name, role, status, last_login, created_at, updated_at FROM users ORDER BY username"
        ).fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_user()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: String,
    username: String,
    email: String,
    password_hash: String,
    full_name: String,
    role: String,
    status: String,
    last_login: Option<String>,
    created_at: String,
    updated_at: String,
}

impl UserRow {
    fn into_user(self) -> User {
        User {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            username: self.username,
            email: self.email,
            password_hash: self.password_hash,
            full_name: self.full_name,
            role: UserRole::from_str(&self.role),
            status: UserStatus::from_str(&self.status),
            last_login: self.last_login.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}
