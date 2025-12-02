//! Store adapters and re-exports.
//!
//! This module groups the individual store adapters and re-exports the
//! commonly-used types (for example the `StoreManager` and store traits)
//! so callers can `use crate::store::*` to access storage-related APIs.
pub mod category;
pub mod category_group;
pub mod entry;
pub mod manager;
pub mod traits;

pub use crate::error::Result;
pub use manager::*;
pub use traits::*;

#[cfg(test)]
pub use category::tests::create_and_read as create_and_read_category;
#[cfg(test)]
pub use category_group::tests::create_and_read as create_and_read_group;
