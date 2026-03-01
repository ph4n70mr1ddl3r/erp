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
        sqlx::query(r#"INSERT INTO partners (id, partner_number, name, legal_name, partner_type, tier,
            parent_partner_id, primary_contact_id, website, email, phone, address, city, state, country,
            postal_code, tax_id, registration_date, agreement_date, agreement_expiry, contract_value,
            currency, commission_rate, discount_rate, credit_limit, payment_terms_days, certification_level,
            certifications, specializations, regions_served, industries_served, annual_revenue,
            employee_count, notes, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(p.base.id).bind(&p.partner_number).bind(&p.name).bind(&p.legal_name).bind(&p.partner_type)
            .bind(&p.tier).bind(p.parent_partner_id).bind(p.primary_contact_id).bind(&p.website).bind(&p.email)
            .bind(&p.phone).bind(&p.address).bind(&p.city).bind(&p.state).bind(&p.country).bind(&p.postal_code)
            .bind(&p.tax_id).bind(p.registration_date).bind(p.agreement_date).bind(p.agreement_expiry)
            .bind(p.contract_value).bind(&p.currency).bind(p.commission_rate).bind(p.discount_rate)
            .bind(p.credit_limit).bind(p.payment_terms_days).bind(&p.certification_level).bind(&p.certifications)
            .bind(&p.specializations).bind(&p.regions_served).bind(&p.industries_served).bind(p.annual_revenue)
            .bind(p.employee_count).bind(&p.notes).bind(&p.status).bind(p.created_at).bind(p.updated_at)
            .execute(&self.pool).await?;
        Ok(p)
    }
    async fn get_partner(&self, _id: Uuid) -> Result<Option<Partner>> { Ok(None) }
    async fn list_partners(&self, _status: Option<PartnerStatus>) -> Result<Vec<Partner>> { Ok(vec![]) }
    async fn update_partner(&self, partner: &Partner) -> Result<Partner> { Ok(partner.clone()) }
    async fn create_contact(&self, contact: &PartnerContact) -> Result<PartnerContact> {
        let c = contact.clone();
        sqlx::query(r#"INSERT INTO partner_contacts (id, partner_id, first_name, last_name, email,
            phone, mobile, title, department, is_primary, receive_notifications, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(c.base.id).bind(c.partner_id).bind(&c.first_name).bind(&c.last_name).bind(&c.email)
            .bind(&c.phone).bind(&c.mobile).bind(&c.title).bind(&c.department).bind(c.is_primary)
            .bind(c.receive_notifications).bind(&c.status).bind(c.created_at).bind(c.updated_at)
            .execute(&self.pool).await?;
        Ok(c)
    }
    async fn create_agreement(&self, agreement: &PartnerAgreement) -> Result<PartnerAgreement> {
        let a = agreement.clone();
        sqlx::query(r#"INSERT INTO partner_agreements (id, agreement_number, partner_id, agreement_type,
            name, description, start_date, end_date, auto_renew, renewal_term_months, notice_period_days,
            commission_rate, discount_rate, min_sales_target, max_sales_limit, territory, exclusivity,
            document_path, signed_date, signed_by, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(a.base.id).bind(&a.agreement_number).bind(a.partner_id).bind(&a.agreement_type).bind(&a.name)
            .bind(&a.description).bind(a.start_date).bind(a.end_date).bind(a.auto_renew).bind(a.renewal_term_months)
            .bind(a.notice_period_days).bind(a.commission_rate).bind(a.discount_rate).bind(a.min_sales_target)
            .bind(a.max_sales_limit).bind(&a.territory).bind(a.exclusivity).bind(&a.document_path)
            .bind(a.signed_date).bind(a.signed_by).bind(&a.status).bind(a.created_at).bind(a.updated_at)
            .execute(&self.pool).await?;
        Ok(a)
    }
    async fn create_deal(&self, deal: &PartnerDeal) -> Result<PartnerDeal> {
        let d = deal.clone();
        sqlx::query(r#"INSERT INTO partner_deals (id, deal_number, partner_id, customer_id, customer_name,
            deal_name, description, deal_type, stage, amount, currency, expected_close_date, probability,
            lead_source, products, partner_commission, internal_sales_rep_id, partner_contact_id, notes,
            won_date, lost_date, lost_reason, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(d.base.id).bind(&d.deal_number).bind(d.partner_id).bind(d.customer_id).bind(&d.customer_name)
            .bind(&d.deal_name).bind(&d.description).bind(&d.deal_type).bind(&d.stage).bind(d.amount)
            .bind(&d.currency).bind(d.expected_close_date).bind(d.probability).bind(&d.lead_source)
            .bind(&d.products).bind(d.partner_commission).bind(d.internal_sales_rep_id).bind(d.partner_contact_id)
            .bind(&d.notes).bind(d.won_date).bind(d.lost_date).bind(&d.lost_reason).bind(d.created_at).bind(d.updated_at)
            .execute(&self.pool).await?;
        Ok(d)
    }
    async fn update_deal(&self, deal: &PartnerDeal) -> Result<PartnerDeal> { Ok(deal.clone()) }
    async fn create_deal_registration(&self, reg: &PartnerDealRegistration) -> Result<PartnerDealRegistration> {
        let r = reg.clone();
        sqlx::query(r#"INSERT INTO partner_deal_registrations (id, registration_number, partner_id,
            deal_id, customer_name, opportunity_name, estimated_value, currency, expected_close_date,
            products, registration_date, expiry_date, status, approved_by, approved_at, rejection_reason, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(r.base.id).bind(&r.registration_number).bind(r.partner_id).bind(r.deal_id).bind(&r.customer_name)
            .bind(&r.opportunity_name).bind(r.estimated_value).bind(&r.currency).bind(r.expected_close_date)
            .bind(&r.products).bind(r.registration_date).bind(r.expiry_date).bind(&r.status).bind(r.approved_by)
            .bind(r.approved_at).bind(&r.rejection_reason).bind(r.created_at)
            .execute(&self.pool).await?;
        Ok(r)
    }
    async fn create_commission(&self, commission: &PartnerCommission) -> Result<PartnerCommission> {
        let c = commission.clone();
        sqlx::query(r#"INSERT INTO partner_commissions (id, commission_number, partner_id, deal_id,
            invoice_id, commission_date, revenue_amount, commission_rate, commission_amount, currency,
            status, paid_date, payment_reference, notes, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(c.base.id).bind(&c.commission_number).bind(c.partner_id).bind(c.deal_id).bind(c.invoice_id)
            .bind(c.commission_date).bind(c.revenue_amount).bind(c.commission_rate).bind(c.commission_amount)
            .bind(&c.currency).bind(&c.status).bind(c.paid_date).bind(&c.payment_reference).bind(&c.notes).bind(c.created_at)
            .execute(&self.pool).await?;
        Ok(c)
    }
    async fn create_performance(&self, perf: &PartnerPerformance) -> Result<PartnerPerformance> {
        let p = perf.clone();
        sqlx::query(r#"INSERT INTO partner_performances (id, partner_id, period_type, period_start,
            period_end, deals_opened, deals_won, deals_lost, total_pipeline, total_revenue,
            total_commission, win_rate_percent, avg_deal_size, avg_sales_cycle_days, customer_satisfaction,
            target_revenue, attainment_percent, currency, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(p.base.id).bind(p.partner_id).bind(&p.period_type).bind(p.period_start).bind(p.period_end)
            .bind(p.deals_opened).bind(p.deals_won).bind(p.deals_lost).bind(p.total_pipeline).bind(p.total_revenue)
            .bind(p.total_commission).bind(p.win_rate_percent).bind(p.avg_deal_size).bind(p.avg_sales_cycle_days)
            .bind(p.customer_satisfaction).bind(p.target_revenue).bind(p.attainment_percent).bind(&p.currency).bind(p.created_at)
            .execute(&self.pool).await?;
        Ok(p)
    }
}
