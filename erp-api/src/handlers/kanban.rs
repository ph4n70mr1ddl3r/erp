use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiResult;
use crate::db::AppState;
use crate::handlers::auth::AuthUser;
use erp_kanban::{KanbanService, KanbanBoard, KanbanCard, KanbanCardComment, KanbanCardChecklist, KanbanActivityLog, BoardSummary, CreateBoardRequest, CreateCardRequest, MoveCardRequest, KanbanBoardType, KanbanCardType, KanbanCardPriority, KanbanSwimlaneType, CreateColumnRequest};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Serialize)]
pub struct KanbanBoardResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub board_type: String,
    pub team_id: Option<String>,
    pub project_id: Option<String>,
    pub columns: Vec<KanbanColumnResponse>,
    pub swimlane_type: String,
    pub default_wip_limit: Option<i32>,
    pub allow_card_reordering: bool,
    pub show_card_count: bool,
    pub show_wip_limits: bool,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct KanbanColumnResponse {
    pub id: String,
    pub board_id: String,
    pub name: String,
    pub position: i32,
    pub wip_limit: Option<i32>,
    pub is_done_column: bool,
    pub is_backlog: bool,
    pub color: Option<String>,
    pub created_at: String,
}

impl From<KanbanBoard> for KanbanBoardResponse {
    fn from(b: KanbanBoard) -> Self {
        Self {
            id: b.base.id.to_string(),
            name: b.name,
            description: b.description,
            board_type: format!("{:?}", b.board_type),
            team_id: b.team_id.map(|id| id.to_string()),
            project_id: b.project_id.map(|id| id.to_string()),
            columns: b.columns.into_iter().map(|c| KanbanColumnResponse {
                id: c.id.to_string(),
                board_id: c.board_id.to_string(),
                name: c.name,
                position: c.position,
                wip_limit: c.wip_limit,
                is_done_column: c.is_done_column,
                is_backlog: c.is_backlog,
                color: c.color,
                created_at: c.created_at.to_rfc3339(),
            }).collect(),
            swimlane_type: format!("{:?}", b.swimlane_type),
            default_wip_limit: b.default_wip_limit,
            allow_card_reordering: b.allow_card_reordering,
            show_card_count: b.show_card_count,
            show_wip_limits: b.show_wip_limits,
            status: format!("{:?}", b.status),
            created_at: b.created_at.to_rfc3339(),
            updated_at: b.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct KanbanCardResponse {
    pub id: String,
    pub board_id: String,
    pub column_id: String,
    pub swimlane_id: Option<String>,
    pub card_type: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub position: i32,
    pub assignee_ids: Vec<String>,
    pub reporter_id: Option<String>,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub completed_date: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub story_points: Option<i32>,
    pub tags: Vec<String>,
    pub external_ref_type: Option<String>,
    pub external_ref_id: Option<String>,
    pub blocked: bool,
    pub blocked_reason: Option<String>,
    pub parent_card_id: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<KanbanCard> for KanbanCardResponse {
    fn from(c: KanbanCard) -> Self {
        Self {
            id: c.base.id.to_string(),
            board_id: c.board_id.to_string(),
            column_id: c.column_id.to_string(),
            swimlane_id: c.swimlane_id.map(|id| id.to_string()),
            card_type: format!("{:?}", c.card_type),
            title: c.title,
            description: c.description,
            priority: format!("{:?}", c.priority),
            position: c.position,
            assignee_ids: c.assignee_ids.into_iter().map(|id| id.to_string()).collect(),
            reporter_id: c.reporter_id.map(|id| id.to_string()),
            due_date: c.due_date.map(|d| d.to_rfc3339()),
            start_date: c.start_date.map(|d| d.to_rfc3339()),
            completed_date: c.completed_date.map(|d| d.to_rfc3339()),
            estimated_hours: c.estimated_hours,
            actual_hours: c.actual_hours,
            story_points: c.story_points,
            tags: c.tags,
            external_ref_type: c.external_ref_type,
            external_ref_id: c.external_ref_id.map(|id| id.to_string()),
            blocked: c.blocked,
            blocked_reason: c.blocked_reason,
            parent_card_id: c.parent_card_id.map(|id| id.to_string()),
            status: format!("{:?}", c.status),
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct KanbanCommentResponse {
    pub id: String,
    pub card_id: String,
    pub author_id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<KanbanCardComment> for KanbanCommentResponse {
    fn from(c: KanbanCardComment) -> Self {
        Self {
            id: c.id.to_string(),
            card_id: c.card_id.to_string(),
            author_id: c.author_id.to_string(),
            content: c.content,
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.map(|d| d.to_rfc3339()),
        }
    }
}

#[derive(Serialize)]
pub struct KanbanChecklistResponse {
    pub id: String,
    pub card_id: String,
    pub title: String,
    pub position: i32,
    pub items: Vec<KanbanChecklistItemResponse>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct KanbanChecklistItemResponse {
    pub id: String,
    pub checklist_id: String,
    pub content: String,
    pub position: i32,
    pub completed: bool,
    pub completed_by: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
}

impl From<KanbanCardChecklist> for KanbanChecklistResponse {
    fn from(c: KanbanCardChecklist) -> Self {
        Self {
            id: c.id.to_string(),
            card_id: c.card_id.to_string(),
            title: c.title,
            position: c.position,
            items: c.items.into_iter().map(|i| KanbanChecklistItemResponse {
                id: i.id.to_string(),
                checklist_id: i.checklist_id.to_string(),
                content: i.content,
                position: i.position,
                completed: i.completed,
                completed_by: i.completed_by.map(|id| id.to_string()),
                completed_at: i.completed_at.map(|d| d.to_rfc3339()),
                created_at: i.created_at.to_rfc3339(),
            }).collect(),
            created_at: c.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct KanbanActivityResponse {
    pub id: String,
    pub board_id: String,
    pub card_id: Option<String>,
    pub action_type: String,
    pub actor_id: String,
    pub description: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub created_at: String,
}

impl From<KanbanActivityLog> for KanbanActivityResponse {
    fn from(a: KanbanActivityLog) -> Self {
        Self {
            id: a.id.to_string(),
            board_id: a.board_id.to_string(),
            card_id: a.card_id.map(|id| id.to_string()),
            action_type: format!("{:?}", a.action_type),
            actor_id: a.actor_id.to_string(),
            description: a.description,
            old_value: a.old_value,
            new_value: a.new_value,
            created_at: a.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct BoardSummaryResponse {
    pub board_id: String,
    pub board_name: String,
    pub total_cards: i32,
    pub cards_by_column: Vec<ColumnSummaryResponse>,
    pub wip_violations: i32,
    pub overdue_cards: i32,
    pub blocked_cards: i32,
}

#[derive(Serialize)]
pub struct ColumnSummaryResponse {
    pub column_id: String,
    pub column_name: String,
    pub card_count: i32,
    pub wip_limit: Option<i32>,
    pub is_over_wip: bool,
}

impl From<BoardSummary> for BoardSummaryResponse {
    fn from(s: BoardSummary) -> Self {
        Self {
            board_id: s.board_id.to_string(),
            board_name: s.board_name,
            total_cards: s.total_cards,
            cards_by_column: s.cards_by_column.into_iter().map(|c| ColumnSummaryResponse {
                column_id: c.column_id.to_string(),
                column_name: c.column_name,
                card_count: c.card_count,
                wip_limit: c.wip_limit,
                is_over_wip: c.is_over_wip,
            }).collect(),
            wip_violations: s.wip_violations,
            overdue_cards: s.overdue_cards,
            blocked_cards: s.blocked_cards,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateBoardRequestDto {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub board_type: String,
    pub team_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub columns: Vec<CreateColumnRequestDto>,
    pub default_wip_limit: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateColumnRequestDto {
    pub name: String,
    pub wip_limit: Option<i32>,
    #[serde(default)]
    pub is_done_column: bool,
    #[serde(default)]
    pub is_backlog: bool,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateCardRequestDto {
    pub board_id: Uuid,
    pub column_id: Uuid,
    pub swimlane_id: Option<Uuid>,
    #[serde(default)]
    pub card_type: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub priority: String,
    pub assignee_ids: Vec<Uuid>,
    pub due_date: Option<String>,
    pub estimated_hours: Option<f64>,
    pub story_points: Option<i32>,
    pub tags: Vec<String>,
    pub external_ref_type: Option<String>,
    pub external_ref_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct MoveCardRequestDto {
    pub card_id: Uuid,
    pub to_column_id: Uuid,
    pub to_position: i32,
}

#[derive(Deserialize)]
pub struct AddCommentRequestDto {
    pub content: String,
}

#[derive(Deserialize)]
pub struct AddChecklistRequestDto {
    pub title: String,
}

#[derive(Deserialize)]
pub struct BlockCardRequestDto {
    pub reason: String,
}

pub async fn create_board(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<CreateBoardRequestDto>,
) -> ApiResult<Json<KanbanBoardResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let board_type = match req.board_type.as_str() {
        "Production" => KanbanBoardType::Production,
        "Purchase" => KanbanBoardType::Purchase,
        "Sales" => KanbanBoardType::Sales,
        "Support" => KanbanBoardType::Support,
        "Maintenance" => KanbanBoardType::Maintenance,
        "Project" => KanbanBoardType::Project,
        _ => KanbanBoardType::Generic,
    };

    let columns: Vec<CreateColumnRequest> = req.columns.into_iter().map(|c| CreateColumnRequest {
        name: c.name,
        wip_limit: c.wip_limit,
        is_done_column: c.is_done_column,
        is_backlog: c.is_backlog,
        color: c.color,
    }).collect();

    let svc = KanbanService::new();
    let board = svc.create_board(&state.pool, CreateBoardRequest {
        name: req.name,
        description: req.description,
        board_type,
        team_id: req.team_id,
        project_id: req.project_id,
        columns,
        default_wip_limit: req.default_wip_limit,
    }, user_id).await?;
    
    Ok(Json(KanbanBoardResponse::from(board)))
}

pub async fn get_board(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<KanbanBoardResponse>> {
    let svc = KanbanService::new();
    let board = svc.get_board(&state.pool, id).await?
        .ok_or_else(|| anyhow::anyhow!("Board not found"))?;
    Ok(Json(KanbanBoardResponse::from(board)))
}

pub async fn list_boards(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<ApiResponse<Vec<KanbanBoardResponse>>>> {
    let svc = KanbanService::new();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let boards = svc.list_boards(&state.pool, page, limit).await?;
    let items: Vec<KanbanBoardResponse> = boards.into_iter().map(KanbanBoardResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn delete_board(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let svc = KanbanService::new();
    svc.delete_board(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_card(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<CreateCardRequestDto>,
) -> ApiResult<Json<KanbanCardResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let card_type = match req.card_type.as_str() {
        "Defect" => KanbanCardType::Defect,
        "Story" => KanbanCardType::Story,
        "Epic" => KanbanCardType::Epic,
        "ProductionOrder" => KanbanCardType::ProductionOrder,
        "PurchaseRequest" => KanbanCardType::PurchaseRequest,
        "MaintenanceRequest" => KanbanCardType::MaintenanceRequest,
        "Custom" => KanbanCardType::Custom,
        _ => KanbanCardType::Task,
    };

    let priority = match req.priority.as_str() {
        "Lowest" => KanbanCardPriority::Lowest,
        "Low" => KanbanCardPriority::Low,
        "High" => KanbanCardPriority::High,
        "Highest" => KanbanCardPriority::Highest,
        "Critical" => KanbanCardPriority::Critical,
        _ => KanbanCardPriority::Medium,
    };

    let due_date = req.due_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
        .map(|d| d.with_timezone(&chrono::Utc));

    let svc = KanbanService::new();
    let card = svc.create_card(&state.pool, CreateCardRequest {
        board_id: req.board_id,
        column_id: req.column_id,
        swimlane_id: req.swimlane_id,
        card_type,
        title: req.title,
        description: req.description,
        priority,
        assignee_ids: req.assignee_ids,
        due_date,
        estimated_hours: req.estimated_hours,
        story_points: req.story_points,
        tags: req.tags,
        external_ref_type: req.external_ref_type,
        external_ref_id: req.external_ref_id,
    }, user_id).await?;
    
    Ok(Json(KanbanCardResponse::from(card)))
}

pub async fn get_card(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<KanbanCardResponse>> {
    let svc = KanbanService::new();
    let card = svc.get_card(&state.pool, id).await?
        .ok_or_else(|| anyhow::anyhow!("Card not found"))?;
    Ok(Json(KanbanCardResponse::from(card)))
}

pub async fn list_cards(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<KanbanCardResponse>>>> {
    let svc = KanbanService::new();
    let cards = svc.list_cards_by_board(&state.pool, board_id).await?;
    let items: Vec<KanbanCardResponse> = cards.into_iter().map(KanbanCardResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn move_card(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<MoveCardRequestDto>,
) -> ApiResult<Json<KanbanCardResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = KanbanService::new();
    let card = svc.move_card(&state.pool, MoveCardRequest {
        card_id: req.card_id,
        to_column_id: req.to_column_id,
        to_position: req.to_position,
    }, user_id).await?;
    
    Ok(Json(KanbanCardResponse::from(card)))
}

pub async fn delete_card(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = KanbanService::new();
    svc.delete_card(&state.pool, id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn block_card(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<BlockCardRequestDto>,
) -> ApiResult<Json<KanbanCardResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = KanbanService::new();
    let card = svc.block_card(&state.pool, id, req.reason, user_id).await?;
    Ok(Json(KanbanCardResponse::from(card)))
}

pub async fn unblock_card(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<KanbanCardResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = KanbanService::new();
    let card = svc.unblock_card(&state.pool, id, user_id).await?;
    Ok(Json(KanbanCardResponse::from(card)))
}

pub async fn add_comment(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(card_id): Path<Uuid>,
    Json(req): Json<AddCommentRequestDto>,
) -> ApiResult<Json<KanbanCommentResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = KanbanService::new();
    let comment = svc.add_comment(&state.pool, card_id, req.content, user_id).await?;
    Ok(Json(KanbanCommentResponse::from(comment)))
}

pub async fn list_comments(
    State(state): State<AppState>,
    Path(card_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<KanbanCommentResponse>>>> {
    let svc = KanbanService::new();
    let comments = svc.list_comments(&state.pool, card_id).await?;
    let items: Vec<KanbanCommentResponse> = comments.into_iter().map(KanbanCommentResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn add_checklist(
    State(state): State<AppState>,
    Path(card_id): Path<Uuid>,
    Json(req): Json<AddChecklistRequestDto>,
) -> ApiResult<Json<KanbanChecklistResponse>> {
    let svc = KanbanService::new();
    let checklist = svc.add_checklist(&state.pool, card_id, req.title).await?;
    Ok(Json(KanbanChecklistResponse::from(checklist)))
}

pub async fn list_checklists(
    State(state): State<AppState>,
    Path(card_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<KanbanChecklistResponse>>>> {
    let svc = KanbanService::new();
    let checklists = svc.list_checklists(&state.pool, card_id).await?;
    let items: Vec<KanbanChecklistResponse> = checklists.into_iter().map(KanbanChecklistResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn list_activities(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<ApiResponse<Vec<KanbanActivityResponse>>>> {
    let svc = KanbanService::new();
    let limit = query.limit.unwrap_or(50);
    let activities = svc.list_activities(&state.pool, board_id, limit).await?;
    let items: Vec<KanbanActivityResponse> = activities.into_iter().map(KanbanActivityResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn get_board_summary(
    State(state): State<AppState>,
    Path(board_id): Path<Uuid>,
) -> ApiResult<Json<BoardSummaryResponse>> {
    let svc = KanbanService::new();
    let summary = svc.get_board_summary(&state.pool, board_id).await?;
    Ok(Json(BoardSummaryResponse::from(summary)))
}
