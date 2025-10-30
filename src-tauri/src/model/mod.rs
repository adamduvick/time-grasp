// main.rs
use std::error::Error;

use crate::model::types::PublicEntry;

mod store;
mod types;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let pool = store::Store::new("data.db").await?.get_pool().clone();

    println!("✅ Database connected and migrations applied.");

    // Example query to verify connection
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master;")
        .fetch_one(&pool)
        .await?;

    println!("Database has {} objects defined.", row.0);

    // Example: fetch all entries
    let rows: Vec<PublicEntry> = sqlx::query_as::<_, PublicEntry>("SELECT * FROM v_public_entries")
        .fetch_all(&pool)
        .await?;

    for entry in rows {
        println!("{:?}", entry);
    }

    Ok(())
}
