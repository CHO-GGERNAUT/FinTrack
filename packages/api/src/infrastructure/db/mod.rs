pub mod repositories {
    mod card_repository;
    pub use card_repository::CardRepositoryPostgres;
    mod user_repository;
    pub use user_repository::UserRepositoryPostgres;
}
pub mod schema;

use sqlx::postgres::{PgPool, PgPoolOptions};

use super::config::Config;
pub type ArcPgPool = std::sync::Arc<sqlx::PgPool>;

pub fn create_pool() -> PgPool {
    let config = Config::get();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&config.database_url)
        .expect("Failed to create database pool");

    pool
}
