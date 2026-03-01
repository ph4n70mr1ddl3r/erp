use crate::models::*;
use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[async_trait]
pub trait PLMRepository {
    async fn create_item(&self, item: &PLMItem) -> anyhow::Result<()>;
    async fn get_item(&self, id: Uuid) -> anyhow::Result<Option<PLMItem>>;
    async fn list_items(&self, status: Option<ItemStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<PLMItem>>;
    async fn update_item(&self, item: &PLMItem) -> anyhow::Result<()>;
    
    async fn create_document(&self, doc: &PLMDocument) -> anyhow::Result<()>;
    async fn get_document(&self, id: Uuid) -> anyhow::Result<Option<PLMDocument>>;
    async fn list_documents(&self, item_id: Option<Uuid>) -> anyhow::Result<Vec<PLMDocument>>;
    async fn update_document(&self, doc: &PLMDocument) -> anyhow::Result<()>;
    
    async fn create_bom(&self, bom: &PLMBOM) -> anyhow::Result<()>;
    async fn get_bom(&self, id: Uuid) -> anyhow::Result<Option<PLMBOM>>;
    async fn list_boms(&self, item_id: Option<Uuid>) -> anyhow::Result<Vec<PLMBOM>>;
    
    async fn create_bom_line(&self, line: &PLMBOMLine) -> anyhow::Result<()>;
    async fn list_bom_lines(&self, bom_id: Uuid) -> anyhow::Result<Vec<PLMBOMLine>>;
    
    async fn create_ecr(&self, ecr: &EngineeringChangeRequest) -> anyhow::Result<()>;
    async fn get_ecr(&self, id: Uuid) -> anyhow::Result<Option<EngineeringChangeRequest>>;
    async fn list_ecrs(&self, status: Option<ChangeRequestStatus>) -> anyhow::Result<Vec<EngineeringChangeRequest>>;
    async fn update_ecr(&self, ecr: &EngineeringChangeRequest) -> anyhow::Result<()>;
    
    async fn create_ecn(&self, ecn: &EngineeringChangeNotice) -> anyhow::Result<()>;
    async fn get_ecn(&self, id: Uuid) -> anyhow::Result<Option<EngineeringChangeNotice>>;
    async fn list_ecns(&self, ecr_id: Option<Uuid>) -> anyhow::Result<Vec<EngineeringChangeNotice>>;
    async fn update_ecn(&self, ecn: &EngineeringChangeNotice) -> anyhow::Result<()>;
    
    async fn create_ecn_affected_item(&self, item: &ECNAffectedItem) -> anyhow::Result<()>;
    async fn list_ecn_affected_items(&self, ecn_id: Uuid) -> anyhow::Result<Vec<ECNAffectedItem>>;
    
    async fn create_workflow(&self, workflow: &PLMWorkflow) -> anyhow::Result<()>;
    async fn get_workflow(&self, id: Uuid) -> anyhow::Result<Option<PLMWorkflow>>;
    async fn update_workflow(&self, workflow: &PLMWorkflow) -> anyhow::Result<()>;
    
    async fn create_workflow_step(&self, step: &PLMWorkflowStep) -> anyhow::Result<()>;
    async fn list_workflow_steps(&self, workflow_id: Uuid) -> anyhow::Result<Vec<PLMWorkflowStep>>;
    async fn update_workflow_step(&self, step: &PLMWorkflowStep) -> anyhow::Result<()>;
    
    async fn create_cad_file(&self, cad: &CADFile) -> anyhow::Result<()>;
    async fn get_cad_file(&self, id: Uuid) -> anyhow::Result<Option<CADFile>>;
    async fn list_cad_files(&self, document_id: Uuid) -> anyhow::Result<Vec<CADFile>>;
    
    async fn create_specification(&self, spec: &Specification) -> anyhow::Result<()>;
    async fn get_specification(&self, id: Uuid) -> anyhow::Result<Option<Specification>>;
    async fn list_specifications(&self, item_id: Option<Uuid>) -> anyhow::Result<Vec<Specification>>;
    
    async fn create_spec_parameter(&self, param: &SpecificationParameter) -> anyhow::Result<()>;
    async fn list_spec_parameters(&self, spec_id: Uuid) -> anyhow::Result<Vec<SpecificationParameter>>;
    
    async fn create_design_review(&self, review: &DesignReview) -> anyhow::Result<()>;
    async fn get_design_review(&self, id: Uuid) -> anyhow::Result<Option<DesignReview>>;
    async fn list_design_reviews(&self, item_id: Uuid) -> anyhow::Result<Vec<DesignReview>>;
    async fn update_design_review(&self, review: &DesignReview) -> anyhow::Result<()>;
    
    async fn create_compliance_requirement(&self, req: &ComplianceRequirement) -> anyhow::Result<()>;
    async fn list_compliance_requirements(&self) -> anyhow::Result<Vec<ComplianceRequirement>>;
    
    async fn create_item_compliance(&self, compliance: &ItemCompliance) -> anyhow::Result<()>;
    async fn list_item_compliances(&self, item_id: Uuid) -> anyhow::Result<Vec<ItemCompliance>>;
}

pub struct SqlitePLMRepository {
    pool: SqlitePool,
}

impl SqlitePLMRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct ItemRow {
    id: String,
    item_number: String,
    name: String,
    description: Option<String>,
    category: String,
    status: String,
    version: String,
    revision: i32,
    lifecycle_phase: String,
    owner_id: Option<String>,
    product_id: Option<String>,
    parent_item_id: Option<String>,
    effective_date: Option<String>,
    obsolete_date: Option<String>,
    security_classification: String,
    created_at: String,
    updated_at: String,
}

impl From<ItemRow> for PLMItem {
    fn from(row: ItemRow) -> Self {
        PLMItem {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            item_number: row.item_number,
            name: row.name,
            description: row.description,
            category: row.category,
            status: row.status.parse().unwrap_or(ItemStatus::Draft),
            version: row.version,
            revision: row.revision,
            lifecycle_phase: row.lifecycle_phase,
            owner_id: row.owner_id.and_then(|s| Uuid::parse_str(&s).ok()),
            product_id: row.product_id.and_then(|s| Uuid::parse_str(&s).ok()),
            parent_item_id: row.parent_item_id.and_then(|s| Uuid::parse_str(&s).ok()),
            effective_date: row.effective_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            obsolete_date: row.obsolete_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            security_classification: row.security_classification,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct DocumentRow {
    id: String,
    document_number: String,
    title: String,
    description: Option<String>,
    document_type: String,
    status: String,
    version: String,
    revision: i32,
    file_path: Option<String>,
    file_size: Option<i64>,
    file_format: Option<String>,
    checksum: Option<String>,
    owner_id: Option<String>,
    checked_out_by: Option<String>,
    checked_out_at: Option<String>,
    effective_date: Option<String>,
    obsolete_date: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<DocumentRow> for PLMDocument {
    fn from(row: DocumentRow) -> Self {
        PLMDocument {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            document_number: row.document_number,
            title: row.title,
            description: row.description,
            document_type: row.document_type.parse().unwrap_or(DocumentType::Other),
            status: row.status.parse().unwrap_or(ItemStatus::Draft),
            version: row.version,
            revision: row.revision,
            file_path: row.file_path,
            file_size: row.file_size,
            file_format: row.file_format,
            checksum: row.checksum,
            owner_id: row.owner_id.and_then(|s| Uuid::parse_str(&s).ok()),
            checked_out_by: row.checked_out_by.and_then(|s| Uuid::parse_str(&s).ok()),
            checked_out_at: row.checked_out_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            effective_date: row.effective_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            obsolete_date: row.obsolete_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[async_trait]
impl PLMRepository for SqlitePLMRepository {
    async fn create_item(&self, item: &PLMItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_items (id, item_number, name, description, category, status, version,
                revision, lifecycle_phase, owner_id, product_id, parent_item_id, effective_date,
                obsolete_date, security_classification, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(item.id.to_string())
        .bind(&item.item_number)
        .bind(&item.name)
        .bind(&item.description)
        .bind(&item.category)
        .bind(format!("{:?}", item.status))
        .bind(&item.version)
        .bind(item.revision)
        .bind(&item.lifecycle_phase)
        .bind(item.owner_id.map(|id| id.to_string()))
        .bind(item.product_id.map(|id| id.to_string()))
        .bind(item.parent_item_id.map(|id| id.to_string()))
        .bind(item.effective_date.map(|d| d.to_rfc3339()))
        .bind(item.obsolete_date.map(|d| d.to_rfc3339()))
        .bind(&item.security_classification)
        .bind(item.created_at.to_rfc3339())
        .bind(item.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_item(&self, id: Uuid) -> anyhow::Result<Option<PLMItem>> {
        let row: Option<ItemRow> = sqlx::query_as::<_, ItemRow>(
            r#"SELECT id, item_number, name, description, category, status,
                version, revision, lifecycle_phase, owner_id, product_id, parent_item_id,
                effective_date, obsolete_date, security_classification, created_at, updated_at
                FROM plm_items WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_items(&self, status: Option<ItemStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<PLMItem>> {
        let rows: Vec<ItemRow> = if let Some(s) = status {
            sqlx::query_as::<_, ItemRow>(
                r#"SELECT id, item_number, name, description, category, status,
                    version, revision, lifecycle_phase, owner_id, product_id, parent_item_id,
                    effective_date, obsolete_date, security_classification, created_at, updated_at
                    FROM plm_items WHERE status = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            )
            .bind(format!("{:?}", s))
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, ItemRow>(
                r#"SELECT id, item_number, name, description, category, status,
                    version, revision, lifecycle_phase, owner_id, product_id, parent_item_id,
                    effective_date, obsolete_date, security_classification, created_at, updated_at
                    FROM plm_items ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool).await?
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_item(&self, item: &PLMItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE plm_items SET name = ?, description = ?, status = ?, version = ?,
                revision = ?, lifecycle_phase = ?, effective_date = ?, obsolete_date = ?,
                updated_at = ? WHERE id = ?"#,
        )
        .bind(&item.name)
        .bind(&item.description)
        .bind(format!("{:?}", item.status))
        .bind(&item.version)
        .bind(item.revision)
        .bind(&item.lifecycle_phase)
        .bind(item.effective_date.map(|d| d.to_rfc3339()))
        .bind(item.obsolete_date.map(|d| d.to_rfc3339()))
        .bind(item.updated_at.to_rfc3339())
        .bind(item.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_document(&self, doc: &PLMDocument) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_documents (id, document_number, title, description, document_type,
                status, version, revision, file_path, file_size, file_format, checksum, owner_id,
                checked_out_by, checked_out_at, effective_date, obsolete_date, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(doc.id.to_string())
        .bind(&doc.document_number)
        .bind(&doc.title)
        .bind(&doc.description)
        .bind(format!("{:?}", doc.document_type))
        .bind(format!("{:?}", doc.status))
        .bind(&doc.version)
        .bind(doc.revision)
        .bind(&doc.file_path)
        .bind(doc.file_size)
        .bind(&doc.file_format)
        .bind(&doc.checksum)
        .bind(doc.owner_id.map(|id| id.to_string()))
        .bind(doc.checked_out_by.map(|id| id.to_string()))
        .bind(doc.checked_out_at.map(|d| d.to_rfc3339()))
        .bind(doc.effective_date.map(|d| d.to_rfc3339()))
        .bind(doc.obsolete_date.map(|d| d.to_rfc3339()))
        .bind(doc.created_at.to_rfc3339())
        .bind(doc.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_document(&self, id: Uuid) -> anyhow::Result<Option<PLMDocument>> {
        let row: Option<DocumentRow> = sqlx::query_as::<_, DocumentRow>(
            r#"SELECT id, document_number, title, description, document_type,
                status, version, revision, file_path, file_size, file_format,
                checksum, owner_id, checked_out_by, checked_out_at, effective_date, obsolete_date,
                created_at, updated_at FROM plm_documents WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_documents(&self, _item_id: Option<Uuid>) -> anyhow::Result<Vec<PLMDocument>> {
        let rows: Vec<DocumentRow> = sqlx::query_as::<_, DocumentRow>(
            r#"SELECT id, document_number, title, description, document_type,
                status, version, revision, file_path, file_size, file_format,
                checksum, owner_id, checked_out_by, checked_out_at, effective_date, obsolete_date,
                created_at, updated_at FROM plm_documents ORDER BY created_at DESC"#,
        )
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_document(&self, doc: &PLMDocument) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE plm_documents SET title = ?, description = ?, status = ?, version = ?,
                revision = ?, checked_out_by = ?, checked_out_at = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&doc.title)
        .bind(&doc.description)
        .bind(format!("{:?}", doc.status))
        .bind(&doc.version)
        .bind(doc.revision)
        .bind(doc.checked_out_by.map(|id| id.to_string()))
        .bind(doc.checked_out_at.map(|d| d.to_rfc3339()))
        .bind(doc.updated_at.to_rfc3339())
        .bind(doc.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_bom(&self, bom: &PLMBOM) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_boms (id, bom_number, name, description, item_id, version,
                revision, status, bom_type, quantity, unit_of_measure, effective_date,
                obsolete_date, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(bom.id.to_string())
        .bind(&bom.bom_number)
        .bind(&bom.name)
        .bind(&bom.description)
        .bind(bom.item_id.to_string())
        .bind(&bom.version)
        .bind(bom.revision)
        .bind(format!("{:?}", bom.status))
        .bind(&bom.bom_type)
        .bind(bom.quantity)
        .bind(&bom.unit_of_measure)
        .bind(bom.effective_date.map(|d| d.to_rfc3339()))
        .bind(bom.obsolete_date.map(|d| d.to_rfc3339()))
        .bind(bom.created_at.to_rfc3339())
        .bind(bom.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_bom(&self, id: Uuid) -> anyhow::Result<Option<PLMBOM>> {
        let row: Option<BOMRow> = sqlx::query_as::<_, BOMRow>(
            r#"SELECT id, bom_number, name, description, item_id, version, revision,
                status, bom_type, quantity, unit_of_measure, effective_date,
                obsolete_date, created_at, updated_at FROM plm_boms WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_boms(&self, item_id: Option<Uuid>) -> anyhow::Result<Vec<PLMBOM>> {
        let rows: Vec<BOMRow> = if let Some(iid) = item_id {
            sqlx::query_as::<_, BOMRow>(
                r#"SELECT id, bom_number, name, description, item_id, version, revision,
                    status, bom_type, quantity, unit_of_measure, effective_date,
                    obsolete_date, created_at, updated_at FROM plm_boms WHERE item_id = ?"#,
            )
            .bind(iid.to_string())
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, BOMRow>(
                r#"SELECT id, bom_number, name, description, item_id, version, revision,
                    status, bom_type, quantity, unit_of_measure, effective_date,
                    obsolete_date, created_at, updated_at FROM plm_boms ORDER BY created_at DESC"#,
            )
            .fetch_all(&self.pool).await?
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_bom_line(&self, line: &PLMBOMLine) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_bom_lines (id, bom_id, item_id, line_number, quantity,
                unit_of_measure, find_number, reference_designator, substitute_item_id,
                is_phantom, sort_order, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(line.id.to_string())
        .bind(line.bom_id.to_string())
        .bind(line.item_id.to_string())
        .bind(line.line_number)
        .bind(line.quantity)
        .bind(&line.unit_of_measure)
        .bind(line.find_number)
        .bind(&line.reference_designator)
        .bind(line.substitute_item_id.map(|id| id.to_string()))
        .bind(line.is_phantom)
        .bind(line.sort_order)
        .bind(line.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_bom_lines(&self, bom_id: Uuid) -> anyhow::Result<Vec<PLMBOMLine>> {
        let rows: Vec<BOMLineRow> = sqlx::query_as::<_, BOMLineRow>(
            r#"SELECT id, bom_id, item_id, line_number, quantity, unit_of_measure, find_number,
                reference_designator, substitute_item_id, is_phantom, sort_order, created_at
                FROM plm_bom_lines WHERE bom_id = ? ORDER BY sort_order"#,
        )
        .bind(bom_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_ecr(&self, ecr: &EngineeringChangeRequest) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_ecrs (id, ecr_number, title, description, reason, priority, status,
                change_type, requested_by, submitted_at, target_date, implemented_date,
                impact_assessment, cost_estimate, currency, approved_by, approved_at,
                rejected_reason, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(ecr.id.to_string())
        .bind(&ecr.ecr_number)
        .bind(&ecr.title)
        .bind(&ecr.description)
        .bind(&ecr.reason)
        .bind(format!("{:?}", ecr.priority))
        .bind(format!("{:?}", ecr.status))
        .bind(&ecr.change_type)
        .bind(ecr.requested_by.to_string())
        .bind(ecr.submitted_at.map(|d| d.to_rfc3339()))
        .bind(ecr.target_date.map(|d| d.to_rfc3339()))
        .bind(ecr.implemented_date.map(|d| d.to_rfc3339()))
        .bind(&ecr.impact_assessment)
        .bind(ecr.cost_estimate)
        .bind(&ecr.currency)
        .bind(ecr.approved_by.map(|id| id.to_string()))
        .bind(ecr.approved_at.map(|d| d.to_rfc3339()))
        .bind(&ecr.rejected_reason)
        .bind(ecr.created_at.to_rfc3339())
        .bind(ecr.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_ecr(&self, id: Uuid) -> anyhow::Result<Option<EngineeringChangeRequest>> {
        let row: Option<ECRRow> = sqlx::query_as::<_, ECRRow>(
            r#"SELECT id, ecr_number, title, description, reason, priority,
                status, change_type, requested_by, submitted_at, target_date,
                implemented_date, impact_assessment, cost_estimate, currency, approved_by,
                approved_at, rejected_reason, created_at, updated_at FROM plm_ecrs WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_ecrs(&self, status: Option<ChangeRequestStatus>) -> anyhow::Result<Vec<EngineeringChangeRequest>> {
        let rows: Vec<ECRRow> = if let Some(s) = status {
            sqlx::query_as::<_, ECRRow>(
                r#"SELECT id, ecr_number, title, description, reason, priority,
                    status, change_type, requested_by, submitted_at, target_date,
                    implemented_date, impact_assessment, cost_estimate, currency, approved_by,
                    approved_at, rejected_reason, created_at, updated_at FROM plm_ecrs WHERE status = ?"#,
            )
            .bind(format!("{:?}", s))
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, ECRRow>(
                r#"SELECT id, ecr_number, title, description, reason, priority,
                    status, change_type, requested_by, submitted_at, target_date,
                    implemented_date, impact_assessment, cost_estimate, currency, approved_by,
                    approved_at, rejected_reason, created_at, updated_at FROM plm_ecrs ORDER BY created_at DESC"#,
            )
            .fetch_all(&self.pool).await?
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_ecr(&self, ecr: &EngineeringChangeRequest) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE plm_ecrs SET title = ?, description = ?, reason = ?, priority = ?,
                status = ?, change_type = ?, target_date = ?, impact_assessment = ?,
                cost_estimate = ?, currency = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&ecr.title)
        .bind(&ecr.description)
        .bind(&ecr.reason)
        .bind(format!("{:?}", ecr.priority))
        .bind(format!("{:?}", ecr.status))
        .bind(&ecr.change_type)
        .bind(ecr.target_date.map(|d| d.to_rfc3339()))
        .bind(&ecr.impact_assessment)
        .bind(ecr.cost_estimate)
        .bind(&ecr.currency)
        .bind(ecr.updated_at.to_rfc3339())
        .bind(ecr.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_ecn(&self, ecn: &EngineeringChangeNotice) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_ecns (id, ecn_number, ecr_id, title, description, status,
                effective_date, implementation_instructions, created_by, approved_by, approved_at,
                created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(ecn.id.to_string())
        .bind(&ecn.ecn_number)
        .bind(ecn.ecr_id.to_string())
        .bind(&ecn.title)
        .bind(&ecn.description)
        .bind(format!("{:?}", ecn.status))
        .bind(ecn.effective_date.to_rfc3339())
        .bind(&ecn.implementation_instructions)
        .bind(ecn.created_by.to_string())
        .bind(ecn.approved_by.map(|id| id.to_string()))
        .bind(ecn.approved_at.map(|d| d.to_rfc3339()))
        .bind(ecn.created_at.to_rfc3339())
        .bind(ecn.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_ecn(&self, id: Uuid) -> anyhow::Result<Option<EngineeringChangeNotice>> {
        let row: Option<ECNRow> = sqlx::query_as::<_, ECNRow>(
            r#"SELECT id, ecn_number, ecr_id, title, description, status,
                effective_date, implementation_instructions, created_by, approved_by, approved_at,
                created_at, updated_at FROM plm_ecns WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_ecns(&self, ecr_id: Option<Uuid>) -> anyhow::Result<Vec<EngineeringChangeNotice>> {
        let rows: Vec<ECNRow> = if let Some(eid) = ecr_id {
            sqlx::query_as::<_, ECNRow>(
                r#"SELECT id, ecn_number, ecr_id, title, description, status,
                    effective_date, implementation_instructions, created_by, approved_by, approved_at,
                    created_at, updated_at FROM plm_ecns WHERE ecr_id = ?"#,
            )
            .bind(eid.to_string())
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, ECNRow>(
                r#"SELECT id, ecn_number, ecr_id, title, description, status,
                    effective_date, implementation_instructions, created_by, approved_by, approved_at,
                    created_at, updated_at FROM plm_ecns ORDER BY created_at DESC"#,
            )
            .fetch_all(&self.pool).await?
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_ecn(&self, ecn: &EngineeringChangeNotice) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE plm_ecns SET title = ?, description = ?, status = ?,
                effective_date = ?, implementation_instructions = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&ecn.title)
        .bind(&ecn.description)
        .bind(format!("{:?}", ecn.status))
        .bind(ecn.effective_date.to_rfc3339())
        .bind(&ecn.implementation_instructions)
        .bind(ecn.updated_at.to_rfc3339())
        .bind(ecn.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_ecn_affected_item(&self, item: &ECNAffectedItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_ecn_affected_items (id, ecn_id, item_id, old_revision, new_revision,
                old_version, new_version, change_description, disposition, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(item.id.to_string())
        .bind(item.ecn_id.to_string())
        .bind(item.item_id.to_string())
        .bind(&item.old_revision)
        .bind(&item.new_revision)
        .bind(&item.old_version)
        .bind(&item.new_version)
        .bind(&item.change_description)
        .bind(&item.disposition)
        .bind(item.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_ecn_affected_items(&self, ecn_id: Uuid) -> anyhow::Result<Vec<ECNAffectedItem>> {
        let rows: Vec<ECNAffectedItemRow> = sqlx::query_as::<_, ECNAffectedItemRow>(
            r#"SELECT id, ecn_id, item_id, old_revision, new_revision,
                old_version, new_version, change_description, disposition, created_at
                FROM plm_ecn_affected_items WHERE ecn_id = ?"#,
        )
        .bind(ecn_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_workflow(&self, workflow: &PLMWorkflow) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_workflows (id, workflow_number, name, description, workflow_type,
                status, initiated_by, current_step, total_steps, started_at, completed_at,
                created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(workflow.id.to_string())
        .bind(&workflow.workflow_number)
        .bind(&workflow.name)
        .bind(&workflow.description)
        .bind(&workflow.workflow_type)
        .bind(&workflow.status)
        .bind(workflow.initiated_by.to_string())
        .bind(workflow.current_step)
        .bind(workflow.total_steps)
        .bind(workflow.started_at.to_rfc3339())
        .bind(workflow.completed_at.map(|d| d.to_rfc3339()))
        .bind(workflow.created_at.to_rfc3339())
        .bind(workflow.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_workflow(&self, id: Uuid) -> anyhow::Result<Option<PLMWorkflow>> {
        let row: Option<WorkflowRow> = sqlx::query_as::<_, WorkflowRow>(
            r#"SELECT id, workflow_number, name, description, workflow_type,
                status, initiated_by, current_step, total_steps, started_at, completed_at,
                created_at, updated_at FROM plm_workflows WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn update_workflow(&self, workflow: &PLMWorkflow) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE plm_workflows SET name = ?, description = ?, status = ?,
                current_step = ?, completed_at = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&workflow.name)
        .bind(&workflow.description)
        .bind(&workflow.status)
        .bind(workflow.current_step)
        .bind(workflow.completed_at.map(|d| d.to_rfc3339()))
        .bind(workflow.updated_at.to_rfc3339())
        .bind(workflow.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_workflow_step(&self, step: &PLMWorkflowStep) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_workflow_steps (id, workflow_id, step_number, step_name, step_type,
                assignee_id, role_id, status, due_date, completed_at, completed_by, comments, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(step.id.to_string())
        .bind(step.workflow_id.to_string())
        .bind(step.step_number)
        .bind(&step.step_name)
        .bind(&step.step_type)
        .bind(step.assignee_id.map(|id| id.to_string()))
        .bind(step.role_id.map(|id| id.to_string()))
        .bind(&step.status)
        .bind(step.due_date.map(|d| d.to_rfc3339()))
        .bind(step.completed_at.map(|d| d.to_rfc3339()))
        .bind(step.completed_by.map(|id| id.to_string()))
        .bind(&step.comments)
        .bind(step.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_workflow_steps(&self, workflow_id: Uuid) -> anyhow::Result<Vec<PLMWorkflowStep>> {
        let rows: Vec<WorkflowStepRow> = sqlx::query_as::<_, WorkflowStepRow>(
            r#"SELECT id, workflow_id, step_number, step_name, step_type,
                assignee_id, role_id, status, due_date, completed_at, completed_by, comments, created_at
                FROM plm_workflow_steps WHERE workflow_id = ? ORDER BY step_number"#,
        )
        .bind(workflow_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_workflow_step(&self, step: &PLMWorkflowStep) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE plm_workflow_steps SET step_name = ?, assignee_id = ?, role_id = ?,
                status = ?, due_date = ?, completed_at = ?, completed_by = ?, comments = ? WHERE id = ?"#,
        )
        .bind(&step.step_name)
        .bind(step.assignee_id.map(|id| id.to_string()))
        .bind(step.role_id.map(|id| id.to_string()))
        .bind(&step.status)
        .bind(step.due_date.map(|d| d.to_rfc3339()))
        .bind(step.completed_at.map(|d| d.to_rfc3339()))
        .bind(step.completed_by.map(|id| id.to_string()))
        .bind(&step.comments)
        .bind(step.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_cad_file(&self, cad: &CADFile) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_cad_files (id, document_id, file_name, file_path, file_size,
                cad_system, format, version, thumbnail_path, geometry_data, metadata, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(cad.id.to_string())
        .bind(cad.document_id.to_string())
        .bind(&cad.file_name)
        .bind(&cad.file_path)
        .bind(cad.file_size)
        .bind(&cad.cad_system)
        .bind(&cad.format)
        .bind(&cad.version)
        .bind(&cad.thumbnail_path)
        .bind(cad.geometry_data.as_ref().map(|v| v.to_string()))
        .bind(cad.metadata.as_ref().map(|v| v.to_string()))
        .bind(cad.created_at.to_rfc3339())
        .bind(cad.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_cad_file(&self, id: Uuid) -> anyhow::Result<Option<CADFile>> {
        let row: Option<CADFileRow> = sqlx::query_as::<_, CADFileRow>(
            r#"SELECT id, document_id, file_name, file_path, file_size,
                cad_system, format, version, thumbnail_path, geometry_data, metadata, created_at, updated_at
                FROM plm_cad_files WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_cad_files(&self, document_id: Uuid) -> anyhow::Result<Vec<CADFile>> {
        let rows: Vec<CADFileRow> = sqlx::query_as::<_, CADFileRow>(
            r#"SELECT id, document_id, file_name, file_path, file_size,
                cad_system, format, version, thumbnail_path, geometry_data, metadata, created_at, updated_at
                FROM plm_cad_files WHERE document_id = ?"#,
        )
        .bind(document_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_specification(&self, spec: &Specification) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_specifications (id, spec_number, name, description, item_id,
                spec_type, status, version, revision, parameters, owner_id, effective_date,
                created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(spec.id.to_string())
        .bind(&spec.spec_number)
        .bind(&spec.name)
        .bind(&spec.description)
        .bind(spec.item_id.map(|id| id.to_string()))
        .bind(&spec.spec_type)
        .bind(format!("{:?}", spec.status))
        .bind(&spec.version)
        .bind(spec.revision)
        .bind(spec.parameters.to_string())
        .bind(spec.owner_id.map(|id| id.to_string()))
        .bind(spec.effective_date.map(|d| d.to_rfc3339()))
        .bind(spec.created_at.to_rfc3339())
        .bind(spec.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_specification(&self, id: Uuid) -> anyhow::Result<Option<Specification>> {
        let row: Option<SpecRow> = sqlx::query_as::<_, SpecRow>(
            r#"SELECT id, spec_number, name, description, item_id,
                spec_type, status, version, revision, parameters, owner_id, effective_date,
                created_at, updated_at FROM plm_specifications WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_specifications(&self, item_id: Option<Uuid>) -> anyhow::Result<Vec<Specification>> {
        let rows: Vec<SpecRow> = if let Some(iid) = item_id {
            sqlx::query_as::<_, SpecRow>(
                r#"SELECT id, spec_number, name, description, item_id,
                    spec_type, status, version, revision, parameters, owner_id, effective_date,
                    created_at, updated_at FROM plm_specifications WHERE item_id = ?"#,
            )
            .bind(iid.to_string())
            .fetch_all(&self.pool).await?
        } else {
            sqlx::query_as::<_, SpecRow>(
                r#"SELECT id, spec_number, name, description, item_id,
                    spec_type, status, version, revision, parameters, owner_id, effective_date,
                    created_at, updated_at FROM plm_specifications ORDER BY created_at DESC"#,
            )
            .fetch_all(&self.pool).await?
        };
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_spec_parameter(&self, param: &SpecificationParameter) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_spec_parameters (id, spec_id, parameter_name, parameter_type,
                target_value, min_value, max_value, unit, test_method, is_critical, sort_order, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(param.id.to_string())
        .bind(param.spec_id.to_string())
        .bind(&param.parameter_name)
        .bind(&param.parameter_type)
        .bind(&param.target_value)
        .bind(&param.min_value)
        .bind(&param.max_value)
        .bind(&param.unit)
        .bind(&param.test_method)
        .bind(param.is_critical)
        .bind(param.sort_order)
        .bind(param.created_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_spec_parameters(&self, spec_id: Uuid) -> anyhow::Result<Vec<SpecificationParameter>> {
        let rows: Vec<SpecParamRow> = sqlx::query_as::<_, SpecParamRow>(
            r#"SELECT id, spec_id, parameter_name, parameter_type, target_value,
                min_value, max_value, unit, test_method, is_critical, sort_order, created_at
                FROM plm_spec_parameters WHERE spec_id = ? ORDER BY sort_order"#,
        )
        .bind(spec_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_design_review(&self, review: &DesignReview) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_design_reviews (id, review_number, item_id, review_type, status,
                scheduled_date, conducted_date, facilitator_id, location, outcome, action_items,
                created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(review.id.to_string())
        .bind(&review.review_number)
        .bind(review.item_id.to_string())
        .bind(&review.review_type)
        .bind(&review.status)
        .bind(review.scheduled_date.to_rfc3339())
        .bind(review.conducted_date.map(|d| d.to_rfc3339()))
        .bind(review.facilitator_id.map(|id| id.to_string()))
        .bind(&review.location)
        .bind(&review.outcome)
        .bind(&review.action_items)
        .bind(review.created_at.to_rfc3339())
        .bind(review.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_design_review(&self, id: Uuid) -> anyhow::Result<Option<DesignReview>> {
        let row: Option<DesignReviewRow> = sqlx::query_as::<_, DesignReviewRow>(
            r#"SELECT id, review_number, item_id, review_type, status,
                scheduled_date, conducted_date, facilitator_id, location, outcome, action_items,
                created_at, updated_at FROM plm_design_reviews WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool).await?;
        Ok(row.map(Into::into))
    }

    async fn list_design_reviews(&self, item_id: Uuid) -> anyhow::Result<Vec<DesignReview>> {
        let rows: Vec<DesignReviewRow> = sqlx::query_as::<_, DesignReviewRow>(
            r#"SELECT id, review_number, item_id, review_type, status,
                scheduled_date, conducted_date, facilitator_id, location, outcome, action_items,
                created_at, updated_at FROM plm_design_reviews WHERE item_id = ? ORDER BY created_at DESC"#,
        )
        .bind(item_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_design_review(&self, review: &DesignReview) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE plm_design_reviews SET review_type = ?, status = ?,
                scheduled_date = ?, conducted_date = ?, facilitator_id = ?, location = ?,
                outcome = ?, action_items = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&review.review_type)
        .bind(&review.status)
        .bind(review.scheduled_date.to_rfc3339())
        .bind(review.conducted_date.map(|d| d.to_rfc3339()))
        .bind(review.facilitator_id.map(|id| id.to_string()))
        .bind(&review.location)
        .bind(&review.outcome)
        .bind(&review.action_items)
        .bind(review.updated_at.to_rfc3339())
        .bind(review.id.to_string())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_compliance_requirement(&self, req: &ComplianceRequirement) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_compliance_requirements (id, requirement_code, name, description,
                regulation, category, mandatory, verification_method, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(req.id.to_string())
        .bind(&req.requirement_code)
        .bind(&req.name)
        .bind(&req.description)
        .bind(&req.regulation)
        .bind(&req.category)
        .bind(req.mandatory)
        .bind(&req.verification_method)
        .bind(req.created_at.to_rfc3339())
        .bind(req.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_compliance_requirements(&self) -> anyhow::Result<Vec<ComplianceRequirement>> {
        let rows: Vec<ComplianceReqRow> = sqlx::query_as::<_, ComplianceReqRow>(
            r#"SELECT id, requirement_code, name, description, regulation, category,
                mandatory, verification_method, created_at, updated_at FROM plm_compliance_requirements"#,
        )
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_item_compliance(&self, compliance: &ItemCompliance) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO plm_item_compliances (id, item_id, requirement_id, status, certified,
                certification_date, certification_expiry, certifying_body, certificate_number,
                notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(compliance.id.to_string())
        .bind(compliance.item_id.to_string())
        .bind(compliance.requirement_id.to_string())
        .bind(&compliance.status)
        .bind(compliance.certified)
        .bind(compliance.certification_date.map(|d| d.to_rfc3339()))
        .bind(compliance.certification_expiry.map(|d| d.to_rfc3339()))
        .bind(&compliance.certifying_body)
        .bind(&compliance.certificate_number)
        .bind(&compliance.notes)
        .bind(compliance.created_at.to_rfc3339())
        .bind(compliance.updated_at.to_rfc3339())
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn list_item_compliances(&self, item_id: Uuid) -> anyhow::Result<Vec<ItemCompliance>> {
        let rows: Vec<ItemComplianceRow> = sqlx::query_as::<_, ItemComplianceRow>(
            r#"SELECT id, item_id, requirement_id, status, certified,
                certification_date, certification_expiry, certifying_body, certificate_number,
                notes, created_at, updated_at FROM plm_item_compliances WHERE item_id = ?"#,
        )
        .bind(item_id.to_string())
        .fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

// Row structs for FromRow
#[derive(Debug, FromRow)]
struct BOMRow {
    id: String,
    bom_number: String,
    name: String,
    description: Option<String>,
    item_id: String,
    version: String,
    revision: i32,
    status: String,
    bom_type: String,
    quantity: f64,
    unit_of_measure: String,
    effective_date: Option<String>,
    obsolete_date: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<BOMRow> for PLMBOM {
    fn from(row: BOMRow) -> Self {
        PLMBOM {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            bom_number: row.bom_number,
            name: row.name,
            description: row.description,
            item_id: Uuid::parse_str(&row.item_id).unwrap_or(Uuid::nil()),
            version: row.version,
            revision: row.revision,
            status: row.status.parse().unwrap_or(ItemStatus::Draft),
            bom_type: row.bom_type,
            quantity: row.quantity,
            unit_of_measure: row.unit_of_measure,
            effective_date: row.effective_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            obsolete_date: row.obsolete_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct BOMLineRow {
    id: String,
    bom_id: String,
    item_id: String,
    line_number: i32,
    quantity: f64,
    unit_of_measure: String,
    find_number: Option<i32>,
    reference_designator: Option<String>,
    substitute_item_id: Option<String>,
    is_phantom: bool,
    sort_order: i32,
    created_at: String,
}

impl From<BOMLineRow> for PLMBOMLine {
    fn from(row: BOMLineRow) -> Self {
        PLMBOMLine {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            bom_id: Uuid::parse_str(&row.bom_id).unwrap_or(Uuid::nil()),
            item_id: Uuid::parse_str(&row.item_id).unwrap_or(Uuid::nil()),
            line_number: row.line_number,
            quantity: row.quantity,
            unit_of_measure: row.unit_of_measure,
            find_number: row.find_number,
            reference_designator: row.reference_designator,
            substitute_item_id: row.substitute_item_id.and_then(|s| Uuid::parse_str(&s).ok()),
            is_phantom: row.is_phantom,
            sort_order: row.sort_order,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ECRRow {
    id: String,
    ecr_number: String,
    title: String,
    description: String,
    reason: String,
    priority: String,
    status: String,
    change_type: String,
    requested_by: String,
    submitted_at: Option<String>,
    target_date: Option<String>,
    implemented_date: Option<String>,
    impact_assessment: Option<String>,
    cost_estimate: Option<i64>,
    currency: Option<String>,
    approved_by: Option<String>,
    approved_at: Option<String>,
    rejected_reason: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ECRRow> for EngineeringChangeRequest {
    fn from(row: ECRRow) -> Self {
        EngineeringChangeRequest {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            ecr_number: row.ecr_number,
            title: row.title,
            description: row.description,
            reason: row.reason,
            priority: row.priority.parse().unwrap_or(ChangeRequestPriority::Medium),
            status: row.status.parse().unwrap_or(ChangeRequestStatus::Draft),
            change_type: row.change_type,
            requested_by: Uuid::parse_str(&row.requested_by).unwrap_or(Uuid::nil()),
            submitted_at: row.submitted_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            target_date: row.target_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            implemented_date: row.implemented_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            impact_assessment: row.impact_assessment,
            cost_estimate: row.cost_estimate,
            currency: row.currency,
            approved_by: row.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            approved_at: row.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            rejected_reason: row.rejected_reason,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ECNRow {
    id: String,
    ecn_number: String,
    ecr_id: String,
    title: String,
    description: String,
    status: String,
    effective_date: String,
    implementation_instructions: Option<String>,
    created_by: String,
    approved_by: Option<String>,
    approved_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ECNRow> for EngineeringChangeNotice {
    fn from(row: ECNRow) -> Self {
        EngineeringChangeNotice {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            ecn_number: row.ecn_number,
            ecr_id: Uuid::parse_str(&row.ecr_id).unwrap_or(Uuid::nil()),
            title: row.title,
            description: row.description,
            status: row.status.parse().unwrap_or(ChangeRequestStatus::Draft),
            effective_date: chrono::DateTime::parse_from_rfc3339(&row.effective_date).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            implementation_instructions: row.implementation_instructions,
            created_by: Uuid::parse_str(&row.created_by).unwrap_or(Uuid::nil()),
            approved_by: row.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            approved_at: row.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ECNAffectedItemRow {
    id: String,
    ecn_id: String,
    item_id: String,
    old_revision: String,
    new_revision: String,
    old_version: String,
    new_version: String,
    change_description: String,
    disposition: String,
    created_at: String,
}

impl From<ECNAffectedItemRow> for ECNAffectedItem {
    fn from(row: ECNAffectedItemRow) -> Self {
        ECNAffectedItem {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            ecn_id: Uuid::parse_str(&row.ecn_id).unwrap_or(Uuid::nil()),
            item_id: Uuid::parse_str(&row.item_id).unwrap_or(Uuid::nil()),
            old_revision: row.old_revision,
            new_revision: row.new_revision,
            old_version: row.old_version,
            new_version: row.new_version,
            change_description: row.change_description,
            disposition: row.disposition,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct WorkflowRow {
    id: String,
    workflow_number: String,
    name: String,
    description: Option<String>,
    workflow_type: String,
    status: String,
    initiated_by: String,
    current_step: i32,
    total_steps: i32,
    started_at: String,
    completed_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<WorkflowRow> for PLMWorkflow {
    fn from(row: WorkflowRow) -> Self {
        PLMWorkflow {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            workflow_number: row.workflow_number,
            name: row.name,
            description: row.description,
            workflow_type: row.workflow_type,
            status: row.status,
            initiated_by: Uuid::parse_str(&row.initiated_by).unwrap_or(Uuid::nil()),
            current_step: row.current_step,
            total_steps: row.total_steps,
            started_at: chrono::DateTime::parse_from_rfc3339(&row.started_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            completed_at: row.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct WorkflowStepRow {
    id: String,
    workflow_id: String,
    step_number: i32,
    step_name: String,
    step_type: String,
    assignee_id: Option<String>,
    role_id: Option<String>,
    status: String,
    due_date: Option<String>,
    completed_at: Option<String>,
    completed_by: Option<String>,
    comments: Option<String>,
    created_at: String,
}

impl From<WorkflowStepRow> for PLMWorkflowStep {
    fn from(row: WorkflowStepRow) -> Self {
        PLMWorkflowStep {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            workflow_id: Uuid::parse_str(&row.workflow_id).unwrap_or(Uuid::nil()),
            step_number: row.step_number,
            step_name: row.step_name,
            step_type: row.step_type,
            assignee_id: row.assignee_id.and_then(|s| Uuid::parse_str(&s).ok()),
            role_id: row.role_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: row.status,
            due_date: row.due_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            completed_at: row.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            completed_by: row.completed_by.and_then(|s| Uuid::parse_str(&s).ok()),
            comments: row.comments,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct CADFileRow {
    id: String,
    document_id: String,
    file_name: String,
    file_path: String,
    file_size: i64,
    cad_system: String,
    format: String,
    version: String,
    thumbnail_path: Option<String>,
    geometry_data: Option<String>,
    metadata: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<CADFileRow> for CADFile {
    fn from(row: CADFileRow) -> Self {
        CADFile {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            document_id: Uuid::parse_str(&row.document_id).unwrap_or(Uuid::nil()),
            file_name: row.file_name,
            file_path: row.file_path,
            file_size: row.file_size,
            cad_system: row.cad_system,
            format: row.format,
            version: row.version,
            thumbnail_path: row.thumbnail_path,
            geometry_data: row.geometry_data.and_then(|s| serde_json::from_str(&s).ok()),
            metadata: row.metadata.and_then(|s| serde_json::from_str(&s).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct SpecRow {
    id: String,
    spec_number: String,
    name: String,
    description: Option<String>,
    item_id: Option<String>,
    spec_type: String,
    status: String,
    version: String,
    revision: i32,
    parameters: String,
    owner_id: Option<String>,
    effective_date: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<SpecRow> for Specification {
    fn from(row: SpecRow) -> Self {
        Specification {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            spec_number: row.spec_number,
            name: row.name,
            description: row.description,
            item_id: row.item_id.and_then(|s| Uuid::parse_str(&s).ok()),
            spec_type: row.spec_type,
            status: row.status.parse().unwrap_or(ItemStatus::Draft),
            version: row.version,
            revision: row.revision,
            parameters: serde_json::from_str(&row.parameters).unwrap_or(serde_json::Value::Null),
            owner_id: row.owner_id.and_then(|s| Uuid::parse_str(&s).ok()),
            effective_date: row.effective_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct SpecParamRow {
    id: String,
    spec_id: String,
    parameter_name: String,
    parameter_type: String,
    target_value: String,
    min_value: Option<String>,
    max_value: Option<String>,
    unit: Option<String>,
    test_method: Option<String>,
    is_critical: bool,
    sort_order: i32,
    created_at: String,
}

impl From<SpecParamRow> for SpecificationParameter {
    fn from(row: SpecParamRow) -> Self {
        SpecificationParameter {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            spec_id: Uuid::parse_str(&row.spec_id).unwrap_or(Uuid::nil()),
            parameter_name: row.parameter_name,
            parameter_type: row.parameter_type,
            target_value: row.target_value,
            min_value: row.min_value,
            max_value: row.max_value,
            unit: row.unit,
            test_method: row.test_method,
            is_critical: row.is_critical,
            sort_order: row.sort_order,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct DesignReviewRow {
    id: String,
    review_number: String,
    item_id: String,
    review_type: String,
    status: String,
    scheduled_date: String,
    conducted_date: Option<String>,
    facilitator_id: Option<String>,
    location: Option<String>,
    outcome: Option<String>,
    action_items: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<DesignReviewRow> for DesignReview {
    fn from(row: DesignReviewRow) -> Self {
        DesignReview {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            review_number: row.review_number,
            item_id: Uuid::parse_str(&row.item_id).unwrap_or(Uuid::nil()),
            review_type: row.review_type,
            status: row.status,
            scheduled_date: chrono::DateTime::parse_from_rfc3339(&row.scheduled_date).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            conducted_date: row.conducted_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            facilitator_id: row.facilitator_id.and_then(|s| Uuid::parse_str(&s).ok()),
            location: row.location,
            outcome: row.outcome,
            action_items: row.action_items,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ComplianceReqRow {
    id: String,
    requirement_code: String,
    name: String,
    description: Option<String>,
    regulation: String,
    category: String,
    mandatory: bool,
    verification_method: String,
    created_at: String,
    updated_at: String,
}

impl From<ComplianceReqRow> for ComplianceRequirement {
    fn from(row: ComplianceReqRow) -> Self {
        ComplianceRequirement {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            requirement_code: row.requirement_code,
            name: row.name,
            description: row.description,
            regulation: row.regulation,
            category: row.category,
            mandatory: row.mandatory,
            verification_method: row.verification_method,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(Debug, FromRow)]
struct ItemComplianceRow {
    id: String,
    item_id: String,
    requirement_id: String,
    status: String,
    certified: bool,
    certification_date: Option<String>,
    certification_expiry: Option<String>,
    certifying_body: Option<String>,
    certificate_number: Option<String>,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ItemComplianceRow> for ItemCompliance {
    fn from(row: ItemComplianceRow) -> Self {
        ItemCompliance {
            id: Uuid::parse_str(&row.id).unwrap_or(Uuid::nil()),
            item_id: Uuid::parse_str(&row.item_id).unwrap_or(Uuid::nil()),
            requirement_id: Uuid::parse_str(&row.requirement_id).unwrap_or(Uuid::nil()),
            status: row.status,
            certified: row.certified,
            certification_date: row.certification_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            certification_expiry: row.certification_expiry.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
            certifying_body: row.certifying_body,
            certificate_number: row.certificate_number,
            notes: row.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
