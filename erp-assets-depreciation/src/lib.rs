use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DepreciationMethod {
    StraightLine,
    DoubleDecliningBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedAsset {
    pub id: Uuid,
    pub name: String,
    pub purchase_price: f64,
    pub salvage_value: f64,
    pub useful_life_years: u32,
    pub purchase_date: DateTime<Utc>,
    pub method: DepreciationMethod,
}

impl FixedAsset {
    pub fn new(
        name: String,
        purchase_price: f64,
        salvage_value: f64,
        useful_life_years: u32,
        purchase_date: DateTime<Utc>,
        method: DepreciationMethod,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            purchase_price,
            salvage_value,
            useful_life_years,
            purchase_date,
            method,
        }
    }

    /// Calculates the depreciation expense for a specific year of the asset's life.
    /// `year` is 1-indexed (e.g., year 1, year 2...).
    pub fn calculate_depreciation_for_year(&self, year: u32) -> Result<f64, &'static str> {
        if year == 0 || year > self.useful_life_years {
            return Err("Invalid year for depreciation calculation");
        }

        match self.method {
            DepreciationMethod::StraightLine => {
                let total_depreciable_base = self.purchase_price - self.salvage_value;
                Ok(total_depreciable_base / self.useful_life_years as f64)
            }
            DepreciationMethod::DoubleDecliningBalance => {
                let depreciation_rate = (1.0 / self.useful_life_years as f64) * 2.0;
                let mut current_book_value = self.purchase_price;

                for y in 1..=year {
                    let expense = current_book_value * depreciation_rate;
                    
                    // Asset cannot be depreciated below salvage value
                    if current_book_value - expense < self.salvage_value {
                        if y == year {
                            return Ok(current_book_value - self.salvage_value);
                        }
                        current_book_value = self.salvage_value;
                    } else {
                        if y == year {
                            return Ok(expense);
                        }
                        current_book_value -= expense;
                    }
                }
                Ok(0.0)
            }
        }
    }

    pub fn get_book_value_after_years(&self, years: u32) -> f64 {
        let mut book_value = self.purchase_price;
        for y in 1..=years {
            if let Ok(expense) = self.calculate_depreciation_for_year(y) {
                book_value -= expense;
            }
        }
        book_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_straight_line_depreciation() {
        let purchase_date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let asset = FixedAsset::new(
            "Company Van".to_string(),
            30000.0,
            5000.0,
            5,
            purchase_date,
            DepreciationMethod::StraightLine,
        );

        let annual_expense = asset.calculate_depreciation_for_year(1).unwrap();
        assert_eq!(annual_expense, 5000.0); // (30000 - 5000) / 5
        
        assert_eq!(asset.get_book_value_after_years(5), 5000.0);
    }

    #[test]
    fn test_double_declining_depreciation() {
        let purchase_date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let asset = FixedAsset::new(
            "Server Rack".to_string(),
            10000.0,
            1000.0,
            4,
            purchase_date,
            DepreciationMethod::DoubleDecliningBalance,
        );

        // Rate is (1/4)*2 = 50%
        let year_1 = asset.calculate_depreciation_for_year(1).unwrap();
        assert_eq!(year_1, 5000.0);

        let year_2 = asset.calculate_depreciation_for_year(2).unwrap();
        assert_eq!(year_2, 2500.0); // 50% of remaining 5000

        let year_3 = asset.calculate_depreciation_for_year(3).unwrap();
        assert_eq!(year_3, 1250.0); // 50% of remaining 2500

        // Year 4 should only depreciate down to salvage value (1000)
        let year_4 = asset.calculate_depreciation_for_year(4).unwrap();
        assert_eq!(year_4, 250.0); // 1250 - 1000
        
        assert_eq!(asset.get_book_value_after_years(4), 1000.0);
    }
}
