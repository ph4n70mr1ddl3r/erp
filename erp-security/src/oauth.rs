use crate::models::*;
use chrono::{DateTime, Utc};
use erp_core::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct TokenRequest {
    grant_type: String,
    code: String,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
    expires_in: Option<i64>,
    refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GenericUserInfo {
    sub: Option<String>,
    id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    login: Option<String>,
    username: Option<String>,
    picture: Option<String>,
    avatar_url: Option<String>,
}

#[allow(clippy::type_complexity)]
pub struct OAuthService {
    client: Client,
    state_store: std::sync::Arc<tokio::sync::RwLock<HashMap<String, (String, DateTime<Utc>)>>>,
}

impl Default for OAuthService {
    fn default() -> Self { Self::new() }
}

impl OAuthService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            state_store: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_authorize_url(
        &self,
        provider: &OAuthProvider,
        redirect_uri: &str,
    ) -> OAuthAuthorizeUrl {
        let state = Uuid::new_v4().to_string();
        
        let mut url = format!("{}?", provider.authorize_url);
        url.push_str(&format!("client_id={}", provider.client_id));
        url.push_str(&format!("&redirect_uri={}", urlencoding::encode(redirect_uri)));
        url.push_str(&format!("&scope={}", urlencoding::encode(&provider.scope)));
        url.push_str("&response_type=code");
        url.push_str(&format!("&state={}", state));

        self.state_store.write().await.insert(
            state.clone(),
            (provider.id.to_string(), Utc::now() + chrono::Duration::minutes(10)),
        );

        OAuthAuthorizeUrl { url, state }
    }

    pub async fn exchange_code(
        &self,
        provider: &OAuthProvider,
        code: &str,
        redirect_uri: &str,
    ) -> Result<(String, Option<String>, Option<DateTime<Utc>>)> {
        let request = TokenRequest {
            grant_type: "authorization_code".to_string(),
            code: code.to_string(),
            redirect_uri: redirect_uri.to_string(),
            client_id: provider.client_id.clone(),
            client_secret: provider.client_secret.clone(),
        };

        let response = self
            .client
            .post(&provider.token_url)
            .form(&request)
            .send()
            .await
            .map_err(|e| erp_core::Error::Internal(anyhow::anyhow!("Token request failed: {}", e)))?;

        let token: TokenResponse = response
            .json()
            .await
            .map_err(|e| erp_core::Error::Internal(anyhow::anyhow!("Token parse failed: {}", e)))?;

        let expires_at = token.expires_in.map(|secs| Utc::now() + chrono::Duration::seconds(secs));

        Ok((token.access_token, token.refresh_token, expires_at))
    }

    pub async fn get_user_info(
        &self,
        provider: &OAuthProvider,
        access_token: &str,
    ) -> Result<OAuthUserInfo> {
        let response = self
            .client
            .get(&provider.userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| erp_core::Error::Internal(anyhow::anyhow!("User info request failed: {}", e)))?;

        let info: GenericUserInfo = response
            .json()
            .await
            .map_err(|e| erp_core::Error::Internal(anyhow::anyhow!("User info parse failed: {}", e)))?;

        let provider_user_id = info.sub
            .or(info.id)
            .or(info.login)
            .or(info.username.clone())
            .ok_or_else(|| erp_core::Error::validation("No user ID in OAuth response"))?;

        let email = info.email
            .ok_or_else(|| erp_core::Error::validation("No email in OAuth response"))?;

        Ok(OAuthUserInfo {
            provider_user_id,
            email,
            name: info.name.or(info.username),
            picture: info.picture.or(info.avatar_url),
        })
    }

    pub async fn validate_state(&self, state: &str) -> Result<Uuid> {
        let store = self.state_store.read().await;
        let (provider_id, expires) = store
            .get(state)
            .ok_or_else(|| erp_core::Error::validation("Invalid OAuth state"))?;

        if Utc::now() > *expires {
            return Err(erp_core::Error::validation("OAuth state expired"));
        }

        Uuid::parse_str(provider_id)
            .map_err(|_| erp_core::Error::validation("Invalid provider ID in state"))
    }

    pub async fn cleanup_expired_states(&self) {
        let mut store = self.state_store.write().await;
        let now = Utc::now();
        store.retain(|_, (_, expires)| now < *expires);
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        fn is_safe(c: char) -> bool {
            c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~'
        }
        s.chars()
            .map(|c| {
                if is_safe(c) {
                    c.to_string()
                } else {
                    format!("%{:02X}", c as u32)
                }
            })
            .collect()
    }
}
