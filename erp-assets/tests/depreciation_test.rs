use erp_assets::models::{AssetDepreciation, DepreciationMethod};
use erp_assets::service::AssetDepreciationService;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_straight_line_depreciation() {
    let service = AssetDepreciationService::new();
    
    let dep = AssetDepreciation {
        id: Uuid::new_v4(),
        asset_id: Uuid::new_v4(),
        depreciation_method: DepreciationMethod::StraightLine,
        useful_life_months: 60, // 5 years
        salvage_value: 1000,
        current_value: 6000,
        accumulated_depreciation: 0,
        last_depreciation_date: None,
        currency: "USD".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // total depreciable = (6000 + 0) - 1000 = 5000
    // monthly = 5000 / 60 = 83.33 -> 83
    
    // 12 months = 83 * 12 = 996
    let amount = service.calculate_depreciation_amount_test(&dep, 12);
    assert_eq!(amount, 1000); // 5000 / 60 = 83.33. (5000 * 12) / 60 = 1000.
    // My implementation: (total_depreciable / useful_life) * months
    // (5000 / 60) * 12 = 83 * 12 = 996.
}

#[tokio::test]
async fn test_declining_balance_depreciation() {
    let service = AssetDepreciationService::new();
    
    let dep = AssetDepreciation {
        id: Uuid::new_v4(),
        asset_id: Uuid::new_v4(),
        depreciation_method: DepreciationMethod::DecliningBalance,
        useful_life_months: 60,
        salvage_value: 1000,
        current_value: 6000,
        accumulated_depreciation: 0,
        last_depreciation_date: None,
        currency: "USD".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Factor = 2 / 60 = 0.0333
    // Month 1: 6000 * 0.0333 = 200
    // Month 2: (6000 - 200) * 0.0333 = 5800 * 0.0333 = 193.33 -> 193
    
    let amount = service.calculate_depreciation_amount_test(&dep, 2);
    assert!(amount > 390 && amount < 400); 
}

// Since calculate_depreciation_amount is private, I might need to make it public for testing 
// or test via run_depreciation which needs a pool.
// I'll update service.rs to make it public.
