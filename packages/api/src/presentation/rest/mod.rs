use axum::Router;
mod v1;
use v1::routes::auth_route::routes;

pub fn router() -> Router {
    Router::new().nest("/v1", routes())
}
