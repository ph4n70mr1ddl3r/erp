use axum::{extract::{Path, Query, State}, Json, routing::{get, post}};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Pagination, BaseEntity, Status};
use erp_barcode::{
    Barcode, BarcodeType, BarcodeEntityType, BarcodePrinter, BarcodeTemplate,
    ScanEvent, ScanAction, BarcodeValidation,
    BarcodeService, BarcodePrintService, ScanService, BarcodeValidationService,
};

#[derive(Serialize)]
pub struct BarcodeResponse {
    pub id: Uuid,
    pub barcode: String,
    pub barcode_type: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub is_primary: bool,
}

impl From<Barcode> for BarcodeResponse {
    fn from(b: Barcode) -> Self {
        Self {
            id: b.base.id,
            barcode: b.barcode,
            barcode_type: format!("{:?}", b.barcode_type),
            entity_type: format!("{:?}", b.entity_type),
            entity_id: b.entity_id,
            is_primary: b.is_primary,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateBarcodeRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub barcode_type: String,
    pub barcode: Option<String>,
}

pub async fn get_barcode(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> ApiResult<Json<BarcodeResponse>> {
    let svc = BarcodeService::new();
    Ok(Json(BarcodeResponse::from(svc.get_by_barcode(&state.pool, &code).await?)))
}

pub async fn create_barcode(
    State(state): State<AppState>,
    Json(req): Json<CreateBarcodeRequest>,
) -> ApiResult<Json<BarcodeResponse>> {
    let svc = BarcodeService::new();
    
    let entity_type = match req.entity_type.as_str() {
        "Lot" => BarcodeEntityType::Lot,
        "SerialNumber" => BarcodeEntityType::SerialNumber,
        "Asset" => BarcodeEntityType::Asset,
        "Location" => BarcodeEntityType::Location,
        "Pallet" => BarcodeEntityType::Pallet,
        "Container" => BarcodeEntityType::Container,
        "Document" => BarcodeEntityType::Document,
        "Employee" => BarcodeEntityType::Employee,
        "Customer" => BarcodeEntityType::Customer,
        _ => BarcodeEntityType::Product,
    };
    
    let barcode_type = match req.barcode_type.as_str() {
        "EAN8" => BarcodeType::EAN8,
        "UPC_A" => BarcodeType::UPC_A,
        "UPC_E" => BarcodeType::UPC_E,
        "Code128" => BarcodeType::Code128,
        "Code39" => BarcodeType::Code39,
        "Code93" => BarcodeType::Code93,
        "ITF14" => BarcodeType::ITF14,
        "QRCode" => BarcodeType::QRCode,
        "DataMatrix" => BarcodeType::DataMatrix,
        "PDF417" => BarcodeType::PDF417,
        "GS1_128" => BarcodeType::GS1_128,
        _ => BarcodeType::EAN13,
    };
    
    if let Some(code) = req.barcode {
        let barcode = Barcode {
            base: BaseEntity::new(),
            barcode: code,
            barcode_type,
            entity_type,
            entity_id: req.entity_id,
            definition_id: None,
            is_primary: true,
            status: Status::Active,
            created_at: Utc::now(),
        };
        Ok(Json(BarcodeResponse::from(svc.create(&state.pool, barcode).await?)))
    } else {
        let barcode = svc.generate(&state.pool, entity_type, req.entity_id, barcode_type, None).await?;
        Ok(Json(BarcodeResponse::from(barcode)))
    }
}

#[derive(Serialize)]
pub struct PrintJobResponse {
    pub id: Uuid,
    pub job_number: String,
    pub printer_id: Uuid,
    pub quantity: i32,
    pub status: String,
}

pub async fn create_print_job(
    State(state): State<AppState>,
    Json(req): Json<CreatePrintJobRequest>,
) -> ApiResult<Json<PrintJobResponse>> {
    let svc = BarcodePrintService::new();
    let items: Vec<(Uuid, String, erp_barcode::BarcodeEntityType, Uuid)> = req.items.into_iter().map(|(id, barcode, entity_type, entity_id)| {
        let et = match entity_type.as_str() {
            "Lot" => erp_barcode::BarcodeEntityType::Lot,
            "SerialNumber" => erp_barcode::BarcodeEntityType::SerialNumber,
            "Asset" => erp_barcode::BarcodeEntityType::Asset,
            "Location" => erp_barcode::BarcodeEntityType::Location,
            "Pallet" => erp_barcode::BarcodeEntityType::Pallet,
            "Container" => erp_barcode::BarcodeEntityType::Container,
            "Document" => erp_barcode::BarcodeEntityType::Document,
            "Employee" => erp_barcode::BarcodeEntityType::Employee,
            "Customer" => erp_barcode::BarcodeEntityType::Customer,
            _ => erp_barcode::BarcodeEntityType::Product,
        };
        (id, barcode, et, entity_id)
    }).collect();
    let job = svc.create_job(&state.pool, req.printer_id, req.template_id, items).await?;
    
    Ok(Json(PrintJobResponse {
        id: job.base.id,
        job_number: job.job_number,
        printer_id: job.printer_id,
        quantity: job.quantity,
        status: format!("{:?}", job.status),
    }))
}

#[derive(Deserialize)]
pub struct CreatePrintJobRequest {
    pub printer_id: Uuid,
    pub template_id: Uuid,
    pub items: Vec<(Uuid, String, String, Uuid)>,
}

pub async fn complete_print_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    BarcodePrintService::new().complete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "completed" })))
}

#[derive(Serialize)]
pub struct ScanEventResponse {
    pub id: Uuid,
    pub barcode: String,
    pub action: String,
    pub quantity: i64,
    pub scanned_at: String,
}

impl From<ScanEvent> for ScanEventResponse {
    fn from(e: ScanEvent) -> Self {
        Self {
            id: e.id,
            barcode: e.barcode,
            action: format!("{:?}", e.action),
            quantity: e.quantity,
            scanned_at: e.scanned_at.to_rfc3339(),
        }
    }
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub barcode: String,
    pub barcode_type: String,
    pub scanner_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub action: String,
    pub quantity: Option<i64>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
}

pub async fn scan(
    State(state): State<AppState>,
    Json(req): Json<ScanRequest>,
) -> ApiResult<Json<ScanEventResponse>> {
    let svc = ScanService::new();
    
    let barcode_type = match req.barcode_type.as_str() {
        "EAN8" => BarcodeType::EAN8,
        "UPC_A" => BarcodeType::UPC_A,
        "UPC_E" => BarcodeType::UPC_E,
        "Code128" => BarcodeType::Code128,
        "Code39" => BarcodeType::Code39,
        "QRCode" => BarcodeType::QRCode,
        _ => BarcodeType::EAN13,
    };
    
    let action = match req.action.as_str() {
        "Receive" => ScanAction::Receive,
        "Pick" => ScanAction::Pick,
        "Pack" => ScanAction::Pack,
        "Ship" => ScanAction::Ship,
        "Count" => ScanAction::Count,
        "Move" => ScanAction::Move,
        "Issue" => ScanAction::Issue,
        "Return" => ScanAction::Return,
        "Verify" => ScanAction::Verify,
        _ => ScanAction::Lookup,
    };
    
    let event = svc.scan(
        &state.pool,
        &req.barcode,
        barcode_type,
        req.scanner_id,
        req.user_id,
        req.location_id,
        action,
        req.quantity.unwrap_or(1),
        req.reference_type.as_deref(),
        req.reference_id,
    ).await?;
    
    Ok(Json(ScanEventResponse::from(event)))
}

#[derive(Serialize)]
pub struct ValidationResponse {
    pub barcode: String,
    pub barcode_type: String,
    pub is_valid: bool,
    pub validation_errors: Option<String>,
    pub check_digit: Option<String>,
    pub calculated_check_digit: Option<String>,
}

pub async fn validate_barcode(
    State(_state): State<AppState>,
    Path((code, btype)): Path<(String, String)>,
) -> ApiResult<Json<ValidationResponse>> {
    let barcode_type = match btype.as_str() {
        "EAN8" => BarcodeType::EAN8,
        "UPC_A" => BarcodeType::UPC_A,
        "UPC_E" => BarcodeType::UPC_E,
        "Code128" => BarcodeType::Code128,
        "Code39" => BarcodeType::Code39,
        "QRCode" => BarcodeType::QRCode,
        _ => BarcodeType::EAN13,
    };
    
    let validation = BarcodeValidationService::validate(&code, &barcode_type);
    
    Ok(Json(ValidationResponse {
        barcode: validation.barcode,
        barcode_type: format!("{:?}", validation.barcode_type),
        is_valid: validation.is_valid,
        validation_errors: validation.validation_errors,
        check_digit: validation.check_digit,
        calculated_check_digit: validation.calculated_check_digit,
    }))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/:code", get(get_barcode))
        .route("/", post(create_barcode))
        .route("/print-jobs", post(create_print_job))
        .route("/print-jobs/:id/complete", post(complete_print_job))
        .route("/scan", post(scan))
        .route("/validate/:code/:type", get(validate_barcode))
}
