use sqlx::postgres::{PgPool, PgPoolOptions};

use super::config::Config;

pub fn create_pool() -> PgPool {
    let config = Config::get();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&config.database_url)
        .expect("Failed to create database pool");

    pool
}
pub type ArcPgPool = std::sync::Arc<sqlx::PgPool>;

pub mod user_repository;
