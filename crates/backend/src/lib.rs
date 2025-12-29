//! Core library for the Time Grasp application.
//!
//! This crate exposes the core application layers used by the Tauri
//! frontend: context helpers, model types, persistence/store adapters, and
//! a small backend-model-controller (BMC) layer used by IPC handlers.
//!
//! The module layout is intentionally small and focused so application code
//! can import only the pieces it needs.
#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::sync::Arc;

pub mod bmc;
pub mod ctx;
pub mod error;
pub mod ipc;
pub mod store;
