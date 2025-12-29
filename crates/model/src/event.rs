//! Event envelope used to emit domain events to the UI or other listeners.
//!
//! `HubEvent` is a small, serializable wrapper for publishing typed events
//! from the model layer. It carries a hub name, topic, optional label and
//! optional typed payload.
use serde::{Deserialize, Serialize};

/// A serializable envelope for model-layer events.
///
/// `D` is the optional payload type and must implement `Serialize` and
/// `Clone` so events can be emitted across threads and serialized for
/// transport to the frontend (Tauri) or persisted if needed.
#[derive(Serialize, Clone, Deserialize)]
pub struct HubEvent<D: Serialize + Clone> {
    /// Logical hub name for the event (e.g. "Model").
    pub hub: String,

    /// Topic identifying the entity or subsystem the event is about.
    pub topic: String,

    /// Optional short label (for example the action name such as "create").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Optional typed event payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>,
}
