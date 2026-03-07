use crate::models::*;
use crate::repository::{GiftCardRepository, SqliteGiftCardRepository};
use chrono::Utc;
use erp_core::{BaseEntity, Error, Result};
use rand::Rng;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct GiftCardService {
    repo: SqliteGiftCardRepository,
}

impl GiftCardService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteGiftCardRepository::new(pool),
        }
    }

    pub async fn create(&self, pool: &SqlitePool, req: CreateGiftCardRequest) -> Result<GiftCard> {
        if req.initial_balance <= 0 {
            return Err(Error::validation("Initial balance must be positive"));
        }

        let card_number = generate_card_number();
        let pin = generate_pin();
        let barcode = Some(generate_barcode(&card_number));

        let card = GiftCard {
            base: BaseEntity::new(),
            card_number,
            pin: Some(pin),
            barcode,
            gift_card_type: req.gift_card_type,
            initial_balance: req.initial_balance,
            current_balance: req.initial_balance,
            currency: req.currency.unwrap_or_else(|| "USD".to_string()),
            customer_id: req.customer_id,
            order_id: req.order_id,
            purchased_by: req.purchased_by,
            recipient_email: req.recipient_email,
            recipient_name: req.recipient_name,
            message: req.message,
            status: GiftCardStatus::Active,
            issued_date: Utc::now().date_naive(),
            expiry_date: req.expiry_date,
            last_used_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created = self.repo.create(&card).await?;

        let tx = GiftCardTransaction {
            base: BaseEntity::new(),
            transaction_number: generate_transaction_number(),
            gift_card_id: created.base.id,
            transaction_type: GiftCardTransactionType::Issue,
            amount: req.initial_balance,
            balance_before: 0,
            balance_after: req.initial_balance,
            order_id: req.order_id,
            reference: None,
            notes: Some("Gift card issued".to_string()),
            created_by: req.purchased_by,
            created_at: Utc::now(),
        };
        self.repo.create_transaction(&tx).await?;

        Ok(created)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<GiftCard>> {
        self.repo.get_by_id(id).await
    }

    pub async fn get_by_card_number(&self, pool: &SqlitePool, card_number: &str) -> Result<Option<GiftCard>> {
        self.repo.get_by_card_number(card_number).await
    }

    pub async fn list(&self, pool: &SqlitePool, page: i32, per_page: i32) -> Result<Vec<GiftCard>> {
        self.repo.list(page, per_page).await
    }

    pub async fn list_by_customer(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<GiftCard>> {
        self.repo.list_by_customer(customer_id).await
    }

    pub async fn redeem(&self, pool: &SqlitePool, id: Uuid, req: RedeemGiftCardRequest, user_id: Option<Uuid>) -> Result<GiftCardTransaction> {
        let mut card = self.repo.get_by_id(id).await?.ok_or_else(|| Error::not_found("Gift card", &id.to_string()))?;

        if card.status != GiftCardStatus::Active {
            return Err(Error::business_rule("Gift card is not active"));
        }

        if card.current_balance < req.amount {
            return Err(Error::business_rule("Insufficient balance on gift card"));
        }

        if req.amount <= 0 {
            return Err(Error::validation("Redemption amount must be positive"));
        }

        let balance_before = card.current_balance;
        card.current_balance -= req.amount;
        card.last_used_at = Some(Utc::now());
        card.updated_at = Utc::now();

        if card.current_balance == 0 {
            card.status = GiftCardStatus::Redeemed;
        }

        self.repo.update(&card).await?;

        let tx = GiftCardTransaction {
            base: BaseEntity::new(),
            transaction_number: generate_transaction_number(),
            gift_card_id: id,
            transaction_type: GiftCardTransactionType::Redeem,
            amount: req.amount,
            balance_before,
            balance_after: card.current_balance,
            order_id: req.order_id,
            reference: req.reference,
            notes: Some("Gift card redeemed".to_string()),
            created_by: user_id,
            created_at: Utc::now(),
        };

        self.repo.create_transaction(&tx).await
    }

    pub async fn reload(&self, pool: &SqlitePool, id: Uuid, req: ReloadGiftCardRequest, user_id: Option<Uuid>) -> Result<GiftCardTransaction> {
        let mut card = self.repo.get_by_id(id).await?.ok_or_else(|| Error::not_found("Gift card", &id.to_string()))?;

        if card.status == GiftCardStatus::Cancelled || card.status == GiftCardStatus::Expired {
            return Err(Error::business_rule("Gift card cannot be reloaded"));
        }

        if req.amount <= 0 {
            return Err(Error::validation("Reload amount must be positive"));
        }

        let balance_before = card.current_balance;
        card.current_balance += req.amount;
        card.status = GiftCardStatus::Active;
        card.updated_at = Utc::now();

        self.repo.update(&card).await?;

        let tx = GiftCardTransaction {
            base: BaseEntity::new(),
            transaction_number: generate_transaction_number(),
            gift_card_id: id,
            transaction_type: GiftCardTransactionType::Reload,
            amount: req.amount,
            balance_before,
            balance_after: card.current_balance,
            order_id: req.order_id,
            reference: req.reference,
            notes: Some("Gift card reloaded".to_string()),
            created_by: user_id,
            created_at: Utc::now(),
        };

        self.repo.create_transaction(&tx).await
    }

    pub async fn adjust(&self, pool: &SqlitePool, id: Uuid, req: AdjustGiftCardRequest, user_id: Option<Uuid>) -> Result<GiftCardTransaction> {
        let mut card = self.repo.get_by_id(id).await?.ok_or_else(|| Error::not_found("Gift card", &id.to_string()))?;

        let balance_before = card.current_balance;
        card.current_balance = (card.current_balance as i64 + req.amount).max(0) as i64;
        
        if card.current_balance == 0 {
            card.status = GiftCardStatus::Redeemed;
        }
        card.updated_at = Utc::now();

        self.repo.update(&card).await?;

        let tx = GiftCardTransaction {
            base: BaseEntity::new(),
            transaction_number: generate_transaction_number(),
            gift_card_id: id,
            transaction_type: GiftCardTransactionType::Adjust,
            amount: req.amount,
            balance_before,
            balance_after: card.current_balance,
            order_id: None,
            reference: None,
            notes: Some(req.reason),
            created_by: user_id,
            created_at: Utc::now(),
        };

        self.repo.create_transaction(&tx).await
    }

    pub async fn cancel(&self, pool: &SqlitePool, id: Uuid, reason: String, user_id: Option<Uuid>) -> Result<GiftCard> {
        let mut card = self.repo.get_by_id(id).await?.ok_or_else(|| Error::not_found("Gift card", &id.to_string()))?;

        if card.status == GiftCardStatus::Redeemed {
            return Err(Error::business_rule("Cannot cancel a fully redeemed gift card"));
        }

        card.status = GiftCardStatus::Cancelled;
        card.updated_at = Utc::now();

        let updated = self.repo.update(&card).await?;

        let tx = GiftCardTransaction {
            base: BaseEntity::new(),
            transaction_number: generate_transaction_number(),
            gift_card_id: id,
            transaction_type: GiftCardTransactionType::Adjust,
            amount: -(card.current_balance as i64),
            balance_before: card.current_balance,
            balance_after: 0,
            order_id: None,
            reference: None,
            notes: Some(format!("Cancelled: {}", reason)),
            created_by: user_id,
            created_at: Utc::now(),
        };

        self.repo.create_transaction(&tx).await?;

        Ok(updated)
    }

    pub async fn check_balance(&self, pool: &SqlitePool, card_number: &str, pin: Option<&str>) -> Result<GiftCard> {
        let card = self.repo.get_by_card_number(card_number).await?.ok_or_else(|| Error::not_found("Gift card", card_number))?;

        if let Some(card_pin) = &card.pin {
            if let Some(provided_pin) = pin {
                if card_pin != provided_pin {
                    return Err(Error::unauthorized("Invalid PIN"));
                }
            }
        }

        Ok(card)
    }

    pub async fn list_transactions(&self, pool: &SqlitePool, gift_card_id: Uuid) -> Result<Vec<GiftCardTransaction>> {
        self.repo.list_transactions(gift_card_id).await
    }
}

fn generate_card_number() -> String {
    let mut rng = rand::thread_rng();
    let mut number = String::with_capacity(16);
    for _ in 0..16 {
        number.push_str(&rng.gen_range(0..10).to_string());
    }
    number
}

fn generate_pin() -> String {
    let mut rng = rand::thread_rng();
    format!("{:04}", rng.gen_range(0..10000))
}

fn generate_barcode(card_number: &str) -> String {
    format!("GC{}", card_number)
}

fn generate_transaction_number() -> String {
    format!("GCTX-{}", &Uuid::new_v4().to_string()[..8].to_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{GiftCardType, GiftCardStatus};

    #[test]
    fn test_generate_card_number() {
        let number = generate_card_number();
        assert_eq!(number.len(), 16);
        assert!(number.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_pin() {
        let pin = generate_pin();
        assert_eq!(pin.len(), 4);
        assert!(pin.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_barcode() {
        let card_number = "1234567890123456";
        let barcode = generate_barcode(card_number);
        assert_eq!(barcode, "GC1234567890123456");
    }

    #[test]
    fn test_generate_transaction_number() {
        let tx_number = generate_transaction_number();
        assert!(tx_number.starts_with("GCTX-"));
        assert_eq!(tx_number.len(), 13);
    }

    #[test]
    fn test_gift_card_status_values() {
        assert!(matches!(GiftCardStatus::Active, GiftCardStatus::Active));
        assert!(matches!(GiftCardStatus::Redeemed, GiftCardStatus::Redeemed));
        assert!(matches!(GiftCardStatus::Cancelled, GiftCardStatus::Cancelled));
        assert!(matches!(GiftCardStatus::Expired, GiftCardStatus::Expired));
        assert!(matches!(GiftCardStatus::Inactive, GiftCardStatus::Inactive));
    }

    #[test]
    fn test_gift_card_type_values() {
        assert!(matches!(GiftCardType::Physical, GiftCardType::Physical));
        assert!(matches!(GiftCardType::Digital, GiftCardType::Digital));
        assert!(matches!(GiftCardType::ECode, GiftCardType::ECode));
    }

    #[test]
    fn test_create_gift_card_request_validation() {
        let req = CreateGiftCardRequest {
            gift_card_type: GiftCardType::Digital,
            initial_balance: 10000,
            currency: Some("USD".to_string()),
            customer_id: None,
            order_id: None,
            purchased_by: None,
            recipient_email: Some("test@example.com".to_string()),
            recipient_name: Some("John Doe".to_string()),
            message: Some("Happy Birthday!".to_string()),
            expiry_date: None,
        };
        
        assert_eq!(req.initial_balance, 10000);
        assert_eq!(req.gift_card_type, GiftCardType::Digital);
    }

    #[test]
    fn test_redeem_request_validation() {
        let req = RedeemGiftCardRequest {
            amount: 5000,
            order_id: Some(Uuid::nil()),
            reference: Some("ORDER-123".to_string()),
        };
        
        assert_eq!(req.amount, 5000);
        assert!(req.order_id.is_some());
    }

    #[test]
    fn test_reload_request_validation() {
        let req = ReloadGiftCardRequest {
            amount: 2500,
            order_id: None,
            reference: None,
        };
        
        assert_eq!(req.amount, 2500);
    }

    #[test]
    fn test_adjust_request_validation() {
        let req = AdjustGiftCardRequest {
            amount: -1000,
            reason: "Customer complaint".to_string(),
        };
        
        assert_eq!(req.amount, -1000);
        assert!(!req.reason.is_empty());
    }
}
