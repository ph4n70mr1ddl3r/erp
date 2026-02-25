use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait PartnerRepository: Send + Sync {
    async fn create_partner(&self, partner: &Partner) -> Result<Partner>;
    async fn get_partner(&self, id: Uuid) -> Result<Option<Partner>>;
    async fn list_partners(&self, status: Option<PartnerStatus>) -> Result<Vec<Partner>>;
    async fn update_partner(&self, partner: &Partner) -> Result<Partner>;
    async fn create_contact(&self, contact: &PartnerContact) -> Result<PartnerContact>;
    async fn create_agreement(&self, agreement: &PartnerAgreement) -> Result<PartnerAgreement>;
    async fn create_deal(&self, deal: &PartnerDeal) -> Result<PartnerDeal>;
    async fn update_deal(&self, deal: &PartnerDeal) -> Result<PartnerDeal>;
    async fn create_deal_registration(&self, reg: &PartnerDealRegistration) -> Result<PartnerDealRegistration>;
    async fn create_commission(&self, commission: &PartnerCommission) -> Result<PartnerCommission>;
    async fn create_performance(&self, perf: &PartnerPerformance) -> Result<PartnerPerformance>;
}

pub struct SqlitePartnerRepository { pool: SqlitePool }
impl SqlitePartnerRepository { pub fn new(pool: SqlitePool) -> Self { Self { pool } } }

#[async_trait]
impl PartnerRepository for SqlitePartnerRepository {
    async fn create_partner(&self, partner: &Partner) -> Result<Partner> {
        let p = partner.clone();
        sqlx::query!(r#"INSERT INTO partners (id, partner_number, name, legal_name, partner_type, tier,
            parent_partner_id, primary_contact_id, website, email, phone, address, city, state, country,
            postal_code, tax_id, registration_date, agreement_date, agreement_expiry, contract_value,
            currency, commission_rate, discount_rate, credit_limit, payment_terms_days, certification_level,
            certifications, specializations, regions_served, industries_served, annual_revenue,
            employee_count, notes, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            p.base.id, p.partner_number, p.name, p.legal_name, p.partner_type, p.tier, p.parent_partner_id,
            p.primary_contact_id, p.website, p.email, p.phone, p.address, p.city, p.state, p.country,
            p.postal_code, p.tax_id, p.registration_date, p.agreement_date, p.agreement_expiry, p.contract_value,
            p.currency, p.commission_rate, p.discount_rate, p.credit_limit, p.payment_terms_days, p.certification_level,
            p.certifications, p.specializations, p.regions_served, p.industries_served, p.annual_revenue,
            p.employee_count, p.notes, p.status, p.created_at, p.updated_at).execute(&self.pool).await?;
        Ok(p)
    }
    async fn get_partner(&self, _id: Uuid) -> Result<Option<Partner>> { Ok(None) }
    async fn list_partners(&self, _status: Option<PartnerStatus>) -> Result<Vec<Partner>> { Ok(vec![]) }
    async fn update_partner(&self, partner: &Partner) -> Result<Partner> { Ok(partner.clone()) }
    async fn create_contact(&self, contact: &PartnerContact) -> Result<PartnerContact> {
        let c = contact.clone();
        sqlx::query!(r#"INSERT INTO partner_contacts (id, partner_id, first_name, last_name, email,
            phone, mobile, title, department, is_primary, receive_notifications, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            c.base.id, c.partner_id, c.first_name, c.last_name, c.email, c.phone, c.mobile,
            c.title, c.department, c.is_primary, c.receive_notifications, c.status, c.created_at, c.updated_at).execute(&self.pool).await?;
        Ok(c)
    }
    async fn create_agreement(&self, agreement: &PartnerAgreement) -> Result<PartnerAgreement> {
        let a = agreement.clone();
        sqlx::query!(r#"INSERT INTO partner_agreements (id, agreement_number, partner_id, agreement_type,
            name, description, start_date, end_date, auto_renew, renewal_term_months, notice_period_days,
            commission_rate, discount_rate, min_sales_target, max_sales_limit, territory, exclusivity,
            document_path, signed_date, signed_by, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            a.base.id, a.agreement_number, a.partner_id, a.agreement_type, a.name, a.description,
            a.start_date, a.end_date, a.auto_renew, a.renewal_term_months, a.notice_period_days,
            a.commission_rate, a.discount_rate, a.min_sales_target, a.max_sales_limit, a.territory, a.exclusivity,
            a.document_path, a.signed_date, a.signed_by, a.status, a.created_at, a.updated_at).execute(&self.pool).await?;
        Ok(a)
    }
    async fn create_deal(&self, deal: &PartnerDeal) -> Result<PartnerDeal> {
        let d = deal.clone();
        sqlx::query!(r#"INSERT INTO partner_deals (id, deal_number, partner_id, customer_id, customer_name,
            deal_name, description, deal_type, stage, amount, currency, expected_close_date, probability,
            lead_source, products, partner_commission, internal_sales_rep_id, partner_contact_id, notes,
            won_date, lost_date, lost_reason, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            d.base.id, d.deal_number, d.partner_id, d.customer_id, d.customer_name, d.deal_name, d.description,
            d.deal_type, d.stage, d.amount, d.currency, d.expected_close_date, d.probability, d.lead_source,
            d.products, d.partner_commission, d.internal_sales_rep_id, d.partner_contact_id, d.notes,
            d.won_date, d.lost_date, d.lost_reason, d.created_at, d.updated_at).execute(&self.pool).await?;
        Ok(d)
    }
    async fn update_deal(&self, deal: &PartnerDeal) -> Result<PartnerDeal> { Ok(deal.clone()) }
    async fn create_deal_registration(&self, reg: &PartnerDealRegistration) -> Result<PartnerDealRegistration> {
        let r = reg.clone();
        sqlx::query!(r#"INSERT INTO partner_deal_registrations (id, registration_number, partner_id,
            deal_id, customer_name, opportunity_name, estimated_value, currency, expected_close_date,
            products, registration_date, expiry_date, status, approved_by, approved_at, rejection_reason, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            r.base.id, r.registration_number, r.partner_id, r.deal_id, r.customer_name, r.opportunity_name,
            r.estimated_value, r.currency, r.expected_close_date, r.products, r.registration_date, r.expiry_date,
            r.status, r.approved_by, r.approved_at, r.rejection_reason, r.created_at).execute(&self.pool).await?;
        Ok(r)
    }
    async fn create_commission(&self, commission: &PartnerCommission) -> Result<PartnerCommission> {
        let c = commission.clone();
        sqlx::query!(r#"INSERT INTO partner_commissions (id, commission_number, partner_id, deal_id,
            invoice_id, commission_date, revenue_amount, commission_rate, commission_amount, currency,
            status, paid_date, payment_reference, notes, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            c.base.id, c.commission_number, c.partner_id, c.deal_id, c.invoice_id, c.commission_date,
            c.revenue_amount, c.commission_rate, c.commission_amount, c.currency, c.status,
            c.paid_date, c.payment_reference, c.notes, c.created_at).execute(&self.pool).await?;
        Ok(c)
    }
    async fn create_performance(&self, perf: &PartnerPerformance) -> Result<PartnerPerformance> {
        let p = perf.clone();
        sqlx::query!(r#"INSERT INTO partner_performances (id, partner_id, period_type, period_start,
            period_end, deals_opened, deals_won, deals_lost, total_pipeline, total_revenue,
            total_commission, win_rate_percent, avg_deal_size, avg_sales_cycle_days, customer_satisfaction,
            target_revenue, attainment_percent, currency, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            p.base.id, p.partner_id, p.period_type, p.period_start, p.period_end, p.deals_opened,
            p.deals_won, p.deals_lost, p.total_pipeline, p.total_revenue, p.total_commission,
            p.win_rate_percent, p.avg_deal_size, p.avg_sales_cycle_days, p.customer_satisfaction,
            p.target_revenue, p.attainment_percent, p.currency, p.created_at).execute(&self.pool).await?;
        Ok(p)
    }
}
