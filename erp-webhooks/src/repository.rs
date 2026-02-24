use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait WebhookEndpointRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, endpoint: &WebhookEndpoint) -> anyhow::Result<WebhookEndpoint>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<WebhookEndpoint>>;
    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<WebhookEndpoint>>;
    async fn list_active(&self, pool: &SqlitePool) -> anyhow::Result<Vec<WebhookEndpoint>>;
    async fn list_for_event(&self, pool: &SqlitePool, event_type: &WebhookEventType) -> anyhow::Result<Vec<WebhookEndpoint>>;
    async fn update(&self, pool: &SqlitePool, endpoint: &WebhookEndpoint) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
    async fn update_stats(&self, pool: &SqlitePool, id: Uuid, success: bool) -> anyhow::Result<()>;
}

pub struct SqliteWebhookEndpointRepository;

#[async_trait]
impl WebhookEndpointRepository for SqliteWebhookEndpointRepository {
    async fn create(&self, pool: &SqlitePool, endpoint: &WebhookEndpoint) -> anyhow::Result<WebhookEndpoint> {
        let events_json = serde_json::to_string(&endpoint.events)?;
        let headers_json = endpoint.headers.as_ref().map(|h| serde_json::to_string(h).unwrap());
        let auth_json = endpoint.authentication.as_ref().map(|a| serde_json::to_string(a).unwrap());
        let retry_json = serde_json::to_string(&endpoint.retry_policy)?;
        
        let now = Utc::now();
        sqlx::query_as::<_, WebhookEndpoint>(
            r#"
            INSERT INTO webhook_endpoints (
                id, name, description, url, secret, events, headers, authentication,
                timeout_seconds, retry_policy, status, created_by, last_triggered_at,
                last_success_at, last_failure_at, total_triggers, successful_triggers,
                failed_triggers, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(endpoint.base.id)
        .bind(&endpoint.name)
        .bind(&endpoint.description)
        .bind(&endpoint.url)
        .bind(&endpoint.secret)
        .bind(&events_json)
        .bind(&headers_json)
        .bind(&auth_json)
        .bind(endpoint.timeout_seconds)
        .bind(&retry_json)
        .bind(&endpoint.status)
        .bind(endpoint.created_by)
        .bind(endpoint.last_triggered_at)
        .bind(endpoint.last_success_at)
        .bind(endpoint.last_failure_at)
        .bind(endpoint.total_triggers)
        .bind(endpoint.successful_triggers)
        .bind(endpoint.failed_triggers)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<WebhookEndpoint>> {
        sqlx::query_as::<_, WebhookEndpoint>("SELECT * FROM webhook_endpoints WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<WebhookEndpoint>> {
        sqlx::query_as::<_, WebhookEndpoint>("SELECT * FROM webhook_endpoints ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_active(&self, pool: &SqlitePool) -> anyhow::Result<Vec<WebhookEndpoint>> {
        sqlx::query_as::<_, WebhookEndpoint>("SELECT * FROM webhook_endpoints WHERE status = 'Active' ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_for_event(&self, pool: &SqlitePool, event_type: &WebhookEventType) -> anyhow::Result<Vec<WebhookEndpoint>> {
        let event_str = serde_json::to_string(&vec![event_type.clone()])?;
        sqlx::query_as::<_, WebhookEndpoint>(
            "SELECT * FROM webhook_endpoints WHERE status = 'Active' AND events LIKE ?"
        )
        .bind(format!("%{}%", event_str.trim_matches(|c| c == '[' || c == ']' || c == '"')))
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, endpoint: &WebhookEndpoint) -> anyhow::Result<()> {
        let events_json = serde_json::to_string(&endpoint.events)?;
        let headers_json = endpoint.headers.as_ref().map(|h| serde_json::to_string(h).unwrap());
        let auth_json = endpoint.authentication.as_ref().map(|a| serde_json::to_string(a).unwrap());
        let retry_json = serde_json::to_string(&endpoint.retry_policy)?;
        
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE webhook_endpoints SET
                name = ?, description = ?, url = ?, secret = ?, events = ?, headers = ?,
                authentication = ?, timeout_seconds = ?, retry_policy = ?, status = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&endpoint.name)
        .bind(&endpoint.description)
        .bind(&endpoint.url)
        .bind(&endpoint.secret)
        .bind(&events_json)
        .bind(&headers_json)
        .bind(&auth_json)
        .bind(endpoint.timeout_seconds)
        .bind(&retry_json)
        .bind(&endpoint.status)
        .bind(now)
        .bind(endpoint.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM webhook_endpoints WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_stats(&self, pool: &SqlitePool, id: Uuid, success: bool) -> anyhow::Result<()> {
        let now = Utc::now();
        if success {
            sqlx::query(
                r#"UPDATE webhook_endpoints SET
                    total_triggers = total_triggers + 1,
                    successful_triggers = successful_triggers + 1,
                    last_triggered_at = ?,
                    last_success_at = ?
                WHERE id = ?"#
            )
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                r#"UPDATE webhook_endpoints SET
                    total_triggers = total_triggers + 1,
                    failed_triggers = failed_triggers + 1,
                    last_triggered_at = ?,
                    last_failure_at = ?
                WHERE id = ?"#
            )
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        }
        Ok(())
    }
}

#[async_trait]
pub trait WebhookDeliveryRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, delivery: &WebhookDelivery) -> anyhow::Result<WebhookDelivery>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<WebhookDelivery>>;
    async fn list_by_endpoint(&self, pool: &SqlitePool, endpoint_id: Uuid, limit: i32) -> anyhow::Result<Vec<WebhookDelivery>>;
    async fn list_pending(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<WebhookDelivery>>;
    async fn update(&self, pool: &SqlitePool, delivery: &WebhookDelivery) -> anyhow::Result<()>;
}

pub struct SqliteWebhookDeliveryRepository;

#[async_trait]
impl WebhookDeliveryRepository for SqliteWebhookDeliveryRepository {
    async fn create(&self, pool: &SqlitePool, delivery: &WebhookDelivery) -> anyhow::Result<WebhookDelivery> {
        let headers_json = delivery.headers.as_ref().map(|h| serde_json::to_string(h).unwrap());
        let response_headers_json = delivery.response_headers.as_ref().map(|h| serde_json::to_string(h).unwrap());
        
        let now = Utc::now();
        sqlx::query_as::<_, WebhookDelivery>(
            r#"
            INSERT INTO webhook_deliveries (
                id, endpoint_id, event_type, event_id, payload, headers, response_status,
                response_body, response_headers, duration_ms, attempt_number, max_attempts,
                next_retry_at, delivered_at, status, error_message, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(delivery.base.id)
        .bind(delivery.endpoint_id)
        .bind(&delivery.event_type)
        .bind(delivery.event_id)
        .bind(&delivery.payload)
        .bind(&headers_json)
        .bind(delivery.response_status)
        .bind(&delivery.response_body)
        .bind(&response_headers_json)
        .bind(delivery.duration_ms)
        .bind(delivery.attempt_number)
        .bind(delivery.max_attempts)
        .bind(delivery.next_retry_at)
        .bind(delivery.delivered_at)
        .bind(&delivery.status)
        .bind(&delivery.error_message)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<WebhookDelivery>> {
        sqlx::query_as::<_, WebhookDelivery>("SELECT * FROM webhook_deliveries WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_endpoint(&self, pool: &SqlitePool, endpoint_id: Uuid, limit: i32) -> anyhow::Result<Vec<WebhookDelivery>> {
        sqlx::query_as::<_, WebhookDelivery>(
            "SELECT * FROM webhook_deliveries WHERE endpoint_id = ? ORDER BY created_at DESC LIMIT ?"
        )
        .bind(endpoint_id)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn list_pending(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<WebhookDelivery>> {
        let now = Utc::now();
        sqlx::query_as::<_, WebhookDelivery>(
            "SELECT * FROM webhook_deliveries WHERE status IN ('Pending', 'Retrying') AND (next_retry_at IS NULL OR next_retry_at <= ?) LIMIT ?"
        )
        .bind(now)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, delivery: &WebhookDelivery) -> anyhow::Result<()> {
        let response_headers_json = delivery.response_headers.as_ref().map(|h| serde_json::to_string(h).unwrap());
        
        sqlx::query(
            r#"
            UPDATE webhook_deliveries SET
                response_status = ?, response_body = ?, response_headers = ?,
                duration_ms = ?, attempt_number = ?, next_retry_at = ?,
                delivered_at = ?, status = ?, error_message = ?
            WHERE id = ?
            "#,
        )
        .bind(delivery.response_status)
        .bind(&delivery.response_body)
        .bind(&response_headers_json)
        .bind(delivery.duration_ms)
        .bind(delivery.attempt_number)
        .bind(delivery.next_retry_at)
        .bind(delivery.delivered_at)
        .bind(&delivery.status)
        .bind(&delivery.error_message)
        .bind(delivery.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
pub trait WebhookEventRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, event: &WebhookEvent) -> anyhow::Result<WebhookEvent>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<WebhookEvent>>;
    async fn list_recent(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<WebhookEvent>>;
    async fn mark_delivered(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteWebhookEventRepository;

#[async_trait]
impl WebhookEventRepository for SqliteWebhookEventRepository {
    async fn create(&self, pool: &SqlitePool, event: &WebhookEvent) -> anyhow::Result<WebhookEvent> {
        let now = Utc::now();
        sqlx::query_as::<_, WebhookEvent>(
            r#"
            INSERT INTO webhook_events (
                id, event_type, source_entity_type, source_entity_id, payload,
                triggered_by, triggered_at, delivered, delivery_count, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(event.base.id)
        .bind(&event.event_type)
        .bind(&event.source_entity_type)
        .bind(event.source_entity_id)
        .bind(&event.payload)
        .bind(event.triggered_by)
        .bind(event.triggered_at)
        .bind(event.delivered)
        .bind(event.delivery_count)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<WebhookEvent>> {
        sqlx::query_as::<_, WebhookEvent>("SELECT * FROM webhook_events WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_recent(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<WebhookEvent>> {
        sqlx::query_as::<_, WebhookEvent>("SELECT * FROM webhook_events ORDER BY created_at DESC LIMIT ?")
            .bind(limit)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn mark_delivered(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("UPDATE webhook_events SET delivered = TRUE, delivery_count = delivery_count + 1 WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
