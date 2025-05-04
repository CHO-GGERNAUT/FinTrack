use axum::{Extension, Json, http::StatusCode};
use sqlx::PgPool;

use crate::{
    application::commands::user::RegisterUserPasswordHandler,
    infrastructure::persistence::postgres::unit_of_works::UserUnitOfWorkPg,
    presentation::rest::v1::schemas::user::{CreateUserRequest, CreateUserResponse},
};

pub async fn create_user_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, (StatusCode, String)> {
    let uow = UserUnitOfWorkPg::new(pool).await.map_err(|e| {
        tracing::error!("Failed to create user unit of work: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;
    let handler = RegisterUserPasswordHandler::new(uow);
    let res = handler
        .execute(req.into())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(res.into()))
}
