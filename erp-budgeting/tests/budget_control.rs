use erp_budgeting::{BudgetService, BudgetLine, Budget, BudgetEnforcementLevel, BudgetStatus, BudgetType, BudgetAvailabilityStatus};
use erp_core::models::{BaseEntity, Money, Currency};
use chrono::Utc;
use uuid::Uuid;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::test]
async fn test_budget_availability_hard_enforcement() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    let service = BudgetService::new(pool);

    let budget = Budget {
        base: BaseEntity::new(),
        name: "Test Budget".to_string(),
        code: "TEST-001".to_string(),
        description: None,
        budget_type: BudgetType::Operating,
        status: BudgetStatus::Active,
        enforcement_level: BudgetEnforcementLevel::Hard,
        fiscal_year: 2026,
        start_date: Utc::now(),
        end_date: Utc::now(),
        total_amount: Money::new(1000, Currency::USD),
        currency: "USD".to_string(),
        department_id: None,
        project_id: None,
        owner_id: Uuid::new_v4(),
        approval_workflow_id: None,
        version: 1,
        parent_budget_id: None,
        is_template: false,
    };

    let mut line = BudgetLine {
        base: BaseEntity::new(),
        budget_id: budget.base.id,
        account_id: Uuid::new_v4(),
        account_code: "6000".to_string(),
        account_name: "Office Supplies".to_string(),
        description: None,
        planned_amount: Money::new(500, Currency::USD),
        committed_amount: Money::new(0, Currency::USD),
        actual_amount: Money::new(0, Currency::USD),
        variance_amount: Money::new(500, Currency::USD),
        variance_percent: 100.0,
        period_start: Utc::now(),
        period_end: Utc::now(),
        cost_center_id: None,
        notes: None,
    };

    // 1. Check availability for 400 (should be available)
    let res = service.check_budget_availability(&line, &budget, 400).await.unwrap();
    assert!(res.is_available);
    assert_eq!(res.status, BudgetAvailabilityStatus::Success);

    // 2. Commit 400
    service.commit_funds(&mut line, &budget, 400).await.unwrap();
    assert_eq!(line.committed_amount.amount, 400);

    // 3. Check availability for 200 (should NOT be available because 500-400=100)
    let res = service.check_budget_availability(&line, &budget, 200).await.unwrap();
    assert!(!res.is_available);
    assert_eq!(res.status, BudgetAvailabilityStatus::Blocked);

    // 4. Attempt to commit 200 (should fail with Error)
    let err = service.commit_funds(&mut line, &budget, 200).await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_budget_availability_advisory_enforcement() {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    let service = BudgetService::new(pool);

    let budget = Budget {
        base: BaseEntity::new(),
        name: "Test Budget".to_string(),
        code: "TEST-002".to_string(),
        description: None,
        budget_type: BudgetType::Operating,
        status: BudgetStatus::Active,
        enforcement_level: BudgetEnforcementLevel::Advisory,
        fiscal_year: 2026,
        start_date: Utc::now(),
        end_date: Utc::now(),
        total_amount: Money::new(1000, Currency::USD),
        currency: "USD".to_string(),
        department_id: None,
        project_id: None,
        owner_id: Uuid::new_v4(),
        approval_workflow_id: None,
        version: 1,
        parent_budget_id: None,
        is_template: false,
    };

    let mut line = BudgetLine {
        base: BaseEntity::new(),
        budget_id: budget.base.id,
        account_id: Uuid::new_v4(),
        account_code: "6000".to_string(),
        account_name: "Office Supplies".to_string(),
        description: None,
        planned_amount: Money::new(500, Currency::USD),
        committed_amount: Money::new(0, Currency::USD),
        actual_amount: Money::new(0, Currency::USD),
        variance_amount: Money::new(500, Currency::USD),
        variance_percent: 100.0,
        period_start: Utc::now(),
        period_end: Utc::now(),
        cost_center_id: None,
        notes: None,
    };

    // Check availability for 600 (should be available but with Warning)
    let res = service.check_budget_availability(&line, &budget, 600).await.unwrap();
    assert!(res.is_available);
    assert_eq!(res.status, BudgetAvailabilityStatus::Warning);

    // Should allow committing despite being over budget
    service.commit_funds(&mut line, &budget, 600).await.unwrap();
    assert_eq!(line.committed_amount.amount, 600);
}
