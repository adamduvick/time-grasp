//! Storage manager utilities.
//!
//! This module contains a small helper that owns a `SqlitePool` and exposes
//! a convenient API for constructing and accessing the pool. It centralizes
//! connection creation so the rest of the application can request a ready
//! `SqlitePool` or a cloned handle.
use std::path::PathBuf;
use std::{env, str::FromStr};

use anyhow::Context;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::sync::OnceCell;

use crate::store::Result;

/// Lightweight wrapper owning the application's `SqlitePool`.
///
/// `StoreManager` centralizes pool creation and provides a small API to
/// obtain cloned `SqlitePool` instances for callers. It is intentionally
/// minimal and designed to be stored in application state (e.g. Tauri
/// managed state).
pub struct StoreManager {
    app_data_dir: Option<PathBuf>,
    pool: OnceCell<SqlitePool>,
}

impl StoreManager {
    /// Create a new `StoreManager` by connecting to the database URL found
    /// in the `DATABASE_URL` environment variable.
    pub fn new(app_data_dir: std::path::PathBuf) -> Self {
        Self {
            app_data_dir: Some(app_data_dir),
            pool: OnceCell::new(),
        }
    }

    /// Eagerly initialize the pool (optional). Useful if you want to warm it up in
    /// a background task at startup.
    pub async fn init(&self) -> Result<()> {
        let _ = self.pool().await?;
        Ok(())
    }

    /// Construct a `StoreManager` from an existing `SqlitePool`.
    pub fn from_pool(pool: SqlitePool) -> Self {
        Self {
            app_data_dir: None,
            pool: pool.clone().into(),
        }
    }

    /// Return a cloned `SqlitePool` handle.
    pub async fn pool(&self) -> Result<SqlitePool> {
        let pool = self
            .pool
            .get_or_try_init(|| async {
                // TODO, for dev, give option to init database according to environment
                // variable.
                //
                // Path to your SQLite database file.
                // let database_url = env::var("DATABASE_URL")?;

                println!("Initializing database");

                let opts = match &self.app_data_dir {
                    Some(dir) => {
                        std::fs::create_dir_all(dir.clone())?;
                        let path = dir.join("time_grasp.db");
                        let path = path.to_str().context("Invalid path")?;
                        println!("DB path: {}", path);
                        SqliteConnectOptions::from_str(path)?
                    }
                    None => {
                        // TODO: this code path will never be exercised
                        // also, this API is not great currently. An in-memory instantiation
                        // should be more explicit than just `app_data_dir = None`
                        let path = ":memory:";
                        println!("DB path: {}", path);
                        SqliteConnectOptions::from_str(path)?
                    }
                };

                let opts = opts
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
                // TODO: gaurd this with a dev-only flag.
                match crate::store::seed_for_dev::seed_for_dev(&pool, seed_type).await {
                    Ok(_) => println!("✅ Completed seeding dev db with {:?}", seed_type),
                    Err(e) => eprintln!("❌ Seeding skipped for the following reasons: {:?}", e),
                }

                Ok::<_, crate::error::Error>(pool)
            })
            .await?;

        Ok(pool.clone())
    }
}
