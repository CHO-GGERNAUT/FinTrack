use axum::{Extension, Json, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    application::{
        command::card::{CreateCardUsecase, DeleteCardUsecase},
        dto::{Claims, CreateCardInput, DeleteCardInput},
    },
    infrastructure::db::unit_of_works::CardUnitOfWorkPostgres,
    presentation::schemas::card::{
        CreateCardRequest, CreateCardResponse, DeleteCardRequest, DeleteCardResponse,
    },
};

pub async fn create_card_handler(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Option<Claims>>,
    Json(req): Json<CreateCardRequest>,
) -> Result<Json<CreateCardResponse>, (StatusCode, String)> {
    let owner_id = if let Some(claims) = claims {
        Uuid::parse_str(&claims.user_id).unwrap()
    } else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    };
    let uow = CardUnitOfWorkPostgres::new(pool).await.map_err(|e| {
        tracing::error!("Failed to create unit of work: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    let usecase = CreateCardUsecase::new(uow);

    let card_number_last4 = split_card_number_blocks(&req.card_number)
        .last()
        .cloned()
        .unwrap_or_default();

    let encrypted_card_number = encrypt_card_number_bcrypt(&req.card_number).map_err(|e| {
        tracing::error!("Failed to encrypt card number: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    let res = usecase
        .execute(CreateCardInput {
            owner_id,
            card_number_last4,
            encrypted_card_number,
            issued_at: req.issued_at,
            expires_at: req.expires_at,
            billing_day: req.billing_day,
            brand: req.brand,
            issuer: req.issuer,
            card_type: req.card_type,
            name: req.name,
            memo: req.memo,
        })
        .await
        .map_err(|e| {
            tracing::error!("Failed to create card: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    Ok(Json(res.into()))
}

pub async fn delete_card_handler(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Option<Claims>>,
    Json(req): Json<DeleteCardRequest>,
) -> Result<Json<DeleteCardResponse>, (StatusCode, String)> {
    let owner_id = if let Some(claims) = claims {
        Uuid::parse_str(&claims.user_id).unwrap()
    } else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    };
    let uow = CardUnitOfWorkPostgres::new(pool).await.map_err(|e| {
        tracing::error!("Failed to create unit of work: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    let usecase = DeleteCardUsecase::new(uow);
    let res = usecase
        .execute(DeleteCardInput {
            owner_id,
            account_id: req.account_id,
        })
        .await
        .map_err(|e| {
            tracing::error!("Failed to create card: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(res.into()))
}

fn split_card_number_blocks(card_number: &str) -> Vec<String> {
    card_number
        .split(|c: char| !c.is_ascii_digit()) // 숫자가 아닌 문자를 구분자로 사용
        .filter(|s| !s.is_empty()) // 빈 문자열 제거
        .map(|s| s.to_string())
        .collect()
}

fn encrypt_card_number_bcrypt(card_number: &str) -> Result<Vec<u8>, bcrypt::BcryptError> {
    let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();
    let hashed = hash(digits, DEFAULT_COST)?;
    Ok(hashed.as_bytes().to_vec())
}
