use chrono::{DateTime, NaiveDateTime, Utc};
use model as dto;
use model::{DurationMillis, Uuid};
use reactive_stores::{Field, Patch, PatchField, Store};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Store, PartialEq)]
pub struct Entry {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub category_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<DurationMillis>,
}

impl PatchField for Entry {
    fn patch_field(
        &mut self,
        new: Self,
        path: &reactive_stores::StorePath,
        notify: &mut dyn FnMut(&reactive_stores::StorePath),
    ) {
        if new != *self {
            *self = new;
            notify(path);
        }
    }
}

impl Entry {
    pub fn new(name: String) -> Entry {
        let id = Uuid::new_v4();
        let category_id = Uuid::parse_str("e0294da8-bfca-476a-a943-5a28f5771928").unwrap();
        let start_time = Utc::now();
        Entry {
            id,
            name,
            note: None,
            category_id,
            start_time,
            end_time: None,
            duration: None,
        }
    }
}

impl Entry {
    pub fn create_update_dto(&self, prev: &Entry) -> dto::U_Entry {
        dto::U_Entry {
            id: self.id,
            name: if self.name != prev.name {
                Some(self.name.clone())
            } else {
                None
            },
            note: if self.note != prev.note {
                match &self.note {
                    Some(v) => model::FieldUpdate::Set(v.to_string()),
                    None => model::FieldUpdate::Clear,
                }
            } else {
                model::FieldUpdate::Unchanged
            },
            category_id: if self.category_id != prev.category_id {
                Some(self.category_id)
            } else {
                None
            },
            start_time: if self.start_time != prev.start_time {
                Some(self.start_time.into())
            } else {
                None
            },
            end_time: if self.end_time != prev.end_time {
                Some(self.end_time.map(|dt| dt.into()))
            } else {
                None
            },
        }
    }
}

// implement conversions from model::Entry to fontend Entry
//
impl Into<Entry> for dto::R_Entry {
    fn into(self) -> Entry {
        Entry {
            id: self.id,
            name: self.name,
            note: self.note,
            category_id: self.category_id,
            start_time: self.start_time.into(),
            end_time: self.end_time.map(|dt| dt.into()),
            duration: self.duration,
        }
    }
}

impl Into<dto::C_Entry> for Entry {
    fn into(self) -> dto::C_Entry {
        dto::C_Entry {
            id: self.id,
            name: self.name,
            note: self.note,
            category_id: self.category_id,
            start_time: self.start_time.into(),
            end_time: self.end_time.map(|dt| dt.into()),
        }
    }
}

impl Into<dto::D_Entry> for Entry {
    fn into(self) -> dto::D_Entry {
        dto::D_Entry {
            id: self.id,
            tombstone_reason: "Deleted from frontend".to_string(),
        }
    }
}
