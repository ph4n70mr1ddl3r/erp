use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency};
use crate::models::*;
use crate::repository::*;

pub struct VendorService { repo: SqliteVendorRepository }
impl Default for VendorService {
    fn default() -> Self {
        Self::new()
    }
}

impl VendorService {
    pub fn new() -> Self { Self { repo: SqliteVendorRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Vendor> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Vendor>> { self.repo.find_all(pool, pagination).await }
    pub async fn create(&self, pool: &SqlitePool, vendor: Vendor) -> Result<Vendor> {
        if vendor.code.is_empty() { return Err(Error::validation("Vendor code is required")); }
        if vendor.name.is_empty() { return Err(Error::validation("Vendor name is required")); }
        self.repo.create(pool, vendor).await
    }
    pub async fn update(&self, pool: &SqlitePool, vendor: Vendor) -> Result<Vendor> { self.repo.update(pool, vendor).await }
}

pub struct PurchaseOrderService { repo: SqlitePurchaseOrderRepository }
impl Default for PurchaseOrderService {
    fn default() -> Self {
        Self::new()
    }
}

impl PurchaseOrderService {
    pub fn new() -> Self { Self { repo: SqlitePurchaseOrderRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<PurchaseOrder> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<PurchaseOrder>> { self.repo.find_all(pool, pagination).await }
    
    pub async fn create(&self, pool: &SqlitePool, mut order: PurchaseOrder) -> Result<PurchaseOrder> {
        if order.lines.is_empty() { return Err(Error::validation("PO must have at least one line")); }
        let subtotal: i64 = order.lines.iter().map(|l| l.line_total.amount).sum();
        order.subtotal = Money::new(subtotal, Currency::USD);
        order.total = Money::new(subtotal + order.tax_amount.amount, Currency::USD);
        order.po_number = format!("PO-{}", Utc::now().format("%Y%m%d%H%M%S"));
        order.base = BaseEntity::new();
        order.status = Status::Draft;
        for line in &mut order.lines { line.id = Uuid::new_v4(); }
        self.repo.create(pool, order).await
    }
    
    pub async fn submit(&self, pool: &SqlitePool, id: Uuid) -> Result<()> { self.repo.update_status(pool, id, Status::Pending).await }
    pub async fn approve(&self, pool: &SqlitePool, id: Uuid) -> Result<()> { self.repo.update_status(pool, id, Status::Approved).await }
    pub async fn receive(&self, pool: &SqlitePool, id: Uuid) -> Result<()> { self.repo.update_status(pool, id, Status::Completed).await }
}

pub struct SupplierScorecardService;

impl Default for SupplierScorecardService {
    fn default() -> Self {
        Self::new()
    }
}

impl SupplierScorecardService {
    pub fn new() -> Self { Self }

    pub async fn create_scorecard(
        pool: &SqlitePool,
        vendor_id: Uuid,
        period: &str,
    ) -> Result<SupplierScorecard> {
        let now = chrono::Utc::now();
        let scorecard = SupplierScorecard {
            id: Uuid::new_v4(),
            vendor_id,
            period: period.to_string(),
            on_time_delivery: 0,
            quality_score: 0,
            price_competitiveness: 0,
            responsiveness: 0,
            overall_score: 0,
            total_orders: 0,
            total_value: 0,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO supplier_scorecards (id, vendor_id, period, on_time_delivery, quality_score, price_competitiveness, responsiveness, overall_score, total_orders, total_value, created_at)
             VALUES (?, ?, ?, 0, 0, 0, 0, 0, 0, 0, ?)"
        )
        .bind(scorecard.id.to_string())
        .bind(scorecard.vendor_id.to_string())
        .bind(&scorecard.period)
        .bind(scorecard.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(scorecard)
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<SupplierScorecard> {
        let row = sqlx::query_as::<_, ScorecardRow>(
            "SELECT id, vendor_id, period, on_time_delivery, quality_score, price_competitiveness, responsiveness, overall_score, total_orders, total_value, created_at
             FROM supplier_scorecards WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("SupplierScorecard", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn get_for_vendor(pool: &SqlitePool, vendor_id: Uuid) -> Result<Vec<SupplierScorecard>> {
        let rows = sqlx::query_as::<_, ScorecardRow>(
            "SELECT id, vendor_id, period, on_time_delivery, quality_score, price_competitiveness, responsiveness, overall_score, total_orders, total_value, created_at
             FROM supplier_scorecards WHERE vendor_id = ? ORDER BY period DESC"
        )
        .bind(vendor_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn record_performance(
        pool: &SqlitePool,
        vendor_id: Uuid,
        order_id: Uuid,
        expected_date: Option<&str>,
        delivery_date: Option<&str>,
        quality_rating: i32,
        notes: Option<&str>,
    ) -> Result<VendorPerformance> {
        let now = chrono::Utc::now();
        let exp = expected_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
            .map(|d| d.with_timezone(&chrono::Utc));
        let del = delivery_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
            .map(|d| d.with_timezone(&chrono::Utc));
        
        let on_time = match (&exp, &del) {
            (Some(e), Some(d)) => d <= e,
            _ => false,
        };
        
        let perf = VendorPerformance {
            id: Uuid::new_v4(),
            vendor_id,
            order_id,
            delivery_date: del,
            expected_date: exp,
            on_time,
            quality_rating,
            notes: notes.map(|s| s.to_string()),
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO vendor_performance (id, vendor_id, order_id, delivery_date, expected_date, on_time, quality_rating, notes, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(perf.id.to_string())
        .bind(perf.vendor_id.to_string())
        .bind(perf.order_id.to_string())
        .bind(perf.delivery_date.map(|d| d.to_rfc3339()))
        .bind(perf.expected_date.map(|d| d.to_rfc3339()))
        .bind(if perf.on_time { 1 } else { 0 })
        .bind(perf.quality_rating)
        .bind(&perf.notes)
        .bind(perf.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(perf)
    }

    pub async fn calculate_score(pool: &SqlitePool, scorecard_id: Uuid) -> Result<SupplierScorecard> {
        let scorecard = Self::get(pool, scorecard_id).await?;
        let period_start = format!("{}-01", scorecard.period);
        
        let stats: (i64, i64, i64) = sqlx::query_as(
            "SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN on_time = 1 THEN 1 ELSE 0 END) as on_time_count,
                AVG(quality_rating) as avg_quality
             FROM vendor_performance 
             WHERE vendor_id = ? AND created_at >= ?"
        )
        .bind(scorecard.vendor_id.to_string())
        .bind(&period_start)
        .fetch_one(pool)
        .await
        .map_err(Error::Database)?;
        
        let total = stats.0 as i32;
        let on_time_count = stats.1 as i32;
        let avg_quality = stats.2 as i32;
        
        let on_time_pct = if total > 0 { (on_time_count * 100) / total } else { 0 };
        let overall = (on_time_pct + avg_quality) / 2;
        
        sqlx::query(
            "UPDATE supplier_scorecards SET on_time_delivery = ?, quality_score = ?, overall_score = ?, total_orders = ? WHERE id = ?"
        )
        .bind(on_time_pct)
        .bind(avg_quality)
        .bind(overall)
        .bind(total)
        .bind(scorecard_id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get(pool, scorecard_id).await
    }
}

use erp_inventory::ValuationService;

pub struct LandedCostService {
    repo: SqliteLandedCostRepository,
    po_repo: SqlitePurchaseOrderRepository,
    inventory_service: ValuationService,
}

impl Default for LandedCostService {
    fn default() -> Self {
        Self::new()
    }
}

impl LandedCostService {
    pub fn new() -> Self {
        Self {
            repo: SqliteLandedCostRepository,
            po_repo: SqlitePurchaseOrderRepository,
            inventory_service: ValuationService::new(),
        }
    }

    pub async fn create_voucher(
        &self,
        pool: &SqlitePool,
        voucher_number: &str,
        reference_type: LandedCostReferenceType,
        reference_id: Uuid,
        lines: Vec<LandedCostLine>,
    ) -> Result<LandedCostVoucher> {
        let total_amount = lines.iter().map(|l| l.amount.amount).sum();
        let voucher = LandedCostVoucher {
            base: BaseEntity::new(),
            voucher_number: voucher_number.to_string(),
            voucher_date: Utc::now(),
            reference_type,
            reference_id,
            total_landed_cost: Money::new(total_amount, Currency::USD),
            status: Status::Draft,
            lines,
        };
        
        self.repo.create_voucher(pool, voucher).await
    }

    pub async fn post_voucher(&self, pool: &SqlitePool, voucher_id: Uuid) -> Result<()> {
        let voucher = self.repo.find_voucher_by_id(pool, voucher_id).await?;
        if voucher.status == Status::Posted {
            return Err(Error::business_rule("Voucher already posted"));
        }

        let order = self.po_repo.find_by_id(pool, voucher.reference_id).await?;
        let categories = self.repo.find_categories(pool).await?;
        
        let allocations = self.calculate_allocations(&voucher, &order, &categories)?;
        
        // Persist allocations
        self.repo.save_allocations(pool, allocations.clone()).await?;

        // Update inventory costs
        for allocation in allocations {
            // Find a warehouse for the product (simple approach: take the first one or use PO details if available)
            // For now, we'll assume a default warehouse or lookup
            let warehouse_id = Uuid::nil(); // Placeholder - in a real system we'd get this from the GR
            
            self.inventory_service.apply_landed_costs(
                pool,
                allocation.item_id,
                warehouse_id,
                allocation.allocated_amount.amount,
                &voucher.voucher_number,
            ).await?;
        }

        self.repo.update_voucher_status(pool, voucher_id, Status::Posted).await?;
        
        Ok(())
    }

    pub fn calculate_allocations(
        &self,
        voucher: &LandedCostVoucher,
        order: &PurchaseOrder,
        categories: &[LandedCostCategory],
    ) -> Result<Vec<LandedCostAllocation>> {
        let mut all_allocations = Vec::new();

        for voucher_line in &voucher.lines {
            let category = categories.iter()
                .find(|c| c.id == voucher_line.category_id)
                .ok_or_else(|| Error::validation(&format!("Category not found: {}", voucher_line.category_id)))?;

            let line_allocations = match category.allocation_method {
                LandedCostAllocationMethod::ByValue => {
                    let total_po_value: i64 = order.lines.iter().map(|l| l.line_total.amount).sum();
                    if total_po_value == 0 { return Err(Error::validation("Cannot allocate by value to a zero-value PO")); }
                    
                    order.lines.iter().map(|line| {
                        let factor = line.line_total.amount as f64 / total_po_value as f64;
                        let amount = (voucher_line.amount.amount as f64 * factor) as i64;
                        LandedCostAllocation {
                            id: Uuid::new_v4(),
                            voucher_id: voucher.base.id,
                            item_id: line.product_id,
                            allocated_amount: Money::new(amount, order.total.currency.clone()),
                            allocation_factor: factor,
                        }
                    }).collect::<Vec<_>>()
                },
                LandedCostAllocationMethod::ByQuantity => {
                    let total_qty: i64 = order.lines.iter().map(|l| l.quantity).sum();
                    if total_qty == 0 { return Err(Error::validation("Cannot allocate by quantity to a zero-quantity PO")); }
                    
                    order.lines.iter().map(|line| {
                        let factor = line.quantity as f64 / total_qty as f64;
                        let amount = (voucher_line.amount.amount as f64 * factor) as i64;
                        LandedCostAllocation {
                            id: Uuid::new_v4(),
                            voucher_id: voucher.base.id,
                            item_id: line.product_id,
                            allocated_amount: Money::new(amount, order.total.currency.clone()),
                            allocation_factor: factor,
                        }
                    }).collect::<Vec<_>>()
                },
                LandedCostAllocationMethod::Equal => {
                    let count = order.lines.len();
                    if count == 0 { return Err(Error::validation("Cannot allocate to a PO with no lines")); }
                    
                    order.lines.iter().map(|line| {
                        let factor = 1.0 / count as f64;
                        let amount = (voucher_line.amount.amount as f64 * factor) as i64;
                        LandedCostAllocation {
                            id: Uuid::new_v4(),
                            voucher_id: voucher.base.id,
                            item_id: line.product_id,
                            allocated_amount: Money::new(amount, order.total.currency.clone()),
                            allocation_factor: factor,
                        }
                    }).collect::<Vec<_>>()
                },
                _ => return Err(Error::validation("Allocation method not yet implemented")),
            };
            all_allocations.extend(line_allocations);
        }

        // Aggregate by item_id
        let mut final_allocations = std::collections::HashMap::new();
        for alloc in all_allocations {
            let entry = final_allocations.entry(alloc.item_id).or_insert(LandedCostAllocation {
                id: Uuid::new_v4(),
                voucher_id: voucher.base.id,
                item_id: alloc.item_id,
                allocated_amount: Money::new(0, order.total.currency.clone()),
                allocation_factor: 0.0,
            });
            entry.allocated_amount.amount += alloc.allocated_amount.amount;
            entry.allocation_factor += alloc.allocation_factor;
        }

        Ok(final_allocations.into_values().collect())
    }
}

pub struct ThreeWayMatchService;

impl ThreeWayMatchService {
    pub fn perform_match(
        invoice: &VendorInvoice,
        po: &PurchaseOrder,
        gr: &GoodsReceipt,
    ) -> Result<ThreeWayMatch> {
        if invoice.purchase_order_id != po.base.id {
            return Err(Error::validation("Invoice PO ID does not match Purchase Order ID"));
        }
        if gr.purchase_order_id != po.base.id {
            return Err(Error::validation("Goods Receipt PO ID does not match Purchase Order ID"));
        }

        let mut price_matched = true;
        let mut quantity_matched = true;
        let mut price_variance = 0;
        let mut quantity_variance = 0;

        for inv_line in &invoice.lines {
            // Find corresponding PO line
            let po_line = po.lines.iter()
                .find(|l| l.product_id == inv_line.product_id)
                .ok_or_else(|| Error::validation(&format!("Product {} not found in PO", inv_line.product_id)))?;

            // Find corresponding GR line
            let gr_line = gr.lines.iter()
                .find(|l| l.product_id == inv_line.product_id)
                .ok_or_else(|| Error::validation(&format!("Product {} not found in Goods Receipt", inv_line.product_id)))?;

            // 1. Price Match (Invoice vs PO)
            if inv_line.unit_price.amount != po_line.unit_price.amount {
                price_matched = false;
                price_variance += (inv_line.unit_price.amount - po_line.unit_price.amount) * inv_line.quantity;
            }

            // 2. Quantity Match (Invoice vs Goods Receipt)
            if inv_line.quantity != gr_line.quantity_received {
                quantity_matched = false;
                quantity_variance += inv_line.quantity - gr_line.quantity_received;
            }
        }

        let status = if price_matched && quantity_matched {
            MatchStatus::Matched
        } else {
            MatchStatus::Variance
        };

        let notes = if status == MatchStatus::Variance {
            Some(format!(
                "Price Variance: {}, Quantity Variance: {}",
                price_variance, quantity_variance
            ))
        } else {
            None
        };

        Ok(ThreeWayMatch {
            id: Uuid::new_v4(),
            invoice_id: invoice.base.id,
            purchase_order_id: po.base.id,
            goods_receipt_id: gr.base.id,
            price_matched,
            quantity_matched,
            price_variance,
            quantity_variance,
            status,
            notes,
            created_at: Utc::now(),
        })
    }
}

#[derive(sqlx::FromRow)]
struct ScorecardRow {

    id: String,
    vendor_id: String,
    period: String,
    on_time_delivery: i64,
    quality_score: i64,
    price_competitiveness: i64,
    responsiveness: i64,
    overall_score: i64,
    total_orders: i64,
    total_value: i64,
    created_at: String,
}

impl From<ScorecardRow> for SupplierScorecard {
    fn from(r: ScorecardRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            vendor_id: Uuid::parse_str(&r.vendor_id).unwrap_or_default(),
            period: r.period,
            on_time_delivery: r.on_time_delivery as i32,
            quality_score: r.quality_score as i32,
            price_competitiveness: r.price_competitiveness as i32,
            responsiveness: r.responsiveness as i32,
            overall_score: r.overall_score as i32,
            total_orders: r.total_orders as i32,
            total_value: r.total_value,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
