//! IPC handlers (Tauri commands) for entries.
//!
//! These commands forward requests to the `EntryBmc` backend-model-controller
//! which implements the actual store logic and emits hub events. Each command
//! documents the `EntryBmc` method it delegates to so readers can follow the
//! implementation via the generated docs.
use tauri::AppHandle;
use tauri::Wry;
use tauri::command;

use crate::bmc::EntryBmc;
use crate::ctx::Ctx;
use crate::error::Result;
use model::*;

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
