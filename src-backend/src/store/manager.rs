//! Storage manager utilities.
//!
//! This module contains a small helper that owns a `SqlitePool` and exposes
//! a convenient API for constructing and accessing the pool. It centralizes
//! connection creation so the rest of the application can request a ready
//! `SqlitePool` or a cloned handle.
use std::{env, str::FromStr};

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

use crate::store::Result;

/// Lightweight wrapper owning the application's `SqlitePool`.
///
/// `StoreManager` centralizes pool creation and provides a small API to
/// obtain cloned `SqlitePool` instances for callers. It is intentionally
/// minimal and designed to be stored in application state (e.g. Tauri
/// managed state).
pub struct StoreManager {
    pool: SqlitePool,
}

impl StoreManager {
    /// Create a new `StoreManager` by connecting to the database URL found
    /// in the `DATABASE_URL` environment variable.
    pub async fn new() -> Result<Self> {
        // Path to your SQLite database file.
        // ":memory:" for in-memory; "time_grasp.db" for persistent file.
        let database_url = env::var("DATABASE_URL")?;

        // Helpful diagnostics
        eprintln!("DB path: {}", database_url.clone());

        let opts = SqliteConnectOptions::from_str(&database_url)?
            .create_if_missing(true)
            .foreign_keys(true) // PRAGMA foreign_keys=ON for each conn
            .journal_mode(SqliteJournalMode::Wal); // optional: better concurrency

        let pool: SqlitePool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await?;

        // Apply migrations. Assumes your .sql files live in a `migrations/` folder.
        // Example: `migrations/0001_init.sql`
        //
        // The `sqlx::migrate!()` macro embeds migration metadata at compile time.
        sqlx::migrate!("./migrations").run(&pool).await?;
        let seed_type = crate::store::seed_for_dev::SeedType::Random;
        println!("➡️ Seeding dev db with {:?}", seed_type);
        match crate::store::seed_for_dev::seed_for_dev(&pool, seed_type).await {
            Ok(_) => println!("✅ Completed seeding dev db with {:?}", seed_type),
            Err(e) => eprintln!("❌ Seeding skipped for the following reasons: {:?}", e),
        }
        Ok(Self { pool })
    }

    /// Construct a `StoreManager` from an existing `SqlitePool`.
    pub fn from_pool(pool: SqlitePool) -> Self {
        Self { pool: pool.clone() }
    }

    /// Return a cloned `SqlitePool` handle.
    pub fn pool(&self) -> SqlitePool {
        self.pool.clone()
    }
}
