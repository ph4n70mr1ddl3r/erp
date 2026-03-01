use chrono::{DateTime, Utc};
use erp_core::error::{Error, Result};
use erp_core::models::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct DataQualityRuleService {
    rule_repo: SqliteQualityRuleRepository,
}

impl DataQualityRuleService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            rule_repo: SqliteQualityRuleRepository::new(pool),
        }
    }

    pub async fn create_rule(
        &self,
        _pool: &SqlitePool,
        name: String,
        code: String,
        rule_type: RuleType,
        severity: RuleSeverity,
        target_entity: String,
        target_field: String,
        condition: String,
        threshold: Option<f64>,
    ) -> Result<DataQualityRule> {
        if name.trim().is_empty() {
            return Err(Error::validation("Rule name is required".to_string()));
        }
        if condition.trim().is_empty() {
            return Err(Error::validation("Condition is required".to_string()));
        }
        let rule = DataQualityRule {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            rule_type,
            severity,
            target_entity,
            target_field,
            condition,
            threshold,
            is_active: true,
            schedule: None,
            last_run: None,
            last_result: None,
            tags: Vec::new(),
        };
        self.rule_repo.create(&rule).await
    }

    pub async fn get_rule(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<DataQualityRule>> {
        self.rule_repo.find_by_id(id).await
    }

    pub async fn get_rules_for_entity(&self, _pool: &SqlitePool, entity: &str) -> Result<Vec<DataQualityRule>> {
        self.rule_repo.find_by_entity(entity).await
    }

    pub async fn execute_rule(&self, _pool: &SqlitePool, rule: &DataQualityRule, data: &[serde_json::Value]) -> Result<QualityScore> {
        let mut passed = 0i64;
        let mut failed = 0i64;
        let mut errors = Vec::new();
        for record in data {
            let field_value = record.get(&rule.target_field);
            let passes = self.evaluate_condition(field_value, &rule.condition, &rule.rule_type)?;
            if passes {
                passed += 1;
            } else {
                failed += 1;
                errors.push(DataQualityError {
                    record_id: record.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    field: rule.target_field.clone(),
                    error_type: format!("{:?}", rule.rule_type),
                    message: format!("Rule '{}' failed", rule.name),
                    actual_value: field_value.map(|v| v.to_string()),
                    expected_value: None,
                    severity: rule.severity.clone(),
                });
            }
        }
        let total = passed + failed;
        let score = if total > 0 { (passed as f64 / total as f64) * 100.0 } else { 100.0 };
        Ok(QualityScore {
            score,
            grade: Self::score_to_grade(score),
            passed_records: passed,
            failed_records: failed,
            total_records: total,
            error_count: errors.len() as i64,
        })
    }

    fn evaluate_condition(&self, value: Option<&serde_json::Value>, condition: &str, rule_type: &RuleType) -> Result<bool> {
        match rule_type {
            RuleType::Completeness => Ok(value.is_some() && !value.map(|v| v.is_null()).unwrap_or(true)),
            RuleType::Format => {
                let val = value.and_then(|v| v.as_str()).unwrap_or("");
                Ok(regex::Regex::new(condition).map(|re| re.is_match(val)).unwrap_or(false))
            }
            RuleType::Range => {
                let num = value.and_then(|v| v.as_f64());
                let parts: Vec<&str> = condition.split(',').collect();
                if parts.len() == 2 {
                    let min: f64 = parts[0].parse().unwrap_or(f64::MIN);
                    let max: f64 = parts[1].parse().unwrap_or(f64::MAX);
                    Ok(num.map(|n| n >= min && n <= max).unwrap_or(false))
                } else {
                    Ok(false)
                }
            }
            RuleType::Pattern => {
                let val = value.and_then(|v| v.as_str()).unwrap_or("");
                Ok(regex::Regex::new(condition).map(|re| re.is_match(val)).unwrap_or(false))
            }
            _ => Ok(true),
        }
    }

    fn score_to_grade(score: f64) -> QualityGrade {
        if score >= 95.0 { QualityGrade::A }
        else if score >= 85.0 { QualityGrade::B }
        else if score >= 70.0 { QualityGrade::C }
        else if score >= 50.0 { QualityGrade::D }
        else { QualityGrade::F }
    }
}

pub struct DataProfilingService {
    pool: SqlitePool,
}

impl DataProfilingService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn profile_entity(&self, entity: &str, data: &[serde_json::Value]) -> Result<DataQualityProfile> {
        let mut field_profiles = Vec::new();
        if let Some(first_record) = data.first() {
            for (field_name, _) in first_record.as_object().unwrap_or(&serde_json::Map::new()) {
                let field_values: Vec<_> = data.iter()
                    .filter_map(|r| r.get(field_name).cloned())
                    .collect();
                let profile = self.profile_field(field_name, &field_values);
                field_profiles.push(profile);
            }
        }
        let overall_score = field_profiles.iter()
            .map(|p| 100.0 - p.null_percent)
            .sum::<f64>() / field_profiles.len().max(1) as f64;
        Ok(DataQualityProfile {
            base: BaseEntity::new(),
            name: format!("{} Profile", entity),
            entity: entity.to_string(),
            profile_date: Utc::now(),
            total_records: data.len() as i64,
            field_profiles,
            overall_quality_score: overall_score,
        })
    }

    fn profile_field(&self, field_name: &str, values: &[serde_json::Value]) -> FieldProfile {
        let total = values.len() as i64;
        let null_count = values.iter().filter(|v| v.is_null()).count() as i64;
        let null_percent = if total > 0 { (null_count as f64 / total as f64) * 100.0 } else { 0.0 };
        let non_null: Vec<_> = values.iter().filter(|v| !v.is_null()).collect();
        let unique_count = non_null.iter().collect::<std::collections::HashSet<_>>().len() as i64;
        let unique_percent = if total > 0 { (unique_count as f64 / total as f64) * 100.0 } else { 0.0 };
        let distinct_values = unique_count;
        let mut top_values = Vec::new();
        let mut value_counts = std::collections::HashMap::new();
        for v in &non_null {
            *value_counts.entry(v.to_string()).or_insert(0) += 1;
        }
        let mut sorted_counts: Vec<_> = value_counts.into_iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));
        for (value, count) in sorted_counts.into_iter().take(10) {
            top_values.push(ValueFrequency {
                value,
                count,
                percent: if total > 0 { (count as f64 / total as f64) * 100.0 } else { 0.0 },
            });
        }
        FieldProfile {
            field_name: field_name.to_string(),
            data_type: self.infer_data_type(&non_null),
            null_count,
            null_percent,
            unique_count,
            unique_percent,
            distinct_values,
            min_value: None,
            max_value: None,
            avg_value: None,
            std_dev: None,
            pattern_match_percent: None,
            top_values,
            outliers: Vec::new(),
        }
    }

    fn infer_data_type(&self, values: &[&serde_json::Value]) -> String {
        if values.is_empty() {
            return "unknown".to_string();
        }
        let first = values[0];
        if first.is_string() { "string".to_string() }
        else if first.is_number() { "number".to_string() }
        else if first.is_boolean() { "boolean".to_string() }
        else if first.is_array() { "array".to_string() }
        else if first.is_object() { "object".to_string() }
        else { "unknown".to_string() }
    }
}

pub struct DataCleansingService {
    pool: SqlitePool,
}

impl DataCleansingService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_cleansing_job(
        &self,
        name: String,
        source_entity: String,
        transformations: Vec<DataTransformation>,
        created_by: Uuid,
    ) -> Result<DataCleansingJob> {
        let job = DataCleansingJob {
            base: BaseEntity::new(),
            name,
            description: None,
            source_entity,
            target_entity: None,
            transformations,
            status: JobStatus::Pending,
            created_by,
            started_at: None,
            completed_at: None,
            records_processed: 0,
            records_modified: 0,
            records_failed: 0,
            error_log: None,
        };
        Ok(job)
    }

    pub async fn apply_transformation(&self, value: &serde_json::Value, transformation: &DataTransformation) -> Result<serde_json::Value> {
        let result = match transformation.transformation_type {
            TransformationType::Trim => {
                value.as_str()
                    .map(|s| serde_json::json!(s.trim()))
                    .unwrap_or(value.clone())
            }
            TransformationType::Uppercase => {
                value.as_str()
                    .map(|s| serde_json::json!(s.to_uppercase()))
                    .unwrap_or(value.clone())
            }
            TransformationType::Lowercase => {
                value.as_str()
                    .map(|s| serde_json::json!(s.to_lowercase()))
                    .unwrap_or(value.clone())
            }
            TransformationType::TitleCase => {
                value.as_str()
                    .map(|s| {
                        let title: String = s.split_whitespace()
                            .map(|word| {
                                let mut chars = word.chars();
                                match chars.next() {
                                    Some(c) => c.to_uppercase().chain(chars.map(|c| c.to_lowercase().next().unwrap())).collect(),
                                    None => String::new(),
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ");
                        serde_json::json!(title)
                    })
                    .unwrap_or(value.clone())
            }
            TransformationType::DefaultValue => {
                if value.is_null() {
                    transformation.parameters.get("default")
                        .cloned()
                        .unwrap_or(value.clone())
                } else {
                    value.clone()
                }
            }
            TransformationType::Replace => {
                let from = transformation.parameters.get("from").and_then(|v| v.as_str()).unwrap_or("");
                let to = transformation.parameters.get("to").and_then(|v| v.as_str()).unwrap_or("");
                value.as_str()
                    .map(|s| serde_json::json!(s.replace(from, to)))
                    .unwrap_or(value.clone())
            }
            _ => value.clone(),
        };
        Ok(result)
    }

    pub async fn cleanse_record(&self, record: &serde_json::Value, transformations: &[DataTransformation]) -> Result<serde_json::Value> {
        let mut result = record.clone();
        let obj = result.as_object_mut().ok_or_else(|| Error::validation("Record must be an object".to_string()))?;
        for transform in transformations {
            if let Some(value) = obj.get(&transform.field).cloned() {
                let transformed = self.apply_transformation(&value, transform).await?;
                obj.insert(transform.field.clone(), transformed);
            }
        }
        Ok(result)
    }
}

pub struct DataMatchingService {
    pool: SqlitePool,
}

impl DataMatchingService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_matching_rule(
        &self,
        name: String,
        entity: String,
        match_fields: Vec<MatchField>,
        blocking_keys: Vec<String>,
        match_threshold: f64,
    ) -> Result<DataMatchingRule> {
        if !(0.0..=1.0).contains(&match_threshold) {
            return Err(Error::validation("Threshold must be between 0 and 1".to_string()));
        }
        let rule = DataMatchingRule {
            base: BaseEntity::new(),
            name,
            entity,
            match_fields,
            blocking_keys,
            match_threshold,
            is_active: true,
        };
        Ok(rule)
    }

    pub fn calculate_similarity(&self, val1: &str, val2: &str, method: &ComparisonMethod) -> f64 {
        match method {
            ComparisonMethod::Exact => {
                if val1.to_lowercase() == val2.to_lowercase() { 1.0 } else { 0.0 }
            }
            ComparisonMethod::Levenshtein => {
                let distance = Self::levenshtein_distance(val1, val2);
                let max_len = val1.len().max(val2.len()).max(1);
                1.0 - (distance as f64 / max_len as f64)
            }
            ComparisonMethod::JaroWinkler => {
                Self::jaro_winkler(val1, val2)
            }
            _ => {
                if val1.to_lowercase() == val2.to_lowercase() { 1.0 } else { 0.0 }
            }
        }
    }

    fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let len1 = s1.len();
        let len2 = s2.len();
        if len1 == 0 { return len2; }
        if len2 == 0 { return len1; }
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        #[allow(clippy::needless_range_loop)]
        for i in 0..=len1 { matrix[i][0] = i; }
        #[allow(clippy::needless_range_loop)]
        for j in 0..=len2 { matrix[0][j] = j; }
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }
        matrix[len1][len2]
    }

    fn jaro_winkler(s1: &str, s2: &str) -> f64 {
        let s1: Vec<char> = s1.to_lowercase().chars().collect();
        let s2: Vec<char> = s2.to_lowercase().chars().collect();
        let len1 = s1.len();
        let len2 = s2.len();
        if len1 == 0 && len2 == 0 { return 1.0; }
        if len1 == 0 || len2 == 0 { return 0.0; }
        let match_distance = (len1.max(len2) / 2).saturating_sub(1);
        let mut s1_matches = vec![false; len1];
        let mut s2_matches = vec![false; len2];
        let mut matches = 0usize;
        let mut transpositions = 0usize;
        for i in 0..len1 {
            let start = i.saturating_sub(match_distance);
            let end = (i + match_distance + 1).min(len2);
            for j in start..end {
                if s2_matches[j] || s1[i] != s2[j] { continue; }
                s1_matches[i] = true;
                s2_matches[j] = true;
                matches += 1;
                break;
            }
        }
        if matches == 0 { return 0.0; }
        let mut k = 0usize;
        for i in 0..len1 {
            if !s1_matches[i] { continue; }
            while !s2_matches[k] { k += 1; }
            if s1[i] != s2[k] { transpositions += 1; }
            k += 1;
        }
        let jaro = (matches as f64 / len1 as f64 + matches as f64 / len2 as f64 + 
                   (matches - transpositions / 2) as f64 / matches as f64) / 3.0;
        let prefix = s1.iter().zip(s2.iter()).take(4).filter(|(a, b)| a == b).count() as f64;
        jaro + prefix * 0.1 * (1.0 - jaro)
    }

    pub async fn find_duplicates(&self, records: &[serde_json::Value], rule: &DataMatchingRule) -> Result<Vec<DuplicateGroup>> {
        let mut groups = Vec::new();
        let mut processed = std::collections::HashSet::new();
        for i in 0..records.len() {
            if processed.contains(&i) { continue; }
            let mut duplicates = Vec::new();
            for j in (i + 1)..records.len() {
                if processed.contains(&j) { continue; }
                let mut total_weight = 0.0;
                let mut total_score = 0.0;
                for match_field in &rule.match_fields {
                    let val1 = records[i].get(&match_field.field)
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let val2 = records[j].get(&match_field.field)
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let sim = self.calculate_similarity(val1, val2, &match_field.comparison_method);
                    total_score += sim * match_field.weight;
                    total_weight += match_field.weight;
                }
                let final_score = if total_weight > 0.0 { total_score / total_weight } else { 0.0 };
                if final_score >= rule.match_threshold {
                    duplicates.push(j);
                    processed.insert(j);
                }
            }
            if !duplicates.is_empty() {
                let canonical_id = records[i].get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let dup_ids: Vec<String> = duplicates.iter()
                    .filter_map(|&idx| records[idx].get("id").and_then(|v| v.as_str()).map(|s| s.to_string()))
                    .collect();
                groups.push(DuplicateGroup {
                    base: BaseEntity::new(),
                    entity: rule.entity.clone(),
                    canonical_id,
                    duplicate_ids: dup_ids,
                    match_score: 1.0,
                    detected_at: Utc::now(),
                    resolved_at: None,
                    resolved_by: None,
                    resolution_type: None,
                });
            }
        }
        Ok(groups)
    }
}

pub struct DataQualityDashboardService {
    pool: SqlitePool,
}

impl DataQualityDashboardService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn generate_dashboard(&self, entity_scores: Vec<EntityQualityScore>) -> Result<DataQualityDashboard> {
        let dashboard = DataQualityDashboard {
            base: BaseEntity::new(),
            name: "Data Quality Dashboard".to_string(),
            entity_scores,
            trend_data: Vec::new(),
            top_issues: Vec::new(),
            last_updated: Utc::now(),
        };
        Ok(dashboard)
    }

    pub async fn calculate_overall_score(scores: &[EntityQualityScore]) -> f64 {
        if scores.is_empty() {
            return 0.0;
        }
        scores.iter().map(|s| s.score).sum::<f64>() / scores.len() as f64
    }

    pub fn get_grade_color(grade: &QualityGrade) -> &str {
        match grade {
            QualityGrade::A => "green",
            QualityGrade::B => "blue",
            QualityGrade::C => "yellow",
            QualityGrade::D => "orange",
            QualityGrade::F => "red",
        }
    }
}

pub struct DataQualityReportService {
    pool: SqlitePool,
}

impl DataQualityReportService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn generate_report(
        &self,
        name: String,
        report_type: ReportType,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        generated_by: Uuid,
        entity_details: Vec<EntityReportDetail>,
    ) -> Result<DataQualityReport> {
        let overall_score = entity_details.iter().map(|e| e.score).sum::<f64>() / entity_details.len().max(1) as f64;
        let overall_grade = Self::score_to_grade(overall_score);
        let total_errors: i64 = entity_details.iter()
            .flat_map(|e| e.error_breakdown.values())
            .sum();
        let summary = QualityReportSummary {
            overall_score,
            overall_grade,
            total_rules_executed: entity_details.len() as i64,
            total_errors_found: total_errors,
            improvement_from_previous: 0.0,
            top_issues: Vec::new(),
        };
        let report = DataQualityReport {
            base: BaseEntity::new(),
            name,
            report_type,
            period_start,
            period_end,
            generated_at: Utc::now(),
            generated_by,
            summary,
            entity_details,
        };
        Ok(report)
    }

    fn score_to_grade(score: f64) -> QualityGrade {
        if score >= 95.0 { QualityGrade::A }
        else if score >= 85.0 { QualityGrade::B }
        else if score >= 70.0 { QualityGrade::C }
        else if score >= 50.0 { QualityGrade::D }
        else { QualityGrade::F }
    }
}
