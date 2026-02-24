use crate::models::*;
use async_trait::async_trait;
use sqlx::SqlitePool;
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
        sqlx::query!(
            r#"INSERT INTO mdm_entities (id, entity_type, entity_code, entity_name, source_system,
                source_id, golden_record_id, quality_score, completeness_score, accuracy_score,
                timeliness_score, consistency_score, last_verified, next_verification, status,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            entity.id, entity.entity_type, entity.entity_code, entity.entity_name, entity.source_system,
            entity.source_id, entity.golden_record_id, entity.quality_score, entity.completeness_score,
            entity.accuracy_score, entity.timeliness_score, entity.consistency_score, entity.last_verified,
            entity.next_verification, entity.status as _, entity.created_at, entity.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_master_entity(&self, id: Uuid) -> anyhow::Result<Option<MasterDataEntity>> {
        let entity = sqlx::query_as!(
            MasterDataEntity,
            r#"SELECT id, entity_type, entity_code, entity_name, source_system, source_id,
                golden_record_id, quality_score, completeness_score, accuracy_score,
                timeliness_score, consistency_score, last_verified, next_verification,
                status as "status: _", created_at, updated_at FROM mdm_entities WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(entity)
    }

    async fn list_master_entities(&self, entity_type: Option<&str>, limit: i32, offset: i32) -> anyhow::Result<Vec<MasterDataEntity>> {
        let entities = if let Some(et) = entity_type {
            sqlx::query_as!(
                MasterDataEntity,
                r#"SELECT id, entity_type, entity_code, entity_name, source_system, source_id,
                    golden_record_id, quality_score, completeness_score, accuracy_score,
                    timeliness_score, consistency_score, last_verified, next_verification,
                    status as "status: _", created_at, updated_at FROM mdm_entities
                    WHERE entity_type = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
                    et, limit, offset
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                MasterDataEntity,
                r#"SELECT id, entity_type, entity_code, entity_name, source_system, source_id,
                    golden_record_id, quality_score, completeness_score, accuracy_score,
                    timeliness_score, consistency_score, last_verified, next_verification,
                    status as "status: _", created_at, updated_at FROM mdm_entities
                    ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
                    limit, offset
            ).fetch_all(&self.pool).await?
        };
        Ok(entities)
    }

    async fn update_master_entity(&self, entity: &MasterDataEntity) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE mdm_entities SET entity_name = ?, golden_record_id = ?, quality_score = ?,
                completeness_score = ?, accuracy_score = ?, timeliness_score = ?, consistency_score = ?,
                last_verified = ?, next_verification = ?, status = ?, updated_at = ? WHERE id = ?"#,
            entity.entity_name, entity.golden_record_id, entity.quality_score, entity.completeness_score,
            entity.accuracy_score, entity.timeliness_score, entity.consistency_score, entity.last_verified,
            entity.next_verification, entity.status as _, entity.updated_at, entity.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_golden_record(&self, record: &GoldenRecord) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_golden_records (id, entity_type, golden_code, name, attributes,
                source_count, confidence_score, steward_id, status, version, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            record.id, record.entity_type, record.golden_code, record.name, record.attributes,
            record.source_count, record.confidence_score, record.steward_id, record.status as _,
            record.version, record.created_at, record.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_golden_record(&self, id: Uuid) -> anyhow::Result<Option<GoldenRecord>> {
        let record = sqlx::query_as!(
            GoldenRecord,
            r#"SELECT id, entity_type, golden_code, name, attributes, source_count, confidence_score,
                steward_id, status as "status: _", version, created_at, updated_at
                FROM mdm_golden_records WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(record)
    }

    async fn list_golden_records(&self, entity_type: &str) -> anyhow::Result<Vec<GoldenRecord>> {
        let records = sqlx::query_as!(
            GoldenRecord,
            r#"SELECT id, entity_type, golden_code, name, attributes, source_count, confidence_score,
                steward_id, status as "status: _", version, created_at, updated_at
                FROM mdm_golden_records WHERE entity_type = ? ORDER BY created_at DESC"#,
            entity_type
        ).fetch_all(&self.pool).await?;
        Ok(records)
    }

    async fn update_golden_record(&self, record: &GoldenRecord) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE mdm_golden_records SET name = ?, attributes = ?, source_count = ?,
                confidence_score = ?, status = ?, version = ?, updated_at = ? WHERE id = ?"#,
            record.name, record.attributes, record.source_count, record.confidence_score,
            record.status as _, record.version, record.updated_at, record.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_golden_record_source(&self, source: &GoldenRecordSource) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_golden_record_sources (id, golden_record_id, source_entity_id,
                match_score, is_primary, contributed_at, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            source.id, source.golden_record_id, source.source_entity_id, source.match_score,
            source.is_primary, source.contributed_at, source.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_golden_record_sources(&self, golden_record_id: Uuid) -> anyhow::Result<Vec<GoldenRecordSource>> {
        let sources = sqlx::query_as!(
            GoldenRecordSource,
            r#"SELECT id, golden_record_id, source_entity_id, match_score, is_primary,
                contributed_at, created_at FROM mdm_golden_record_sources
                WHERE golden_record_id = ?"#,
            golden_record_id
        ).fetch_all(&self.pool).await?;
        Ok(sources)
    }

    async fn create_quality_rule(&self, rule: &DataQualityRule) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_quality_rules (id, rule_code, name, description, entity_type,
                field_name, rule_type, rule_expression, severity, is_active, auto_fix, fix_expression,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.id, rule.rule_code, rule.name, rule.description, rule.entity_type, rule.field_name,
            rule.rule_type, rule.rule_expression, rule.severity, rule.is_active, rule.auto_fix,
            rule.fix_expression, rule.created_at, rule.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_quality_rule(&self, id: Uuid) -> anyhow::Result<Option<DataQualityRule>> {
        let rule = sqlx::query_as!(
            DataQualityRule,
            r#"SELECT id, rule_code, name, description, entity_type, field_name, rule_type,
                rule_expression, severity, is_active, auto_fix, fix_expression, created_at, updated_at
                FROM mdm_quality_rules WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(rule)
    }

    async fn list_quality_rules(&self, entity_type: Option<&str>) -> anyhow::Result<Vec<DataQualityRule>> {
        let rules = if let Some(et) = entity_type {
            sqlx::query_as!(
                DataQualityRule,
                r#"SELECT id, rule_code, name, description, entity_type, field_name, rule_type,
                    rule_expression, severity, is_active, auto_fix, fix_expression, created_at, updated_at
                    FROM mdm_quality_rules WHERE entity_type = ? AND is_active = 1"#,
                    et
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                DataQualityRule,
                r#"SELECT id, rule_code, name, description, entity_type, field_name, rule_type,
                    rule_expression, severity, is_active, auto_fix, fix_expression, created_at, updated_at
                    FROM mdm_quality_rules WHERE is_active = 1"#
            ).fetch_all(&self.pool).await?
        };
        Ok(rules)
    }

    async fn create_quality_violation(&self, violation: &DataQualityViolation) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_quality_violations (id, rule_id, entity_id, entity_type, field_name,
                current_value, expected_value, severity, status, detected_at, resolved_at,
                resolved_by, resolution_notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            violation.id, violation.rule_id, violation.entity_id, violation.entity_type,
            violation.field_name, violation.current_value, violation.expected_value, violation.severity,
            violation.status, violation.detected_at, violation.resolved_at, violation.resolved_by,
            violation.resolution_notes, violation.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_quality_violations(&self, entity_id: Option<Uuid>, status: Option<&str>) -> anyhow::Result<Vec<DataQualityViolation>> {
        let violations = sqlx::query_as!(
            DataQualityViolation,
            r#"SELECT id, rule_id, entity_id, entity_type, field_name, current_value, expected_value,
                severity, status, detected_at, resolved_at, resolved_by, resolution_notes, created_at
                FROM mdm_quality_violations ORDER BY detected_at DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(violations)
    }

    async fn update_quality_violation(&self, violation: &DataQualityViolation) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE mdm_quality_violations SET status = ?, resolved_at = ?, resolved_by = ?,
                resolution_notes = ? WHERE id = ?"#,
            violation.status, violation.resolved_at, violation.resolved_by, violation.resolution_notes,
            violation.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_match_rule(&self, rule: &MatchRule) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_match_rules (id, rule_code, name, description, entity_type,
                match_type, blocking_rules, matching_rules, threshold_score, is_active,
                created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.id, rule.rule_code, rule.name, rule.description, rule.entity_type, rule.match_type,
            rule.blocking_rules, rule.matching_rules, rule.threshold_score, rule.is_active,
            rule.created_at, rule.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_match_rules(&self, entity_type: &str) -> anyhow::Result<Vec<MatchRule>> {
        let rules = sqlx::query_as!(
            MatchRule,
            r#"SELECT id, rule_code, name, description, entity_type, match_type, blocking_rules,
                matching_rules, threshold_score, is_active, created_at, updated_at
                FROM mdm_match_rules WHERE entity_type = ? AND is_active = 1"#,
            entity_type
        ).fetch_all(&self.pool).await?;
        Ok(rules)
    }

    async fn create_match_result(&self, result: &MatchResult) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_match_results (id, rule_id, entity1_id, entity2_id, match_score,
                status, matched_at, reviewed_by, reviewed_at, decision_notes, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            result.id, result.rule_id, result.entity1_id, result.entity2_id, result.match_score,
            result.status as _, result.matched_at, result.reviewed_by, result.reviewed_at,
            result.decision_notes, result.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_match_results(&self, rule_id: Uuid) -> anyhow::Result<Vec<MatchResult>> {
        let results = sqlx::query_as!(
            MatchResult,
            r#"SELECT id, rule_id, entity1_id, entity2_id, match_score, status as "status: _",
                matched_at, reviewed_by, reviewed_at, decision_notes, created_at
                FROM mdm_match_results WHERE rule_id = ?"#,
            rule_id
        ).fetch_all(&self.pool).await?;
        Ok(results)
    }

    async fn update_match_result(&self, result: &MatchResult) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE mdm_match_results SET status = ?, reviewed_by = ?, reviewed_at = ?,
                decision_notes = ? WHERE id = ?"#,
            result.status as _, result.reviewed_by, result.reviewed_at, result.decision_notes, result.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_data_domain(&self, domain: &DataDomain) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_data_domains (id, domain_code, name, description, parent_domain_id,
                owner_id, steward_id, data_classification, retention_policy, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            domain.id, domain.domain_code, domain.name, domain.description, domain.parent_domain_id,
            domain.owner_id, domain.steward_id, domain.data_classification, domain.retention_policy,
            domain.created_at, domain.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_data_domains(&self) -> anyhow::Result<Vec<DataDomain>> {
        let domains = sqlx::query_as!(
            DataDomain,
            r#"SELECT id, domain_code, name, description, parent_domain_id, owner_id, steward_id,
                data_classification, retention_policy, created_at, updated_at
                FROM mdm_data_domains ORDER BY domain_code"#
        ).fetch_all(&self.pool).await?;
        Ok(domains)
    }

    async fn create_data_attribute(&self, attr: &DataAttribute) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_data_attributes (id, domain_id, attribute_code, name, description,
                data_type, max_length, is_required, is_unique, default_value, validation_regex,
                allowed_values, business_rules, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            attr.id, attr.domain_id, attr.attribute_code, attr.name, attr.description,
            attr.data_type, attr.max_length, attr.is_required, attr.is_unique, attr.default_value,
            attr.validation_regex, attr.allowed_values, attr.business_rules, attr.created_at, attr.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_data_attributes(&self, domain_id: Uuid) -> anyhow::Result<Vec<DataAttribute>> {
        let attrs = sqlx::query_as!(
            DataAttribute,
            r#"SELECT id, domain_id, attribute_code, name, description, data_type, max_length,
                is_required, is_unique, default_value, validation_regex, allowed_values,
                business_rules, created_at, updated_at FROM mdm_data_attributes WHERE domain_id = ?
                ORDER BY attribute_code"#,
            domain_id
        ).fetch_all(&self.pool).await?;
        Ok(attrs)
    }

    async fn create_data_steward(&self, steward: &DataSteward) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_data_stewards (id, user_id, domain_id, entity_types, responsibilities,
                is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            steward.id, steward.user_id, steward.domain_id, steward.entity_types, steward.responsibilities,
            steward.is_active, steward.created_at, steward.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_data_stewards(&self) -> anyhow::Result<Vec<DataSteward>> {
        let stewards = sqlx::query_as!(
            DataSteward,
            r#"SELECT id, user_id, domain_id, entity_types, responsibilities, is_active,
                created_at, updated_at FROM mdm_data_stewards WHERE is_active = 1"#
        ).fetch_all(&self.pool).await?;
        Ok(stewards)
    }

    async fn create_duplicate_record(&self, dup: &DuplicateRecord) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_duplicate_records (id, entity_type, primary_entity_id,
                duplicate_entity_id, similarity_score, matched_fields, status, merge_initiated_by,
                merge_initiated_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            dup.id, dup.entity_type, dup.primary_entity_id, dup.duplicate_entity_id,
            dup.similarity_score, dup.matched_fields, dup.status, dup.merge_initiated_by,
            dup.merge_initiated_at, dup.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_duplicate_records(&self, entity_type: Option<&str>, status: Option<&str>) -> anyhow::Result<Vec<DuplicateRecord>> {
        let records = sqlx::query_as!(
            DuplicateRecord,
            r#"SELECT id, entity_type, primary_entity_id, duplicate_entity_id, similarity_score,
                matched_fields, status, merge_initiated_by, merge_initiated_at, created_at
                FROM mdm_duplicate_records ORDER BY created_at DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(records)
    }

    async fn update_duplicate_record(&self, dup: &DuplicateRecord) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE mdm_duplicate_records SET status = ?, merge_initiated_by = ?,
                merge_initiated_at = ? WHERE id = ?"#,
            dup.status, dup.merge_initiated_by, dup.merge_initiated_at, dup.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_reference_data(&self, data: &ReferenceData) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_reference_data (id, category, code, name, description, parent_code,
                sort_order, is_active, effective_date, expiry_date, attributes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            data.id, data.category, data.code, data.name, data.description, data.parent_code,
            data.sort_order, data.is_active, data.effective_date, data.expiry_date, data.attributes,
            data.created_at, data.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_reference_data(&self, category: &str) -> anyhow::Result<Vec<ReferenceData>> {
        let data = sqlx::query_as!(
            ReferenceData,
            r#"SELECT id, category, code, name, description, parent_code, sort_order, is_active,
                effective_date, expiry_date, attributes, created_at, updated_at
                FROM mdm_reference_data WHERE category = ? AND is_active = 1 ORDER BY sort_order"#,
            category
        ).fetch_all(&self.pool).await?;
        Ok(data)
    }

    async fn update_reference_data(&self, data: &ReferenceData) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE mdm_reference_data SET name = ?, description = ?, sort_order = ?, is_active = ?,
                effective_date = ?, expiry_date = ?, attributes = ?, updated_at = ? WHERE id = ?"#,
            data.name, data.description, data.sort_order, data.is_active, data.effective_date,
            data.expiry_date, data.attributes, data.updated_at, data.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_import_job(&self, job: &DataImportJob) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_import_jobs (id, job_name, entity_type, source_file, total_records,
                processed_records, success_records, failed_records, duplicate_records, status,
                started_at, completed_at, error_log, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            job.id, job.job_name, job.entity_type, job.source_file, job.total_records,
            job.processed_records, job.success_records, job.failed_records, job.duplicate_records,
            job.status, job.started_at, job.completed_at, job.error_log, job.created_at, job.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_import_job(&self, id: Uuid) -> anyhow::Result<Option<DataImportJob>> {
        let job = sqlx::query_as!(
            DataImportJob,
            r#"SELECT id, job_name, entity_type, source_file, total_records, processed_records,
                success_records, failed_records, duplicate_records, status, started_at, completed_at,
                error_log, created_at, updated_at FROM mdm_import_jobs WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(job)
    }

    async fn update_import_job(&self, job: &DataImportJob) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE mdm_import_jobs SET processed_records = ?, success_records = ?, failed_records = ?,
                duplicate_records = ?, status = ?, started_at = ?, completed_at = ?, error_log = ?,
                updated_at = ? WHERE id = ?"#,
            job.processed_records, job.success_records, job.failed_records, job.duplicate_records,
            job.status, job.started_at, job.completed_at, job.error_log, job.updated_at, job.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_export_job(&self, job: &DataExportJob) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO mdm_export_jobs (id, job_name, entity_type, filter_criteria, export_format,
                output_file, total_records, status, started_at, completed_at, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            job.id, job.job_name, job.entity_type, job.filter_criteria, job.export_format,
            job.output_file, job.total_records, job.status, job.started_at, job.completed_at,
            job.created_at, job.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_export_job(&self, id: Uuid) -> anyhow::Result<Option<DataExportJob>> {
        let job = sqlx::query_as!(
            DataExportJob,
            r#"SELECT id, job_name, entity_type, filter_criteria, export_format, output_file,
                total_records, status, started_at, completed_at, created_at, updated_at
                FROM mdm_export_jobs WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(job)
    }

    async fn update_export_job(&self, job: &DataExportJob) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE mdm_export_jobs SET output_file = ?, total_records = ?, status = ?,
                started_at = ?, completed_at = ?, updated_at = ? WHERE id = ?"#,
            job.output_file, job.total_records, job.status, job.started_at, job.completed_at,
            job.updated_at, job.id
        ).execute(&self.pool).await?;
        Ok(())
    }
}
