use crate::models::*;
use crate::repository::{PartnerRepository, SqlitePartnerRepository};
use chrono::{NaiveDate, Utc};
use erp_core::{BaseEntity, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct PartnerService { repo: SqlitePartnerRepository }
impl PartnerService {
    pub fn new(pool: SqlitePool) -> Self { Self { repo: SqlitePartnerRepository::new(pool) } }

    pub async fn create_partner(&self, _pool: &SqlitePool, req: CreatePartnerRequest) -> Result<Partner> {
        let partner = Partner {
            base: BaseEntity::new(),
            partner_number: format!("PTR-{}", Uuid::new_v4()),
            name: req.name,
            legal_name: req.legal_name,
            partner_type: req.partner_type,
            tier: PartnerTier::Registered,
            parent_partner_id: None,
            primary_contact_id: None,
            website: req.website,
            email: req.email,
            phone: req.phone,
            address: req.address,
            city: req.city,
            state: req.state,
            country: req.country,
            postal_code: req.postal_code,
            tax_id: req.tax_id,
            registration_date: Some(Utc::now().date_naive()),
            agreement_date: None,
            agreement_expiry: None,
            contract_value: None,
            currency: req.currency.unwrap_or_else(|| "USD".to_string()),
            commission_rate: req.commission_rate.unwrap_or(10.0),
            discount_rate: req.discount_rate.unwrap_or(0.0),
            credit_limit: req.credit_limit,
            payment_terms_days: req.payment_terms_days.unwrap_or(30),
            certification_level: None,
            certifications: None,
            specializations: req.specializations,
            regions_served: req.regions_served,
            industries_served: req.industries_served,
            annual_revenue: req.annual_revenue,
            employee_count: req.employee_count,
            notes: req.notes,
            status: PartnerStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_partner(&partner).await
    }

    pub async fn create_deal(&self, _pool: &SqlitePool, req: CreateDealRequest) -> Result<PartnerDeal> {
        let deal = PartnerDeal {
            base: BaseEntity::new(),
            deal_number: format!("DEAL-{}", Uuid::new_v4()),
            partner_id: req.partner_id,
            customer_id: req.customer_id,
            customer_name: req.customer_name,
            deal_name: req.deal_name,
            description: req.description,
            deal_type: req.deal_type,
            stage: DealStage::Qualified,
            amount: req.amount,
            currency: req.currency,
            expected_close_date: req.expected_close_date,
            probability: 20,
            lead_source: req.lead_source,
            products: req.products,
            partner_commission: (req.amount as f64 * 0.1) as i64,
            internal_sales_rep_id: None,
            partner_contact_id: None,
            notes: req.notes,
            won_date: None,
            lost_date: None,
            lost_reason: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_deal(&deal).await
    }

    pub async fn register_deal(&self, _pool: &SqlitePool, req: RegisterDealRequest) -> Result<PartnerDealRegistration> {
        let reg = PartnerDealRegistration {
            base: BaseEntity::new(),
            registration_number: format!("REG-{}", Uuid::new_v4()),
            partner_id: req.partner_id,
            deal_id: req.deal_id,
            customer_name: req.customer_name,
            opportunity_name: req.opportunity_name,
            estimated_value: req.estimated_value,
            currency: req.currency,
            expected_close_date: req.expected_close_date,
            products: req.products,
            registration_date: Utc::now().date_naive(),
            expiry_date: Utc::now().date_naive() + chrono::Duration::days(90),
            status: RegistrationStatus::Pending,
            approved_by: None,
            approved_at: None,
            rejection_reason: None,
            created_at: Utc::now(),
        };
        self.repo.create_deal_registration(&reg).await
    }

    pub async fn calculate_commission(&self, _pool: &SqlitePool, partner_id: Uuid, deal_id: Uuid, revenue: i64, rate: f64) -> Result<PartnerCommission> {
        let commission = PartnerCommission {
            base: BaseEntity::new(),
            commission_number: format!("COMM-{}", Uuid::new_v4()),
            partner_id,
            deal_id: Some(deal_id),
            invoice_id: None,
            commission_date: Utc::now().date_naive(),
            revenue_amount: revenue,
            commission_rate: rate,
            commission_amount: (revenue as f64 * rate / 100.0) as i64,
            currency: "USD".to_string(),
            status: CommissionStatus::Accrued,
            paid_date: None,
            payment_reference: None,
            notes: None,
            created_at: Utc::now(),
        };
        self.repo.create_commission(&commission).await
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CreatePartnerRequest {
    pub name: String,
    pub legal_name: Option<String>,
    pub partner_type: PartnerType,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub tax_id: Option<String>,
    pub currency: Option<String>,
    pub commission_rate: Option<f64>,
    pub discount_rate: Option<f64>,
    pub credit_limit: Option<i64>,
    pub payment_terms_days: Option<i32>,
    pub specializations: Option<String>,
    pub regions_served: Option<String>,
    pub industries_served: Option<String>,
    pub annual_revenue: Option<i64>,
    pub employee_count: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateDealRequest {
    pub partner_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_name: String,
    pub deal_name: String,
    pub description: Option<String>,
    pub deal_type: DealType,
    pub amount: i64,
    pub currency: String,
    pub expected_close_date: NaiveDate,
    pub lead_source: Option<String>,
    pub products: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RegisterDealRequest {
    pub partner_id: Uuid,
    pub deal_id: Option<Uuid>,
    pub customer_name: String,
    pub opportunity_name: String,
    pub estimated_value: i64,
    pub currency: String,
    pub expected_close_date: NaiveDate,
    pub products: Option<String>,
}
