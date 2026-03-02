use async_trait::async_trait;
use erp_core::error::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait ArticleRepository: Send + Sync {
    async fn create(&self, article: &KnowledgeArticle) -> Result<KnowledgeArticle>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeArticle>>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<KnowledgeArticle>>;
    async fn find_all(&self, page: i32, limit: i32) -> Result<Vec<KnowledgeArticle>>;
    async fn find_by_category(&self, category: KnowledgeCategory) -> Result<Vec<KnowledgeArticle>>;
    async fn find_by_author(&self, author_id: Uuid) -> Result<Vec<KnowledgeArticle>>;
    async fn find_published(&self) -> Result<Vec<KnowledgeArticle>>;
    async fn search(&self, query: &str, filters: SearchFilters) -> Result<Vec<KnowledgeArticle>>;
    async fn update(&self, article: &KnowledgeArticle) -> Result<KnowledgeArticle>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn increment_view_count(&self, id: Uuid) -> Result<()>;
}

#[derive(serde::Serialize)]
pub struct SearchFilters {
    pub category: Option<KnowledgeCategory>,
    pub article_type: Option<ArticleType>,
    pub tags: Vec<String>,
    pub author_id: Option<Uuid>,
    pub date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub date_to: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait]
pub trait ArticleVersionRepository: Send + Sync {
    async fn create(&self, version: &ArticleVersion) -> Result<ArticleVersion>;
    async fn find_by_article(&self, article_id: Uuid) -> Result<Vec<ArticleVersion>>;
    async fn find_version(&self, article_id: Uuid, version: i32) -> Result<Option<ArticleVersion>>;
}

#[async_trait]
pub trait ArticleFeedbackRepository: Send + Sync {
    async fn create(&self, feedback: &ArticleFeedback) -> Result<ArticleFeedback>;
    async fn find_by_article(&self, article_id: Uuid) -> Result<Vec<ArticleFeedback>>;
    async fn get_summary(&self, article_id: Uuid) -> Result<FeedbackSummary>;
}

pub struct FeedbackSummary {
    pub article_id: Uuid,
    pub total_feedback: i32,
    pub helpful_count: i32,
    pub not_helpful_count: i32,
    pub average_rating: f64,
}

#[async_trait]
pub trait KnowledgeCategoryRepository: Send + Sync {
    async fn create(&self, category: &KnowledgeCategoryEntity) -> Result<KnowledgeCategoryEntity>;
    async fn find_all(&self) -> Result<Vec<KnowledgeCategoryEntity>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<KnowledgeCategoryEntity>>;
    async fn update(&self, category: &KnowledgeCategoryEntity) -> Result<KnowledgeCategoryEntity>;
}

#[allow(dead_code)]
pub struct SqliteArticleRepository {
    pool: SqlitePool,
}

impl SqliteArticleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticleRepository for SqliteArticleRepository {
    async fn create(&self, article: &KnowledgeArticle) -> Result<KnowledgeArticle> {
        Ok(article.clone())
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<KnowledgeArticle>> {
        Ok(None)
    }

    async fn find_by_slug(&self, _slug: &str) -> Result<Option<KnowledgeArticle>> {
        Ok(None)
    }

    async fn find_all(&self, _page: i32, _limit: i32) -> Result<Vec<KnowledgeArticle>> {
        Ok(Vec::new())
    }

    async fn find_by_category(&self, _category: KnowledgeCategory) -> Result<Vec<KnowledgeArticle>> {
        Ok(Vec::new())
    }

    async fn find_by_author(&self, _author_id: Uuid) -> Result<Vec<KnowledgeArticle>> {
        Ok(Vec::new())
    }

    async fn find_published(&self) -> Result<Vec<KnowledgeArticle>> {
        Ok(Vec::new())
    }

    async fn search(&self, _query: &str, _filters: SearchFilters) -> Result<Vec<KnowledgeArticle>> {
        Ok(Vec::new())
    }

    async fn update(&self, article: &KnowledgeArticle) -> Result<KnowledgeArticle> {
        Ok(article.clone())
    }

    async fn delete(&self, _id: Uuid) -> Result<()> {
        Ok(())
    }

    async fn increment_view_count(&self, _id: Uuid) -> Result<()> {
        Ok(())
    }
}
