use anyhow::Result;
use chrono::Utc;
use erp_core::{Currency, Error, Money, Paginated, Pagination};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::CreditNoteRepository;

pub struct CreditNoteService;

impl CreditNoteService {
    pub async fn create(
        pool: &SqlitePool,
        req: CreateCreditNoteRequest,
    ) -> Result<CreditNote> {
        let credit_note_number = CreditNoteRepository::get_next_number(pool).await?;
        let now = Utc::now();
        let credit_note_date = req.credit_note_date.unwrap_or(now);
        let currency = Currency::USD;

        let lines: Vec<CreditNoteLine> = req
            .lines
            .iter()
            .map(|l| CreditNoteLine {
                id: Uuid::new_v4(),
                product_id: l.product_id,
                description: l.description.clone(),
                quantity: l.quantity,
                unit_price: Money::new(l.unit_price, currency.clone()),
                line_total: Money::new(l.quantity * l.unit_price, currency.clone()),
            })
            .collect();

        let subtotal = lines.iter().map(|l| l.line_total.amount).sum();
        let tax_amount = 0i64;
        let total = subtotal + tax_amount;

        let credit_note = CreditNote {
            base: erp_core::BaseEntity::new(),
            credit_note_number,
            customer_id: req.customer_id,
            invoice_id: req.invoice_id,
            credit_note_date,
            lines,
            subtotal: Money::new(subtotal, currency.clone()),
            tax_amount: Money::new(tax_amount, currency.clone()),
            total: Money::new(total, currency.clone()),
            reason: req.reason,
            notes: req.notes,
            status: CreditNoteStatus::Draft,
            applied_amount: Money::zero(currency),
        };

        CreditNoteRepository::create(pool, &credit_note).await?;

        Ok(credit_note)
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<CreditNote> {
        CreditNoteRepository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow::anyhow!(Error::not_found("CreditNote", &id.to_string())))
    }

    pub async fn list(pool: &SqlitePool, page: u32, per_page: u32) -> Result<Paginated<CreditNote>> {
        let pagination = Pagination { page, per_page };
        CreditNoteRepository::list(pool, pagination).await
    }

    pub async fn list_by_customer(pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<CreditNote>> {
        CreditNoteRepository::list_by_customer(pool, customer_id).await
    }

    pub async fn issue(pool: &SqlitePool, id: Uuid) -> Result<CreditNote> {
        let mut credit_note = Self::get(pool, id).await?;
        
        if credit_note.status != CreditNoteStatus::Draft {
            return Err(Error::business_rule("Only draft credit notes can be issued").into());
        }

        credit_note.status = CreditNoteStatus::Issued;
        CreditNoteRepository::update_status(pool, id, &CreditNoteStatus::Issued).await?;

        Ok(credit_note)
    }

    pub async fn apply_to_invoice(
        pool: &SqlitePool,
        id: Uuid,
        req: ApplyCreditNoteRequest,
    ) -> Result<CreditNoteApplication> {
        let credit_note = Self::get(pool, id).await?;

        if credit_note.status != CreditNoteStatus::Issued {
            return Err(Error::business_rule("Only issued credit notes can be applied").into());
        }

        let available_amount = credit_note.total.amount - credit_note.applied_amount.amount;
        if req.amount > available_amount {
            return Err(Error::business_rule("Amount exceeds available credit").into());
        }

        let application = CreditNoteApplication {
            id: Uuid::new_v4(),
            credit_note_id: id,
            invoice_id: req.invoice_id,
            amount: Money::new(req.amount, credit_note.total.currency.clone()),
            applied_at: Utc::now(),
        };

        CreditNoteRepository::create_application(pool, &application).await?;

        let new_applied = credit_note.applied_amount.amount + req.amount;
        CreditNoteRepository::update_applied_amount(pool, id, new_applied).await?;

        if new_applied >= credit_note.total.amount {
            CreditNoteRepository::update_status(pool, id, &CreditNoteStatus::Applied).await?;
        }

        Ok(application)
    }

    pub async fn void(pool: &SqlitePool, id: Uuid) -> Result<CreditNote> {
        let mut credit_note = Self::get(pool, id).await?;

        if credit_note.applied_amount.amount > 0 {
            return Err(Error::business_rule("Cannot void a credit note that has been applied").into());
        }

        credit_note.status = CreditNoteStatus::Void;
        CreditNoteRepository::update_status(pool, id, &CreditNoteStatus::Void).await?;

        Ok(credit_note)
    }
}
