use axum::{
    body::Bytes,
    extract::{Path, State},
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::db::AppState;
use crate::handlers::auth::AuthUser;
use erp_payments::{PaymentService, GatewayService, CreatePaymentRequest, ProcessPaymentRequest, CreateRefundRequest, PaymentMethod};
use erp_payments::{StripeService, CreatePaymentIntentRequest, CreateCheckoutSessionRequest};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/gateways", get(list_gateways).post(create_gateway))
        .route("/payments", post(create_payment))
        .route("/payments/:id", get(get_payment))
        .route("/payments/customer/:customer_id", get(list_customer_payments))
        .route("/payments/:id/refund", post(refund_payment))
        .route("/process", post(process_payment))
        .route("/stripe/intents", post(create_stripe_intent))
        .route("/stripe/intents/:id", get(get_stripe_intent))
        .route("/stripe/intents/:id/cancel", post(cancel_stripe_intent))
        .route("/stripe/checkout", post(create_stripe_checkout))
        .route("/stripe/checkout/:id", get(get_stripe_checkout))
        .route("/stripe/refund", post(create_stripe_refund))
        .route("/stripe/config", get(get_stripe_config))
        .route("/stripe/webhook", post(stripe_webhook))
}

#[derive(Deserialize)]
pub struct CreateGatewayBody {
    pub code: String,
    pub name: String,
    pub gateway_type: String,
    #[serde(default)]
    pub supported_methods: Vec<String>,
}

async fn create_gateway(
    State(state): State<AppState>,
    axum::Extension(_auth_user): axum::Extension<AuthUser>,
    Json(body): Json<CreateGatewayBody>,
) -> Json<serde_json::Value> {
    match GatewayService::create(&state.pool, body.code, body.name, body.gateway_type, body.supported_methods).await {
        Ok(gateway) => Json(json!({
            "id": gateway.id,
            "code": gateway.code,
            "name": gateway.name,
            "gateway_type": gateway.gateway_type,
            "is_active": gateway.is_active
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn list_gateways(State(state): State<AppState>) -> Json<serde_json::Value> {
    match GatewayService::list_active(&state.pool).await {
        Ok(gateways) => Json(json!({
            "items": gateways.iter().map(|g| json!({
                "id": g.id,
                "code": g.code,
                "name": g.name,
                "gateway_type": g.gateway_type,
                "is_live": g.is_live
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreatePaymentBody {
    pub gateway_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub amount: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default)]
    pub payment_method: String,
    pub card_last_four: Option<String>,
    pub card_brand: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account_last_four: Option<String>,
    pub check_number: Option<String>,
    pub notes: Option<String>,
}

fn default_currency() -> String { "USD".to_string() }

async fn create_payment(
    State(state): State<AppState>,
    Json(body): Json<CreatePaymentBody>,
) -> Json<serde_json::Value> {
    if body.amount <= 0 {
        return Json(json!({ "error": "Amount must be greater than zero" }));
    }
    
    let req = CreatePaymentRequest {
        gateway_id: body.gateway_id,
        invoice_id: body.invoice_id,
        customer_id: body.customer_id,
        amount: body.amount,
        currency: body.currency,
        payment_method: match body.payment_method.as_str() {
            "CreditCard" => PaymentMethod::CreditCard,
            "DebitCard" => PaymentMethod::DebitCard,
            "BankTransfer" => PaymentMethod::BankTransfer,
            "ACH" => PaymentMethod::ACH,
            "WireTransfer" => PaymentMethod::WireTransfer,
            "Check" => PaymentMethod::Check,
            "Cash" => PaymentMethod::Cash,
            "PayPal" => PaymentMethod::PayPal,
            "Stripe" => PaymentMethod::Stripe,
            _ => PaymentMethod::Other,
        },
        card_last_four: body.card_last_four,
        card_brand: body.card_brand,
        bank_name: body.bank_name,
        bank_account_last_four: body.bank_account_last_four,
        check_number: body.check_number,
        notes: body.notes,
    };
    match PaymentService::create(&state.pool, req, None).await {
        Ok(payment) => Json(json!({
            "id": payment.id,
            "payment_number": payment.payment_number,
            "amount": payment.amount,
            "currency": payment.currency,
            "status": payment.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_payment(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    match PaymentService::get(&state.pool, id).await {
        Ok(Some(payment)) => Json(json!({
            "id": payment.id,
            "payment_number": payment.payment_number,
            "amount": payment.amount,
            "currency": payment.currency,
            "status": payment.status,
            "payment_method": payment.payment_method,
            "paid_at": payment.paid_at
        })),
        Ok(None) => Json(json!({ "error": "Payment not found" })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn list_customer_payments(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> Json<serde_json::Value> {
    match PaymentService::list_by_customer(&state.pool, customer_id).await {
        Ok(payments) => Json(json!({
            "items": payments.iter().map(|p| json!({
                "id": p.id,
                "payment_number": p.payment_number,
                "amount": p.amount,
                "status": p.status,
                "paid_at": p.paid_at
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct RefundPaymentBody {
    pub amount: i64,
    pub reason: String,
}

async fn refund_payment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<RefundPaymentBody>,
) -> Json<serde_json::Value> {
    if body.amount <= 0 {
        return Json(json!({ "error": "Refund amount must be greater than zero" }));
    }
    
    let req = CreateRefundRequest {
        payment_id: id,
        amount: body.amount,
        reason: body.reason,
    };
    match PaymentService::refund(&state.pool, req, None).await {
        Ok(refund) => Json(json!({
            "id": refund.id,
            "refund_number": refund.refund_number,
            "amount": refund.amount,
            "status": refund.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct ProcessPaymentBody {
    pub gateway_id: Uuid,
    pub customer_id: Uuid,
    pub amount: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub payment_method_token: String,
    pub invoice_id: Option<Uuid>,
    pub description: Option<String>,
    pub metadata: Option<String>,
}

async fn process_payment(
    State(state): State<AppState>,
    Json(body): Json<ProcessPaymentBody>,
) -> Json<serde_json::Value> {
    if body.amount <= 0 {
        return Json(json!({ "error": "Amount must be greater than zero" }));
    }
    
    let req = ProcessPaymentRequest {
        gateway_id: body.gateway_id,
        customer_id: body.customer_id,
        amount: body.amount,
        currency: body.currency,
        payment_method_token: body.payment_method_token,
        invoice_id: body.invoice_id,
        description: body.description,
        metadata: body.metadata,
    };
    match GatewayService::process_payment(&state.pool, req).await {
        Ok(payment) => Json(json!({
            "id": payment.id,
            "payment_number": payment.payment_number,
            "amount": payment.amount,
            "status": payment.status,
            "processing_fee": payment.processing_fee
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateStripeIntentBody {
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub amount: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub description: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

async fn create_stripe_intent(
    State(state): State<AppState>,
    Json(body): Json<CreateStripeIntentBody>,
) -> Json<serde_json::Value> {
    if body.amount <= 0 {
        return Json(json!({ "error": "Amount must be greater than zero" }));
    }
    
    let stripe = match StripeService::from_env() {
        Ok(s) => s,
        Err(e) => return Json(json!({ "error": format!("Failed to initialize Stripe: {}", e) })),
    };
    let req = CreatePaymentIntentRequest {
        customer_id: body.customer_id,
        invoice_id: body.invoice_id,
        amount: body.amount,
        currency: body.currency,
        description: body.description,
        metadata: body.metadata,
    };
    
    match stripe.create_payment_intent(&state.pool, req).await {
        Ok(intent) => Json(json!({
            "id": intent.id,
            "stripe_intent_id": intent.stripe_intent_id,
            "client_secret": intent.client_secret,
            "amount": intent.amount,
            "currency": intent.currency,
            "status": intent.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_stripe_intent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let stripe = match StripeService::from_env() {
        Ok(s) => s,
        Err(e) => return Json(json!({ "error": format!("Failed to initialize Stripe: {}", e) })),
    };
    match stripe.get_payment_intent(&state.pool, id).await {
        Ok(Some(intent)) => Json(json!({
            "id": intent.id,
            "stripe_intent_id": intent.stripe_intent_id,
            "amount": intent.amount,
            "currency": intent.currency,
            "status": intent.status,
            "created_at": intent.created_at
        })),
        Ok(None) => Json(json!({ "error": "Payment intent not found" })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CancelStripeIntentBody {
    pub stripe_intent_id: String,
}

async fn cancel_stripe_intent(
    State(_state): State<AppState>,
    Json(body): Json<CancelStripeIntentBody>,
) -> Json<serde_json::Value> {
    let stripe = match StripeService::from_env() {
        Ok(s) => s,
        Err(e) => return Json(json!({ "error": format!("Failed to initialize Stripe: {}", e) })),
    };
    match stripe.cancel_payment_intent(&body.stripe_intent_id).await {
        Ok(intent) => Json(json!({
            "id": intent.id,
            "status": intent.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateStripeCheckoutBody {
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub amount: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub description: Option<String>,
    pub success_url: String,
    pub cancel_url: String,
    pub customer_email: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

async fn create_stripe_checkout(
    State(state): State<AppState>,
    Json(body): Json<CreateStripeCheckoutBody>,
) -> Json<serde_json::Value> {
    if body.amount <= 0 {
        return Json(json!({ "error": "Amount must be greater than zero" }));
    }
    
    let stripe = match StripeService::from_env() {
        Ok(s) => s,
        Err(e) => return Json(json!({ "error": format!("Failed to initialize Stripe: {}", e) })),
    };
    let req = CreateCheckoutSessionRequest {
        customer_id: body.customer_id,
        invoice_id: body.invoice_id,
        amount: body.amount,
        currency: body.currency,
        description: body.description,
        success_url: body.success_url,
        cancel_url: body.cancel_url,
        customer_email: body.customer_email,
        metadata: body.metadata,
    };
    
    match stripe.create_checkout_session(&state.pool, req).await {
        Ok(session) => Json(json!({
            "id": session.id,
            "stripe_session_id": session.stripe_session_id,
            "checkout_url": session.checkout_url,
            "amount": session.amount,
            "currency": session.currency,
            "status": session.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_stripe_checkout(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let stripe = match StripeService::from_env() {
        Ok(s) => s,
        Err(e) => return Json(json!({ "error": format!("Failed to initialize Stripe: {}", e) })),
    };
    match stripe.get_checkout_session(&state.pool, id).await {
        Ok(Some(session)) => Json(json!({
            "id": session.id,
            "stripe_session_id": session.stripe_session_id,
            "checkout_url": session.checkout_url,
            "amount": session.amount,
            "currency": session.currency,
            "status": session.status,
            "completed_at": session.completed_at
        })),
        Ok(None) => Json(json!({ "error": "Checkout session not found" })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateStripeRefundBody {
    pub stripe_intent_id: String,
    pub amount: Option<i64>,
    pub reason: Option<String>,
}

async fn create_stripe_refund(
    State(state): State<AppState>,
    Json(body): Json<CreateStripeRefundBody>,
) -> Json<serde_json::Value> {
    let stripe = match StripeService::from_env() {
        Ok(s) => s,
        Err(e) => return Json(json!({ "error": format!("Failed to initialize Stripe: {}", e) })),
    };
    match stripe.create_refund(&state.pool, &body.stripe_intent_id, body.amount, body.reason).await {
        Ok(refund) => Json(json!({
            "id": refund.id,
            "amount": refund.amount,
            "status": refund.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_stripe_config() -> Json<serde_json::Value> {
    let stripe = match StripeService::from_env() {
        Ok(s) => s,
        Err(e) => return Json(json!({ "error": format!("Failed to initialize Stripe: {}", e) })),
    };
    Json(json!({
        "publishable_key": stripe.get_publishable_key()
    }))
}

async fn stripe_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Json<serde_json::Value> {
    let stripe = match StripeService::from_env() {
        Ok(s) => s,
        Err(e) => return Json(json!({ "error": format!("Failed to initialize Stripe: {}", e) })),
    };
    
    let signature = match headers.get("stripe-signature").and_then(|v| v.to_str().ok()) {
        Some(s) => s,
        None => return Json(json!({ "error": "Missing stripe-signature header" })),
    };
    
    match stripe.verify_webhook_signature(&body, signature) {
        Ok(webhook) => {
            match stripe.process_webhook_event(&state.pool, webhook).await {
                Ok(()) => Json(json!({ "received": true })),
                Err(e) => Json(json!({ "error": e.to_string() })),
            }
        }
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}
