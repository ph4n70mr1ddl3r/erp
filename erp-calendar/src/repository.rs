use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait CalendarEventRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, event: &CalendarEvent) -> anyhow::Result<CalendarEvent>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<CalendarEvent>>;
    async fn list(&self, pool: &SqlitePool, calendar_id: Option<Uuid>, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<CalendarEvent>>;
    async fn list_by_organizer(&self, pool: &SqlitePool, organizer_id: Uuid) -> anyhow::Result<Vec<CalendarEvent>>;
    async fn update(&self, pool: &SqlitePool, event: &CalendarEvent) -> anyhow::Result<()>;
    async fn cancel(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteCalendarEventRepository;

#[async_trait]
impl CalendarEventRepository for SqliteCalendarEventRepository {
    async fn create(&self, pool: &SqlitePool, event: &CalendarEvent) -> anyhow::Result<CalendarEvent> {
        let now = Utc::now();
        sqlx::query_as::<_, CalendarEvent>(
            r#"INSERT INTO calendar_events (
                id, title, description, location, is_virtual, virtual_meeting_url, virtual_meeting_provider,
                event_type, status, start_at, end_at, is_all_day, timezone, recurrence_pattern,
                recurrence_rule, recurrence_end_date, recurrence_count, parent_event_id, organizer_id,
                calendar_id, color, visibility, priority, capacity, current_attendees, allow_registration,
                registration_url, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(event.id)
        .bind(&event.title)
        .bind(&event.description)
        .bind(&event.location)
        .bind(event.is_virtual)
        .bind(&event.virtual_meeting_url)
        .bind(&event.virtual_meeting_provider)
        .bind(&event.event_type)
        .bind(&event.status)
        .bind(event.start_at)
        .bind(event.end_at)
        .bind(event.is_all_day)
        .bind(&event.timezone)
        .bind(&event.recurrence_pattern)
        .bind(&event.recurrence_rule)
        .bind(event.recurrence_end_date)
        .bind(event.recurrence_count)
        .bind(event.parent_event_id)
        .bind(event.organizer_id)
        .bind(event.calendar_id)
        .bind(&event.color)
        .bind(&event.visibility)
        .bind(event.priority)
        .bind(event.capacity)
        .bind(event.current_attendees)
        .bind(event.allow_registration)
        .bind(&event.registration_url)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<CalendarEvent>> {
        sqlx::query_as::<_, CalendarEvent>("SELECT * FROM calendar_events WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, calendar_id: Option<Uuid>, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<CalendarEvent>> {
        let query = if calendar_id.is_some() {
            "SELECT * FROM calendar_events WHERE calendar_id = ? AND start_at >= ? AND end_at <= ? AND status != 'Cancelled' ORDER BY start_at ASC"
        } else {
            "SELECT * FROM calendar_events WHERE start_at >= ? AND end_at <= ? AND status != 'Cancelled' ORDER BY start_at ASC"
        };
        
        let mut q = sqlx::query_as::<_, CalendarEvent>(query);
        if let Some(cid) = calendar_id { q = q.bind(cid); }
        q = q.bind(start).bind(end);
        q.fetch_all(pool).await.map_err(Into::into)
    }

    async fn list_by_organizer(&self, pool: &SqlitePool, organizer_id: Uuid) -> anyhow::Result<Vec<CalendarEvent>> {
        sqlx::query_as::<_, CalendarEvent>("SELECT * FROM calendar_events WHERE organizer_id = ? ORDER BY start_at DESC")
            .bind(organizer_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, event: &CalendarEvent) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(r#"UPDATE calendar_events SET title=?, description=?, location=?, start_at=?, end_at=?, status=?, updated_at=? WHERE id=?"#)
            .bind(&event.title)
            .bind(&event.description)
            .bind(&event.location)
            .bind(event.start_at)
            .bind(event.end_at)
            .bind(&event.status)
            .bind(now)
            .bind(event.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn cancel(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE calendar_events SET status='Cancelled', updated_at=? WHERE id=?")
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM calendar_events WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait EventAttendeeRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, attendee: &EventAttendee) -> anyhow::Result<EventAttendee>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EventAttendee>>;
    async fn list_by_event(&self, pool: &SqlitePool, event_id: Uuid) -> anyhow::Result<Vec<EventAttendee>>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: AttendeeStatus, message: Option<String>) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteEventAttendeeRepository;

#[async_trait]
impl EventAttendeeRepository for SqliteEventAttendeeRepository {
    async fn create(&self, pool: &SqlitePool, attendee: &EventAttendee) -> anyhow::Result<EventAttendee> {
        let now = Utc::now();
        sqlx::query_as::<_, EventAttendee>(
            r#"INSERT INTO event_attendees (
                id, event_id, user_id, email, name, status, role, response_message,
                responded_at, reminder_sent, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(attendee.id)
        .bind(attendee.event_id)
        .bind(attendee.user_id)
        .bind(&attendee.email)
        .bind(&attendee.name)
        .bind(&attendee.status)
        .bind(&attendee.role)
        .bind(&attendee.response_message)
        .bind(attendee.responded_at)
        .bind(attendee.reminder_sent)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EventAttendee>> {
        sqlx::query_as::<_, EventAttendee>("SELECT * FROM event_attendees WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_event(&self, pool: &SqlitePool, event_id: Uuid) -> anyhow::Result<Vec<EventAttendee>> {
        sqlx::query_as::<_, EventAttendee>("SELECT * FROM event_attendees WHERE event_id = ? ORDER BY role, name")
            .bind(event_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: AttendeeStatus, message: Option<String>) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE event_attendees SET status=?, response_message=?, responded_at=?, updated_at=? WHERE id=?")
            .bind(&status)
            .bind(&message)
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM event_attendees WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait CalendarRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, calendar: &Calendar) -> anyhow::Result<Calendar>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Calendar>>;
    async fn list_by_owner(&self, pool: &SqlitePool, owner_id: Uuid) -> anyhow::Result<Vec<Calendar>>;
    async fn update(&self, pool: &SqlitePool, calendar: &Calendar) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteCalendarRepository;

#[async_trait]
impl CalendarRepository for SqliteCalendarRepository {
    async fn create(&self, pool: &SqlitePool, calendar: &Calendar) -> anyhow::Result<Calendar> {
        let now = Utc::now();
        sqlx::query_as::<_, Calendar>(
            r#"INSERT INTO calendars (
                id, name, description, color, owner_id, is_default, is_public, timezone,
                working_hours_start, working_hours_end, working_days, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(calendar.id)
        .bind(&calendar.name)
        .bind(&calendar.description)
        .bind(&calendar.color)
        .bind(calendar.owner_id)
        .bind(calendar.is_default)
        .bind(calendar.is_public)
        .bind(&calendar.timezone)
        .bind(&calendar.working_hours_start)
        .bind(&calendar.working_hours_end)
        .bind(&calendar.working_days)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Calendar>> {
        sqlx::query_as::<_, Calendar>("SELECT * FROM calendars WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_owner(&self, pool: &SqlitePool, owner_id: Uuid) -> anyhow::Result<Vec<Calendar>> {
        sqlx::query_as::<_, Calendar>("SELECT * FROM calendars WHERE owner_id = ? ORDER BY is_default DESC, name")
            .bind(owner_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, calendar: &Calendar) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE calendars SET name=?, description=?, color=?, updated_at=? WHERE id=?")
            .bind(&calendar.name)
            .bind(&calendar.description)
            .bind(&calendar.color)
            .bind(now)
            .bind(calendar.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM calendars WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
