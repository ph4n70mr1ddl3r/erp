use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

#[allow(dead_code)]
pub struct ChatChannelService {
    channel_repo: SqliteChatChannelRepository,
    membership_repo: SqliteChatMembershipRepository,
    message_repo: SqliteChatMessageRepository,
}

impl Default for ChatChannelService {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatChannelService {
    pub fn new() -> Self {
        Self {
            channel_repo: SqliteChatChannelRepository,
            membership_repo: SqliteChatMembershipRepository,
            message_repo: SqliteChatMessageRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        channel_type: ChannelType,
        is_private: bool,
        owner_id: Uuid,
    ) -> anyhow::Result<ChatChannel> {
        let channel = ChatChannel {
            base: BaseEntity::new(),
            name,
            description,
            channel_type,
            avatar_url: None,
            is_private,
            is_archived: false,
            owner_id,
            parent_channel_id: None,
            related_entity_type: None,
            related_entity_id: None,
            topic: None,
            slow_mode: false,
            slow_mode_delay: None,
            allow_mentions: true,
            allow_reactions: true,
            allow_threads: true,
            auto_join: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let channel = self.channel_repo.create(pool, &channel).await?;
        
        let membership = ChatMembership {
            base: BaseEntity::new(),
            channel_id: channel.base.id,
            user_id: owner_id,
            role: MembershipRole::Owner,
            nickname: None,
            muted: false,
            muted_until: None,
            notifications_enabled: true,
            last_read_at: None,
            last_read_message_id: None,
            unread_count: 0,
            unread_mentions: 0,
            starred: false,
            hidden: false,
            joined_at: Utc::now(),
            invited_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.membership_repo.create(pool, &membership).await?;
        
        Ok(channel)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ChatChannel>> {
        self.channel_repo.get_by_id(pool, id).await
    }

    pub async fn list(&self, pool: &SqlitePool, channel_type: Option<ChannelType>) -> anyhow::Result<Vec<ChatChannel>> {
        self.channel_repo.list(pool, channel_type).await
    }

    pub async fn list_for_user(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<ChatChannel>> {
        self.channel_repo.list_for_user(pool, user_id).await
    }

    pub async fn join(&self, pool: &SqlitePool, channel_id: Uuid, user_id: Uuid) -> anyhow::Result<ChatMembership> {
        let membership = ChatMembership {
            base: BaseEntity::new(),
            channel_id,
            user_id,
            role: MembershipRole::Member,
            nickname: None,
            muted: false,
            muted_until: None,
            notifications_enabled: true,
            last_read_at: None,
            last_read_message_id: None,
            unread_count: 0,
            unread_mentions: 0,
            starred: false,
            hidden: false,
            joined_at: Utc::now(),
            invited_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.membership_repo.create(pool, &membership).await
    }

    pub async fn leave(&self, pool: &SqlitePool, channel_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        self.membership_repo.delete(pool, channel_id, user_id).await
    }

    pub async fn archive(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.channel_repo.archive(pool, id).await
    }
}

pub struct ChatMessageService {
    message_repo: SqliteChatMessageRepository,
    membership_repo: SqliteChatMembershipRepository,
}

impl Default for ChatMessageService {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatMessageService {
    pub fn new() -> Self {
        Self {
            message_repo: SqliteChatMessageRepository,
            membership_repo: SqliteChatMembershipRepository,
        }
    }

    pub async fn send(
        &self,
        pool: &SqlitePool,
        channel_id: Uuid,
        sender_id: Uuid,
        content: String,
        parent_message_id: Option<Uuid>,
    ) -> anyhow::Result<ChatMessage> {
        let message = ChatMessage {
            base: BaseEntity::new(),
            channel_id,
            sender_id,
            parent_message_id,
            thread_id: None,
            message_type: MessageType::Text,
            content,
            formatted_content: None,
            attachments: None,
            mentions: None,
            reactions: None,
            reply_count: 0,
            status: MessageStatus::Sent,
            edited_at: None,
            edited_by: None,
            deleted_at: None,
            deleted_by: None,
            pinned_at: None,
            pinned_by: None,
            starred_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let message = self.message_repo.create(pool, &message).await?;
        
        if let Some(parent_id) = parent_message_id {
            if self.message_repo.get_by_id(pool, parent_id).await?.is_some() {
                if let Some(_membership) = self.membership_repo.get(pool, channel_id, sender_id).await? {
                    self.membership_repo.update_last_read(pool, channel_id, sender_id, message.base.id).await?;
                }
            }
        }
        
        Ok(message)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ChatMessage>> {
        self.message_repo.get_by_id(pool, id).await
    }

    pub async fn list_by_channel(&self, pool: &SqlitePool, channel_id: Uuid, limit: i32, before: Option<chrono::DateTime<Utc>>) -> anyhow::Result<Vec<ChatMessage>> {
        self.message_repo.list_by_channel(pool, channel_id, limit, before).await
    }

    pub async fn edit(&self, pool: &SqlitePool, id: Uuid, content: String) -> anyhow::Result<()> {
        if let Some(mut message) = self.message_repo.get_by_id(pool, id).await? {
            message.content = content;
            self.message_repo.update(pool, &message).await?;
        }
        Ok(())
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid, deleted_by: Uuid) -> anyhow::Result<()> {
        self.message_repo.delete(pool, id, deleted_by).await
    }

    pub async fn pin(&self, pool: &SqlitePool, id: Uuid, pinned_by: Uuid) -> anyhow::Result<()> {
        self.message_repo.pin(pool, id, pinned_by).await
    }

    pub async fn get_replies(&self, pool: &SqlitePool, message_id: Uuid) -> anyhow::Result<Vec<ChatMessage>> {
        self.message_repo.list_replies(pool, message_id).await
    }
}

pub struct DirectMessageService;

impl Default for DirectMessageService {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectMessageService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send(
        &self,
        pool: &SqlitePool,
        sender_id: Uuid,
        recipient_id: Uuid,
        content: String,
    ) -> anyhow::Result<DirectMessage> {
        let dm = DirectMessage {
            base: BaseEntity::new(),
            sender_id,
            recipient_id,
            message_type: MessageType::Text,
            content,
            formatted_content: None,
            attachments: None,
            status: MessageStatus::Sent,
            read_at: None,
            edited_at: None,
            deleted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let now = Utc::now();
        sqlx::query_as::<_, DirectMessage>(
            r#"INSERT INTO direct_messages (
                id, sender_id, recipient_id, message_type, content, formatted_content,
                attachments, status, read_at, edited_at, deleted_at, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(dm.base.id)
        .bind(dm.sender_id)
        .bind(dm.recipient_id)
        .bind(&dm.message_type)
        .bind(&dm.content)
        .bind(&dm.formatted_content)
        .bind(&dm.attachments)
        .bind(&dm.status)
        .bind(dm.read_at)
        .bind(dm.edited_at)
        .bind(dm.deleted_at)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn list_conversation(&self, pool: &SqlitePool, user1: Uuid, user2: Uuid, limit: i32) -> anyhow::Result<Vec<DirectMessage>> {
        sqlx::query_as::<_, DirectMessage>(
            r#"SELECT * FROM direct_messages 
               WHERE (sender_id = ? AND recipient_id = ?) OR (sender_id = ? AND recipient_id = ?)
               AND deleted_at IS NULL
               ORDER BY created_at DESC LIMIT ?"#
        )
        .bind(user1)
        .bind(user2)
        .bind(user2)
        .bind(user1)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn mark_read(&self, pool: &SqlitePool, recipient_id: Uuid, sender_id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE direct_messages SET read_at = ?, updated_at = ? WHERE recipient_id = ? AND sender_id = ? AND read_at IS NULL")
            .bind(now)
            .bind(now)
            .bind(recipient_id)
            .bind(sender_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
