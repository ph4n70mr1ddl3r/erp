use crate::models::*;
use crate::repository::*;
use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct StripeConfig {
    pub secret_key: String,
    pub webhook_secret: String,
    pub publishable_key: String,
    pub is_live: bool,
}

impl StripeConfig {
    pub fn from_env() -> Self {
        Self {
            secret_key: std::env::var("STRIPE_SECRET_KEY").unwrap_or_else(|_| "sk_test_".to_string()),
            webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_else(|_| "whsec_".to_string()),
            publishable_key: std::env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_else(|_| "pk_test_".to_string()),
            is_live: std::env::var("STRIPE_LIVE").unwrap_or_else(|_| "false".to_string()) == "true",
        }
    }
    
    pub fn api_base_url(&self) -> &str {
        "https://api.stripe.com/v1"
    }
}

pub struct StripeService {
    client: Client,
    config: StripeConfig,
}

impl StripeService {
    pub fn new(config: StripeConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }
    
    pub fn from_env() -> Self {
        Self::new(StripeConfig::from_env())
    }
    
    pub async fn create_payment_intent(
        &self,
        pool: &SqlitePool,
        req: CreatePaymentIntentRequest,
    ) -> Result<PaymentIntentResponse> {
        let amount_in_cents = req.amount;
        let currency = req.currency.to_lowercase();
        
        let mut form_data: Vec<(String, String)> = vec![
            ("amount".to_string(), amount_in_cents.to_string()),
            ("currency".to_string(), currency.clone()),
            ("automatic_payment_methods[enabled]".to_string(), "true".to_string()),
        ];
        
        if let Some(ref desc) = req.description {
            form_data.push(("description".to_string(), desc.clone()));
        }
        
        if let Some(ref metadata) = req.metadata {
            for (key, value) in metadata {
                form_data.push((format!("metadata[{}]", key), value.clone()));
            }
        }
        
        let response = self.client
            .post(format!("{}/payment_intents", self.config.api_base_url()))
            .basic_auth(&self.config.secret_key, Some(""))
            .form(&form_data)
            .send()
            .await
            .context("Failed to send request to Stripe")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Stripe API error: {}", error_text);
        }
        
        let stripe_response: StripePaymentIntentResponse = response
            .json()
            .await
            .context("Failed to parse Stripe response")?;
        
        let now = Utc::now();
        let payment_intent = StripePaymentIntent {
            id: Uuid::new_v4(),
            stripe_intent_id: stripe_response.id.clone(),
            customer_id: req.customer_id,
            invoice_id: req.invoice_id,
            amount: req.amount,
            currency: currency.clone(),
            status: stripe_response.status.clone(),
            client_secret: stripe_response.client_secret.clone(),
            description: req.description,
            metadata: req.metadata.map(|m| serde_json::to_string(&m).unwrap_or_default()),
            created_at: now,
            updated_at: now,
        };
        
        StripeRepository::create_payment_intent(pool, &payment_intent).await?;
        
        Ok(PaymentIntentResponse {
            id: payment_intent.id,
            stripe_intent_id: payment_intent.stripe_intent_id,
            client_secret: stripe_response.client_secret.unwrap_or_default(),
            amount: payment_intent.amount,
            currency: payment_intent.currency,
            status: payment_intent.status,
        })
    }
    
    pub async fn create_checkout_session(
        &self,
        pool: &SqlitePool,
        req: CreateCheckoutSessionRequest,
    ) -> Result<CheckoutSessionResponse> {
        let amount_in_cents = req.amount;
        let currency = req.currency.to_lowercase();
        
        let mut form_data: Vec<(String, String)> = vec![
            ("mode".to_string(), "payment".to_string()),
            ("line_items[0][price_data][currency]".to_string(), currency.clone()),
            ("line_items[0][price_data][unit_amount]".to_string(), amount_in_cents.to_string()),
            ("line_items[0][price_data][product_data][name]".to_string(), req.description.clone().unwrap_or_else(|| "Payment".to_string())),
            ("line_items[0][quantity]".to_string(), "1".to_string()),
            ("success_url".to_string(), req.success_url.clone()),
            ("cancel_url".to_string(), req.cancel_url.clone()),
        ];
        
        if let Some(ref email) = req.customer_email {
            form_data.push(("customer_email".to_string(), email.clone()));
        }
        
        if let Some(ref metadata) = req.metadata {
            for (key, value) in metadata {
                form_data.push((format!("metadata[{}]", key), value.clone()));
            }
        }
        
        let response = self.client
            .post(format!("{}/checkout/sessions", self.config.api_base_url()))
            .basic_auth(&self.config.secret_key, Some(""))
            .form(&form_data)
            .send()
            .await
            .context("Failed to send request to Stripe")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Stripe API error: {}", error_text);
        }
        
        let stripe_response: StripeCheckoutSessionResponse = response
            .json()
            .await
            .context("Failed to parse Stripe response")?;
        
        let now = Utc::now();
        let checkout_session = StripeCheckoutSession {
            id: Uuid::new_v4(),
            stripe_session_id: stripe_response.id.clone(),
            customer_id: req.customer_id,
            invoice_id: req.invoice_id,
            amount: req.amount,
            currency: currency.clone(),
            status: stripe_response.status.clone(),
            checkout_url: stripe_response.url.clone(),
            success_url: req.success_url,
            cancel_url: req.cancel_url,
            payment_intent_id: stripe_response.payment_intent,
            expires_at: None,
            completed_at: None,
            created_at: now,
        };
        
        StripeRepository::create_checkout_session(pool, &checkout_session).await?;
        
        Ok(CheckoutSessionResponse {
            id: checkout_session.id,
            stripe_session_id: checkout_session.stripe_session_id,
            checkout_url: stripe_response.url.unwrap_or_default(),
            amount: checkout_session.amount,
            currency: checkout_session.currency,
            status: checkout_session.status,
        })
    }
    
    pub async fn retrieve_payment_intent(&self, payment_intent_id: &str) -> Result<StripePaymentIntentResponse> {
        let response = self.client
            .get(&format!("{}/payment_intents/{}", self.config.api_base_url(), payment_intent_id))
            .basic_auth(&self.config.secret_key, Some(""))
            .send()
            .await
            .context("Failed to retrieve payment intent from Stripe")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Stripe API error: {}", error_text);
        }
        
        response.json().await.context("Failed to parse Stripe response")
    }
    
    pub async fn cancel_payment_intent(&self, payment_intent_id: &str) -> Result<StripePaymentIntentResponse> {
        let response = self.client
            .post(format!("{}/payment_intents/{}/cancel", self.config.api_base_url().to_string(), payment_intent_id))
            .basic_auth(&self.config.secret_key, Some(""))
            .send()
            .await
            .context("Failed to cancel payment intent")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Stripe API error: {}", error_text);
        }
        
        response.json().await.context("Failed to parse Stripe response")
    }
    
    pub async fn create_refund(
        &self,
        pool: &SqlitePool,
        payment_intent_id: &str,
        amount: Option<i64>,
        reason: Option<String>,
    ) -> Result<StripeRefundResponse> {
let mut form_data: Vec<(String, String)> = vec![("payment_intent".to_string(), payment_intent_id.to_string())];
        
        if let Some(amt) = amount {
            form_data.push(("amount".to_string(), amt.to_string()));
        }
        
        if let Some(rsn) = reason {
            form_data.push(("reason".to_string(), rsn));
        }
    }
        
        if let Some(rsn) = reason {
            form_data.push(("reason", rsn));
        }
        
        let response = self.client
            .post(format!("{}/refunds", self.config.api_base_url().to_string()))
            .basic_auth(&self.config.secret_key, Some(""))
            .form(&form_data)
            .send()
            .await
            .context("Failed to create refund with Stripe")?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Stripe API error: {}", error_text);
        }
        
        let refund_response: StripeRefundResponse = response
            .json()
            .await
            .context("Failed to parse Stripe response")?;
        
        if refund_response.status == "succeeded" {
            if let Some(intent_id) = &refund_response.payment_intent {
                if let Ok(Some(mut payment_intent)) = StripeRepository::get_payment_intent_by_stripe_id(pool, intent_id).await {
                    payment_intent.status = "refunded".to_string();
                    payment_intent.updated_at = Utc::now();
                    let _ = StripeRepository::update_payment_intent_status(pool, &payment_intent).await;
                }
            }
        }
        
        Ok(refund_response)
    }
    
    pub fn verify_webhook_signature(&self, payload: &[u8], signature: &str) -> Result<StripeWebhookPayload> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(self.config.webhook_secret.as_bytes())
            .context("Invalid webhook secret")?;
        mac.update(payload);
        
        let expected_signature = hex::encode(mac.finalize().into_bytes());
        
        let signature_parts: Vec<&str> = signature.split(',').collect();
        let mut signature_value = "";
        let mut timestamp = "";
        
        for part in signature_parts {
            if let Some(val) = part.strip_prefix("v1=") {
                signature_value = val;
            } else if let Some(val) = part.strip_prefix("t=") {
                timestamp = val;
            }
        }
        
        if signature_value.is_empty() || timestamp.is_empty() {
            anyhow::bail!("Invalid webhook signature format");
        }
        
        let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));
        let mut mac2 = HmacSha256::new_from_slice(self.config.webhook_secret.as_bytes())?;
        mac2.update(signed_payload.as_bytes());
        let expected = hex::encode(mac2.finalize().into_bytes());
        
        if !expected.eq_ignore_ascii_case(signature_value) {
            anyhow::bail!("Webhook signature verification failed");
        }
        
        let payload_str = String::from_utf8_lossy(payload);
        let webhook: StripeWebhookPayload = serde_json::from_str(&payload_str)
            .context("Failed to parse webhook payload")?;
        
        Ok(webhook)
    }
    
    pub async fn process_webhook_event(
        &self,
        pool: &SqlitePool,
        webhook: StripeWebhookPayload,
    ) -> Result<()> {
        let now = Utc::now();
        let event_id = Uuid::new_v4();
        
        let webhook_event = StripeWebhookEvent {
            id: event_id,
            stripe_event_id: webhook.id.clone(),
            event_type: webhook.event_type.clone(),
            payload: serde_json::to_string(&webhook).unwrap_or_default(),
            processed: false,
            processed_at: None,
            error_message: None,
            created_at: now,
        };
        
        StripeRepository::create_webhook_event(pool, &webhook_event).await?;
        
        let result = self.handle_webhook_event(pool, &webhook).await;
        
        match result {
            Ok(()) => {
                sqlx::query(
                    r#"UPDATE stripe_webhook_events SET processed = 1, processed_at = ? WHERE id = ?"#
                )
                .bind(now.to_rfc3339())
                .bind(event_id.to_string())
                .execute(pool).await?;
            }
            Err(e) => {
                sqlx::query(
                    r#"UPDATE stripe_webhook_events SET error_message = ? WHERE id = ?"#
                )
                .bind(e.to_string())
                .bind(event_id.to_string())
                .execute(pool).await?;
            }
        }
        
        Ok(())
    }
    
    async fn handle_webhook_event(&self, pool: &SqlitePool, webhook: &StripeWebhookPayload) -> Result<()> {
        match webhook.event_type.as_str() {
            "payment_intent.succeeded" => {
                if let Some(data) = &webhook.data.object {
                    if let Some(intent_id) = data.get("id").and_then(|v| v.as_str()) {
                        if let Ok(Some(mut payment_intent)) = StripeRepository::get_payment_intent_by_stripe_id(pool, intent_id).await {
                            payment_intent.status = "succeeded".to_string();
                            payment_intent.updated_at = Utc::now();
                            StripeRepository::update_payment_intent_status(pool, &payment_intent).await?;
                            
                            let payment = Payment {
                                id: Uuid::new_v4(),
                                payment_number: format!("PAY-{}", Utc::now().format("%Y%m%d%H%M%S")),
                                gateway_id: None,
                                invoice_id: payment_intent.invoice_id,
                                customer_id: payment_intent.customer_id,
                                amount: payment_intent.amount,
                                currency: payment_intent.currency.clone(),
                                payment_method: PaymentMethod::Stripe,
                                status: PaymentStatus::Completed,
                                gateway_transaction_id: Some(intent_id.to_string()),
                                gateway_response: Some(serde_json::to_string(&data).unwrap_or_default()),
                                card_last_four: data.get("charges").and_then(|c| c.get("data")).and_then(|d| d.get(0)).and_then(|c| c.get("payment_method_details")).and_then(|p| p.get("card")).and_then(|c| c.get("last4")).and_then(|v| v.as_str()).map(|s| s.to_string()),
                                card_brand: data.get("charges").and_then(|c| c.get("data")).and_then(|d| d.get(0)).and_then(|c| c.get("payment_method_details")).and_then(|p| p.get("card")).and_then(|c| c.get("brand")).and_then(|v| v.as_str()).map(|s| s.to_string()),
                                bank_name: None,
                                bank_account_last_four: None,
                                check_number: None,
                                refunded_amount: 0,
                                refund_reason: None,
                                processing_fee: data.get("charges").and_then(|c| c.get("data")).and_then(|d| d.get(0)).and_then(|c| c.get("balance_transaction")).and_then(|b| b.get("fee")).and_then(|v| v.as_i64()).unwrap_or(0),
                                notes: payment_intent.description.clone(),
                                paid_at: Some(Utc::now()),
                                created_at: Utc::now(),
                                created_by: None,
                            };
                            
                            PaymentRepository::create(pool, &payment).await?;
                        }
                    }
                }
            }
            "payment_intent.payment_failed" => {
                if let Some(data) = &webhook.data.object {
                    if let Some(intent_id) = data.get("id").and_then(|v| v.as_str()) {
                        if let Ok(Some(mut payment_intent)) = StripeRepository::get_payment_intent_by_stripe_id(pool, intent_id).await {
                            payment_intent.status = "failed".to_string();
                            payment_intent.updated_at = Utc::now();
                            StripeRepository::update_payment_intent_status(pool, &payment_intent).await?;
                        }
                    }
                }
            }
            "checkout.session.completed" => {
                if let Some(data) = &webhook.data.object {
                    if let Some(session_id) = data.get("id").and_then(|v| v.as_str()) {
                        if let Ok(Some(mut checkout_session)) = StripeRepository::get_checkout_session_by_stripe_id(pool, session_id).await {
                            checkout_session.status = "completed".to_string();
                            checkout_session.completed_at = Some(Utc::now());
                            StripeRepository::update_checkout_session_status(pool, &checkout_session).await?;
                        }
                    }
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    pub async fn get_payment_intent(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<StripePaymentIntent>> {
        StripeRepository::get_payment_intent_by_id(pool, id).await
    }
    
    pub async fn get_checkout_session(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<StripeCheckoutSession>> {
        StripeRepository::get_checkout_session_by_id(pool, id).await
    }
    
    pub async fn list_payment_intents_by_customer(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<StripePaymentIntent>> {
        StripeRepository::list_payment_intents_by_customer(pool, customer_id).await
    }
    
    pub fn get_publishable_key(&self) -> &str {
        &self.config.publishable_key
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripePaymentIntentResponse {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub amount_capturable: i64,
    pub amount_received: i64,
    pub currency: String,
    pub status: String,
    pub client_secret: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeCheckoutSessionResponse {
    pub id: String,
    pub object: String,
    pub status: String,
    pub url: Option<String>,
    pub payment_intent: Option<String>,
    pub customer_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeRefundResponse {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub status: String,
    pub payment_intent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeWebhookPayload {
    pub id: String,
    pub object: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: StripeWebhookData,
    pub created: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StripeWebhookData {
    pub object: Option<serde_json::Value>,
}
