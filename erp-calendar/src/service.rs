use chrono::{DateTime, Utc};

use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

#[allow(dead_code)]
pub struct CalendarEventService {
    event_repo: SqliteCalendarEventRepository,
    attendee_repo: SqliteEventAttendeeRepository,
    calendar_repo: SqliteCalendarRepository,
}

impl Default for CalendarEventService {
    fn default() -> Self {
        Self::new()
    }
}

impl CalendarEventService {
    pub fn new() -> Self {
        Self {
            event_repo: SqliteCalendarEventRepository,
            attendee_repo: SqliteEventAttendeeRepository,
            calendar_repo: SqliteCalendarRepository,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        pool: &SqlitePool,
        title: String,
        description: Option<String>,
        location: Option<String>,
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        is_all_day: bool,
        organizer_id: Uuid,
        calendar_id: Option<Uuid>,
    ) -> anyhow::Result<CalendarEvent> {
        let event = CalendarEvent {
            id: Uuid::new_v4(),
            title,
            description,
            location,
            is_virtual: false,
            virtual_meeting_url: None,
            virtual_meeting_provider: None,
            event_type: EventType::Meeting,
            status: EventStatus::Confirmed,
            start_at,
            end_at,
            is_all_day,
            timezone: "UTC".to_string(),
            recurrence_pattern: RecurrencePattern::None,
            recurrence_rule: None,
            recurrence_end_date: None,
            recurrence_count: None,
            parent_event_id: None,
            organizer_id,
            calendar_id,
            color: None,
            visibility: EventVisibility::Private,
            priority: 5,
            capacity: None,
            current_attendees: 1,
            allow_registration: false,
            registration_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.event_repo.create(pool, &event).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<CalendarEvent>> {
        self.event_repo.get_by_id(pool, id).await
    }

    pub async fn list(&self, pool: &SqlitePool, calendar_id: Option<Uuid>, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<CalendarEvent>> {
        self.event_repo.list(pool, calendar_id, start, end).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(&self, pool: &SqlitePool, id: Uuid, title: Option<String>, description: Option<String>, location: Option<String>, start_at: Option<DateTime<Utc>>, end_at: Option<DateTime<Utc>>) -> anyhow::Result<()> {
        if let Some(mut event) = self.event_repo.get_by_id(pool, id).await? {
            if let Some(t) = title { event.title = t; }
            if let Some(d) = description { event.description = Some(d); }
            if let Some(l) = location { event.location = Some(l); }
            if let Some(s) = start_at { event.start_at = s; }
            if let Some(e) = end_at { event.end_at = e; }
            self.event_repo.update(pool, &event).await?;
        }
        Ok(())
    }

    pub async fn cancel(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.event_repo.cancel(pool, id).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.event_repo.delete(pool, id).await
    }

    pub async fn add_attendee(
        &self,
        pool: &SqlitePool,
        event_id: Uuid,
        email: String,
        name: Option<String>,
        user_id: Option<Uuid>,
        role: AttendeeRole,
    ) -> anyhow::Result<EventAttendee> {
        let attendee = EventAttendee {
            id: Uuid::new_v4(),
            event_id,
            user_id,
            email,
            name,
            status: AttendeeStatus::NeedsAction,
            role,
            response_message: None,
            responded_at: None,
            reminder_sent: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.attendee_repo.create(pool, &attendee).await
    }

    pub async fn respond(&self, pool: &SqlitePool, attendee_id: Uuid, status: AttendeeStatus, message: Option<String>) -> anyhow::Result<()> {
        self.attendee_repo.update_status(pool, attendee_id, status, message).await
    }

    pub async fn list_attendees(&self, pool: &SqlitePool, event_id: Uuid) -> anyhow::Result<Vec<EventAttendee>> {
        self.attendee_repo.list_by_event(pool, event_id).await
    }

    pub async fn remove_attendee(&self, pool: &SqlitePool, attendee_id: Uuid) -> anyhow::Result<()> {
        self.attendee_repo.delete(pool, attendee_id).await
    }

    pub async fn check_conflicts(&self, pool: &SqlitePool, user_id: Uuid, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<CalendarEvent>> {
        sqlx::query_as::<_, CalendarEvent>(
            r#"SELECT e.* FROM calendar_events e
               JOIN event_attendees a ON e.id = a.event_id
               WHERE a.user_id = ? AND e.status != 'Cancelled'
               AND ((e.start_at <= ? AND e.end_at >= ?) OR (e.start_at < ? AND e.end_at > ?))
               ORDER BY e.start_at"#
        )
        .bind(user_id)
        .bind(end)
        .bind(start)
        .bind(end)
        .bind(start)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }
}

pub struct CalendarService {
    calendar_repo: SqliteCalendarRepository,
}

impl Default for CalendarService {
    fn default() -> Self {
        Self::new()
    }
}

impl CalendarService {
    pub fn new() -> Self {
        Self {
            calendar_repo: SqliteCalendarRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        color: String,
        owner_id: Uuid,
        is_default: bool,
    ) -> anyhow::Result<Calendar> {
        let calendar = Calendar {
            id: Uuid::new_v4(),
            name,
            description,
            color,
            owner_id,
            is_default,
            is_public: false,
            timezone: "UTC".to_string(),
            working_hours_start: Some("09:00".to_string()),
            working_hours_end: Some("17:00".to_string()),
            working_days: Some("1,2,3,4,5".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.calendar_repo.create(pool, &calendar).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Calendar>> {
        self.calendar_repo.get_by_id(pool, id).await
    }

    pub async fn list_for_user(&self, pool: &SqlitePool, owner_id: Uuid) -> anyhow::Result<Vec<Calendar>> {
        self.calendar_repo.list_by_owner(pool, owner_id).await
    }

    pub async fn update(&self, pool: &SqlitePool, id: Uuid, name: Option<String>, description: Option<String>, color: Option<String>) -> anyhow::Result<()> {
        if let Some(mut calendar) = self.calendar_repo.get_by_id(pool, id).await? {
            if let Some(n) = name { calendar.name = n; }
            if let Some(d) = description { calendar.description = Some(d); }
            if let Some(c) = color { calendar.color = c; }
            self.calendar_repo.update(pool, &calendar).await?;
        }
        Ok(())
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.calendar_repo.delete(pool, id).await
    }
}

pub struct EventReminderService;

impl Default for EventReminderService {
    fn default() -> Self {
        Self::new()
    }
}

impl EventReminderService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        event_id: Uuid,
        user_id: Uuid,
        reminder_type: ReminderType,
        minutes_before: i32,
    ) -> anyhow::Result<EventReminder> {
        let now = Utc::now();
        sqlx::query_as::<_, EventReminder>(
            r#"INSERT INTO event_reminders (
                id, event_id, user_id, reminder_type, minutes_before, sent_at, snoozed_until, active, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(Uuid::new_v4())
        .bind(event_id)
        .bind(user_id)
        .bind(&reminder_type)
        .bind(minutes_before)
        .bind(None::<DateTime<Utc>>)
        .bind(None::<DateTime<Utc>>)
        .bind(true)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn list_for_event(&self, pool: &SqlitePool, event_id: Uuid) -> anyhow::Result<Vec<EventReminder>> {
        sqlx::query_as::<_, EventReminder>("SELECT * FROM event_reminders WHERE event_id = ? AND active = 1")
            .bind(event_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    pub async fn dismiss(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE event_reminders SET active = 0, updated_at = ? WHERE id = ?")
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn snooze(&self, pool: &SqlitePool, id: Uuid, snooze_until: DateTime<Utc>) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE event_reminders SET snoozed_until = ?, updated_at = ? WHERE id = ?")
            .bind(snooze_until)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
