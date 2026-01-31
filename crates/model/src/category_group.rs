//! Models for category groups.
//!
//! Types in this module represent the database row for category groups and
//! small DTOs used for create/update/delete operations. All types implement
//! `sqlx::FromRow` to make mapping between DB rows and domain structs easy.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{EpochMillis, FieldUpdate};

/// A full representation of a category group as stored in the database.
///
/// Includes stable identifiers, human-facing fields, optimistic-concurrency
/// versioning, and lifecycle timestamps used for creation, updates, and
/// soft-deletion (tombstones). These types are primarily consumed by the
/// persistence/store layer; fields are public for convenient inspection.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryGroup {
    /// Stable UUID primary key for the category group.
    pub id: Uuid,

    /// The display name for the group.
    pub name: String,

    /// Optional user-provided note or description.
    pub note: Option<String>,

    /// Logical version used for optimistic concurrency or change tracking.
    pub version: i32,

    /// Epoch milliseconds when the record was created.
    pub created_at: EpochMillis,

    /// Epoch milliseconds when the record was last updated.
    pub updated_at: EpochMillis,

    /// Optional deletion timestamp (soft-delete / tombstone).
    pub deleted_at: Option<EpochMillis>,

    /// If deleted, the user UUID that performed the deletion.
    pub deleted_by_user: Option<Uuid>,

    /// If deleted, the device UUID that performed the deletion.
    pub deleted_by_device: Option<Uuid>,

    /// Optional textual reason for tombstoning this group.
    pub tombstone_reason: Option<String>,
}

/// Lightweight struct used when creating a new `CategoryGroup`.
///
/// Contains only the fields required at insert time. Timestamps and
/// server-generated metadata are added by the persistence layer.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryGroupForCreate {
    /// UUID to use for the new group (allows client-supplied IDs).
    pub id: Uuid,

    /// Desired name for the new group.
    pub name: String,

    /// Optional initial note/description.
    pub note: Option<String>,
}

/// Fields used to update an existing `CategoryGroup`.
///
/// All fields are optional so callers can supply only the values they want
/// to change. `note` is an `Option<Option<String>>` to allow differentiating
/// between "no change" (None) and "set to null/clear" (Some(None)).
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryGroupForUpdate {
    /// The UUID of the group to update.
    pub id: Uuid,

    /// Optional new name. `None` means "leave unchanged".
    pub name: Option<String>,

    /// Optional new note. `Some(Some(text))` sets a note,
    /// `Some(None)` clears it, and `None` means "leave unchanged".
    pub note: FieldUpdate<String>,
}

/// Parameters required to delete (tombstone) a `CategoryGroup`.
///
/// The persistence layer uses this to mark a group as deleted and record
/// a textual reason for the tombstone.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryGroupForDelete {
    /// The UUID of the group to delete.
    pub id: Uuid,

    /// Human-readable reason for the tombstone operation.
    pub tombstone_reason: String,
}

/// Filter used when querying category groups from storage.
///
/// Currently minimal; supports filtering by id. Additional predicates can be
/// added as needed for searches and list endpoints.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CategoryGroupFilter {
    /// Optional id to restrict queries to a single group.
    pub id: Option<Uuid>,
}

impl CategoryGroupFilter {
    /// Create a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify the id to filter by.
    pub fn id(mut self, id: Option<Uuid>) -> Self {
        self.id = id;
        self
    }
}
