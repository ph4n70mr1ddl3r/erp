use crate::models::*;
use async_trait::async_trait;
use sqlx::SqlitePool;
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

#[async_trait]
impl PLMRepository for SqlitePLMRepository {
    async fn create_item(&self, item: &PLMItem) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_items (id, item_number, name, description, category, status, version,
                revision, lifecycle_phase, owner_id, product_id, parent_item_id, effective_date,
                obsolete_date, security_classification, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            item.id, item.item_number, item.name, item.description, item.category,
            item.status as _, item.version, item.revision, item.lifecycle_phase,
            item.owner_id, item.product_id, item.parent_item_id, item.effective_date,
            item.obsolete_date, item.security_classification, item.created_at, item.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_item(&self, id: Uuid) -> anyhow::Result<Option<PLMItem>> {
        let item = sqlx::query_as!(
            PLMItem,
            r#"SELECT id, item_number, name, description, category, status as "status: _",
                version, revision, lifecycle_phase, owner_id, product_id, parent_item_id,
                effective_date, obsolete_date, security_classification, created_at, updated_at
                FROM plm_items WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(item)
    }

    async fn list_items(&self, status: Option<ItemStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<PLMItem>> {
        let items = if let Some(s) = status {
            sqlx::query_as!(
                PLMItem,
                r#"SELECT id, item_number, name, description, category, status as "status: _",
                    version, revision, lifecycle_phase, owner_id, product_id, parent_item_id,
                    effective_date, obsolete_date, security_classification, created_at, updated_at
                    FROM plm_items WHERE status = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
                    s as _, limit, offset
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                PLMItem,
                r#"SELECT id, item_number, name, description, category, status as "status: _",
                    version, revision, lifecycle_phase, owner_id, product_id, parent_item_id,
                    effective_date, obsolete_date, security_classification, created_at, updated_at
                    FROM plm_items ORDER BY created_at DESC LIMIT ? OFFSET ?"#,
                    limit, offset
            ).fetch_all(&self.pool).await?
        };
        Ok(items)
    }

    async fn update_item(&self, item: &PLMItem) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE plm_items SET name = ?, description = ?, status = ?, version = ?,
                revision = ?, lifecycle_phase = ?, effective_date = ?, obsolete_date = ?,
                updated_at = ? WHERE id = ?"#,
            item.name, item.description, item.status as _, item.version, item.revision,
            item.lifecycle_phase, item.effective_date, item.obsolete_date, item.updated_at, item.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_document(&self, doc: &PLMDocument) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_documents (id, document_number, title, description, document_type,
                status, version, revision, file_path, file_size, file_format, checksum, owner_id,
                checked_out_by, checked_out_at, effective_date, obsolete_date, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            doc.id, doc.document_number, doc.title, doc.description, doc.document_type as _,
            doc.status as _, doc.version, doc.revision, doc.file_path, doc.file_size,
            doc.file_format, doc.checksum, doc.owner_id, doc.checked_out_by, doc.checked_out_at,
            doc.effective_date, doc.obsolete_date, doc.created_at, doc.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_document(&self, id: Uuid) -> anyhow::Result<Option<PLMDocument>> {
        let doc = sqlx::query_as!(
            PLMDocument,
            r#"SELECT id, document_number, title, description, document_type as "document_type: _",
                status as "status: _", version, revision, file_path, file_size, file_format,
                checksum, owner_id, checked_out_by, checked_out_at, effective_date, obsolete_date,
                created_at, updated_at FROM plm_documents WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(doc)
    }

    async fn list_documents(&self, item_id: Option<Uuid>) -> anyhow::Result<Vec<PLMDocument>> {
        let docs = sqlx::query_as!(
            PLMDocument,
            r#"SELECT id, document_number, title, description, document_type as "document_type: _",
                status as "status: _", version, revision, file_path, file_size, file_format,
                checksum, owner_id, checked_out_by, checked_out_at, effective_date, obsolete_date,
                created_at, updated_at FROM plm_documents ORDER BY created_at DESC"#
        ).fetch_all(&self.pool).await?;
        Ok(docs)
    }

    async fn update_document(&self, doc: &PLMDocument) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE plm_documents SET title = ?, description = ?, status = ?, version = ?,
                revision = ?, checked_out_by = ?, checked_out_at = ?, updated_at = ? WHERE id = ?"#,
            doc.title, doc.description, doc.status as _, doc.version, doc.revision,
            doc.checked_out_by, doc.checked_out_at, doc.updated_at, doc.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_bom(&self, bom: &PLMBOM) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_boms (id, bom_number, name, description, item_id, version,
                revision, status, bom_type, quantity, unit_of_measure, effective_date,
                obsolete_date, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            bom.id, bom.bom_number, bom.name, bom.description, bom.item_id, bom.version,
            bom.revision, bom.status as _, bom.bom_type, bom.quantity, bom.unit_of_measure,
            bom.effective_date, bom.obsolete_date, bom.created_at, bom.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_bom(&self, id: Uuid) -> anyhow::Result<Option<PLMBOM>> {
        let bom = sqlx::query_as!(
            PLMBOM,
            r#"SELECT id, bom_number, name, description, item_id, version, revision,
                status as "status: _", bom_type, quantity, unit_of_measure, effective_date,
                obsolete_date, created_at, updated_at FROM plm_boms WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(bom)
    }

    async fn list_boms(&self, item_id: Option<Uuid>) -> anyhow::Result<Vec<PLMBOM>> {
        let boms = if let Some(iid) = item_id {
            sqlx::query_as!(
                PLMBOM,
                r#"SELECT id, bom_number, name, description, item_id, version, revision,
                    status as "status: _", bom_type, quantity, unit_of_measure, effective_date,
                    obsolete_date, created_at, updated_at FROM plm_boms WHERE item_id = ?"#,
                iid
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                PLMBOM,
                r#"SELECT id, bom_number, name, description, item_id, version, revision,
                    status as "status: _", bom_type, quantity, unit_of_measure, effective_date,
                    obsolete_date, created_at, updated_at FROM plm_boms ORDER BY created_at DESC"#
            ).fetch_all(&self.pool).await?
        };
        Ok(boms)
    }

    async fn create_bom_line(&self, line: &PLMBOMLine) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_bom_lines (id, bom_id, item_id, line_number, quantity,
                unit_of_measure, find_number, reference_designator, substitute_item_id,
                is_phantom, sort_order, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            line.id, line.bom_id, line.item_id, line.line_number, line.quantity,
            line.unit_of_measure, line.find_number, line.reference_designator,
            line.substitute_item_id, line.is_phantom, line.sort_order, line.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_bom_lines(&self, bom_id: Uuid) -> anyhow::Result<Vec<PLMBOMLine>> {
        let lines = sqlx::query_as!(
            PLMBOMLine,
            r#"SELECT id, bom_id, item_id, line_number, quantity, unit_of_measure, find_number,
                reference_designator, substitute_item_id, is_phantom, sort_order, created_at
                FROM plm_bom_lines WHERE bom_id = ? ORDER BY sort_order"#,
            bom_id
        ).fetch_all(&self.pool).await?;
        Ok(lines)
    }

    async fn create_ecr(&self, ecr: &EngineeringChangeRequest) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_ecrs (id, ecr_number, title, description, reason, priority, status,
                change_type, requested_by, submitted_at, target_date, implemented_date,
                impact_assessment, cost_estimate, currency, approved_by, approved_at,
                rejected_reason, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            ecr.id, ecr.ecr_number, ecr.title, ecr.description, ecr.reason, ecr.priority as _,
            ecr.status as _, ecr.change_type, ecr.requested_by, ecr.submitted_at, ecr.target_date,
            ecr.implemented_date, ecr.impact_assessment, ecr.cost_estimate, ecr.currency,
            ecr.approved_by, ecr.approved_at, ecr.rejected_reason, ecr.created_at, ecr.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_ecr(&self, id: Uuid) -> anyhow::Result<Option<EngineeringChangeRequest>> {
        let ecr = sqlx::query_as!(
            EngineeringChangeRequest,
            r#"SELECT id, ecr_number, title, description, reason, priority as "priority: _",
                status as "status: _", change_type, requested_by, submitted_at, target_date,
                implemented_date, impact_assessment, cost_estimate, currency, approved_by,
                approved_at, rejected_reason, created_at, updated_at FROM plm_ecrs WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(ecr)
    }

    async fn list_ecrs(&self, status: Option<ChangeRequestStatus>) -> anyhow::Result<Vec<EngineeringChangeRequest>> {
        let ecrs = if let Some(s) = status {
            sqlx::query_as!(
                EngineeringChangeRequest,
                r#"SELECT id, ecr_number, title, description, reason, priority as "priority: _",
                    status as "status: _", change_type, requested_by, submitted_at, target_date,
                    implemented_date, impact_assessment, cost_estimate, currency, approved_by,
                    approved_at, rejected_reason, created_at, updated_at
                    FROM plm_ecrs WHERE status = ? ORDER BY created_at DESC"#,
                    s as _
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                EngineeringChangeRequest,
                r#"SELECT id, ecr_number, title, description, reason, priority as "priority: _",
                    status as "status: _", change_type, requested_by, submitted_at, target_date,
                    implemented_date, impact_assessment, cost_estimate, currency, approved_by,
                    approved_at, rejected_reason, created_at, updated_at
                    FROM plm_ecrs ORDER BY created_at DESC"#
            ).fetch_all(&self.pool).await?
        };
        Ok(ecrs)
    }

    async fn update_ecr(&self, ecr: &EngineeringChangeRequest) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE plm_ecrs SET title = ?, description = ?, reason = ?, priority = ?, status = ?,
                change_type = ?, submitted_at = ?, target_date = ?, implemented_date = ?,
                impact_assessment = ?, cost_estimate = ?, approved_by = ?, approved_at = ?,
                rejected_reason = ?, updated_at = ? WHERE id = ?"#,
            ecr.title, ecr.description, ecr.reason, ecr.priority as _, ecr.status as _,
            ecr.change_type, ecr.submitted_at, ecr.target_date, ecr.implemented_date,
            ecr.impact_assessment, ecr.cost_estimate, ecr.approved_by, ecr.approved_at,
            ecr.rejected_reason, ecr.updated_at, ecr.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_ecn(&self, ecn: &EngineeringChangeNotice) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_ecns (id, ecn_number, ecr_id, title, description, status,
                effective_date, implementation_instructions, created_by, approved_by,
                approved_at, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            ecn.id, ecn.ecn_number, ecn.ecr_id, ecn.title, ecn.description, ecn.status as _,
            ecn.effective_date, ecn.implementation_instructions, ecn.created_by, ecn.approved_by,
            ecn.approved_at, ecn.created_at, ecn.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_ecn(&self, id: Uuid) -> anyhow::Result<Option<EngineeringChangeNotice>> {
        let ecn = sqlx::query_as!(
            EngineeringChangeNotice,
            r#"SELECT id, ecn_number, ecr_id, title, description, status as "status: _",
                effective_date, implementation_instructions, created_by, approved_by,
                approved_at, created_at, updated_at FROM plm_ecns WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(ecn)
    }

    async fn list_ecns(&self, ecr_id: Option<Uuid>) -> anyhow::Result<Vec<EngineeringChangeNotice>> {
        let ecns = if let Some(eid) = ecr_id {
            sqlx::query_as!(
                EngineeringChangeNotice,
                r#"SELECT id, ecn_number, ecr_id, title, description, status as "status: _",
                    effective_date, implementation_instructions, created_by, approved_by,
                    approved_at, created_at, updated_at FROM plm_ecns WHERE ecr_id = ?"#,
                eid
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                EngineeringChangeNotice,
                r#"SELECT id, ecn_number, ecr_id, title, description, status as "status: _",
                    effective_date, implementation_instructions, created_by, approved_by,
                    approved_at, created_at, updated_at FROM plm_ecns ORDER BY created_at DESC"#
            ).fetch_all(&self.pool).await?
        };
        Ok(ecns)
    }

    async fn update_ecn(&self, ecn: &EngineeringChangeNotice) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE plm_ecns SET title = ?, description = ?, status = ?, effective_date = ?,
                implementation_instructions = ?, approved_by = ?, approved_at = ?, updated_at = ?
                WHERE id = ?"#,
            ecn.title, ecn.description, ecn.status as _, ecn.effective_date,
            ecn.implementation_instructions, ecn.approved_by, ecn.approved_at, ecn.updated_at, ecn.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_ecn_affected_item(&self, item: &ECNAffectedItem) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_ecn_affected_items (id, ecn_id, item_id, old_revision, new_revision,
                old_version, new_version, change_description, disposition, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            item.id, item.ecn_id, item.item_id, item.old_revision, item.new_revision,
            item.old_version, item.new_version, item.change_description, item.disposition, item.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_ecn_affected_items(&self, ecn_id: Uuid) -> anyhow::Result<Vec<ECNAffectedItem>> {
        let items = sqlx::query_as!(
            ECNAffectedItem,
            r#"SELECT id, ecn_id, item_id, old_revision, new_revision, old_version, new_version,
                change_description, disposition, created_at FROM plm_ecn_affected_items WHERE ecn_id = ?"#,
            ecn_id
        ).fetch_all(&self.pool).await?;
        Ok(items)
    }

    async fn create_workflow(&self, workflow: &PLMWorkflow) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_workflows (id, workflow_number, name, description, workflow_type,
                status, initiated_by, current_step, total_steps, started_at, completed_at,
                created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            workflow.id, workflow.workflow_number, workflow.name, workflow.description,
            workflow.workflow_type, workflow.status, workflow.initiated_by, workflow.current_step,
            workflow.total_steps, workflow.started_at, workflow.completed_at,
            workflow.created_at, workflow.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_workflow(&self, id: Uuid) -> anyhow::Result<Option<PLMWorkflow>> {
        let workflow = sqlx::query_as!(
            PLMWorkflow,
            r#"SELECT id, workflow_number, name, description, workflow_type, status, initiated_by,
                current_step, total_steps, started_at, completed_at, created_at, updated_at
                FROM plm_workflows WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(workflow)
    }

    async fn update_workflow(&self, workflow: &PLMWorkflow) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE plm_workflows SET current_step = ?, status = ?, completed_at = ?, updated_at = ?
                WHERE id = ?"#,
            workflow.current_step, workflow.status, workflow.completed_at, workflow.updated_at, workflow.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_workflow_step(&self, step: &PLMWorkflowStep) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_workflow_steps (id, workflow_id, step_number, step_name, step_type,
                assignee_id, role_id, status, due_date, completed_at, completed_by, comments, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            step.id, step.workflow_id, step.step_number, step.step_name, step.step_type,
            step.assignee_id, step.role_id, step.status, step.due_date, step.completed_at,
            step.completed_by, step.comments, step.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_workflow_steps(&self, workflow_id: Uuid) -> anyhow::Result<Vec<PLMWorkflowStep>> {
        let steps = sqlx::query_as!(
            PLMWorkflowStep,
            r#"SELECT id, workflow_id, step_number, step_name, step_type, assignee_id, role_id,
                status, due_date, completed_at, completed_by, comments, created_at
                FROM plm_workflow_steps WHERE workflow_id = ? ORDER BY step_number"#,
            workflow_id
        ).fetch_all(&self.pool).await?;
        Ok(steps)
    }

    async fn update_workflow_step(&self, step: &PLMWorkflowStep) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE plm_workflow_steps SET status = ?, completed_at = ?, completed_by = ?,
                comments = ? WHERE id = ?"#,
            step.status, step.completed_at, step.completed_by, step.comments, step.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_cad_file(&self, cad: &CADFile) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_cad_files (id, document_id, file_name, file_path, file_size,
                cad_system, format, version, thumbnail_path, geometry_data, metadata, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            cad.id, cad.document_id, cad.file_name, cad.file_path, cad.file_size,
            cad.cad_system, cad.format, cad.version, cad.thumbnail_path, cad.geometry_data,
            cad.metadata, cad.created_at, cad.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_cad_file(&self, id: Uuid) -> anyhow::Result<Option<CADFile>> {
        let cad = sqlx::query_as!(
            CADFile,
            r#"SELECT id, document_id, file_name, file_path, file_size, cad_system, format,
                version, thumbnail_path, geometry_data, metadata, created_at, updated_at
                FROM plm_cad_files WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(cad)
    }

    async fn list_cad_files(&self, document_id: Uuid) -> anyhow::Result<Vec<CADFile>> {
        let files = sqlx::query_as!(
            CADFile,
            r#"SELECT id, document_id, file_name, file_path, file_size, cad_system, format,
                version, thumbnail_path, geometry_data, metadata, created_at, updated_at
                FROM plm_cad_files WHERE document_id = ?"#,
            document_id
        ).fetch_all(&self.pool).await?;
        Ok(files)
    }

    async fn create_specification(&self, spec: &Specification) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_specifications (id, spec_number, name, description, item_id,
                spec_type, status, version, revision, parameters, owner_id, effective_date,
                created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            spec.id, spec.spec_number, spec.name, spec.description, spec.item_id,
            spec.spec_type, spec.status as _, spec.version, spec.revision, spec.parameters,
            spec.owner_id, spec.effective_date, spec.created_at, spec.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_specification(&self, id: Uuid) -> anyhow::Result<Option<Specification>> {
        let spec = sqlx::query_as!(
            Specification,
            r#"SELECT id, spec_number, name, description, item_id, spec_type, status as "status: _",
                version, revision, parameters, owner_id, effective_date, created_at, updated_at
                FROM plm_specifications WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(spec)
    }

    async fn list_specifications(&self, item_id: Option<Uuid>) -> anyhow::Result<Vec<Specification>> {
        let specs = if let Some(iid) = item_id {
            sqlx::query_as!(
                Specification,
                r#"SELECT id, spec_number, name, description, item_id, spec_type, status as "status: _",
                    version, revision, parameters, owner_id, effective_date, created_at, updated_at
                    FROM plm_specifications WHERE item_id = ?"#,
                iid
            ).fetch_all(&self.pool).await?
        } else {
            sqlx::query_as!(
                Specification,
                r#"SELECT id, spec_number, name, description, item_id, spec_type, status as "status: _",
                    version, revision, parameters, owner_id, effective_date, created_at, updated_at
                    FROM plm_specifications ORDER BY created_at DESC"#
            ).fetch_all(&self.pool).await?
        };
        Ok(specs)
    }

    async fn create_spec_parameter(&self, param: &SpecificationParameter) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_spec_parameters (id, spec_id, parameter_name, parameter_type,
                target_value, min_value, max_value, unit, test_method, is_critical, sort_order, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            param.id, param.spec_id, param.parameter_name, param.parameter_type,
            param.target_value, param.min_value, param.max_value, param.unit, param.test_method,
            param.is_critical, param.sort_order, param.created_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_spec_parameters(&self, spec_id: Uuid) -> anyhow::Result<Vec<SpecificationParameter>> {
        let params = sqlx::query_as!(
            SpecificationParameter,
            r#"SELECT id, spec_id, parameter_name, parameter_type, target_value, min_value,
                max_value, unit, test_method, is_critical, sort_order, created_at
                FROM plm_spec_parameters WHERE spec_id = ? ORDER BY sort_order"#,
            spec_id
        ).fetch_all(&self.pool).await?;
        Ok(params)
    }

    async fn create_design_review(&self, review: &DesignReview) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_design_reviews (id, review_number, item_id, review_type, status,
                scheduled_date, conducted_date, facilitator_id, location, outcome, action_items,
                created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            review.id, review.review_number, review.item_id, review.review_type, review.status,
            review.scheduled_date, review.conducted_date, review.facilitator_id, review.location,
            review.outcome, review.action_items, review.created_at, review.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_design_review(&self, id: Uuid) -> anyhow::Result<Option<DesignReview>> {
        let review = sqlx::query_as!(
            DesignReview,
            r#"SELECT id, review_number, item_id, review_type, status, scheduled_date,
                conducted_date, facilitator_id, location, outcome, action_items, created_at, updated_at
                FROM plm_design_reviews WHERE id = ?"#,
            id
        ).fetch_optional(&self.pool).await?;
        Ok(review)
    }

    async fn list_design_reviews(&self, item_id: Uuid) -> anyhow::Result<Vec<DesignReview>> {
        let reviews = sqlx::query_as!(
            DesignReview,
            r#"SELECT id, review_number, item_id, review_type, status, scheduled_date,
                conducted_date, facilitator_id, location, outcome, action_items, created_at, updated_at
                FROM plm_design_reviews WHERE item_id = ? ORDER BY scheduled_date DESC"#,
            item_id
        ).fetch_all(&self.pool).await?;
        Ok(reviews)
    }

    async fn update_design_review(&self, review: &DesignReview) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE plm_design_reviews SET status = ?, conducted_date = ?, outcome = ?,
                action_items = ?, updated_at = ? WHERE id = ?"#,
            review.status, review.conducted_date, review.outcome, review.action_items,
            review.updated_at, review.id
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn create_compliance_requirement(&self, req: &ComplianceRequirement) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_compliance_requirements (id, requirement_code, name, description,
                regulation, category, mandatory, verification_method, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            req.id, req.requirement_code, req.name, req.description, req.regulation,
            req.category, req.mandatory, req.verification_method, req.created_at, req.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_compliance_requirements(&self) -> anyhow::Result<Vec<ComplianceRequirement>> {
        let reqs = sqlx::query_as!(
            ComplianceRequirement,
            r#"SELECT id, requirement_code, name, description, regulation, category, mandatory,
                verification_method, created_at, updated_at FROM plm_compliance_requirements
                ORDER BY requirement_code"#
        ).fetch_all(&self.pool).await?;
        Ok(reqs)
    }

    async fn create_item_compliance(&self, compliance: &ItemCompliance) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO plm_item_compliances (id, item_id, requirement_id, status, certified,
                certification_date, certification_expiry, certifying_body, certificate_number,
                notes, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            compliance.id, compliance.item_id, compliance.requirement_id, compliance.status,
            compliance.certified, compliance.certification_date, compliance.certification_expiry,
            compliance.certifying_body, compliance.certificate_number, compliance.notes,
            compliance.created_at, compliance.updated_at
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn list_item_compliances(&self, item_id: Uuid) -> anyhow::Result<Vec<ItemCompliance>> {
        let compliances = sqlx::query_as!(
            ItemCompliance,
            r#"SELECT id, item_id, requirement_id, status, certified, certification_date,
                certification_expiry, certifying_body, certificate_number, notes, created_at, updated_at
                FROM plm_item_compliances WHERE item_id = ?"#,
            item_id
        ).fetch_all(&self.pool).await?;
        Ok(compliances)
    }
}
