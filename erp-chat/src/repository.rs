use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait ChatChannelRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, channel: &ChatChannel) -> anyhow::Result<ChatChannel>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ChatChannel>>;
    async fn list(&self, pool: &SqlitePool, channel_type: Option<ChannelType>) -> anyhow::Result<Vec<ChatChannel>>;
    async fn list_for_user(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<ChatChannel>>;
    async fn update(&self, pool: &SqlitePool, channel: &ChatChannel) -> anyhow::Result<()>;
    async fn archive(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteChatChannelRepository;

#[async_trait]
impl ChatChannelRepository for SqliteChatChannelRepository {
    async fn create(&self, pool: &SqlitePool, channel: &ChatChannel) -> anyhow::Result<ChatChannel> {
        let now = Utc::now();
        sqlx::query_as::<_, ChatChannel>(
            r#"INSERT INTO chat_channels (
                id, name, description, channel_type, avatar_url, is_private, is_archived,
                owner_id, parent_channel_id, related_entity_type, related_entity_id, topic,
                slow_mode, slow_mode_delay, allow_mentions, allow_reactions, allow_threads,
                auto_join, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(channel.base.id)
        .bind(&channel.name)
        .bind(&channel.description)
        .bind(&channel.channel_type)
        .bind(&channel.avatar_url)
        .bind(channel.is_private)
        .bind(channel.is_archived)
        .bind(channel.owner_id)
        .bind(channel.parent_channel_id)
        .bind(&channel.related_entity_type)
        .bind(channel.related_entity_id)
        .bind(&channel.topic)
        .bind(channel.slow_mode)
        .bind(channel.slow_mode_delay)
        .bind(channel.allow_mentions)
        .bind(channel.allow_reactions)
        .bind(channel.allow_threads)
        .bind(channel.auto_join)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ChatChannel>> {
        sqlx::query_as::<_, ChatChannel>("SELECT * FROM chat_channels WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, channel_type: Option<ChannelType>) -> anyhow::Result<Vec<ChatChannel>> {
        let mut query = "SELECT * FROM chat_channels WHERE is_archived = 0".to_string();
        if channel_type.is_some() { query.push_str(" AND channel_type = ?"); }
        query.push_str(" ORDER BY created_at DESC");
        
        let mut q = sqlx::query_as::<_, ChatChannel>(&query);
        if let Some(ct) = channel_type { q = q.bind(ct); }
        q.fetch_all(pool).await.map_err(Into::into)
    }

    async fn list_for_user(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<ChatChannel>> {
        sqlx::query_as::<_, ChatChannel>(
            r#"SELECT c.* FROM chat_channels c
               JOIN chat_memberships m ON c.id = m.channel_id
               WHERE m.user_id = ? AND c.is_archived = 0 AND m.hidden = 0
               ORDER BY c.created_at DESC"#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, channel: &ChatChannel) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE chat_channels SET name=?, description=?, topic=?, updated_at=? WHERE id=?")
            .bind(&channel.name)
            .bind(&channel.description)
            .bind(&channel.topic)
            .bind(now)
            .bind(channel.base.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn archive(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE chat_channels SET is_archived=1, updated_at=? WHERE id=?")
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM chat_channels WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait ChatMessageRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, message: &ChatMessage) -> anyhow::Result<ChatMessage>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ChatMessage>>;
    async fn list_by_channel(&self, pool: &SqlitePool, channel_id: Uuid, limit: i32, before: Option<DateTime<Utc>>) -> anyhow::Result<Vec<ChatMessage>>;
    async fn list_replies(&self, pool: &SqlitePool, message_id: Uuid) -> anyhow::Result<Vec<ChatMessage>>;
    async fn update(&self, pool: &SqlitePool, message: &ChatMessage) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid, deleted_by: Uuid) -> anyhow::Result<()>;
    async fn pin(&self, pool: &SqlitePool, id: Uuid, pinned_by: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteChatMessageRepository;

#[async_trait]
impl ChatMessageRepository for SqliteChatMessageRepository {
    async fn create(&self, pool: &SqlitePool, message: &ChatMessage) -> anyhow::Result<ChatMessage> {
        let now = Utc::now();
        sqlx::query_as::<_, ChatMessage>(
            r#"INSERT INTO chat_messages (
                id, channel_id, sender_id, parent_message_id, thread_id, message_type,
                content, formatted_content, attachments, mentions, reactions, reply_count,
                status, edited_at, edited_by, deleted_at, deleted_by, pinned_at, pinned_by,
                starred_count, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(message.base.id)
        .bind(message.channel_id)
        .bind(message.sender_id)
        .bind(message.parent_message_id)
        .bind(message.thread_id)
        .bind(&message.message_type)
        .bind(&message.content)
        .bind(&message.formatted_content)
        .bind(&message.attachments)
        .bind(message.mentions.as_ref().and_then(|v| serde_json::to_string(v).ok()))
        .bind(&message.reactions)
        .bind(message.reply_count)
        .bind(&message.status)
        .bind(message.edited_at)
        .bind(message.edited_by)
        .bind(message.deleted_at)
        .bind(message.deleted_by)
        .bind(message.pinned_at)
        .bind(message.pinned_by)
        .bind(message.starred_count)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ChatMessage>> {
        sqlx::query_as::<_, ChatMessage>("SELECT * FROM chat_messages WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_channel(&self, pool: &SqlitePool, channel_id: Uuid, limit: i32, before: Option<DateTime<Utc>>) -> anyhow::Result<Vec<ChatMessage>> {
        let query = if before.is_some() {
            "SELECT * FROM chat_messages WHERE channel_id = ? AND deleted_at IS NULL AND created_at < ? ORDER BY created_at DESC LIMIT ?"
        } else {
            "SELECT * FROM chat_messages WHERE channel_id = ? AND deleted_at IS NULL ORDER BY created_at DESC LIMIT ?"
        };
        
        let mut q = sqlx::query_as::<_, ChatMessage>(query)
            .bind(channel_id);
        if let Some(b) = before { q = q.bind(b); }
        q = q.bind(limit);
        q.fetch_all(pool).await.map_err(Into::into)
    }

    async fn list_replies(&self, pool: &SqlitePool, message_id: Uuid) -> anyhow::Result<Vec<ChatMessage>> {
        sqlx::query_as::<_, ChatMessage>("SELECT * FROM chat_messages WHERE parent_message_id = ? AND deleted_at IS NULL ORDER BY created_at ASC")
            .bind(message_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, message: &ChatMessage) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE chat_messages SET content=?, formatted_content=?, status='Edited', edited_at=?, updated_at=? WHERE id=?")
            .bind(&message.content)
            .bind(&message.formatted_content)
            .bind(now)
            .bind(now)
            .bind(message.base.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid, deleted_by: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE chat_messages SET status='Deleted', deleted_at=?, deleted_by=?, updated_at=? WHERE id=?")
            .bind(now)
            .bind(deleted_by)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn pin(&self, pool: &SqlitePool, id: Uuid, pinned_by: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE chat_messages SET pinned_at=?, pinned_by=?, updated_at=? WHERE id=?")
            .bind(now)
            .bind(pinned_by)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait ChatMembershipRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, membership: &ChatMembership) -> anyhow::Result<ChatMembership>;
    async fn get(&self, pool: &SqlitePool, channel_id: Uuid, user_id: Uuid) -> anyhow::Result<Option<ChatMembership>>;
    async fn list_members(&self, pool: &SqlitePool, channel_id: Uuid) -> anyhow::Result<Vec<ChatMembership>>;
    async fn update(&self, pool: &SqlitePool, membership: &ChatMembership) -> anyhow::Result<()>;
    async fn update_last_read(&self, pool: &SqlitePool, channel_id: Uuid, user_id: Uuid, message_id: Uuid) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, channel_id: Uuid, user_id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteChatMembershipRepository;

#[async_trait]
impl ChatMembershipRepository for SqliteChatMembershipRepository {
    async fn create(&self, pool: &SqlitePool, membership: &ChatMembership) -> anyhow::Result<ChatMembership> {
        let now = Utc::now();
        sqlx::query_as::<_, ChatMembership>(
            r#"INSERT INTO chat_memberships (
                id, channel_id, user_id, role, nickname, muted, muted_until,
                notifications_enabled, last_read_at, last_read_message_id, unread_count,
                unread_mentions, starred, hidden, joined_at, invited_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(membership.base.id)
        .bind(membership.channel_id)
        .bind(membership.user_id)
        .bind(&membership.role)
        .bind(&membership.nickname)
        .bind(membership.muted)
        .bind(membership.muted_until)
        .bind(membership.notifications_enabled)
        .bind(membership.last_read_at)
        .bind(membership.last_read_message_id)
        .bind(membership.unread_count)
        .bind(membership.unread_mentions)
        .bind(membership.starred)
        .bind(membership.hidden)
        .bind(membership.joined_at)
        .bind(membership.invited_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get(&self, pool: &SqlitePool, channel_id: Uuid, user_id: Uuid) -> anyhow::Result<Option<ChatMembership>> {
        sqlx::query_as::<_, ChatMembership>("SELECT * FROM chat_memberships WHERE channel_id = ? AND user_id = ?")
            .bind(channel_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_members(&self, pool: &SqlitePool, channel_id: Uuid) -> anyhow::Result<Vec<ChatMembership>> {
        sqlx::query_as::<_, ChatMembership>("SELECT * FROM chat_memberships WHERE channel_id = ? ORDER BY joined_at ASC")
            .bind(channel_id)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, membership: &ChatMembership) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE chat_memberships SET role=?, muted=?, notifications_enabled=?, updated_at=? WHERE id=?")
            .bind(&membership.role)
            .bind(membership.muted)
            .bind(membership.notifications_enabled)
            .bind(now)
            .bind(membership.base.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_last_read(&self, pool: &SqlitePool, channel_id: Uuid, user_id: Uuid, message_id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE chat_memberships SET last_read_at=?, last_read_message_id=?, unread_count=0, updated_at=? WHERE channel_id=? AND user_id=?")
            .bind(now)
            .bind(message_id)
            .bind(now)
            .bind(channel_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, channel_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM chat_memberships WHERE channel_id = ? AND user_id = ?")
            .bind(channel_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
