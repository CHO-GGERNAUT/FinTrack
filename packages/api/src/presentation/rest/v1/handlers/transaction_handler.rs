use axum::{Extension, Json, http::StatusCode};
use sqlx::PgPool;

use crate::{
    application::{command::transaction::CreateTransactionUsecase, dto::CreateTransactionInput},
    infrastructure::db::unit_of_works::TransactionUnitOfWorkPostgres,
    presentation::schemas::transaction::{CreateTransactionRequest, CreateTransactionResponse},
};

pub async fn create_transaction_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<Json<CreateTransactionResponse>, (StatusCode, String)> {
    let uow = TransactionUnitOfWorkPostgres::new(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let usecase = CreateTransactionUsecase::new(uow);
    let res = usecase
        .execute(CreateTransactionInput::from(req))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(res.into()))
}
