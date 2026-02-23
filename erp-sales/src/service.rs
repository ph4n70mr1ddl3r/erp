use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
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

pub struct TerritoryService;

impl TerritoryService {
    pub fn new() -> Self { Self }

    pub async fn create_territory(
        pool: &SqlitePool,
        code: &str,
        name: &str,
        description: Option<&str>,
        parent_territory_id: Option<Uuid>,
        manager_id: Option<Uuid>,
        geography_type: Option<&str>,
        geography_codes: Option<&str>,
        target_revenue: i64,
    ) -> Result<SalesTerritory> {
        let now = chrono::Utc::now();
        let territory = SalesTerritory {
            id: Uuid::new_v4(),
            code: code.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            parent_territory_id,
            manager_id,
            geography_type: geography_type.map(|s| s.to_string()),
            geography_codes: geography_codes.map(|s| s.to_string()),
            target_revenue,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO sales_territories (id, code, name, description, parent_territory_id, manager_id, geography_type, geography_codes, target_revenue, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(territory.id.to_string())
        .bind(&territory.code)
        .bind(&territory.name)
        .bind(&territory.description)
        .bind(territory.parent_territory_id.map(|id| id.to_string()))
        .bind(territory.manager_id.map(|id| id.to_string()))
        .bind(&territory.geography_type)
        .bind(&territory.geography_codes)
        .bind(territory.target_revenue)
        .bind(territory.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(territory)
    }

    pub async fn assign_territory(
        pool: &SqlitePool,
        territory_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
        sales_rep_id: Uuid,
        effective_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
        is_primary: bool,
    ) -> Result<TerritoryAssignment> {
        let now = chrono::Utc::now();
        let assignment = TerritoryAssignment {
            id: Uuid::new_v4(),
            territory_id,
            entity_type: entity_type.to_string(),
            entity_id,
            sales_rep_id,
            effective_date,
            end_date,
            is_primary,
        };
        
        sqlx::query(
            "INSERT INTO territory_assignments (id, territory_id, entity_type, entity_id, sales_rep_id, effective_date, end_date, is_primary)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(assignment.id.to_string())
        .bind(assignment.territory_id.to_string())
        .bind(&assignment.entity_type)
        .bind(assignment.entity_id.to_string())
        .bind(assignment.sales_rep_id.to_string())
        .bind(assignment.effective_date.to_rfc3339())
        .bind(assignment.end_date.map(|d| d.to_rfc3339()))
        .bind(assignment.is_primary as i32)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(assignment)
    }

    pub async fn get_territory_performance(pool: &SqlitePool, territory_id: Uuid) -> Result<TerritoryPerformance> {
        let row = sqlx::query_as::<_, TerritoryPerfRow>(
            "SELECT t.id, t.code, t.name, t.target_revenue,
                    COALESCE(SUM(so.total), 0) as actual_revenue,
                    COUNT(DISTINCT so.id) as order_count
             FROM sales_territories t
             LEFT JOIN territory_assignments ta ON t.id = ta.territory_id
             LEFT JOIN sales_orders so ON ta.entity_id = so.customer_id AND so.status = 'Completed'
             WHERE t.id = ?
             GROUP BY t.id"
        )
        .bind(territory_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Territory", &territory_id.to_string()))?;
        
        Ok(TerritoryPerformance {
            territory_id: Uuid::parse_str(&row.id).unwrap_or_default(),
            code: row.code,
            name: row.name,
            target_revenue: row.target_revenue,
            actual_revenue: row.actual_revenue,
            order_count: row.order_count,
        })
    }
}

#[derive(sqlx::FromRow)]
struct TerritoryPerfRow {
    id: String,
    code: String,
    name: String,
    target_revenue: i64,
    actual_revenue: i64,
    order_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerritoryPerformance {
    pub territory_id: Uuid,
    pub code: String,
    pub name: String,
    pub target_revenue: i64,
    pub actual_revenue: i64,
    pub order_count: i64,
}

pub struct CommissionService;

impl CommissionService {
    pub fn new() -> Self { Self }

    pub async fn create_plan(
        pool: &SqlitePool,
        name: &str,
        description: Option<&str>,
        plan_type: CommissionPlanType,
        effective_date: DateTime<Utc>,
        expiry_date: Option<DateTime<Utc>>,
    ) -> Result<CommissionPlan> {
        let now = chrono::Utc::now();
        let plan = CommissionPlan {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            plan_type: plan_type.clone(),
            effective_date,
            expiry_date,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO commission_plans (id, name, description, plan_type, effective_date, expiry_date, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(plan.id.to_string())
        .bind(&plan.name)
        .bind(&plan.description)
        .bind(format!("{:?}", plan_type))
        .bind(plan.effective_date.to_rfc3339())
        .bind(plan.expiry_date.map(|d| d.to_rfc3339()))
        .bind(plan.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(plan)
    }

    pub async fn add_tier(
        pool: &SqlitePool,
        plan_id: Uuid,
        tier_number: i32,
        min_amount: i64,
        max_amount: Option<i64>,
        rate_percent: f64,
    ) -> Result<CommissionTier> {
        let tier = CommissionTier {
            id: Uuid::new_v4(),
            plan_id,
            tier_number,
            min_amount,
            max_amount,
            rate_percent,
        };
        
        sqlx::query(
            "INSERT INTO commission_tiers (id, plan_id, tier_number, min_amount, max_amount, rate_percent)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(tier.id.to_string())
        .bind(tier.plan_id.to_string())
        .bind(tier.tier_number)
        .bind(tier.min_amount)
        .bind(tier.max_amount)
        .bind(tier.rate_percent)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(tier)
    }

    pub async fn calculate_commission(
        pool: &SqlitePool,
        sales_rep_id: Uuid,
        plan_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<SalesRepCommission> {
        let sales_row = sqlx::query_as::<_, SalesSummaryRow>(
            "SELECT COALESCE(SUM(total), 0) as gross_sales, 
                    COALESCE(SUM(CASE WHEN status = 'Cancelled' THEN total ELSE 0 END), 0) as returns
             FROM sales_orders 
             WHERE assigned_to = ? AND order_date >= ? AND order_date <= ?"
        )
        .bind(sales_rep_id.to_string())
        .bind(period_start.to_rfc3339())
        .bind(period_end.to_rfc3339())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let gross_sales = sales_row.gross_sales;
        let returns = sales_row.returns;
        let net_sales = gross_sales - returns;
        
        let tier_row = sqlx::query_as::<_, TierRow>(
            "SELECT rate_percent FROM commission_tiers 
             WHERE plan_id = ? AND min_amount <= ? AND (max_amount IS NULL OR max_amount >= ?)
             ORDER BY tier_number DESC LIMIT 1"
        )
        .bind(plan_id.to_string())
        .bind(net_sales)
        .bind(net_sales)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let rate = tier_row.map(|t| t.rate_percent).unwrap_or(5.0);
        let commission_amount = (net_sales as f64 * rate / 100.0) as i64;
        
        let now = chrono::Utc::now();
        let commission = SalesRepCommission {
            id: Uuid::new_v4(),
            sales_rep_id,
            plan_id,
            period_start,
            period_end,
            gross_sales,
            returns,
            net_sales,
            commission_rate: rate,
            commission_amount,
            status: CommissionStatus::Calculated,
            paid_at: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO sales_rep_commissions (id, sales_rep_id, plan_id, period_start, period_end, gross_sales, returns, net_sales, commission_rate, commission_amount, status, paid_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Calculated', NULL, ?)"
        )
        .bind(commission.id.to_string())
        .bind(commission.sales_rep_id.to_string())
        .bind(commission.plan_id.to_string())
        .bind(commission.period_start.to_rfc3339())
        .bind(commission.period_end.to_rfc3339())
        .bind(commission.gross_sales)
        .bind(commission.returns)
        .bind(commission.net_sales)
        .bind(commission.commission_rate)
        .bind(commission.commission_amount)
        .bind(commission.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(commission)
    }

    pub async fn process_commission(pool: &SqlitePool, commission_id: Uuid, approve: bool) -> Result<SalesRepCommission> {
        let now = chrono::Utc::now();
        let status = if approve { CommissionStatus::Approved } else { CommissionStatus::Adjusted };
        
        sqlx::query(
            "UPDATE sales_rep_commissions SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(now.to_rfc3339())
        .bind(commission_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let row = sqlx::query_as::<_, CommissionRow>(
            "SELECT id, sales_rep_id, plan_id, period_start, period_end, gross_sales, returns, net_sales, commission_rate, commission_amount, status, paid_at, created_at
             FROM sales_rep_commissions WHERE id = ?"
        )
        .bind(commission_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct SalesSummaryRow {
    gross_sales: i64,
    returns: i64,
}

#[derive(sqlx::FromRow)]
struct TierRow {
    rate_percent: f64,
}

#[derive(sqlx::FromRow)]
struct CommissionRow {
    id: String,
    sales_rep_id: String,
    plan_id: String,
    period_start: String,
    period_end: String,
    gross_sales: i64,
    returns: i64,
    net_sales: i64,
    commission_rate: f64,
    commission_amount: i64,
    status: String,
    paid_at: Option<String>,
    created_at: String,
}

impl From<CommissionRow> for SalesRepCommission {
    fn from(r: CommissionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            sales_rep_id: Uuid::parse_str(&r.sales_rep_id).unwrap_or_default(),
            plan_id: Uuid::parse_str(&r.plan_id).unwrap_or_default(),
            period_start: chrono::DateTime::parse_from_rfc3339(&r.period_start)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            period_end: chrono::DateTime::parse_from_rfc3339(&r.period_end)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            gross_sales: r.gross_sales,
            returns: r.returns,
            net_sales: r.net_sales,
            commission_rate: r.commission_rate,
            commission_amount: r.commission_amount,
            status: match r.status.as_str() {
                "Approved" => CommissionStatus::Approved,
                "Paid" => CommissionStatus::Paid,
                "Adjusted" => CommissionStatus::Adjusted,
                _ => CommissionStatus::Calculated,
            },
            paid_at: r.paid_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct ContractService;

impl ContractService {
    pub fn new() -> Self { Self }

    pub async fn create_contract(
        pool: &SqlitePool,
        title: &str,
        customer_id: Uuid,
        contract_type: ContractType,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        value: i64,
        currency: &str,
        billing_cycle: Option<BillingCycle>,
        auto_renew: bool,
        renewal_notice_days: i32,
        terms: Option<&str>,
    ) -> Result<Contract> {
        let now = chrono::Utc::now();
        let contract_number = format!("CTR-{}", now.format("%Y%m%d%H%M%S"));
        let contract = Contract {
            id: Uuid::new_v4(),
            contract_number: contract_number.clone(),
            title: title.to_string(),
            customer_id,
            contract_type: contract_type.clone(),
            start_date,
            end_date,
            value,
            currency: currency.to_string(),
            billing_cycle: billing_cycle.clone(),
            auto_renew,
            renewal_notice_days,
            terms: terms.map(|s| s.to_string()),
            status: ContractStatus::Draft,
            signed_date: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO contracts (id, contract_number, title, customer_id, contract_type, start_date, end_date, value, currency, billing_cycle, auto_renew, renewal_notice_days, terms, status, signed_date, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Draft', NULL, ?)"
        )
        .bind(contract.id.to_string())
        .bind(&contract.contract_number)
        .bind(&contract.title)
        .bind(contract.customer_id.to_string())
        .bind(format!("{:?}", contract_type))
        .bind(contract.start_date.to_rfc3339())
        .bind(contract.end_date.to_rfc3339())
        .bind(contract.value)
        .bind(&contract.currency)
        .bind(contract.billing_cycle.clone().map(|bc| format!("{:?}", bc)))
        .bind(contract.auto_renew as i32)
        .bind(contract.renewal_notice_days)
        .bind(&contract.terms)
        .bind(contract.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(contract)
    }

    pub async fn add_line(
        pool: &SqlitePool,
        contract_id: Uuid,
        product_id: Option<Uuid>,
        description: &str,
        quantity: i64,
        unit_price: i64,
        billing_type: BillingType,
        billing_frequency: Option<BillingFrequency>,
        next_billing_date: Option<DateTime<Utc>>,
    ) -> Result<ContractLine> {
        let line = ContractLine {
            id: Uuid::new_v4(),
            contract_id,
            product_id,
            description: description.to_string(),
            quantity,
            unit_price,
            billing_type: billing_type.clone(),
            billing_frequency: billing_frequency.clone(),
            next_billing_date,
            status: ContractLineStatus::Active,
        };
        
        sqlx::query(
            "INSERT INTO contract_lines (id, contract_id, product_id, description, quantity, unit_price, billing_type, billing_frequency, next_billing_date, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'Active')"
        )
        .bind(line.id.to_string())
        .bind(line.contract_id.to_string())
        .bind(line.product_id.map(|id| id.to_string()))
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price)
        .bind(format!("{:?}", billing_type))
        .bind(line.billing_frequency.clone().map(|bf| format!("{:?}", bf)))
        .bind(line.next_billing_date.map(|d| d.to_rfc3339()))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(line)
    }

    pub async fn renew_contract(pool: &SqlitePool, contract_id: Uuid, new_end_date: DateTime<Utc>, new_value: Option<i64>) -> Result<ContractRenewal> {
        let now = chrono::Utc::now();
        let contract = Self::get_contract(pool, contract_id).await?;
        
        let renewal = ContractRenewal {
            id: Uuid::new_v4(),
            contract_id,
            renewal_date: now,
            new_start_date: contract.end_date,
            new_end_date,
            new_value,
            status: RenewalStatus::Pending,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO contract_renewals (id, contract_id, renewal_date, new_start_date, new_end_date, new_value, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, 'Pending', ?)"
        )
        .bind(renewal.id.to_string())
        .bind(renewal.contract_id.to_string())
        .bind(renewal.renewal_date.to_rfc3339())
        .bind(renewal.new_start_date.to_rfc3339())
        .bind(renewal.new_end_date.to_rfc3339())
        .bind(renewal.new_value)
        .bind(renewal.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE contracts SET end_date = ?, value = COALESCE(?, value), status = 'Active' WHERE id = ?"
        )
        .bind(new_end_date.to_rfc3339())
        .bind(new_value)
        .bind(contract_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(renewal)
    }

    pub async fn terminate_contract(pool: &SqlitePool, contract_id: Uuid, reason: Option<&str>) -> Result<Contract> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE contracts SET status = 'Terminated', terms = COALESCE(terms || ?) WHERE id = ?"
        )
        .bind(reason.map(|r| format!("\nTermination reason: {}", r)))
        .bind(contract_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE contract_lines SET status = 'Cancelled' WHERE contract_id = ?"
        )
        .bind(contract_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_contract(pool, contract_id).await
    }

    pub async fn get_expiring_contracts(pool: &SqlitePool, days: i32) -> Result<Vec<Contract>> {
        let rows = sqlx::query_as::<_, ContractRow>(
            "SELECT id, contract_number, title, customer_id, contract_type, start_date, end_date, value, currency, billing_cycle, auto_renew, renewal_notice_days, terms, status, signed_date, created_at
             FROM contracts 
             WHERE status = 'Active' AND date(end_date) <= date('now', '+' || ? || ' days')
             ORDER BY end_date ASC"
        )
        .bind(days.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn get_contract(pool: &SqlitePool, id: Uuid) -> Result<Contract> {
        let row = sqlx::query_as::<_, ContractRow>(
            "SELECT id, contract_number, title, customer_id, contract_type, start_date, end_date, value, currency, billing_cycle, auto_renew, renewal_notice_days, terms, status, signed_date, created_at
             FROM contracts WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Contract", &id.to_string()))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct ContractRow {
    id: String,
    contract_number: String,
    title: String,
    customer_id: String,
    contract_type: String,
    start_date: String,
    end_date: String,
    value: i64,
    currency: String,
    billing_cycle: Option<String>,
    auto_renew: i32,
    renewal_notice_days: i32,
    terms: Option<String>,
    status: String,
    signed_date: Option<String>,
    created_at: String,
}

impl From<ContractRow> for Contract {
    fn from(r: ContractRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            contract_number: r.contract_number,
            title: r.title,
            customer_id: Uuid::parse_str(&r.customer_id).unwrap_or_default(),
            contract_type: match r.contract_type.as_str() {
                "TimeAndMaterials" => ContractType::TimeAndMaterials,
                "Subscription" => ContractType::Subscription,
                "Milestone" => ContractType::Milestone,
                _ => ContractType::Fixed,
            },
            start_date: chrono::DateTime::parse_from_rfc3339(&r.start_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            end_date: chrono::DateTime::parse_from_rfc3339(&r.end_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            value: r.value,
            currency: r.currency,
            billing_cycle: r.billing_cycle.and_then(|bc| match bc.as_str() {
                "Quarterly" => Some(BillingCycle::Quarterly),
                "Annually" => Some(BillingCycle::Annually),
                "OneTime" => Some(BillingCycle::OneTime),
                _ => Some(BillingCycle::Monthly),
            }),
            auto_renew: r.auto_renew != 0,
            renewal_notice_days: r.renewal_notice_days,
            terms: r.terms,
            status: match r.status.as_str() {
                "Pending" => ContractStatus::Pending,
                "Active" => ContractStatus::Active,
                "Expired" => ContractStatus::Expired,
                "Terminated" => ContractStatus::Terminated,
                _ => ContractStatus::Draft,
            },
            signed_date: r.signed_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct SubscriptionService;

impl SubscriptionService {
    pub fn new() -> Self { Self }

    pub async fn create_plan(
        pool: &SqlitePool,
        plan_code: &str,
        name: &str,
        description: Option<&str>,
        billing_cycle: BillingCycleInterval,
        billing_interval: i32,
        setup_fee: i64,
        base_price: i64,
        trial_days: i32,
        features: Option<&str>,
    ) -> Result<SubscriptionPlan> {
        let now = chrono::Utc::now();
        let plan = SubscriptionPlan {
            id: Uuid::new_v4(),
            plan_code: plan_code.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            billing_cycle: billing_cycle.clone(),
            billing_interval,
            setup_fee,
            base_price,
            trial_days,
            features: features.map(|s| s.to_string()),
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO subscription_plans (id, plan_code, name, description, billing_cycle, billing_interval, setup_fee, base_price, trial_days, features, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(plan.id.to_string())
        .bind(&plan.plan_code)
        .bind(&plan.name)
        .bind(&plan.description)
        .bind(format!("{:?}", billing_cycle))
        .bind(plan.billing_interval)
        .bind(plan.setup_fee)
        .bind(plan.base_price)
        .bind(plan.trial_days)
        .bind(&plan.features)
        .bind(plan.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(plan)
    }

    pub async fn create_subscription(
        pool: &SqlitePool,
        customer_id: Uuid,
        plan_id: Uuid,
        contract_id: Option<Uuid>,
        quantity: i32,
        price_override: Option<i64>,
    ) -> Result<Subscription> {
        let now = chrono::Utc::now();
        let subscription_number = format!("SUB-{}", now.format("%Y%m%d%H%M%S"));
        
        let plan_row = sqlx::query_as::<_, PlanRow>(
            "SELECT billing_interval, trial_days FROM subscription_plans WHERE id = ?"
        )
        .bind(plan_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let period_end = now + chrono::Duration::days((plan_row.billing_interval * 30) as i64);
        let status = if plan_row.trial_days > 0 {
            SubscriptionStatus::Trial
        } else {
            SubscriptionStatus::Active
        };
        
        let subscription = Subscription {
            id: Uuid::new_v4(),
            subscription_number: subscription_number.clone(),
            customer_id,
            plan_id,
            contract_id,
            start_date: now,
            end_date: None,
            current_period_start: now,
            current_period_end: period_end,
            quantity,
            price_override,
            status: status.clone(),
            cancelled_at: None,
            cancellation_reason: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO subscriptions (id, subscription_number, customer_id, plan_id, contract_id, start_date, end_date, current_period_start, current_period_end, quantity, price_override, status, cancelled_at, cancellation_reason, created_at)
             VALUES (?, ?, ?, ?, ?, NULL, ?, ?, ?, ?, ?, ?, NULL, NULL, ?)"
        )
        .bind(subscription.id.to_string())
        .bind(&subscription.subscription_number)
        .bind(subscription.customer_id.to_string())
        .bind(subscription.plan_id.to_string())
        .bind(subscription.contract_id.map(|id| id.to_string()))
        .bind(subscription.start_date.to_rfc3339())
        .bind(subscription.current_period_start.to_rfc3339())
        .bind(subscription.current_period_end.to_rfc3339())
        .bind(subscription.quantity)
        .bind(subscription.price_override)
        .bind(format!("{:?}", status))
        .bind(subscription.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(subscription)
    }

    pub async fn record_usage(
        pool: &SqlitePool,
        subscription_id: Uuid,
        usage_type: UsageType,
        quantity: i64,
        unit_price: Option<i64>,
    ) -> Result<SubscriptionUsage> {
        let now = chrono::Utc::now();
        let total_amount = unit_price.map(|up| quantity * up);
        
        let usage = SubscriptionUsage {
            id: Uuid::new_v4(),
            subscription_id,
            usage_date: now,
            usage_type: usage_type.clone(),
            quantity,
            unit_price,
            total_amount,
            invoice_id: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO subscription_usage (id, subscription_id, usage_date, usage_type, quantity, unit_price, total_amount, invoice_id, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, NULL, ?)"
        )
        .bind(usage.id.to_string())
        .bind(usage.subscription_id.to_string())
        .bind(usage.usage_date.to_rfc3339())
        .bind(format!("{:?}", usage_type))
        .bind(usage.quantity)
        .bind(usage.unit_price)
        .bind(usage.total_amount)
        .bind(usage.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(usage)
    }

    pub async fn generate_invoice(pool: &SqlitePool, subscription_id: Uuid) -> Result<SubscriptionInvoice> {
        let now = chrono::Utc::now();
        
        let sub_row = sqlx::query_as::<_, SubRow>(
            "SELECT s.current_period_start, s.current_period_end, s.quantity, s.price_override, p.base_price
             FROM subscriptions s
             JOIN subscription_plans p ON s.plan_id = p.id
             WHERE s.id = ?"
        )
        .bind(subscription_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let base_amount = sub_row.price_override.unwrap_or(sub_row.base_price) * sub_row.quantity as i64;
        
        let usage_row = sqlx::query_as::<_, UsageSumRow>(
            "SELECT COALESCE(SUM(total_amount), 0) as total_usage
             FROM subscription_usage
             WHERE subscription_id = ? AND invoice_id IS NULL"
        )
        .bind(subscription_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let subtotal = base_amount + usage_row.total_usage;
        let tax_amount = (subtotal as f64 * 0.1) as i64;
        let total = subtotal + tax_amount;
        
        let invoice_number = format!("INV-{}", now.format("%Y%m%d%H%M%S"));
        let due_date = now + chrono::Duration::days(30);
        
        let invoice = SubscriptionInvoice {
            id: Uuid::new_v4(),
            invoice_number: invoice_number.clone(),
            subscription_id,
            invoice_date: now,
            due_date,
            period_start: chrono::DateTime::parse_from_rfc3339(&sub_row.current_period_start)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            period_end: chrono::DateTime::parse_from_rfc3339(&sub_row.current_period_end)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            subtotal,
            tax_amount,
            total,
            amount_paid: 0,
            status: SubscriptionInvoiceStatus::Draft,
            paid_at: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO subscription_invoices (id, invoice_number, subscription_id, invoice_date, due_date, period_start, period_end, subtotal, tax_amount, total, amount_paid, status, paid_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 'Draft', NULL, ?)"
        )
        .bind(invoice.id.to_string())
        .bind(&invoice.invoice_number)
        .bind(invoice.subscription_id.to_string())
        .bind(invoice.invoice_date.to_rfc3339())
        .bind(invoice.due_date.to_rfc3339())
        .bind(invoice.period_start.to_rfc3339())
        .bind(invoice.period_end.to_rfc3339())
        .bind(invoice.subtotal)
        .bind(invoice.tax_amount)
        .bind(invoice.total)
        .bind(invoice.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE subscription_usage SET invoice_id = ? WHERE subscription_id = ? AND invoice_id IS NULL"
        )
        .bind(invoice.id.to_string())
        .bind(subscription_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(invoice)
    }

    pub async fn cancel_subscription(pool: &SqlitePool, subscription_id: Uuid, reason: Option<&str>) -> Result<Subscription> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE subscriptions SET status = 'Cancelled', cancelled_at = ?, cancellation_reason = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(reason)
        .bind(subscription_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_subscription(pool, subscription_id).await
    }

    pub async fn reactivate_subscription(pool: &SqlitePool, subscription_id: Uuid) -> Result<Subscription> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE subscriptions SET status = 'Active', cancelled_at = NULL, cancellation_reason = NULL WHERE id = ?"
        )
        .bind(subscription_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_subscription(pool, subscription_id).await
    }

    async fn get_subscription(pool: &SqlitePool, id: Uuid) -> Result<Subscription> {
        let row = sqlx::query_as::<_, SubscriptionRow>(
            "SELECT id, subscription_number, customer_id, plan_id, contract_id, start_date, end_date, current_period_start, current_period_end, quantity, price_override, status, cancelled_at, cancellation_reason, created_at
             FROM subscriptions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Subscription", &id.to_string()))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct PlanRow {
    billing_interval: i32,
    trial_days: i32,
}

#[derive(sqlx::FromRow)]
struct SubRow {
    current_period_start: String,
    current_period_end: String,
    quantity: i32,
    price_override: Option<i64>,
    base_price: i64,
}

#[derive(sqlx::FromRow)]
struct UsageSumRow {
    total_usage: i64,
}

#[derive(sqlx::FromRow)]
struct SubscriptionRow {
    id: String,
    subscription_number: String,
    customer_id: String,
    plan_id: String,
    contract_id: Option<String>,
    start_date: String,
    end_date: Option<String>,
    current_period_start: String,
    current_period_end: String,
    quantity: i32,
    price_override: Option<i64>,
    status: String,
    cancelled_at: Option<String>,
    cancellation_reason: Option<String>,
    created_at: String,
}

impl From<SubscriptionRow> for Subscription {
    fn from(r: SubscriptionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            subscription_number: r.subscription_number,
            customer_id: Uuid::parse_str(&r.customer_id).unwrap_or_default(),
            plan_id: Uuid::parse_str(&r.plan_id).unwrap_or_default(),
            contract_id: r.contract_id.and_then(|id| Uuid::parse_str(&id).ok()),
            start_date: chrono::DateTime::parse_from_rfc3339(&r.start_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            end_date: r.end_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            current_period_start: chrono::DateTime::parse_from_rfc3339(&r.current_period_start)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            current_period_end: chrono::DateTime::parse_from_rfc3339(&r.current_period_end)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            quantity: r.quantity,
            price_override: r.price_override,
            status: match r.status.as_str() {
                "Active" => SubscriptionStatus::Active,
                "PastDue" => SubscriptionStatus::PastDue,
                "Suspended" => SubscriptionStatus::Suspended,
                "Cancelled" => SubscriptionStatus::Cancelled,
                "Expired" => SubscriptionStatus::Expired,
                _ => SubscriptionStatus::Trial,
            },
            cancelled_at: r.cancelled_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            cancellation_reason: r.cancellation_reason,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct CampaignService;

impl CampaignService {
    pub fn new() -> Self { Self }

    pub async fn create_campaign(
        pool: &SqlitePool,
        name: &str,
        description: Option<&str>,
        campaign_type: CampaignType,
        channel: CampaignChannel,
        start_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
        budget: i64,
        target_audience: Option<&str>,
        objectives: Option<&str>,
    ) -> Result<MarketingCampaign> {
        let now = chrono::Utc::now();
        let campaign_code = format!("CMP-{}", now.format("%Y%m%d%H%M%S"));
        let campaign = MarketingCampaign {
            id: Uuid::new_v4(),
            campaign_code: campaign_code.clone(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            campaign_type: campaign_type.clone(),
            channel: channel.clone(),
            start_date,
            end_date,
            budget,
            actual_spend: 0,
            target_audience: target_audience.map(|s| s.to_string()),
            objectives: objectives.map(|s| s.to_string()),
            status: CampaignStatus::Draft,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO marketing_campaigns (id, campaign_code, name, description, campaign_type, channel, start_date, end_date, budget, actual_spend, target_audience, objectives, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, 'Draft', ?)"
        )
        .bind(campaign.id.to_string())
        .bind(&campaign.campaign_code)
        .bind(&campaign.name)
        .bind(&campaign.description)
        .bind(format!("{:?}", campaign_type))
        .bind(format!("{:?}", channel))
        .bind(campaign.start_date.to_rfc3339())
        .bind(campaign.end_date.map(|d| d.to_rfc3339()))
        .bind(campaign.budget)
        .bind(&campaign.target_audience)
        .bind(&campaign.objectives)
        .bind(campaign.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(campaign)
    }

    pub async fn add_lead(pool: &SqlitePool, campaign_id: Uuid, lead_id: Uuid) -> Result<CampaignLead> {
        let campaign_lead = CampaignLead {
            id: Uuid::new_v4(),
            campaign_id,
            lead_id,
            responded_at: None,
            response_type: None,
            converted: false,
            conversion_value: None,
        };
        
        sqlx::query(
            "INSERT INTO campaign_leads (id, campaign_id, lead_id, responded_at, response_type, converted, conversion_value)
             VALUES (?, ?, ?, NULL, NULL, 0, NULL)"
        )
        .bind(campaign_lead.id.to_string())
        .bind(campaign_lead.campaign_id.to_string())
        .bind(campaign_lead.lead_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(campaign_lead)
    }

    pub async fn track_response(
        pool: &SqlitePool,
        campaign_id: Uuid,
        lead_id: Uuid,
        response_type: ResponseType,
        converted: bool,
        conversion_value: Option<i64>,
    ) -> Result<CampaignLead> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE campaign_leads SET responded_at = ?, response_type = ?, converted = ?, conversion_value = ? WHERE campaign_id = ? AND lead_id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(format!("{:?}", response_type))
        .bind(converted as i32)
        .bind(conversion_value)
        .bind(campaign_id.to_string())
        .bind(lead_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let row = sqlx::query_as::<_, CampaignLeadRow>(
            "SELECT id, campaign_id, lead_id, responded_at, response_type, converted, conversion_value FROM campaign_leads WHERE campaign_id = ? AND lead_id = ?"
        )
        .bind(campaign_id.to_string())
        .bind(lead_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(row.into())
    }

    pub async fn calculate_roi(pool: &SqlitePool, campaign_id: Uuid) -> Result<CampaignROI> {
        let row = sqlx::query_as::<_, CampaignStatsRow>(
            "SELECT c.budget, c.actual_spend,
                    COUNT(cl.id) as lead_count,
                    SUM(CASE WHEN cl.responded_at IS NOT NULL THEN 1 ELSE 0 END) as response_count,
                    SUM(CASE WHEN cl.converted = 1 THEN 1 ELSE 0 END) as conversion_count,
                    COALESCE(SUM(cl.conversion_value), 0) as total_revenue
             FROM marketing_campaigns c
             LEFT JOIN campaign_leads cl ON c.id = cl.campaign_id
             WHERE c.id = ?
             GROUP BY c.id"
        )
        .bind(campaign_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Campaign", &campaign_id.to_string()))?;
        
        let spend = if row.actual_spend > 0 { row.actual_spend } else { row.budget };
        let roi = if spend > 0 {
            ((row.total_revenue - spend) as f64 / spend as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(CampaignROI {
            campaign_id,
            budget: row.budget,
            actual_spend: row.actual_spend,
            lead_count: row.lead_count,
            response_count: row.response_count,
            conversion_count: row.conversion_count,
            total_revenue: row.total_revenue,
            roi_percent: roi,
        })
    }
}

#[derive(sqlx::FromRow)]
struct CampaignLeadRow {
    id: String,
    campaign_id: String,
    lead_id: String,
    responded_at: Option<String>,
    response_type: Option<String>,
    converted: i32,
    conversion_value: Option<i64>,
}

impl From<CampaignLeadRow> for CampaignLead {
    fn from(r: CampaignLeadRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            campaign_id: Uuid::parse_str(&r.campaign_id).unwrap_or_default(),
            lead_id: Uuid::parse_str(&r.lead_id).unwrap_or_default(),
            responded_at: r.responded_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            response_type: r.response_type.and_then(|rt| match rt.as_str() {
                "Clicked" => Some(ResponseType::Clicked),
                "Replied" => Some(ResponseType::Replied),
                "Bounced" => Some(ResponseType::Bounced),
                "Unsubscribed" => Some(ResponseType::Unsubscribed),
                "Converted" => Some(ResponseType::Converted),
                _ => Some(ResponseType::Opened),
            }),
            converted: r.converted != 0,
            conversion_value: r.conversion_value,
        }
    }
}

#[derive(sqlx::FromRow)]
struct CampaignStatsRow {
    budget: i64,
    actual_spend: i64,
    lead_count: i64,
    response_count: i64,
    conversion_count: i64,
    total_revenue: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignROI {
    pub campaign_id: Uuid,
    pub budget: i64,
    pub actual_spend: i64,
    pub lead_count: i64,
    pub response_count: i64,
    pub conversion_count: i64,
    pub total_revenue: i64,
    pub roi_percent: f64,
}
