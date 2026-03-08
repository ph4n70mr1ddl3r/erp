use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductInformation {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub attributes: serde_json::Value, // Dynamic attributes like color, size, material
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProductInformation {
    pub fn new(sku: String, name: String, category: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            sku,
            name,
            description: None,
            category,
            attributes: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated_at = Utc::now();
    }

    pub fn set_attribute(&mut self, key: &str, value: serde_json::Value) {
        if let Some(obj) = self.attributes.as_object_mut() {
            obj.insert(key.to_string(), value);
            self.updated_at = Utc::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_and_update_product_info() {
        let mut product = ProductInformation::new(
            "SKU-123".to_string(),
            "Comfort Chair".to_string(),
            "Furniture".to_string(),
        );

        assert_eq!(product.sku, "SKU-123");
        assert_eq!(product.name, "Comfort Chair");
        assert_eq!(product.description, None);

        product.set_description("An ergonomic comfort chair.".to_string());
        assert_eq!(product.description.as_deref(), Some("An ergonomic comfort chair."));

        product.set_attribute("color", json!("Black"));
        product.set_attribute("weight_kg", json!(15.5));

        assert_eq!(product.attributes.get("color").unwrap(), &json!("Black"));
        assert_eq!(product.attributes.get("weight_kg").unwrap(), &json!(15.5));
    }
}
