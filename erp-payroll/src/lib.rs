use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PayrollRunStatus {
    Draft,
    Processing,
    Review,
    Approved,
    Paid,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PayItemType {
    Earning,
    Deduction,
    Tax,
    Reimbursement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayItem {
    pub id: Uuid,
    pub name: String,
    pub item_type: PayItemType,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payslip {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub payroll_run_id: Uuid,
    pub items: Vec<PayItem>,
    pub net_pay: f64,
    pub gross_pay: f64,
    pub total_deductions: f64,
    pub total_taxes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayrollRun {
    pub id: Uuid,
    pub pay_period_start: NaiveDate,
    pub pay_period_end: NaiveDate,
    pub payment_date: NaiveDate,
    pub status: PayrollRunStatus,
    pub payslips: Vec<Payslip>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Payslip {
    pub fn new(employee_id: Uuid, payroll_run_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id,
            payroll_run_id,
            items: Vec::new(),
            net_pay: 0.0,
            gross_pay: 0.0,
            total_deductions: 0.0,
            total_taxes: 0.0,
        }
    }

    pub fn add_item(&mut self, item: PayItem) {
        match item.item_type {
            PayItemType::Earning | PayItemType::Reimbursement => {
                self.gross_pay += item.amount;
            }
            PayItemType::Deduction => {
                self.total_deductions += item.amount;
            }
            PayItemType::Tax => {
                self.total_taxes += item.amount;
            }
        }
        self.items.push(item);
        self.calculate_net_pay();
    }

    fn calculate_net_pay(&mut self) {
        self.net_pay = self.gross_pay - self.total_deductions - self.total_taxes;
    }
}

impl PayrollRun {
    pub fn new(
        pay_period_start: NaiveDate,
        pay_period_end: NaiveDate,
        payment_date: NaiveDate,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            pay_period_start,
            pay_period_end,
            payment_date,
            status: PayrollRunStatus::Draft,
            payslips: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_payslip(&mut self, payslip: Payslip) {
        self.payslips.push(payslip);
        self.updated_at = Utc::now();
    }

    pub fn approve(&mut self) {
        if self.status == PayrollRunStatus::Review || self.status == PayrollRunStatus::Draft {
            self.status = PayrollRunStatus::Approved;
            self.updated_at = Utc::now();
        }
    }

    pub fn mark_paid(&mut self) {
        if self.status == PayrollRunStatus::Approved {
            self.status = PayrollRunStatus::Paid;
            self.updated_at = Utc::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payslip_calculations() {
        let employee_id = Uuid::new_v4();
        let payroll_run_id = Uuid::new_v4();
        
        let mut payslip = Payslip::new(employee_id, payroll_run_id);
        
        payslip.add_item(PayItem {
            id: Uuid::new_v4(),
            name: "Base Salary".to_string(),
            item_type: PayItemType::Earning,
            amount: 5000.0,
        });

        payslip.add_item(PayItem {
            id: Uuid::new_v4(),
            name: "Health Insurance".to_string(),
            item_type: PayItemType::Deduction,
            amount: 200.0,
        });

        payslip.add_item(PayItem {
            id: Uuid::new_v4(),
            name: "Income Tax".to_string(),
            item_type: PayItemType::Tax,
            amount: 1000.0,
        });
        
        payslip.add_item(PayItem {
            id: Uuid::new_v4(),
            name: "Internet Stipend".to_string(),
            item_type: PayItemType::Reimbursement,
            amount: 50.0,
        });

        assert_eq!(payslip.gross_pay, 5050.0);
        assert_eq!(payslip.total_deductions, 200.0);
        assert_eq!(payslip.total_taxes, 1000.0);
        assert_eq!(payslip.net_pay, 3850.0);
    }

    #[test]
    fn test_payroll_run_lifecycle() {
        let start = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 3, 15).unwrap();
        let pay_date = NaiveDate::from_ymd_opt(2026, 3, 20).unwrap();
        
        let mut run = PayrollRun::new(start, end, pay_date);
        assert_eq!(run.status, PayrollRunStatus::Draft);

        let employee_id = Uuid::new_v4();
        let mut payslip = Payslip::new(employee_id, run.id);
        payslip.add_item(PayItem {
            id: Uuid::new_v4(),
            name: "Hourly Wages".to_string(),
            item_type: PayItemType::Earning,
            amount: 1500.0,
        });
        
        run.add_payslip(payslip);
        assert_eq!(run.payslips.len(), 1);
        
        run.approve();
        assert_eq!(run.status, PayrollRunStatus::Approved);
        
        run.mark_paid();
        assert_eq!(run.status, PayrollRunStatus::Paid);
    }
}
