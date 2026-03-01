use erp_core::BaseEntity;
use rand::Rng;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct WebhookService {
    endpoint_repo: SqliteWebhookEndpointRepository,
    delivery_repo: SqliteWebhookDeliveryRepository,
    event_repo: SqliteWebhookEventRepository,
}

impl WebhookService {
    pub fn new() -> Self {
        Self {
            endpoint_repo: SqliteWebhookEndpointRepository,
            delivery_repo: SqliteWebhookDeliveryRepository,
            event_repo: SqliteWebhookEventRepository,
        }
    }

    pub async fn create_endpoint(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        url: String,
        events: Vec<WebhookEventType>,
        created_by: Uuid,
        headers: Option<serde_json::Value>,
        authentication: Option<WebhookAuth>,
    ) -> anyhow::Result<WebhookEndpoint> {
        let secret = generate_secret();
        
        let endpoint = WebhookEndpoint {
            base: BaseEntity::new(),
            name,
            description,
            url,
            secret,
            events,
            headers,
            authentication,
            timeout_seconds: 30,
            retry_policy: RetryPolicy::default(),
            status: WebhookStatus::Active,
            created_by,
            last_triggered_at: None,
            last_success_at: None,
            last_failure_at: None,
            total_triggers: 0,
            successful_triggers: 0,
            failed_triggers: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        self.endpoint_repo.create(pool, &endpoint).await
    }

    pub async fn get_endpoint(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<WebhookEndpoint>> {
        self.endpoint_repo.get_by_id(pool, id).await
    }

    pub async fn list_endpoints(&self, pool: &SqlitePool) -> anyhow::Result<Vec<WebhookEndpoint>> {
        self.endpoint_repo.list(pool).await
    }

    pub async fn update_endpoint(&self, pool: &SqlitePool, endpoint: &WebhookEndpoint) -> anyhow::Result<()> {
        self.endpoint_repo.update(pool, endpoint).await
    }

    pub async fn delete_endpoint(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.endpoint_repo.delete(pool, id).await
    }

    pub async fn trigger(
        &self,
        pool: &SqlitePool,
        event_type: WebhookEventType,
        source_entity_type: String,
        source_entity_id: Uuid,
        payload: serde_json::Value,
        triggered_by: Uuid,
    ) -> anyhow::Result<Vec<WebhookDelivery>> {
        let event = WebhookEvent {
            base: BaseEntity::new(),
            event_type: event_type.clone(),
            source_entity_type,
            source_entity_id,
            payload: payload.clone(),
            triggered_by,
            triggered_at: chrono::Utc::now(),
            delivered: false,
            delivery_count: 0,
            created_at: chrono::Utc::now(),
        };
        
        let created_event = self.event_repo.create(pool, &event).await?;
        
        let endpoints = self.endpoint_repo.list_for_event(pool, &event_type).await?;
        let mut deliveries = Vec::new();
        
        for endpoint in endpoints {
            let delivery = WebhookDelivery {
                base: BaseEntity::new(),
                endpoint_id: endpoint.base.id,
                event_type: event_type.clone(),
                event_id: created_event.base.id,
                payload: payload.clone(),
                headers: endpoint.headers.clone(),
                response_status: None,
                response_body: None,
                response_headers: None,
                duration_ms: None,
                attempt_number: 0,
                max_attempts: endpoint.retry_policy.max_retries,
                next_retry_at: Some(chrono::Utc::now()),
                delivered_at: None,
                status: DeliveryStatus::Pending,
                error_message: None,
                created_at: chrono::Utc::now(),
            };
            
            let created_delivery = self.delivery_repo.create(pool, &delivery).await?;
            deliveries.push(created_delivery);
        }
        
        Ok(deliveries)
    }

    pub async fn get_deliveries(&self, pool: &SqlitePool, endpoint_id: Uuid, limit: i32) -> anyhow::Result<Vec<WebhookDelivery>> {
        self.delivery_repo.list_by_endpoint(pool, endpoint_id, limit).await
    }

    pub async fn get_pending_deliveries(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<WebhookDelivery>> {
        self.delivery_repo.list_pending(pool, limit).await
    }

    pub async fn process_delivery(&self, pool: &SqlitePool, delivery_id: Uuid) -> anyhow::Result<WebhookDelivery> {
        let mut delivery = self.delivery_repo.get_by_id(pool, delivery_id).await?
            .ok_or_else(|| anyhow::anyhow!("Delivery not found"))?;
        
        let endpoint = self.endpoint_repo.get_by_id(pool, delivery.endpoint_id).await?
            .ok_or_else(|| anyhow::anyhow!("Endpoint not found"))?;
        
        let start = std::time::Instant::now();
        delivery.attempt_number += 1;
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(endpoint.timeout_seconds as u64))
            .build()?;
        
        let signature = WebhookSignature::new(&endpoint.secret, &serde_json::to_vec(&delivery.payload)?);
        
        let mut request = client
            .post(&endpoint.url)
            .json(&delivery.payload)
            .header("Content-Type", "application/json")
            .header("X-Webhook-Signature", &signature.signature)
            .header("X-Webhook-Timestamp", signature.timestamp)
            .header("X-Webhook-Event", format!("{:?}", delivery.event_type))
            .header("X-Webhook-Delivery", delivery.base.id.to_string());
        
        if let Some(auth) = &endpoint.authentication {
            request = apply_auth(request, auth);
        }
        
        if let Some(serde_json::Value::Object(map)) = &endpoint.headers {
            for (key, value) in map {
                if let serde_json::Value::String(v) = value {
                    request = request.header(key, v);
                }
            }
        }
        
        match request.send().await {
            Ok(response) => {
                let duration_ms = start.elapsed().as_millis() as i64;
                let status = response.status().as_u16() as i32;
                let body = response.text().await.unwrap_or_default();
                
                if (200..300).contains(&status) {
                    delivery.status = DeliveryStatus::Delivered;
                    delivery.delivered_at = Some(chrono::Utc::now());
                    self.endpoint_repo.update_stats(pool, endpoint.base.id, true).await?;
                    self.event_repo.mark_delivered(pool, delivery.event_id).await?;
                } else if delivery.attempt_number >= delivery.max_attempts {
                    delivery.status = DeliveryStatus::Abandoned;
                    delivery.error_message = Some(format!("HTTP {}: {}", status, body));
                    self.endpoint_repo.update_stats(pool, endpoint.base.id, false).await?;
                } else {
                    delivery.status = DeliveryStatus::Retrying;
                    delivery.next_retry_at = Some(calculate_next_retry(&endpoint.retry_policy, delivery.attempt_number));
                    delivery.error_message = Some(format!("HTTP {}: {}", status, body));
                }
                
                delivery.response_status = Some(status);
                delivery.response_body = Some(body);
                delivery.duration_ms = Some(duration_ms);
            }
            Err(e) => {
                if delivery.attempt_number >= delivery.max_attempts {
                    delivery.status = DeliveryStatus::Abandoned;
                    self.endpoint_repo.update_stats(pool, endpoint.base.id, false).await?;
                } else {
                    delivery.status = DeliveryStatus::Retrying;
                    delivery.next_retry_at = Some(calculate_next_retry(&endpoint.retry_policy, delivery.attempt_number));
                }
                delivery.error_message = Some(e.to_string());
            }
        }
        
        self.delivery_repo.update(pool, &delivery).await?;
        Ok(delivery)
    }

    pub async fn rotate_secret(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<String> {
        let mut endpoint = self.endpoint_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Endpoint not found"))?;
        
        let new_secret = generate_secret();
        endpoint.secret = new_secret.clone();
        
        self.endpoint_repo.update(pool, &endpoint).await?;
        Ok(new_secret)
    }

    pub async fn ping_endpoint(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<bool> {
        let endpoint = self.endpoint_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Endpoint not found"))?;
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        
        let response = client
            .get(&endpoint.url)
            .send()
            .await?;
        
        Ok(response.status().is_success())
    }
}

fn generate_secret() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn calculate_next_retry(policy: &RetryPolicy, attempt: i32) -> chrono::DateTime<chrono::Utc> {
    let delay_ms = std::cmp::min(
        (policy.initial_delay_ms as f64 * policy.backoff_multiplier.powi(attempt - 1)) as i64,
        policy.max_delay_ms,
    );
    chrono::Utc::now() + chrono::Duration::milliseconds(delay_ms)
}

fn apply_auth(request: reqwest::RequestBuilder, auth: &WebhookAuth) -> reqwest::RequestBuilder {
    match auth.auth_type {
        WebhookAuthType::Basic => {
            if let (Some(username), Some(password)) = (&auth.username, &auth.password) {
                request.basic_auth(username, Some(password))
            } else {
                request
            }
        }
        WebhookAuthType::Bearer => {
            if let Some(token) = &auth.bearer_token {
                request.bearer_auth(token)
            } else {
                request
            }
        }
        WebhookAuthType::ApiKey => {
            if let (Some(key), Some(header)) = (&auth.api_key, &auth.api_key_header) {
                request.header(header, key)
            } else {
                request
            }
        }
        _ => request,
    }
}
