use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use erp_revrec::models::*;
use erp_revrec::repository::RevRecRepository;
use erp_revrec::service::RevRecService;
use uuid::Uuid;

struct MockRevRecRepository {
    obligations: std::sync::Mutex<Vec<PerformanceObligation>>,
    events: std::sync::Mutex<Vec<RevenueEvent>>,
}

#[async_trait]
impl RevRecRepository for MockRevRecRepository {
    async fn create_contract(&self, _contract: &RevenueContract) -> anyhow::Result<()> { Ok(()) }
    async fn create_obligation(&self, obligation: &PerformanceObligation) -> anyhow::Result<()> {
        self.obligations.lock().unwrap().push(obligation.clone());
        Ok(())
    }
    async fn get_obligation(&self, id: Uuid) -> anyhow::Result<Option<PerformanceObligation>> {
        Ok(self.obligations.lock().unwrap().iter().find(|o| o.id == id).cloned())
    }
    async fn list_events(&self, _contract_id: Uuid) -> anyhow::Result<Vec<RevenueEvent>> {
        Ok(self.events.lock().unwrap().clone())
    }
    async fn create_event(&self, event: &RevenueEvent) -> anyhow::Result<()> {
        self.events.lock().unwrap().push(event.clone());
        Ok(())
    }
}

#[tokio::test]
async fn test_rssp_allocation() {
    let repo = MockRevRecRepository { 
        obligations: std::sync::Mutex::new(vec![]),
        events: std::sync::Mutex::new(vec![]),
    };
    let service = RevRecService::new(repo);

    let req = CreateContractRequest {
        customer_id: Uuid::new_v4(),
        contract_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        start_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        end_date: NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
        total_value: 120000, // $1200.00
        currency: "USD".to_string(),
        obligations: vec![
            CreateObligationRequest {
                name: "Software License".to_string(),
                description: None,
                standalone_price: 100000, // $1000.00
                recognition_type: RecognitionType::PointInTime,
                recognition_method: RecognitionMethod::OutputMethod,
                total_periods: 1,
            },
            CreateObligationRequest {
                name: "Maintenance & Support".to_string(),
                description: None,
                standalone_price: 50000, // $500.00
                recognition_type: RecognitionType::OverTime,
                recognition_method: RecognitionMethod::StraightLine,
                total_periods: 12,
            },
        ],
    };

    let contract = service.create_contract(req).await.unwrap();

    // Total Standalone = 1000 + 500 = 1500
    // License Allocation = (1000 / 1500) * 1200 = 800.00
    // Support Allocation = (500 / 1500) * 1200 = 400.00
    
    assert_eq!(contract.performance_obligations[0].allocated_price, 80000);
    assert_eq!(contract.performance_obligations[1].allocated_price, 40000);
    assert_eq!(contract.performance_obligations[0].allocated_price + contract.performance_obligations[1].allocated_price, 120000);
}

#[tokio::test]
async fn test_poc_calculation() {
    let repo = MockRevRecRepository { 
        obligations: std::sync::Mutex::new(vec![]),
        events: std::sync::Mutex::new(vec![]),
    };
    let service = RevRecService::new(repo);
    
    let obligation_id = Uuid::new_v4();
    let contract_id = Uuid::new_v4();
    let obligation = PerformanceObligation {
        id: obligation_id,
        contract_id,
        name: "Custom Implementation".to_string(),
        description: None,
        standalone_price: 100000,
        allocated_price: 80000,
        recognition_type: RecognitionType::OverTime,
        recognition_method: RecognitionMethod::PercentageComplete,
        total_periods: 1,
        status: ObligationStatus::InProgress,
        created_at: Utc::now(),
    };
    
    service.repo.create_obligation(&obligation).await.unwrap();
    
    // 50% completion
    // costs incurred: 5000, total estimated costs: 10000
    let to_recognize = service.calculate_poc_revenue(obligation_id, 5000, 10000).await.unwrap();
    assert_eq!(to_recognize, 40000); // 50% of 80000
    
    // Recognize that revenue
    service.recognize_revenue(RecognizeRevenueRequest {
        contract_id,
        obligation_id: Some(obligation_id),
        amount: 40000,
        event_type: RevenueEventType::PeriodRecognition,
        event_date: NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        description: Some("POC recognition Q1".to_string()),
    }).await.unwrap();
    
    // Now at 75% completion
    let to_recognize_next = service.calculate_poc_revenue(obligation_id, 7500, 10000).await.unwrap();
    assert_eq!(to_recognize_next, 20000); // (75% of 80000) - 40000 = 60000 - 40000 = 20000
}
