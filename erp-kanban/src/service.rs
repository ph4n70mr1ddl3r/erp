use crate::models::*;
use crate::repository::{KanbanRepository, SqliteKanbanRepository};
use anyhow::{Context, Result};
use chrono::Utc;
use erp_core::{BaseEntity, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct KanbanService {
    repo: SqliteKanbanRepository,
}

impl KanbanService {
    pub fn new() -> Self {
        Self { repo: SqliteKanbanRepository }
    }

    pub async fn create_board(&self, pool: &SqlitePool, req: CreateBoardRequest, user_id: Uuid) -> Result<KanbanBoard> {
        let now = Utc::now();
        let board_id = Uuid::new_v4();
        
        let columns: Vec<KanbanColumn> = req.columns.into_iter().enumerate().map(|(idx, c)| KanbanColumn {
            id: Uuid::new_v4(),
            board_id,
            name: c.name,
            position: idx as i32,
            wip_limit: c.wip_limit,
            is_done_column: c.is_done_column,
            is_backlog: c.is_backlog,
            color: c.color,
            auto_assign_on_move: None,
            created_at: now,
        }).collect();

        let board = KanbanBoard {
            base: BaseEntity::new_with_id(board_id),
            name: req.name,
            description: req.description,
            board_type: req.board_type,
            team_id: req.team_id,
            project_id: req.project_id,
            columns: columns.clone(),
            swimlane_type: KanbanSwimlaneType::None,
            swimlanes: vec![],
            default_wip_limit: req.default_wip_limit,
            allow_card_reordering: true,
            show_card_count: true,
            show_wip_limits: true,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        };

        let created = self.repo.create_board(pool, &board).await?;

        self.repo.log_activity(pool, &KanbanActivityLog {
            id: Uuid::new_v4(),
            board_id: created.base.id,
            card_id: None,
            action_type: KanbanActionType::CardCreated,
            actor_id: user_id,
            description: format!("Created board '{}'", created.name),
            old_value: None,
            new_value: None,
            created_at: now,
        }).await?;

        Ok(created)
    }

    pub async fn get_board(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KanbanBoard>> {
        self.repo.get_board(pool, id).await
    }

    pub async fn list_boards(&self, pool: &SqlitePool, page: i32, limit: i32) -> Result<Vec<KanbanBoard>> {
        let offset = (page - 1) * limit;
        self.repo.list_boards(pool, limit, offset).await
    }

    pub async fn update_board(&self, pool: &SqlitePool, board: &KanbanBoard) -> Result<KanbanBoard> {
        self.repo.update_board(pool, board).await
    }

    pub async fn delete_board(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete_board(pool, id).await
    }

    pub async fn create_card(&self, pool: &SqlitePool, req: CreateCardRequest, user_id: Uuid) -> Result<KanbanCard> {
        let now = Utc::now();
        
        let existing_cards = self.repo.list_cards_by_column(pool, req.column_id).await?;
        let next_position = existing_cards.len() as i32;

        let card = KanbanCard {
            base: BaseEntity::new(),
            board_id: req.board_id,
            column_id: req.column_id,
            swimlane_id: req.swimlane_id,
            card_type: req.card_type,
            title: req.title.clone(),
            description: req.description,
            priority: req.priority,
            position: next_position,
            assignee_ids: req.assignee_ids,
            reporter_id: Some(user_id),
            due_date: req.due_date,
            start_date: None,
            completed_date: None,
            estimated_hours: req.estimated_hours,
            actual_hours: None,
            story_points: req.story_points,
            tags: req.tags,
            external_ref_type: req.external_ref_type,
            external_ref_id: req.external_ref_id,
            blocked: false,
            blocked_reason: None,
            parent_card_id: None,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        };

        let created = self.repo.create_card(pool, &card).await?;

        self.repo.log_activity(pool, &KanbanActivityLog {
            id: Uuid::new_v4(),
            board_id: req.board_id,
            card_id: Some(created.base.id),
            action_type: KanbanActionType::CardCreated,
            actor_id: user_id,
            description: format!("Created card '{}'", req.title),
            old_value: None,
            new_value: None,
            created_at: now,
        }).await?;

        self.check_wip_limit(pool, req.board_id, req.column_id).await?;

        Ok(created)
    }

    pub async fn get_card(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KanbanCard>> {
        self.repo.get_card(pool, id).await
    }

    pub async fn list_cards_by_board(&self, pool: &SqlitePool, board_id: Uuid) -> Result<Vec<KanbanCard>> {
        self.repo.list_cards_by_board(pool, board_id).await
    }

    pub async fn move_card(&self, pool: &SqlitePool, req: MoveCardRequest, user_id: Uuid) -> Result<KanbanCard> {
        let card = self.repo.get_card(pool, req.card_id).await?
            .context("Card not found")?;

        let from_column_id = card.column_id;
        let from_position = card.position;
        let now = Utc::now();

        let moves = self.repo.get_card_moves(pool, req.card_id).await?;
        let time_in_column = if let Some(last_move) = moves.first() {
            Some((now - last_move.moved_at).num_seconds())
        } else {
            Some((now - card.created_at).num_seconds())
        };

        let card_move = KanbanCardMove {
            id: Uuid::new_v4(),
            card_id: req.card_id,
            from_column_id,
            to_column_id: req.to_column_id,
            from_position,
            to_position: req.to_position,
            moved_by: user_id,
            moved_at: now,
            time_in_from_column_seconds: time_in_column,
        };
        self.repo.move_card(pool, &card_move).await?;

        let mut updated_card = card.clone();
        updated_card.column_id = req.to_column_id;
        updated_card.position = req.to_position;
        updated_card.updated_at = now;

        let board = self.repo.get_board(pool, card.board_id).await?.context("Board not found")?;
        if let Some(target_col) = board.columns.iter().find(|c| c.id == req.to_column_id) {
            if target_col.is_done_column && updated_card.completed_date.is_none() {
                updated_card.completed_date = Some(now);
            }
        }

        let updated = self.repo.update_card(pool, &updated_card).await?;

        self.repo.log_activity(pool, &KanbanActivityLog {
            id: Uuid::new_v4(),
            board_id: card.board_id,
            card_id: Some(req.card_id),
            action_type: KanbanActionType::CardMoved,
            actor_id: user_id,
            description: format!("Moved card from column to new column"),
            old_value: Some(from_column_id.to_string()),
            new_value: Some(req.to_column_id.to_string()),
            created_at: now,
        }).await?;

        self.check_wip_limit(pool, card.board_id, from_column_id).await?;
        self.check_wip_limit(pool, card.board_id, req.to_column_id).await?;

        Ok(updated)
    }

    pub async fn update_card(&self, pool: &SqlitePool, card: &KanbanCard, user_id: Uuid) -> Result<KanbanCard> {
        let updated = self.repo.update_card(pool, card).await?;

        self.repo.log_activity(pool, &KanbanActivityLog {
            id: Uuid::new_v4(),
            board_id: card.board_id,
            card_id: Some(card.base.id),
            action_type: KanbanActionType::CardUpdated,
            actor_id: user_id,
            description: format!("Updated card '{}'", card.title),
            old_value: None,
            new_value: None,
            created_at: Utc::now(),
        }).await?;

        Ok(updated)
    }

    pub async fn delete_card(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<()> {
        let card = self.repo.get_card(pool, id).await?.context("Card not found")?;

        self.repo.log_activity(pool, &KanbanActivityLog {
            id: Uuid::new_v4(),
            board_id: card.board_id,
            card_id: Some(id),
            action_type: KanbanActionType::CardDeleted,
            actor_id: user_id,
            description: format!("Deleted card '{}'", card.title),
            old_value: None,
            new_value: None,
            created_at: Utc::now(),
        }).await?;

        self.repo.delete_card(pool, id).await
    }

    pub async fn add_comment(&self, pool: &SqlitePool, card_id: Uuid, content: String, author_id: Uuid) -> Result<KanbanCardComment> {
        let card = self.repo.get_card(pool, card_id).await?.context("Card not found")?;
        let now = Utc::now();

        let comment = KanbanCardComment {
            id: Uuid::new_v4(),
            card_id,
            author_id,
            content: content.clone(),
            created_at: now,
            updated_at: None,
        };

        let created = self.repo.create_comment(pool, &comment).await?;

        self.repo.log_activity(pool, &KanbanActivityLog {
            id: Uuid::new_v4(),
            board_id: card.board_id,
            card_id: Some(card_id),
            action_type: KanbanActionType::CommentAdded,
            actor_id: author_id,
            description: format!("Added comment: {}", content),
            old_value: None,
            new_value: None,
            created_at: now,
        }).await?;

        Ok(created)
    }

    pub async fn list_comments(&self, pool: &SqlitePool, card_id: Uuid) -> Result<Vec<KanbanCardComment>> {
        self.repo.list_comments(pool, card_id).await
    }

    pub async fn add_checklist(&self, pool: &SqlitePool, card_id: Uuid, title: String) -> Result<KanbanCardChecklist> {
        let now = Utc::now();
        let checklist = KanbanCardChecklist {
            id: Uuid::new_v4(),
            card_id,
            title,
            position: 0,
            items: vec![],
            created_at: now,
        };
        self.repo.create_checklist(pool, &checklist).await
    }

    pub async fn list_checklists(&self, pool: &SqlitePool, card_id: Uuid) -> Result<Vec<KanbanCardChecklist>> {
        self.repo.list_checklists(pool, card_id).await
    }

    pub async fn toggle_checklist_item(&self, pool: &SqlitePool, item_id: Uuid, completed: bool, user_id: Uuid) -> Result<KanbanCardChecklistItem> {
        let now = Utc::now();
        let mut item = KanbanCardChecklistItem {
            id: item_id,
            checklist_id: Uuid::nil(),
            content: String::new(),
            position: 0,
            completed,
            completed_by: if completed { Some(user_id) } else { None },
            completed_at: if completed { Some(now) } else { None },
            created_at: now,
        };
        self.repo.update_checklist_item(pool, &item).await
    }

    pub async fn block_card(&self, pool: &SqlitePool, card_id: Uuid, reason: String, user_id: Uuid) -> Result<KanbanCard> {
        let mut card = self.repo.get_card(pool, card_id).await?.context("Card not found")?;
        card.blocked = true;
        card.blocked_reason = Some(reason.clone());
        card.updated_at = Utc::now();

        let updated = self.repo.update_card(pool, &card).await?;

        self.repo.log_activity(pool, &KanbanActivityLog {
            id: Uuid::new_v4(),
            board_id: card.board_id,
            card_id: Some(card_id),
            action_type: KanbanActionType::CardBlocked,
            actor_id: user_id,
            description: format!("Blocked card: {}", reason),
            old_value: None,
            new_value: None,
            created_at: Utc::now(),
        }).await?;

        Ok(updated)
    }

    pub async fn unblock_card(&self, pool: &SqlitePool, card_id: Uuid, user_id: Uuid) -> Result<KanbanCard> {
        let mut card = self.repo.get_card(pool, card_id).await?.context("Card not found")?;
        card.blocked = false;
        card.blocked_reason = None;
        card.updated_at = Utc::now();

        let updated = self.repo.update_card(pool, &card).await?;

        self.repo.log_activity(pool, &KanbanActivityLog {
            id: Uuid::new_v4(),
            board_id: card.board_id,
            card_id: Some(card_id),
            action_type: KanbanActionType::CardUnblocked,
            actor_id: user_id,
            description: "Unblocked card".to_string(),
            old_value: None,
            new_value: None,
            created_at: Utc::now(),
        }).await?;

        Ok(updated)
    }

    pub async fn list_activities(&self, pool: &SqlitePool, board_id: Uuid, limit: i32) -> Result<Vec<KanbanActivityLog>> {
        self.repo.list_activities(pool, board_id, limit).await
    }

    pub async fn get_board_summary(&self, pool: &SqlitePool, board_id: Uuid) -> Result<BoardSummary> {
        let board = self.repo.get_board(pool, board_id).await?.context("Board not found")?;
        let cards = self.repo.list_cards_by_board(pool, board_id).await?;

        let mut cards_by_column = Vec::new();
        let mut wip_violations = 0;
        let mut overdue_count = 0;
        let mut blocked_count = 0;
        let now = Utc::now();

        for col in &board.columns {
            let col_cards: Vec<_> = cards.iter().filter(|c| c.column_id == col.id).collect();
            let count = col_cards.len() as i32;
            let is_over = col.wip_limit.map(|limit| count > limit).unwrap_or(false);
            if is_over {
                wip_violations += 1;
            }

            for card in &col_cards {
                if card.blocked {
                    blocked_count += 1;
                }
                if let Some(due) = card.due_date {
                    if due < now && card.completed_date.is_none() {
                        overdue_count += 1;
                    }
                }
            }

            cards_by_column.push(ColumnSummary {
                column_id: col.id,
                column_name: col.name.clone(),
                card_count: count,
                wip_limit: col.wip_limit,
                is_over_wip: is_over,
            });
        }

        Ok(BoardSummary {
            board_id,
            board_name: board.name,
            total_cards: cards.len() as i32,
            cards_by_column,
            wip_violations,
            overdue_cards: overdue_count,
            blocked_cards: blocked_count,
        })
    }

    async fn check_wip_limit(&self, pool: &SqlitePool, board_id: Uuid, column_id: Uuid) -> Result<()> {
        let board = self.repo.get_board(pool, board_id).await?.context("Board not found")?;
        let column = board.columns.iter().find(|c| c.id == column_id);

        if let Some(col) = column {
            if let Some(limit) = col.wip_limit {
                let count = self.repo.count_cards_in_column(pool, column_id).await?;
                if count > limit {
                    self.repo.create_wip_violation(pool, &KanbanWipViolation {
                        id: Uuid::new_v4(),
                        board_id,
                        column_id,
                        current_count: count,
                        wip_limit: limit,
                        violated_at: Utc::now(),
                        resolved_at: None,
                        created_at: Utc::now(),
                    }).await?;
                }
            }
        }

        Ok(())
    }
}
