use chrono::{DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EventType {
    Meeting,
    Appointment,
    Task,
    Reminder,
    AllDay,
    Recurring,
    Holiday,
    Blocked,
    OutOfOffice,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EventStatus {
    Tentative,
    Confirmed,
    Cancelled,
    Completed,
    Postponed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RecurrencePattern {
    None,
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AttendeeStatus {
    NeedsAction,
    Accepted,
    Declined,
    Tentative,
    Delegated,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReminderType {
    Email,
    Notification,
    SMS,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub is_virtual: bool,
    pub virtual_meeting_url: Option<String>,
    pub virtual_meeting_provider: Option<String>,
    pub event_type: EventType,
    pub status: EventStatus,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub is_all_day: bool,
    pub timezone: String,
    pub recurrence_pattern: RecurrencePattern,
    pub recurrence_rule: Option<String>,
    pub recurrence_end_date: Option<NaiveDate>,
    pub recurrence_count: Option<i32>,
    pub parent_event_id: Option<Uuid>,
    pub organizer_id: Uuid,
    pub calendar_id: Option<Uuid>,
    pub color: Option<String>,
    pub visibility: EventVisibility,
    pub priority: i32,
    pub capacity: Option<i32>,
    pub current_attendees: i32,
    pub allow_registration: bool,
    pub registration_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EventVisibility {
    Public,
    Private,
    Confidential,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventAttendee {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Option<Uuid>,
    pub email: String,
    pub name: Option<String>,
    pub status: AttendeeStatus,
    pub role: AttendeeRole,
    pub response_message: Option<String>,
    pub responded_at: Option<DateTime<Utc>>,
    pub reminder_sent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AttendeeRole {
    Organizer,
    Required,
    Optional,
    Resource,
    NonParticipant,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EventReminder {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub reminder_type: ReminderType,
    pub minutes_before: i32,
    pub sent_at: Option<DateTime<Utc>>,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Calendar {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub owner_id: Uuid,
    pub is_default: bool,
    pub is_public: bool,
    pub timezone: String,
    pub working_hours_start: Option<String>,
    pub working_hours_end: Option<String>,
    pub working_days: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarShare {
    pub id: Uuid,
    pub calendar_id: Uuid,
    pub shared_with_user_id: Option<Uuid>,
    pub shared_with_email: Option<String>,
    pub permission: CalendarPermission,
    pub share_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CalendarPermission {
    ViewOnly,
    ViewBusyOnly,
    Edit,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResource {
    pub id: Uuid,
    pub name: String,
    pub resource_type: ResourceType,
    pub location: Option<String>,
    pub capacity: i32,
    pub email: Option<String>,
    pub calendar_id: Option<Uuid>,
    pub available: bool,
    pub booking_enabled: bool,
    pub auto_accept: bool,
    pub approval_required: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ResourceType {
    Room,
    Equipment,
    Vehicle,
    Catering,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBooking {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub event_id: Uuid,
    pub booked_by: Uuid,
    pub status: BookingStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BookingStatus {
    Pending,
    Confirmed,
    Cancelled,
    Rejected,
    Completed,
}
