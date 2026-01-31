//! Models for time entries.
//!
//! Types in this module represent the database row for time entries and
//! small DTOs used for create/update/delete operations. All types implement
//! `sqlx::FromRow` to make mapping between DB rows and domain structs easy.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{DurationMillis, EpochMillis, FieldUpdate};

/// A recorded time entry in the system.
///
/// Includes identifying metadata, human-facing fields, category association,
/// timing information (start, optional end, and computed duration), and
/// lifecycle timestamps used for creation, updates, and soft-deletion
/// (tombstones). These types are primarily consumed by the
/// persistence/store layer; fields are public for convenient inspection.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// UUID primary key for the entry.
    pub id: Uuid,

    /// The display name for the entry.
    pub name: String,

    /// Optional user-provided note or description.
    pub note: Option<String>,

    /// The category this entry belongs to.
    pub category_id: Uuid,

    /// Start time of the entry, in epoch milliseconds.
    pub start_time: EpochMillis,

    /// Optional end time in epoch milliseconds; `None` indicates an
    /// in-progress entry.
    pub end_time: Option<EpochMillis>,

    /// Optional cached duration in milliseconds. May be `None` until an
    /// entry is completed or the DB calculates it.
    pub duration: Option<DurationMillis>,

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

    /// Optional textual reason for tombstoning this entry.
    pub tombstone_reason: Option<String>,
}

/// Data required to create a new `Entry`.
///
/// This contains the minimal fields needed at insert time. Additional
/// metadata such as timestamps and version are provided by the persistence
/// layer.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryForCreate {
    /// UUID to use for the new entry (client-supplied IDs are supported).
    pub id: Uuid,

    /// Name for the new entry.
    pub name: String,

    /// Optional initial note/description.
    pub note: Option<String>,

    /// Category the entry should belong to.
    pub category_id: Uuid,

    /// Start time for the entry.
    pub start_time: EpochMillis,

    /// Optional end time if the entry is created in a completed state.
    pub end_time: Option<EpochMillis>,
}

/// Fields used to update an existing `Entry`.
///
/// All non-id fields are optional so callers can provide only the fields
/// they want to change. `note` and `end_time` use nested `Option` types to
/// allow distinguishing between "no change" and "clear the value".
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryForUpdate {
    /// The UUID of the entry to update.
    pub id: Uuid,

    /// Optional new name. `None` means "leave unchanged".
    pub name: Option<String>,

    /// Optional new note. `Some(Some(text))` sets a note,
    /// `Some(None)` clears it, and `None` means "leave unchanged".
    pub note: FieldUpdate<String>,

    /// Optional new category id to move the entry to.
    pub category_id: Option<Uuid>,

    /// Optional new start time.
    pub start_time: Option<EpochMillis>,

    /// Optional new end time. `Some(Some(ts))` sets an end time,
    /// `Some(None)` clears it (marks in-progress), and `None` means
    /// "leave unchanged".
    pub end_time: FieldUpdate<EpochMillis>,
}

/// Parameters required to delete (tombstone) an `Entry`.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryForDelete {
    /// The UUID of the entry to delete.
    pub id: Uuid,

    /// Human-readable reason for the tombstone operation.
    pub tombstone_reason: String,
}

/// Filter used when querying entries from storage.
///
/// Currently only supports filtering by id but can be extended with
/// additional predicates as needed.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EntryFilter {
    /// Optional id to restrict queries to a single entry.
    pub id: Option<Uuid>,

    /// bool to return deleted rows
    pub deleted: bool,

    /// Optional sort field.
    pub sort_by: Option<EntrySortField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntrySortField {
    /// Sort by start time.
    StartTime(SortOrder),
    /// Sort by end time.
    EndTime(SortOrder),
    /// Sort by duration.
    Duration(SortOrder),
    /// Sort by name.
    Name(SortOrder),
}

impl std::default::Default for EntrySortField {
    fn default() -> Self {
        EntrySortField::Name(SortOrder::Ascending)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SortOrder {
    /// Ascending order.
    #[default]
    Ascending,
    /// Descending order.
    Descending,
}

impl EntryFilter {
    /// Create a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify the id to filter by.
    pub fn id(mut self, id: Option<Uuid>) -> Self {
        self.id = id;
        self
    }

    /// Specify the sort field.
    pub fn sort_by(mut self, sort_by: Option<EntrySortField>) -> Self {
        self.sort_by = sort_by;
        self
    }
}
