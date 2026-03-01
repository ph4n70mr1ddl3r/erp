use crate::models::*;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct RiskRepository;

impl RiskRepository {
    pub async fn create(pool: &SqlitePool, risk: &Risk) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO risks (id, code, title, description, category, status, probability, impact, risk_score, inherent_risk_level, residual_risk_level, owner_id, department, identified_date, target_resolution_date, actual_resolution_date, created_at, updated_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(risk.id.to_string())
        .bind(&risk.code)
        .bind(&risk.title)
        .bind(&risk.description)
        .bind(format!("{:?}", risk.category))
        .bind(format!("{:?}", risk.status))
        .bind(risk.probability)
        .bind(format!("{:?}", risk.impact))
        .bind(risk.risk_score)
        .bind(format!("{:?}", risk.inherent_risk_level))
        .bind(format!("{:?}", risk.residual_risk_level))
        .bind(risk.owner_id.map(|id| id.to_string()))
        .bind(&risk.department)
        .bind(risk.identified_date.to_rfc3339())
        .bind(risk.target_resolution_date.map(|d| d.to_rfc3339()))
        .bind(risk.actual_resolution_date.map(|d| d.to_rfc3339()))
        .bind(risk.created_at.to_rfc3339())
        .bind(risk.updated_at.to_rfc3339())
        .bind(risk.created_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(())
    }

    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Risk>> {
        let row = sqlx::query(
            r#"SELECT id, code, title, description, category, status, probability, impact, risk_score, inherent_risk_level, residual_risk_level, owner_id, department, identified_date, target_resolution_date, actual_resolution_date, created_at, updated_at, created_by FROM risks WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        Ok(row.map(|r| Self::row_to_risk(&r)))
    }

    pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Risk>> {
        let rows = sqlx::query(
            r#"SELECT id, code, title, description, category, status, probability, impact, risk_score, inherent_risk_level, residual_risk_level, owner_id, department, identified_date, target_resolution_date, actual_resolution_date, created_at, updated_at, created_by FROM risks ORDER BY risk_score DESC"#
        )
        .fetch_all(pool).await?;
        Ok(rows.iter().map(Self::row_to_risk).collect())
    }

    fn row_to_risk(r: &sqlx::sqlite::SqliteRow) -> Risk {
        Risk {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str()).unwrap(),
            code: r.get("code"),
            title: r.get("title"),
            description: r.get("description"),
            category: match r.get::<String, _>("category").as_str() {
                "Strategic" => RiskCategory::Strategic,
                "Operational" => RiskCategory::Operational,
                "Financial" => RiskCategory::Financial,
                "Compliance" => RiskCategory::Compliance,
                "Reputational" => RiskCategory::Reputational,
                "Technology" => RiskCategory::Technology,
                "Environmental" => RiskCategory::Environmental,
                _ => RiskCategory::SupplyChain,
            },
            status: match r.get::<String, _>("status").as_str() {
                "Assessing" => RiskStatus::Assessing,
                "Mitigating" => RiskStatus::Mitigating,
                "Monitoring" => RiskStatus::Monitoring,
                "Closed" => RiskStatus::Closed,
                "Accepted" => RiskStatus::Accepted,
                _ => RiskStatus::Identified,
            },
            probability: r.get("probability"),
            impact: match r.get::<String, _>("impact").as_str() {
                "VeryLow" => RiskLevel::VeryLow,
                "Low" => RiskLevel::Low,
                "Medium" => RiskLevel::Medium,
                "High" => RiskLevel::High,
                "VeryHigh" => RiskLevel::VeryHigh,
                _ => RiskLevel::Critical,
            },
            risk_score: r.get("risk_score"),
            inherent_risk_level: match r.get::<String, _>("inherent_risk_level").as_str() {
                "VeryLow" => RiskLevel::VeryLow,
                "Low" => RiskLevel::Low,
                "Medium" => RiskLevel::Medium,
                "High" => RiskLevel::High,
                "VeryHigh" => RiskLevel::VeryHigh,
                _ => RiskLevel::Critical,
            },
            residual_risk_level: match r.get::<String, _>("residual_risk_level").as_str() {
                "VeryLow" => RiskLevel::VeryLow,
                "Low" => RiskLevel::Low,
                "Medium" => RiskLevel::Medium,
                "High" => RiskLevel::High,
                "VeryHigh" => RiskLevel::VeryHigh,
                _ => RiskLevel::Critical,
            },
            owner_id: r.get::<Option<String>, _>("owner_id").and_then(|s| Uuid::parse_str(&s).ok()),
            department: r.get("department"),
            identified_date: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("identified_date")).unwrap().with_timezone(&chrono::Utc),
            target_resolution_date: r.get::<Option<String>, _>("target_resolution_date").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            actual_resolution_date: r.get::<Option<String>, _>("actual_resolution_date").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
            created_by: r.get::<Option<String>, _>("created_by").and_then(|s| Uuid::parse_str(&s).ok()),
        }
    }
}

pub struct MitigationPlanRepository;

impl MitigationPlanRepository {
    pub async fn create(pool: &SqlitePool, plan: &MitigationPlan) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO mitigation_plans (id, risk_id, title, description, strategy, owner_id, status, priority, start_date, target_date, completion_date, budget, actual_cost, effectiveness, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(plan.id.to_string())
        .bind(plan.risk_id.to_string())
        .bind(&plan.title)
        .bind(&plan.description)
        .bind(&plan.strategy)
        .bind(plan.owner_id.map(|id| id.to_string()))
        .bind(&plan.status)
        .bind(&plan.priority)
        .bind(plan.start_date.to_rfc3339())
        .bind(plan.target_date.to_rfc3339())
        .bind(plan.completion_date.map(|d| d.to_rfc3339()))
        .bind(plan.budget)
        .bind(plan.actual_cost)
        .bind(plan.effectiveness)
        .bind(plan.created_at.to_rfc3339())
        .bind(plan.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }
}

pub struct RiskControlRepository;

impl RiskControlRepository {
    pub async fn create(pool: &SqlitePool, control: &RiskControl) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO risk_controls (id, code, name, description, control_type, frequency, owner_id, effectiveness, last_test_date, next_test_date, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(control.id.to_string())
        .bind(&control.code)
        .bind(&control.name)
        .bind(&control.description)
        .bind(&control.control_type)
        .bind(&control.frequency)
        .bind(control.owner_id.map(|id| id.to_string()))
        .bind(&control.effectiveness)
        .bind(control.last_test_date.map(|d| d.to_rfc3339()))
        .bind(control.next_test_date.map(|d| d.to_rfc3339()))
        .bind(control.is_active)
        .bind(control.created_at.to_rfc3339())
        .bind(control.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }
}
