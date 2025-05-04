use axum::{Extension, Json};
use sqlx::PgPool;

use crate::{
    application::{
        commands::user::RegisterUserPasswordHandler,
        errors::ApplicationError,
        interfaces::services::token_service::Claims,
        queries::user::{GetUserHandler, GetUserQuery},
    },
    infrastructure::persistence::postgres::unit_of_works::UserUnitOfWorkPg,
    presentation::rest::{
        error::RestApiResult,
        v1::schemas::user::{CreateUserRequest, CreateUserResponse, UserResponse},
    },
};

pub async fn create_user_handler(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> RestApiResult<Json<CreateUserResponse>> {
    let uow = UserUnitOfWorkPg::new(pool)
        .await
        .map_err(|e| ApplicationError::from(e))?;
    let handler = RegisterUserPasswordHandler::new(uow);
    let res = handler.execute(req.into()).await?;
    Ok(Json(res.into()))
}

pub async fn get_user_handler(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Option<Claims>>,
) -> RestApiResult<Json<UserResponse>> {
    tracing::debug!("Claims: {:?}", claims);
    let user_id = claims
        .as_ref()
        .ok_or(ApplicationError::Authorization("Unauthorized".to_string()))?
        .sub
        .parse::<uuid::Uuid>()
        .unwrap();
    let uow = UserUnitOfWorkPg::new(pool)
        .await
        .map_err(|e| ApplicationError::from(e))?;
    let handler = GetUserHandler::new(uow);
    let res = handler.execute(GetUserQuery { user_id }).await?;
    Ok(Json(res.user.into()))
}
