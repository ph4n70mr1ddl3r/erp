use crate::models::*;
use crate::repository::MDMRepository;
use chrono::Utc;
use regex::Regex;
use uuid::Uuid;

pub struct MDMService<R: MDMRepository> {
    repo: R,
}

impl<R: MDMRepository> MDMService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_golden_record(&self, req: CreateGoldenRecordRequest) -> anyhow::Result<GoldenRecord> {
        let now = Utc::now();
        let golden_code = format!("GR-{}-{}", req.entity_type, now.format("%Y%m%d%H%M%S"));
        let record = GoldenRecord {
            id: Uuid::new_v4(),
            entity_type: req.entity_type,
            golden_code,
            name: req.name,
            attributes: req.attributes,
            source_count: req.source_entity_ids.len() as i32,
            confidence_score: 100,
            steward_id: None,
            status: GovernanceStatus::Active,
            version: 1,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_golden_record(&record).await?;
        
        for (i, source_id) in req.source_entity_ids.iter().enumerate() {
            let source = GoldenRecordSource {
                id: Uuid::new_v4(),
                golden_record_id: record.id,
                source_entity_id: *source_id,
                match_score: 100,
                is_primary: i == 0,
                contributed_at: now,
                created_at: now,
            };
            self.repo.create_golden_record_source(&source).await?;
        }
        
        Ok(record)
    }

    pub async fn get_golden_record(&self, id: Uuid) -> anyhow::Result<Option<GoldenRecord>> {
        self.repo.get_golden_record(id).await
    }

    pub async fn create_quality_rule(&self, req: CreateDataQualityRuleRequest) -> anyhow::Result<DataQualityRule> {
        let now = Utc::now();
        let rule = DataQualityRule {
            id: Uuid::new_v4(),
            rule_code: req.rule_code,
            name: req.name,
            description: req.description,
            entity_type: req.entity_type,
            field_name: req.field_name,
            rule_type: req.rule_type,
            rule_expression: req.rule_expression,
            severity: req.severity,
            is_active: true,
            auto_fix: req.auto_fix,
            fix_expression: req.fix_expression,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_quality_rule(&rule).await?;
        Ok(rule)
    }

    pub async fn run_quality_check(&self, entity_id: Uuid) -> anyhow::Result<Vec<DataQualityViolation>> {
        let entity = self.repo.get_master_entity(entity_id).await?
            .ok_or_else(|| anyhow::anyhow!("Entity not found"))?;
        
        let rules = self.repo.list_quality_rules(Some(&entity.entity_type)).await?;
        let mut violations = Vec::new();
        
        for rule in rules {
            if let Some(violation) = self.evaluate_rule(&entity, &rule).await? {
                violations.push(violation);
            }
        }
        
        Ok(violations)
    }

    async fn evaluate_rule(&self, entity: &MasterDataEntity, rule: &DataQualityRule) -> anyhow::Result<Option<DataQualityViolation>> {
        let is_violation = match rule.rule_type.as_str() {
            "required" => entity.entity_name.is_empty(),
            "regex" => {
                if let Ok(re) = Regex::new(&rule.rule_expression) {
                    !re.is_match(&entity.entity_code)
                } else {
                    false
                }
            }
            "length" => {
                let max_len: i32 = rule.rule_expression.parse().unwrap_or(255);
                entity.entity_name.len() > max_len as usize
            }
            "range" => {
                let parts: Vec<&str> = rule.rule_expression.split('-').collect();
                if parts.len() == 2 {
                    let min: i32 = parts[0].parse().unwrap_or(0);
                    let max: i32 = parts[1].parse().unwrap_or(100);
                    entity.quality_score < min || entity.quality_score > max
                } else {
                    false
                }
            }
            _ => false,
        };
        
        if is_violation {
            let violation = DataQualityViolation {
                id: Uuid::new_v4(),
                rule_id: rule.id,
                entity_id: entity.id,
                entity_type: entity.entity_type.clone(),
                field_name: rule.field_name.clone(),
                current_value: Some(entity.entity_name.clone()),
                expected_value: Some(rule.rule_expression.clone()),
                severity: rule.severity.clone(),
                status: "Open".to_string(),
                detected_at: Utc::now(),
                resolved_at: None,
                resolved_by: None,
                resolution_notes: None,
                created_at: Utc::now(),
            };
            self.repo.create_quality_violation(&violation).await?;
            Ok(Some(violation))
        } else {
            Ok(None)
        }
    }

    pub async fn resolve_violation(&self, violation_id: Uuid, resolved_by: Uuid, notes: Option<String>) -> anyhow::Result<DataQualityViolation> {
        let mut violations = self.repo.list_quality_violations(Some(violation_id), Some("Open")).await?;
        let mut violation = violations.pop().ok_or_else(|| anyhow::anyhow!("Violation not found"))?;
        violation.status = "Resolved".to_string();
        violation.resolved_at = Some(Utc::now());
        violation.resolved_by = Some(resolved_by);
        violation.resolution_notes = notes;
        self.repo.update_quality_violation(&violation).await?;
        Ok(violation)
    }

    pub async fn find_duplicates(&self, entity_type: &str) -> anyhow::Result<Vec<DuplicateRecord>> {
        let entities = self.repo.list_master_entities(Some(entity_type), 1000, 0).await?;
        let mut duplicates = Vec::new();
        
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let similarity = self.calculate_similarity(&entities[i], &entities[j]);
                if similarity >= 80 {
                    let dup = DuplicateRecord {
                        id: Uuid::new_v4(),
                        entity_type: entity_type.to_string(),
                        primary_entity_id: entities[i].id,
                        duplicate_entity_id: entities[j].id,
                        similarity_score: similarity,
                        matched_fields: serde_json::json!({"name": true, "code": true}),
                        status: "Pending".to_string(),
                        merge_initiated_by: None,
                        merge_initiated_at: None,
                        created_at: Utc::now(),
                    };
                    self.repo.create_duplicate_record(&dup).await?;
                    duplicates.push(dup);
                }
            }
        }
        
        Ok(duplicates)
    }

    fn calculate_similarity(&self, e1: &MasterDataEntity, e2: &MasterDataEntity) -> i32 {
        let name_sim = if e1.entity_name.to_lowercase() == e2.entity_name.to_lowercase() { 50 } else { 0 };
        let code_sim = if e1.entity_code.to_lowercase() == e2.entity_code.to_lowercase() { 30 } else { 0 };
        let source_sim = if e1.source_system == e2.source_system { 20 } else { 0 };
        name_sim + code_sim + source_sim
    }

    pub async fn merge_records(&self, req: MergeRecordsRequest, merged_by: Uuid) -> anyhow::Result<GoldenRecord> {
        let primary = self.repo.get_master_entity(req.primary_entity_id).await?
            .ok_or_else(|| anyhow::anyhow!("Primary entity not found"))?;
        
        let golden = GoldenRecord {
            id: Uuid::new_v4(),
            entity_type: primary.entity_type.clone(),
            golden_code: format!("GR-{}", Utc::now().format("%Y%m%d%H%M%S")),
            name: primary.entity_name.clone(),
            attributes: serde_json::json!({}),
            source_count: (req.duplicate_entity_ids.len() + 1) as i32,
            confidence_score: 100,
            steward_id: Some(merged_by),
            status: GovernanceStatus::Active,
            version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_golden_record(&golden).await?;
        
        for dup_id in &req.duplicate_entity_ids {
            if let Some(dup) = self.repo.get_master_entity(*dup_id).await? {
                let mut dup_record = dup;
                dup_record.golden_record_id = Some(golden.id);
                dup_record.status = GovernanceStatus::Archived;
                dup_record.updated_at = Utc::now();
                self.repo.update_master_entity(&dup_record).await?;
            }
        }
        
        let mut primary_record = primary;
        primary_record.golden_record_id = Some(golden.id);
        primary_record.updated_at = Utc::now();
        self.repo.update_master_entity(&primary_record).await?;
        
        Ok(golden)
    }

    pub async fn get_quality_dashboard(&self, entity_type: &str) -> anyhow::Result<DataQualityDashboard> {
        let entities = self.repo.list_master_entities(Some(entity_type), 10000, 0).await?;
        let _violations = self.repo.list_quality_violations(None, Some("Open")).await?;
        
        let total = entities.len() as i64;
        let with_issues = entities.iter().filter(|e| e.quality_score < 80).count() as i64;
        
        let avg_completeness = if entities.is_empty() { 0.0 } else {
            entities.iter().map(|e| e.completeness_score as f64).sum::<f64>() / entities.len() as f64
        };
        let avg_accuracy = if entities.is_empty() { 0.0 } else {
            entities.iter().map(|e| e.accuracy_score as f64).sum::<f64>() / entities.len() as f64
        };
        let avg_timeliness = if entities.is_empty() { 0.0 } else {
            entities.iter().map(|e| e.timeliness_score as f64).sum::<f64>() / entities.len() as f64
        };
        let avg_consistency = if entities.is_empty() { 0.0 } else {
            entities.iter().map(|e| e.consistency_score as f64).sum::<f64>() / entities.len() as f64
        };
        
        Ok(DataQualityDashboard {
            entity_type: entity_type.to_string(),
            total_records: total,
            records_with_issues: with_issues,
            avg_completeness,
            avg_accuracy,
            avg_timeliness,
            avg_consistency,
            overall_score: (avg_completeness + avg_accuracy + avg_timeliness + avg_consistency) / 4.0,
            issues_by_severity: serde_json::json!({"high": 0, "medium": 0, "low": 0}),
            issues_by_rule: serde_json::json!({}),
        })
    }

    pub async fn create_import_job(&self, job_name: String, entity_type: String, source_file: String) -> anyhow::Result<DataImportJob> {
        let now = Utc::now();
        let job = DataImportJob {
            id: Uuid::new_v4(),
            job_name,
            entity_type,
            source_file,
            total_records: 0,
            processed_records: 0,
            success_records: 0,
            failed_records: 0,
            duplicate_records: 0,
            status: "Pending".to_string(),
            started_at: None,
            completed_at: None,
            error_log: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_import_job(&job).await?;
        Ok(job)
    }

    pub async fn start_import_job(&self, job_id: Uuid) -> anyhow::Result<DataImportJob> {
        let mut job = self.repo.get_import_job(job_id).await?.ok_or_else(|| anyhow::anyhow!("Job not found"))?;
        job.status = "Running".to_string();
        job.started_at = Some(Utc::now());
        job.updated_at = Utc::now();
        self.repo.update_import_job(&job).await?;
        Ok(job)
    }

    pub async fn complete_import_job(&self, job_id: Uuid, success: i32, failed: i32, duplicates: i32) -> anyhow::Result<DataImportJob> {
        let mut job = self.repo.get_import_job(job_id).await?.ok_or_else(|| anyhow::anyhow!("Job not found"))?;
        job.status = "Completed".to_string();
        job.success_records = success;
        job.failed_records = failed;
        job.duplicate_records = duplicates;
        job.processed_records = success + failed + duplicates;
        job.completed_at = Some(Utc::now());
        job.updated_at = Utc::now();
        self.repo.update_import_job(&job).await?;
        Ok(job)
    }
}
