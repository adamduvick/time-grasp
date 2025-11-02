use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct EntryForCreate {
    pub global_id: Uuid,
    pub payee: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub note: Option<String>,
    pub category_global_id: Option<Uuid>,
}

pub struct EntryForUpdate {
    pub global_id: Uuid,
    pub payee: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<Option<DateTime<Utc>>>, // Some(None) clears end_time
    pub note: Option<Option<String>>,            // Some(None) clears note
    pub category_global_id: Option<Option<Uuid>>, // Some(None) clears category
}

pub struct EntryForDelete {
    pub global_id: Uuid,
    pub delete_type: DeleteType,
}

pub enum DeleteType {
    Soft,
    Hard,
}
