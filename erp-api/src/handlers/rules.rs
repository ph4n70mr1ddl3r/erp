use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::BaseEntity;
use erp_rules::{
    RulesService, BusinessRule, RuleSet, RuleSetMember, RuleExecution,
    RuleVariable, RuleFunction, RuleTemplate, DecisionTable, DecisionTableRow,
    RuleType, RuleStatus, ExecutionMode, VariableSource, HitPolicy,
};

#[derive(Deserialize)]
pub struct CreateRuleRequest {
    pub name: String,
    pub code: String,
    pub entity_type: String,
    pub conditions: String,
    pub actions: String,
    pub rule_type: Option<String>,
}

#[derive(Serialize)]
pub struct RuleResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub entity_type: String,
    pub status: String,
}

pub async fn list_rules(
    State(state): State<AppState>,
    Query(query): Query<EntityTypeQuery>,
) -> ApiResult<Json<Vec<RuleResponse>>> {
    let service = RulesService::new();
    let rules = service.list_rules(&state.pool, query.entity_type.as_deref()).await?;
    Ok(Json(rules.into_iter().map(|r| RuleResponse {
        id: r.base.id,
        name: r.name,
        code: r.code,
        entity_type: r.entity_type,
        status: format!("{:?}", r.status),
    }).collect()))
}

#[derive(Deserialize)]
pub struct EntityTypeQuery {
    pub entity_type: Option<String>,
}

pub async fn create_rule(
    State(state): State<AppState>,
    Json(req): Json<CreateRuleRequest>,
) -> ApiResult<Json<RuleResponse>> {
    let service = RulesService::new();
    let rule_type = match req.rule_type.as_deref() {
        Some("Calculation") => RuleType::Calculation,
        Some("Transformation") => RuleType::Transformation,
        Some("Routing") => RuleType::Routing,
        Some("Approval") => RuleType::Approval,
        Some("Notification") => RuleType::Notification,
        Some("Discount") => RuleType::Discount,
        Some("Pricing") => RuleType::Pricing,
        Some("Workflow") => RuleType::Workflow,
        Some("BusinessConstraint") => RuleType::BusinessConstraint,
        _ => RuleType::Validation,
    };
    let rule = BusinessRule {
        base: BaseEntity::new(),
        name: req.name,
        code: req.code,
        description: None,
        rule_type,
        entity_type: req.entity_type,
        status: RuleStatus::Active,
        priority: 100,
        version: 1,
        effective_from: None,
        effective_to: None,
        conditions: req.conditions,
        actions: req.actions,
        else_actions: None,
        tags: None,
        owner_id: None,
        created_by: None,
    };
    let created = service.create_rule(&state.pool, rule).await?;
    Ok(Json(RuleResponse {
        id: created.base.id,
        name: created.name,
        code: created.code,
        entity_type: created.entity_type,
        status: format!("{:?}", created.status),
    }))
}

pub async fn get_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<RuleResponse>> {
    let service = RulesService::new();
    let rule = service.get_rule(&state.pool, id).await?;
    Ok(Json(RuleResponse {
        id: rule.base.id,
        name: rule.name,
        code: rule.code,
        entity_type: rule.entity_type,
        status: format!("{:?}", rule.status),
    }))
}

pub async fn delete_rule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = RulesService::new();
    service.delete_rule(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

#[derive(Deserialize)]
pub struct ExecuteRulesRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub context: serde_json::Value,
}

#[derive(Serialize)]
pub struct ExecutionResponse {
    pub rule_id: Uuid,
    pub matched: bool,
    pub actions_executed: String,
}

pub async fn execute_rules(
    State(state): State<AppState>,
    Json(req): Json<ExecuteRulesRequest>,
) -> ApiResult<Json<Vec<ExecutionResponse>>> {
    let service = RulesService::new();
    let executions = service.execute_rules(&state.pool, &req.entity_type, req.entity_id, req.context).await?;
    Ok(Json(executions.into_iter().map(|e| ExecutionResponse {
        rule_id: e.rule_id,
        matched: e.matched,
        actions_executed: e.actions_executed,
    }).collect()))
}

#[derive(Deserialize)]
pub struct CreateRulesetRequest {
    pub name: String,
    pub code: String,
    pub entity_type: String,
    pub execution_mode: Option<String>,
}

#[derive(Serialize)]
pub struct RulesetResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub entity_type: String,
}

pub async fn list_rulesets(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<RulesetResponse>>> {
    let service = RulesService::new();
    let rulesets = service.list_rulesets(&state.pool).await?;
    Ok(Json(rulesets.into_iter().map(|r| RulesetResponse {
        id: r.base.id,
        name: r.name,
        code: r.code,
        entity_type: r.entity_type,
    }).collect()))
}

pub async fn create_ruleset(
    State(state): State<AppState>,
    Json(req): Json<CreateRulesetRequest>,
) -> ApiResult<Json<RulesetResponse>> {
    let service = RulesService::new();
    let execution_mode = match req.execution_mode.as_deref() {
        Some("Parallel") => ExecutionMode::Parallel,
        Some("FirstMatch") => ExecutionMode::FirstMatch,
        Some("AllMatches") => ExecutionMode::AllMatches,
        _ => ExecutionMode::Sequential,
    };
    let ruleset = RuleSet {
        base: BaseEntity::new(),
        name: req.name,
        code: req.code,
        description: None,
        entity_type: req.entity_type,
        status: RuleStatus::Active,
        version: 1,
        effective_from: None,
        effective_to: None,
        execution_mode,
    };
    let created = service.create_ruleset(&state.pool, ruleset).await?;
    Ok(Json(RulesetResponse {
        id: created.base.id,
        name: created.name,
        code: created.code,
        entity_type: created.entity_type,
    }))
}

#[derive(Deserialize)]
pub struct AddRuleToRulesetRequest {
    pub ruleset_id: Uuid,
    pub rule_id: Uuid,
    pub sort_order: i32,
}

pub async fn add_rule_to_ruleset(
    State(state): State<AppState>,
    Json(req): Json<AddRuleToRulesetRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = RulesService::new();
    service.add_rule_to_ruleset(&state.pool, req.ruleset_id, req.rule_id, req.sort_order).await?;
    Ok(Json(serde_json::json!({ "status": "added" })))
}

#[derive(Deserialize)]
pub struct CreateDecisionTableRequest {
    pub name: String,
    pub code: String,
    pub entity_type: String,
    pub input_columns: String,
    pub output_columns: String,
}

#[derive(Serialize)]
pub struct DecisionTableResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub entity_type: String,
}

pub async fn create_decision_table(
    State(state): State<AppState>,
    Json(req): Json<CreateDecisionTableRequest>,
) -> ApiResult<Json<DecisionTableResponse>> {
    let service = RulesService::new();
    let table = DecisionTable {
        base: BaseEntity::new(),
        name: req.name,
        code: req.code,
        description: None,
        entity_type: req.entity_type,
        input_columns: req.input_columns,
        output_columns: req.output_columns,
        hit_policy: HitPolicy::First,
        status: RuleStatus::Active,
        version: 1,
    };
    let created = service.create_decision_table(&state.pool, table).await?;
    Ok(Json(DecisionTableResponse {
        id: created.base.id,
        name: created.name,
        code: created.code,
        entity_type: created.entity_type,
    }))
}

#[derive(Deserialize)]
pub struct AddDecisionRowRequest {
    pub table_id: Uuid,
    pub row_number: i32,
    pub inputs: String,
    pub outputs: String,
}

pub async fn add_decision_row(
    State(state): State<AppState>,
    Json(req): Json<AddDecisionRowRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = RulesService::new();
    service.add_decision_row(&state.pool, req.table_id, req.row_number, req.inputs, req.outputs).await?;
    Ok(Json(serde_json::json!({ "status": "added" })))
}

#[derive(Deserialize)]
pub struct EvaluateTableRequest {
    pub table_id: Uuid,
    pub inputs: serde_json::Value,
}

pub async fn evaluate_table(
    State(state): State<AppState>,
    Json(req): Json<EvaluateTableRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = RulesService::new();
    let result = service.evaluate_decision_table(&state.pool, req.table_id, req.inputs).await?;
    Ok(Json(serde_json::json!({
        "result": result
    })))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/rules", axum::routing::get(list_rules).post(create_rule))
        .route("/rules/:id", axum::routing::get(get_rule).delete(delete_rule))
        .route("/rules/execute", axum::routing::post(execute_rules))
        .route("/rulesets", axum::routing::get(list_rulesets).post(create_ruleset))
        .route("/rulesets/add-rule", axum::routing::post(add_rule_to_ruleset))
        .route("/decision-tables", axum::routing::post(create_decision_table))
        .route("/decision-tables/rows", axum::routing::post(add_decision_row))
        .route("/decision-tables/evaluate", axum::routing::post(evaluate_table))
}
