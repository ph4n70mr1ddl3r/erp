use anyhow::Result;
use uuid::Uuid;
use erp_core::models::BaseEntity;
use crate::models::*;
use crate::repository::{FraudRepository, SqliteFraudRepository};
use chrono::Utc;

pub struct FraudService {
    repo: SqliteFraudRepository,
}

impl FraudService {
    pub fn new() -> Self {
        Self {
            repo: SqliteFraudRepository::new(),
        }
    }
    
    pub async fn create_alert(&self, request: CreateAlertRequest) -> Result<FraudAlert> {
        let alert = FraudAlert {
            base: BaseEntity::new(),
            alert_type: request.alert_type,
            severity: request.severity,
            status: AlertStatus::New,
            entity_type: request.entity_type,
            entity_id: request.entity_id,
            rule_id: request.rule_id,
            score: request.score,
            risk_factors: request.risk_factors,
            description: request.description,
            detected_at: Utc::now(),
            reviewed_by: None,
            reviewed_at: None,
            resolution: None,
            assigned_to: None,
            due_date: None,
        };
        
        self.repo.create_alert(&alert).await?;
        Ok(alert)
    }
    
    pub async fn get_alert(&self, id: Uuid) -> Result<Option<FraudAlert>> {
        self.repo.get_alert(id).await
    }
    
    pub async fn list_alerts(&self, status: Option<AlertStatus>, severity: Option<AlertSeverity>, limit: i64) -> Result<Vec<FraudAlert>> {
        self.repo.list_alerts(status, severity, limit).await
    }
    
    pub async fn review_alert(&self, id: Uuid, reviewer_id: Uuid, request: ReviewAlertRequest) -> Result<FraudAlert> {
        let mut alert = self.repo.get_alert(id).await?
            .ok_or_else(|| anyhow::anyhow!("Alert not found"))?;
        
        alert.status = request.status;
        alert.reviewed_by = Some(reviewer_id);
        alert.reviewed_at = Some(Utc::now());
        alert.resolution = request.resolution;
        
        self.repo.update_alert(&alert).await?;
        Ok(alert)
    }
    
    pub async fn assign_alert(&self, id: Uuid, assignee_id: Uuid) -> Result<FraudAlert> {
        let mut alert = self.repo.get_alert(id).await?
            .ok_or_else(|| anyhow::anyhow!("Alert not found"))?;
        
        alert.assigned_to = Some(assignee_id);
        alert.status = AlertStatus::UnderReview;
        
        self.repo.update_alert(&alert).await?;
        Ok(alert)
    }
    
    pub async fn escalate_alert(&self, id: Uuid) -> Result<FraudAlert> {
        let mut alert = self.repo.get_alert(id).await?
            .ok_or_else(|| anyhow::anyhow!("Alert not found"))?;
        
        alert.status = AlertStatus::Escalated;
        alert.severity = match alert.severity {
            AlertSeverity::Low => AlertSeverity::Medium,
            AlertSeverity::Medium => AlertSeverity::High,
            AlertSeverity::High => AlertSeverity::Critical,
            AlertSeverity::Critical => AlertSeverity::Critical,
        };
        
        self.repo.update_alert(&alert).await?;
        Ok(alert)
    }
    
    pub async fn create_rule(&self, rule: FraudRule) -> Result<FraudRule> {
        self.repo.create_rule(&rule).await?;
        Ok(rule)
    }
    
    pub async fn list_rules(&self, enabled_only: bool) -> Result<Vec<FraudRule>> {
        self.repo.list_rules(enabled_only).await
    }
    
    pub async fn evaluate_transaction(&self, entity_type: &str, entity_id: Uuid, transaction_data: serde_json::Value) -> Result<Vec<FraudAlert>> {
        let rules = self.repo.list_rules(true).await?;
        let mut alerts = Vec::new();
        
        for rule in rules {
            if self.matches_rule(&rule, entity_type, &transaction_data)? {
                let score = self.calculate_score(&rule, &transaction_data);
                let risk_factors = self.extract_risk_factors(&rule, &transaction_data);
                
                let alert = self.create_alert(CreateAlertRequest {
                    alert_type: AlertType::TransactionAnomaly,
                    severity: self.severity_from_score(score),
                    entity_type: entity_type.to_string(),
                    entity_id,
                    rule_id: Some(rule.base.id),
                    score,
                    risk_factors,
                    description: format!("Rule '{}' triggered", rule.name),
                }).await?;
                
                alerts.push(alert);
            }
        }
        
        Ok(alerts)
    }
    
    fn matches_rule(&self, rule: &FraudRule, entity_type: &str, data: &serde_json::Value) -> Result<bool> {
        let mut matches = true;
        
        for condition in &rule.conditions {
            let field_value = data.get(&condition.field);
            let condition_met = match &condition.operator {
                ConditionOperator::Equals => {
                    field_value == Some(&condition.value)
                }
                ConditionOperator::GreaterThan => {
                    field_value.and_then(|v| v.as_f64()).unwrap_or(0.0) > condition.value.as_f64().unwrap_or(f64::MAX)
                }
                ConditionOperator::LessThan => {
                    field_value.and_then(|v| v.as_f64()).unwrap_or(f64::MAX) < condition.value.as_f64().unwrap_or(0.0)
                }
                ConditionOperator::Contains => {
                    field_value.and_then(|v| v.as_str()).map(|s| s.contains(condition.value.as_str().unwrap_or(""))).unwrap_or(false)
                }
                _ => true
            };
            
            if !condition_met {
                matches = false;
                break;
            }
        }
        
        Ok(matches)
    }
    
    fn calculate_score(&self, rule: &FraudRule, data: &serde_json::Value) -> f64 {
        let mut score = 0.5;
        
        for condition in &rule.conditions {
            score += condition.weight * 0.1;
        }
        
        score.min(1.0)
    }
    
    fn extract_risk_factors(&self, rule: &FraudRule, data: &serde_json::Value) -> Vec<RiskFactor> {
        rule.conditions.iter().map(|c| RiskFactor {
            factor_type: c.field.clone(),
            description: format!("{} condition matched", c.field),
            weight: c.weight,
            value: c.value.clone(),
            contribution: c.weight * 0.1,
        }).collect()
    }
    
    fn severity_from_score(&self, score: f64) -> AlertSeverity {
        if score >= 0.9 {
            AlertSeverity::Critical
        } else if score >= 0.7 {
            AlertSeverity::High
        } else if score >= 0.5 {
            AlertSeverity::Medium
        } else {
            AlertSeverity::Low
        }
    }
    
    pub async fn create_case(&self, alert_ids: Vec<Uuid>, title: String, description: String) -> Result<FraudCase> {
        let case_number = format!("FC-{}", Utc::now().format("%Y%m%d%H%M"));
        
        let case = FraudCase {
            base: BaseEntity::new(),
            case_number,
            title,
            description,
            status: CaseStatus::Open,
            priority: CasePriority::Medium,
            assigned_investigator: None,
            alert_ids,
            related_entities: Vec::new(),
            timeline: vec![CaseEvent {
                timestamp: Utc::now(),
                event_type: "created".to_string(),
                description: "Case created".to_string(),
                user_id: None,
                metadata: serde_json::json!({}),
            }],
            evidence: Vec::new(),
            estimated_loss: 0,
            actual_loss: None,
            recovery_amount: None,
            opened_at: Utc::now(),
            closed_at: None,
            resolution: None,
        };
        
        self.repo.create_case(&case).await?;
        Ok(case)
    }
    
    pub async fn get_case(&self, id: Uuid) -> Result<Option<FraudCase>> {
        self.repo.get_case(id).await
    }
    
    pub async fn list_cases(&self, status: Option<CaseStatus>, limit: i64) -> Result<Vec<FraudCase>> {
        self.repo.list_cases(status, limit).await
    }
    
    pub async fn add_evidence(&self, case_id: Uuid, evidence: Evidence) -> Result<FraudCase> {
        let mut case = self.repo.get_case(case_id).await?
            .ok_or_else(|| anyhow::anyhow!("Case not found"))?;
        
        case.evidence.push(evidence);
        self.repo.update_case(&case).await?;
        Ok(case)
    }
    
    pub async fn resolve_case(&self, case_id: Uuid, resolution: CaseResolution) -> Result<FraudCase> {
        let mut case = self.repo.get_case(case_id).await?
            .ok_or_else(|| anyhow::anyhow!("Case not found"))?;
        
        case.status = CaseStatus::Closed;
        case.closed_at = Some(Utc::now());
        case.resolution = Some(resolution);
        
        self.repo.update_case(&case).await?;
        Ok(case)
    }
    
    pub async fn calculate_vendor_risk(&self, vendor_id: Uuid) -> Result<VendorRiskProfile> {
        let profile = VendorRiskProfile {
            vendor_id,
            risk_score: 0.25,
            risk_level: RiskLevel::Low,
            risk_factors: vec![RiskFactor {
                factor_type: "payment_history".to_string(),
                description: "Good payment history".to_string(),
                weight: 0.3,
                value: serde_json::json!({"on_time_rate": 0.95}),
                contribution: 0.05,
            }],
            historical_alerts: 0,
            payment_anomalies: 0,
            days_since_first_transaction: 365,
            total_transaction_value: 10000000,
            average_transaction_value: 5000,
            transaction_count: 200,
            duplicate_invoice_attempts: 0,
            address_changes: 0,
            bank_account_changes: 0,
        };
        
        self.repo.save_vendor_risk_profile(&profile).await?;
        Ok(profile)
    }
    
    pub async fn calculate_employee_risk(&self, employee_id: Uuid) -> Result<EmployeeRiskProfile> {
        let profile = EmployeeRiskProfile {
            employee_id,
            risk_score: 0.15,
            risk_level: RiskLevel::VeryLow,
            risk_factors: vec![],
            expense_anomalies: 0,
            access_violations: 0,
            policy_violations: 0,
            total_expense_value: 500000,
            average_expense_value: 250,
            expense_count: 200,
            after_hours_access: 5,
            data_export_count: 2,
        };
        
        self.repo.save_employee_risk_profile(&profile).await?;
        Ok(profile)
    }
    
    pub async fn get_analytics(&self, period_start: chrono::DateTime<Utc>, period_end: chrono::DateTime<Utc>) -> Result<FraudAnalytics> {
        self.repo.get_analytics(period_start, period_end).await
    }
}
