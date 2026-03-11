use erp_manufacturing::BillOfMaterial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentChangeType {
    Added,
    Deleted,
    QuantityChanged,
    Unchanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDiff {
    pub product_id: Uuid,
    pub change_type: ComponentChangeType,
    pub old_quantity: Option<i64>,
    pub new_quantity: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomDiffReport {
    pub source_bom_id: Uuid,
    pub target_bom_id: Uuid,
    pub component_changes: Vec<ComponentDiff>,
}

pub struct BomDiffEngine;

impl BomDiffEngine {
    pub fn compare(source: &BillOfMaterial, target: &BillOfMaterial) -> BomDiffReport {
        let mut changes = Vec::new();

        // 1. Identify Deletions and Quantity Changes
        for s_comp in &source.components {
            let t_comp = target.components.iter().find(|c| c.product_id == s_comp.product_id);
            
            match t_comp {
                Some(tc) => {
                    if tc.quantity != s_comp.quantity {
                        changes.push(ComponentDiff {
                            product_id: s_comp.product_id,
                            change_type: ComponentChangeType::QuantityChanged,
                            old_quantity: Some(s_comp.quantity),
                            new_quantity: Some(tc.quantity),
                        });
                    } else {
                        changes.push(ComponentDiff {
                            product_id: s_comp.product_id,
                            change_type: ComponentChangeType::Unchanged,
                            old_quantity: Some(s_comp.quantity),
                            new_quantity: Some(tc.quantity),
                        });
                    }
                }
                None => {
                    changes.push(ComponentDiff {
                        product_id: s_comp.product_id,
                        change_type: ComponentChangeType::Deleted,
                        old_quantity: Some(s_comp.quantity),
                        new_quantity: None,
                    });
                }
            }
        }

        // 2. Identify Additions
        for t_comp in &target.components {
            if !source.components.iter().any(|c| c.product_id == t_comp.product_id) {
                changes.push(ComponentDiff {
                    product_id: t_comp.product_id,
                    change_type: ComponentChangeType::Added,
                    old_quantity: None,
                    new_quantity: Some(t_comp.quantity),
                });
            }
        }

        BomDiffReport {
            source_bom_id: source.base.id,
            target_bom_id: target.base.id,
            component_changes: changes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use erp_core::{BaseEntity, Status};
    use erp_manufacturing::BomComponent;
    use chrono::Utc;

    fn create_mock_bom(id: Uuid, components: Vec<BomComponent>) -> BillOfMaterial {
        BillOfMaterial {
            base: BaseEntity {
                id,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            product_id: Uuid::new_v4(),
            name: "Test BOM".to_string(),
            version: "1.0".to_string(),
            quantity: 1,
            components,
            operations: Vec::new(),
            status: Status::Active,
        }
    }

    #[test]
    fn test_bom_comparison() {
        let p1 = Uuid::new_v4();
        let p2 = Uuid::new_v4();
        let p3 = Uuid::new_v4();

        let bom1 = create_mock_bom(Uuid::new_v4(), vec![
            BomComponent { id: Uuid::new_v4(), product_id: p1, quantity: 10, unit: "EA".to_string(), scrap_percent: 0.0 },
            BomComponent { id: Uuid::new_v4(), product_id: p2, quantity: 5, unit: "EA".to_string(), scrap_percent: 0.0 },
        ]);

        let bom2 = create_mock_bom(Uuid::new_v4(), vec![
            BomComponent { id: Uuid::new_v4(), product_id: p1, quantity: 12, unit: "EA".to_string(), scrap_percent: 0.0 }, // Changed qty
            BomComponent { id: Uuid::new_v4(), product_id: p3, quantity: 1, unit: "EA".to_string(), scrap_percent: 0.0 },  // Added
        ]);

        let report = BomDiffEngine::compare(&bom1, &bom2);

        // p1: QuantityChanged
        let c1 = report.component_changes.iter().find(|c| c.product_id == p1).unwrap();
        assert_eq!(c1.change_type, ComponentChangeType::QuantityChanged);
        assert_eq!(c1.old_quantity, Some(10));
        assert_eq!(c1.new_quantity, Some(12));

        // p2: Deleted
        let c2 = report.component_changes.iter().find(|c| c.product_id == p2).unwrap();
        assert_eq!(c2.change_type, ComponentChangeType::Deleted);

        // p3: Added
        let c3 = report.component_changes.iter().find(|c| c.product_id == p3).unwrap();
        assert_eq!(c3.change_type, ComponentChangeType::Added);
        assert_eq!(c3.new_quantity, Some(1));
    }
}
