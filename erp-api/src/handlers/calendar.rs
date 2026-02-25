use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::handlers::auth::AuthUser;
use erp_calendar::*;

#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub is_all_day: bool,
    pub calendar_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct ListEventsQuery {
    pub calendar_id: Option<Uuid>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct AddAttendeeRequest {
    pub email: String,
    pub name: Option<String>,
    pub user_id: Option<Uuid>,
    pub role: AttendeeRole,
}

#[derive(Deserialize)]
pub struct RespondRequest {
    pub status: AttendeeStatus,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct EventResponse {
    pub id: Uuid,
    pub title: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub status: EventStatus,
}

impl From<CalendarEvent> for EventResponse {
    fn from(e: CalendarEvent) -> Self {
        Self {
            id: e.base.id,
            title: e.title,
            start_at: e.start_at,
            end_at: e.end_at,
            status: e.status,
        }
    }
}

pub async fn create_event(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(req): Json<CreateEventRequest>,
) -> Result<Json<EventResponse>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = CalendarEventService::new();
    let event = service
        .create(
            &state.pool,
            req.title,
            req.description,
            req.location,
            req.start_at,
            req.end_at,
            req.is_all_day,
            user_id,
            req.calendar_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(EventResponse::from(event)))
}

pub async fn list_events(
    State(state): State<AppState>,
    Query(query): Query<ListEventsQuery>,
) -> Result<Json<Vec<EventResponse>>, StatusCode> {
    let service = CalendarEventService::new();
    let events = service
        .list(&state.pool, query.calendar_id, query.start, query.end)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(events.into_iter().map(EventResponse::from).collect()))
}

pub async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CalendarEvent>, StatusCode> {
    let service = CalendarEventService::new();
    let event = service
        .get(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(event))
}

pub async fn cancel_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = CalendarEventService::new();
    service.cancel(&state.pool, id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn add_attendee(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
    Json(req): Json<AddAttendeeRequest>,
) -> Result<Json<EventAttendee>, StatusCode> {
    let service = CalendarEventService::new();
    let attendee = service
        .add_attendee(&state.pool, event_id, req.email, req.name, req.user_id, req.role)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(attendee))
}

pub async fn respond(
    State(state): State<AppState>,
    Path(attendee_id): Path<Uuid>,
    Json(req): Json<RespondRequest>,
) -> Result<StatusCode, StatusCode> {
    let service = CalendarEventService::new();
    service.respond(&state.pool, attendee_id, req.status, req.message).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn list_attendees(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Vec<EventAttendee>>, StatusCode> {
    let service = CalendarEventService::new();
    let attendees = service
        .list_attendees(&state.pool, event_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(attendees))
}

#[derive(Deserialize)]
pub struct CreateCalendarRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}

pub async fn create_calendar(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(req): Json<CreateCalendarRequest>,
) -> Result<Json<Calendar>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = CalendarService::new();
    let calendar = service
        .create(&state.pool, req.name, req.description, req.color, user_id, false)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(calendar))
}

pub async fn list_calendars(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Calendar>>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = CalendarService::new();
    let calendars = service
        .list_for_user(&state.pool, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(calendars))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/events", axum::routing::post(create_event).get(list_events))
        .route("/events/:id", axum::routing::get(get_event))
        .route("/events/:id/cancel", axum::routing::post(cancel_event))
        .route("/events/:event_id/attendees", axum::routing::post(add_attendee).get(list_attendees))
        .route("/attendees/:attendee_id/respond", axum::routing::post(respond))
        .route("/calendars", axum::routing::post(create_calendar).get(list_calendars))
}
