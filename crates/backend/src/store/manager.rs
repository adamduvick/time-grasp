//! Storage manager utilities.
//!
//! This module contains a small helper that owns a `SqlitePool` and exposes
//! a convenient API for constructing and accessing the pool. It centralizes
//! connection creation so the rest of the application can request a ready
//! `SqlitePool` or a cloned handle.
use std::path::PathBuf;
use std::{env, str::FromStr};

use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::sync::OnceCell;

use crate::store::{Error, Result};

/// Lightweight wrapper owning the application's `SqlitePool`.
///
/// `StoreManager` centralizes pool creation and provides a small API to
/// obtain cloned `SqlitePool` instances for callers. It is intentionally
/// minimal and designed to be stored in application state (e.g. Tauri
/// managed state).
pub struct StoreManager {
    store_type: StoreType,
    pool: OnceCell<SqlitePool>,
}

pub enum StoreType {
    AppData(PathBuf),
    Memory,
    Dev,
    FromPool,
}

impl StoreManager {
    pub fn new(store_type: StoreType) -> Result<Self> {
        match store_type {
            StoreType::FromPool => Err(Error::Custom(
                "Use `StoreManager::from_pool` to create a manager of `StoreType::FromPool`",
            ))?,
            _ => Ok(Self {
                store_type,
                pool: OnceCell::new(),
            }),
        }
    }

    pub fn from_path(app_data_dir: PathBuf) -> Self {
        Self {
            store_type: StoreType::AppData(app_data_dir),
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
    pub fn from_pool(pool: &SqlitePool) -> Self {
        Self {
            store_type: StoreType::FromPool,
            pool: pool.clone().into(),
        }
    }

    /// Return a cloned `SqlitePool` handle.
    pub async fn pool(&self) -> Result<SqlitePool> {
        let pool = self
            .pool
            .get_or_try_init(|| async {
                println!("Initializing database");

                let opts = match &self.store_type {
                    StoreType::AppData(dir) => {
                        std::fs::create_dir_all(dir.clone())?;
                        let path = dir.join("time_grasp.db");
                        let path = path.to_str().ok_or(Error::Custom("Invalid path"))?;
                        println!("DB path: {path}");
                        SqliteConnectOptions::from_str(path)?
                    }
                    StoreType::Memory => {
                        let path = ":memory:";
                        println!("DB path: {path}");
                        SqliteConnectOptions::from_str(path)?
                    }
                    StoreType::Dev => {
                        let path = env::var("DATABASE_URL")?;
                        println!("DB path: {path}");
                        SqliteConnectOptions::from_str(&path)?
                    }
                    StoreType::FromPool => Err(Error::Custom(
                        "Unreachable: StoreManager from Pool should already be initialized",
                    ))?,
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
                let seed_type = crate::store::seed_for_dev::SeedType::Representative;
                println!("➡️ Seeding dev db with {seed_type:?}");
                // TODO: gaurd this with a dev-only flag.
                match crate::store::seed_for_dev::seed_for_dev(&pool, seed_type).await {
                    Ok(()) => println!("✅ Completed seeding dev db with {seed_type:?}"),
                    Err(e) => eprintln!("❌ Seeding skipped for the following reasons: {e:?}"),
                }

                Ok::<_, crate::error::Error>(pool)
            })
            .await?;

        Ok(pool.clone())
    }
}
