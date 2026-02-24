use chrono::{DateTime, Utc};
use erp_core::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndexEntry {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub title: String,
    pub content: Option<String>,
    pub keywords: Option<String>,
    pub tenant_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub entity_types: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub entity_type: String,
    pub entity_id: String,
    pub title: String,
    pub snippet: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: i64,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub title: String,
    pub content: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub tenant_id: Option<String>,
}

pub struct SearchService;

impl SearchService {
    pub fn new() -> Self {
        Self
    }

    pub async fn index(&self, pool: &SqlitePool, req: IndexRequest) -> Result<SearchIndexEntry> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let keywords = req.keywords.map(|k| k.join(", "));

        sqlx::query(
            r#"INSERT OR REPLACE INTO search_index 
               (id, entity_type, entity_id, title, content, keywords, tenant_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(id.to_string())
        .bind(&req.entity_type)
        .bind(&req.entity_id)
        .bind(&req.title)
        .bind(&req.content)
        .bind(&keywords)
        .bind(&req.tenant_id)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(SearchIndexEntry {
            id,
            entity_type: req.entity_type,
            entity_id: req.entity_id,
            title: req.title,
            content: req.content,
            keywords,
            tenant_id: req.tenant_id,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn remove_from_index(&self, pool: &SqlitePool, entity_type: &str, entity_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM search_index WHERE entity_type = ? AND entity_id = ?")
            .bind(entity_type)
            .bind(entity_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn search(&self, pool: &SqlitePool, req: SearchRequest) -> Result<SearchResponse> {
        let limit = req.limit.unwrap_or(50).min(100);
        let offset = req.offset.unwrap_or(0);
        let search_term = format!("%{}%", req.query.to_lowercase());

        let results = if let Some(entity_types) = &req.entity_types {
            let placeholders: Vec<String> = entity_types.iter().map(|_| "?".to_string()).collect();
            let query = format!(
                r#"SELECT entity_type, entity_id, title, content 
                   FROM search_index 
                   WHERE (LOWER(title) LIKE ? OR LOWER(content) LIKE ? OR LOWER(keywords) LIKE ?)
                   AND entity_type IN ({})
                   ORDER BY 
                     CASE WHEN LOWER(title) LIKE ? THEN 1 ELSE 2 END,
                     created_at DESC
                   LIMIT ? OFFSET ?"#,
                placeholders.join(",")
            );

            let mut sql_query = sqlx::query(&query);
            sql_query = sql_query.bind(&search_term).bind(&search_term).bind(&search_term);
            for et in entity_types {
                sql_query = sql_query.bind(et);
            }
            sql_query = sql_query.bind(&search_term).bind(limit).bind(offset);

            let rows: Vec<SearchRow> = sql_query
                .map(|row: sqlx::sqlite::SqliteRow| {
                    use sqlx::Row;
                    SearchRow {
                        entity_type: row.get("entity_type"),
                        entity_id: row.get("entity_id"),
                        title: row.get("title"),
                        content: row.get("content"),
                    }
                })
                .fetch_all(pool)
                .await?;

            rows
        } else {
            let query = r#"SELECT entity_type, entity_id, title, content 
                          FROM search_index 
                          WHERE (LOWER(title) LIKE ? OR LOWER(content) LIKE ? OR LOWER(keywords) LIKE ?)
                          ORDER BY 
                            CASE WHEN LOWER(title) LIKE ? THEN 1 ELSE 2 END,
                            created_at DESC
                          LIMIT ? OFFSET ?"#;

            let rows: Vec<SearchRow> = sqlx::query(query)
                .bind(&search_term)
                .bind(&search_term)
                .bind(&search_term)
                .bind(&search_term)
                .bind(limit)
                .bind(offset)
                .map(|row: sqlx::sqlite::SqliteRow| {
                    use sqlx::Row;
                    SearchRow {
                        entity_type: row.get("entity_type"),
                        entity_id: row.get("entity_id"),
                        title: row.get("title"),
                        content: row.get("content"),
                    }
                })
                .fetch_all(pool)
                .await?;

            rows
        };

        let total: i64 = if let Some(entity_types) = &req.entity_types {
            let placeholders: Vec<String> = entity_types.iter().map(|_| "?".to_string()).collect();
            let count_query = format!(
                r#"SELECT COUNT(*) as count FROM search_index 
                   WHERE (LOWER(title) LIKE ? OR LOWER(content) LIKE ? OR LOWER(keywords) LIKE ?)
                   AND entity_type IN ({})"#,
                placeholders.join(",")
            );
            
            let mut sql_query = sqlx::query_scalar::<_, i64>(&count_query);
            sql_query = sql_query.bind(&search_term).bind(&search_term).bind(&search_term);
            for et in entity_types {
                sql_query = sql_query.bind(et);
            }
            sql_query.fetch_one(pool).await?
        } else {
            sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(*) FROM search_index 
                   WHERE (LOWER(title) LIKE ? OR LOWER(content) LIKE ? OR LOWER(keywords) LIKE ?)"#
            )
            .bind(&search_term)
            .bind(&search_term)
            .bind(&search_term)
            .fetch_one(pool)
            .await?
        };

        let results: Vec<SearchResult> = results
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let snippet = Self::create_snippet(&row.content, &req.query);
                SearchResult {
                    entity_type: row.entity_type,
                    entity_id: row.entity_id,
                    title: row.title,
                    snippet,
                    relevance_score: 1.0 / (1.0 + i as f64),
                }
            })
            .collect();

        Ok(SearchResponse {
            results,
            total,
            query: req.query,
        })
    }

    fn create_snippet(content: &Option<String>, query: &str) -> String {
        let content = match content {
            Some(c) if !c.is_empty() => c,
            _ => return String::new(),
        };

        let lower_content = content.to_lowercase();
        let lower_query = query.to_lowercase();

        if let Some(pos) = lower_content.find(&lower_query) {
            let start = pos.saturating_sub(50);
            let end = (pos + query.len() + 50).min(content.len());
            let mut snippet = content[start..end].to_string();
            if start > 0 {
                snippet = format!("...{}", snippet);
            }
            if end < content.len() {
                snippet = format!("{}...", snippet);
            }
            snippet
        } else {
            let end = 100.min(content.len());
            let mut snippet = content[..end].to_string();
            if end < content.len() {
                snippet.push_str("...");
            }
            snippet
        }
    }

    pub async fn rebuild_index(&self, pool: &SqlitePool) -> Result<u64> {
        let result = sqlx::query("DELETE FROM search_index")
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn get_stats(&self, pool: &SqlitePool) -> Result<SearchStats> {
        let total_entries: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM search_index")
            .fetch_one(pool)
            .await?;

        let entity_counts: Vec<EntityCount> = sqlx::query_as(
            "SELECT entity_type, COUNT(*) as count FROM search_index GROUP BY entity_type ORDER BY count DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(SearchStats {
            total_entries,
            entity_counts,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
struct SearchRow {
    entity_type: String,
    entity_id: String,
    title: String,
    content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchStats {
    pub total_entries: i64,
    pub entity_counts: Vec<EntityCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EntityCount {
    pub entity_type: String,
    pub count: i64,
}
