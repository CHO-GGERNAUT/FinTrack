use sqlx::{Error, PgPool, postgres::PgPoolOptions};

use crate::infrastructure::config::Config;

pub fn create_pool() -> Result<PgPool, Error> {
    let config = Config::get();
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&config.database_url)
}
