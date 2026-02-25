use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

const MAX_REQUESTS: u32 = 10;
const WINDOW_SECS: u64 = 60;

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

    pub async fn check(&self, key: &str) -> Result<(), StatusCode> {
        let mut entries = self.entries.lock().await;
        let now = Instant::now();

        if let Some(entry) = entries.get_mut(key) {
            if now.duration_since(entry.window_start) > Duration::from_secs(WINDOW_SECS) {
                entry.count = 1;
                entry.window_start = now;
            } else if entry.count >= MAX_REQUESTS {
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
}

pub async fn rate_limit_middleware(
    rate_limiter: axum::extract::Extension<RateLimiter>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let key = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    rate_limiter.check(&key).await?;
    Ok(next.run(req).await)
}
