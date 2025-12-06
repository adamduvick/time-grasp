//! IPC handlers (Tauri commands) for categories.
//!
//! These commands are thin shims that build a `Ctx` from the provided
//! `AppHandle` and forward the request to the `CategoryBmc` backend-model-
//! controller. See the corresponding `CategoryBmc` methods for implementation
//! details.
use tauri::command;
use tauri::AppHandle;
use tauri::Wry;

use crate::bmc::CategoryBmc;
use crate::ctx::Ctx;
use crate::error::Result;
use model::*;

/// Create a new category.
///
/// For implementation details see [`crate::bmc::CategoryBmc::create`].
#[command]
pub async fn create_category(app: AppHandle<Wry>, data: C_Category) -> Result<Uuid> {
    let ctx = Ctx::from_app(app);
    CategoryBmc::create(ctx, data).await
}

/// Read a category by id.
///
/// For implementation details see [`crate::bmc::CategoryBmc::read`].
#[command]
pub async fn read_category(app: AppHandle<Wry>, id: Uuid) -> Result<R_Category> {
    let ctx = Ctx::from_app(app);
    CategoryBmc::read(ctx, id).await
}

/// List categories that match `filter`.
///
/// For implementation details see [`crate::bmc::CategoryBmc::list`].
#[command]
pub async fn list_category(app: AppHandle<Wry>, filter: CategoryFilter) -> Result<Vec<R_Category>> {
    let ctx = Ctx::from_app(app);
    CategoryBmc::list(ctx, filter).await
}

/// Update a category.
///
/// For implementation details see [`crate::bmc::CategoryBmc::update`].
#[command]
pub async fn update_category(app: AppHandle<Wry>, data: U_Category) -> Result<Uuid> {
    let ctx = Ctx::from_app(app);
    CategoryBmc::update(ctx, data).await
}

/// Delete (tombstone) a category.
///
/// For implementation details see [`crate::bmc::CategoryBmc::delete`].
#[command]
pub async fn delete_category(app: AppHandle<Wry>, data: D_Category) -> Result<Uuid> {
    let ctx = Ctx::from_app(app);
    CategoryBmc::delete(ctx, data).await
}
