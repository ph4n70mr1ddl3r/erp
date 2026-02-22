use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::{Error, Result, Status};
use crate::models::{CustomFieldDefinition, CustomFieldType, CustomFieldValue};

pub struct CustomFieldService;

impl CustomFieldService {
    pub fn new() -> Self { Self }

    pub async fn create_definition(
        pool: &SqlitePool,
        entity_type: &str,
        field_name: &str,
        field_label: &str,
        field_type: CustomFieldType,
        required: bool,
        options: Option<&str>,
        sort_order: i32,
    ) -> Result<CustomFieldDefinition> {
        let now = Utc::now();
        let def = CustomFieldDefinition {
            id: Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            field_name: field_name.to_string(),
            field_label: field_label.to_string(),
            field_type,
            required,
            options: options.map(|s| s.to_string()),
            sort_order,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO custom_field_definitions (id, entity_type, field_name, field_label, field_type, required, options, sort_order, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(def.id.to_string())
        .bind(&def.entity_type)
        .bind(&def.field_name)
        .bind(&def.field_label)
        .bind(format!("{:?}", def.field_type))
        .bind(if def.required { 1 } else { 0 })
        .bind(&def.options)
        .bind(def.sort_order)
        .bind(def.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(def)
    }

    pub async fn get_definitions_for_entity(pool: &SqlitePool, entity_type: &str) -> Result<Vec<CustomFieldDefinition>> {
        let rows = sqlx::query_as::<_, CustomFieldDefRow>(
            "SELECT id, entity_type, field_name, field_label, field_type, required, options, sort_order, status, created_at
             FROM custom_field_definitions WHERE entity_type = ? AND status = 'Active' ORDER BY sort_order"
        )
        .bind(entity_type)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn set_value(
        pool: &SqlitePool,
        definition_id: Uuid,
        entity_id: Uuid,
        value: &str,
    ) -> Result<CustomFieldValue> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO custom_field_values (id, definition_id, entity_id, value, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(definition_id, entity_id) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at"
        )
        .bind(id.to_string())
        .bind(definition_id.to_string())
        .bind(entity_id.to_string())
        .bind(value)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(CustomFieldValue {
            id,
            definition_id,
            entity_id,
            value: Some(value.to_string()),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_values_for_entity(pool: &SqlitePool, entity_id: Uuid) -> Result<Vec<CustomFieldValue>> {
        let rows = sqlx::query_as::<_, CustomFieldValueRow>(
            "SELECT id, definition_id, entity_id, value, created_at, updated_at
             FROM custom_field_values WHERE entity_id = ?"
        )
        .bind(entity_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn delete_definition(pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE custom_field_definitions SET status = 'Inactive' WHERE id = ?"
        )
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct CustomFieldDefRow {
    id: String,
    entity_type: String,
    field_name: String,
    field_label: String,
    field_type: String,
    required: i64,
    options: Option<String>,
    sort_order: i64,
    status: String,
    created_at: String,
}

impl From<CustomFieldDefRow> for CustomFieldDefinition {
    fn from(r: CustomFieldDefRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            entity_type: r.entity_type,
            field_name: r.field_name,
            field_label: r.field_label,
            field_type: match r.field_type.as_str() {
                "Number" => CustomFieldType::Number,
                "Date" => CustomFieldType::Date,
                "Boolean" => CustomFieldType::Boolean,
                "Select" => CustomFieldType::Select,
                "MultiSelect" => CustomFieldType::MultiSelect,
                _ => CustomFieldType::Text,
            },
            required: r.required != 0,
            options: r.options,
            sort_order: r.sort_order as i32,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct CustomFieldValueRow {
    id: String,
    definition_id: String,
    entity_id: String,
    value: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<CustomFieldValueRow> for CustomFieldValue {
    fn from(r: CustomFieldValueRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            definition_id: Uuid::parse_str(&r.definition_id).unwrap_or_default(),
            entity_id: Uuid::parse_str(&r.entity_id).unwrap_or_default(),
            value: r.value,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}
