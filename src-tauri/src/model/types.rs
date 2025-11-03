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
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>, // must be >= start_time if present
    pub note: Option<String>,
    pub category_global_id: Option<Uuid>, // None -> reset to system default
}

/// Patch-style update. Use optimistic locking at the SQL layer with `WHERE global_id = ? AND version = ?`.
#[derive(Debug, Clone)]
pub struct EntryForUpdate {
    pub global_id: Uuid,
    pub name: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Clearable<DateTime<Utc>>,
    pub note: Clearable<String>,
    pub category_global_id: Clearable<Uuid>,
}

#[derive(Debug, Clone, Copy)]
pub struct EntryForDelete {
    pub global_id: Uuid,
    pub delete_type: DeleteType,
}

/// Payload to create an Category.
///
/// Notes:
/// - `global_id` is stored as a 16-byte BLOB in SQLite.
/// - Timestamps are provided in UTC by the application; DB enforces monotonicity via CHECKs.
/// - If `group_global_id` is `None`, the DB default applies (system "Ungrouped").
#[derive(Debug, Clone)]
pub struct CategoryForCreate {
    pub global_id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub group_global_id: Option<Uuid>, // None -> reset to system default
}

/// Patch-style update. Use optimistic locking at the SQL layer with `WHERE global_id = ? AND version = ?`.
#[derive(Debug, Clone)]
pub struct CategoryForUpdate {
    pub global_id: Uuid,
    pub name: Option<String>,
    pub note: Clearable<String>,
    pub group_global_id: Clearable<Uuid>,
}

#[derive(Debug, Clone, Copy)]
pub struct CategoryForDelete {
    pub global_id: Uuid,
    pub delete_type: DeleteType,
}

/// Payload to create an Category Group.
///
/// Notes:
/// - `global_id` is stored as a 16-byte BLOB in SQLite.
/// - Timestamps are provided in UTC by the application; DB enforces monotonicity via CHECKs.
#[derive(Debug, Clone)]
pub struct CategoryGroupForCreate {
    pub global_id: Uuid,
    pub name: String,
    pub note: Option<String>,
}

/// Patch-style update. Use optimistic locking at the SQL layer with `WHERE global_id = ? AND version = ?`.
#[derive(Debug, Clone)]
pub struct CategoryGroupForUpdate {
    pub global_id: Uuid,
    pub name: Option<String>,
    pub note: Clearable<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct CategoryGroupForDelete {
    pub global_id: Uuid,
    pub delete_type: DeleteType,
}

/// Helper alias: Some(Some(T)) = update to value; Some(None) = clear/reset; None = leave untouched.
pub type Clearable<T> = Option<Option<T>>;

#[derive(Debug, Clone, Copy)]
pub enum DeleteType {
    Soft,
    Hard,
}
