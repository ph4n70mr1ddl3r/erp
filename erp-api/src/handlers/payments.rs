use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::db::AppState;
use erp_payments::{PaymentService, GatewayService, CreatePaymentRequest, ProcessPaymentRequest, CreateRefundRequest, PaymentMethod};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/gateways", get(list_gateways).post(create_gateway))
        .route("/payments", post(create_payment))
        .route("/payments/:id", get(get_payment))
        .route("/payments/customer/:customer_id", get(list_customer_payments))
        .route("/payments/:id/refund", post(refund_payment))
        .route("/process", post(process_payment))
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
