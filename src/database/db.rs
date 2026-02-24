use tracing;
use sqlx::{ Pool, Postgres, postgres::PgPoolOptions };
use dotenvy::dotenv;
use std::env;

pub async fn db_create_pool() -> Result<Pool<Postgres>, sqlx::Error>
{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set!");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|err| {
            tracing::error!("Database Connection Error: {}", err);
            err
        })?;

    println!("DATABASE CONNECTION ESTABLISHED!");

    Ok(pool)
}