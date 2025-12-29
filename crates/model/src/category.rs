//! Models for categories.
//!
//! Types in this module represent the database row for categories and small
//! DTOs used for create/update/delete operations. All types implement
//! `sqlx::FromRow` to make mapping between DB rows and domain structs easy.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::EpochMillis;

/// A complete category record as stored in the database.
///
/// Includes stable identifiers, human-facing fields, the group the category
/// belongs to, optimistic-concurrency versioning, and lifecycle timestamps
/// used for creation, updates, and soft-deletion (tombstones). These types
/// are primarily consumed by the persistence/store layer; fields are public
/// for convenient inspection.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    /// UUID primary key for the category.
    pub id: Uuid,

    /// The display name for the category.
    pub name: String,

    /// Optional user-provided note or description.
    pub note: Option<String>,

    /// UUID of the `CategoryGroup` this category belongs to.
    pub group_id: Uuid,

    /// Logical version used for optimistic concurrency or change tracking.
    pub version: i32,

    /// Creation timestamp in epoch milliseconds.
    pub created_at: EpochMillis,

    /// Last-updated timestamp in epoch milliseconds.
    pub updated_at: EpochMillis,

    /// Optional deletion timestamp (soft-delete / tombstone semantics).
    pub deleted_at: Option<EpochMillis>,

    /// If deleted, the user UUID that performed the deletion.
    pub deleted_by_user: Option<Uuid>,

    /// If deleted, the device UUID that performed the deletion.
    pub deleted_by_device: Option<Uuid>,

    /// Optional textual reason for tombstoning this category.
    pub tombstone_reason: Option<String>,
}

/// Data required to create a new `Category`.
///
/// This contains the minimal fields needed at insert time. Additional
/// metadata such as timestamps and version are provided by the persistence
/// layer.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryForCreate {
    /// UUID to use for the new category (client-supplied IDs are supported).
    pub id: Uuid,

    /// Name for the new category.
    pub name: String,

    /// Optional initial note/description.
    pub note: Option<String>,

    /// The group this category should belong to.
    pub group_id: Uuid,
}

/// Fields used to update an existing `Category`.
///
/// All non-id fields are optional so callers can provide only the fields
/// they want to change. `note` uses `Option<Option<String>>` to allow
/// distinguishing between "no change" and "clear note".
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryForUpdate {
    /// The UUID of the category to update.
    pub id: Uuid,

    /// Optional new name. `None` means "leave unchanged".
    pub name: Option<String>,

    /// Optional new note. `Some(Some(text))` sets a note,
    /// `Some(None)` clears it, and `None` means "leave unchanged".
    pub note: Option<Option<String>>,

    /// Optional new group id to move the category to.
    pub group_id: Option<Uuid>,
}

/// Parameters required to delete (tombstone) a `Category`.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryForDelete {
    /// The UUID of the category to delete.
    pub id: Uuid,

    /// Human-readable reason for the tombstone operation.
    pub tombstone_reason: String,
}

/// Filter used when querying categories from storage.
///
/// Currently only supports filtering by id but can be extended with
/// additional predicates as needed.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryFilter {
    /// Optional id to restrict queries to a single category.
    pub id: Option<Uuid>,
}

impl Default for CategoryFilter {
    fn default() -> Self {
        Self { id: None }
    }
}

impl CategoryFilter {
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
