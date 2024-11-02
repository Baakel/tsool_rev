pub mod getters;
pub mod setters;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::CONFIG;

pub async fn start_db() -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().connect(&*CONFIG.database_url).await
}

