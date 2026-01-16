// use chrono::{DateTime, NaiveDateTime, Utc};
use leptos::prelude::*;
use model as dto;
use model::{DurationMillis, Uuid};
use reactive_stores::{Field, Patch, PatchField, Store, StorePath};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use time::{Duration, OffsetDateTime, PrimitiveDateTime, UtcDateTime, UtcOffset};

use crate::model::support::get_timezone_offset;
use crate::model::timeframe::TimeFrame;

static TOMBSTONE_REASON: &str = "Deleted from frontend";

// region:      --- entry

#[derive(Clone, Debug, Store, PartialEq)]
pub struct Entry {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub category_id: Uuid,
    pub time_frame: TimeFrame,
}

impl Entry {
    pub fn new(name: String) -> Entry {
        let id = Uuid::new_v4();
        let category_id = Uuid::parse_str("67607575-3ec7-49c0-a352-78a9358ede39").unwrap();
        let time_frame = TimeFrame::with_utc_start_and_end(time::UtcDateTime::now().into(), None)
            .expect("converting back to UTC type should never fail");
        Entry {
            id,
            name,
            note: None,
            category_id,
            time_frame,
        }
    }

    pub fn with_offset(&mut self, offset: UtcOffset) {
        leptos::logging::log!("Entry::with_offset {offset:?}");
        self.time_frame.assume_offset(offset);
    }
}

impl Updatable<dto::U_Entry> for Entry {
    fn new_update_dto(&self, new: &Self) -> dto::U_Entry {
        dto::U_Entry {
            id: self.id,
            name: if self.name != new.name {
                Some(new.name.clone())
            } else {
                None
            },
            note: if self.note != new.note {
                match &new.note {
                    Some(v) => model::FieldUpdate::Set(v.to_string()),
                    None => model::FieldUpdate::Clear,
                }
            } else {
                model::FieldUpdate::Unchanged
            },
            category_id: if self.category_id != new.category_id {
                Some(new.category_id)
            } else {
                None
            },
            start_time: if self.time_frame != new.time_frame {
                Some(new.time_frame.get_utc_start_time().into())
            } else {
                None
            },
            end_time: if self.time_frame != new.time_frame {
                Some(new.time_frame.get_utc_end_time().map(|dt| dt.into()))
            } else {
                None
            },
        }
    }
}

impl TryFrom<dto::R_Entry> for Entry {
    type Error = crate::error::Error;

    fn try_from(value: dto::R_Entry) -> Result<Self, Self::Error> {
        Ok(Entry {
            id: value.id,
            name: value.name,
            note: value.note,
            category_id: value.category_id,
            time_frame: TimeFrame::with_utc_start_and_end(value.start_time, value.end_time)?,
        })
    }
}

impl From<Entry> for dto::C_Entry {
    fn from(value: Entry) -> Self {
        dto::C_Entry {
            id: value.id,
            name: value.name,
            note: value.note,
            category_id: value.category_id,
            start_time: value.time_frame.get_utc_start_time().into(),
            end_time: value.time_frame.get_utc_end_time().map(|dt| dt.into()),
        }
    }
}

impl From<Entry> for dto::D_Entry {
    fn from(value: Entry) -> Self {
        dto::D_Entry {
            id: value.id,
            tombstone_reason: TOMBSTONE_REASON.into(),
        }
    }
}

// endregion:   --- entry

// region:      --- category

#[derive(Clone, Debug, Store, PartialEq)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
    pub group_id: Uuid,
}

impl Updatable<dto::U_Category> for Category {
    fn new_update_dto(&self, new: &Self) -> dto::U_Category {
        dto::U_Category {
            id: self.id,
            name: if self.name != new.name {
                Some(new.name.clone())
            } else {
                None
            },
            note: if self.note != new.note {
                todo!()
            } else {
                todo!()
            },
            group_id: if self.group_id != new.group_id {
                Some(new.group_id)
            } else {
                None
            },
        }
    }
}

impl Into<Category> for dto::R_Category {
    fn into(self) -> Category {
        Category {
            id: self.id,
            name: self.name,
            note: self.note,
            group_id: self.group_id,
        }
    }
}

impl Into<dto::C_Category> for Category {
    fn into(self) -> dto::C_Category {
        dto::C_Category {
            id: self.id,
            name: self.name,
            note: self.note,
            group_id: self.group_id,
        }
    }
}

impl Into<dto::D_Category> for Category {
    fn into(self) -> dto::D_Category {
        dto::D_Category {
            id: self.id,
            tombstone_reason: TOMBSTONE_REASON.into(),
        }
    }
}

// endregion:   --- category

// region:      --- group

#[derive(Clone, Debug, Store, PartialEq)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub note: Option<String>,
}

impl Updatable<dto::U_Group> for Group {
    fn new_update_dto(&self, new: &Self) -> dto::U_Group {
        dto::U_Group {
            id: self.id,
            name: if self.name != new.name {
                Some(new.name.clone())
            } else {
                None
            },
            note: if self.note != new.note {
                todo!()
            } else {
                todo!()
            },
        }
    }
}

impl Into<Group> for dto::R_Group {
    fn into(self) -> Group {
        Group {
            id: self.id,
            name: self.name,
            note: self.note,
        }
    }
}

impl Into<dto::C_Group> for Group {
    fn into(self) -> dto::C_Group {
        dto::C_Group {
            id: self.id,
            name: self.name,
            note: self.note,
        }
    }
}

impl Into<dto::D_Group> for Group {
    fn into(self) -> dto::D_Group {
        dto::D_Group {
            id: self.id,
            tombstone_reason: TOMBSTONE_REASON.into(),
        }
    }
}

// endregion:   --- group

// region:      --- PatchField impls

impl PatchField for Entry {
    fn patch_field(&mut self, new: Self, path: &StorePath, notify: &mut dyn FnMut(&StorePath)) {
        leptos::logging::log!("Entry::patch_field called: {:?}", self);
        if new != *self {
            leptos::logging::log!("Entry::patch_field change detected");
            *self = new;
            notify(path);
        } else {
            leptos::logging::log!("Entry::patch_field no change");
        }
    }
}

impl PatchField for Category {
    fn patch_field(&mut self, new: Self, path: &StorePath, notify: &mut dyn FnMut(&StorePath)) {
        if new != *self {
            *self = new;
            notify(path);
        }
    }
}

impl PatchField for Group {
    fn patch_field(&mut self, new: Self, path: &StorePath, notify: &mut dyn FnMut(&StorePath)) {
        if new != *self {
            *self = new;
            notify(path);
        }
    }
}

// endregion:   --- PatchField impls

// region:     --- helpers

pub trait Updatable<T> {
    fn new_update_dto(&self, new: &Self) -> T;
}

// endregion:  --- helpers
