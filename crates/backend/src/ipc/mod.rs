//! IPC surface for the application — Tauri command handlers.
//!
//! This module exposes the IPC (Tauri) entry points used by the frontend to
//! interact with the application. Handlers in this module are intentionally
//! thin: they translate incoming payloads, construct a [`Ctx`][`crate::ctx::Ctx`] from the
//! [`AppHandle`][`tauri::AppHandle`], and forward the request to the appropriate [backend-model
//! controller (BMC)][`crate::bmc`] which performs store operations and emits events.
//!
//! Exposed submodules map to domain areas and are re-exported here for
//! convenient `use` from other crates or integration tests.
pub mod category;
pub mod category_group;
pub mod entry;

pub use category::*;
pub use category_group::*;
pub use entry::*;
