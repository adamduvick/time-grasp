//! IPC surface for the application — Tauri command handlers.
//!
//! This module exposes the IPC (Tauri) entry points used by the frontend to
//! interact with the application. Handlers in this module are intentionally
//! thin: they translate incoming payloads, construct a [`Ctx`][`crate::ctx::Ctx`] from the
//! [`AppHandle`][`tauri::AppHandle`], and forward the request to the appropriate [backend-model
//! controller (BMC)][`crate::bmc`] which performs store operations and emits events.
//!
//! Exposed submodules map to domain areas and are re-exported here for
//! convenient `use` from other crates or integration tests.
use tauri::AppHandle;
use tauri::Wry;
use tauri::command;

use crate::bmc::{CategoryBmc, EntryBmc, GroupBmc};
use crate::ctx::Ctx;
use crate::error::Result;
use model::*;

/// Create a new category group.
///
/// For implementation details see [`GroupBmc::create`][`crate::bmc::GroupBmc::create`].
#[command]
pub async fn create_group(app: AppHandle<Wry>, data: C_Group) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    GroupBmc::create(ctx, data).await
}

/// Read a category group by id.
///
/// For implementation details see [`GroupBmc::read`][`crate::bmc::GroupBmc::read`].
#[command]
pub async fn read_group(app: AppHandle<Wry>, id: Uuid) -> Result<R_Group> {
    let ctx = Ctx::from_app(app)?;
    GroupBmc::read(ctx, id).await
}

/// List category groups matching `filter`.
///
/// For implementation details see [`GroupBmc::list`][`crate::bmc::GroupBmc::list`].
#[command]
pub async fn list_group(app: AppHandle<Wry>, filter: CategoryGroupFilter) -> Result<Vec<R_Group>> {
    let ctx = Ctx::from_app(app)?;
    GroupBmc::list(ctx, filter).await
}

/// Update a category group.
///
/// For implementation details see [`GroupBmc::update`][`crate::bmc::GroupBmc::update`].
#[command]
pub async fn update_group(app: AppHandle<Wry>, data: U_Group) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    GroupBmc::update(ctx, data).await
}

/// Delete (tombstone) a category group.
///
/// For implementation details see [`GroupBmc::delete`][`crate::bmc::GroupBmc::delete`].
#[command]
pub async fn delete_group(app: AppHandle<Wry>, data: D_Group) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    GroupBmc::delete(ctx, data).await
}

/// Create a new category.
///
/// For implementation details see [`crate::bmc::CategoryBmc::create`].
#[command]
pub async fn create_category(app: AppHandle<Wry>, data: C_Category) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    CategoryBmc::create(ctx, data).await
}

/// Read a category by id.
///
/// For implementation details see [`crate::bmc::CategoryBmc::read`].
#[command]
pub async fn read_category(app: AppHandle<Wry>, id: Uuid) -> Result<R_Category> {
    let ctx = Ctx::from_app(app)?;
    CategoryBmc::read(ctx, id).await
}

/// List categories that match `filter`.
///
/// For implementation details see [`crate::bmc::CategoryBmc::list`].
#[command]
pub async fn list_category(app: AppHandle<Wry>, filter: CategoryFilter) -> Result<Vec<R_Category>> {
    let ctx = Ctx::from_app(app)?;
    CategoryBmc::list(ctx, filter).await
}

/// Update a category.
///
/// For implementation details see [`crate::bmc::CategoryBmc::update`].
#[command]
pub async fn update_category(app: AppHandle<Wry>, data: U_Category) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    CategoryBmc::update(ctx, data).await
}

/// Delete (tombstone) a category.
///
/// For implementation details see [`crate::bmc::CategoryBmc::delete`].
#[command]
pub async fn delete_category(app: AppHandle<Wry>, data: D_Category) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    CategoryBmc::delete(ctx, data).await
}

/// Create a new entry.
///
/// For implementation details see [`crate::bmc::EntryBmc::create`].
#[command]
pub async fn create_entry(app: AppHandle<Wry>, data: C_Entry) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    EntryBmc::create(ctx, data).await
}

/// Read an entry by id.
///
/// For implementation details see [`crate::bmc::EntryBmc::read`].
#[command]
pub async fn read_entry(app: AppHandle<Wry>, id: Uuid) -> Result<R_Entry> {
    let ctx = Ctx::from_app(app)?;
    EntryBmc::read(ctx, id).await
}

/// List entries matching `filter`.
///
/// For implementation details see [`crate::bmc::EntryBmc::list`].
#[command]
pub async fn list_entry(app: AppHandle<Wry>, filter: EntryFilter) -> Result<Vec<R_Entry>> {
    let ctx = Ctx::from_app(app)?;
    EntryBmc::list(ctx, filter).await
}

/// Update an entry.
///
/// For implementation details see [`crate::bmc::EntryBmc::update`].
#[command]
pub async fn update_entry(app: AppHandle<Wry>, data: U_Entry) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    EntryBmc::update(ctx, data).await
}

/// Delete (tombstone) an entry.
///
/// For implementation details see [`crate::bmc::EntryBmc::delete`].
#[command]
pub async fn delete_entry(app: AppHandle<Wry>, data: D_Entry) -> Result<Uuid> {
    let ctx = Ctx::from_app(app)?;
    EntryBmc::delete(ctx, data).await
}
