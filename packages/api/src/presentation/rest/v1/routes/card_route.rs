use crate::presentation::rest::v1::handlers::card_handler::issue_card_handler;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new().route("/", post(issue_card_handler))
}
