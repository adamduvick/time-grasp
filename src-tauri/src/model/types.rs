use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Wrapper to implement `From`s and `TryFrom`s.
///
/// Notes:
/// - Certain types will need extra impls to get them to decode from the database properly
#[derive(Debug, Clone, sqlx::Type)]
pub struct W<T>(T);

impl From<Vec<u8>> for W<Uuid> {
    fn from(bytes: Vec<u8>) -> Self {
        let uuid = Uuid::from_slice(&bytes).expect("invalid uuid bytes");
        W(uuid)
    }
}

impl From<Vec<u8>> for W<Option<Uuid>> {
    fn from(bytes: Vec<u8>) -> Self {
        match bytes.as_slice() {
            [] => return W(None),
            bytes => {
                let uuid = Uuid::from_slice(&bytes).expect("invalid uuid bytes");
                W(Some(uuid))
            }
        }
    }
}

impl From<Option<Option<DateTime<Utc>>>> for W<Option<DateTime<Utc>>> {
    fn from(dt: Option<Option<DateTime<Utc>>>) -> Self {
        match dt {
            Some(inner) => W(inner),
            None => W(None),
        }
    }
}

/// Payload to create an Entry.
///
/// Notes:
/// - `uuid` is stored as a 16-byte BLOB in SQLite.
/// - Timestamps are provided in UTC by the application; DB enforces monotonicity via CHECKs.
/// - If `category_uuid` is `None`, the DB default applies (system "Uncategorized").
#[derive(Debug, Clone)]
pub struct EntryForCreate {
    pub uuid: Uuid,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>, // must be >= start_time if present
    pub note: Option<String>,
    pub category_uuid: Option<Uuid>,
}

/// Patch-style update. Use optimistic locking at the SQL layer with `WHERE uuid = ? AND version = ?`.
#[derive(Debug, Clone)]
pub struct EntryForUpdate {
    pub uuid: Uuid,
    pub name: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Clearable<DateTime<Utc>>,
    pub note: Clearable<String>,
    pub category_uuid: Clearable<Uuid>,
}

#[derive(Debug, Clone, Copy)]
pub struct EntryForDelete {
    pub uuid: Uuid,
    pub delete_type: DeleteType,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Entry {
    pub uuid: W<Uuid>,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: W<Option<DateTime<Utc>>>,
    pub note: Option<String>,
    pub category_uuid: W<Option<Uuid>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: W<Option<DateTime<Utc>>>,
    pub version: i64,
}

/// Payload to create an Category.
///
/// Notes:
/// - `uuid` is stored as a 16-byte BLOB in SQLite.
/// - Timestamps are provided in UTC by the application; DB enforces monotonicity via CHECKs.
/// - If `group_uuid` is `None`, the DB default applies (system "Ungrouped").
#[derive(Debug, Clone)]
pub struct CategoryForCreate {
    pub uuid: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub group_uuid: Option<Uuid>, // None -> reset to system default
}

/// Patch-style update. Use optimistic locking at the SQL layer with `WHERE uuid = ? AND version = ?`.
#[derive(Debug, Clone)]
pub struct CategoryForUpdate {
    pub uuid: Uuid,
    pub name: Option<String>,
    pub note: Clearable<String>,
    pub group_uuid: Clearable<Uuid>,
}

#[derive(Debug, Clone, Copy)]
pub struct CategoryForDelete {
    pub uuid: Uuid,
    pub delete_type: DeleteType,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Category {
    pub uuid: W<Uuid>,
    pub name: String,
    pub note: Option<String>,
    pub group_uuid: W<Option<Uuid>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: W<Option<DateTime<Utc>>>,
    pub version: i64,
}

/// Payload to create an Category Group.
///
/// Notes:
/// - `uuid` is stored as a 16-byte BLOB in SQLite.
/// - Timestamps are provided in UTC by the application; DB enforces monotonicity via CHECKs.
#[derive(Debug, Clone)]
pub struct CategoryGroupForCreate {
    pub uuid: Uuid,
    pub name: String,
    pub note: Option<String>,
}

/// Patch-style update. Use optimistic locking at the SQL layer with `WHERE uuid = ? AND version = ?`.
#[derive(Debug, Clone)]
pub struct CategoryGroupForUpdate {
    pub uuid: Uuid,
    pub name: Option<String>,
    pub note: Clearable<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct CategoryGroupForDelete {
    pub uuid: Uuid,
    pub delete_type: DeleteType,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CategoryGroup {
    pub uuid: W<Uuid>,
    pub name: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: W<Option<DateTime<Utc>>>,
    pub version: i64,
}

/// Helper alias: Some(Some(T)) = update to value; Some(None) = clear/reset; None = leave untouched.
pub type Clearable<T> = Option<Option<T>>;

#[derive(Debug, Clone, Copy)]
pub enum DeleteType {
    Soft,
    Hard,
}
