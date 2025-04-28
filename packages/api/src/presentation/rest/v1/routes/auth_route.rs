use crate::presentation::rest::v1::handlers::auth_handler::login_handler;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new().route("/login", post(login_handler))
}
