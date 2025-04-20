use crate::presentation::rest::v1::handlers::card_handler::{
    create_card_handler, delete_card_handler,
};
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new().route("/", post(create_card_handler).delete(delete_card_handler))
}
