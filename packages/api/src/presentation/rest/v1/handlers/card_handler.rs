use axum::{Extension, Json};
use sqlx::PgPool;

use crate::{
    application::{
        commands::card::IssueCardHandler, errors::ApplicationError,
        interfaces::services::token_service::Claims,
    },
    infrastructure::persistence::postgres::unit_of_works::CardUnitOfWorkPg,
    presentation::rest::{
        error::{RestApiError, RestApiResult},
        v1::schemas::card::{IssueCardRequest, IssueCardResponse},
    },
};

pub async fn issue_card_handler(
    Extension(pool): Extension<PgPool>,
    Extension(claims): Extension<Option<Claims>>,
    Json(req): Json<IssueCardRequest>,
) -> RestApiResult<Json<IssueCardResponse>> {
    let user_id = claims
        .as_ref()
        .ok_or(RestApiError::Authentication)?
        .sub
        .parse::<uuid::Uuid>()
        .unwrap();

    let uow = CardUnitOfWorkPg::new(pool)
        .await
        .map_err(|e| ApplicationError::from(e))?;

    let handler = IssueCardHandler::new(uow);

    let res = handler.execute((req, user_id).into()).await?;

    Ok(Json(res.into()))
}
