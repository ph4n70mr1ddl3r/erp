use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use crate::repository::*;
use erp_core::Result;

pub struct BPMService {
    process_repo: SqliteProcessRepository,
}

impl Default for BPMService {
    fn default() -> Self {
        Self::new()
    }
}

impl BPMService {
    pub fn new() -> Self {
        Self { process_repo: SqliteProcessRepository }
    }

    pub async fn create_process(&self, pool: &SqlitePool, name: String, code: String, 
        category: String, owner_id: Uuid, diagram_data: serde_json::Value) -> Result<ProcessDefinition> {
        let now = Utc::now();
        let def = ProcessDefinition {
            id: Uuid::new_v4(),
            name,
            code,
            description: None,
            category,
            version: 1,
            status: "Draft".to_string(),
            bpmn_xml: None,
            diagram_data: Some(diagram_data),
            variables: None,
            forms: None,
            owner_id,
            published_at: None,
            published_by: None,
            created_at: now,
            updated_at: now,
        };
        self.process_repo.create_definition(pool, &def).await
    }

    pub async fn get_process(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<ProcessDefinition>> {
        self.process_repo.get_definition(pool, id).await
    }

    pub async fn list_processes(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<ProcessDefinition>> {
        self.process_repo.list_definitions(pool, category).await
    }

    pub async fn publish_process(&self, pool: &SqlitePool, id: Uuid, published_by: Uuid) -> Result<ProcessDefinition> {
        let now = Utc::now();
        sqlx::query(r#"
            UPDATE bpm_process_definitions SET status = 'Published', published_at = ?, published_by = ?, updated_at = ?
            WHERE id = ?
        "#)
        .bind(now.to_rfc3339())
        .bind(published_by.to_string())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await?;

        self.process_repo.get_definition(pool, id).await?.ok_or_else(|| anyhow::anyhow!("Process not found").into())
    }

    pub async fn start_instance(&self, pool: &SqlitePool, process_definition_id: Uuid, 
        started_by: Uuid, business_key: Option<String>, variables: Option<serde_json::Value>) -> Result<ProcessInstance> {
        let def = self.process_repo.get_definition(pool, process_definition_id).await?
            .ok_or_else(|| anyhow::anyhow!("Process definition not found"))?;

        let now = Utc::now();
        let instance = ProcessInstance {
            id: Uuid::new_v4(),
            process_definition_id,
            process_definition_version: def.version,
            business_key,
            status: "Running".to_string(),
            variables,
            started_by,
            started_at: now,
            completed_at: None,
            current_node_id: None,
            parent_instance_id: None,
            created_at: now,
            updated_at: now,
        };

        sqlx::query(r#"
            INSERT INTO bpm_process_instances (id, process_definition_id, process_definition_version, 
                business_key, status, variables, started_by, started_at, completed_at, current_node_id, 
                parent_instance_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(instance.id.to_string())
        .bind(instance.process_definition_id.to_string())
        .bind(instance.process_definition_version)
        .bind(&instance.business_key)
        .bind(&instance.status)
        .bind(&instance.variables)
        .bind(instance.started_by.to_string())
        .bind(instance.started_at.to_rfc3339())
        .bind(instance.completed_at.map(|d| d.to_rfc3339()))
        .bind(&instance.current_node_id)
        .bind(instance.parent_instance_id.map(|id| id.to_string()))
        .bind(instance.created_at.to_rfc3339())
        .bind(instance.updated_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(instance)
    }

    pub async fn get_instance(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<ProcessInstance>> {
        let row = sqlx::query_as::<_, (String, String, i32, Option<String>, String, Option<String>, String, String, Option<String>, Option<String>, Option<String>, String, String)>(
            "SELECT id, process_definition_id, process_definition_version, business_key, status, variables, started_by, started_at, completed_at, current_node_id, parent_instance_id, created_at, updated_at FROM bpm_process_instances WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| ProcessInstance {
            id: r.0.parse().unwrap_or_default(),
            process_definition_id: r.1.parse().unwrap_or_default(),
            process_definition_version: r.2,
            business_key: r.3,
            status: r.4.to_string(),
            variables: r.5.and_then(|s| serde_json::from_str(&s).ok()),
            started_by: r.6.parse().unwrap_or_default(),
            started_at: r.7.parse().unwrap_or_default(),
            completed_at: r.8.and_then(|s| s.parse().ok()),
            current_node_id: r.9,
            parent_instance_id: r.10.and_then(|s| s.parse().ok()),
            created_at: r.11.parse().unwrap_or_default(),
            updated_at: r.12.parse().unwrap_or_default(),
        }))
    }

    pub async fn list_active_instances(&self, pool: &SqlitePool) -> Result<Vec<ProcessInstance>> {
        let rows = sqlx::query_as::<_, (String, String, i32, Option<String>, String, Option<String>, String, String, Option<String>, Option<String>, Option<String>, String, String)>(
            "SELECT id, process_definition_id, process_definition_version, business_key, status, variables, started_by, started_at, completed_at, current_node_id, parent_instance_id, created_at, updated_at FROM bpm_process_instances WHERE status = 'Running' ORDER BY started_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| ProcessInstance {
            id: r.0.parse().unwrap_or_default(),
            process_definition_id: r.1.parse().unwrap_or_default(),
            process_definition_version: r.2,
            business_key: r.3,
            status: r.4.to_string(),
            variables: r.5.and_then(|s| serde_json::from_str(&s).ok()),
            started_by: r.6.parse().unwrap_or_default(),
            started_at: r.7.parse().unwrap_or_default(),
            completed_at: r.8.and_then(|s| s.parse().ok()),
            current_node_id: r.9,
            parent_instance_id: r.10.and_then(|s| s.parse().ok()),
            created_at: r.11.parse().unwrap_or_default(),
            updated_at: r.12.parse().unwrap_or_default(),
        }).collect())
    }

    pub async fn create_task(&self, pool: &SqlitePool, process_instance_id: Uuid, 
        process_definition_id: Uuid, node_id: String, name: String, 
        assignee_id: Option<Uuid>) -> Result<ProcessTask> {
        let now = Utc::now();
        let task = ProcessTask {
            id: Uuid::new_v4(),
            process_instance_id,
            process_definition_id,
            node_id,
            name,
            task_type: "UserTask".to_string(),
            status: "Created".to_string(),
            assignee_id,
            candidate_users: None,
            candidate_groups: None,
            form_key: None,
            form_data: None,
            variables: None,
            priority: 0,
            due_date: None,
            created_at: now,
            claimed_at: None,
            completed_at: None,
            completed_by: None,
            outcome: None,
            updated_at: now,
        };

        sqlx::query(r#"
            INSERT INTO bpm_process_tasks (id, process_instance_id, process_definition_id, node_id, 
                name, task_type, status, assignee_id, candidate_users, candidate_groups, form_key, 
                form_data, variables, priority, due_date, created_at, claimed_at, completed_at, 
                completed_by, outcome, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(task.id.to_string())
        .bind(task.process_instance_id.to_string())
        .bind(task.process_definition_id.to_string())
        .bind(&task.node_id)
        .bind(&task.name)
        .bind(&task.task_type)
        .bind(&task.status)
        .bind(task.assignee_id.map(|id| id.to_string()))
        .bind(&task.candidate_users)
        .bind(&task.candidate_groups)
        .bind(&task.form_key)
        .bind(&task.form_data)
        .bind(&task.variables)
        .bind(task.priority)
        .bind(task.due_date.map(|d| d.to_rfc3339()))
        .bind(task.created_at.to_rfc3339())
        .bind(task.claimed_at.map(|d| d.to_rfc3339()))
        .bind(task.completed_at.map(|d| d.to_rfc3339()))
        .bind(task.completed_by.map(|id| id.to_string()))
        .bind(&task.outcome)
        .bind(task.updated_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(task)
    }

    pub async fn claim_task(&self, pool: &SqlitePool, task_id: Uuid, user_id: Uuid) -> Result<ProcessTask> {
        let now = Utc::now();
        sqlx::query(r#"
            UPDATE bpm_process_tasks SET status = 'InProgress', assignee_id = ?, claimed_at = ?, updated_at = ?
            WHERE id = ? AND status IN ('Created', 'Assigned')
        "#)
        .bind(user_id.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(task_id.to_string())
        .execute(pool)
        .await?;

        let row = sqlx::query(r#"
            SELECT id, process_instance_id, process_definition_id, node_id, name, task_type, status, 
                assignee_id, candidate_users, candidate_groups, form_key, form_data, variables, 
                priority, due_date, created_at, claimed_at, completed_at, completed_by, outcome, updated_at 
            FROM bpm_process_tasks WHERE id = ?
        "#)
        .bind(task_id.to_string())
        .fetch_optional(pool)
        .await?;

        row.map(|r| ProcessTask {
            id: r.get::<String, _>("id").parse().unwrap_or_default(),
            process_instance_id: r.get::<String, _>("process_instance_id").parse().unwrap_or_default(),
            process_definition_id: r.get::<String, _>("process_definition_id").parse().unwrap_or_default(),
            node_id: r.get("node_id"),
            name: r.get("name"),
            task_type: r.get::<String, _>("task_type"),
            status: r.get::<String, _>("status"),
            assignee_id: r.get::<Option<String>, _>("assignee_id").and_then(|s| s.parse().ok()),
            candidate_users: r.get::<Option<String>, _>("candidate_users").and_then(|s| serde_json::from_str(&s).ok()),
            candidate_groups: r.get::<Option<String>, _>("candidate_groups").and_then(|s| serde_json::from_str(&s).ok()),
            form_key: r.get("form_key"),
            form_data: r.get::<Option<String>, _>("form_data").and_then(|s| serde_json::from_str(&s).ok()),
            variables: r.get::<Option<String>, _>("variables").and_then(|s| serde_json::from_str(&s).ok()),
            priority: r.get::<i32, _>("priority"),
            due_date: r.get::<Option<String>, _>("due_date").and_then(|s| s.parse().ok()),
            created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
            claimed_at: r.get::<Option<String>, _>("claimed_at").and_then(|s| s.parse().ok()),
            completed_at: r.get::<Option<String>, _>("completed_at").and_then(|s| s.parse().ok()),
            completed_by: r.get::<Option<String>, _>("completed_by").and_then(|s| s.parse().ok()),
            outcome: r.get("outcome"),
            updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
        }).ok_or_else(|| anyhow::anyhow!("Task not found").into())
    }

    pub async fn complete_task(&self, pool: &SqlitePool, task_id: Uuid, user_id: Uuid, 
        outcome: Option<String>) -> Result<ProcessTask> {
        let now = Utc::now();
        sqlx::query(r#"
            UPDATE bpm_process_tasks SET status = 'Completed', completed_at = ?, completed_by = ?, 
                outcome = ?, updated_at = ?
            WHERE id = ? AND assignee_id = ? AND status = 'InProgress'
        "#)
        .bind(now.to_rfc3339())
        .bind(user_id.to_string())
        .bind(&outcome)
        .bind(now.to_rfc3339())
        .bind(task_id.to_string())
        .bind(user_id.to_string())
        .execute(pool)
        .await?;

        let row = sqlx::query(r#"
            SELECT id, process_instance_id, process_definition_id, node_id, name, task_type, status, 
                assignee_id, candidate_users, candidate_groups, form_key, form_data, variables, 
                priority, due_date, created_at, claimed_at, completed_at, completed_by, outcome, updated_at 
            FROM bpm_process_tasks WHERE id = ?
        "#)
        .bind(task_id.to_string())
        .fetch_optional(pool)
        .await?;

        row.map(|r| ProcessTask {
            id: r.get::<String, _>("id").parse().unwrap_or_default(),
            process_instance_id: r.get::<String, _>("process_instance_id").parse().unwrap_or_default(),
            process_definition_id: r.get::<String, _>("process_definition_id").parse().unwrap_or_default(),
            node_id: r.get("node_id"),
            name: r.get("name"),
            task_type: r.get::<String, _>("task_type"),
            status: r.get::<String, _>("status"),
            assignee_id: r.get::<Option<String>, _>("assignee_id").and_then(|s| s.parse().ok()),
            candidate_users: r.get::<Option<String>, _>("candidate_users").and_then(|s| serde_json::from_str(&s).ok()),
            candidate_groups: r.get::<Option<String>, _>("candidate_groups").and_then(|s| serde_json::from_str(&s).ok()),
            form_key: r.get("form_key"),
            form_data: r.get::<Option<String>, _>("form_data").and_then(|s| serde_json::from_str(&s).ok()),
            variables: r.get::<Option<String>, _>("variables").and_then(|s| serde_json::from_str(&s).ok()),
            priority: r.get::<i32, _>("priority"),
            due_date: r.get::<Option<String>, _>("due_date").and_then(|s| s.parse().ok()),
            created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
            claimed_at: r.get::<Option<String>, _>("claimed_at").and_then(|s| s.parse().ok()),
            completed_at: r.get::<Option<String>, _>("completed_at").and_then(|s| s.parse().ok()),
            completed_by: r.get::<Option<String>, _>("completed_by").and_then(|s| s.parse().ok()),
            outcome: r.get("outcome"),
            updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
        }).ok_or_else(|| anyhow::anyhow!("Task not found").into())
    }

    pub async fn get_user_tasks(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<ProcessTask>> {
        let rows = sqlx::query(r#"
            SELECT id, process_instance_id, process_definition_id, node_id, name, task_type, status, 
                assignee_id, candidate_users, candidate_groups, form_key, form_data, variables, 
                priority, due_date, created_at, claimed_at, completed_at, completed_by, outcome, updated_at 
            FROM bpm_process_tasks 
            WHERE (assignee_id = ? OR candidate_users LIKE ?) AND status IN ('Created', 'Assigned', 'InProgress') 
            ORDER BY priority DESC, created_at ASC
        "#)
        .bind(user_id.to_string())
        .bind(format!("%{}%", user_id))
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| ProcessTask {
            id: r.get::<String, _>("id").parse().unwrap_or_default(),
            process_instance_id: r.get::<String, _>("process_instance_id").parse().unwrap_or_default(),
            process_definition_id: r.get::<String, _>("process_definition_id").parse().unwrap_or_default(),
            node_id: r.get("node_id"),
            name: r.get("name"),
            task_type: r.get::<String, _>("task_type"),
            status: r.get::<String, _>("status"),
            assignee_id: r.get::<Option<String>, _>("assignee_id").and_then(|s| s.parse().ok()),
            candidate_users: r.get::<Option<String>, _>("candidate_users").and_then(|s| serde_json::from_str(&s).ok()),
            candidate_groups: r.get::<Option<String>, _>("candidate_groups").and_then(|s| serde_json::from_str(&s).ok()),
            form_key: r.get("form_key"),
            form_data: r.get::<Option<String>, _>("form_data").and_then(|s| serde_json::from_str(&s).ok()),
            variables: r.get::<Option<String>, _>("variables").and_then(|s| serde_json::from_str(&s).ok()),
            priority: r.get::<i32, _>("priority"),
            due_date: r.get::<Option<String>, _>("due_date").and_then(|s| s.parse().ok()),
            created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
            claimed_at: r.get::<Option<String>, _>("claimed_at").and_then(|s| s.parse().ok()),
            completed_at: r.get::<Option<String>, _>("completed_at").and_then(|s| s.parse().ok()),
            completed_by: r.get::<Option<String>, _>("completed_by").and_then(|s| s.parse().ok()),
            outcome: r.get("outcome"),
            updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
        }).collect())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn add_node(&self, pool: &SqlitePool, process_definition_id: Uuid, node_id: String, 
        name: String, task_type: String, position_x: i32, position_y: i32) -> Result<ProcessNode> {
        let now = Utc::now();
        let node = ProcessNode {
            id: Uuid::new_v4(),
            process_definition_id,
            node_id: node_id.clone(),
            name,
            task_type,
            gateway_type: None,
            config: None,
            assignee_expression: None,
            candidate_groups: None,
            form_key: None,
            script: None,
            service_name: None,
            position_x,
            position_y,
            incoming_flows: None,
            outgoing_flows: None,
            created_at: now,
            updated_at: now,
        };

        sqlx::query(r#"
            INSERT INTO bpm_process_nodes (id, process_definition_id, node_id, name, task_type, 
                gateway_type, config, assignee_expression, candidate_groups, form_key, script, 
                service_name, position_x, position_y, incoming_flows, outgoing_flows, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(node.id.to_string())
        .bind(process_definition_id.to_string())
        .bind(&node.node_id)
        .bind(&node.name)
        .bind(&node.task_type)
        .bind(&node.gateway_type)
        .bind(&node.config)
        .bind(&node.assignee_expression)
        .bind(&node.candidate_groups)
        .bind(&node.form_key)
        .bind(&node.script)
        .bind(&node.service_name)
        .bind(node.position_x)
        .bind(node.position_y)
        .bind(&node.incoming_flows)
        .bind(&node.outgoing_flows)
        .bind(node.created_at.to_rfc3339())
        .bind(node.updated_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(node)
    }

    pub async fn add_flow(&self, pool: &SqlitePool, process_definition_id: Uuid, flow_id: String, 
        source_node_id: String, target_node_id: String, condition_expression: Option<String>) -> Result<ProcessFlow> {
        let now = Utc::now();
        let flow = ProcessFlow {
            id: Uuid::new_v4(),
            process_definition_id,
            flow_id: flow_id.clone(),
            name: None,
            source_node_id: source_node_id.clone(),
            target_node_id: target_node_id.clone(),
            condition_expression: condition_expression.clone(),
            is_default: condition_expression.is_none(),
            created_at: now,
            updated_at: now,
        };

        sqlx::query(r#"
            INSERT INTO bpm_process_flows (id, process_definition_id, flow_id, name, source_node_id, 
                target_node_id, condition_expression, is_default, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(flow.id.to_string())
        .bind(process_definition_id.to_string())
        .bind(&flow.flow_id)
        .bind(&flow.name)
        .bind(&flow.source_node_id)
        .bind(&flow.target_node_id)
        .bind(&flow.condition_expression)
        .bind(flow.is_default)
        .bind(flow.created_at.to_rfc3339())
        .bind(flow.updated_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(flow)
    }
}
