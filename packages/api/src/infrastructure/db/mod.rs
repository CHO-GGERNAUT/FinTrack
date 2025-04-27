pub type ArcPgPool = std::sync::Arc<sqlx::PgPool>;

pub mod repositories;
pub mod schema;

pub mod create_pool;
pub mod unit_of_works;

