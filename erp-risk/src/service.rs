use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct RiskService;

impl RiskService {
    pub async fn create(pool: &SqlitePool, req: CreateRiskRequest, user_id: Option<Uuid>) -> Result<Risk> {
        let now = Utc::now();
        let code = format!("RSK-{}", now.format("%Y%m%d%H%M%S"));
        let risk_score = Self::calculate_score(req.probability, &req.impact);
        let inherent_risk_level = Self::score_to_level(risk_score);
        
        let risk = Risk {
            id: Uuid::new_v4(),
            code,
            title: req.title,
            description: req.description,
            category: req.category,
            status: RiskStatus::Identified,
            probability: req.probability,
            impact: req.impact.clone(),
            risk_score,
            inherent_risk_level: inherent_risk_level.clone(),
            residual_risk_level: inherent_risk_level,
            owner_id: req.owner_id,
            department: req.department,
            identified_date: now,
            target_resolution_date: req.target_resolution_date,
            actual_resolution_date: None,
            created_at: now,
            updated_at: now,
            created_by: user_id,
        };
        RiskRepository::create(pool, &risk).await?;
        Ok(risk)
    }

    fn calculate_score(probability: f64, impact: &RiskLevel) -> i32 {
        let impact_score = match impact {
            RiskLevel::VeryLow => 1,
            RiskLevel::Low => 2,
            RiskLevel::Medium => 3,
            RiskLevel::High => 4,
            RiskLevel::VeryHigh => 5,
            RiskLevel::Critical => 6,
        };
        (probability * impact_score as f64 * 10.0) as i32
    }

    fn score_to_level(score: i32) -> RiskLevel {
        match score {
            0..=10 => RiskLevel::VeryLow,
            11..=20 => RiskLevel::Low,
            21..=35 => RiskLevel::Medium,
            36..=50 => RiskLevel::High,
            51..=70 => RiskLevel::VeryHigh,
            _ => RiskLevel::Critical,
        }
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<Option<Risk>> {
        RiskRepository::get_by_id(pool, id).await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Risk>> {
        RiskRepository::list_all(pool).await
    }

    pub async fn list_by_category(pool: &SqlitePool, category: RiskCategory) -> Result<Vec<Risk>> {
        let all = RiskRepository::list_all(pool).await?;
        Ok(all.into_iter().filter(|r| std::mem::discriminant(&r.category) == std::mem::discriminant(&category)).collect())
    }

    pub async fn update_status(pool: &SqlitePool, id: Uuid, status: RiskStatus) -> Result<Risk> {
        let now = Utc::now();
        let status_str = format!("{:?}", status);
        let resolution_date = if matches!(status, RiskStatus::Closed) { Some(now.to_rfc3339()) } else { None };
        
        sqlx::query(
            r#"UPDATE risks SET status = ?, actual_resolution_date = COALESCE(?, actual_resolution_date), updated_at = ? WHERE id = ?"#
        )
        .bind(&status_str)
        .bind(resolution_date)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool).await?;
        
        RiskRepository::get_by_id(pool, id).await?.ok_or_else(|| anyhow::anyhow!("Risk not found"))
    }

    pub async fn assess(pool: &SqlitePool, risk_id: Uuid, probability: f64, impact: RiskLevel, assessor_id: Option<Uuid>) -> Result<RiskAssessment> {
        let now = Utc::now();
        let risk = RiskRepository::get_by_id(pool, risk_id).await?.ok_or_else(|| anyhow::anyhow!("Risk not found"))?;
        
        let assessment = RiskAssessment {
            id: Uuid::new_v4(),
            risk_id,
            assessment_date: now,
            assessor_id,
            probability_before: risk.probability,
            impact_before: risk.impact.clone(),
            probability_after: probability,
            impact_after: impact.clone(),
            notes: None,
            created_at: now,
        };
        
        sqlx::query(
            r#"INSERT INTO risk_assessments (id, risk_id, assessment_date, assessor_id, probability_before, impact_before, probability_after, impact_after, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(assessment.id.to_string())
        .bind(assessment.risk_id.to_string())
        .bind(assessment.assessment_date.to_rfc3339())
        .bind(assessment.assessor_id.map(|id| id.to_string()))
        .bind(assessment.probability_before)
        .bind(format!("{:?}", assessment.impact_before))
        .bind(assessment.probability_after)
        .bind(format!("{:?}", assessment.impact_after))
        .bind(&assessment.notes)
        .bind(assessment.created_at.to_rfc3339())
        .execute(pool).await?;
        
        let new_score = Self::calculate_score(probability, &impact);
        let new_level = Self::score_to_level(new_score);
        
        sqlx::query(
            r#"UPDATE risks SET probability = ?, impact = ?, risk_score = ?, residual_risk_level = ?, status = 'Monitoring', updated_at = ? WHERE id = ?"#
        )
        .bind(probability)
        .bind(format!("{:?}", impact))
        .bind(new_score)
        .bind(format!("{:?}", new_level))
        .bind(now.to_rfc3339())
        .bind(risk_id.to_string())
        .execute(pool).await?;
        
        Ok(assessment)
    }

    pub async fn get_dashboard(pool: &SqlitePool) -> Result<RiskDashboard> {
        let risks = RiskRepository::list_all(pool).await?;
        
        let total_risks = risks.len() as i32;
        let high_risks = risks.iter().filter(|r| matches!(r.residual_risk_level, RiskLevel::High | RiskLevel::VeryHigh | RiskLevel::Critical)).count() as i32;
        let medium_risks = risks.iter().filter(|r| matches!(r.residual_risk_level, RiskLevel::Medium)).count() as i32;
        let low_risks = risks.iter().filter(|r| matches!(r.residual_risk_level, RiskLevel::Low | RiskLevel::VeryLow)).count() as i32;
        
        let mut category_counts: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        for risk in &risks {
            let cat = format!("{:?}", risk.category);
            *category_counts.entry(cat).or_insert(0) += 1;
        }
        
        Ok(RiskDashboard {
            total_risks,
            high_risks,
            medium_risks,
            low_risks,
            open_incidents: 0,
            mitigations_in_progress: 0,
            overdue_mitigations: 0,
            risk_by_category: json!(category_counts),
            risk_trend: json!({}),
        })
    }
}

pub struct MitigationService;

impl MitigationService {
    pub async fn create(pool: &SqlitePool, req: CreateMitigationRequest) -> Result<MitigationPlan> {
        let now = Utc::now();
        let plan = MitigationPlan {
            id: Uuid::new_v4(),
            risk_id: req.risk_id,
            title: req.title,
            description: req.description,
            strategy: req.strategy,
            owner_id: req.owner_id,
            status: "Planned".to_string(),
            priority: req.priority,
            start_date: now,
            target_date: req.target_date,
            completion_date: None,
            budget: req.budget,
            actual_cost: 0,
            effectiveness: None,
            created_at: now,
            updated_at: now,
        };
        MitigationPlanRepository::create(pool, &plan).await?;
        
        sqlx::query("UPDATE risks SET status = 'Mitigating', updated_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(req.risk_id.to_string())
            .execute(pool).await?;
        
        Ok(plan)
    }

    pub async fn add_task(pool: &SqlitePool, plan_id: Uuid, title: String, assigned_to: Option<Uuid>, due_date: chrono::DateTime<Utc>) -> Result<MitigationTask> {
        let now = Utc::now();
        let task = MitigationTask {
            id: Uuid::new_v4(),
            plan_id,
            title,
            description: None,
            assigned_to,
            status: "Pending".to_string(),
            due_date,
            completed_at: None,
            created_at: now,
        };
        sqlx::query(
            r#"INSERT INTO mitigation_tasks (id, plan_id, title, description, assigned_to, status, due_date, completed_at, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(task.id.to_string())
        .bind(task.plan_id.to_string())
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.assigned_to.map(|id| id.to_string()))
        .bind(&task.status)
        .bind(task.due_date.to_rfc3339())
        .bind(task.completed_at.map(|d| d.to_rfc3339()))
        .bind(task.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(task)
    }

    pub async fn complete_task(pool: &SqlitePool, task_id: Uuid) -> Result<MitigationTask> {
        let now = Utc::now();
        sqlx::query("UPDATE mitigation_tasks SET status = 'Completed', completed_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(task_id.to_string())
            .execute(pool).await?;
        
        let rows = sqlx::query("SELECT id, plan_id, title, description, assigned_to, status, due_date, completed_at, created_at FROM mitigation_tasks WHERE id = ?")
            .bind(task_id.to_string())
            .fetch_all(pool).await?;
        
        Ok(MitigationTask {
            id: Uuid::parse_str(rows[0].get::<String, _>("id").as_str()).unwrap(),
            plan_id: Uuid::parse_str(rows[0].get::<String, _>("plan_id").as_str()).unwrap(),
            title: rows[0].get("title"),
            description: rows[0].get("description"),
            assigned_to: rows[0].get::<Option<String>, _>("assigned_to").and_then(|s| Uuid::parse_str(&s).ok()),
            status: rows[0].get("status"),
            due_date: chrono::DateTime::parse_from_rfc3339(&rows[0].get::<String, _>("due_date")).unwrap().with_timezone(&chrono::Utc),
            completed_at: Some(now),
            created_at: chrono::DateTime::parse_from_rfc3339(&rows[0].get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
        })
    }
}

pub struct RiskControlService;

impl RiskControlService {
    pub async fn create(pool: &SqlitePool, code: String, name: String, description: String, control_type: String, frequency: String, owner_id: Option<Uuid>) -> Result<RiskControl> {
        let now = Utc::now();
        let control = RiskControl {
            id: Uuid::new_v4(),
            code,
            name,
            description,
            control_type,
            frequency,
            owner_id,
            effectiveness: "Effective".to_string(),
            last_test_date: None,
            next_test_date: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        RiskControlRepository::create(pool, &control).await?;
        Ok(control)
    }

    pub async fn link_to_risk(pool: &SqlitePool, risk_id: Uuid, control_id: Uuid, effectiveness: String) -> Result<RiskControlMapping> {
        let now = Utc::now();
        let mapping = RiskControlMapping {
            id: Uuid::new_v4(),
            risk_id,
            control_id,
            control_effectiveness: effectiveness,
            created_at: now,
        };
        sqlx::query(
            r#"INSERT INTO risk_control_mappings (id, risk_id, control_id, control_effectiveness, created_at)
               VALUES (?, ?, ?, ?, ?)"#
        )
        .bind(mapping.id.to_string())
        .bind(mapping.risk_id.to_string())
        .bind(mapping.control_id.to_string())
        .bind(&mapping.control_effectiveness)
        .bind(mapping.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(mapping)
    }
}
