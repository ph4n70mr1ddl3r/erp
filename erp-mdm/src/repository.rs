use crate::models::*;
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

#[async_trait]
pub trait MDMRepository {
    async fn create_master_entity(&self, entity: &MasterDataEntity) -> anyhow::Result<()>;
    async fn get_master_entity(&self, id: Uuid) -> anyhow::Result<Option<MasterDataEntity>>;
    async fn list_master_entities(&self, entity_type: Option<&str>, limit: i32, offset: i32) -> anyhow::Result<Vec<MasterDataEntity>>;
    async fn update_master_entity(&self, entity: &MasterDataEntity) -> anyhow::Result<()>;
    
    async fn create_golden_record(&self, record: &GoldenRecord) -> anyhow::Result<()>;
    async fn get_golden_record(&self, id: Uuid) -> anyhow::Result<Option<GoldenRecord>>;
    async fn list_golden_records(&self, entity_type: &str) -> anyhow::Result<Vec<GoldenRecord>>;
    async fn update_golden_record(&self, record: &GoldenRecord) -> anyhow::Result<()>;
    
    async fn create_golden_record_source(&self, source: &GoldenRecordSource) -> anyhow::Result<()>;
    async fn list_golden_record_sources(&self, golden_record_id: Uuid) -> anyhow::Result<Vec<GoldenRecordSource>>;
    
    async fn create_quality_rule(&self, rule: &DataQualityRule) -> anyhow::Result<()>;
    async fn get_quality_rule(&self, id: Uuid) -> anyhow::Result<Option<DataQualityRule>>;
    async fn list_quality_rules(&self, entity_type: Option<&str>) -> anyhow::Result<Vec<DataQualityRule>>;
    
    async fn create_quality_violation(&self, violation: &DataQualityViolation) -> anyhow::Result<()>;
    async fn list_quality_violations(&self, entity_id: Option<Uuid>, status: Option<&str>) -> anyhow::Result<Vec<DataQualityViolation>>;
    async fn update_quality_violation(&self, violation: &DataQualityViolation) -> anyhow::Result<()>;
    
    async fn create_match_rule(&self, rule: &MatchRule) -> anyhow::Result<()>;
    async fn list_match_rules(&self, entity_type: &str) -> anyhow::Result<Vec<MatchRule>>;
    
    async fn create_match_result(&self, result: &MatchResult) -> anyhow::Result<()>;
    async fn list_match_results(&self, rule_id: Uuid) -> anyhow::Result<Vec<MatchResult>>;
    async fn update_match_result(&self, result: &MatchResult) -> anyhow::Result<()>;
    
    async fn create_data_domain(&self, domain: &DataDomain) -> anyhow::Result<()>;
    async fn list_data_domains(&self) -> anyhow::Result<Vec<DataDomain>>;
    
    async fn create_data_attribute(&self, attr: &DataAttribute) -> anyhow::Result<()>;
    async fn list_data_attributes(&self, domain_id: Uuid) -> anyhow::Result<Vec<DataAttribute>>;
    
    async fn create_data_steward(&self, steward: &DataSteward) -> anyhow::Result<()>;
    async fn list_data_stewards(&self) -> anyhow::Result<Vec<DataSteward>>;
    
    async fn create_duplicate_record(&self, dup: &DuplicateRecord) -> anyhow::Result<()>;
    async fn list_duplicate_records(&self, entity_type: Option<&str>, status: Option<&str>) -> anyhow::Result<Vec<DuplicateRecord>>;
    async fn update_duplicate_record(&self, dup: &DuplicateRecord) -> anyhow::Result<()>;
    
    async fn create_reference_data(&self, data: &ReferenceData) -> anyhow::Result<()>;
    async fn list_reference_data(&self, category: &str) -> anyhow::Result<Vec<ReferenceData>>;
    async fn update_reference_data(&self, data: &ReferenceData) -> anyhow::Result<()>;
    
    async fn create_import_job(&self, job: &DataImportJob) -> anyhow::Result<()>;
    async fn get_import_job(&self, id: Uuid) -> anyhow::Result<Option<DataImportJob>>;
    async fn update_import_job(&self, job: &DataImportJob) -> anyhow::Result<()>;
    
    async fn create_export_job(&self, job: &DataExportJob) -> anyhow::Result<()>;
    async fn get_export_job(&self, id: Uuid) -> anyhow::Result<Option<DataExportJob>>;
    async fn update_export_job(&self, job: &DataExportJob) -> anyhow::Result<()>;
}

pub struct SqliteMDMRepository {
    pool: SqlitePool,
}

impl SqliteMDMRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MDMRepository for SqliteMDMRepository {
    async fn create_master_entity(&self, entity: &MasterDataEntity) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_entities (id, entity_type, entity_code, entity_name, source_system,
                source_id, golden_record_id, quality_score, completeness_score, accuracy_score,
                timeliness_score, consistency_score, last_verified, next_verification, status,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(entity.id.to_string())
        .bind(&entity.entity_type)
        .bind(&entity.entity_code)
        .bind(&entity.entity_name)
        .bind(&entity.source_system)
        .bind(&entity.source_id)
        .bind(entity.golden_record_id.map(|u| u.to_string()))
        .bind(entity.quality_score)
        .bind(entity.completeness_score)
        .bind(entity.accuracy_score)
        .bind(entity.timeliness_score)
        .bind(entity.consistency_score)
        .bind(entity.last_verified)
        .bind(entity.next_verification)
        .bind(format!("{:?}", entity.status))
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_master_entity(&self, id: Uuid) -> anyhow::Result<Option<MasterDataEntity>> {
        let row = sqlx::query(
            r#"SELECT id, entity_type, entity_code, entity_name, source_system, source_id,
                golden_record_id, quality_score, completeness_score, accuracy_score,
                timeliness_score, consistency_score, last_verified, next_verification,
                status, created_at, updated_at FROM mdm_entities WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;

        match row {
            Some(row) => {
                let entity = MasterDataEntity {
                    id: Uuid::parse_str(row.get::<&str, _>("id"))?,
                    entity_type: row.get("entity_type"),
                    entity_code: row.get("entity_code"),
                    entity_name: row.get("entity_name"),
                    source_system: row.get("source_system"),
                    source_id: row.get("source_id"),
                    golden_record_id: row.get::<Option<&str>, _>("golden_record_id").and_then(|s| Uuid::parse_str(s).ok()),
                    quality_score: row.get("quality_score"),
                    completeness_score: row.get("completeness_score"),
                    accuracy_score: row.get("accuracy_score"),
                    timeliness_score: row.get("timeliness_score"),
                    consistency_score: row.get("consistency_score"),
                    last_verified: row.get("last_verified"),
                    next_verification: row.get("next_verification"),
                    status: match row.get::<&str, _>("status") {
                        "Active" => GovernanceStatus::Active,
                        "InReview" => GovernanceStatus::InReview,
                        "Deprecated" => GovernanceStatus::Deprecated,
                        _ => GovernanceStatus::Archived,
                    },
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(entity))
            }
            None => Ok(None),
        }
    }

    async fn list_master_entities(&self, entity_type: Option<&str>, limit: i32, offset: i32) -> anyhow::Result<Vec<MasterDataEntity>> {
        let rows = if let Some(et) = entity_type {
            sqlx::query(
                r#"SELECT id, entity_type, entity_code, entity_name, source_system, source_id,
                    golden_record_id, quality_score, completeness_score, accuracy_score,
                    timeliness_score, consistency_score, last_verified, next_verification,
                    status, created_at, updated_at FROM mdm_entities
                    WHERE entity_type = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"#
            )
            .bind(et)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query(
                r#"SELECT id, entity_type, entity_code, entity_name, source_system, source_id,
                    golden_record_id, quality_score, completeness_score, accuracy_score,
                    timeliness_score, consistency_score, last_verified, next_verification,
                    status, created_at, updated_at FROM mdm_entities
                    ORDER BY created_at DESC LIMIT ? OFFSET ?"#
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool).await?
        };

        let entities = rows.into_iter().map(|row| {
            MasterDataEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                entity_type: row.get("entity_type"),
                entity_code: row.get("entity_code"),
                entity_name: row.get("entity_name"),
                source_system: row.get("source_system"),
                source_id: row.get("source_id"),
                golden_record_id: row.get::<Option<&str>, _>("golden_record_id").and_then(|s| Uuid::parse_str(s).ok()),
                quality_score: row.get("quality_score"),
                completeness_score: row.get("completeness_score"),
                accuracy_score: row.get("accuracy_score"),
                timeliness_score: row.get("timeliness_score"),
                consistency_score: row.get("consistency_score"),
                last_verified: row.get("last_verified"),
                next_verification: row.get("next_verification"),
                status: match row.get::<&str, _>("status") {
                    "Active" => GovernanceStatus::Active,
                    "InReview" => GovernanceStatus::InReview,
                    "Deprecated" => GovernanceStatus::Deprecated,
                    _ => GovernanceStatus::Archived,
                },
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        Ok(entities)
    }

    async fn update_master_entity(&self, entity: &MasterDataEntity) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE mdm_entities SET entity_name = ?, golden_record_id = ?, quality_score = ?,
                completeness_score = ?, accuracy_score = ?, timeliness_score = ?, consistency_score = ?,
                last_verified = ?, next_verification = ?, status = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&entity.entity_name)
        .bind(entity.golden_record_id.map(|u| u.to_string()))
        .bind(entity.quality_score)
        .bind(entity.completeness_score)
        .bind(entity.accuracy_score)
        .bind(entity.timeliness_score)
        .bind(entity.consistency_score)
        .bind(entity.last_verified)
        .bind(entity.next_verification)
        .bind(format!("{:?}", entity.status))
        .bind(entity.updated_at)
        .bind(entity.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_golden_record(&self, record: &GoldenRecord) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_golden_records (id, entity_type, golden_code, name, attributes,
                source_count, confidence_score, steward_id, status, version, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(record.id.to_string())
        .bind(&record.entity_type)
        .bind(&record.golden_code)
        .bind(&record.name)
        .bind(record.attributes.clone())
        .bind(record.source_count)
        .bind(record.confidence_score)
        .bind(record.steward_id.map(|u| u.to_string()))
        .bind(format!("{:?}", record.status))
        .bind(record.version)
        .bind(record.created_at)
        .bind(record.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_golden_record(&self, id: Uuid) -> anyhow::Result<Option<GoldenRecord>> {
        let row = sqlx::query(
            r#"SELECT id, entity_type, golden_code, name, attributes, source_count, confidence_score,
                steward_id, status, version, created_at, updated_at
                FROM mdm_golden_records WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;

        match row {
            Some(row) => {
                let record = GoldenRecord {
                    id: Uuid::parse_str(row.get::<&str, _>("id"))?,
                    entity_type: row.get("entity_type"),
                    golden_code: row.get("golden_code"),
                    name: row.get("name"),
                    attributes: row.get("attributes"),
                    source_count: row.get("source_count"),
                    confidence_score: row.get("confidence_score"),
                    steward_id: row.get::<Option<&str>, _>("steward_id").and_then(|s| Uuid::parse_str(s).ok()),
                    status: match row.get::<&str, _>("status") {
                        "Active" => GovernanceStatus::Active,
                        "InReview" => GovernanceStatus::InReview,
                        "Deprecated" => GovernanceStatus::Deprecated,
                        _ => GovernanceStatus::Archived,
                    },
                    version: row.get("version"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    async fn list_golden_records(&self, entity_type: &str) -> anyhow::Result<Vec<GoldenRecord>> {
        let rows = sqlx::query(
            r#"SELECT id, entity_type, golden_code, name, attributes, source_count, confidence_score,
                steward_id, status, version, created_at, updated_at
                FROM mdm_golden_records WHERE entity_type = ? ORDER BY created_at DESC"#
        )
        .bind(entity_type)
        .fetch_all(&self.pool).await?;

        let records = rows.into_iter().map(|row| {
            GoldenRecord {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                entity_type: row.get("entity_type"),
                golden_code: row.get("golden_code"),
                name: row.get("name"),
                attributes: row.get("attributes"),
                source_count: row.get("source_count"),
                confidence_score: row.get("confidence_score"),
                steward_id: row.get::<Option<&str>, _>("steward_id").and_then(|s| Uuid::parse_str(s).ok()),
                status: match row.get::<&str, _>("status") {
                    "Active" => GovernanceStatus::Active,
                    "InReview" => GovernanceStatus::InReview,
                    "Deprecated" => GovernanceStatus::Deprecated,
                    _ => GovernanceStatus::Archived,
                },
                version: row.get("version"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        Ok(records)
    }

    async fn update_golden_record(&self, record: &GoldenRecord) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE mdm_golden_records SET name = ?, attributes = ?, source_count = ?,
                confidence_score = ?, status = ?, version = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&record.name)
        .bind(record.attributes.clone())
        .bind(record.source_count)
        .bind(record.confidence_score)
        .bind(format!("{:?}", record.status))
        .bind(record.version)
        .bind(record.updated_at)
        .bind(record.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_golden_record_source(&self, source: &GoldenRecordSource) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_golden_record_sources (id, golden_record_id, source_entity_id,
                match_score, is_primary, contributed_at, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(source.id.to_string())
        .bind(source.golden_record_id.to_string())
        .bind(source.source_entity_id.to_string())
        .bind(source.match_score)
        .bind(source.is_primary)
        .bind(source.contributed_at)
        .bind(source.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_golden_record_sources(&self, golden_record_id: Uuid) -> anyhow::Result<Vec<GoldenRecordSource>> {
        let rows = sqlx::query(
            r#"SELECT id, golden_record_id, source_entity_id, match_score, is_primary,
                contributed_at, created_at FROM mdm_golden_record_sources
                WHERE golden_record_id = ?"#
        )
        .bind(golden_record_id.to_string())
        .fetch_all(&self.pool).await?;

        let sources = rows.into_iter().map(|row| {
            GoldenRecordSource {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                golden_record_id: Uuid::parse_str(row.get::<&str, _>("golden_record_id")).unwrap(),
                source_entity_id: Uuid::parse_str(row.get::<&str, _>("source_entity_id")).unwrap(),
                match_score: row.get("match_score"),
                is_primary: row.get("is_primary"),
                contributed_at: row.get("contributed_at"),
                created_at: row.get("created_at"),
            }
        }).collect();
        Ok(sources)
    }

    async fn create_quality_rule(&self, rule: &DataQualityRule) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_quality_rules (id, rule_code, name, description, entity_type,
                field_name, rule_type, rule_expression, severity, is_active, auto_fix, fix_expression,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(rule.id.to_string())
        .bind(&rule.rule_code)
        .bind(&rule.name)
        .bind(&rule.description)
        .bind(&rule.entity_type)
        .bind(&rule.field_name)
        .bind(&rule.rule_type)
        .bind(&rule.rule_expression)
        .bind(&rule.severity)
        .bind(rule.is_active)
        .bind(rule.auto_fix)
        .bind(&rule.fix_expression)
        .bind(rule.created_at)
        .bind(rule.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_quality_rule(&self, id: Uuid) -> anyhow::Result<Option<DataQualityRule>> {
        let row = sqlx::query(
            r#"SELECT id, rule_code, name, description, entity_type, field_name, rule_type,
                rule_expression, severity, is_active, auto_fix, fix_expression, created_at, updated_at
                FROM mdm_quality_rules WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;

        match row {
            Some(row) => {
                let rule = DataQualityRule {
                    id: Uuid::parse_str(row.get::<&str, _>("id"))?,
                    rule_code: row.get("rule_code"),
                    name: row.get("name"),
                    description: row.get("description"),
                    entity_type: row.get("entity_type"),
                    field_name: row.get("field_name"),
                    rule_type: row.get("rule_type"),
                    rule_expression: row.get("rule_expression"),
                    severity: row.get("severity"),
                    is_active: row.get("is_active"),
                    auto_fix: row.get("auto_fix"),
                    fix_expression: row.get("fix_expression"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(rule))
            }
            None => Ok(None),
        }
    }

    async fn list_quality_rules(&self, entity_type: Option<&str>) -> anyhow::Result<Vec<DataQualityRule>> {
        let rows = if let Some(et) = entity_type {
            sqlx::query(
                r#"SELECT id, rule_code, name, description, entity_type, field_name, rule_type,
                    rule_expression, severity, is_active, auto_fix, fix_expression, created_at, updated_at
                    FROM mdm_quality_rules WHERE entity_type = ? AND is_active = 1"#
            )
            .bind(et)
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query(
                r#"SELECT id, rule_code, name, description, entity_type, field_name, rule_type,
                    rule_expression, severity, is_active, auto_fix, fix_expression, created_at, updated_at
                    FROM mdm_quality_rules WHERE is_active = 1"#
            )
            .fetch_all(&self.pool).await?
        };

        let rules = rows.into_iter().map(|row| {
            DataQualityRule {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                rule_code: row.get("rule_code"),
                name: row.get("name"),
                description: row.get("description"),
                entity_type: row.get("entity_type"),
                field_name: row.get("field_name"),
                rule_type: row.get("rule_type"),
                rule_expression: row.get("rule_expression"),
                severity: row.get("severity"),
                is_active: row.get("is_active"),
                auto_fix: row.get("auto_fix"),
                fix_expression: row.get("fix_expression"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        Ok(rules)
    }

    async fn create_quality_violation(&self, violation: &DataQualityViolation) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_quality_violations (id, rule_id, entity_id, entity_type, field_name,
                current_value, expected_value, severity, status, detected_at, resolved_at,
                resolved_by, resolution_notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(violation.id.to_string())
        .bind(violation.rule_id.to_string())
        .bind(violation.entity_id.to_string())
        .bind(&violation.entity_type)
        .bind(&violation.field_name)
        .bind(&violation.current_value)
        .bind(&violation.expected_value)
        .bind(&violation.severity)
        .bind(&violation.status)
        .bind(violation.detected_at)
        .bind(violation.resolved_at)
        .bind(violation.resolved_by.map(|u| u.to_string()))
        .bind(&violation.resolution_notes)
        .bind(violation.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_quality_violations(&self, _entity_id: Option<Uuid>, _status: Option<&str>) -> anyhow::Result<Vec<DataQualityViolation>> {
        let rows = sqlx::query(
            r#"SELECT id, rule_id, entity_id, entity_type, field_name, current_value, expected_value,
                severity, status, detected_at, resolved_at, resolved_by, resolution_notes, created_at
                FROM mdm_quality_violations ORDER BY detected_at DESC"#
        )
        .fetch_all(&self.pool).await?;

        let violations = rows.into_iter().map(|row| {
            DataQualityViolation {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                rule_id: Uuid::parse_str(row.get::<&str, _>("rule_id")).unwrap(),
                entity_id: Uuid::parse_str(row.get::<&str, _>("entity_id")).unwrap(),
                entity_type: row.get("entity_type"),
                field_name: row.get("field_name"),
                current_value: row.get("current_value"),
                expected_value: row.get("expected_value"),
                severity: row.get("severity"),
                status: row.get("status"),
                detected_at: row.get("detected_at"),
                resolved_at: row.get("resolved_at"),
                resolved_by: row.get::<Option<&str>, _>("resolved_by").and_then(|s| Uuid::parse_str(s).ok()),
                resolution_notes: row.get("resolution_notes"),
                created_at: row.get("created_at"),
            }
        }).collect();
        Ok(violations)
    }

    async fn update_quality_violation(&self, violation: &DataQualityViolation) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE mdm_quality_violations SET status = ?, resolved_at = ?, resolved_by = ?,
                resolution_notes = ? WHERE id = ?"#
        )
        .bind(&violation.status)
        .bind(violation.resolved_at)
        .bind(violation.resolved_by.map(|u| u.to_string()))
        .bind(&violation.resolution_notes)
        .bind(violation.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_match_rule(&self, rule: &MatchRule) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_match_rules (id, rule_code, name, description, entity_type,
                match_type, blocking_rules, matching_rules, threshold_score, is_active,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(rule.id.to_string())
        .bind(&rule.rule_code)
        .bind(&rule.name)
        .bind(&rule.description)
        .bind(&rule.entity_type)
        .bind(&rule.match_type)
        .bind(rule.blocking_rules.clone())
        .bind(rule.matching_rules.clone())
        .bind(rule.threshold_score)
        .bind(rule.is_active)
        .bind(rule.created_at)
        .bind(rule.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_match_rules(&self, entity_type: &str) -> anyhow::Result<Vec<MatchRule>> {
        let rows = sqlx::query(
            r#"SELECT id, rule_code, name, description, entity_type, match_type, blocking_rules,
                matching_rules, threshold_score, is_active, created_at, updated_at
                FROM mdm_match_rules WHERE entity_type = ? AND is_active = 1"#
        )
        .bind(entity_type)
        .fetch_all(&self.pool).await?;

        let rules = rows.into_iter().map(|row| {
            MatchRule {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                rule_code: row.get("rule_code"),
                name: row.get("name"),
                description: row.get("description"),
                entity_type: row.get("entity_type"),
                match_type: row.get("match_type"),
                blocking_rules: row.get("blocking_rules"),
                matching_rules: row.get("matching_rules"),
                threshold_score: row.get("threshold_score"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        Ok(rules)
    }

    async fn create_match_result(&self, result: &MatchResult) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_match_results (id, rule_id, entity1_id, entity2_id, match_score,
                status, matched_at, reviewed_by, reviewed_at, decision_notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(result.id.to_string())
        .bind(result.rule_id.to_string())
        .bind(result.entity1_id.to_string())
        .bind(result.entity2_id.to_string())
        .bind(result.match_score)
        .bind(format!("{:?}", result.status))
        .bind(result.matched_at)
        .bind(result.reviewed_by.map(|u| u.to_string()))
        .bind(result.reviewed_at)
        .bind(&result.decision_notes)
        .bind(result.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_match_results(&self, rule_id: Uuid) -> anyhow::Result<Vec<MatchResult>> {
        let rows = sqlx::query(
            r#"SELECT id, rule_id, entity1_id, entity2_id, match_score, status,
                matched_at, reviewed_by, reviewed_at, decision_notes, created_at
                FROM mdm_match_results WHERE rule_id = ?"#
        )
        .bind(rule_id.to_string())
        .fetch_all(&self.pool).await?;

        let results = rows.into_iter().map(|row| {
            MatchResult {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                rule_id: Uuid::parse_str(row.get::<&str, _>("rule_id")).unwrap(),
                entity1_id: Uuid::parse_str(row.get::<&str, _>("entity1_id")).unwrap(),
                entity2_id: Uuid::parse_str(row.get::<&str, _>("entity2_id")).unwrap(),
                match_score: row.get("match_score"),
                status: match row.get::<&str, _>("status") {
                    "Pending" => MatchStatus::Pending,
                    "Matched" => MatchStatus::Matched,
                    "Unmatched" => MatchStatus::Unmatched,
                    "Confirmed" => MatchStatus::Confirmed,
                    _ => MatchStatus::Rejected,
                },
                matched_at: row.get("matched_at"),
                reviewed_by: row.get::<Option<&str>, _>("reviewed_by").and_then(|s| Uuid::parse_str(s).ok()),
                reviewed_at: row.get("reviewed_at"),
                decision_notes: row.get("decision_notes"),
                created_at: row.get("created_at"),
            }
        }).collect();
        Ok(results)
    }

    async fn update_match_result(&self, result: &MatchResult) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE mdm_match_results SET status = ?, reviewed_by = ?, reviewed_at = ?,
                decision_notes = ? WHERE id = ?"#
        )
        .bind(format!("{:?}", result.status))
        .bind(result.reviewed_by.map(|u| u.to_string()))
        .bind(result.reviewed_at)
        .bind(&result.decision_notes)
        .bind(result.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_data_domain(&self, domain: &DataDomain) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_data_domains (id, domain_code, name, description, parent_domain_id,
                owner_id, steward_id, data_classification, retention_policy, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(domain.id.to_string())
        .bind(&domain.domain_code)
        .bind(&domain.name)
        .bind(&domain.description)
        .bind(domain.parent_domain_id.map(|u| u.to_string()))
        .bind(domain.owner_id.map(|u| u.to_string()))
        .bind(domain.steward_id.map(|u| u.to_string()))
        .bind(&domain.data_classification)
        .bind(&domain.retention_policy)
        .bind(domain.created_at)
        .bind(domain.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_data_domains(&self) -> anyhow::Result<Vec<DataDomain>> {
        let rows = sqlx::query(
            r#"SELECT id, domain_code, name, description, parent_domain_id, owner_id, steward_id,
                data_classification, retention_policy, created_at, updated_at
                FROM mdm_data_domains ORDER BY domain_code"#
        )
        .fetch_all(&self.pool).await?;

        let domains = rows.into_iter().map(|row| {
            DataDomain {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                domain_code: row.get("domain_code"),
                name: row.get("name"),
                description: row.get("description"),
                parent_domain_id: row.get::<Option<&str>, _>("parent_domain_id").and_then(|s| Uuid::parse_str(s).ok()),
                owner_id: row.get::<Option<&str>, _>("owner_id").and_then(|s| Uuid::parse_str(s).ok()),
                steward_id: row.get::<Option<&str>, _>("steward_id").and_then(|s| Uuid::parse_str(s).ok()),
                data_classification: row.get("data_classification"),
                retention_policy: row.get("retention_policy"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        Ok(domains)
    }

    async fn create_data_attribute(&self, attr: &DataAttribute) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_data_attributes (id, domain_id, attribute_code, name, description,
                data_type, max_length, is_required, is_unique, default_value, validation_regex,
                allowed_values, business_rules, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(attr.id.to_string())
        .bind(attr.domain_id.to_string())
        .bind(&attr.attribute_code)
        .bind(&attr.name)
        .bind(&attr.description)
        .bind(&attr.data_type)
        .bind(attr.max_length)
        .bind(attr.is_required)
        .bind(attr.is_unique)
        .bind(&attr.default_value)
        .bind(&attr.validation_regex)
        .bind(attr.allowed_values.clone())
        .bind(attr.business_rules.clone())
        .bind(attr.created_at)
        .bind(attr.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_data_attributes(&self, domain_id: Uuid) -> anyhow::Result<Vec<DataAttribute>> {
        let rows = sqlx::query(
            r#"SELECT id, domain_id, attribute_code, name, description, data_type, max_length,
                is_required, is_unique, default_value, validation_regex, allowed_values,
                business_rules, created_at, updated_at FROM mdm_data_attributes WHERE domain_id = ?
                ORDER BY attribute_code"#
        )
        .bind(domain_id.to_string())
        .fetch_all(&self.pool).await?;

        let attrs = rows.into_iter().map(|row| {
            DataAttribute {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                domain_id: Uuid::parse_str(row.get::<&str, _>("domain_id")).unwrap(),
                attribute_code: row.get("attribute_code"),
                name: row.get("name"),
                description: row.get("description"),
                data_type: row.get("data_type"),
                max_length: row.get("max_length"),
                is_required: row.get("is_required"),
                is_unique: row.get("is_unique"),
                default_value: row.get("default_value"),
                validation_regex: row.get("validation_regex"),
                allowed_values: row.get("allowed_values"),
                business_rules: row.get("business_rules"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        Ok(attrs)
    }

    async fn create_data_steward(&self, steward: &DataSteward) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_data_stewards (id, user_id, domain_id, entity_types, responsibilities,
                is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(steward.id.to_string())
        .bind(steward.user_id.to_string())
        .bind(steward.domain_id.map(|u| u.to_string()))
        .bind(steward.entity_types.clone())
        .bind(steward.responsibilities.clone())
        .bind(steward.is_active)
        .bind(steward.created_at)
        .bind(steward.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_data_stewards(&self) -> anyhow::Result<Vec<DataSteward>> {
        let rows = sqlx::query(
            r#"SELECT id, user_id, domain_id, entity_types, responsibilities, is_active,
                created_at, updated_at FROM mdm_data_stewards WHERE is_active = 1"#
        )
        .fetch_all(&self.pool).await?;

        let stewards = rows.into_iter().map(|row| {
            DataSteward {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                user_id: Uuid::parse_str(row.get::<&str, _>("user_id")).unwrap(),
                domain_id: row.get::<Option<&str>, _>("domain_id").and_then(|s| Uuid::parse_str(s).ok()),
                entity_types: row.get("entity_types"),
                responsibilities: row.get("responsibilities"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        Ok(stewards)
    }

    async fn create_duplicate_record(&self, dup: &DuplicateRecord) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_duplicate_records (id, entity_type, primary_entity_id,
                duplicate_entity_id, similarity_score, matched_fields, status, merge_initiated_by,
                merge_initiated_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(dup.id.to_string())
        .bind(&dup.entity_type)
        .bind(dup.primary_entity_id.to_string())
        .bind(dup.duplicate_entity_id.to_string())
        .bind(dup.similarity_score)
        .bind(dup.matched_fields.clone())
        .bind(&dup.status)
        .bind(dup.merge_initiated_by.map(|u| u.to_string()))
        .bind(dup.merge_initiated_at)
        .bind(dup.created_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_duplicate_records(&self, _entity_type: Option<&str>, _status: Option<&str>) -> anyhow::Result<Vec<DuplicateRecord>> {
        let rows = sqlx::query(
            r#"SELECT id, entity_type, primary_entity_id, duplicate_entity_id, similarity_score,
                matched_fields, status, merge_initiated_by, merge_initiated_at, created_at
                FROM mdm_duplicate_records ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool).await?;

        let records = rows.into_iter().map(|row| {
            DuplicateRecord {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                entity_type: row.get("entity_type"),
                primary_entity_id: Uuid::parse_str(row.get::<&str, _>("primary_entity_id")).unwrap(),
                duplicate_entity_id: Uuid::parse_str(row.get::<&str, _>("duplicate_entity_id")).unwrap(),
                similarity_score: row.get("similarity_score"),
                matched_fields: row.get("matched_fields"),
                status: row.get("status"),
                merge_initiated_by: row.get::<Option<&str>, _>("merge_initiated_by").and_then(|s| Uuid::parse_str(s).ok()),
                merge_initiated_at: row.get("merge_initiated_at"),
                created_at: row.get("created_at"),
            }
        }).collect();
        Ok(records)
    }

    async fn update_duplicate_record(&self, dup: &DuplicateRecord) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE mdm_duplicate_records SET status = ?, merge_initiated_by = ?,
                merge_initiated_at = ? WHERE id = ?"#
        )
        .bind(&dup.status)
        .bind(dup.merge_initiated_by.map(|u| u.to_string()))
        .bind(dup.merge_initiated_at)
        .bind(dup.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_reference_data(&self, data: &ReferenceData) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_reference_data (id, category, code, name, description, parent_code,
                sort_order, is_active, effective_date, expiry_date, attributes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(data.id.to_string())
        .bind(&data.category)
        .bind(&data.code)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.parent_code)
        .bind(data.sort_order)
        .bind(data.is_active)
        .bind(data.effective_date)
        .bind(data.expiry_date)
        .bind(data.attributes.clone())
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_reference_data(&self, category: &str) -> anyhow::Result<Vec<ReferenceData>> {
        let rows = sqlx::query(
            r#"SELECT id, category, code, name, description, parent_code, sort_order, is_active,
                effective_date, expiry_date, attributes, created_at, updated_at
                FROM mdm_reference_data WHERE category = ? AND is_active = 1 ORDER BY sort_order"#
        )
        .bind(category)
        .fetch_all(&self.pool).await?;

        let data = rows.into_iter().map(|row| {
            ReferenceData {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                category: row.get("category"),
                code: row.get("code"),
                name: row.get("name"),
                description: row.get("description"),
                parent_code: row.get("parent_code"),
                sort_order: row.get("sort_order"),
                is_active: row.get("is_active"),
                effective_date: row.get("effective_date"),
                expiry_date: row.get("expiry_date"),
                attributes: row.get("attributes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();
        Ok(data)
    }

    async fn update_reference_data(&self, data: &ReferenceData) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE mdm_reference_data SET name = ?, description = ?, sort_order = ?, is_active = ?,
                effective_date = ?, expiry_date = ?, attributes = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.sort_order)
        .bind(data.is_active)
        .bind(data.effective_date)
        .bind(data.expiry_date)
        .bind(data.attributes.clone())
        .bind(data.updated_at)
        .bind(data.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_import_job(&self, job: &DataImportJob) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_import_jobs (id, job_name, entity_type, source_file, total_records,
                processed_records, success_records, failed_records, duplicate_records, status,
                started_at, completed_at, error_log, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(job.id.to_string())
        .bind(&job.job_name)
        .bind(&job.entity_type)
        .bind(&job.source_file)
        .bind(job.total_records)
        .bind(job.processed_records)
        .bind(job.success_records)
        .bind(job.failed_records)
        .bind(job.duplicate_records)
        .bind(&job.status)
        .bind(job.started_at)
        .bind(job.completed_at)
        .bind(&job.error_log)
        .bind(job.created_at)
        .bind(job.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_import_job(&self, id: Uuid) -> anyhow::Result<Option<DataImportJob>> {
        let row = sqlx::query(
            r#"SELECT id, job_name, entity_type, source_file, total_records, processed_records,
                success_records, failed_records, duplicate_records, status, started_at, completed_at,
                error_log, created_at, updated_at FROM mdm_import_jobs WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;

        match row {
            Some(row) => {
                let job = DataImportJob {
                    id: Uuid::parse_str(row.get::<&str, _>("id"))?,
                    job_name: row.get("job_name"),
                    entity_type: row.get("entity_type"),
                    source_file: row.get("source_file"),
                    total_records: row.get("total_records"),
                    processed_records: row.get("processed_records"),
                    success_records: row.get("success_records"),
                    failed_records: row.get("failed_records"),
                    duplicate_records: row.get("duplicate_records"),
                    status: row.get("status"),
                    started_at: row.get("started_at"),
                    completed_at: row.get("completed_at"),
                    error_log: row.get("error_log"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(job))
            }
            None => Ok(None),
        }
    }

    async fn update_import_job(&self, job: &DataImportJob) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE mdm_import_jobs SET processed_records = ?, success_records = ?, failed_records = ?,
                duplicate_records = ?, status = ?, started_at = ?, completed_at = ?, error_log = ?,
                updated_at = ? WHERE id = ?"#
        )
        .bind(job.processed_records)
        .bind(job.success_records)
        .bind(job.failed_records)
        .bind(job.duplicate_records)
        .bind(&job.status)
        .bind(job.started_at)
        .bind(job.completed_at)
        .bind(&job.error_log)
        .bind(job.updated_at)
        .bind(job.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_export_job(&self, job: &DataExportJob) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO mdm_export_jobs (id, job_name, entity_type, filter_criteria, export_format,
                output_file, total_records, status, started_at, completed_at, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(job.id.to_string())
        .bind(&job.job_name)
        .bind(&job.entity_type)
        .bind(job.filter_criteria.clone())
        .bind(&job.export_format)
        .bind(&job.output_file)
        .bind(job.total_records)
        .bind(&job.status)
        .bind(job.started_at)
        .bind(job.completed_at)
        .bind(job.created_at)
        .bind(job.updated_at)
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_export_job(&self, id: Uuid) -> anyhow::Result<Option<DataExportJob>> {
        let row = sqlx::query(
            r#"SELECT id, job_name, entity_type, filter_criteria, export_format, output_file,
                total_records, status, started_at, completed_at, created_at, updated_at
                FROM mdm_export_jobs WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;

        match row {
            Some(row) => {
                let job = DataExportJob {
                    id: Uuid::parse_str(row.get::<&str, _>("id"))?,
                    job_name: row.get("job_name"),
                    entity_type: row.get("entity_type"),
                    filter_criteria: row.get("filter_criteria"),
                    export_format: row.get("export_format"),
                    output_file: row.get("output_file"),
                    total_records: row.get("total_records"),
                    status: row.get("status"),
                    started_at: row.get("started_at"),
                    completed_at: row.get("completed_at"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(job))
            }
            None => Ok(None),
        }
    }

    async fn update_export_job(&self, job: &DataExportJob) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE mdm_export_jobs SET output_file = ?, total_records = ?, status = ?,
                started_at = ?, completed_at = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&job.output_file)
        .bind(job.total_records)
        .bind(&job.status)
        .bind(job.started_at)
        .bind(job.completed_at)
        .bind(job.updated_at)
        .bind(job.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }
}
