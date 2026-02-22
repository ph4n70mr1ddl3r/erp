use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency};
use crate::models::*;
use crate::repository::*;

pub struct CustomerService { repo: SqliteCustomerRepository }
impl CustomerService {
    pub fn new() -> Self { Self { repo: SqliteCustomerRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Customer> { self.repo.find_by_id(pool, id).await }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Customer>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, customer: Customer) -> Result<Customer> {
        if customer.code.is_empty() { return Err(Error::validation("Customer code is required")); }
        if customer.name.is_empty() { return Err(Error::validation("Customer name is required")); }
        self.repo.create(pool, customer).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, customer: Customer) -> Result<Customer> {
        self.repo.update(pool, customer).await
    }
}

pub struct SalesOrderService { repo: SqliteSalesOrderRepository }
impl SalesOrderService {
    pub fn new() -> Self { Self { repo: SqliteSalesOrderRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<SalesOrder> { self.repo.find_by_id(pool, id).await }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<SalesOrder>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut order: SalesOrder) -> Result<SalesOrder> {
        if order.lines.is_empty() { return Err(Error::validation("Order must have at least one line")); }
        
        let subtotal: i64 = order.lines.iter().map(|l| l.line_total.amount).sum();
        order.subtotal = Money::new(subtotal, Currency::USD);
        order.total = Money::new(subtotal + order.tax_amount.amount, Currency::USD);
        order.order_number = format!("SO-{}", Utc::now().format("%Y%m%d%H%M%S"));
        order.base = BaseEntity::new();
        order.status = Status::Draft;
        
        for line in &mut order.lines { line.id = Uuid::new_v4(); }
        self.repo.create(pool, order).await
    }
    
    pub async fn confirm(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Approved).await
    }
    
    pub async fn ship(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Completed).await
    }
}

pub struct QuotationService { repo: SqliteQuotationRepository }
impl QuotationService {
    pub fn new() -> Self { Self { repo: SqliteQuotationRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<SalesQuote> { 
        self.repo.find_by_id(pool, id).await 
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<SalesQuote>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut quote: SalesQuote) -> Result<SalesQuote> {
        if quote.lines.is_empty() { return Err(Error::validation("Quote must have at least one line")); }
        
        let subtotal: i64 = quote.lines.iter().map(|l| l.line_total.amount).sum();
        quote.subtotal = Money::new(subtotal, Currency::USD);
        quote.total = Money::new(subtotal + quote.tax_amount.amount, Currency::USD);
        quote.quote_number = format!("QT-{}", Utc::now().format("%Y%m%d%H%M%S"));
        quote.base = BaseEntity::new();
        quote.status = Status::Draft;
        
        for line in &mut quote.lines { line.id = Uuid::new_v4(); }
        self.repo.create(pool, quote).await
    }
    
    pub async fn send(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Pending).await
    }
    
    pub async fn accept(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Approved).await
    }
    
    pub async fn reject(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Rejected).await
    }
    
    pub async fn convert_to_order(&self, pool: &SqlitePool, id: Uuid) -> Result<SalesOrder> {
        let quote = self.repo.find_by_id(pool, id).await?;
        if quote.status != Status::Approved {
            return Err(Error::business_rule("Only accepted quotes can be converted to orders"));
        }
        self.repo.convert_to_order(pool, quote).await
    }
}

pub struct LeadService;

impl LeadService {
    pub fn new() -> Self { Self }

    pub async fn create(
        pool: &SqlitePool,
        company_name: &str,
        contact_name: Option<&str>,
        email: Option<&str>,
        phone: Option<&str>,
        source: Option<&str>,
        industry: Option<&str>,
        estimated_value: i64,
        assigned_to: Option<Uuid>,
    ) -> Result<Lead> {
        let now = chrono::Utc::now();
        let lead_number = format!("LD-{}", now.format("%Y%m%d%H%M%S"));
        let lead = Lead {
            id: Uuid::new_v4(),
            lead_number: lead_number.clone(),
            company_name: company_name.to_string(),
            contact_name: contact_name.map(|s| s.to_string()),
            email: email.map(|s| s.to_string()),
            phone: phone.map(|s| s.to_string()),
            source: source.map(|s| s.to_string()),
            industry: industry.map(|s| s.to_string()),
            estimated_value,
            status: LeadStatus::New,
            assigned_to,
            notes: None,
            converted_to_customer: None,
            converted_at: None,
            created_at: now,
            updated_at: now,
        };
        
        sqlx::query(
            "INSERT INTO leads (id, lead_number, company_name, contact_name, email, phone, source, industry, estimated_value, status, assigned_to, notes, converted_to_customer, converted_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'New', ?, NULL, NULL, NULL, ?, ?)"
        )
        .bind(lead.id.to_string())
        .bind(&lead.lead_number)
        .bind(&lead.company_name)
        .bind(&lead.contact_name)
        .bind(&lead.email)
        .bind(&lead.phone)
        .bind(&lead.source)
        .bind(&lead.industry)
        .bind(lead.estimated_value)
        .bind(lead.assigned_to.map(|id| id.to_string()))
        .bind(lead.created_at.to_rfc3339())
        .bind(lead.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(lead)
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<Lead> {
        let row = sqlx::query_as::<_, LeadRow>(
            "SELECT id, lead_number, company_name, contact_name, email, phone, source, industry, estimated_value, status, assigned_to, notes, converted_to_customer, converted_at, created_at, updated_at
             FROM leads WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Lead", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list(pool: &SqlitePool, status: Option<LeadStatus>) -> Result<Vec<Lead>> {
        let rows = match status {
            Some(s) => {
                sqlx::query_as::<_, LeadRow>(
                    "SELECT id, lead_number, company_name, contact_name, email, phone, source, industry, estimated_value, status, assigned_to, notes, converted_to_customer, converted_at, created_at, updated_at
                     FROM leads WHERE status = ? ORDER BY created_at DESC"
                )
                .bind(format!("{:?}", s))
                .fetch_all(pool)
                .await
            }
            None => {
                sqlx::query_as::<_, LeadRow>(
                    "SELECT id, lead_number, company_name, contact_name, email, phone, source, industry, estimated_value, status, assigned_to, notes, converted_to_customer, converted_at, created_at, updated_at
                     FROM leads ORDER BY created_at DESC"
                )
                .fetch_all(pool)
                .await
            }
        }.map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update_status(pool: &SqlitePool, id: Uuid, status: LeadStatus) -> Result<Lead> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE leads SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get(pool, id).await
    }

    pub async fn convert_to_customer(pool: &SqlitePool, id: Uuid, customer_id: Uuid) -> Result<Lead> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE leads SET status = 'Converted', converted_to_customer = ?, converted_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(customer_id.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get(pool, id).await
    }
}

#[derive(sqlx::FromRow)]
struct LeadRow {
    id: String,
    lead_number: String,
    company_name: String,
    contact_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    source: Option<String>,
    industry: Option<String>,
    estimated_value: i64,
    status: String,
    assigned_to: Option<String>,
    notes: Option<String>,
    converted_to_customer: Option<String>,
    converted_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<LeadRow> for Lead {
    fn from(r: LeadRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            lead_number: r.lead_number,
            company_name: r.company_name,
            contact_name: r.contact_name,
            email: r.email,
            phone: r.phone,
            source: r.source,
            industry: r.industry,
            estimated_value: r.estimated_value,
            status: match r.status.as_str() {
                "Contacted" => LeadStatus::Contacted,
                "Qualified" => LeadStatus::Qualified,
                "Unqualified" => LeadStatus::Unqualified,
                "Converted" => LeadStatus::Converted,
                "Lost" => LeadStatus::Lost,
                _ => LeadStatus::New,
            },
            assigned_to: r.assigned_to.and_then(|id| Uuid::parse_str(&id).ok()),
            notes: r.notes,
            converted_to_customer: r.converted_to_customer.and_then(|id| Uuid::parse_str(&id).ok()),
            converted_at: r.converted_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct OpportunityService;

impl OpportunityService {
    pub fn new() -> Self { Self }

    pub async fn create(
        pool: &SqlitePool,
        name: &str,
        customer_id: Option<Uuid>,
        lead_id: Option<Uuid>,
        amount: i64,
        expected_close_date: Option<&str>,
        description: Option<&str>,
        assigned_to: Option<Uuid>,
    ) -> Result<Opportunity> {
        let now = chrono::Utc::now();
        let opportunity_number = format!("OP-{}", now.format("%Y%m%d%H%M%S"));
        let opp = Opportunity {
            id: Uuid::new_v4(),
            opportunity_number: opportunity_number.clone(),
            name: name.to_string(),
            customer_id,
            lead_id,
            stage: OpportunityStage::Prospecting,
            probability: 10,
            expected_close_date: expected_close_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            amount,
            description: description.map(|s| s.to_string()),
            assigned_to,
            status: OpportunityStatus::Open,
            activities: vec![],
            created_at: now,
            updated_at: now,
        };
        
        sqlx::query(
            "INSERT INTO opportunities (id, opportunity_number, name, customer_id, lead_id, stage, probability, expected_close_date, amount, description, assigned_to, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, 'Prospecting', 10, ?, ?, ?, ?, 'Open', ?, ?)"
        )
        .bind(opp.id.to_string())
        .bind(&opp.opportunity_number)
        .bind(&opp.name)
        .bind(opp.customer_id.map(|id| id.to_string()))
        .bind(opp.lead_id.map(|id| id.to_string()))
        .bind(opp.expected_close_date.map(|d| d.to_rfc3339()))
        .bind(opp.amount)
        .bind(&opp.description)
        .bind(opp.assigned_to.map(|id| id.to_string()))
        .bind(opp.created_at.to_rfc3339())
        .bind(opp.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(opp)
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<Opportunity> {
        let row = sqlx::query_as::<_, OpportunityRow>(
            "SELECT id, opportunity_number, name, customer_id, lead_id, stage, probability, expected_close_date, amount, description, assigned_to, status, created_at, updated_at
             FROM opportunities WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Opportunity", &id.to_string()))?;
        
        let activities = Self::get_activities(pool, id).await?;
        Ok(row.into_opportunity(activities))
    }

    async fn get_activities(pool: &SqlitePool, opportunity_id: Uuid) -> Result<Vec<OpportunityActivity>> {
        let rows = sqlx::query_as::<_, ActivityRow>(
            "SELECT id, opportunity_id, activity_type, subject, description, due_date, completed, created_at
             FROM opportunity_activities WHERE opportunity_id = ? ORDER BY created_at DESC"
        )
        .bind(opportunity_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn list(pool: &SqlitePool, stage: Option<OpportunityStage>) -> Result<Vec<Opportunity>> {
        let rows = match stage {
            Some(s) => {
                sqlx::query_as::<_, OpportunityRow>(
                    "SELECT id, opportunity_number, name, customer_id, lead_id, stage, probability, expected_close_date, amount, description, assigned_to, status, created_at, updated_at
                     FROM opportunities WHERE stage = ? ORDER BY created_at DESC"
                )
                .bind(format!("{:?}", s))
                .fetch_all(pool)
                .await
            }
            None => {
                sqlx::query_as::<_, OpportunityRow>(
                    "SELECT id, opportunity_number, name, customer_id, lead_id, stage, probability, expected_close_date, amount, description, assigned_to, status, created_at, updated_at
                     FROM opportunities ORDER BY created_at DESC"
                )
                .fetch_all(pool)
                .await
            }
        }.map_err(|e| Error::Database(e))?;
        
        let mut opportunities = Vec::new();
        for row in rows {
            let activities = Self::get_activities(pool, row.id.parse().unwrap_or_default()).await?;
            opportunities.push(row.into_opportunity(activities));
        }
        Ok(opportunities)
    }

    pub async fn update_stage(pool: &SqlitePool, id: Uuid, stage: OpportunityStage) -> Result<Opportunity> {
        let now = chrono::Utc::now();
        let probability = match stage {
            OpportunityStage::Prospecting => 10,
            OpportunityStage::Qualification => 25,
            OpportunityStage::Proposal => 50,
            OpportunityStage::Negotiation => 75,
            OpportunityStage::ClosedWon => 100,
            OpportunityStage::ClosedLost => 0,
        };
        
        let status = match stage {
            OpportunityStage::ClosedWon => OpportunityStatus::Won,
            OpportunityStage::ClosedLost => OpportunityStatus::Lost,
            _ => OpportunityStatus::Open,
        };
        
        sqlx::query(
            "UPDATE opportunities SET stage = ?, probability = ?, status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", stage))
        .bind(probability)
        .bind(format!("{:?}", status))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get(pool, id).await
    }

    pub async fn add_activity(
        pool: &SqlitePool,
        opportunity_id: Uuid,
        activity_type: ActivityType,
        subject: &str,
        description: Option<&str>,
        due_date: Option<&str>,
    ) -> Result<OpportunityActivity> {
        let now = chrono::Utc::now();
        let activity = OpportunityActivity {
            id: Uuid::new_v4(),
            opportunity_id,
            activity_type: activity_type.clone(),
            subject: subject.to_string(),
            description: description.map(|s| s.to_string()),
            due_date: due_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            completed: false,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO opportunity_activities (id, opportunity_id, activity_type, subject, description, due_date, completed, created_at)
             VALUES (?, ?, ?, ?, ?, ?, 0, ?)"
        )
        .bind(activity.id.to_string())
        .bind(activity.opportunity_id.to_string())
        .bind(format!("{:?}", activity.activity_type))
        .bind(&activity.subject)
        .bind(&activity.description)
        .bind(activity.due_date.map(|d| d.to_rfc3339()))
        .bind(activity.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(activity)
    }

    pub async fn complete_activity(pool: &SqlitePool, id: Uuid) -> Result<OpportunityActivity> {
        sqlx::query(
            "UPDATE opportunity_activities SET completed = 1 WHERE id = ?"
        )
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let row = sqlx::query_as::<_, ActivityRow>(
            "SELECT id, opportunity_id, activity_type, subject, description, due_date, completed, created_at
             FROM opportunity_activities WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct OpportunityRow {
    id: String,
    opportunity_number: String,
    name: String,
    customer_id: Option<String>,
    lead_id: Option<String>,
    stage: String,
    probability: i64,
    expected_close_date: Option<String>,
    amount: i64,
    description: Option<String>,
    assigned_to: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl OpportunityRow {
    fn into_opportunity(self, activities: Vec<OpportunityActivity>) -> Opportunity {
        Opportunity {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            opportunity_number: self.opportunity_number,
            name: self.name,
            customer_id: self.customer_id.and_then(|id| Uuid::parse_str(&id).ok()),
            lead_id: self.lead_id.and_then(|id| Uuid::parse_str(&id).ok()),
            stage: match self.stage.as_str() {
                "Qualification" => OpportunityStage::Qualification,
                "Proposal" => OpportunityStage::Proposal,
                "Negotiation" => OpportunityStage::Negotiation,
                "ClosedWon" => OpportunityStage::ClosedWon,
                "ClosedLost" => OpportunityStage::ClosedLost,
                _ => OpportunityStage::Prospecting,
            },
            probability: self.probability as i32,
            expected_close_date: self.expected_close_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            amount: self.amount,
            description: self.description,
            assigned_to: self.assigned_to.and_then(|id| Uuid::parse_str(&id).ok()),
            status: match self.status.as_str() {
                "Won" => OpportunityStatus::Won,
                "Lost" => OpportunityStatus::Lost,
                _ => OpportunityStatus::Open,
            },
            activities,
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ActivityRow {
    id: String,
    opportunity_id: String,
    activity_type: String,
    subject: String,
    description: Option<String>,
    due_date: Option<String>,
    completed: i64,
    created_at: String,
}

impl From<ActivityRow> for OpportunityActivity {
    fn from(r: ActivityRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            opportunity_id: Uuid::parse_str(&r.opportunity_id).unwrap_or_default(),
            activity_type: match r.activity_type.as_str() {
                "Meeting" => ActivityType::Meeting,
                "Email" => ActivityType::Email,
                "Task" => ActivityType::Task,
                "Note" => ActivityType::Note,
                _ => ActivityType::Call,
            },
            subject: r.subject,
            description: r.description,
            due_date: r.due_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            completed: r.completed != 0,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
