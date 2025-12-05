//! Domain models and DTOs used across the crate.
//!
//! This module exposes the application domain types for categories, category
//! groups, and entries along with their CRUD-oriented helper types. Conventions
//! used here:
//!
//! - Types prefixed with `C_` are create/insert payloads (consumed to create
//!   a new record).
//! - Types prefixed with `R_` are read/view models returned from the store.
//! - Types prefixed with `U_` are update payloads (partial updates supported
//!   by optional fields).
//! - Types prefixed with `D_` are delete/tombstone payloads.
//!
//! Common helper types such as `EpochMillis` and `DurationMillis` live in
//! `model::support` and are re-exported here for convenience.
//!
//! Consumers should prefer the `R_` types for read operations and `C_/U_/D_`
//! types when performing store mutations.
// pub mod category;
// pub mod category_group;
// pub mod entry;
// pub mod event;
// pub mod support;

// pub use category::*;
// pub use category_group::*;
// pub use entry::*;
// pub use event::*;
// pub use support::*;
// pub use uuid::Uuid;

// // CRUD-type re-exported aliases for CategoryGroup
// pub use category_group::CategoryGroup as R_Group;
// pub use category_group::CategoryGroupForCreate as C_Group;
// pub use category_group::CategoryGroupForDelete as D_Group;
// pub use category_group::CategoryGroupForUpdate as U_Group;

// // CRUD-type re-exported aliases for Category
// pub use category::Category as R_Category;
// pub use category::CategoryForCreate as C_Category;
// pub use category::CategoryForDelete as D_Category;
// pub use category::CategoryForUpdate as U_Category;

// // CRUD-type re-exported aliases for Entry
// pub use entry::Entry as R_Entry;
// pub use entry::EntryForCreate as C_Entry;
// pub use entry::EntryForDelete as D_Entry;
// pub use entry::EntryForUpdate as U_Entry;

pub use model::model::*;
