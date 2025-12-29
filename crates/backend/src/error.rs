//! Common error types used throughout the application.
//!
//! This module defines the `Error` enum and a convenient `Result<T>` alias
//! so other modules can return `crate::error::Result<T>` for fallible
//! operations.

/// Result alias using the crate-wide `Error` type.
pub type Result<T> = core::result::Result<T, Error>;

/// Top-level error enum for the application.
///
/// Wraps common error kinds used by the model and store layers. New variants
/// may be added as the application grows (for example for validation or
/// network errors).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SQLx/database related errors.
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    /// SQLx/database related errors.
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    /// SQLx/database related errors.
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// Environment variable resolution errors (for example missing
    /// `DATABASE_URL`).
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),

    /// Currently, the only way this will fail is do a `app_handle.try_state::<T>()`
    /// where `T` is before that state is managed by tauri. This means you either never
    /// called `.manage(data: T)` or you tried to get at the state before it's been called
    #[error("Context error; failed to access state that is not managed by tauri")]
    CtxFail,

    /// Any errors results
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Custom
    #[error("Custom error: {0}")]
    Custom(&'static str),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
