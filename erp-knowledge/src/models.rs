use chrono::{DateTime, Utc};
use erp_core::models::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[derive(PartialEq)]
pub enum ArticleStatus {
    Draft,
    PendingReview,
    Published,
    Archived,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ArticleType {
    HowTo,
    FAQ,
    Troubleshooting,
    BestPractice,
    Policy,
    Procedure,
    Reference,
    Tutorial,
    WhitePaper,
    CaseStudy,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KnowledgeCategory {
    Technical,
    HR,
    Finance,
    Operations,
    Sales,
    Marketing,
    Legal,
    Compliance,
    IT,
    CustomerService,
    Product,
    Training,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeArticle {
    pub base: BaseEntity,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub summary: Option<String>,
    pub article_type: ArticleType,
    pub category: KnowledgeCategory,
    pub status: ArticleStatus,
    pub author_id: Uuid,
    pub reviewer_id: Option<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub version: i32,
    pub parent_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub view_count: i64,
    pub helpful_count: i64,
    pub not_helpful_count: i64,
    pub average_rating: f64,
    pub rating_count: i32,
    pub is_featured: bool,
    pub is_internal: bool,
    pub language: String,
    pub related_articles: Vec<Uuid>,
    pub attachments: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeCategoryEntity {
    pub base: BaseEntity,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub article_count: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleVersion {
    pub base: BaseEntity,
    pub article_id: Uuid,
    pub version: i32,
    pub title: String,
    pub content: String,
    pub change_summary: Option<String>,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleFeedback {
    pub base: BaseEntity,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub is_helpful: bool,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleRevision {
    pub base: BaseEntity,
    pub article_id: Uuid,
    pub revision_type: RevisionType,
    pub old_content: String,
    pub new_content: String,
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RevisionType {
    Create,
    Edit,
    Publish,
    Archive,
    Restore,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSearch {
    pub base: BaseEntity,
    pub query: String,
    pub user_id: Uuid,
    pub results_count: i32,
    pub clicked_article_id: Option<Uuid>,
    pub searched_at: DateTime<Utc>,
    pub filters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleBookmark {
    pub base: BaseEntity,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub folder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleSubscription {
    pub base: BaseEntity,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub subscribed_at: DateTime<Utc>,
    pub notify_on_update: bool,
    pub notify_on_comment: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub category: KnowledgeCategory,
    pub template_content: String,
    pub placeholders: Vec<TemplatePlaceholder>,
    pub created_by: Uuid,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePlaceholder {
    pub name: String,
    pub description: String,
    pub default_value: Option<String>,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleWorkflow {
    pub base: BaseEntity,
    pub article_id: Uuid,
    pub workflow_type: WorkflowType,
    pub status: WorkflowStatus,
    pub initiated_by: Uuid,
    pub current_step: i32,
    pub total_steps: i32,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WorkflowType {
    Review,
    Approval,
    Translation,
    Update,
    Deprecation,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WorkflowStatus {
    Pending,
    InProgress,
    Completed,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTag {
    pub base: BaseEntity,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub usage_count: i32,
    pub is_trending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleTranslation {
    pub base: BaseEntity,
    pub article_id: Uuid,
    pub language: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub translated_by: Option<Uuid>,
    pub translated_at: Option<DateTime<Utc>>,
    pub status: TranslationStatus,
    pub needs_update: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TranslationStatus {
    Pending,
    InProgress,
    Completed,
    NeedsReview,
    Outdated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeAnalytics {
    pub base: BaseEntity,
    pub date: DateTime<Utc>,
    pub total_articles: i32,
    pub published_articles: i32,
    pub total_views: i64,
    pub unique_visitors: i64,
    pub search_count: i64,
    pub avg_rating: f64,
    pub helpful_rate: f64,
    pub top_articles: Vec<TopArticle>,
    pub top_searches: Vec<TopSearch>,
    pub category_breakdown: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopArticle {
    pub article_id: Uuid,
    pub title: String,
    pub views: i64,
    pub rating: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSearch {
    pub query: String,
    pub count: i64,
    pub results_found: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleComment {
    pub base: BaseEntity,
    pub article_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub user_id: Uuid,
    pub content: String,
    pub status: CommentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommentStatus {
    Pending,
    Approved,
    Rejected,
    Spam,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleLink {
    pub base: BaseEntity,
    pub source_article_id: Uuid,
    pub target_article_id: Uuid,
    pub link_type: LinkType,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LinkType {
    Related,
    SeeAlso,
    Prerequisite,
    Next,
    Previous,
    Parent,
}
