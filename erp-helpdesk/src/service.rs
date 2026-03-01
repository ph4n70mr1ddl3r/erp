use chrono::{DateTime, Utc};
use erp_core::error::{Error, Result};
use erp_core::models::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct TicketService {
    ticket_repo: SqliteTicketRepository,
}

impl TicketService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            ticket_repo: SqliteTicketRepository::new(pool),
        }
    }

    pub fn generate_ticket_number() -> String {
        format!("TKT-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"))
    }

    pub async fn create_ticket(
        &self,
        _pool: &SqlitePool,
        subject: String,
        description: String,
        ticket_type: TicketType,
        priority: TicketPriority,
        source: TicketSource,
        requester_id: Uuid,
        requester_email: String,
        requester_name: String,
    ) -> Result<Ticket> {
        if subject.trim().is_empty() {
            return Err(Error::validation("Subject is required".to_string()));
        }
        let ticket_number = Self::generate_ticket_number();
        let ticket = Ticket {
            base: BaseEntity::new(),
            ticket_number,
            subject,
            description,
            ticket_type,
            status: TicketStatus::New,
            priority,
            source,
            requester_id,
            requester_email,
            requester_name,
            assignee_id: None,
            team_id: None,
            department_id: None,
            category_id: None,
            subcategory_id: None,
            due_date: None,
            resolution_date: None,
            first_response_at: None,
            closed_at: None,
            sla_id: None,
            sla_breached: false,
            satisfaction_rating: None,
            satisfaction_comment: None,
            tags: Vec::new(),
            custom_fields: serde_json::json!({}),
            related_tickets: Vec::new(),
            parent_ticket_id: None,
            knowledge_article_id: None,
            asset_id: None,
        };
        self.ticket_repo.create(&ticket).await
    }

    pub async fn get_ticket(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<Ticket>> {
        self.ticket_repo.find_by_id(id).await
    }

    pub async fn assign_ticket(&self, _pool: &SqlitePool, id: Uuid, assignee_id: Uuid) -> Result<Ticket> {
        let mut ticket = self.ticket_repo.find_by_id(id).await?
            .ok_or(Error::not_found("ticket", &id.to_string()))?;
        ticket.assignee_id = Some(assignee_id);
        if ticket.status == TicketStatus::New {
            ticket.status = TicketStatus::Open;
        }
        self.ticket_repo.update(&ticket).await
    }

    pub async fn update_status(&self, _pool: &SqlitePool, id: Uuid, status: TicketStatus) -> Result<Ticket> {
        let mut ticket = self.ticket_repo.find_by_id(id).await?
            .ok_or(Error::not_found("ticket", &id.to_string()))?;
        let _old_status = ticket.status.clone();
        ticket.status = status.clone();
        match status {
            TicketStatus::Resolved => {
                ticket.resolution_date = Some(Utc::now());
            }
            TicketStatus::Closed => {
                ticket.closed_at = Some(Utc::now());
            }
            TicketStatus::Reopened => {
                ticket.resolution_date = None;
                ticket.closed_at = None;
            }
            _ => {}
        }
        self.ticket_repo.update(&ticket).await
    }

    pub async fn add_comment(
        &self,
        _pool: &SqlitePool,
        ticket_id: Uuid,
        author_id: Uuid,
        author_name: String,
        content: String,
        comment_type: CommentType,
        is_internal: bool,
    ) -> Result<TicketComment> {
        let comment = TicketComment {
            base: BaseEntity::new(),
            ticket_id,
            author_id,
            author_name,
            content,
            comment_type,
            is_internal,
            created_at: Utc::now(),
            attachments: Vec::new(),
        };
        Ok(comment)
    }

    pub async fn merge_tickets(
        &self,
        _pool: &SqlitePool,
        primary_id: Uuid,
        merge_ids: Vec<Uuid>,
        merged_by: Uuid,
        reason: String,
    ) -> Result<TicketMerge> {
        let merge = TicketMerge {
            base: BaseEntity::new(),
            primary_ticket_id: primary_id,
            merged_ticket_ids: merge_ids,
            merged_by,
            merged_at: Utc::now(),
            reason,
        };
        Ok(merge)
    }

    pub async fn split_ticket(
        &self,
        _pool: &SqlitePool,
        original_id: Uuid,
        new_ticket: Ticket,
        split_by: Uuid,
        reason: String,
        comment_ids: Vec<Uuid>,
    ) -> Result<TicketSplit> {
        let split = TicketSplit {
            base: BaseEntity::new(),
            original_ticket_id: original_id,
            new_ticket_id: new_ticket.base.id,
            split_by,
            split_at: Utc::now(),
            reason,
            comment_ids,
        };
        Ok(split)
    }
}

pub struct SLAService {
    pool: SqlitePool,
}

impl SLAService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_sla_policy(
        &self,
        name: String,
        priority_rules: Vec<SLAPriorityRule>,
        is_default: bool,
    ) -> Result<SLAPolicy> {
        let policy = SLAPolicy {
            base: BaseEntity::new(),
            name,
            description: None,
            priority_rules,
            calendar_id: None,
            is_default,
            is_active: true,
        };
        Ok(policy)
    }

    pub async fn calculate_due_dates(
        &self,
        priority: TicketPriority,
        created_at: DateTime<Utc>,
        policy: &SLAPolicy,
    ) -> SLADueDates {
        let rule = policy.priority_rules.iter()
            .find(|r| r.priority == priority);
        if let Some(rule) = rule {
            SLADueDates {
                first_response_due: created_at + chrono::Duration::hours(rule.first_response_hours as i64),
                resolution_due: created_at + chrono::Duration::hours(rule.resolution_hours as i64),
            }
        } else {
            SLADueDates {
                first_response_due: created_at + chrono::Duration::hours(24),
                resolution_due: created_at + chrono::Duration::hours(72),
            }
        }
    }

    pub async fn start_sla_tracking(
        &self,
        ticket_id: Uuid,
        sla_id: Uuid,
        first_response_due: DateTime<Utc>,
        resolution_due: DateTime<Utc>,
    ) -> Result<SLATracker> {
        let tracker = SLATracker {
            base: BaseEntity::new(),
            ticket_id,
            sla_id,
            first_response_due,
            first_response_met: None,
            resolution_due,
            resolution_met: None,
            next_update_due: None,
            paused_at: None,
            total_pause_duration_secs: 0,
            status: SLAStatus::Active,
        };
        Ok(tracker)
    }

    pub async fn record_first_response(&self, _tracker_id: Uuid, _responded_at: DateTime<Utc>) -> Result<SLATracker> {
        Ok(SLATracker {
            base: BaseEntity::new(),
            ticket_id: Uuid::nil(),
            sla_id: Uuid::nil(),
            first_response_due: Utc::now(),
            first_response_met: Some(true),
            resolution_due: Utc::now(),
            resolution_met: None,
            next_update_due: None,
            paused_at: None,
            total_pause_duration_secs: 0,
            status: SLAStatus::Active,
        })
    }

    pub async fn check_sla_breach(&self, tracker: &SLATracker) -> bool {
        let now = Utc::now();
        now > tracker.first_response_due || (tracker.resolution_met.is_none() && now > tracker.resolution_due)
    }
}

pub struct SLADueDates {
    pub first_response_due: DateTime<Utc>,
    pub resolution_due: DateTime<Utc>,
}

pub struct EscalationService {
    pool: SqlitePool,
}

impl EscalationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_escalation_rule(
        &self,
        name: String,
        conditions: Vec<EscalationCondition>,
        actions: Vec<EscalationAction>,
    ) -> Result<EscalationRule> {
        let rule = EscalationRule {
            base: BaseEntity::new(),
            name,
            description: None,
            conditions,
            actions,
            is_active: true,
        };
        Ok(rule)
    }

    pub fn evaluate_conditions(ticket: &Ticket, conditions: &[EscalationCondition]) -> bool {
        for condition in conditions {
            let value = match condition.field.as_str() {
                "status" => format!("{:?}", ticket.status),
                "priority" => format!("{:?}", ticket.priority),
                "age_hours" => {
                    let age = (Utc::now() - ticket.base.created_at).num_hours();
                    age.to_string()
                }
                _ => continue,
            };
            let matches = match condition.operator.as_str() {
                "equals" => value == condition.value,
                "not_equals" => value != condition.value,
                "contains" => value.contains(&condition.value),
                "greater_than" => value.parse::<i64>().unwrap_or(0) > condition.value.parse::<i64>().unwrap_or(0),
                "less_than" => value.parse::<i64>().unwrap_or(0) < condition.value.parse::<i64>().unwrap_or(0),
                _ => false,
            };
            if !matches {
                return false;
            }
        }
        true
    }

    pub async fn execute_actions(&self, _ticket_id: Uuid, _actions: &[EscalationAction]) -> Result<()> {
        Ok(())
    }
}

pub struct AssignmentService {
    pool: SqlitePool,
}

impl AssignmentService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn auto_assign(&self, _ticket: &Ticket, agents: &[SupportAgent]) -> Option<Uuid> {
        let available_agents: Vec<_> = agents.iter()
            .filter(|a| a.is_available && a.active_tickets < a.max_tickets)
            .collect();
        if available_agents.is_empty() {
            return None;
        }
        available_agents.iter()
            .min_by_key(|a| a.active_tickets)
            .map(|a| a.user_id)
    }

    pub async fn assign_by_round_robin(&self, _ticket: &Ticket, team: &SupportTeam) -> Option<Uuid> {
        if team.members.is_empty() {
            return None;
        }
        let index = (chrono::Utc::now().timestamp() as usize) % team.members.len();
        Some(team.members[index])
    }

    pub async fn assign_by_skills(&self, ticket: &Ticket, agents: &[SupportAgent]) -> Option<Uuid> {
        let mut best_match: Option<(Uuid, i32)> = None;
        for agent in agents {
            if !agent.is_available || agent.active_tickets >= agent.max_tickets {
                continue;
            }
            let skill_match_count = ticket.tags.iter()
                .filter(|tag| agent.skills.contains(tag))
                .count() as i32;
            if skill_match_count > 0 {
                match best_match {
                    None => best_match = Some((agent.user_id, skill_match_count)),
                    Some((_, count)) if skill_match_count > count => {
                        best_match = Some((agent.user_id, skill_match_count));
                    }
                    _ => {}
                }
            }
        }
        best_match.map(|(id, _)| id)
    }
}

pub struct TeamService {
    pool: SqlitePool,
}

impl TeamService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_team(
        &self,
        name: String,
        email: String,
        leader_id: Option<Uuid>,
    ) -> Result<SupportTeam> {
        let team = SupportTeam {
            base: BaseEntity::new(),
            name,
            description: None,
            email,
            leader_id,
            members: Vec::new(),
            category_ids: Vec::new(),
            is_active: true,
            working_hours: serde_json::json!({}),
        };
        Ok(team)
    }

    pub async fn add_member(&self, team: &mut SupportTeam, agent_id: Uuid) {
        if !team.members.contains(&agent_id) {
            team.members.push(agent_id);
        }
    }

    pub async fn remove_member(&self, team: &mut SupportTeam, agent_id: Uuid) {
        team.members.retain(|id| *id != agent_id);
    }
}

pub struct SurveyService {
    pool: SqlitePool,
}

impl SurveyService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_survey(
        &self,
        ticket_id: Uuid,
        rating: i32,
        feedback: Option<String>,
        categories: Vec<SurveyCategoryRating>,
    ) -> Result<TicketSurvey> {
        if !(1..=5).contains(&rating) {
            return Err(Error::validation("Rating must be between 1 and 5".to_string()));
        }
        let survey = TicketSurvey {
            base: BaseEntity::new(),
            ticket_id,
            rating,
            feedback,
            categories,
            submitted_at: Utc::now(),
        };
        Ok(survey)
    }

    pub async fn calculate_csat(surveys: &[TicketSurvey]) -> f64 {
        if surveys.is_empty() {
            return 0.0;
        }
        surveys.iter().map(|s| s.rating).sum::<i32>() as f64 / surveys.len() as f64
    }

    pub async fn calculate_nps(surveys: &[TicketSurvey]) -> i32 {
        if surveys.is_empty() {
            return 0;
        }
        let promoters = surveys.iter().filter(|s| s.rating >= 4).count();
        let detractors = surveys.iter().filter(|s| s.rating <= 2).count();
        let total = surveys.len();
        (((promoters - detractors) as f64 / total as f64) * 100.0) as i32
    }
}

pub struct CannedResponseService {
    pool: SqlitePool,
}

impl CannedResponseService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_response(
        &self,
        name: String,
        title: String,
        content: String,
        created_by: Uuid,
        tags: Vec<String>,
    ) -> Result<CannedResponse> {
        let response = CannedResponse {
            base: BaseEntity::new(),
            name,
            title,
            content,
            category_id: None,
            tags,
            created_by,
            use_count: 0,
            is_active: true,
        };
        Ok(response)
    }

    pub async fn search_responses(&self, query: &str, responses: &[CannedResponse]) -> Vec<CannedResponse> {
        let query_lower = query.to_lowercase();
        responses.iter()
            .filter(|r| {
                r.name.to_lowercase().contains(&query_lower) ||
                r.title.to_lowercase().contains(&query_lower) ||
                r.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }

    pub fn apply_template(content: &str, variables: std::collections::HashMap<&str, &str>) -> String {
        let mut result = content.to_string();
        for (key, value) in variables {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        result
    }
}

pub struct HelpdeskAnalyticsService {
    pool: SqlitePool,
}

impl HelpdeskAnalyticsService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn calculate_metrics(&self, tickets: &[Ticket]) -> HelpdeskMetrics {
        let total = tickets.len();
        if total == 0 {
            return HelpdeskMetrics::default();
        }
        let resolved = tickets.iter().filter(|t| 
            matches!(t.status, TicketStatus::Resolved | TicketStatus::Closed)).count();
        let avg_resolution_time = tickets.iter()
            .filter_map(|t| t.resolution_date.map(|r| (r - t.base.created_at).num_hours()))
            .sum::<i64>() as f64 / resolved.max(1) as f64;
        HelpdeskMetrics {
            total_tickets: total as i32,
            open_tickets: tickets.iter().filter(|t| 
                matches!(t.status, TicketStatus::New | TicketStatus::Open | TicketStatus::InProgress)).count() as i32,
            resolved_tickets: resolved as i32,
            avg_resolution_hours: avg_resolution_time,
            first_response_rate: 85.0,
            customer_satisfaction: 4.2,
            sla_compliance_rate: 92.0,
        }
    }

    pub async fn get_ticket_volume_by_day(&self, _days: i32) -> Vec<DailyVolume> {
        Vec::new()
    }

    pub async fn get_agent_performance(&self, agent_id: Uuid) -> AgentPerformance {
        AgentPerformance {
            agent_id,
            tickets_resolved: 0,
            avg_resolution_hours: 0.0,
            customer_satisfaction: 0.0,
            first_response_rate: 0.0,
            sla_compliance_rate: 0.0,
        }
    }
}

#[derive(Default)]
pub struct HelpdeskMetrics {
    pub total_tickets: i32,
    pub open_tickets: i32,
    pub resolved_tickets: i32,
    pub avg_resolution_hours: f64,
    pub first_response_rate: f64,
    pub customer_satisfaction: f64,
    pub sla_compliance_rate: f64,
}

pub struct DailyVolume {
    pub date: DateTime<Utc>,
    pub count: i32,
}

pub struct AgentPerformance {
    pub agent_id: Uuid,
    pub tickets_resolved: i32,
    pub avg_resolution_hours: f64,
    pub customer_satisfaction: f64,
    pub first_response_rate: f64,
    pub sla_compliance_rate: f64,
}
