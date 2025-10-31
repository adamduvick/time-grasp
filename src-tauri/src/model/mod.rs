// main.rs
use anyhow::{Context, Result};
use std::env;

use crate::model::types::PublicEntry;

mod seed_for_dev;
mod store;
mod types;

#[tokio::main]
pub async fn main() -> Result<()> {
    let pool = store::Store::new(env::var("DATABASE_URL")?)
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

    #[derive(Debug)]
    struct Entry {
        payee: String,
        memo: String,
        category: String,
    }

    let entries: Vec<Entry> =
        sqlx::query_as!(Entry, "SELECT payee, memo, category FROM v_public_entries")
            .fetch_all(&pool)
            .await?;

    for entry in entries {
        println!("{:?}", entry);
    }

    Ok(())
}
