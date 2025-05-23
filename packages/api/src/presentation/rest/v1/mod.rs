mod schemas {
    pub mod auth;
    pub mod card;
    pub mod user;
}

mod handlers {
    pub mod auth_handler;
    pub mod card_handler;
    pub mod user_handler;
}
pub mod routes {
    pub mod auth_route;
    pub mod card_route;
    pub mod user_route;
}

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", routes::auth_route::routes())
        .nest("/users", routes::user_route::routes())
        .nest("/cards", routes::card_route::routes())
}
