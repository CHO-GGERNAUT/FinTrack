use crate::presentation::rest::v1::handlers::user_handler::create_user_handler;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new().route("/", post(create_user_handler))
}
