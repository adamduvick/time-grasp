use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Payload to create an Entry.
///
/// Notes:
/// - `global_id` is stored as a 16-byte BLOB in SQLite.
/// - Timestamps are provided in UTC by the application; DB enforces monotonicity via CHECKs.
/// - If `category_global_id` is `None`, the DB default applies (system "Uncategorized").
#[derive(Debug, Clone)]
pub struct EntryForCreate {
    pub global_id: Uuid,
    pub payee: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>, // must be >= start_time if present
    pub note: Option<String>,
    pub category_global_id: Option<Uuid>, // None -> reset to system default
}

/// Helper alias: Some(Some(T)) = update to value; Some(None) = clear/reset; None = leave untouched.
pub type Clearable<T> = Option<Option<T>>;

/// Patch-style update. Use optimistic locking at the SQL layer with `WHERE global_id = ? AND version = ?`.
#[derive(Debug, Clone)]
pub struct EntryForUpdate {
    pub global_id: Uuid,
    pub payee: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Clearable<DateTime<Utc>>, // Some(None) clears end_time
    pub note: Clearable<String>,            // Some(None) clears note
    /// Some(Some(uuid)) -> set to that category; Some(None) -> reset to DB default; None -> unchanged
    pub category_global_id: Clearable<Uuid>,
}

#[derive(Debug, Clone, Copy)]
pub struct EntryForDelete {
    pub global_id: Uuid,
    pub delete_type: DeleteType,
}

#[derive(Debug, Clone, Copy)]
pub enum DeleteType {
    Soft,
    Hard,
}
