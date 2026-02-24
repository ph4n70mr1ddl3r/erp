use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait RulesRepository: Send + Sync {
    async fn create_rule(&self, pool: &SqlitePool, rule: BusinessRule) -> Result<BusinessRule>;
    async fn get_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<BusinessRule>;
    async fn get_rule_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Option<BusinessRule>>;
    async fn list_rules(&self, pool: &SqlitePool, entity_type: Option<&str>) -> Result<Vec<BusinessRule>>;
    async fn update_rule(&self, pool: &SqlitePool, rule: BusinessRule) -> Result<BusinessRule>;
    async fn delete_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn create_ruleset(&self, pool: &SqlitePool, ruleset: RuleSet) -> Result<RuleSet>;
    async fn get_ruleset(&self, pool: &SqlitePool, id: Uuid) -> Result<RuleSet>;
    async fn list_rulesets(&self, pool: &SqlitePool) -> Result<Vec<RuleSet>>;
    async fn add_rule_to_ruleset(&self, pool: &SqlitePool, member: RuleSetMember) -> Result<RuleSetMember>;
    async fn create_execution(&self, pool: &SqlitePool, exec: RuleExecution) -> Result<RuleExecution>;
    async fn create_variable(&self, pool: &SqlitePool, var: RuleVariable) -> Result<RuleVariable>;
    async fn list_variables(&self, pool: &SqlitePool) -> Result<Vec<RuleVariable>>;
    async fn create_function(&self, pool: &SqlitePool, func: RuleFunction) -> Result<RuleFunction>;
    async fn list_functions(&self, pool: &SqlitePool) -> Result<Vec<RuleFunction>>;
    async fn create_template(&self, pool: &SqlitePool, template: RuleTemplate) -> Result<RuleTemplate>;
    async fn list_templates(&self, pool: &SqlitePool) -> Result<Vec<RuleTemplate>>;
    async fn create_decision_table(&self, pool: &SqlitePool, table: DecisionTable) -> Result<DecisionTable>;
    async fn get_decision_table(&self, pool: &SqlitePool, id: Uuid) -> Result<DecisionTable>;
    async fn create_decision_row(&self, pool: &SqlitePool, row: DecisionTableRow) -> Result<DecisionTableRow>;
    async fn list_decision_rows(&self, pool: &SqlitePool, table_id: Uuid) -> Result<Vec<DecisionTableRow>>;
}

pub struct SqliteRulesRepository;

#[async_trait]
impl RulesRepository for SqliteRulesRepository {
    async fn create_rule(&self, pool: &SqlitePool, rule: BusinessRule) -> Result<BusinessRule> {
        sqlx::query!(
            r#"INSERT INTO business_rules (id, name, code, description, rule_type, entity_type, status,
               priority, version, effective_from, effective_to, conditions, actions, else_actions, tags,
               owner_id, created_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.base.id.to_string(),
            rule.name,
            rule.code,
            rule.description,
            format!("{:?}", rule.rule_type),
            rule.entity_type,
            format!("{:?}", rule.status),
            rule.priority,
            rule.version,
            rule.effective_from.map(|d| d.to_rfc3339()),
            rule.effective_to.map(|d| d.to_rfc3339()),
            rule.conditions,
            rule.actions,
            rule.else_actions,
            rule.tags,
            rule.owner_id.map(|id| id.to_string()),
            rule.created_by.map(|id| id.to_string()),
            rule.base.created_at.to_rfc3339(),
            rule.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(rule)
    }

    async fn get_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<BusinessRule> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, rule_type, entity_type, status, priority, version,
               effective_from, effective_to, conditions, actions, else_actions, tags, owner_id, created_by,
               created_at, updated_at FROM business_rules WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(BusinessRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            rule_type: RuleType::Validation,
            entity_type: row.entity_type,
            status: RuleStatus::Active,
            priority: row.priority,
            version: row.version,
            effective_from: row.effective_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.effective_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            conditions: row.conditions,
            actions: row.actions,
            else_actions: row.else_actions,
            tags: row.tags,
            owner_id: row.owner_id.and_then(|s| Uuid::parse_str(&s).ok()),
            created_by: None,
        })
    }

    async fn get_rule_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Option<BusinessRule>> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, rule_type, entity_type, status, priority, version,
               effective_from, effective_to, conditions, actions, else_actions, tags, owner_id, created_by,
               created_at, updated_at FROM business_rules WHERE code = ?"#,
            code
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|row| BusinessRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            rule_type: RuleType::Validation,
            entity_type: row.entity_type,
            status: RuleStatus::Active,
            priority: row.priority,
            version: row.version,
            effective_from: row.effective_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.effective_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            conditions: row.conditions,
            actions: row.actions,
            else_actions: row.else_actions,
            tags: row.tags,
            owner_id: row.owner_id.and_then(|s| Uuid::parse_str(&s).ok()),
            created_by: None,
        }))
    }

    async fn list_rules(&self, pool: &SqlitePool, entity_type: Option<&str>) -> Result<Vec<BusinessRule>> {
        let rows = if let Some(et) = entity_type {
            sqlx::query!(
                r#"SELECT id, name, code, description, rule_type, entity_type, status, priority, version,
                   effective_from, effective_to, conditions, actions, else_actions, tags, owner_id, created_by,
                   created_at, updated_at FROM business_rules WHERE entity_type = ? AND status = 'Active'
                   ORDER BY priority"#,
                et
            ).fetch_all(pool).await?
        } else {
            sqlx::query!(
                r#"SELECT id, name, code, description, rule_type, entity_type, status, priority, version,
                   effective_from, effective_to, conditions, actions, else_actions, tags, owner_id, created_by,
                   created_at, updated_at FROM business_rules WHERE status = 'Active' ORDER BY priority"#
            ).fetch_all(pool).await?
        };
        
        Ok(rows.into_iter().map(|row| BusinessRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            rule_type: RuleType::Validation,
            entity_type: row.entity_type,
            status: RuleStatus::Active,
            priority: row.priority,
            version: row.version,
            effective_from: row.effective_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.effective_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            conditions: row.conditions,
            actions: row.actions,
            else_actions: row.else_actions,
            tags: row.tags,
            owner_id: row.owner_id.and_then(|s| Uuid::parse_str(&s).ok()),
            created_by: None,
        }).collect())
    }

    async fn update_rule(&self, pool: &SqlitePool, rule: BusinessRule) -> Result<BusinessRule> {
        sqlx::query!(
            r#"UPDATE business_rules SET name = ?, description = ?, status = ?, priority = ?,
               conditions = ?, actions = ?, else_actions = ?, updated_at = ? WHERE id = ?"#,
            rule.name,
            rule.description,
            format!("{:?}", rule.status),
            rule.priority,
            rule.conditions,
            rule.actions,
            rule.else_actions,
            rule.base.updated_at.to_rfc3339(),
            rule.base.id.to_string(),
        ).execute(pool).await?;
        Ok(rule)
    }

    async fn delete_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM business_rules WHERE id = ?", id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_ruleset(&self, pool: &SqlitePool, ruleset: RuleSet) -> Result<RuleSet> {
        sqlx::query!(
            r#"INSERT INTO rule_sets (id, name, code, description, entity_type, status, version,
               effective_from, effective_to, execution_mode, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            ruleset.base.id.to_string(),
            ruleset.name,
            ruleset.code,
            ruleset.description,
            ruleset.entity_type,
            format!("{:?}", ruleset.status),
            ruleset.version,
            ruleset.effective_from.map(|d| d.to_rfc3339()),
            ruleset.effective_to.map(|d| d.to_rfc3339()),
            format!("{:?}", ruleset.execution_mode),
            ruleset.base.created_at.to_rfc3339(),
            ruleset.base.updated_at.to_rfc3339(),
            ruleset.base.created_by.map(|id| id.to_string()),
            ruleset.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(ruleset)
    }

    async fn get_ruleset(&self, pool: &SqlitePool, id: Uuid) -> Result<RuleSet> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, entity_type, status, version, effective_from,
               effective_to, execution_mode, created_at, updated_at FROM rule_sets WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(RuleSet {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            entity_type: row.entity_type,
            status: RuleStatus::Active,
            version: row.version,
            effective_from: row.effective_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.effective_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            execution_mode: ExecutionMode::Sequential,
        })
    }

    async fn list_rulesets(&self, pool: &SqlitePool) -> Result<Vec<RuleSet>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, description, entity_type, status, version, effective_from,
               effective_to, execution_mode, created_at, updated_at FROM rule_sets WHERE status = 'Active'"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| RuleSet {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            entity_type: row.entity_type,
            status: RuleStatus::Active,
            version: row.version,
            effective_from: row.effective_from.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.effective_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            execution_mode: ExecutionMode::Sequential,
        }).collect())
    }

    async fn add_rule_to_ruleset(&self, pool: &SqlitePool, member: RuleSetMember) -> Result<RuleSetMember> {
        sqlx::query!(
            r#"INSERT INTO rule_set_members (id, ruleset_id, rule_id, sort_order, is_required,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            member.base.id.to_string(),
            member.ruleset_id.to_string(),
            member.rule_id.to_string(),
            member.sort_order,
            member.is_required as i32,
            member.base.created_at.to_rfc3339(),
            member.base.updated_at.to_rfc3339(),
            member.base.created_by.map(|id| id.to_string()),
            member.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(member)
    }

    async fn create_execution(&self, pool: &SqlitePool, exec: RuleExecution) -> Result<RuleExecution> {
        sqlx::query!(
            r#"INSERT INTO rule_executions (id, rule_id, ruleset_id, entity_type, entity_id,
               triggered_at, conditions_evaluated, matched, actions_executed, result, error,
               execution_time_ms, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            exec.base.id.to_string(),
            exec.rule_id.to_string(),
            exec.ruleset_id.map(|id| id.to_string()),
            exec.entity_type,
            exec.entity_id.to_string(),
            exec.triggered_at.to_rfc3339(),
            exec.conditions_evaluated,
            exec.matched as i32,
            exec.actions_executed,
            exec.result,
            exec.error,
            exec.execution_time_ms,
            exec.base.created_at.to_rfc3339(),
            exec.base.updated_at.to_rfc3339(),
            exec.base.created_by.map(|id| id.to_string()),
            exec.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(exec)
    }

    async fn create_variable(&self, pool: &SqlitePool, var: RuleVariable) -> Result<RuleVariable> {
        sqlx::query!(
            r#"INSERT INTO rule_variables (id, name, code, description, data_type, default_value,
               source_type, source_config, is_constant, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            var.base.id.to_string(),
            var.name,
            var.code,
            var.description,
            var.data_type,
            var.default_value,
            format!("{:?}", var.source_type),
            var.source_config,
            var.is_constant as i32,
            format!("{:?}", var.status),
            var.base.created_at.to_rfc3339(),
            var.base.updated_at.to_rfc3339(),
            var.base.created_by.map(|id| id.to_string()),
            var.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(var)
    }

    async fn list_variables(&self, pool: &SqlitePool) -> Result<Vec<RuleVariable>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, description, data_type, default_value, source_type,
               source_config, is_constant, status, created_at, updated_at FROM rule_variables"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| RuleVariable {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            data_type: row.data_type,
            default_value: row.default_value,
            source_type: VariableSource::Static,
            source_config: row.source_config,
            is_constant: row.is_constant == 1,
            status: erp_core::Status::Active,
        }).collect())
    }

    async fn create_function(&self, pool: &SqlitePool, func: RuleFunction) -> Result<RuleFunction> {
        sqlx::query!(
            r#"INSERT INTO rule_functions (id, name, code, description, return_type, parameters,
               function_body, is_builtin, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            func.base.id.to_string(),
            func.name,
            func.code,
            func.description,
            func.return_type,
            func.parameters,
            func.function_body,
            func.is_builtin as i32,
            format!("{:?}", func.status),
            func.base.created_at.to_rfc3339(),
            func.base.updated_at.to_rfc3339(),
            func.base.created_by.map(|id| id.to_string()),
            func.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(func)
    }

    async fn list_functions(&self, pool: &SqlitePool) -> Result<Vec<RuleFunction>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, description, return_type, parameters, function_body,
               is_builtin, status, created_at, updated_at FROM rule_functions"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| RuleFunction {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            return_type: row.return_type,
            parameters: row.parameters,
            function_body: row.function_body,
            is_builtin: row.is_builtin == 1,
            status: erp_core::Status::Active,
        }).collect())
    }

    async fn create_template(&self, pool: &SqlitePool, template: RuleTemplate) -> Result<RuleTemplate> {
        sqlx::query!(
            r#"INSERT INTO rule_templates (id, name, code, description, rule_type, entity_type,
               template, variables, is_builtin, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            template.base.id.to_string(),
            template.name,
            template.code,
            template.description,
            format!("{:?}", template.rule_type),
            template.entity_type,
            template.template,
            template.variables,
            template.is_builtin as i32,
            format!("{:?}", template.status),
            template.base.created_at.to_rfc3339(),
            template.base.updated_at.to_rfc3339(),
            template.base.created_by.map(|id| id.to_string()),
            template.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(template)
    }

    async fn list_templates(&self, pool: &SqlitePool) -> Result<Vec<RuleTemplate>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, description, rule_type, entity_type, template, variables,
               is_builtin, status, created_at, updated_at FROM rule_templates"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| RuleTemplate {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            rule_type: RuleType::Validation,
            entity_type: row.entity_type,
            template: row.template,
            variables: row.variables,
            is_builtin: row.is_builtin == 1,
            status: erp_core::Status::Active,
        }).collect())
    }

    async fn create_decision_table(&self, pool: &SqlitePool, table: DecisionTable) -> Result<DecisionTable> {
        sqlx::query!(
            r#"INSERT INTO decision_tables (id, name, code, description, entity_type, input_columns,
               output_columns, hit_policy, status, version, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            table.base.id.to_string(),
            table.name,
            table.code,
            table.description,
            table.entity_type,
            table.input_columns,
            table.output_columns,
            format!("{:?}", table.hit_policy),
            format!("{:?}", table.status),
            table.version,
            table.base.created_at.to_rfc3339(),
            table.base.updated_at.to_rfc3339(),
            table.base.created_by.map(|id| id.to_string()),
            table.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(table)
    }

    async fn get_decision_table(&self, pool: &SqlitePool, id: Uuid) -> Result<DecisionTable> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, description, entity_type, input_columns, output_columns,
               hit_policy, status, version, created_at, updated_at FROM decision_tables WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(DecisionTable {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            code: row.code,
            description: row.description,
            entity_type: row.entity_type,
            input_columns: row.input_columns,
            output_columns: row.output_columns,
            hit_policy: HitPolicy::First,
            status: RuleStatus::Active,
            version: row.version,
        })
    }

    async fn create_decision_row(&self, pool: &SqlitePool, row: DecisionTableRow) -> Result<DecisionTableRow> {
        sqlx::query!(
            r#"INSERT INTO decision_table_rows (id, table_id, row_number, inputs, outputs,
               description, is_active, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            row.base.id.to_string(),
            row.table_id.to_string(),
            row.row_number,
            row.inputs,
            row.outputs,
            row.description,
            row.is_active as i32,
            row.base.created_at.to_rfc3339(),
            row.base.updated_at.to_rfc3339(),
            row.base.created_by.map(|id| id.to_string()),
            row.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(row)
    }

    async fn list_decision_rows(&self, pool: &SqlitePool, table_id: Uuid) -> Result<Vec<DecisionTableRow>> {
        let rows = sqlx::query!(
            r#"SELECT id, table_id, row_number, inputs, outputs, description, is_active,
               created_at, updated_at FROM decision_table_rows WHERE table_id = ? ORDER BY row_number"#,
            table_id.to_string()
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| DecisionTableRow {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            table_id: Uuid::parse_str(&r.table_id).unwrap(),
            row_number: r.row_number,
            inputs: r.inputs,
            outputs: r.outputs,
            description: r.description,
            is_active: r.is_active == 1,
        }).collect())
    }
}
