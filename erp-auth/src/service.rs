use sqlx::SqlitePool;
use uuid::Uuid;
use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use erp_core::{BaseEntity, Error, Result};
use crate::models::*;
use crate::repository::*;
use crate::jwt;

const MIN_PASSWORD_LENGTH: usize = 8;

fn validate_password_strength(password: &str) -> Result<()> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(Error::validation(format!(
            "Password must be at least {} characters",
            MIN_PASSWORD_LENGTH
        )));
    }
    
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_numeric());
    
    if !has_letter || !has_digit {
        return Err(Error::validation("Password must contain both letters and numbers"));
    }
    
    Ok(())
}

pub struct AuthService {
    repo: SqliteUserRepository,
}

impl AuthService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { 
            repo: SqliteUserRepository::new(pool),
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse> {
        if req.username.is_empty() || req.email.is_empty() || req.password.is_empty() {
            return Err(Error::validation("Username, email, and password are required"));
        }
        
        validate_password_strength(&req.password)?;
        
        let password_hash = self.hash_password(&req.password)?;
        
        let user = User {
            base: BaseEntity::new(),
            username: req.username,
            email: req.email,
            password_hash,
            full_name: req.full_name,
            role: UserRole::User,
            status: UserStatus::Active,
            last_login: None,
        };
        
        let user = self.repo.create(user).await.map_err(|e| {
            if matches!(e, Error::Database(_)) {
                Error::Conflict("Registration failed. Please try different credentials.".to_string())
            } else {
                e
            }
        })?;
        let (token, expires_at) = jwt::generate_token(&user.base.id.to_string(), &user.username, user.role.as_str(), 24)?;
        
        Ok(AuthResponse {
            token,
            expires_at,
            user: UserInfo {
                id: user.base.id,
                username: user.username,
                email: user.email,
                full_name: user.full_name,
                role: user.role.as_str().to_string(),
            },
        })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse> {
        let user = self.repo.find_by_username(&req.username).await.map_err(|e| {
            match e {
                Error::NotFound(_) => Error::Unauthorized,
                _ => e
            }
        })?;
        
        if user.status != UserStatus::Active {
            return Err(Error::Unauthorized);
        }
        
        if !self.verify_password(&req.password, &user.password_hash)? {
            return Err(Error::Unauthorized);
        }
        
        self.repo.update_last_login(user.base.id).await?;
        
        let (token, expires_at) = jwt::generate_token(&user.base.id.to_string(), &user.username, user.role.as_str(), 24)?;
        
        Ok(AuthResponse {
            token,
            expires_at,
            user: UserInfo {
                id: user.base.id,
                username: user.username,
                email: user.email,
                full_name: user.full_name,
                role: user.role.as_str().to_string(),
            },
        })
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User> {
        self.repo.find_by_id(id).await
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.repo.list().await
    }

    pub fn validate_token(&self, token: &str) -> Result<jwt::TokenData> {
        jwt::validate_token(token).map_err(|_| Error::Unauthorized)
    }

    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2.hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| Error::Internal(anyhow::anyhow!("Password hash error: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid hash: {}", e)))?;
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

pub fn has_permission(role: &str, permission: &str) -> bool {
    let role_perms: Vec<&str> = match role {
        "Admin" => vec!["*"],
        "Finance" => vec!["finance:*", "sales:read", "purchasing:read"],
        "Warehouse" => vec!["inventory:*", "purchasing:*", "manufacturing:read"],
        "Sales" => vec!["sales:*", "inventory:read", "customers:*"],
        "HR" => vec!["hr:*", "employees:*"],
        _ => vec!["*:read"],
    };
    
    role_perms.iter().any(|p| {
        if *p == "*" || *p == permission {
            return true;
        }
        if p.ends_with(":*") {
            let prefix = &p[..p.len()-1];
            return permission.starts_with(prefix);
        }
        if p.starts_with("*:") {
            let suffix = &p[1..];
            return permission.ends_with(suffix);
        }
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hash_password() {
        let pool = SqlitePool::connect_lazy("sqlite::memory:").unwrap();
        let svc = AuthService::new(pool);
        let password = "testpassword123";
        let hash = svc.hash_password(password).unwrap();
        
        assert_ne!(hash, password);
        assert!(hash.starts_with("$argon2"));
    }
    
    #[tokio::test]
    async fn test_verify_password_correct() {
        let pool = SqlitePool::connect_lazy("sqlite::memory:").unwrap();
        let svc = AuthService::new(pool);
        let password = "testpassword123";
        let hash = svc.hash_password(password).unwrap();
        
        assert!(svc.verify_password(password, &hash).unwrap());
    }
    
    #[tokio::test]
    async fn test_verify_password_incorrect() {
        let pool = SqlitePool::connect_lazy("sqlite::memory:").unwrap();
        let svc = AuthService::new(pool);
        let password = "testpassword123";
        let hash = svc.hash_password(password).unwrap();
        
        assert!(!svc.verify_password("wrongpassword", &hash).unwrap());
    }
    
    #[test]
    fn test_has_permission_admin() {
        assert!(has_permission("Admin", "finance:write"));
        assert!(has_permission("Admin", "anything:anything"));
    }
    
    #[test]
    fn test_has_permission_finance() {
        assert!(has_permission("Finance", "finance:write"));
        assert!(has_permission("Finance", "finance:read"));
        assert!(has_permission("Finance", "sales:read"));
        assert!(!has_permission("Finance", "sales:write"));
        assert!(!has_permission("Finance", "hr:read"));
    }
    
    #[test]
    fn test_has_permission_user() {
        assert!(has_permission("User", "finance:read"));
        assert!(has_permission("User", "inventory:read"));
        assert!(!has_permission("User", "finance:write"));
    }
    
    #[test]
    fn test_password_strength_too_short() {
        assert!(validate_password_strength("abc1").is_err());
    }
    
    #[test]
    fn test_password_strength_no_digit() {
        assert!(validate_password_strength("passwordonly").is_err());
    }
    
    #[test]
    fn test_password_strength_no_letter() {
        assert!(validate_password_strength("12345678").is_err());
    }
    
    #[test]
    fn test_password_strength_valid() {
        assert!(validate_password_strength("password1").is_ok());
    }
}
