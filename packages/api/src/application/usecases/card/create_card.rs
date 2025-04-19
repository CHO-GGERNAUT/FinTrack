use uuid::Uuid;

use crate::{
    application::{
        dto::{CreateCardInput, CreateCardOutput},
        error::{ApplicationError, Result},
    },
    domain::repositories::card_repository::CardRepository,
};

#[derive(Clone)]

pub struct CreateCardUseCase<R: CardRepository> {
    pub repo: R,
}

impl<R: CardRepository> CreateCardUseCase<R> {
    pub async fn execute(&self, create_dto: CreateCardInput) -> Result<CreateCardOutput> {
        if let Some(billing_day) = create_dto.billing_day {
            if billing_day < 1 || billing_day > 31 {
                ApplicationError::ValidationError(
                    "Billing day must be between 1 and 31".to_string(),
                );
            }
        }
        self.repo.save(create_dto.into()).await.map_err(|e| {
            ApplicationError::RepositoryError(format!("Failed to save card: {}", e))
        })?;
        Ok(CreateCardOutput {
            card_id: Uuid::new_v4(),
            created_at: String::default(),
        })
    }
}
