#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::models::*;
    use crate::repository::*;
    use crate::service::WarrantyService;
    use uuid::Uuid;
    use async_trait::async_trait;

    struct MockWarrantyRepository;

    #[async_trait]
    impl WarrantyRepository for MockWarrantyRepository {
        async fn create_policy(&self, _: &WarrantyPolicy) -> Result<(), sqlx::Error> { Ok(()) }
        async fn get_policy(&self, _: Uuid) -> Result<Option<WarrantyPolicy>, sqlx::Error> { Ok(None) }
        async fn list_policies(&self, _: Option<String>) -> Result<Vec<WarrantyPolicy>, sqlx::Error> { Ok(vec![]) }
        async fn update_policy(&self, _: &WarrantyPolicy) -> Result<(), sqlx::Error> { Ok(()) }
        async fn delete_policy(&self, _: Uuid) -> Result<(), sqlx::Error> { Ok(()) }
        async fn create_product_warranty(&self, _: &ProductWarranty) -> Result<(), sqlx::Error> { Ok(()) }
        async fn get_product_warranty(&self, _: Uuid) -> Result<Option<ProductWarranty>, sqlx::Error> { Ok(None) }
        async fn get_product_warranty_by_number(&self, _: &str) -> Result<Option<ProductWarranty>, sqlx::Error> { Ok(None) }
        async fn list_product_warranties(&self, _: Option<Uuid>, _: Option<Uuid>, _: Option<String>) -> Result<Vec<ProductWarranty>, sqlx::Error> {
            Ok(vec![])
        }
        async fn list_expiring_warranties(&self, _: i32) -> Result<Vec<ProductWarranty>, sqlx::Error> { Ok(vec![]) }
        async fn update_product_warranty(&self, _: &ProductWarranty) -> Result<(), sqlx::Error> { Ok(()) }
        async fn create_claim(&self, _: &WarrantyClaim) -> Result<(), sqlx::Error> { Ok(()) }
        async fn get_claim(&self, _: Uuid) -> Result<Option<WarrantyClaim>, sqlx::Error> { Ok(None) }
        async fn get_claim_by_number(&self, _: &str) -> Result<Option<WarrantyClaim>, sqlx::Error> { Ok(None) }
        async fn list_claims(&self, _: Option<Uuid>, _: Option<Uuid>, _: Option<WarrantyClaimStatus>) -> Result<Vec<WarrantyClaim>, sqlx::Error> { Ok(vec![]) }
        async fn update_claim(&self, _: &WarrantyClaim) -> Result<(), sqlx::Error> { Ok(()) }
        async fn create_claim_line(&self, _: &WarrantyClaimLine) -> Result<(), sqlx::Error> { Ok(()) }
        async fn list_claim_lines(&self, _: Uuid) -> Result<Vec<WarrantyClaimLine>, sqlx::Error> { Ok(vec![]) }
        async fn delete_claim_lines(&self, _: Uuid) -> Result<(), sqlx::Error> { Ok(()) }
        async fn create_claim_labor(&self, _: &WarrantyClaimLabor) -> Result<(), sqlx::Error> { Ok(()) }
        async fn list_claim_labors(&self, _: Uuid) -> Result<Vec<WarrantyClaimLabor>, sqlx::Error> { Ok(vec![]) }
        async fn delete_claim_labors(&self, _: Uuid) -> Result<(), sqlx::Error> { Ok(()) }
        async fn create_extension(&self, _: &WarrantyExtension) -> Result<(), sqlx::Error> { Ok(()) }
        async fn list_extensions(&self, _: Uuid) -> Result<Vec<WarrantyExtension>, sqlx::Error> { Ok(vec![]) }
        async fn get_analytics(&self) -> Result<WarrantyAnalytics, sqlx::Error> {
            Ok(WarrantyAnalytics {
                total_warranties: 0, active_warranties: 0, expired_warranties: 0,
                total_claims: 0, open_claims: 0, approved_claims: 0, rejected_claims: 0,
                total_claim_cost: 0, average_resolution_days: 0.0,
                claims_by_category: serde_json::json!({}), claims_by_month: serde_json::json!({}),
            })
        }
        async fn create_recall(&self, _: &ProductRecall) -> Result<(), sqlx::Error> { Ok(()) }
        async fn get_recall(&self, _: Uuid) -> Result<Option<ProductRecall>, sqlx::Error> { Ok(None) }
        async fn list_recalls(&self, _: Option<RecallStatus>) -> Result<Vec<ProductRecall>, sqlx::Error> { Ok(vec![]) }
        async fn update_recall(&self, _: &ProductRecall) -> Result<(), sqlx::Error> { Ok(()) }
        async fn create_recall_item(&self, _: &RecallAffectedItem) -> Result<(), sqlx::Error> { Ok(()) }
        async fn list_recall_items(&self, _: Uuid) -> Result<Vec<RecallAffectedItem>, sqlx::Error> { Ok(vec![]) }
        async fn update_recall_item(&self, _: &RecallAffectedItem) -> Result<(), sqlx::Error> { Ok(()) }
        async fn create_recall_notification(&self, _: &RecallNotification) -> Result<(), sqlx::Error> { Ok(()) }
        async fn list_recall_notifications(&self, _: Uuid) -> Result<Vec<RecallNotification>, sqlx::Error> { Ok(vec![]) }
    }

    #[tokio::test]
    async fn test_create_recall() -> Result<(), String> {
        let service = WarrantyService::new(MockWarrantyRepository);
        let product_id = Uuid::new_v4();
        let req = CreateRecallRequest {
            title: "Safety Recall SR-2026-01".to_string(),
            description: "Faulty battery in smart widget".to_string(),
            reason: "Overheating risk".to_string(),
            severity: RecallSeverity::Critical,
            product_id,
            affected_lots: Some("LOT-A, LOT-B".to_string()),
        };
        let recall = service.create_recall(req).await?;
        assert_eq!(recall.title, "Safety Recall SR-2026-01");
        assert_eq!(recall.status, RecallStatus::Draft);
        assert_eq!(recall.product_id, product_id);
        Ok(())
    }
}
