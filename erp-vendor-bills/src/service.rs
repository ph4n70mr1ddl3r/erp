use chrono::Utc;
use erp_core::{BaseEntity, Currency, Error, Money, Paginated, Pagination, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{
    MatchException, MatchExceptionType, MatchStatus, ThreeWayMatchResult, VendorBill,
    VendorBillLine, VendorBillLineCreateRequest, VendorBillPayment, VendorBillStatus,
};
use crate::repository::VendorBillRepository;

pub struct VendorBillService;

impl VendorBillService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<VendorBill> {
        VendorBillRepository::find_by_id(pool, id)
            .await
            .map_err(Error::Internal)
    }

    pub async fn list(pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<VendorBill>> {
        VendorBillRepository::find_all(pool, pagination)
            .await
            .map_err(Error::Internal)
    }

    pub async fn list_by_vendor(pool: &SqlitePool, vendor_id: Uuid) -> Result<Vec<VendorBill>> {
        VendorBillRepository::find_by_vendor(pool, vendor_id)
            .await
            .map_err(Error::Internal)
    }

    pub async fn create(
        pool: &SqlitePool,
        vendor_id: Uuid,
        vendor_invoice_number: String,
        purchase_order_id: Option<Uuid>,
        bill_date: chrono::DateTime<Utc>,
        due_date: chrono::DateTime<Utc>,
        line_requests: Vec<VendorBillLineCreateRequest>,
        notes: Option<String>,
    ) -> Result<VendorBill> {
        if vendor_invoice_number.is_empty() {
            return Err(Error::validation("Vendor invoice number is required"));
        }
        if line_requests.is_empty() {
            return Err(Error::validation("Vendor bill must have at least one line"));
        }
        if due_date < bill_date {
            return Err(Error::validation("Due date cannot be before bill date"));
        }

        let base = BaseEntity::new();
        let bill_number = format!("VB-{}", Utc::now().format("%Y%m%d%H%M%S"));

        let lines: Vec<VendorBillLine> = line_requests
            .into_iter()
            .map(|req| {
                let line_total = Money::new(
                    ((req.quantity as f64) * (req.unit_price as f64) * (1.0 + req.tax_rate / 100.0)) as i64,
                    Currency::USD,
                );
                VendorBillLine {
                    id: Uuid::new_v4(),
                    bill_id: base.id,
                    po_line_id: req.po_line_id,
                    product_id: req.product_id,
                    description: req.description,
                    quantity: req.quantity,
                    unit_price: Money::new(req.unit_price, Currency::USD),
                    tax_rate: req.tax_rate,
                    line_total,
                    match_quantity: 0,
                    match_status: MatchStatus::Unmatched,
                }
            })
            .collect();

        let subtotal: i64 = lines.iter().map(|l| (l.quantity as f64 * l.unit_price.amount as f64) as i64).sum();
        let tax_amount: i64 = lines
            .iter()
            .map(|l| ((l.quantity as f64 * l.unit_price.amount as f64) * l.tax_rate / 100.0) as i64)
            .sum();

        let bill = VendorBill {
            base,
            bill_number,
            vendor_invoice_number,
            vendor_id,
            purchase_order_id,
            bill_date,
            due_date,
            lines,
            subtotal: Money::new(subtotal, Currency::USD),
            tax_amount: Money::new(tax_amount, Currency::USD),
            total: Money::new(subtotal + tax_amount, Currency::USD),
            amount_paid: Money::new(0, Currency::USD),
            status: VendorBillStatus::Draft,
            match_status: MatchStatus::Unmatched,
            notes,
        };

        VendorBillRepository::create(pool, bill.clone())
            .await
            .map_err(Error::Internal)?;
        Ok(bill)
    }

    pub async fn submit(pool: &SqlitePool, id: Uuid) -> Result<()> {
        let bill = Self::get(pool, id).await?;
        if bill.status != VendorBillStatus::Draft {
            return Err(Error::business_rule("Only draft bills can be submitted"));
        }
        VendorBillRepository::update_status(pool, id, VendorBillStatus::Pending)
            .await
            .map_err(Error::Internal)
    }

    pub async fn approve(pool: &SqlitePool, id: Uuid) -> Result<()> {
        let bill = Self::get(pool, id).await?;
        if bill.status != VendorBillStatus::Pending {
            return Err(Error::business_rule("Only pending bills can be approved"));
        }
        VendorBillRepository::update_status(pool, id, VendorBillStatus::Approved)
            .await
            .map_err(Error::Internal)
    }

    pub async fn void(pool: &SqlitePool, id: Uuid) -> Result<()> {
        let bill = Self::get(pool, id).await?;
        if bill.status == VendorBillStatus::Paid || bill.status == VendorBillStatus::PartiallyPaid {
            return Err(Error::business_rule("Cannot void a bill with payments"));
        }
        VendorBillRepository::update_status(pool, id, VendorBillStatus::Void)
            .await
            .map_err(Error::Internal)
    }

    pub async fn record_payment(
        pool: &SqlitePool,
        bill_id: Uuid,
        payment_id: Uuid,
        amount: i64,
    ) -> Result<()> {
        let bill = Self::get(pool, bill_id).await?;
        if bill.status != VendorBillStatus::Approved && bill.status != VendorBillStatus::PartiallyPaid {
            return Err(Error::business_rule("Can only record payments on approved or partially paid bills"));
        }

        let payment = VendorBillPayment {
            id: Uuid::new_v4(),
            bill_id,
            payment_id,
            amount: Money::new(amount, Currency::USD),
            applied_at: Utc::now(),
        };

        VendorBillRepository::record_payment(pool, payment)
            .await
            .map_err(Error::Internal)?;

        let updated_bill = Self::get(pool, bill_id).await?;
        let new_status = if updated_bill.amount_paid.amount >= updated_bill.total.amount {
            VendorBillStatus::Paid
        } else {
            VendorBillStatus::PartiallyPaid
        };
        VendorBillRepository::update_status(pool, bill_id, new_status)
            .await
            .map_err(Error::Internal)?;

        Ok(())
    }

    pub async fn perform_three_way_match(pool: &SqlitePool, id: Uuid) -> Result<ThreeWayMatchResult> {
        let bill = Self::get(pool, id).await?;

        if bill.purchase_order_id.is_none() {
            return Ok(ThreeWayMatchResult {
                bill_id: id,
                po_id: None,
                total_matched_lines: 0,
                total_unmatched_lines: bill.lines.len() as i32,
                total_exceptions: bill.lines.len() as i32,
                match_status: MatchStatus::Exception,
                exceptions: bill
                    .lines
                    .iter()
                    .map(|l| MatchException {
                        bill_line_id: l.id,
                        exception_type: MatchExceptionType::MissingPO,
                        expected_value: "Purchase Order".to_string(),
                        actual_value: "None".to_string(),
                        message: "Bill line has no associated purchase order".to_string(),
                    })
                    .collect(),
            });
        }

        let mut matched_lines = 0;
        let mut unmatched_lines = 0;
        let mut exceptions = Vec::new();

        for line in &bill.lines {
            if line.po_line_id.is_none() {
                exceptions.push(MatchException {
                    bill_line_id: line.id,
                    exception_type: MatchExceptionType::MissingPO,
                    expected_value: "PO Line Reference".to_string(),
                    actual_value: "None".to_string(),
                    message: "Bill line is not linked to a PO line".to_string(),
                });
                unmatched_lines += 1;
                continue;
            }

            let po_line_result = sqlx::query_as::<_, (i64, i64)>(
                "SELECT quantity, unit_price FROM purchase_order_lines WHERE id = ?",
            )
            .bind(line.po_line_id.unwrap().to_string())
            .fetch_optional(pool)
            .await;

            match po_line_result {
                Ok(Some((po_qty, po_price))) => {
                    if line.quantity != po_qty {
                        exceptions.push(MatchException {
                            bill_line_id: line.id,
                            exception_type: MatchExceptionType::QuantityVariance,
                            expected_value: po_qty.to_string(),
                            actual_value: line.quantity.to_string(),
                            message: format!("Quantity variance: expected {}, got {}", po_qty, line.quantity),
                        });
                        unmatched_lines += 1;
                    } else if line.unit_price.amount != po_price {
                        exceptions.push(MatchException {
                            bill_line_id: line.id,
                            exception_type: MatchExceptionType::PriceVariance,
                            expected_value: format!("${}", po_price as f64 / 100.0),
                            actual_value: format!("${}", line.unit_price.amount as f64 / 100.0),
                            message: format!(
                                "Price variance: expected ${}, got ${}",
                                po_price as f64 / 100.0,
                                line.unit_price.amount as f64 / 100.0
                            ),
                        });
                        unmatched_lines += 1;
                    } else {
                        matched_lines += 1;
                    }
                }
                _ => {
                    exceptions.push(MatchException {
                        bill_line_id: line.id,
                        exception_type: MatchExceptionType::MissingReceipt,
                        expected_value: "PO Line".to_string(),
                        actual_value: "Not Found".to_string(),
                        message: "Referenced PO line not found".to_string(),
                    });
                    unmatched_lines += 1;
                }
            }
        }

        let match_status = if exceptions.is_empty() {
            MatchStatus::FullyMatched
        } else if matched_lines > 0 {
            MatchStatus::PartiallyMatched
        } else {
            MatchStatus::Exception
        };

        VendorBillRepository::update_match_status(pool, id, match_status.clone())
            .await
            .map_err(Error::Internal)?;

        Ok(ThreeWayMatchResult {
            bill_id: id,
            po_id: bill.purchase_order_id,
            total_matched_lines: matched_lines,
            total_unmatched_lines: unmatched_lines,
            total_exceptions: exceptions.len() as i32,
            match_status,
            exceptions,
        })
    }

    pub async fn delete(pool: &SqlitePool, id: Uuid) -> Result<()> {
        let bill = Self::get(pool, id).await?;
        if bill.status != VendorBillStatus::Draft {
            return Err(Error::business_rule("Only draft bills can be deleted"));
        }
        VendorBillRepository::delete(pool, id)
            .await
            .map_err(Error::Internal)
    }
}

impl Default for VendorBillService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_empty_invoice_number() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(async { sqlx::SqlitePool::connect(":memory:").await.unwrap() });

        let result = rt.block_on(VendorBillService::create(
            &pool,
            Uuid::new_v4(),
            "".to_string(),
            None,
            Utc::now(),
            Utc::now(),
            vec![],
            None,
        ));
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_empty_lines() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(async { sqlx::SqlitePool::connect(":memory:").await.unwrap() });

        let result = rt.block_on(VendorBillService::create(
            &pool,
            Uuid::new_v4(),
            "INV-001".to_string(),
            None,
            Utc::now(),
            Utc::now(),
            vec![],
            None,
        ));
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_due_date_before_bill_date() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(async { sqlx::SqlitePool::connect(":memory:").await.unwrap() });

        let now = Utc::now();
        let past = now - chrono::Duration::days(1);

        let result = rt.block_on(VendorBillService::create(
            &pool,
            Uuid::new_v4(),
            "INV-001".to_string(),
            None,
            now,
            past,
            vec![VendorBillLineCreateRequest {
                po_line_id: None,
                product_id: None,
                description: "Test".to_string(),
                quantity: 1,
                unit_price: 1000,
                tax_rate: 0.0,
            }],
            None,
        ));
        assert!(result.is_err());
    }
}
