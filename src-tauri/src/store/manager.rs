//! Storage manager utilities.
//!
//! This module contains a small helper that owns a `SqlitePool` and exposes
//! a convenient API for constructing and accessing the pool. It centralizes
//! connection creation so the rest of the application can request a ready
//! `SqlitePool` or a cloned handle.
use std::env;

use sqlx::SqlitePool;

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
        let conn_str = env::var("DATABASE_URL")?;
        let pool = SqlitePool::connect(&conn_str).await?;
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
