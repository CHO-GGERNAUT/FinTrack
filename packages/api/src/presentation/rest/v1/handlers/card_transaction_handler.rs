use axum::{Extension, Json, extract::Path, http::StatusCode};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    application::{
        command::transactions::CreateCardTransactionUsecase,
        dto::{Claims, CreateCardTransactionInput},
    },
    infrastructure::db::unit_of_works::CardTransactionUnitOfWorkPostgres,
    presentation::schemas::transaction::{
        CreateCardTransactionRequest, CreateCardTransactionResponse,
    },
};

pub async fn create_card_transaction_handler(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Option<Claims>>,
    Path(card_id): Path<Uuid>,
    Json(req): Json<CreateCardTransactionRequest>,
) -> Result<Json<CreateCardTransactionResponse>, (StatusCode, String)> {
    let user_id = if let Some(claims) = claims {
        Uuid::parse_str(&claims.sub).unwrap()
    } else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    };
    let uow = CardTransactionUnitOfWorkPostgres::new(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let usecase = CreateCardTransactionUsecase::new(uow);
    let input: CreateCardTransactionInput = (req, card_id, user_id).try_into().map_err(|e| {
        tracing::error!("Failed to convert request to input: {}", e);
        (StatusCode::BAD_REQUEST, e)
    })?;
    let res = usecase
        .execute(input)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(res.into()))
}
