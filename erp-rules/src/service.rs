use sqlx::SqlitePool;
use uuid::Uuid;
use serde_json::Value;
use erp_core::{Result, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct RulesService {
    repo: SqliteRulesRepository,
}

impl RulesService {
    pub fn new() -> Self {
        Self { repo: SqliteRulesRepository }
    }

    pub async fn create_rule(&self, pool: &SqlitePool, rule: BusinessRule) -> Result<BusinessRule> {
        self.repo.create_rule(pool, rule).await
    }

    pub async fn get_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<BusinessRule> {
        self.repo.get_rule(pool, id).await
    }

    pub async fn get_rule_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Option<BusinessRule>> {
        self.repo.get_rule_by_code(pool, code).await
    }

    pub async fn list_rules(&self, pool: &SqlitePool, entity_type: Option<&str>) -> Result<Vec<BusinessRule>> {
        self.repo.list_rules(pool, entity_type).await
    }

    pub async fn update_rule(&self, pool: &SqlitePool, rule: BusinessRule) -> Result<BusinessRule> {
        self.repo.update_rule(pool, rule).await
    }

    pub async fn delete_rule(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete_rule(pool, id).await
    }

    pub async fn create_ruleset(&self, pool: &SqlitePool, ruleset: RuleSet) -> Result<RuleSet> {
        self.repo.create_ruleset(pool, ruleset).await
    }

    pub async fn get_ruleset(&self, pool: &SqlitePool, id: Uuid) -> Result<RuleSet> {
        self.repo.get_ruleset(pool, id).await
    }

    pub async fn list_rulesets(&self, pool: &SqlitePool) -> Result<Vec<RuleSet>> {
        self.repo.list_rulesets(pool).await
    }

    pub async fn add_rule_to_ruleset(&self, pool: &SqlitePool, ruleset_id: Uuid, rule_id: Uuid, sort_order: i32) -> Result<RuleSetMember> {
        let member = RuleSetMember {
            base: BaseEntity::new(),
            ruleset_id,
            rule_id,
            sort_order,
            is_required: false,
        };
        self.repo.add_rule_to_ruleset(pool, member).await
    }

    pub async fn execute_rules(&self, pool: &SqlitePool, entity_type: &str, entity_id: Uuid, context: Value) -> Result<Vec<RuleExecution>> {
        let rules = self.repo.list_rules(pool, Some(entity_type)).await?;
        let mut executions = Vec::new();
        
        for rule in rules {
            let start = std::time::Instant::now();
            let matched = self.evaluate_conditions(&rule.conditions, &context);
            
            let actions_executed = if matched {
                rule.actions.clone()
            } else {
                rule.else_actions.clone().unwrap_or_default()
            };
            
            let exec = RuleExecution {
                base: BaseEntity::new(),
                rule_id: rule.base.id,
                ruleset_id: None,
                entity_type: entity_type.to_string(),
                entity_id,
                triggered_at: chrono::Utc::now(),
                conditions_evaluated: rule.conditions.clone(),
                matched,
                actions_executed: actions_executed.clone(),
                result: Some(actions_executed),
                error: None,
                execution_time_ms: start.elapsed().as_millis() as i32,
            };
            
            let saved = self.repo.create_execution(pool, exec).await?;
            executions.push(saved);
        }
        
        Ok(executions)
    }

    fn evaluate_conditions(&self, conditions: &str, context: &Value) -> bool {
        if conditions.is_empty() || conditions == "{}" {
            return true;
        }
        
        if let Ok(cond) = serde_json::from_str::<Value>(conditions) {
            self.evaluate_condition(&cond, context)
        } else {
            false
        }
    }

    fn evaluate_condition(&self, cond: &Value, context: &Value) -> bool {
        if let Some(obj) = cond.as_object() {
            if let (Some(field), Some(op), Some(val)) = 
                (obj.get("field"), obj.get("operator"), obj.get("value")) {
                let field_val = context.get(field.as_str().unwrap_or(""));
                if field_val.is_none() {
                    return false;
                }
                let field_val = field_val.unwrap();
                
                return match op.as_str().unwrap_or("") {
                    "equals" => field_val == val,
                    "notEquals" => field_val != val,
                    "contains" => {
                        if let (Some(s1), Some(s2)) = (field_val.as_str(), val.as_str()) {
                            s1.contains(s2)
                        } else {
                            false
                        }
                    },
                    "greaterThan" => {
                        if let (Some(n1), Some(n2)) = (field_val.as_f64(), val.as_f64()) {
                            n1 > n2
                        } else {
                            false
                        }
                    },
                    "lessThan" => {
                        if let (Some(n1), Some(n2)) = (field_val.as_f64(), val.as_f64()) {
                            n1 < n2
                        } else {
                            false
                        }
                    },
                    _ => false,
                };
            }
            
            if let Some(and) = obj.get("and") {
                if let Some(arr) = and.as_array() {
                    return arr.iter().all(|c| self.evaluate_condition(c, context));
                }
            }
            
            if let Some(or) = obj.get("or") {
                if let Some(arr) = or.as_array() {
                    return arr.iter().any(|c| self.evaluate_condition(c, context));
                }
            }
        }
        false
    }

    pub async fn create_variable(&self, pool: &SqlitePool, var: RuleVariable) -> Result<RuleVariable> {
        self.repo.create_variable(pool, var).await
    }

    pub async fn list_variables(&self, pool: &SqlitePool) -> Result<Vec<RuleVariable>> {
        self.repo.list_variables(pool).await
    }

    pub async fn create_function(&self, pool: &SqlitePool, func: RuleFunction) -> Result<RuleFunction> {
        self.repo.create_function(pool, func).await
    }

    pub async fn list_functions(&self, pool: &SqlitePool) -> Result<Vec<RuleFunction>> {
        self.repo.list_functions(pool).await
    }

    pub async fn create_template(&self, pool: &SqlitePool, template: RuleTemplate) -> Result<RuleTemplate> {
        self.repo.create_template(pool, template).await
    }

    pub async fn list_templates(&self, pool: &SqlitePool) -> Result<Vec<RuleTemplate>> {
        self.repo.list_templates(pool).await
    }

    pub async fn create_decision_table(&self, pool: &SqlitePool, table: DecisionTable) -> Result<DecisionTable> {
        self.repo.create_decision_table(pool, table).await
    }

    pub async fn get_decision_table(&self, pool: &SqlitePool, id: Uuid) -> Result<DecisionTable> {
        self.repo.get_decision_table(pool, id).await
    }

    pub async fn add_decision_row(&self, pool: &SqlitePool, table_id: Uuid, row_number: i32, inputs: String, outputs: String) -> Result<DecisionTableRow> {
        let row = DecisionTableRow {
            base: BaseEntity::new(),
            table_id,
            row_number,
            inputs,
            outputs,
            description: None,
            is_active: true,
        };
        self.repo.create_decision_row(pool, row).await
    }

    pub async fn evaluate_decision_table(&self, pool: &SqlitePool, table_id: Uuid, input_values: Value) -> Result<Option<Value>> {
        let rows = self.repo.list_decision_rows(pool, table_id).await?;
        
        for row in rows {
            if !row.is_active {
                continue;
            }
            
            if let Ok(inputs) = serde_json::from_str::<Value>(&row.inputs) {
                if self.match_inputs(&inputs, &input_values) {
                    if let Ok(outputs) = serde_json::from_str::<Value>(&row.outputs) {
                        return Ok(Some(outputs));
                    }
                }
            }
        }
        
        Ok(None)
    }

    fn match_inputs(&self, inputs: &Value, values: &Value) -> bool {
        if let (Some(obj1), Some(obj2)) = (inputs.as_object(), values.as_object()) {
            for (key, val) in obj1 {
                if let Some(actual) = obj2.get(key) {
                    if val != actual && !val.as_str().map(|s| s == "*").unwrap_or(false) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
