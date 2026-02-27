use crate::models::*;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

#[async_trait]
pub trait KanbanRepository: Send + Sync {
    async fn create_board(&self, pool: &SqlitePool, board: &KanbanBoard) -> Result<KanbanBoard>;
    async fn get_board(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KanbanBoard>>;
    async fn list_boards(&self, pool: &SqlitePool, limit: i32, offset: i32) -> Result<Vec<KanbanBoard>>;
    async fn update_board(&self, pool: &SqlitePool, board: &KanbanBoard) -> Result<KanbanBoard>;
    async fn delete_board(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    
    async fn create_column(&self, pool: &SqlitePool, column: &KanbanColumn) -> Result<KanbanColumn>;
    async fn get_columns_by_board(&self, pool: &SqlitePool, board_id: Uuid) -> Result<Vec<KanbanColumn>>;
    async fn update_column(&self, pool: &SqlitePool, column: &KanbanColumn) -> Result<KanbanColumn>;
    async fn delete_column(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    
    async fn create_card(&self, pool: &SqlitePool, card: &KanbanCard) -> Result<KanbanCard>;
    async fn get_card(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KanbanCard>>;
    async fn list_cards_by_board(&self, pool: &SqlitePool, board_id: Uuid) -> Result<Vec<KanbanCard>>;
    async fn list_cards_by_column(&self, pool: &SqlitePool, column_id: Uuid) -> Result<Vec<KanbanCard>>;
    async fn update_card(&self, pool: &SqlitePool, card: &KanbanCard) -> Result<KanbanCard>;
    async fn delete_card(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    
    async fn move_card(&self, pool: &SqlitePool, card_move: &KanbanCardMove) -> Result<KanbanCardMove>;
    async fn get_card_moves(&self, pool: &SqlitePool, card_id: Uuid) -> Result<Vec<KanbanCardMove>>;
    
    async fn create_comment(&self, pool: &SqlitePool, comment: &KanbanCardComment) -> Result<KanbanCardComment>;
    async fn list_comments(&self, pool: &SqlitePool, card_id: Uuid) -> Result<Vec<KanbanCardComment>>;
    async fn delete_comment(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    
    async fn create_checklist(&self, pool: &SqlitePool, checklist: &KanbanCardChecklist) -> Result<KanbanCardChecklist>;
    async fn list_checklists(&self, pool: &SqlitePool, card_id: Uuid) -> Result<Vec<KanbanCardChecklist>>;
    async fn update_checklist_item(&self, pool: &SqlitePool, item: &KanbanCardChecklistItem) -> Result<KanbanCardChecklistItem>;
    
    async fn log_activity(&self, pool: &SqlitePool, log: &KanbanActivityLog) -> Result<KanbanActivityLog>;
    async fn list_activities(&self, pool: &SqlitePool, board_id: Uuid, limit: i32) -> Result<Vec<KanbanActivityLog>>;
    
    async fn count_cards_in_column(&self, pool: &SqlitePool, column_id: Uuid) -> Result<i32>;
    async fn create_wip_violation(&self, pool: &SqlitePool, violation: &KanbanWipViolation) -> Result<KanbanWipViolation>;
    async fn resolve_wip_violation(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteKanbanRepository;

fn parse_card_from_row(row: &sqlx::sqlite::SqliteRow) -> Option<KanbanCard> {
    let id: String = row.try_get("id").ok()?;
    let board_id: String = row.try_get("board_id").ok()?;
    let column_id: String = row.try_get("column_id").ok()?;
    
    Some(KanbanCard {
        base: erp_core::BaseEntity::new_with_id(id.parse().ok()?),
        board_id: board_id.parse().ok()?,
        column_id: column_id.parse().ok()?,
        swimlane_id: row.try_get::<Option<String>, _>("swimlane_id").ok()?.and_then(|s| s.parse().ok()),
        card_type: serde_json::from_str(&row.try_get::<String, _>("card_type").ok()?).ok()?,
        title: row.try_get("title").ok()?,
        description: row.try_get("description").ok()?,
        priority: serde_json::from_str(&row.try_get::<String, _>("priority").ok()?).ok()?,
        position: row.try_get("position").ok()?,
        assignee_ids: serde_json::from_str(&row.try_get::<String, _>("assignee_ids").ok()?).ok()?,
        reporter_id: row.try_get::<Option<String>, _>("reporter_id").ok()?.and_then(|s| s.parse().ok()),
        due_date: row.try_get::<Option<String>, _>("due_date").ok()?.and_then(|s| s.parse().ok()),
        start_date: row.try_get::<Option<String>, _>("start_date").ok()?.and_then(|s| s.parse().ok()),
        completed_date: row.try_get::<Option<String>, _>("completed_date").ok()?.and_then(|s| s.parse().ok()),
        estimated_hours: row.try_get("estimated_hours").ok()?,
        actual_hours: row.try_get("actual_hours").ok()?,
        story_points: row.try_get("story_points").ok()?,
        tags: serde_json::from_str(&row.try_get::<String, _>("tags").ok()?).ok()?,
        external_ref_type: row.try_get("external_ref_type").ok()?,
        external_ref_id: row.try_get::<Option<String>, _>("external_ref_id").ok()?.and_then(|s| s.parse().ok()),
        blocked: row.try_get::<i32, _>("blocked").ok()? == 1,
        blocked_reason: row.try_get("blocked_reason").ok()?,
        parent_card_id: row.try_get::<Option<String>, _>("parent_card_id").ok()?.and_then(|s| s.parse().ok()),
        status: serde_json::from_str(&row.try_get::<String, _>("status").ok()?).ok()?,
        created_at: row.try_get::<String, _>("created_at").ok()?.parse().ok()?,
        updated_at: row.try_get::<String, _>("updated_at").ok()?.parse().ok()?,
    })
}

#[async_trait]
impl KanbanRepository for SqliteKanbanRepository {
    async fn create_board(&self, pool: &SqlitePool, board: &KanbanBoard) -> Result<KanbanBoard> {
        sqlx::query(
            r#"INSERT INTO kanban_boards (id, name, description, board_type, team_id, project_id, 
               swimlane_type, default_wip_limit, allow_card_reordering, show_card_count, show_wip_limits, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(board.base.id.to_string())
        .bind(&board.name)
        .bind(&board.description)
        .bind(serde_json::to_string(&board.board_type)?)
        .bind(board.team_id.map(|id| id.to_string()))
        .bind(board.project_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&board.swimlane_type)?)
        .bind(board.default_wip_limit)
        .bind(board.allow_card_reordering as i32)
        .bind(board.show_card_count as i32)
        .bind(board.show_wip_limits as i32)
        .bind(serde_json::to_string(&board.status)?)
        .bind(board.created_at.to_rfc3339())
        .bind(board.updated_at.to_rfc3339())
        .execute(pool).await?;

        for column in &board.columns {
            self.create_column(pool, column).await?;
        }
        
        Ok(board.clone())
    }

    async fn get_board(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KanbanBoard>> {
        let row = sqlx::query(
            "SELECT id, name, description, board_type, team_id, project_id, swimlane_type, default_wip_limit, allow_card_reordering, show_card_count, show_wip_limits, status, created_at, updated_at FROM kanban_boards WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;

        if let Some(r) = row {
            let columns = self.get_columns_by_board(pool, id).await?;
            Ok(Some(KanbanBoard {
                base: erp_core::BaseEntity::new_with_id(id),
                name: r.try_get("name")?,
                description: r.try_get("description")?,
                board_type: serde_json::from_str(&r.try_get::<String, _>("board_type")?)?,
                team_id: r.try_get::<Option<String>, _>("team_id")?.and_then(|s| s.parse().ok()),
                project_id: r.try_get::<Option<String>, _>("project_id")?.and_then(|s| s.parse().ok()),
                columns,
                swimlane_type: serde_json::from_str(&r.try_get::<String, _>("swimlane_type")?)?,
                swimlanes: vec![],
                default_wip_limit: r.try_get("default_wip_limit")?,
                allow_card_reordering: r.try_get::<i32, _>("allow_card_reordering")? == 1,
                show_card_count: r.try_get::<i32, _>("show_card_count")? == 1,
                show_wip_limits: r.try_get::<i32, _>("show_wip_limits")? == 1,
                status: serde_json::from_str(&r.try_get::<String, _>("status")?)?,
                created_at: r.try_get::<String, _>("created_at")?.parse()?,
                updated_at: r.try_get::<String, _>("updated_at")?.parse()?,
            }))
        } else {
            Ok(None)
        }
    }

    async fn list_boards(&self, pool: &SqlitePool, limit: i32, offset: i32) -> Result<Vec<KanbanBoard>> {
        let rows = sqlx::query(
            "SELECT id, name, description, board_type, team_id, project_id, swimlane_type, default_wip_limit, allow_card_reordering, show_card_count, show_wip_limits, status, created_at, updated_at FROM kanban_boards ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool).await?;

        let mut boards = Vec::new();
        for r in rows {
            let id: Uuid = r.try_get::<String, _>("id")?.parse()?;
            let columns = self.get_columns_by_board(pool, id).await?;
            boards.push(KanbanBoard {
                base: erp_core::BaseEntity::new_with_id(id),
                name: r.try_get("name")?,
                description: r.try_get("description")?,
                board_type: serde_json::from_str(&r.try_get::<String, _>("board_type")?)?,
                team_id: r.try_get::<Option<String>, _>("team_id")?.and_then(|s| s.parse().ok()),
                project_id: r.try_get::<Option<String>, _>("project_id")?.and_then(|s| s.parse().ok()),
                columns,
                swimlane_type: serde_json::from_str(&r.try_get::<String, _>("swimlane_type")?)?,
                swimlanes: vec![],
                default_wip_limit: r.try_get("default_wip_limit")?,
                allow_card_reordering: r.try_get::<i32, _>("allow_card_reordering")? == 1,
                show_card_count: r.try_get::<i32, _>("show_card_count")? == 1,
                show_wip_limits: r.try_get::<i32, _>("show_wip_limits")? == 1,
                status: serde_json::from_str(&r.try_get::<String, _>("status")?)?,
                created_at: r.try_get::<String, _>("created_at")?.parse()?,
                updated_at: r.try_get::<String, _>("updated_at")?.parse()?,
            });
        }
        Ok(boards)
    }

    async fn update_board(&self, pool: &SqlitePool, board: &KanbanBoard) -> Result<KanbanBoard> {
        let now = Utc::now();
        sqlx::query(
            r#"UPDATE kanban_boards SET name = ?, description = ?, board_type = ?, team_id = ?, project_id = ?,
               swimlane_type = ?, default_wip_limit = ?, allow_card_reordering = ?, show_card_count = ?, 
               show_wip_limits = ?, status = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&board.name)
        .bind(&board.description)
        .bind(serde_json::to_string(&board.board_type)?)
        .bind(board.team_id.map(|id| id.to_string()))
        .bind(board.project_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&board.swimlane_type)?)
        .bind(board.default_wip_limit)
        .bind(board.allow_card_reordering as i32)
        .bind(board.show_card_count as i32)
        .bind(board.show_wip_limits as i32)
        .bind(serde_json::to_string(&board.status)?)
        .bind(now.to_rfc3339())
        .bind(board.base.id.to_string())
        .execute(pool).await?;
        Ok(board.clone())
    }

    async fn delete_board(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM kanban_boards WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_column(&self, pool: &SqlitePool, column: &KanbanColumn) -> Result<KanbanColumn> {
        sqlx::query(
            r#"INSERT INTO kanban_columns (id, board_id, name, position, wip_limit, is_done_column, is_backlog, color, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(column.id.to_string())
        .bind(column.board_id.to_string())
        .bind(&column.name)
        .bind(column.position)
        .bind(column.wip_limit)
        .bind(column.is_done_column as i32)
        .bind(column.is_backlog as i32)
        .bind(&column.color)
        .bind(column.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(column.clone())
    }

    async fn get_columns_by_board(&self, pool: &SqlitePool, board_id: Uuid) -> Result<Vec<KanbanColumn>> {
        let rows = sqlx::query(
            "SELECT id, board_id, name, position, wip_limit, is_done_column, is_backlog, color, created_at FROM kanban_columns WHERE board_id = ? ORDER BY position"
        )
        .bind(board_id.to_string())
        .fetch_all(pool).await?;

        Ok(rows.into_iter().filter_map(|r| {
            Some(KanbanColumn {
                id: r.try_get::<String, _>("id").ok()?.parse().ok()?,
                board_id: r.try_get::<String, _>("board_id").ok()?.parse().ok()?,
                name: r.try_get("name").ok()?,
                position: r.try_get("position").ok()?,
                wip_limit: r.try_get("wip_limit").ok()?,
                is_done_column: r.try_get::<i32, _>("is_done_column").ok()? == 1,
                is_backlog: r.try_get::<i32, _>("is_backlog").ok()? == 1,
                color: r.try_get("color").ok()?,
                auto_assign_on_move: None,
                created_at: r.try_get::<String, _>("created_at").ok()?.parse().ok()?,
            })
        }).collect())
    }

    async fn update_column(&self, pool: &SqlitePool, column: &KanbanColumn) -> Result<KanbanColumn> {
        sqlx::query(
            "UPDATE kanban_columns SET name = ?, position = ?, wip_limit = ?, is_done_column = ?, is_backlog = ?, color = ? WHERE id = ?"
        )
        .bind(&column.name)
        .bind(column.position)
        .bind(column.wip_limit)
        .bind(column.is_done_column as i32)
        .bind(column.is_backlog as i32)
        .bind(&column.color)
        .bind(column.id.to_string())
        .execute(pool).await?;
        Ok(column.clone())
    }

    async fn delete_column(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM kanban_columns WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_card(&self, pool: &SqlitePool, card: &KanbanCard) -> Result<KanbanCard> {
        sqlx::query(
            r#"INSERT INTO kanban_cards (id, board_id, column_id, swimlane_id, card_type, title, description, 
               priority, position, assignee_ids, reporter_id, due_date, start_date, completed_date, 
               estimated_hours, actual_hours, story_points, tags, external_ref_type, external_ref_id, 
               blocked, blocked_reason, parent_card_id, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(card.base.id.to_string())
        .bind(card.board_id.to_string())
        .bind(card.column_id.to_string())
        .bind(card.swimlane_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&card.card_type)?)
        .bind(&card.title)
        .bind(&card.description)
        .bind(serde_json::to_string(&card.priority)?)
        .bind(card.position)
        .bind(serde_json::to_string(&card.assignee_ids)?)
        .bind(card.reporter_id.map(|id| id.to_string()))
        .bind(card.due_date.map(|d| d.to_rfc3339()))
        .bind(card.start_date.map(|d| d.to_rfc3339()))
        .bind(card.completed_date.map(|d| d.to_rfc3339()))
        .bind(card.estimated_hours)
        .bind(card.actual_hours)
        .bind(card.story_points)
        .bind(serde_json::to_string(&card.tags)?)
        .bind(&card.external_ref_type)
        .bind(card.external_ref_id.map(|id| id.to_string()))
        .bind(card.blocked as i32)
        .bind(&card.blocked_reason)
        .bind(card.parent_card_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&card.status)?)
        .bind(card.created_at.to_rfc3339())
        .bind(card.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(card.clone())
    }

    async fn get_card(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KanbanCard>> {
        let row = sqlx::query(
            "SELECT * FROM kanban_cards WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;

        Ok(row.as_ref().and_then(parse_card_from_row))
    }

    async fn list_cards_by_board(&self, pool: &SqlitePool, board_id: Uuid) -> Result<Vec<KanbanCard>> {
        let rows = sqlx::query(
            "SELECT * FROM kanban_cards WHERE board_id = ? ORDER BY column_id, position"
        )
        .bind(board_id.to_string())
        .fetch_all(pool).await?;

        Ok(rows.iter().filter_map(parse_card_from_row).collect())
    }

    async fn list_cards_by_column(&self, pool: &SqlitePool, column_id: Uuid) -> Result<Vec<KanbanCard>> {
        let rows = sqlx::query(
            "SELECT * FROM kanban_cards WHERE column_id = ? ORDER BY position"
        )
        .bind(column_id.to_string())
        .fetch_all(pool).await?;

        Ok(rows.iter().filter_map(parse_card_from_row).collect())
    }

    async fn update_card(&self, pool: &SqlitePool, card: &KanbanCard) -> Result<KanbanCard> {
        let now = Utc::now();
        sqlx::query(
            r#"UPDATE kanban_cards SET column_id = ?, swimlane_id = ?, title = ?, description = ?, 
               priority = ?, position = ?, assignee_ids = ?, reporter_id = ?, due_date = ?, 
               start_date = ?, completed_date = ?, estimated_hours = ?, actual_hours = ?, 
               story_points = ?, tags = ?, external_ref_type = ?, external_ref_id = ?, 
               blocked = ?, blocked_reason = ?, parent_card_id = ?, status = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(card.column_id.to_string())
        .bind(card.swimlane_id.map(|id| id.to_string()))
        .bind(&card.title)
        .bind(&card.description)
        .bind(serde_json::to_string(&card.priority)?)
        .bind(card.position)
        .bind(serde_json::to_string(&card.assignee_ids)?)
        .bind(card.reporter_id.map(|id| id.to_string()))
        .bind(card.due_date.map(|d| d.to_rfc3339()))
        .bind(card.start_date.map(|d| d.to_rfc3339()))
        .bind(card.completed_date.map(|d| d.to_rfc3339()))
        .bind(card.estimated_hours)
        .bind(card.actual_hours)
        .bind(card.story_points)
        .bind(serde_json::to_string(&card.tags)?)
        .bind(&card.external_ref_type)
        .bind(card.external_ref_id.map(|id| id.to_string()))
        .bind(card.blocked as i32)
        .bind(&card.blocked_reason)
        .bind(card.parent_card_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&card.status)?)
        .bind(now.to_rfc3339())
        .bind(card.base.id.to_string())
        .execute(pool).await?;
        Ok(card.clone())
    }

    async fn delete_card(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM kanban_cards WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn move_card(&self, pool: &SqlitePool, card_move: &KanbanCardMove) -> Result<KanbanCardMove> {
        sqlx::query(
            r#"INSERT INTO kanban_card_moves (id, card_id, from_column_id, to_column_id, from_position, to_position, moved_by, moved_at, time_in_from_column_seconds)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(card_move.id.to_string())
        .bind(card_move.card_id.to_string())
        .bind(card_move.from_column_id.to_string())
        .bind(card_move.to_column_id.to_string())
        .bind(card_move.from_position)
        .bind(card_move.to_position)
        .bind(card_move.moved_by.to_string())
        .bind(card_move.moved_at.to_rfc3339())
        .bind(card_move.time_in_from_column_seconds)
        .execute(pool).await?;
        Ok(card_move.clone())
    }

    async fn get_card_moves(&self, pool: &SqlitePool, card_id: Uuid) -> Result<Vec<KanbanCardMove>> {
        let rows = sqlx::query(
            "SELECT id, card_id, from_column_id, to_column_id, from_position, to_position, moved_by, moved_at, time_in_from_column_seconds FROM kanban_card_moves WHERE card_id = ? ORDER BY moved_at DESC"
        )
        .bind(card_id.to_string())
        .fetch_all(pool).await?;

        Ok(rows.into_iter().filter_map(|r| {
            Some(KanbanCardMove {
                id: r.try_get::<String, _>("id").ok()?.parse().ok()?,
                card_id: r.try_get::<String, _>("card_id").ok()?.parse().ok()?,
                from_column_id: r.try_get::<String, _>("from_column_id").ok()?.parse().ok()?,
                to_column_id: r.try_get::<String, _>("to_column_id").ok()?.parse().ok()?,
                from_position: r.try_get("from_position").ok()?,
                to_position: r.try_get("to_position").ok()?,
                moved_by: r.try_get::<String, _>("moved_by").ok()?.parse().ok()?,
                moved_at: r.try_get::<String, _>("moved_at").ok()?.parse().ok()?,
                time_in_from_column_seconds: r.try_get("time_in_from_column_seconds").ok()?,
            })
        }).collect())
    }

    async fn create_comment(&self, pool: &SqlitePool, comment: &KanbanCardComment) -> Result<KanbanCardComment> {
        sqlx::query(
            "INSERT INTO kanban_card_comments (id, card_id, author_id, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(comment.id.to_string())
        .bind(comment.card_id.to_string())
        .bind(comment.author_id.to_string())
        .bind(&comment.content)
        .bind(comment.created_at.to_rfc3339())
        .bind(comment.updated_at.map(|d| d.to_rfc3339()))
        .execute(pool).await?;
        Ok(comment.clone())
    }

    async fn list_comments(&self, pool: &SqlitePool, card_id: Uuid) -> Result<Vec<KanbanCardComment>> {
        let rows = sqlx::query(
            "SELECT id, card_id, author_id, content, created_at, updated_at FROM kanban_card_comments WHERE card_id = ? ORDER BY created_at"
        )
        .bind(card_id.to_string())
        .fetch_all(pool).await?;

        Ok(rows.into_iter().filter_map(|r| {
            Some(KanbanCardComment {
                id: r.try_get::<String, _>("id").ok()?.parse().ok()?,
                card_id: r.try_get::<String, _>("card_id").ok()?.parse().ok()?,
                author_id: r.try_get::<String, _>("author_id").ok()?.parse().ok()?,
                content: r.try_get("content").ok()?,
                created_at: r.try_get::<String, _>("created_at").ok()?.parse().ok()?,
                updated_at: r.try_get::<Option<String>, _>("updated_at").ok()?.and_then(|s| s.parse().ok()),
            })
        }).collect())
    }

    async fn delete_comment(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM kanban_card_comments WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_checklist(&self, pool: &SqlitePool, checklist: &KanbanCardChecklist) -> Result<KanbanCardChecklist> {
        sqlx::query(
            "INSERT INTO kanban_card_checklists (id, card_id, title, position, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(checklist.id.to_string())
        .bind(checklist.card_id.to_string())
        .bind(&checklist.title)
        .bind(checklist.position)
        .bind(checklist.created_at.to_rfc3339())
        .execute(pool).await?;

        for item in &checklist.items {
            sqlx::query(
                "INSERT INTO kanban_card_checklist_items (id, checklist_id, content, position, completed, completed_by, completed_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(item.id.to_string())
            .bind(item.checklist_id.to_string())
            .bind(&item.content)
            .bind(item.position)
            .bind(item.completed as i32)
            .bind(item.completed_by.map(|id| id.to_string()))
            .bind(item.completed_at.map(|d| d.to_rfc3339()))
            .bind(item.created_at.to_rfc3339())
            .execute(pool).await?;
        }

        Ok(checklist.clone())
    }

    async fn list_checklists(&self, pool: &SqlitePool, card_id: Uuid) -> Result<Vec<KanbanCardChecklist>> {
        let checklist_rows = sqlx::query(
            "SELECT id, card_id, title, position, created_at FROM kanban_card_checklists WHERE card_id = ? ORDER BY position"
        )
        .bind(card_id.to_string())
        .fetch_all(pool).await?;

        let mut checklists = Vec::new();
        for cr in checklist_rows {
            let checklist_id: Uuid = cr.try_get::<String, _>("id")?.parse()?;
            let item_rows = sqlx::query(
                "SELECT id, checklist_id, content, position, completed, completed_by, completed_at, created_at FROM kanban_card_checklist_items WHERE checklist_id = ? ORDER BY position"
            )
            .bind(checklist_id.to_string())
            .fetch_all(pool).await?;

            let items: Vec<KanbanCardChecklistItem> = item_rows.into_iter().filter_map(|r| {
                Some(KanbanCardChecklistItem {
                    id: r.try_get::<String, _>("id").ok()?.parse().ok()?,
                    checklist_id: r.try_get::<String, _>("checklist_id").ok()?.parse().ok()?,
                    content: r.try_get("content").ok()?,
                    position: r.try_get("position").ok()?,
                    completed: r.try_get::<i32, _>("completed").ok()? == 1,
                    completed_by: r.try_get::<Option<String>, _>("completed_by").ok()?.and_then(|s| s.parse().ok()),
                    completed_at: r.try_get::<Option<String>, _>("completed_at").ok()?.and_then(|s| s.parse().ok()),
                    created_at: r.try_get::<String, _>("created_at").ok()?.parse().ok()?,
                })
            }).collect();

            checklists.push(KanbanCardChecklist {
                id: checklist_id,
                card_id: cr.try_get::<String, _>("card_id")?.parse()?,
                title: cr.try_get("title")?,
                position: cr.try_get("position")?,
                items,
                created_at: cr.try_get::<String, _>("created_at")?.parse()?,
            });
        }

        Ok(checklists)
    }

    async fn update_checklist_item(&self, pool: &SqlitePool, item: &KanbanCardChecklistItem) -> Result<KanbanCardChecklistItem> {
        sqlx::query(
            "UPDATE kanban_card_checklist_items SET completed = ?, completed_by = ?, completed_at = ? WHERE id = ?"
        )
        .bind(item.completed as i32)
        .bind(item.completed_by.map(|id| id.to_string()))
        .bind(item.completed_at.map(|d| d.to_rfc3339()))
        .bind(item.id.to_string())
        .execute(pool).await?;
        Ok(item.clone())
    }

    async fn log_activity(&self, pool: &SqlitePool, log: &KanbanActivityLog) -> Result<KanbanActivityLog> {
        sqlx::query(
            "INSERT INTO kanban_activity_logs (id, board_id, card_id, action_type, actor_id, description, old_value, new_value, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(log.id.to_string())
        .bind(log.board_id.to_string())
        .bind(log.card_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&log.action_type)?)
        .bind(log.actor_id.to_string())
        .bind(&log.description)
        .bind(&log.old_value)
        .bind(&log.new_value)
        .bind(log.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(log.clone())
    }

    async fn list_activities(&self, pool: &SqlitePool, board_id: Uuid, limit: i32) -> Result<Vec<KanbanActivityLog>> {
        let rows = sqlx::query(
            "SELECT id, board_id, card_id, action_type, actor_id, description, old_value, new_value, created_at FROM kanban_activity_logs WHERE board_id = ? ORDER BY created_at DESC LIMIT ?"
        )
        .bind(board_id.to_string())
        .bind(limit)
        .fetch_all(pool).await?;

        Ok(rows.into_iter().filter_map(|r| {
            Some(KanbanActivityLog {
                id: r.try_get::<String, _>("id").ok()?.parse().ok()?,
                board_id: r.try_get::<String, _>("board_id").ok()?.parse().ok()?,
                card_id: r.try_get::<Option<String>, _>("card_id").ok()?.and_then(|s| s.parse().ok()),
                action_type: serde_json::from_str(&r.try_get::<String, _>("action_type").ok()?).ok()?,
                actor_id: r.try_get::<String, _>("actor_id").ok()?.parse().ok()?,
                description: r.try_get("description").ok()?,
                old_value: r.try_get("old_value").ok()?,
                new_value: r.try_get("new_value").ok()?,
                created_at: r.try_get::<String, _>("created_at").ok()?.parse().ok()?,
            })
        }).collect())
    }

    async fn count_cards_in_column(&self, pool: &SqlitePool, column_id: Uuid) -> Result<i32> {
        let row = sqlx::query(
            "SELECT COUNT(*) as count FROM kanban_cards WHERE column_id = ?"
        )
        .bind(column_id.to_string())
        .fetch_one(pool).await?;
        
        Ok(row.try_get("count")?)
    }

    async fn create_wip_violation(&self, pool: &SqlitePool, violation: &KanbanWipViolation) -> Result<KanbanWipViolation> {
        sqlx::query(
            "INSERT INTO kanban_wip_violations (id, board_id, column_id, current_count, wip_limit, violated_at, resolved_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(violation.id.to_string())
        .bind(violation.board_id.to_string())
        .bind(violation.column_id.to_string())
        .bind(violation.current_count)
        .bind(violation.wip_limit)
        .bind(violation.violated_at.to_rfc3339())
        .bind(violation.resolved_at.map(|d| d.to_rfc3339()))
        .bind(violation.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(violation.clone())
    }

    async fn resolve_wip_violation(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE kanban_wip_violations SET resolved_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }
}
