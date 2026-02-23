use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::*;
use crate::pagination::{Paginated, Pagination};

pub struct TenantService;

impl TenantService {
    pub async fn create_tenant(
        pool: &SqlitePool,
        tenant_code: &str,
        name: &str,
        plan_type: TenantPlanType,
        max_users: i32,
        max_storage_mb: i32,
    ) -> Result<Tenant> {
        let now = Utc::now();
        let tenant = Tenant {
            id: Uuid::new_v4(),
            tenant_code: tenant_code.to_string(),
            name: name.to_string(),
            plan_type,
            max_users,
            max_storage_mb,
            settings: None,
            status: Status::Active,
            created_at: now,
            expires_at: None,
        };

        sqlx::query(
            "INSERT INTO tenants (id, tenant_code, name, plan_type, max_users, max_storage_mb, settings, status, created_at, expires_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(tenant.id.to_string())
        .bind(&tenant.tenant_code)
        .bind(&tenant.name)
        .bind(format!("{:?}", tenant.plan_type))
        .bind(tenant.max_users)
        .bind(tenant.max_storage_mb)
        .bind(&tenant.settings)
        .bind(format!("{:?}", tenant.status))
        .bind(tenant.created_at.to_rfc3339())
        .bind(tenant.expires_at.map(|d| d.to_rfc3339()))
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(tenant)
    }

    pub async fn get_tenant(pool: &SqlitePool, id: Uuid) -> Result<Tenant> {
        let row = sqlx::query_as::<_, TenantRow>(
            "SELECT id, tenant_code, name, plan_type, max_users, max_storage_mb, settings, status, created_at, expires_at
             FROM tenants WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("Tenant", &id.to_string()))?;

        Ok(row.into())
    }

    pub async fn add_user_to_tenant(
        pool: &SqlitePool,
        tenant_id: Uuid,
        user_id: Uuid,
        role: TenantUserRole,
    ) -> Result<TenantUser> {
        let now = Utc::now();
        let tenant_user = TenantUser {
            id: Uuid::new_v4(),
            tenant_id,
            user_id,
            role,
            joined_at: now,
            status: Status::Active,
        };

        sqlx::query(
            "INSERT INTO tenant_users (id, tenant_id, user_id, role, joined_at, status)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(tenant_user.id.to_string())
        .bind(tenant_user.tenant_id.to_string())
        .bind(tenant_user.user_id.to_string())
        .bind(format!("{:?}", tenant_user.role))
        .bind(tenant_user.joined_at.to_rfc3339())
        .bind(format!("{:?}", tenant_user.status))
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(tenant_user)
    }

    pub async fn remove_user(pool: &SqlitePool, tenant_id: Uuid, user_id: Uuid) -> Result<()> {
        let rows = sqlx::query(
            "DELETE FROM tenant_users WHERE tenant_id = ? AND user_id = ?"
        )
        .bind(tenant_id.to_string())
        .bind(user_id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("TenantUser", &format!("{}:{}", tenant_id, user_id)));
        }

        Ok(())
    }

    pub async fn check_tenant_limits(pool: &SqlitePool, tenant_id: Uuid) -> Result<TenantLimits> {
        let tenant = Self::get_tenant(pool, tenant_id).await?;

        let user_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tenant_users WHERE tenant_id = ? AND status = 'Active'"
        )
        .bind(tenant_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(Error::Database)?;

        let storage_used: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(file_size), 0) FROM attachments WHERE entity_id IN 
             (SELECT CAST(user_id AS TEXT) FROM tenant_users WHERE tenant_id = ?)"
        )
        .bind(tenant_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(Error::Database)?;

        Ok(TenantLimits {
            tenant_id,
            max_users: tenant.max_users,
            current_users: user_count.0 as i32,
            max_storage_mb: tenant.max_storage_mb,
            storage_used_mb: (storage_used.0 / (1024 * 1024)) as i32,
            users_exceeded: user_count.0 >= tenant.max_users as i64,
            storage_exceeded: storage_used.0 >= (tenant.max_storage_mb as i64 * 1024 * 1024),
        })
    }
}

pub struct AutomationService;

impl AutomationService {
    pub async fn create_rule(
        pool: &SqlitePool,
        name: &str,
        description: Option<&str>,
        entity_type: &str,
        trigger_event: TriggerEvent,
        conditions: Option<&str>,
        actions: &str,
        priority: i32,
        created_by: Option<Uuid>,
    ) -> Result<AutomationRule> {
        let now = Utc::now();
        let rule = AutomationRule {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            entity_type: entity_type.to_string(),
            trigger_event,
            conditions: conditions.map(|s| s.to_string()),
            actions: actions.to_string(),
            priority,
            active: true,
            created_by,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO automation_rules (id, name, description, entity_type, trigger_event, conditions, actions, priority, active, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(rule.id.to_string())
        .bind(&rule.name)
        .bind(&rule.description)
        .bind(&rule.entity_type)
        .bind(format!("{:?}", rule.trigger_event))
        .bind(&rule.conditions)
        .bind(&rule.actions)
        .bind(rule.priority)
        .bind(if rule.active { 1 } else { 0 })
        .bind(rule.created_by.map(|id| id.to_string()))
        .bind(rule.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(rule)
    }

    pub async fn get_rules(pool: &SqlitePool, entity_type: Option<&str>) -> Result<Vec<AutomationRule>> {
        let rows = if let Some(et) = entity_type {
            sqlx::query_as::<_, AutomationRuleRow>(
                "SELECT id, name, description, entity_type, trigger_event, conditions, actions, priority, active, created_by, created_at
                 FROM automation_rules WHERE entity_type = ? AND active = 1 ORDER BY priority DESC"
            )
            .bind(et)
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?
        } else {
            sqlx::query_as::<_, AutomationRuleRow>(
                "SELECT id, name, description, entity_type, trigger_event, conditions, actions, priority, active, created_by, created_at
                 FROM automation_rules WHERE active = 1 ORDER BY priority DESC"
            )
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?
        };

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update_rule(
        pool: &SqlitePool,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
        conditions: Option<&str>,
        actions: Option<&str>,
        priority: Option<i32>,
        active: Option<bool>,
    ) -> Result<AutomationRule> {
        let existing = sqlx::query_as::<_, AutomationRuleRow>(
            "SELECT id, name, description, entity_type, trigger_event, conditions, actions, priority, active, created_by, created_at
             FROM automation_rules WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("AutomationRule", &id.to_string()))?;

        let rule = AutomationRule {
            id,
            name: name.map(|s| s.to_string()).unwrap_or(existing.name),
            description: description.map(|s| s.to_string()).or(existing.description),
            entity_type: existing.entity_type,
            trigger_event: parse_trigger_event(&existing.trigger_event),
            conditions: conditions.map(|s| s.to_string()).or(existing.conditions),
            actions: actions.map(|s| s.to_string()).unwrap_or(existing.actions),
            priority: priority.unwrap_or(existing.priority as i32),
            active: active.unwrap_or(existing.active != 0),
            created_by: existing.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: parse_datetime(&existing.created_at),
        };

        sqlx::query(
            "UPDATE automation_rules SET name = ?, description = ?, conditions = ?, actions = ?, priority = ?, active = ? WHERE id = ?"
        )
        .bind(&rule.name)
        .bind(&rule.description)
        .bind(&rule.conditions)
        .bind(&rule.actions)
        .bind(rule.priority)
        .bind(if rule.active { 1 } else { 0 })
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(rule)
    }

    pub async fn trigger_rule(
        pool: &SqlitePool,
        rule_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
        trigger_data: Option<&str>,
    ) -> Result<AutomationLog> {
        let rule = sqlx::query_as::<_, AutomationRuleRow>(
            "SELECT id, name, description, entity_type, trigger_event, conditions, actions, priority, active, created_by, created_at
             FROM automation_rules WHERE id = ? AND active = 1"
        )
        .bind(rule_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("AutomationRule", &rule_id.to_string()))?;

        let now = Utc::now();
        let log = AutomationLog {
            id: Uuid::new_v4(),
            rule_id,
            entity_type: Some(entity_type.to_string()),
            entity_id: Some(entity_id),
            trigger_data: trigger_data.map(|s| s.to_string()),
            action_results: Some(rule.actions.clone()),
            success: true,
            error_message: None,
            executed_at: now,
        };

        sqlx::query(
            "INSERT INTO automation_logs (id, rule_id, entity_type, entity_id, trigger_data, action_results, success, error_message, executed_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(log.id.to_string())
        .bind(log.rule_id.to_string())
        .bind(&log.entity_type)
        .bind(log.entity_id.map(|id| id.to_string()))
        .bind(&log.trigger_data)
        .bind(&log.action_results)
        .bind(if log.success { 1 } else { 0 })
        .bind(&log.error_message)
        .bind(log.executed_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(log)
    }

    pub async fn execute_actions(
        pool: &SqlitePool,
        rule_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
        actions: &str,
    ) -> Result<AutomationLog> {
        let now = Utc::now();
        let log = AutomationLog {
            id: Uuid::new_v4(),
            rule_id,
            entity_type: Some(entity_type.to_string()),
            entity_id: Some(entity_id),
            trigger_data: None,
            action_results: Some(actions.to_string()),
            success: true,
            error_message: None,
            executed_at: now,
        };

        sqlx::query(
            "INSERT INTO automation_logs (id, rule_id, entity_type, entity_id, trigger_data, action_results, success, error_message, executed_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(log.id.to_string())
        .bind(log.rule_id.to_string())
        .bind(&log.entity_type)
        .bind(log.entity_id.map(|id| id.to_string()))
        .bind(&log.trigger_data)
        .bind(&log.action_results)
        .bind(if log.success { 1 } else { 0 })
        .bind(&log.error_message)
        .bind(log.executed_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(log)
    }

    pub async fn get_execution_logs(
        pool: &SqlitePool,
        rule_id: Option<Uuid>,
        pagination: Pagination,
    ) -> Result<Paginated<AutomationLog>> {
        let (count_query, data_query) = if let Some(rid) = rule_id {
            (
                sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM automation_logs WHERE rule_id = ?")
                    .bind(rid.to_string()),
                sqlx::query_as::<_, AutomationLogRow>(
                    "SELECT id, rule_id, entity_type, entity_id, trigger_data, action_results, success, error_message, executed_at
                     FROM automation_logs WHERE rule_id = ? ORDER BY executed_at DESC LIMIT ? OFFSET ?"
                )
                .bind(rid.to_string())
            )
        } else {
            (
                sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM automation_logs"),
                sqlx::query_as::<_, AutomationLogRow>(
                    "SELECT id, rule_id, entity_type, entity_id, trigger_data, action_results, success, error_message, executed_at
                     FROM automation_logs ORDER BY executed_at DESC LIMIT ? OFFSET ?"
                )
            )
        };

        let count = count_query.fetch_one(pool).await.map_err(Error::Database)?;

        let rows = data_query
            .bind(pagination.limit() as i64)
            .bind(pagination.offset() as i64)
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?;

        Ok(Paginated::new(
            rows.into_iter().map(|r| r.into()).collect(),
            count.0 as u64,
            pagination,
        ))
    }
}

pub struct EmailService;

impl EmailService {
    pub async fn create_template(
        pool: &SqlitePool,
        template_code: &str,
        name: &str,
        subject: &str,
        body_html: &str,
        body_text: Option<&str>,
        category: Option<&str>,
        variables: Option<&str>,
    ) -> Result<EmailTemplate> {
        let now = Utc::now();
        let template = EmailTemplate {
            id: Uuid::new_v4(),
            template_code: template_code.to_string(),
            name: name.to_string(),
            subject: subject.to_string(),
            body_html: body_html.to_string(),
            body_text: body_text.map(|s| s.to_string()),
            category: category.map(|s| s.to_string()),
            variables: variables.map(|s| s.to_string()),
            status: Status::Active,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO email_templates (id, template_code, name, subject, body_html, body_text, category, variables, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(template.id.to_string())
        .bind(&template.template_code)
        .bind(&template.name)
        .bind(&template.subject)
        .bind(&template.body_html)
        .bind(&template.body_text)
        .bind(&template.category)
        .bind(&template.variables)
        .bind(format!("{:?}", template.status))
        .bind(template.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(template)
    }

    pub async fn get_template(pool: &SqlitePool, id: Uuid) -> Result<EmailTemplate> {
        let row = sqlx::query_as::<_, EmailTemplateRow>(
            "SELECT id, template_code, name, subject, body_html, body_text, category, variables, status, created_at
             FROM email_templates WHERE id = ? AND status = 'Active'"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("EmailTemplate", &id.to_string()))?;

        Ok(row.into())
    }

    pub async fn get_template_by_code(pool: &SqlitePool, code: &str) -> Result<EmailTemplate> {
        let row = sqlx::query_as::<_, EmailTemplateRow>(
            "SELECT id, template_code, name, subject, body_html, body_text, category, variables, status, created_at
             FROM email_templates WHERE template_code = ? AND status = 'Active'"
        )
        .bind(code)
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("EmailTemplate", code))?;

        Ok(row.into())
    }

    pub async fn queue_email(
        pool: &SqlitePool,
        template_id: Option<Uuid>,
        to_address: &str,
        cc_addresses: Option<&str>,
        bcc_addresses: Option<&str>,
        subject: &str,
        body: &str,
        priority: i32,
    ) -> Result<EmailQueue> {
        let now = Utc::now();
        let email = EmailQueue {
            id: Uuid::new_v4(),
            template_id,
            to_address: to_address.to_string(),
            cc_addresses: cc_addresses.map(|s| s.to_string()),
            bcc_addresses: bcc_addresses.map(|s| s.to_string()),
            subject: subject.to_string(),
            body: body.to_string(),
            attachments: None,
            priority,
            attempts: 0,
            max_attempts: 3,
            sent_at: None,
            error_message: None,
            status: EmailStatus::Pending,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO email_queue (id, template_id, to_address, cc_addresses, bcc_addresses, subject, body, attachments, priority, attempts, max_attempts, sent_at, error_message, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(email.id.to_string())
        .bind(email.template_id.map(|id| id.to_string()))
        .bind(&email.to_address)
        .bind(&email.cc_addresses)
        .bind(&email.bcc_addresses)
        .bind(&email.subject)
        .bind(&email.body)
        .bind(&email.attachments)
        .bind(email.priority)
        .bind(email.attempts)
        .bind(email.max_attempts)
        .bind(email.sent_at.map(|d| d.to_rfc3339()))
        .bind(&email.error_message)
        .bind(format!("{:?}", email.status))
        .bind(email.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(email)
    }

    pub async fn process_queue(pool: &SqlitePool, batch_size: i32) -> Result<Vec<EmailQueue>> {
        let rows = sqlx::query_as::<_, EmailQueueRow>(
            "SELECT id, template_id, to_address, cc_addresses, bcc_addresses, subject, body, attachments, priority, attempts, max_attempts, sent_at, error_message, status, created_at
             FROM email_queue WHERE status = 'Pending' AND attempts < max_attempts
             ORDER BY priority DESC, created_at ASC LIMIT ?"
        )
        .bind(batch_size as i64)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;

        let mut processed = Vec::new();
        let now = Utc::now();

        for row in rows {
            let email_id = Uuid::parse_str(&row.id).unwrap_or_default();
            let new_attempts = row.attempts + 1;

            sqlx::query(
                "UPDATE email_queue SET attempts = ?, status = 'Sent', sent_at = ? WHERE id = ?"
            )
            .bind(new_attempts)
            .bind(now.to_rfc3339())
            .bind(row.id.clone())
            .execute(pool)
            .await
            .map_err(Error::Database)?;

            processed.push(EmailQueue {
                id: email_id,
                template_id: row.template_id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
                to_address: row.to_address,
                cc_addresses: row.cc_addresses,
                bcc_addresses: row.bcc_addresses,
                subject: row.subject,
                body: row.body,
                attachments: row.attachments,
                priority: row.priority as i32,
                attempts: new_attempts as i32,
                max_attempts: row.max_attempts as i32,
                sent_at: Some(now),
                error_message: row.error_message,
                status: EmailStatus::Sent,
                created_at: parse_datetime(&row.created_at),
            });
        }

        Ok(processed)
    }

    pub async fn log_email(
        pool: &SqlitePool,
        message_id: Option<&str>,
        direction: EmailDirection,
        from_address: &str,
        to_address: &str,
        subject: Option<&str>,
        body: Option<&str>,
        related_entity_type: Option<&str>,
        related_entity_id: Option<Uuid>,
    ) -> Result<EmailLog> {
        let now = Utc::now();
        let log = EmailLog {
            id: Uuid::new_v4(),
            message_id: message_id.map(|s| s.to_string()),
            direction,
            from_address: from_address.to_string(),
            to_address: to_address.to_string(),
            subject: subject.map(|s| s.to_string()),
            body: body.map(|s| s.to_string()),
            attachments: None,
            related_entity_type: related_entity_type.map(|s| s.to_string()),
            related_entity_id,
            sent_at: now,
            status: EmailStatus::Sent,
        };

        sqlx::query(
            "INSERT INTO email_logs (id, message_id, direction, from_address, to_address, subject, body, attachments, related_entity_type, related_entity_id, sent_at, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(log.id.to_string())
        .bind(&log.message_id)
        .bind(format!("{:?}", log.direction))
        .bind(&log.from_address)
        .bind(&log.to_address)
        .bind(&log.subject)
        .bind(&log.body)
        .bind(&log.attachments)
        .bind(&log.related_entity_type)
        .bind(log.related_entity_id.map(|id| id.to_string()))
        .bind(log.sent_at.to_rfc3339())
        .bind(format!("{:?}", log.status))
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(log)
    }
}

pub struct ReportService;

impl ReportService {
    pub async fn create_report(
        pool: &SqlitePool,
        report_code: &str,
        name: &str,
        description: Option<&str>,
        category: Option<&str>,
        data_source: &str,
        query_text: &str,
        parameters: Option<&str>,
        columns: Option<&str>,
        filters: Option<&str>,
        sorting: Option<&str>,
        grouping: Option<&str>,
        chart_type: Option<&str>,
        created_by: Option<Uuid>,
    ) -> Result<ReportDefinition> {
        let now = Utc::now();
        let report = ReportDefinition {
            id: Uuid::new_v4(),
            report_code: report_code.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            category: category.map(|s| s.to_string()),
            data_source: data_source.to_string(),
            query_text: query_text.to_string(),
            parameters: parameters.map(|s| s.to_string()),
            columns: columns.map(|s| s.to_string()),
            filters: filters.map(|s| s.to_string()),
            sorting: sorting.map(|s| s.to_string()),
            grouping: grouping.map(|s| s.to_string()),
            chart_type: chart_type.map(|s| s.to_string()),
            is_scheduled: false,
            schedule_cron: None,
            created_by,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO report_definitions (id, report_code, name, description, category, data_source, query_text, parameters, columns, filters, sorting, grouping, chart_type, is_scheduled, schedule_cron, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(report.id.to_string())
        .bind(&report.report_code)
        .bind(&report.name)
        .bind(&report.description)
        .bind(&report.category)
        .bind(&report.data_source)
        .bind(&report.query_text)
        .bind(&report.parameters)
        .bind(&report.columns)
        .bind(&report.filters)
        .bind(&report.sorting)
        .bind(&report.grouping)
        .bind(&report.chart_type)
        .bind(if report.is_scheduled { 1 } else { 0 })
        .bind(&report.schedule_cron)
        .bind(report.created_by.map(|id| id.to_string()))
        .bind(report.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(report)
    }

    pub async fn get_report(pool: &SqlitePool, id: Uuid) -> Result<ReportDefinition> {
        let row = sqlx::query_as::<_, ReportDefinitionRow>(
            "SELECT id, report_code, name, description, category, data_source, query_text, parameters, columns, filters, sorting, grouping, chart_type, is_scheduled, schedule_cron, created_by, created_at
             FROM report_definitions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("ReportDefinition", &id.to_string()))?;

        Ok(row.into())
    }

    pub async fn list_reports(pool: &SqlitePool, category: Option<&str>) -> Result<Vec<ReportDefinition>> {
        let rows = if let Some(cat) = category {
            sqlx::query_as::<_, ReportDefinitionRow>(
                "SELECT id, report_code, name, description, category, data_source, query_text, parameters, columns, filters, sorting, grouping, chart_type, is_scheduled, schedule_cron, created_by, created_at
                 FROM report_definitions WHERE category = ? ORDER BY name"
            )
            .bind(cat)
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?
        } else {
            sqlx::query_as::<_, ReportDefinitionRow>(
                "SELECT id, report_code, name, description, category, data_source, query_text, parameters, columns, filters, sorting, grouping, chart_type, is_scheduled, schedule_cron, created_by, created_at
                 FROM report_definitions ORDER BY name"
            )
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?
        };

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn execute_report(
        pool: &SqlitePool,
        report_id: Uuid,
        parameters: Option<&str>,
        created_by: Option<Uuid>,
    ) -> Result<ReportExecution> {
        let report = Self::get_report(pool, report_id).await?;
        let start = std::time::Instant::now();
        let now = Utc::now();

        let mut execution = ReportExecution {
            id: Uuid::new_v4(),
            report_id,
            parameters: parameters.map(|s| s.to_string()),
            row_count: None,
            file_path: None,
            file_format: None,
            file_size: None,
            execution_time_ms: Some(start.elapsed().as_millis() as i32),
            status: ReportExecutionStatus::Running,
            error_message: None,
            created_by,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO report_executions (id, report_id, parameters, row_count, file_path, file_format, file_size, execution_time_ms, status, error_message, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(execution.id.to_string())
        .bind(execution.report_id.to_string())
        .bind(&execution.parameters)
        .bind(execution.row_count)
        .bind(&execution.file_path)
        .bind(&execution.file_format)
        .bind(execution.file_size)
        .bind(execution.execution_time_ms)
        .bind(format!("{:?}", execution.status))
        .bind(&execution.error_message)
        .bind(execution.created_by.map(|id| id.to_string()))
        .bind(execution.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        let count: (i64,) = sqlx::query_as(&report.query_text)
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?;

        execution.row_count = Some(count.0 as i32);
        execution.execution_time_ms = Some(start.elapsed().as_millis() as i32);
        execution.status = ReportExecutionStatus::Completed;

        sqlx::query(
            "UPDATE report_executions SET row_count = ?, execution_time_ms = ?, status = ? WHERE id = ?"
        )
        .bind(execution.row_count)
        .bind(execution.execution_time_ms)
        .bind(format!("{:?}", execution.status))
        .bind(execution.id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(execution)
    }

    pub async fn get_results(pool: &SqlitePool, execution_id: Uuid) -> Result<ReportExecution> {
        let row = sqlx::query_as::<_, ReportExecutionRow>(
            "SELECT id, report_id, parameters, row_count, file_path, file_format, file_size, execution_time_ms, status, error_message, created_by, created_at
             FROM report_executions WHERE id = ?"
        )
        .bind(execution_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("ReportExecution", &execution_id.to_string()))?;

        Ok(row.into())
    }

    pub async fn schedule_report(
        pool: &SqlitePool,
        report_id: Uuid,
        schedule_cron: &str,
    ) -> Result<ReportDefinition> {
        let mut report = Self::get_report(pool, report_id).await?;
        report.is_scheduled = true;
        report.schedule_cron = Some(schedule_cron.to_string());

        sqlx::query(
            "UPDATE report_definitions SET is_scheduled = 1, schedule_cron = ? WHERE id = ?"
        )
        .bind(&report.schedule_cron)
        .bind(report_id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(report)
    }
}

pub struct MobileService;

impl MobileService {
    pub async fn register_device(
        pool: &SqlitePool,
        user_id: Uuid,
        device_type: DeviceType,
        device_token: &str,
        device_name: Option<&str>,
        os_version: Option<&str>,
        app_version: Option<&str>,
    ) -> Result<MobileDevice> {
        let now = Utc::now();
        let device = MobileDevice {
            id: Uuid::new_v4(),
            user_id,
            device_type,
            device_token: device_token.to_string(),
            device_name: device_name.map(|s| s.to_string()),
            os_version: os_version.map(|s| s.to_string()),
            app_version: app_version.map(|s| s.to_string()),
            last_active: Some(now),
            status: Status::Active,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO mobile_devices (id, user_id, device_type, device_token, device_name, os_version, app_version, last_active, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(device.id.to_string())
        .bind(device.user_id.to_string())
        .bind(format!("{:?}", device.device_type))
        .bind(&device.device_token)
        .bind(&device.device_name)
        .bind(&device.os_version)
        .bind(&device.app_version)
        .bind(device.last_active.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", device.status))
        .bind(device.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(device)
    }

    pub async fn unregister_device(pool: &SqlitePool, device_id: Uuid) -> Result<()> {
        let rows = sqlx::query(
            "UPDATE mobile_devices SET status = 'Inactive' WHERE id = ?"
        )
        .bind(device_id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("MobileDevice", &device_id.to_string()));
        }

        Ok(())
    }

    pub async fn send_push_notification(
        pool: &SqlitePool,
        device_id: Uuid,
        title: &str,
        message: &str,
        data: Option<&str>,
    ) -> Result<PushNotification> {
        let now = Utc::now();
        let notification = PushNotification {
            id: Uuid::new_v4(),
            device_id,
            title: title.to_string(),
            message: message.to_string(),
            data: data.map(|s| s.to_string()),
            sent_at: Some(now),
            delivered_at: None,
            read_at: None,
            status: PushNotificationStatus::Sent,
        };

        sqlx::query(
            "INSERT INTO push_notifications (id, device_id, title, message, data, sent_at, delivered_at, read_at, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(notification.id.to_string())
        .bind(notification.device_id.to_string())
        .bind(&notification.title)
        .bind(&notification.message)
        .bind(&notification.data)
        .bind(notification.sent_at.map(|d| d.to_rfc3339()))
        .bind(notification.delivered_at.map(|d| d.to_rfc3339()))
        .bind(notification.read_at.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", notification.status))
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(notification)
    }

    pub async fn get_user_notifications(
        pool: &SqlitePool,
        user_id: Uuid,
        pagination: Pagination,
    ) -> Result<Paginated<PushNotification>> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM push_notifications pn
             JOIN mobile_devices md ON pn.device_id = md.id
             WHERE md.user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(Error::Database)?;

        let rows = sqlx::query_as::<_, PushNotificationRow>(
            "SELECT pn.id, pn.device_id, pn.title, pn.message, pn.data, pn.sent_at, pn.delivered_at, pn.read_at, pn.status
             FROM push_notifications pn
             JOIN mobile_devices md ON pn.device_id = md.id
             WHERE md.user_id = ?
             ORDER BY pn.sent_at DESC
             LIMIT ? OFFSET ?"
        )
        .bind(user_id.to_string())
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;

        Ok(Paginated::new(
            rows.into_iter().map(|r| r.into()).collect(),
            count.0 as u64,
            pagination,
        ))
    }
}

pub struct APIService;

impl APIService {
    pub async fn create_api_key(
        pool: &SqlitePool,
        user_id: Uuid,
        name: &str,
        permissions: Option<&str>,
        rate_limit: i32,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<(APIKey, String)> {
        let now = Utc::now();
        let raw_key = format!("sk_{}", uuid::Uuid::new_v4().simple());
        let key_hash = sha256_hash(&raw_key);

        let api_key = APIKey {
            id: Uuid::new_v4(),
            user_id,
            key_hash,
            name: name.to_string(),
            permissions: permissions.map(|s| s.to_string()),
            rate_limit,
            last_used: None,
            expires_at,
            status: Status::Active,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO api_keys (id, user_id, key_hash, name, permissions, rate_limit, last_used, expires_at, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(api_key.id.to_string())
        .bind(api_key.user_id.to_string())
        .bind(&api_key.key_hash)
        .bind(&api_key.name)
        .bind(&api_key.permissions)
        .bind(api_key.rate_limit)
        .bind(api_key.last_used.map(|d| d.to_rfc3339()))
        .bind(api_key.expires_at.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", api_key.status))
        .bind(api_key.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok((api_key, raw_key))
    }

    pub async fn validate_key(pool: &SqlitePool, raw_key: &str) -> Result<APIKey> {
        let key_hash = sha256_hash(raw_key);
        let now = Utc::now();

        let row = sqlx::query_as::<_, APIKeyRow>(
            "SELECT id, user_id, key_hash, name, permissions, rate_limit, last_used, expires_at, status, created_at
             FROM api_keys WHERE key_hash = ? AND status = 'Active'"
        )
        .bind(&key_hash)
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::Unauthorized)?;

        let api_key: APIKey = row.into();

        if let Some(expires) = api_key.expires_at {
            if expires < now {
                return Err(Error::Unauthorized);
            }
        }

        sqlx::query("UPDATE api_keys SET last_used = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(api_key.id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;

        Ok(api_key)
    }

    pub async fn log_api_usage(
        pool: &SqlitePool,
        api_key_id: Option<Uuid>,
        endpoint: &str,
        method: &str,
        request_size: Option<i32>,
        response_size: Option<i32>,
        response_code: i32,
        response_time_ms: Option<i32>,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<APIUsageLog> {
        let now = Utc::now();
        let log = APIUsageLog {
            id: Uuid::new_v4(),
            api_key_id,
            endpoint: endpoint.to_string(),
            method: method.to_string(),
            request_size,
            response_size,
            response_code,
            response_time_ms,
            ip_address: ip_address.map(|s| s.to_string()),
            user_agent: user_agent.map(|s| s.to_string()),
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO api_usage_logs (id, api_key_id, endpoint, method, request_size, response_size, response_code, response_time_ms, ip_address, user_agent, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(log.id.to_string())
        .bind(log.api_key_id.map(|id| id.to_string()))
        .bind(&log.endpoint)
        .bind(&log.method)
        .bind(log.request_size)
        .bind(log.response_size)
        .bind(log.response_code)
        .bind(log.response_time_ms)
        .bind(&log.ip_address)
        .bind(&log.user_agent)
        .bind(log.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;

        Ok(log)
    }

    pub async fn get_usage_stats(
        pool: &SqlitePool,
        api_key_id: Option<Uuid>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<APIUsageStats> {
        let (total_requests, avg_response_time, error_count): (i64, Option<f64>, i64) = if let Some(kid) = api_key_id {
            sqlx::query_as(
                "SELECT COUNT(*), AVG(response_time_ms), SUM(CASE WHEN response_code >= 400 THEN 1 ELSE 0 END)
                 FROM api_usage_logs
                 WHERE api_key_id = ? AND created_at >= ? AND created_at <= ?"
            )
            .bind(kid.to_string())
            .bind(start_date.to_rfc3339())
            .bind(end_date.to_rfc3339())
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?
        } else {
            sqlx::query_as(
                "SELECT COUNT(*), AVG(response_time_ms), SUM(CASE WHEN response_code >= 400 THEN 1 ELSE 0 END)
                 FROM api_usage_logs
                 WHERE created_at >= ? AND created_at <= ?"
            )
            .bind(start_date.to_rfc3339())
            .bind(end_date.to_rfc3339())
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?
        };

        Ok(APIUsageStats {
            api_key_id,
            start_date,
            end_date,
            total_requests: total_requests as u64,
            avg_response_time_ms: avg_response_time.map(|v| v as i32),
            error_count: error_count as u64,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantLimits {
    pub tenant_id: Uuid,
    pub max_users: i32,
    pub current_users: i32,
    pub max_storage_mb: i32,
    pub storage_used_mb: i32,
    pub users_exceeded: bool,
    pub storage_exceeded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIUsageStats {
    pub api_key_id: Option<Uuid>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_requests: u64,
    pub avg_response_time_ms: Option<i32>,
    pub error_count: u64,
}

fn sha256_hash(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn parse_datetime(s: &str) -> DateTime<Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn parse_trigger_event(s: &str) -> TriggerEvent {
    match s {
        "Create" => TriggerEvent::Create,
        "Update" => TriggerEvent::Update,
        "Delete" => TriggerEvent::Delete,
        "StatusChange" => TriggerEvent::StatusChange,
        "FieldChange" => TriggerEvent::FieldChange,
        "Scheduled" => TriggerEvent::Scheduled,
        _ => TriggerEvent::Manual,
    }
}

#[derive(sqlx::FromRow)]
struct TenantRow {
    id: String,
    tenant_code: String,
    name: String,
    plan_type: String,
    max_users: i64,
    max_storage_mb: i64,
    settings: Option<String>,
    status: String,
    created_at: String,
    expires_at: Option<String>,
}

impl From<TenantRow> for Tenant {
    fn from(r: TenantRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            tenant_code: r.tenant_code,
            name: r.name,
            plan_type: match r.plan_type.as_str() {
                "Starter" => TenantPlanType::Starter,
                "Professional" => TenantPlanType::Professional,
                "Enterprise" => TenantPlanType::Enterprise,
                _ => TenantPlanType::Free,
            },
            max_users: r.max_users as i32,
            max_storage_mb: r.max_storage_mb as i32,
            settings: r.settings,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
            created_at: parse_datetime(&r.created_at),
            expires_at: r.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
        }
    }
}

#[derive(sqlx::FromRow)]
struct AutomationRuleRow {
    id: String,
    name: String,
    description: Option<String>,
    entity_type: String,
    trigger_event: String,
    conditions: Option<String>,
    actions: String,
    priority: i64,
    active: i64,
    created_by: Option<String>,
    created_at: String,
}

impl From<AutomationRuleRow> for AutomationRule {
    fn from(r: AutomationRuleRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            description: r.description,
            entity_type: r.entity_type,
            trigger_event: parse_trigger_event(&r.trigger_event),
            conditions: r.conditions,
            actions: r.actions,
            priority: r.priority as i32,
            active: r.active != 0,
            created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: parse_datetime(&r.created_at),
        }
    }
}

#[derive(sqlx::FromRow)]
struct AutomationLogRow {
    id: String,
    rule_id: String,
    entity_type: Option<String>,
    entity_id: Option<String>,
    trigger_data: Option<String>,
    action_results: Option<String>,
    success: i64,
    error_message: Option<String>,
    executed_at: String,
}

impl From<AutomationLogRow> for AutomationLog {
    fn from(r: AutomationLogRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            rule_id: Uuid::parse_str(&r.rule_id).unwrap_or_default(),
            entity_type: r.entity_type,
            entity_id: r.entity_id.and_then(|id| Uuid::parse_str(&id).ok()),
            trigger_data: r.trigger_data,
            action_results: r.action_results,
            success: r.success != 0,
            error_message: r.error_message,
            executed_at: parse_datetime(&r.executed_at),
        }
    }
}

#[derive(sqlx::FromRow)]
struct EmailTemplateRow {
    id: String,
    template_code: String,
    name: String,
    subject: String,
    body_html: String,
    body_text: Option<String>,
    category: Option<String>,
    variables: Option<String>,
    status: String,
    created_at: String,
}

impl From<EmailTemplateRow> for EmailTemplate {
    fn from(r: EmailTemplateRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            template_code: r.template_code,
            name: r.name,
            subject: r.subject,
            body_html: r.body_html,
            body_text: r.body_text,
            category: r.category,
            variables: r.variables,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
            created_at: parse_datetime(&r.created_at),
        }
    }
}

#[derive(sqlx::FromRow)]
struct EmailQueueRow {
    id: String,
    template_id: Option<String>,
    to_address: String,
    cc_addresses: Option<String>,
    bcc_addresses: Option<String>,
    subject: String,
    body: String,
    attachments: Option<String>,
    priority: i64,
    attempts: i64,
    max_attempts: i64,
    sent_at: Option<String>,
    error_message: Option<String>,
    status: String,
    created_at: String,
}

impl From<EmailQueueRow> for EmailQueue {
    fn from(r: EmailQueueRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            template_id: r.template_id.and_then(|id| Uuid::parse_str(&id).ok()),
            to_address: r.to_address,
            cc_addresses: r.cc_addresses,
            bcc_addresses: r.bcc_addresses,
            subject: r.subject,
            body: r.body,
            attachments: r.attachments,
            priority: r.priority as i32,
            attempts: r.attempts as i32,
            max_attempts: r.max_attempts as i32,
            sent_at: r.sent_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            error_message: r.error_message,
            status: match r.status.as_str() {
                "Sent" => EmailStatus::Sent,
                "Failed" => EmailStatus::Failed,
                "Cancelled" => EmailStatus::Cancelled,
                _ => EmailStatus::Pending,
            },
            created_at: parse_datetime(&r.created_at),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ReportDefinitionRow {
    id: String,
    report_code: String,
    name: String,
    description: Option<String>,
    category: Option<String>,
    data_source: String,
    query_text: String,
    parameters: Option<String>,
    columns: Option<String>,
    filters: Option<String>,
    sorting: Option<String>,
    grouping: Option<String>,
    chart_type: Option<String>,
    is_scheduled: i64,
    schedule_cron: Option<String>,
    created_by: Option<String>,
    created_at: String,
}

impl From<ReportDefinitionRow> for ReportDefinition {
    fn from(r: ReportDefinitionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            report_code: r.report_code,
            name: r.name,
            description: r.description,
            category: r.category,
            data_source: r.data_source,
            query_text: r.query_text,
            parameters: r.parameters,
            columns: r.columns,
            filters: r.filters,
            sorting: r.sorting,
            grouping: r.grouping,
            chart_type: r.chart_type,
            is_scheduled: r.is_scheduled != 0,
            schedule_cron: r.schedule_cron,
            created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: parse_datetime(&r.created_at),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ReportExecutionRow {
    id: String,
    report_id: String,
    parameters: Option<String>,
    row_count: Option<i64>,
    file_path: Option<String>,
    file_format: Option<String>,
    file_size: Option<i64>,
    execution_time_ms: Option<i64>,
    status: String,
    error_message: Option<String>,
    created_by: Option<String>,
    created_at: String,
}

impl From<ReportExecutionRow> for ReportExecution {
    fn from(r: ReportExecutionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            report_id: Uuid::parse_str(&r.report_id).unwrap_or_default(),
            parameters: r.parameters,
            row_count: r.row_count.map(|v| v as i32),
            file_path: r.file_path,
            file_format: r.file_format,
            file_size: r.file_size.map(|v| v as i32),
            execution_time_ms: r.execution_time_ms.map(|v| v as i32),
            status: match r.status.as_str() {
                "Completed" => ReportExecutionStatus::Completed,
                "Failed" => ReportExecutionStatus::Failed,
                "Cancelled" => ReportExecutionStatus::Cancelled,
                _ => ReportExecutionStatus::Running,
            },
            error_message: r.error_message,
            created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: parse_datetime(&r.created_at),
        }
    }
}

#[derive(sqlx::FromRow)]
struct PushNotificationRow {
    id: String,
    device_id: String,
    title: String,
    message: String,
    data: Option<String>,
    sent_at: Option<String>,
    delivered_at: Option<String>,
    read_at: Option<String>,
    status: String,
}

impl From<PushNotificationRow> for PushNotification {
    fn from(r: PushNotificationRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            device_id: Uuid::parse_str(&r.device_id).unwrap_or_default(),
            title: r.title,
            message: r.message,
            data: r.data,
            sent_at: r.sent_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            delivered_at: r.delivered_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            read_at: r.read_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            status: match r.status.as_str() {
                "Sent" => PushNotificationStatus::Sent,
                "Delivered" => PushNotificationStatus::Delivered,
                "Read" => PushNotificationStatus::Read,
                "Failed" => PushNotificationStatus::Failed,
                _ => PushNotificationStatus::Pending,
            },
        }
    }
}

#[derive(sqlx::FromRow)]
struct APIKeyRow {
    id: String,
    user_id: String,
    key_hash: String,
    name: String,
    permissions: Option<String>,
    rate_limit: i64,
    last_used: Option<String>,
    expires_at: Option<String>,
    status: String,
    created_at: String,
}

impl From<APIKeyRow> for APIKey {
    fn from(r: APIKeyRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            user_id: Uuid::parse_str(&r.user_id).unwrap_or_default(),
            key_hash: r.key_hash,
            name: r.name,
            permissions: r.permissions,
            rate_limit: r.rate_limit as i32,
            last_used: r.last_used.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            expires_at: r.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
            created_at: parse_datetime(&r.created_at),
        }
    }
}
