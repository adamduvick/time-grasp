//! Application request context and helper utilities.
//!
//! `Ctx` is the lightweight context object passed through IPC handlers and
//! other application entry points. It exposes short-lived accessors for
//! shared services (for example the `StoreManager`) and convenience helpers
//! such as event emission. This module intentionally keeps the context
//! surface small; further abstractions (traits) can be introduced later to
//! support mocking in tests or richer authorization/tracing in cloud
//! deployments.
//!
//! Notes:
//! - Simple implementation for now.
//! - For cloud applications, this will be used for authorization.
//! - Eventually, this will also be used for "full context" logging/tracing or even performance tracing.
//! - For a single user, desktop application, this object is much simpler as authorization and logging requirements are much reduced.
use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Wry};

use crate::model::event::HubEvent;
use crate::store::StoreManager;

pub struct Ctx {
    store_manager: Arc<StoreManager>,
    app_handle: AppHandle<Wry>,
}

impl Ctx {
    /// Construct a `Ctx` from a Tauri `AppHandle`.
    ///
    /// The `AppHandle` is used to retrieve shared state (for example
    /// an `Arc<StoreManager>`) which is stored inside the context for
    /// later use by handlers.
    pub fn new(app_handle: AppHandle<Wry>) -> Self {
        Ctx {
            store_manager: (*app_handle.state::<Arc<StoreManager>>()).clone(),
            app_handle,
        }
    }

    /// Convenience constructor that allocates the context on the heap and
    /// returns an `Arc<Ctx>` ready to be stored or passed around.
    pub fn from_app(app: AppHandle<Wry>) -> Arc<Ctx> {
        Arc::new(Ctx::new(app))
    }

    /// Return a cloned `Arc<StoreManager>` so callers may access storage.
    pub fn get_store_manager(&self) -> Arc<StoreManager> {
        self.store_manager.clone()
    }

    /// Emit a hub event to any interested listeners.
    ///
    /// This uses Tauri's event emitter; errors are intentionally ignored as
    /// emission is fire-and-forget in the current application design.
    pub fn emit_hub_event<D: Serialize + Clone>(&self, hub_event: HubEvent<D>) {
        // Emit the event to all listeners.
        // Send and forget; ignore any errors.
        let _ = self.app_handle.emit("HubEvent", hub_event);
    }
}

// TODO: extract get_store_manager and emit_hub_event into a trait for easier mocking in tests.
//
// ```rust
// pub trait CtxTrait {
//     fn get_store_manager(&self) -> Arc<StoreManager>;
//     fn emit_hub_event<D: Serialize + Clone>(&self, hub_event: HubEvent<D>);
// }
// ```
//
// or EventSink trait for emit_hub_event only.
// ```rust
// pub trait EventSink {
//     fn emit_hub_event<D: Serialize + Clone>(&self, hub_event: HubEvent<D>);
// }
// ```
//
// and ModelStoreProvider trait for get_store_manager only.
// ```rust
// pub trait ModelStoreProvider {
//     fn get_store_manager(&self) -> Arc<StoreManager>;
// }
// ```
