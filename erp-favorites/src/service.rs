use anyhow::Result;
use erp_core::Error;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{CreateFavoriteRequest, Favorite, FavoriteListResponse, FavoriteType};
use crate::repository::{FavoriteRepository, FavoriteRepositoryTrait};

pub struct FavoriteService {
    repo: FavoriteRepository,
}

impl Default for FavoriteService {
    fn default() -> Self {
        Self::new()
    }
}

impl FavoriteService {
    pub fn new() -> Self {
        Self {
            repo: FavoriteRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        req: CreateFavoriteRequest,
        user_id: Uuid,
    ) -> Result<Favorite> {
        if req.entity_name.trim().is_empty() {
            return Err(Error::validation("Entity name cannot be empty").into());
        }

        let favorite_type: FavoriteType = req
            .favorite_type
            .parse()
            .map_err(|e: String| Error::validation(&e))?;

        if let Some(entity_id) = req.entity_id {
            let exists = self.repo.exists(pool, user_id, &favorite_type, entity_id).await?;
            if exists {
                return Err(Error::business_rule("This item is already in your favorites").into());
            }
        }

        let mut favorite = Favorite::new(
            user_id,
            favorite_type,
            req.entity_id,
            req.entity_name,
            req.entity_code,
        );
        favorite.notes = req.notes;
        favorite.base.created_by = Some(user_id);

        self.repo.create(pool, &favorite).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Favorite> {
        self.repo
            .find_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow::anyhow!(Error::not_found("Favorite", &id.to_string())))
    }

    pub async fn list_for_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<FavoriteListResponse> {
        let items = self.repo.find_by_user(pool, user_id).await?;
        let total = items.len() as i64;
        Ok(FavoriteListResponse { items, total })
    }

    pub async fn list_for_user_by_type(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
    ) -> Result<Vec<Favorite>> {
        self.repo.find_by_user_and_type(pool, user_id, favorite_type).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<()> {
        let favorite = self.get(pool, id).await?;
        if favorite.user_id != user_id {
            return Err(Error::unauthorized("You can only delete your own favorites").into());
        }
        self.repo.delete(pool, id).await
    }

    pub async fn delete_by_entity(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
        entity_id: Uuid,
    ) -> Result<()> {
        self.repo.delete_by_user_and_entity(pool, user_id, favorite_type, entity_id).await
    }

    pub async fn is_favorite(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
        entity_id: Uuid,
    ) -> Result<bool> {
        self.repo.exists(pool, user_id, favorite_type, entity_id).await
    }

    pub async fn toggle_favorite(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        favorite_type: &FavoriteType,
        entity_id: Uuid,
        entity_name: String,
        entity_code: Option<String>,
    ) -> Result<(Favorite, bool)> {
        let existing = self.repo.find_by_user_and_entity(pool, user_id, favorite_type, entity_id).await?;
        
        if let Some(fav) = existing {
            self.repo.delete(pool, fav.base.id).await?;
            Ok((fav, false))
        } else {
            let req = CreateFavoriteRequest {
                favorite_type: favorite_type.to_string(),
                entity_id: Some(entity_id),
                entity_name,
                entity_code,
                notes: None,
            };
            let favorite = self.create(pool, req, user_id).await?;
            Ok((favorite, true))
        }
    }

    pub async fn count_for_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<i64> {
        self.repo.count_by_user(pool, user_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FavoriteType;

    #[test]
    fn test_favorite_type_parse() {
        assert!(matches!("customer".parse::<FavoriteType>(), Ok(FavoriteType::Customer)));
        assert!(matches!("PRODUCT".parse::<FavoriteType>(), Ok(FavoriteType::Product)));
        assert!(matches!("Order".parse::<FavoriteType>(), Ok(FavoriteType::Order)));
        assert!("invalid".parse::<FavoriteType>().is_err());
    }

    #[test]
    fn test_favorite_type_display() {
        assert_eq!(FavoriteType::Customer.to_string(), "Customer");
        assert_eq!(FavoriteType::Product.to_string(), "Product");
        assert_eq!(FavoriteType::Order.to_string(), "Order");
    }

    #[test]
    fn test_favorite_new() {
        let user_id = Uuid::new_v4();
        let entity_id = Uuid::new_v4();
        let favorite = Favorite::new(
            user_id,
            FavoriteType::Customer,
            Some(entity_id),
            "Test Customer".to_string(),
            Some("CUST001".to_string()),
        );

        assert_eq!(favorite.user_id, user_id);
        assert_eq!(favorite.entity_id, Some(entity_id));
        assert_eq!(favorite.entity_name, "Test Customer");
        assert_eq!(favorite.entity_code, Some("CUST001".to_string()));
        assert!(favorite.notes.is_none());
    }

    #[test]
    fn test_create_favorite_request_validation() {
        let svc = FavoriteService::new();
        let user_id = Uuid::new_v4();
        
        let empty_name_req = CreateFavoriteRequest {
            favorite_type: "Customer".to_string(),
            entity_id: Some(Uuid::new_v4()),
            entity_name: "   ".to_string(),
            entity_code: None,
            notes: None,
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .connect(":memory:")
                .await
                .unwrap();
            svc.create(&pool, empty_name_req, user_id).await
        });
        
        assert!(result.is_err());
    }
}
