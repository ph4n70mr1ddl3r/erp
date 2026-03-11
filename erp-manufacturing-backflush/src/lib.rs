use erp_manufacturing::BillOfMaterial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackflushIssue {
    pub product_id: Uuid,
    pub quantity_to_issue: f64,
    pub unit: String,
    pub source_bom_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackflushResult {
    pub finished_good_id: Uuid,
    pub quantity_produced: f64,
    pub material_issues: Vec<BackflushIssue>,
}

pub struct BackflushEngine;

impl BackflushEngine {
    /// Calculates the materials to be back-flushed based on produced quantity and BOM.
    pub fn calculate_issues(
        bom: &BillOfMaterial,
        quantity_produced: f64,
    ) -> BackflushResult {
        let mut issues = Vec::new();

        for component in &bom.components {
            // Formula: Component Qty * Produced Qty * (1 + Scrap%)
            // Assuming quantity in BOM is based on 1 unit of FG (bom.quantity)
            let base_qty_per_unit = component.quantity as f64 / bom.quantity as f64;
            let total_requirement = base_qty_per_unit * quantity_produced;
            let scrap_adjustment = 1.0 + (component.scrap_percent / 100.0);
            
            let final_issue_qty = total_requirement * scrap_adjustment;

            issues.push(BackflushIssue {
                product_id: component.product_id,
                quantity_to_issue: final_issue_qty,
                unit: component.unit.clone(),
                source_bom_id: bom.base.id,
            });
        }

        BackflushResult {
            finished_good_id: bom.product_id,
            quantity_produced,
            material_issues: issues,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use erp_core::{BaseEntity, Status};
    use erp_manufacturing::BomComponent;
    use chrono::Utc;

    fn create_mock_bom(product_id: Uuid, components: Vec<BomComponent>) -> BillOfMaterial {
        BillOfMaterial {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            product_id,
            name: "Table BOM".to_string(),
            version: "1.0".to_string(),
            quantity: 1, // BOM for 1 unit
            components,
            operations: Vec::new(),
            status: Status::Active,
        }
    }

    #[test]
    fn test_backflush_calculation() {
        let table_id = Uuid::new_v4();
        let wood_id = Uuid::new_v4();
        let screw_id = Uuid::new_v4();

        let bom = create_mock_bom(table_id, vec![
            // 4 legs per table, 0% scrap
            BomComponent {
                id: Uuid::new_v4(),
                product_id: wood_id,
                quantity: 4,
                unit: "EA".to_string(),
                scrap_percent: 0.0,
            },
            // 10 screws per table, 10% scrap (maybe some get lost)
            BomComponent {
                id: Uuid::new_v4(),
                product_id: screw_id,
                quantity: 10,
                unit: "EA".to_string(),
                scrap_percent: 10.0,
            },
        ]);

        // Produced 5 tables
        let result = BackflushEngine::calculate_issues(&bom, 5.0);

        assert_eq!(result.material_issues.len(), 2);

        // Wood: 4 * 5 = 20
        let wood_issue = result.material_issues.iter().find(|i| i.product_id == wood_id).unwrap();
        assert!((wood_issue.quantity_to_issue - 20.0).abs() < 1e-10);

        // Screws: 10 * 5 = 50. With 10% scrap -> 55.
        let screw_issue = result.material_issues.iter().find(|i| i.product_id == screw_id).unwrap();
        assert!((screw_issue.quantity_to_issue - 55.0).abs() < 1e-10);
    }
}
