use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::infrastructure::config::Config;

pub fn create_pool() -> PgPool {
    let config = Config::get();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&config.database_url)
        .expect("Failed to create database pool");

    pool
}
