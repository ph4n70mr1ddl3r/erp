use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

static JWT_SECRET: OnceLock<[u8; 32]> = OnceLock::new();

pub fn init_jwt_secret(secret: &str) -> Result<()> {
    if secret.len() < 32 {
        return Err(anyhow::anyhow!(
            "JWT secret must be at least 32 characters, got {}",
            secret.len()
        ));
    }
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, secret.as_bytes());
    let key: [u8; 32] = hasher.finalize().into();
    let _ = JWT_SECRET.set(key);
    Ok(())
}

fn get_secret() -> &'static [u8; 32] {
    JWT_SECRET.get().expect("JWT secret not initialized")
}

pub fn encode_token<T: Serialize>(claims: &T) -> Result<String> {
    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(get_secret());
    Ok(encode(&header, claims, &key)?)
}

pub fn decode_token<T: DeserializeOwned>(token: &str) -> Result<T> {
    let key = DecodingKey::from_secret(get_secret());
    let validation = Validation::new(Algorithm::HS256);
    let data = decode::<T>(token, &key, &validation)?;
    Ok(data.claims)
}

pub fn generate_token(
    user_id: &str,
    username: &str,
    role: &str,
    expiry_hours: i64,
) -> Result<(String, chrono::DateTime<Utc>)> {
    let now = Utc::now();
    let exp = now + Duration::hours(expiry_hours);

    let claims = serde_json::json!({
        "sub": user_id,
        "username": username,
        "role": role,
        "exp": exp.timestamp() as usize,
        "iat": now.timestamp() as usize,
    });

    let token = encode_token(&claims)?;
    Ok((token, exp))
}

pub fn validate_token(token: &str) -> Result<TokenData> {
    let claims: serde_json::Value = decode_token(token)?;

    Ok(TokenData {
        user_id: claims["sub"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing sub"))?
            .to_string(),
        username: claims["username"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing username"))?
            .to_string(),
        role: claims["role"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing role"))?
            .to_string(),
    })
}

#[derive(Debug, Clone)]
pub struct TokenData {
    pub user_id: String,
    pub username: String,
    pub role: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() {
        init_jwt_secret("test-secret-key-must-be-at-least-32-chars")
            .expect("Failed to init JWT secret");
    }

    #[test]
    fn test_generate_and_validate_token() {
        init_test();

        let (token, expires_at) = generate_token("user-123", "testuser", "Admin", 24).unwrap();

        assert!(!token.is_empty());
        assert!(expires_at > Utc::now());

        let data = validate_token(&token).unwrap();
        assert_eq!(data.user_id, "user-123");
        assert_eq!(data.username, "testuser");
        assert_eq!(data.role, "Admin");
    }

    #[test]
    fn test_invalid_token() {
        init_test();

        let result = validate_token("invalid-token");
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_decode_custom_claims() {
        init_test();

        #[derive(serde::Serialize, serde::Deserialize)]
        struct TestClaims {
            sub: String,
            exp: usize,
        }

        let claims = TestClaims {
            sub: "test".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        };

        let token = encode_token(&claims).unwrap();
        let decoded: TestClaims = decode_token(&token).unwrap();

        assert_eq!(decoded.sub, "test");
    }
}
