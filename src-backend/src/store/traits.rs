//! Store trait definitions used by adapters in `src/store`.
//!
//! This module defines the small set of generic persistence traits the
//! application uses to interact with the database. The traits intentionally
//! keep a narrow surface area so implementations can be simple and testable.
//!
//! Contract (short):
//! - Inputs: a shared `SqlitePool` and a typed payload (`T`) representing a
//!   domain-level create/read/update/delete object.
//! - Outputs: `Result<Uuid>` for mutating operations, `Result<T>` or
//!   `Result<Vec<T>>` for read ops. Errors are propagated via the crate's
//!   `store::Result` alias.
//! - Error modes: implementations should return errors on DB failures or
//!   constraint violations; callers may map or handle these as needed.
//!
//! Note: `Readable::BASE_SELECT` is a convenience for SQL-based adapters so
//! implementors can provide a shared SELECT fragment that filters extend.
use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};

use crate::store::Result;
use model::Uuid;

/// Trait for creating/persisting a new entity into storage.
///
/// Implementors should insert the provided `entity` into the backing store
/// (typically a SQL database) using the provided connection pool, and return
/// the `Uuid` of the created record on success.
#[async_trait]
pub trait Creatable<T> {
    /// Create `entity` in the store and return its UUID.
    ///
    /// - `pool` is the shared `SqlitePool` to use for the operation.
    /// - `entity` is consumed and contains the data to insert.
    ///
    /// Returns `Ok(uuid)` when the insert succeeds or an error otherwise.
    async fn create(pool: &SqlitePool, entity: T) -> Result<Uuid>;
}

/// Trait for reading entities from storage.
///
/// `Readable` supports both fetching a single entity by id and listing
/// entities using a filter type. Implementations should return domain model
/// objects rather than raw database rows when possible.
#[async_trait]
pub trait Readable<T> {
    /// Filter type used by `list` operations. Must implement `Filterable`.
    type Filter: Filterable;

    /// Base SQL fragment used by implementations to build select queries.
    ///
    /// Implementations may set this to a constant SELECT clause that the
    /// store layer can extend with WHERE / ORDER BY clauses.
    const BASE_SELECT: &'static str;

    /// Read a single entity by `id`.
    ///
    /// Returns `Ok(entity)` when a row with the requested id exists, or an
    /// error if not found or on database failure.
    async fn read(pool: &SqlitePool, id: Uuid) -> Result<T>;

    /// List entities that match the provided `filter`.
    ///
    /// Implementations should translate `filter.apply(...)` into SQL
    /// predicates and return all matching entities.
    async fn list(pool: &SqlitePool, filter: Self::Filter) -> Result<Vec<T>>;
}

/// Trait for updating an entity in storage.
///
/// Implementations should persist changes contained in `entity` and return
/// the UUID of the updated record on success. Use optimistic concurrency
/// (versioning) at the application level if needed.
#[async_trait]
pub trait Updatable<T> {
    /// Update `entity` in the store and return its UUID on success.
    async fn update(pool: &SqlitePool, entity: T) -> Result<Uuid>;
}

/// Trait for deleting (tombstoning or removing) an entity from storage.
///
/// Implementations may choose to soft-delete (tombstone) or hard-delete
/// depending on application requirements; the method returns the UUID of the
/// affected row on success.
#[async_trait]
pub trait Deletable<T> {
    /// Delete `entity` from the store and return its UUID on success.
    async fn delete(pool: &SqlitePool, entity: T) -> Result<Uuid>;
}

/// Trait describing a SQL query filter builder.
///
/// Types implementing `Filterable` should append WHERE clauses or other
/// predicates to the provided `QueryBuilder<Sqlite>` so the caller can
/// construct a complete query before executing it.
pub trait Filterable {
    /// Apply this filter's predicates to `qb` by appending SQL fragments and
    /// bind parameters as needed.
    fn apply(&self, qb: &mut QueryBuilder<Sqlite>);
}
