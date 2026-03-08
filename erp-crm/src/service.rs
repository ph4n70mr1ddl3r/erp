use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models::{Contact, Lead, LeadStatus, Opportunity, OpportunityStage};

pub struct CrmService {
    leads: Arc<Mutex<HashMap<Uuid, Lead>>>,
    contacts: Arc<Mutex<HashMap<Uuid, Contact>>>,
    opportunities: Arc<Mutex<HashMap<Uuid, Opportunity>>>,
}

impl Default for CrmService {
    fn default() -> Self {
        Self::new()
    }
}

impl CrmService {
    pub fn new() -> Self {
        Self {
            leads: Arc::new(Mutex::new(HashMap::new())),
            contacts: Arc::new(Mutex::new(HashMap::new())),
            opportunities: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_lead(&self, first_name: &str, last_name: &str, company: &str, email: &str) -> Lead {
        let lead = Lead::new(first_name, last_name, company, email);
        self.leads.lock().unwrap().insert(lead.id, lead.clone());
        lead
    }

    pub fn get_lead(&self, id: Uuid) -> Option<Lead> {
        self.leads.lock().unwrap().get(&id).cloned()
    }

    pub fn qualify_lead(&self, id: Uuid) -> Result<Lead, &'static str> {
        let mut leads = self.leads.lock().unwrap();
        if let Some(lead) = leads.get_mut(&id) {
            if lead.status == LeadStatus::New || lead.status == LeadStatus::Contacted {
                lead.status = LeadStatus::Qualified;
                lead.updated_at = chrono::Utc::now();
                return Ok(lead.clone());
            } else {
                return Err("Lead is already qualified or lost.");
            }
        }
        Err("Lead not found.")
    }

    pub fn convert_lead(&self, id: Uuid) -> Result<Contact, &'static str> {
        let leads = self.leads.lock().unwrap();
        let lead = leads.get(&id).ok_or("Lead not found.")?;
        
        let contact = lead.convert_to_contact()?;
        self.contacts.lock().unwrap().insert(contact.id, contact.clone());
        
        Ok(contact)
    }

    pub fn create_opportunity(&self, contact_id: Uuid, name: &str, amount: f64) -> Result<Opportunity, &'static str> {
        let contacts = self.contacts.lock().unwrap();
        if !contacts.contains_key(&contact_id) {
            return Err("Contact not found.");
        }
        
        let opportunity = Opportunity::new(contact_id, name, amount);
        self.opportunities.lock().unwrap().insert(opportunity.id, opportunity.clone());
        
        Ok(opportunity)
    }

    pub fn win_opportunity(&self, id: Uuid) -> Result<Opportunity, &'static str> {
        let mut opportunities = self.opportunities.lock().unwrap();
        if let Some(opp) = opportunities.get_mut(&id) {
            opp.advance_stage(OpportunityStage::ClosedWon);
            return Ok(opp.clone());
        }
        Err("Opportunity not found.")
    }
}
