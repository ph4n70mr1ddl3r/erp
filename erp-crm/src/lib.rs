pub mod models;
pub mod service;

#[cfg(test)]
mod tests {
    use super::service::CrmService;
    use super::models::{LeadStatus, OpportunityStage};

    #[test]
    fn test_crm_lifecycle() {
        let service = CrmService::new();

        // 1. Create a Lead
        let lead = service.create_lead("John", "Doe", "Acme Corp", "john.doe@acme.com");
        assert_eq!(lead.first_name, "John");
        assert_eq!(lead.status, LeadStatus::New);

        // 2. Qualify the Lead
        let qualified_lead = service.qualify_lead(lead.id).expect("Should qualify lead");
        assert_eq!(qualified_lead.status, LeadStatus::Qualified);

        // 3. Convert Lead to Contact
        let contact = service.convert_lead(lead.id).expect("Should convert lead to contact");
        assert_eq!(contact.first_name, "John");
        assert_eq!(contact.company, "Acme Corp");
        assert_eq!(contact.lead_source_id, Some(lead.id));

        // 4. Create an Opportunity for the Contact
        let opportunity = service.create_opportunity(contact.id, "10k Widgets Deal", 10000.0)
            .expect("Should create opportunity");
        assert_eq!(opportunity.stage, OpportunityStage::Prospecting);
        assert_eq!(opportunity.amount, 10000.0);

        // 5. Win the Opportunity
        let won_opportunity = service.win_opportunity(opportunity.id)
            .expect("Should win opportunity");
        assert_eq!(won_opportunity.stage, OpportunityStage::ClosedWon);
    }
}
