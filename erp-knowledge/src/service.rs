use chrono::{DateTime, Utc};
use erp_core::error::{Error, Result};
use erp_core::models::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct ArticleService {
    article_repo: SqliteArticleRepository,
}

impl ArticleService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            article_repo: SqliteArticleRepository::new(pool),
        }
    }

    pub fn generate_slug(title: &str) -> String {
        title
            .to_lowercase()
            .replace(' ', "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
            .trim_matches('-')
            .to_string()
    }

    pub async fn create_article(
        &self,
        _pool: &SqlitePool,
        title: String,
        content: String,
        article_type: ArticleType,
        category: KnowledgeCategory,
        author_id: Uuid,
        tags: Vec<String>,
    ) -> Result<KnowledgeArticle> {
        if title.trim().is_empty() {
            return Err(Error::validation("Title is required".to_string()));
        }
        if content.trim().is_empty() {
            return Err(Error::validation("Content is required".to_string()));
        }
        let slug = Self::generate_slug(&title);
        let article = KnowledgeArticle {
            base: BaseEntity::new(),
            title,
            slug,
            content,
            summary: None,
            article_type,
            category,
            status: ArticleStatus::Draft,
            author_id,
            reviewer_id: None,
            published_at: None,
            expires_at: None,
            version: 1,
            parent_id: None,
            tags,
            view_count: 0,
            helpful_count: 0,
            not_helpful_count: 0,
            average_rating: 0.0,
            rating_count: 0,
            is_featured: false,
            is_internal: false,
            language: "en".to_string(),
            related_articles: Vec::new(),
            attachments: Vec::new(),
        };
        self.article_repo.create(&article).await
    }

    pub async fn get_article(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<KnowledgeArticle>> {
        self.article_repo.find_by_id(id).await
    }

    pub async fn get_article_by_slug(&self, _pool: &SqlitePool, slug: &str) -> Result<Option<KnowledgeArticle>> {
        self.article_repo.find_by_slug(slug).await
    }

    pub async fn publish_article(&self, _pool: &SqlitePool, id: Uuid) -> Result<KnowledgeArticle> {
        let mut article = self.article_repo.find_by_id(id).await?
            .ok_or(Error::not_found("article", &id.to_string()))?;
        if article.status != ArticleStatus::Draft && article.status != ArticleStatus::PendingReview {
            return Err(Error::validation("Article must be draft or pending review".to_string()));
        }
        article.status = ArticleStatus::Published;
        article.published_at = Some(Utc::now());
        self.article_repo.update(&article).await
    }

    pub async fn archive_article(&self, _pool: &SqlitePool, id: Uuid) -> Result<KnowledgeArticle> {
        let mut article = self.article_repo.find_by_id(id).await?
            .ok_or(Error::not_found("article", &id.to_string()))?;
        article.status = ArticleStatus::Archived;
        self.article_repo.update(&article).await
    }

    pub async fn update_article(&self, _pool: &SqlitePool, article: KnowledgeArticle) -> Result<KnowledgeArticle> {
        self.article_repo.update(&article).await
    }

    pub async fn search_articles(
        &self,
        _pool: &SqlitePool,
        query: &str,
        filters: SearchFilters,
    ) -> Result<Vec<KnowledgeArticle>> {
        self.article_repo.search(query, filters).await
    }

    pub async fn record_view(&self, _pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.article_repo.increment_view_count(id).await
    }
}

pub struct ArticleVersionService {
    pool: SqlitePool,
}

impl ArticleVersionService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_version(
        &self,
        article_id: Uuid,
        version: i32,
        title: String,
        content: String,
        change_summary: Option<String>,
        author_id: Uuid,
    ) -> Result<ArticleVersion> {
        let version = ArticleVersion {
            base: BaseEntity::new(),
            article_id,
            version,
            title,
            content,
            change_summary,
            author_id,
            created_at: Utc::now(),
            is_current: true,
        };
        Ok(version)
    }

    pub async fn get_version_history(&self, _article_id: Uuid) -> Result<Vec<ArticleVersion>> {
        Ok(Vec::new())
    }

    pub async fn restore_version(&self, _article_id: Uuid, version: i32) -> Result<KnowledgeArticle> {
        Ok(KnowledgeArticle {
            base: BaseEntity::new(),
            title: String::new(),
            slug: String::new(),
            content: String::new(),
            summary: None,
            article_type: ArticleType::HowTo,
            category: KnowledgeCategory::Technical,
            status: ArticleStatus::Draft,
            author_id: Uuid::nil(),
            reviewer_id: None,
            published_at: None,
            expires_at: None,
            version,
            parent_id: None,
            tags: Vec::new(),
            view_count: 0,
            helpful_count: 0,
            not_helpful_count: 0,
            average_rating: 0.0,
            rating_count: 0,
            is_featured: false,
            is_internal: false,
            language: "en".to_string(),
            related_articles: Vec::new(),
            attachments: Vec::new(),
        })
    }
}

pub struct ArticleFeedbackService {
    pool: SqlitePool,
}

impl ArticleFeedbackService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn submit_feedback(
        &self,
        article_id: Uuid,
        user_id: Uuid,
        is_helpful: bool,
        rating: Option<i32>,
        comment: Option<String>,
    ) -> Result<ArticleFeedback> {
        if let Some(r) = rating {
            if r < 1 || r > 5 {
                return Err(Error::validation("Rating must be between 1 and 5".to_string()));
            }
        }
        let feedback = ArticleFeedback {
            base: BaseEntity::new(),
            article_id,
            user_id,
            is_helpful,
            rating,
            comment,
            submitted_at: Utc::now(),
        };
        Ok(feedback)
    }

    pub async fn get_article_feedback(&self, _article_id: Uuid) -> Result<Vec<ArticleFeedback>> {
        Ok(Vec::new())
    }

    pub async fn calculate_helpfulness_rate(helpful: i64, not_helpful: i64) -> f64 {
        let total = helpful + not_helpful;
        if total > 0 {
            (helpful as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    pub async fn calculate_average_rating(ratings: &[i32]) -> f64 {
        if ratings.is_empty() {
            return 0.0;
        }
        ratings.iter().sum::<i32>() as f64 / ratings.len() as f64
    }
}

pub struct KnowledgeSearchService {
    pool: SqlitePool,
}

impl KnowledgeSearchService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn search(
        &self,
        query: &str,
        user_id: Uuid,
        filters: SearchFilters,
    ) -> Result<SearchResult> {
        let search = KnowledgeSearch {
            base: BaseEntity::new(),
            query: query.to_string(),
            user_id,
            results_count: 0,
            clicked_article_id: None,
            searched_at: Utc::now(),
            filters: serde_json::to_value(&filters).map_err(|e| Error::internal(format!("Failed to serialize filters: {}", e)))?,
        };
        Ok(SearchResult {
            search,
            articles: Vec::new(),
            facets: SearchFacets::default(),
        })
    }

    pub async fn record_click(&self, _search_id: Uuid, _article_id: Uuid) -> Result<()> {
        Ok(())
    }

    pub fn tokenize_query(query: &str) -> Vec<String> {
        query
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty() && s.len() > 2)
            .collect()
    }

    pub fn calculate_relevance_score(article: &KnowledgeArticle, tokens: &[String]) -> f64 {
        let mut score = 0.0;
        let title_lower = article.title.to_lowercase();
        let content_lower = article.content.to_lowercase();
        for token in tokens {
            if title_lower.contains(token) {
                score += 10.0;
            }
            let content_matches = content_lower.matches(token).count() as f64;
            score += content_matches * 0.5;
            if article.tags.iter().any(|t| t.to_lowercase().contains(token)) {
                score += 5.0;
            }
        }
        score += article.view_count as f64 * 0.01;
        score += article.average_rating * 2.0;
        score
    }
}

pub struct SearchResult {
    pub search: KnowledgeSearch,
    pub articles: Vec<KnowledgeArticle>,
    pub facets: SearchFacets,
}

#[derive(Default)]
pub struct SearchFacets {
    pub categories: std::collections::HashMap<String, i32>,
    pub types: std::collections::HashMap<String, i32>,
    pub authors: std::collections::HashMap<Uuid, i32>,
}

pub struct KnowledgeCategoryService {
    pool: SqlitePool,
}

impl KnowledgeCategoryService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_category(
        &self,
        name: String,
        description: Option<String>,
        parent_id: Option<Uuid>,
        icon: Option<String>,
    ) -> Result<KnowledgeCategoryEntity> {
        let slug = name
            .to_lowercase()
            .replace(' ', "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
        let category = KnowledgeCategoryEntity {
            base: BaseEntity::new(),
            name,
            slug,
            description,
            parent_id,
            icon,
            sort_order: 0,
            article_count: 0,
            is_active: true,
        };
        Ok(category)
    }

    pub async fn get_category_tree(&self) -> Result<Vec<CategoryNode>> {
        Ok(Vec::new())
    }
}

pub struct CategoryNode {
    pub category: KnowledgeCategoryEntity,
    pub children: Vec<CategoryNode>,
}

pub struct ArticleWorkflowService {
    pool: SqlitePool,
}

impl ArticleWorkflowService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn initiate_review(
        &self,
        article_id: Uuid,
        initiated_by: Uuid,
        assigned_to: Uuid,
    ) -> Result<ArticleWorkflow> {
        let workflow = ArticleWorkflow {
            base: BaseEntity::new(),
            article_id,
            workflow_type: WorkflowType::Review,
            status: WorkflowStatus::Pending,
            initiated_by,
            current_step: 1,
            total_steps: 2,
            assigned_to: Some(assigned_to),
            due_date: Some(Utc::now() + chrono::Duration::days(7)),
            completed_at: None,
        };
        Ok(workflow)
    }

    pub async fn approve_review(&self, _workflow_id: Uuid) -> Result<ArticleWorkflow> {
        Ok(ArticleWorkflow {
            base: BaseEntity::new(),
            article_id: Uuid::nil(),
            workflow_type: WorkflowType::Review,
            status: WorkflowStatus::Completed,
            initiated_by: Uuid::nil(),
            current_step: 2,
            total_steps: 2,
            assigned_to: None,
            due_date: None,
            completed_at: Some(Utc::now()),
        })
    }

    pub async fn reject_review(&self, _workflow_id: Uuid, _reason: String) -> Result<ArticleWorkflow> {
        Ok(ArticleWorkflow {
            base: BaseEntity::new(),
            article_id: Uuid::nil(),
            workflow_type: WorkflowType::Review,
            status: WorkflowStatus::Rejected,
            initiated_by: Uuid::nil(),
            current_step: 1,
            total_steps: 2,
            assigned_to: None,
            due_date: None,
            completed_at: None,
        })
    }
}

pub struct KnowledgeAnalyticsService {
    pool: SqlitePool,
}

impl KnowledgeAnalyticsService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn generate_daily_analytics(&self, date: DateTime<Utc>) -> Result<KnowledgeAnalytics> {
        let analytics = KnowledgeAnalytics {
            base: BaseEntity::new(),
            date,
            total_articles: 0,
            published_articles: 0,
            total_views: 0,
            unique_visitors: 0,
            search_count: 0,
            avg_rating: 0.0,
            helpful_rate: 0.0,
            top_articles: Vec::new(),
            top_searches: Vec::new(),
            category_breakdown: serde_json::json!({}),
        };
        Ok(analytics)
    }

    pub async fn get_trending_articles(&self, _days: i32) -> Result<Vec<TopArticle>> {
        Ok(Vec::new())
    }

    pub async fn get_search_trends(&self, _days: i32) -> Result<Vec<TopSearch>> {
        Ok(Vec::new())
    }

    pub async fn calculate_knowledge_health_score(&self) -> Result<KnowledgeHealthScore> {
        Ok(KnowledgeHealthScore {
            overall_score: 75.0,
            coverage_score: 80.0,
            freshness_score: 70.0,
            quality_score: 85.0,
            engagement_score: 65.0,
        })
    }
}

pub struct KnowledgeHealthScore {
    pub overall_score: f64,
    pub coverage_score: f64,
    pub freshness_score: f64,
    pub quality_score: f64,
    pub engagement_score: f64,
}

pub struct ArticleTranslationService {
    pool: SqlitePool,
}

impl ArticleTranslationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn request_translation(
        &self,
        article_id: Uuid,
        language: String,
    ) -> Result<ArticleTranslation> {
        let translation = ArticleTranslation {
            base: BaseEntity::new(),
            article_id,
            language,
            title: String::new(),
            content: String::new(),
            summary: None,
            translated_by: None,
            translated_at: None,
            status: TranslationStatus::Pending,
            needs_update: false,
        };
        Ok(translation)
    }

    pub async fn complete_translation(
        &self,
        _translation_id: Uuid,
        title: String,
        content: String,
        translated_by: Uuid,
    ) -> Result<ArticleTranslation> {
        Ok(ArticleTranslation {
            base: BaseEntity::new(),
            article_id: Uuid::nil(),
            language: String::new(),
            title,
            content,
            summary: None,
            translated_by: Some(translated_by),
            translated_at: Some(Utc::now()),
            status: TranslationStatus::Completed,
            needs_update: false,
        })
    }
}

pub struct KnowledgeTemplateService {
    pool: SqlitePool,
}

impl KnowledgeTemplateService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_template(
        &self,
        name: String,
        category: KnowledgeCategory,
        template_content: String,
        placeholders: Vec<TemplatePlaceholder>,
        created_by: Uuid,
    ) -> Result<KnowledgeTemplate> {
        let template = KnowledgeTemplate {
            base: BaseEntity::new(),
            name,
            description: None,
            category,
            template_content,
            placeholders,
            created_by,
            is_active: true,
        };
        Ok(template)
    }

    pub fn apply_template(template: &KnowledgeTemplate, values: std::collections::HashMap<&str, &str>) -> String {
        let mut content = template.template_content.clone();
        for placeholder in &template.placeholders {
            let value = values.get(placeholder.name.as_str())
                .copied()
                .or(placeholder.default_value.as_deref())
                .unwrap_or("");
            content = content.replace(&format!("{{{{{}}}}}", placeholder.name), value);
        }
        content
    }
}
