// main.rs
use anyhow::{Context, Result};
use std::env;

// mod entry;
pub mod model_store;
mod seed_for_dev;
mod types;

// --- Re-exports
pub use model_store::*;

#[tokio::main]
pub async fn main() -> Result<()> {
    let pool = model_store::ModelStore::new(env::var("DATABASE_URL")?)
        .await
        .context("store instantiation failed")?
        .get_pool()
        .clone();

    println!("✅ Database connected and migrations applied.");

    // Example query to verify connection
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master;")
        .fetch_one(&pool)
        .await
        .context("select failed")?;

    println!("Database has {} objects defined.", row.0);

    Ok(())
}
