//! Common error types used throughout the application.
//!
//! This module defines the `Error` enum and a convenient `Result<T>` alias
//! so other modules can return `crate::error::Result<T>` for fallible
//! operations.
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Result alias using the crate-wide `Error` type.
pub type Result<T> = core::result::Result<T, Error>;

/// Top-level error enum for the application.
///
/// Wraps common error kinds used by the model and store layers. New variants
/// may be added as the application grows (for example for validation or
/// network errors).
#[derive(Debug, thiserror::Error, Serialize, Deserialize, Clone)]
#[serde(tag = "kind", content = "message")]
pub enum Error {
    /// Error received while invoking a tauri command.
    #[error("Invoke error: {0}")]
    Invoke(String),

    /// Error received while serializing or deserializing a value.
    #[error("Serde error: {0}")]
    Serde(String),

    /// Error received while serializing or deserializing a value.
    #[error("Serde error: {0}")]
    Time(String),

    /// Error received while serializing or deserializing a value.
    #[error("FmcProvider error: Signal Disposed")]
    FmcProvider,
}

impl From<JsValue> for Error {
    fn from(v: JsValue) -> Self {
        Error::Invoke(v.as_string().unwrap_or_else(|| format!("{v:?}")))
    }
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Error::Serde(e.to_string())
    }
}

impl From<time::error::ComponentRange> for Error {
    fn from(e: time::error::ComponentRange) -> Self {
        Error::Time(format!("{e}").to_string())
    }
}
