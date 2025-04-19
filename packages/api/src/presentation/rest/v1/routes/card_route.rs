use crate::presentation::rest::v1::handlers::card_handler::create_card;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new().route("/", post(create_card))
}
