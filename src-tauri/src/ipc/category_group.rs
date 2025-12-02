//! IPC handlers (Tauri commands) for category groups.
//!
//! These commands forward to the [`GroupBmc`][`crate::bmc::GroupBmc`] backend-model-controller.
//! See the corresponding [`GroupBmc`][`crate::bmc::GroupBmc`] methods for implementation details
//! and emitted events.
use tauri::command;
use tauri::AppHandle;
use tauri::Wry;

use crate::bmc::GroupBmc;
use crate::ctx::Ctx;
use crate::error::Result;
use crate::model::*;

/// Create a new category group.
///
/// For implementation details see [`GroupBmc::create`][`crate::bmc::GroupBmc::create`].
#[command]
pub async fn create_group(app: AppHandle<Wry>, data: C_Group) -> Result<Uuid> {
    let ctx = Ctx::from_app(app);
    GroupBmc::create(ctx, data).await
}

/// Read a category group by id.
///
/// For implementation details see [`GroupBmc::read`][`crate::bmc::GroupBmc::read`].
#[command]
pub async fn read_group(app: AppHandle<Wry>, id: Uuid) -> Result<R_Group> {
    let ctx = Ctx::from_app(app);
    GroupBmc::read(ctx, id).await
}

/// List category groups matching `filter`.
///
/// For implementation details see [`GroupBmc::list`][`crate::bmc::GroupBmc::list`].
#[command]
pub async fn list_group(app: AppHandle<Wry>, filter: CategoryGroupFilter) -> Result<Vec<R_Group>> {
    let ctx = Ctx::from_app(app);
    GroupBmc::list(ctx, filter).await
}

/// Update a category group.
///
/// For implementation details see [`GroupBmc::update`][`crate::bmc::GroupBmc::update`].
#[command]
pub async fn update_group(app: AppHandle<Wry>, data: U_Group) -> Result<Uuid> {
    let ctx = Ctx::from_app(app);
    GroupBmc::update(ctx, data).await
}

/// Delete (tombstone) a category group.
///
/// For implementation details see [`GroupBmc::delete`][`crate::bmc::GroupBmc::delete`].
#[command]
pub async fn delete_group(app: AppHandle<Wry>, data: D_Group) -> Result<Uuid> {
    let ctx = Ctx::from_app(app);
    GroupBmc::delete(ctx, data).await
}
