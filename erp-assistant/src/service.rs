use anyhow::Result;
use uuid::Uuid;
use erp_core::models::BaseEntity;
use crate::models::*;
use crate::repository::{AssistantRepository, SqliteAssistantRepository};
use regex::Regex;
use std::collections::HashMap;

pub struct AssistantService {
    repo: SqliteAssistantRepository,
}

impl AssistantService {
    pub fn new() -> Self {
        Self {
            repo: SqliteAssistantRepository::new(),
        }
    }
    
    pub async fn create_conversation(&self, user_id: Uuid, request: CreateConversationRequest) -> Result<AssistantConversation> {
        let conversation = AssistantConversation {
            base: BaseEntity::new(),
            user_id,
            title: request.title.unwrap_or_else(|| "New Conversation".to_string()),
            context: serde_json::json!({}),
            status: ConversationStatus::Active,
        };
        self.repo.create_conversation(&conversation).await?;
        Ok(conversation)
    }
    
    pub async fn get_conversation(&self, id: Uuid) -> Result<Option<AssistantConversation>> {
        self.repo.get_conversation(id).await
    }
    
    pub async fn list_conversations(&self, user_id: Uuid, limit: i64) -> Result<Vec<AssistantConversation>> {
        self.repo.list_conversations(user_id, limit).await
    }
    
    pub async fn archive_conversation(&self, id: Uuid) -> Result<()> {
        self.repo.archive_conversation(id).await
    }
    
    pub async fn send_message(&self, user_id: Uuid, request: SendMessageRequest) -> Result<AssistantResponse> {
        let user_message = ConversationMessage {
            base: BaseEntity::new(),
            conversation_id: request.conversation_id,
            role: MessageRole::User,
            content: request.message.clone(),
            intent: None,
            entities: serde_json::json!({}),
            action_taken: None,
            action_result: None,
            feedback: None,
        };
        self.repo.create_message(&user_message).await?;
        
        let parsed = self.parse_query(&request.message)?;
        let response = self.generate_response(&parsed, &request.context).await?;
        
        let assistant_message = ConversationMessage {
            base: BaseEntity::new(),
            conversation_id: request.conversation_id,
            role: MessageRole::Assistant,
            content: response.message.clone(),
            intent: Some(parsed.intent.clone()),
            entities: serde_json::to_value(&parsed.entities)?,
            action_taken: response.action.as_ref().map(|a| a.action_type.clone()),
            action_result: response.data.clone(),
            feedback: None,
        };
        self.repo.create_message(&assistant_message).await?;
        
        Ok(response)
    }
    
    pub async fn provide_feedback(&self, request: MessageFeedbackRequest) -> Result<()> {
        let feedback = MessageFeedback {
            rating: request.rating,
            comment: request.comment,
            created_at: chrono::Utc::now(),
        };
        self.repo.update_message_feedback(request.message_id, feedback).await
    }
    
    pub fn parse_query(&self, query: &str) -> Result<ParsedQuery> {
        let normalized = self.normalize_query(query);
        let (intent, confidence) = self.detect_intent(&normalized)?;
        let entities = self.extract_entities(&normalized, &intent)?;
        
        Ok(ParsedQuery {
            intent,
            confidence,
            entities,
            original_query: query.to_string(),
            normalized_query: normalized,
        })
    }
    
    fn normalize_query(&self, query: &str) -> String {
        query.to_lowercase().trim().to_string()
    }
    
    fn detect_intent(&self, query: &str) -> Result<(String, f64)> {
        let intent_patterns = vec![
            (r"show|list|get|display|what (are|is)", "query_data"),
            (r"create|add|new|make", "create_entity"),
            (r"update|change|modify|edit", "update_entity"),
            (r"delete|remove|cancel", "delete_entity"),
            (r"how many|count|total", "aggregate_count"),
            (r"sum|total amount|total value", "aggregate_sum"),
            (r"average|avg|mean", "aggregate_average"),
            (r"trend|over time|history", "trend_analysis"),
            (r"compare|versus|vs|difference", "comparison"),
            (r"forecast|predict|estimate", "forecast"),
            (r"help|how (do|to)|explain", "help"),
            (r"status|progress|where", "status_check"),
        ];
        
        for (pattern, intent) in intent_patterns {
            if Regex::new(pattern)?.is_match(query) {
                return Ok((intent.to_string(), 0.85));
            }
        }
        
        Ok(("unknown".to_string(), 0.3))
    }
    
    fn extract_entities(&self, query: &str, intent: &str) -> Result<Vec<ExtractedEntity>> {
        let mut entities = Vec::new();
        
        let date_patterns = vec![
            (r"\b(\d{4}-\d{2}-\d{2})\b", "date"),
            (r"\b(today|yesterday|tomorrow)\b", "relative_date"),
            (r"\b(last|this|next)\s+(week|month|quarter|year)\b", "period"),
        ];
        
        for (pattern, entity_type) in date_patterns {
            if let Ok(re) = Regex::new(pattern) {
                for cap in re.captures_iter(query) {
                    entities.push(ExtractedEntity {
                        name: entity_type.to_string(),
                        value: cap[0].to_string(),
                        entity_type: entity_type.to_string(),
                        confidence: 0.9,
                        position: (0, 0),
                    });
                }
            }
        }
        
        let entity_keywords = vec![
            ("invoice", "Invoice"),
            ("order", "SalesOrder"),
            ("purchase", "PurchaseOrder"),
            ("customer", "Customer"),
            ("vendor", "Vendor"),
            ("product", "Product"),
            ("employee", "Employee"),
            ("account", "Account"),
        ];
        
        for (keyword, entity_type) in entity_keywords {
            if query.contains(keyword) {
                entities.push(ExtractedEntity {
                    name: "entity_type".to_string(),
                    value: entity_type.to_string(),
                    entity_type: "EntityType".to_string(),
                    confidence: 0.85,
                    position: (0, 0),
                });
                break;
            }
        }
        
        if let Ok(re) = Regex::new(r"\b(\d+(?:\.\d+)?)\b") {
            if let Some(cap) = re.captures(query) {
                entities.push(ExtractedEntity {
                    name: "number".to_string(),
                    value: cap[0].to_string(),
                    entity_type: "Number".to_string(),
                    confidence: 0.8,
                    position: (0, 0),
                });
            }
        }
        
        Ok(entities)
    }
    
    async fn generate_response(&self, parsed: &ParsedQuery, _context: &Option<AssistantContext>) -> Result<AssistantResponse> {
        let (message, data, suggestions) = match parsed.intent.as_str() {
            "query_data" => {
                let msg = "I can help you query data. What specific information would you like to see?".to_string();
                let sugg = vec![
                    "Show me all invoices from last month".to_string(),
                    "List pending purchase orders".to_string(),
                    "Display top 10 customers by revenue".to_string(),
                ];
                (msg, None, sugg)
            }
            "create_entity" => {
                let msg = "I can help you create a new record. What would you like to create?".to_string();
                let sugg = vec![
                    "Create a new customer".to_string(),
                    "Add a new product".to_string(),
                    "Create a purchase order".to_string(),
                ];
                (msg, None, sugg)
            }
            "aggregate_count" => {
                let msg = "I can count records for you. What would you like to count?".to_string();
                let sugg = vec![
                    "How many orders this month?".to_string(),
                    "Count of active customers".to_string(),
                    "Total employees by department".to_string(),
                ];
                (msg, None, sugg)
            }
            "help" => {
                let msg = "I'm your ERP assistant. I can help you:\n• Query and analyze data\n• Create and update records\n• Generate reports\n• Get insights and trends\n\nWhat would you like to do?".to_string();
                let sugg = vec![
                    "Show me a summary of today's activity".to_string(),
                    "What are my pending approvals?".to_string(),
                    "Generate a sales report".to_string(),
                ];
                (msg, None, sugg)
            }
            _ => {
                let msg = "I'm not sure I understand. Could you rephrase your request?".to_string();
                let sugg = vec![
                    "Help me understand what you can do".to_string(),
                    "Show me examples of queries".to_string(),
                ];
                (msg, None, sugg)
            }
        };
        
        Ok(AssistantResponse {
            message,
            action: None,
            suggestions,
            data,
            visualization: None,
        })
    }
    
    pub async fn create_intent(&self, intent: IntentDefinition) -> Result<IntentDefinition> {
        self.repo.create_intent(&intent).await?;
        Ok(intent)
    }
    
    pub async fn list_intents(&self) -> Result<Vec<IntentDefinition>> {
        self.repo.list_intents().await
    }
    
    pub async fn create_skill(&self, skill: AssistantSkill) -> Result<AssistantSkill> {
        self.repo.create_skill(&skill).await?;
        Ok(skill)
    }
    
    pub async fn list_skills(&self, enabled_only: bool) -> Result<Vec<AssistantSkill>> {
        self.repo.list_skills(enabled_only).await
    }
    
    pub async fn create_quick_action(&self, action: QuickAction) -> Result<QuickAction> {
        self.repo.create_quick_action(&action).await?;
        Ok(action)
    }
    
    pub async fn list_quick_actions(&self) -> Result<Vec<QuickAction>> {
        self.repo.list_quick_actions().await
    }
}
