pub mod getters;
pub mod setters;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn start_db() -> Result<PgPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .connect(&db_url)
        .await
}