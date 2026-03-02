use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, Response, StatusCode, header::HeaderName},
    middleware::Next,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

const MAX_REQUESTS: u32 = 10;
const AUTH_MAX_REQUESTS: u32 = 5;
const WINDOW_SECS: u64 = 60;

static X_FORWARDED_FOR: HeaderName = HeaderName::from_static("x-forwarded-for");
static X_REAL_IP: HeaderName = HeaderName::from_static("x-real-ip");

#[derive(Clone)]
pub struct RateLimitEntry {
    pub count: u32,
    pub window_start: Instant,
}

#[derive(Clone, Default)]
pub struct RateLimiter {
    pub entries: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn check(&self, key: &str, max_requests: u32) -> Result<(), StatusCode> {
        let mut entries = self.entries.lock().await;
        let now = Instant::now();

        if let Some(entry) = entries.get_mut(key) {
            if now.duration_since(entry.window_start) > Duration::from_secs(WINDOW_SECS) {
                entry.count = 1;
                entry.window_start = now;
            } else if entry.count >= max_requests {
                return Err(StatusCode::TOO_MANY_REQUESTS);
            } else {
                entry.count += 1;
            }
        } else {
            entries.insert(
                key.to_string(),
                RateLimitEntry {
                    count: 1,
                    window_start: now,
                },
            );
        }

        Ok(())
    }

    pub async fn cleanup(&self) {
        let mut entries = self.entries.lock().await;
        let now = Instant::now();
        entries.retain(|_, entry| {
            now.duration_since(entry.window_start) <= Duration::from_secs(WINDOW_SECS * 2)
        });
    }

    pub fn spawn_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let limiter = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(WINDOW_SECS));
            loop {
                interval.tick().await;
                limiter.cleanup().await;
            }
        })
    }
}

fn extract_client_ip(req: &Request<Body>) -> String {
    if let Some(forwarded_for) = req.headers().get(&X_FORWARDED_FOR) {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                let trimmed = first_ip.trim();
                if !trimmed.is_empty() {
                    return format!("proxy:{}", trimmed);
                }
            }
        }
    }
    
    if let Some(real_ip) = req.headers().get(&X_REAL_IP) {
        if let Ok(ip_str) = real_ip.to_str() {
            return format!("real:{}", ip_str);
        }
    }
    
    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn is_auth_endpoint(path: &str) -> bool {
    path == "/auth/login" || path == "/auth/register"
}

pub async fn rate_limit_middleware(
    rate_limiter: axum::extract::Extension<RateLimiter>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let key = extract_client_ip(&req);
    let path = req.uri().path();
    let max_requests = if is_auth_endpoint(path) {
        AUTH_MAX_REQUESTS
    } else {
        MAX_REQUESTS
    };

    rate_limiter.check(&key, max_requests).await?;
    Ok(next.run(req).await)
}
