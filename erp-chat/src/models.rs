use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ChannelType {
    Direct,
    Group,
    Project,
    Department,
    Team,
    Public,
    Announcement,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MessageType {
    Text,
    File,
    Image,
    Video,
    Audio,
    VoiceNote,
    System,
    Reply,
    Forward,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
    Deleted,
    Edited,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MembershipRole {
    Owner,
    Admin,
    Moderator,
    Member,
    Guest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChannel {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub channel_type: ChannelType,
    pub avatar_url: Option<String>,
    pub is_private: bool,
    pub is_archived: bool,
    pub owner_id: Uuid,
    pub parent_channel_id: Option<Uuid>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<Uuid>,
    pub topic: Option<String>,
    pub slow_mode: bool,
    pub slow_mode_delay: Option<i32>,
    pub allow_mentions: bool,
    pub allow_reactions: bool,
    pub allow_threads: bool,
    pub auto_join: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub base: BaseEntity,
    pub channel_id: Uuid,
    pub sender_id: Uuid,
    pub parent_message_id: Option<Uuid>,
    pub thread_id: Option<Uuid>,
    pub message_type: MessageType,
    pub content: String,
    pub formatted_content: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub mentions: Option<Vec<Uuid>>,
    pub reactions: Option<serde_json::Value>,
    pub reply_count: i32,
    pub status: MessageStatus,
    pub edited_at: Option<DateTime<Utc>>,
    pub edited_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub pinned_at: Option<DateTime<Utc>>,
    pub pinned_by: Option<Uuid>,
    pub starred_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMembership {
    pub base: BaseEntity,
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub role: MembershipRole,
    pub nickname: Option<String>,
    pub muted: bool,
    pub muted_until: Option<DateTime<Utc>>,
    pub notifications_enabled: bool,
    pub last_read_at: Option<DateTime<Utc>>,
    pub last_read_message_id: Option<Uuid>,
    pub unread_count: i32,
    pub unread_mentions: i32,
    pub starred: bool,
    pub hidden: bool,
    pub joined_at: DateTime<Utc>,
    pub invited_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectMessage {
    pub base: BaseEntity,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub message_type: MessageType,
    pub content: String,
    pub formatted_content: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub status: MessageStatus,
    pub read_at: Option<DateTime<Utc>>,
    pub edited_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReaction {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub emoji: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingIndicator {
    pub id: Uuid,
    pub channel_id: Option<Uuid>,
    pub dm_recipient_id: Option<Uuid>,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPresence {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: UserStatus,
    pub status_message: Option<String>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub online: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum UserStatus {
    Online,
    Away,
    Busy,
    Offline,
    Invisible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSearch {
    pub id: Uuid,
    pub message_id: Uuid,
    pub channel_id: Uuid,
    pub sender_id: Uuid,
    pub content_text: String,
    pub created_at: DateTime<Utc>,
}
