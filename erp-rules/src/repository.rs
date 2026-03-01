use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
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
        sqlx::query(
            r#"INSERT INTO business_rules (id, name, code, description, rule_type, entity_type, status,
               priority, version, effective_from, effective_to, conditions, actions, else_actions, tags,
               owner_id, created_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(rule.base.id.to_string())
        .bind(&rule.name)
        .bind(&rule.code)
        .bind(&rule.description)
        .bind(format!("{:?}", rule.rule_type))
        .bind(&rule.entity_type)
        .bind(format!("{:?}", rule.status))
        .bind(rule.priority)
        .bind(rule.version)
        .bind(rule.effective_from.map(|d| d.to_rfc3339()))
        .bind(rule.effective_to.map(|d| d.to_rfc3339()))
        .bind(&rule.conditions)
        .bind(&rule.actions)
        .bind(&rule.else_actions)
        .bind(&rule.tags)
        .bind(rule.owner_id.map(|id| id.to_string()))
        .bind(rule.created_by.map(|id| id.to_string()))
        .bind(rule.base.created_at.to_rfc3339())
        .bind(rule.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(rule)
    }

    async fn get_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<BusinessRule> {
        let row = sqlx::query(
            r#"SELECT id, name, code, description, rule_type, entity_type, status, priority, version,
               effective_from, effective_to, conditions, actions, else_actions, tags, owner_id, created_by,
               created_at, updated_at FROM business_rules WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(BusinessRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            rule_type: RuleType::Validation,
            entity_type: row.get("entity_type"),
            status: RuleStatus::Active,
            priority: row.get("priority"),
            version: row.get("version"),
            effective_from: row.get::<Option<&str>, _>("effective_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.get::<Option<&str>, _>("effective_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            conditions: row.get("conditions"),
            actions: row.get("actions"),
            else_actions: row.get("else_actions"),
            tags: row.get("tags"),
            owner_id: row.get::<Option<&str>, _>("owner_id").and_then(|s| Uuid::parse_str(s).ok()),
            created_by: None,
        })
    }

    async fn get_rule_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Option<BusinessRule>> {
        let row: Option<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, code, description, rule_type, entity_type, status, priority, version,
               effective_from, effective_to, conditions, actions, else_actions, tags, owner_id, created_by,
               created_at, updated_at FROM business_rules WHERE code = ?"#,
        )
        .bind(code)
        .fetch_optional(pool).await?;
        
        Ok(row.map(|row| BusinessRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            rule_type: RuleType::Validation,
            entity_type: row.get("entity_type"),
            status: RuleStatus::Active,
            priority: row.get("priority"),
            version: row.get("version"),
            effective_from: row.get::<Option<&str>, _>("effective_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.get::<Option<&str>, _>("effective_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            conditions: row.get("conditions"),
            actions: row.get("actions"),
            else_actions: row.get("else_actions"),
            tags: row.get("tags"),
            owner_id: row.get::<Option<&str>, _>("owner_id").and_then(|s| Uuid::parse_str(s).ok()),
            created_by: None,
        }))
    }

    async fn list_rules(&self, pool: &SqlitePool, entity_type: Option<&str>) -> Result<Vec<BusinessRule>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = if let Some(et) = entity_type {
            sqlx::query(
                r#"SELECT id, name, code, description, rule_type, entity_type, status, priority, version,
                   effective_from, effective_to, conditions, actions, else_actions, tags, owner_id, created_by,
                   created_at, updated_at FROM business_rules WHERE entity_type = ? AND status = 'Active'
                   ORDER BY priority"#,
            )
            .bind(et)
            .fetch_all(pool).await?
        } else {
            sqlx::query(
                r#"SELECT id, name, code, description, rule_type, entity_type, status, priority, version,
                   effective_from, effective_to, conditions, actions, else_actions, tags, owner_id, created_by,
                   created_at, updated_at FROM business_rules WHERE status = 'Active' ORDER BY priority"#,
            )
            .fetch_all(pool).await?
        };
        
        Ok(rows.into_iter().map(|row| BusinessRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            rule_type: RuleType::Validation,
            entity_type: row.get("entity_type"),
            status: RuleStatus::Active,
            priority: row.get("priority"),
            version: row.get("version"),
            effective_from: row.get::<Option<&str>, _>("effective_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.get::<Option<&str>, _>("effective_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            conditions: row.get("conditions"),
            actions: row.get("actions"),
            else_actions: row.get("else_actions"),
            tags: row.get("tags"),
            owner_id: row.get::<Option<&str>, _>("owner_id").and_then(|s| Uuid::parse_str(s).ok()),
            created_by: None,
        }).collect())
    }

    async fn update_rule(&self, pool: &SqlitePool, rule: BusinessRule) -> Result<BusinessRule> {
        sqlx::query(
            r#"UPDATE business_rules SET name = ?, description = ?, status = ?, priority = ?,
               conditions = ?, actions = ?, else_actions = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&rule.name)
        .bind(&rule.description)
        .bind(format!("{:?}", rule.status))
        .bind(rule.priority)
        .bind(&rule.conditions)
        .bind(&rule.actions)
        .bind(&rule.else_actions)
        .bind(rule.base.updated_at.to_rfc3339())
        .bind(rule.base.id.to_string())
        .execute(pool).await?;
        Ok(rule)
    }

    async fn delete_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM business_rules WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_ruleset(&self, pool: &SqlitePool, ruleset: RuleSet) -> Result<RuleSet> {
        sqlx::query(
            r#"INSERT INTO rule_sets (id, name, code, description, entity_type, status, version,
               effective_from, effective_to, execution_mode, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(ruleset.base.id.to_string())
        .bind(&ruleset.name)
        .bind(&ruleset.code)
        .bind(&ruleset.description)
        .bind(&ruleset.entity_type)
        .bind(format!("{:?}", ruleset.status))
        .bind(ruleset.version)
        .bind(ruleset.effective_from.map(|d| d.to_rfc3339()))
        .bind(ruleset.effective_to.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", ruleset.execution_mode))
        .bind(ruleset.base.created_at.to_rfc3339())
        .bind(ruleset.base.updated_at.to_rfc3339())
        .bind(ruleset.base.created_by.map(|id| id.to_string()))
        .bind(ruleset.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(ruleset)
    }

    async fn get_ruleset(&self, pool: &SqlitePool, id: Uuid) -> Result<RuleSet> {
        let row = sqlx::query(
            r#"SELECT id, name, code, description, entity_type, status, version, effective_from,
               effective_to, execution_mode, created_at, updated_at FROM rule_sets WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(RuleSet {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            entity_type: row.get("entity_type"),
            status: RuleStatus::Active,
            version: row.get("version"),
            effective_from: row.get::<Option<&str>, _>("effective_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.get::<Option<&str>, _>("effective_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            execution_mode: ExecutionMode::Sequential,
        })
    }

    async fn list_rulesets(&self, pool: &SqlitePool) -> Result<Vec<RuleSet>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, code, description, entity_type, status, version, effective_from,
               effective_to, execution_mode, created_at, updated_at FROM rule_sets WHERE status = 'Active'"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| RuleSet {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            entity_type: row.get("entity_type"),
            status: RuleStatus::Active,
            version: row.get("version"),
            effective_from: row.get::<Option<&str>, _>("effective_from").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            effective_to: row.get::<Option<&str>, _>("effective_to").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            execution_mode: ExecutionMode::Sequential,
        }).collect())
    }

    async fn add_rule_to_ruleset(&self, pool: &SqlitePool, member: RuleSetMember) -> Result<RuleSetMember> {
        sqlx::query(
            r#"INSERT INTO rule_set_members (id, ruleset_id, rule_id, sort_order, is_required,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(member.base.id.to_string())
        .bind(member.ruleset_id.to_string())
        .bind(member.rule_id.to_string())
        .bind(member.sort_order)
        .bind(member.is_required as i32)
        .bind(member.base.created_at.to_rfc3339())
        .bind(member.base.updated_at.to_rfc3339())
        .bind(member.base.created_by.map(|id| id.to_string()))
        .bind(member.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(member)
    }

    async fn create_execution(&self, pool: &SqlitePool, exec: RuleExecution) -> Result<RuleExecution> {
        sqlx::query(
            r#"INSERT INTO rule_executions (id, rule_id, ruleset_id, entity_type, entity_id,
               triggered_at, conditions_evaluated, matched, actions_executed, result, error,
               execution_time_ms, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(exec.base.id.to_string())
        .bind(exec.rule_id.to_string())
        .bind(exec.ruleset_id.map(|id| id.to_string()))
        .bind(&exec.entity_type)
        .bind(exec.entity_id.to_string())
        .bind(exec.triggered_at.to_rfc3339())
        .bind(&exec.conditions_evaluated)
        .bind(exec.matched as i32)
        .bind(&exec.actions_executed)
        .bind(exec.result.as_ref())
        .bind(exec.error.as_ref())
        .bind(exec.execution_time_ms)
        .bind(exec.base.created_at.to_rfc3339())
        .bind(exec.base.updated_at.to_rfc3339())
        .bind(exec.base.created_by.map(|id| id.to_string()))
        .bind(exec.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(exec)
    }

    async fn create_variable(&self, pool: &SqlitePool, var: RuleVariable) -> Result<RuleVariable> {
        sqlx::query(
            r#"INSERT INTO rule_variables (id, name, code, description, data_type, default_value,
               source_type, source_config, is_constant, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(var.base.id.to_string())
        .bind(&var.name)
        .bind(&var.code)
        .bind(&var.description)
        .bind(&var.data_type)
        .bind(&var.default_value)
        .bind(format!("{:?}", var.source_type))
        .bind(&var.source_config)
        .bind(var.is_constant as i32)
        .bind(format!("{:?}", var.status))
        .bind(var.base.created_at.to_rfc3339())
        .bind(var.base.updated_at.to_rfc3339())
        .bind(var.base.created_by.map(|id| id.to_string()))
        .bind(var.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(var)
    }

    async fn list_variables(&self, pool: &SqlitePool) -> Result<Vec<RuleVariable>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, code, description, data_type, default_value, source_type,
               source_config, is_constant, status, created_at, updated_at FROM rule_variables"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| RuleVariable {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            data_type: row.get("data_type"),
            default_value: row.get("default_value"),
            source_type: VariableSource::Static,
            source_config: row.get("source_config"),
            is_constant: row.get::<i32, _>("is_constant") == 1,
            status: erp_core::Status::Active,
        }).collect())
    }

    async fn create_function(&self, pool: &SqlitePool, func: RuleFunction) -> Result<RuleFunction> {
        sqlx::query(
            r#"INSERT INTO rule_functions (id, name, code, description, return_type, parameters,
               function_body, is_builtin, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(func.base.id.to_string())
        .bind(&func.name)
        .bind(&func.code)
        .bind(&func.description)
        .bind(&func.return_type)
        .bind(&func.parameters)
        .bind(&func.function_body)
        .bind(func.is_builtin as i32)
        .bind(format!("{:?}", func.status))
        .bind(func.base.created_at.to_rfc3339())
        .bind(func.base.updated_at.to_rfc3339())
        .bind(func.base.created_by.map(|id| id.to_string()))
        .bind(func.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(func)
    }

    async fn list_functions(&self, pool: &SqlitePool) -> Result<Vec<RuleFunction>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, code, description, return_type, parameters, function_body,
               is_builtin, status, created_at, updated_at FROM rule_functions"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| RuleFunction {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            return_type: row.get("return_type"),
            parameters: row.get("parameters"),
            function_body: row.get("function_body"),
            is_builtin: row.get::<i32, _>("is_builtin") == 1,
            status: erp_core::Status::Active,
        }).collect())
    }

    async fn create_template(&self, pool: &SqlitePool, template: RuleTemplate) -> Result<RuleTemplate> {
        sqlx::query(
            r#"INSERT INTO rule_templates (id, name, code, description, rule_type, entity_type,
               template, variables, is_builtin, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(template.base.id.to_string())
        .bind(&template.name)
        .bind(&template.code)
        .bind(&template.description)
        .bind(format!("{:?}", template.rule_type))
        .bind(&template.entity_type)
        .bind(&template.template)
        .bind(&template.variables)
        .bind(template.is_builtin as i32)
        .bind(format!("{:?}", template.status))
        .bind(template.base.created_at.to_rfc3339())
        .bind(template.base.updated_at.to_rfc3339())
        .bind(template.base.created_by.map(|id| id.to_string()))
        .bind(template.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(template)
    }

    async fn list_templates(&self, pool: &SqlitePool) -> Result<Vec<RuleTemplate>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, code, description, rule_type, entity_type, template, variables,
               is_builtin, status, created_at, updated_at FROM rule_templates"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| RuleTemplate {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            rule_type: RuleType::Validation,
            entity_type: row.get("entity_type"),
            template: row.get("template"),
            variables: row.get("variables"),
            is_builtin: row.get::<i32, _>("is_builtin") == 1,
            status: erp_core::Status::Active,
        }).collect())
    }

    async fn create_decision_table(&self, pool: &SqlitePool, table: DecisionTable) -> Result<DecisionTable> {
        sqlx::query(
            r#"INSERT INTO decision_tables (id, name, code, description, entity_type, input_columns,
               output_columns, hit_policy, status, version, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(table.base.id.to_string())
        .bind(&table.name)
        .bind(&table.code)
        .bind(&table.description)
        .bind(&table.entity_type)
        .bind(&table.input_columns)
        .bind(&table.output_columns)
        .bind(format!("{:?}", table.hit_policy))
        .bind(format!("{:?}", table.status))
        .bind(table.version)
        .bind(table.base.created_at.to_rfc3339())
        .bind(table.base.updated_at.to_rfc3339())
        .bind(table.base.created_by.map(|id| id.to_string()))
        .bind(table.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(table)
    }

    async fn get_decision_table(&self, pool: &SqlitePool, id: Uuid) -> Result<DecisionTable> {
        let row = sqlx::query(
            r#"SELECT id, name, code, description, entity_type, input_columns, output_columns,
               hit_policy, status, version, created_at, updated_at FROM decision_tables WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(DecisionTable {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: row.get("name"),
            code: row.get("code"),
            description: row.get("description"),
            entity_type: row.get("entity_type"),
            input_columns: row.get("input_columns"),
            output_columns: row.get("output_columns"),
            hit_policy: HitPolicy::First,
            status: RuleStatus::Active,
            version: row.get("version"),
        })
    }

    async fn create_decision_row(&self, pool: &SqlitePool, row: DecisionTableRow) -> Result<DecisionTableRow> {
        sqlx::query(
            r#"INSERT INTO decision_table_rows (id, table_id, row_number, inputs, outputs,
               description, is_active, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(row.base.id.to_string())
        .bind(row.table_id.to_string())
        .bind(row.row_number)
        .bind(&row.inputs)
        .bind(&row.outputs)
        .bind(&row.description)
        .bind(row.is_active as i32)
        .bind(row.base.created_at.to_rfc3339())
        .bind(row.base.updated_at.to_rfc3339())
        .bind(row.base.created_by.map(|id| id.to_string()))
        .bind(row.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(row)
    }

    async fn list_decision_rows(&self, pool: &SqlitePool, table_id: Uuid) -> Result<Vec<DecisionTableRow>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, table_id, row_number, inputs, outputs, description, is_active,
               created_at, updated_at FROM decision_table_rows WHERE table_id = ? ORDER BY row_number"#,
        )
        .bind(table_id.to_string())
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| DecisionTableRow {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            table_id: Uuid::parse_str(r.get::<&str, _>("table_id")).unwrap(),
            row_number: r.get("row_number"),
            inputs: r.get("inputs"),
            outputs: r.get("outputs"),
            description: r.get("description"),
            is_active: r.get::<i32, _>("is_active") == 1,
        }).collect())
    }
}
