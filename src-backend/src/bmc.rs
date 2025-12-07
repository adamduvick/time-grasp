//! Backend-model-controller (BMC) helpers.
//!
//! This module provides small, focused helpers that glue together the
//! context (`Ctx`), store traits (`Creatable`, `Readable`, etc.), and event
//! emission to provide simple CRUD operations for model entities. The
//! functions are intentionally minimal and designed to be used by IPC
//! handlers or higher-level controllers.
use std::sync::Arc;

use serde::Serialize;

use crate::ctx::Ctx;
use crate::error::Result;
use crate::store::*;
use model::*;

// region:      --- BMC helpers

/// Helper to build a `HubEvent` for BMC operations.
fn hub_event<D: Serialize + Clone>(
    entity: &'static str,
    action: &'static str,
    data: D,
) -> HubEvent<D> {
    HubEvent {
        hub: "Model".to_string(),
        topic: entity.to_string(),
        label: Some(action.to_string()),
        data: Some(data),
    }
}

/// Generic create helper used internally by the BMC wrappers.
async fn create<C: Creatable<C>>(ctx: Arc<Ctx>, entity: &'static str, data: C) -> Result<Uuid> {
    let pool = ctx.get_store_manager().pool();
    let id = C::create(&pool, data).await?;
    let event = hub_event(entity, "create", id);
    ctx.emit_hub_event(event);
    Ok(id)
}

/// Generic read helper used internally by the BMC wrappers.
async fn read<R: Readable<R>>(ctx: Arc<Ctx>, _entity: &'static str, id: Uuid) -> Result<R> {
    let pool = ctx.get_store_manager().pool();
    let record = R::read(&pool, id).await?;
    // No event emitted for read-only operations
    Ok(record)
}

/// Generic list helper used internally by the BMC wrappers.
async fn list<R: Readable<R>>(
    ctx: Arc<Ctx>,
    _entity: &'static str,
    filter: R::Filter,
) -> Result<Vec<R>> {
    let pool = ctx.get_store_manager().pool();
    let records = R::list(&pool, filter).await?;
    // No event emitted for read-only operations
    Ok(records)
}

/// Generic update helper used internally by the BMC wrappers.
async fn update<U: Updatable<U>>(ctx: Arc<Ctx>, entity: &'static str, data: U) -> Result<Uuid> {
    let pool = ctx.get_store_manager().pool();
    let id = U::update(&pool, data).await?;
    let event = hub_event(entity, "update", id);
    ctx.emit_hub_event(event);
    Ok(id)
}

/// Generic delete helper used internally by the BMC wrappers.
async fn delete<D: Deletable<D>>(ctx: Arc<Ctx>, entity: &'static str, data: D) -> Result<Uuid> {
    let pool = ctx.get_store_manager().pool();
    let id = D::delete(&pool, data).await?;
    let event = hub_event(entity, "delete", id);
    ctx.emit_hub_event(event);
    Ok(id)
}

// endregion:   --- BMC helpers

/// Backend-model-controller for `CategoryGroup` operations.
///
/// Thin wrapper around the generic CRUD helpers that provides typed
/// methods used by IPC handlers.
pub struct GroupBmc;

impl GroupBmc {
    const ENTITY: &'static str = "category_group";

    /// Create a new category group.
    pub async fn create(ctx: Arc<Ctx>, data: C_Group) -> Result<Uuid> {
        create(ctx, Self::ENTITY, data).await
    }

    /// Read a category group by id.
    pub async fn read(ctx: Arc<Ctx>, id: Uuid) -> Result<R_Group> {
        read(ctx, Self::ENTITY, id).await
    }

    /// List category groups matching the provided filter.
    pub async fn list(ctx: Arc<Ctx>, filter: CategoryGroupFilter) -> Result<Vec<R_Group>> {
        list(ctx, Self::ENTITY, filter).await
    }

    /// Update an existing category group.
    pub async fn update(ctx: Arc<Ctx>, data: U_Group) -> Result<Uuid> {
        update(ctx, Self::ENTITY, data).await
    }

    /// Delete (tombstone) a category group.
    pub async fn delete(ctx: Arc<Ctx>, data: D_Group) -> Result<Uuid> {
        delete(ctx, Self::ENTITY, data).await
    }
}

/// Backend-model-controller for `CategoryGroup` operations.
///
/// Thin wrapper around the generic CRUD helpers that provides typed
/// methods used by IPC handlers.
pub struct CategoryBmc;

impl CategoryBmc {
    const ENTITY: &'static str = "category";

    /// Create a new category.
    pub async fn create(ctx: Arc<Ctx>, data: C_Category) -> Result<Uuid> {
        create(ctx, Self::ENTITY, data).await
    }

    /// Read a category by id.
    pub async fn read(ctx: Arc<Ctx>, id: Uuid) -> Result<R_Category> {
        read(ctx, Self::ENTITY, id).await
    }

    /// List categories matching the provided filter.
    pub async fn list(ctx: Arc<Ctx>, filter: CategoryFilter) -> Result<Vec<R_Category>> {
        list(ctx, Self::ENTITY, filter).await
    }

    /// Update an existing category.
    pub async fn update(ctx: Arc<Ctx>, data: U_Category) -> Result<Uuid> {
        update(ctx, Self::ENTITY, data).await
    }

    /// Delete (tombstone) a category.
    pub async fn delete(ctx: Arc<Ctx>, data: D_Category) -> Result<Uuid> {
        delete(ctx, Self::ENTITY, data).await
    }
}

/// Backend-model-controller for `CategoryGroup` operations.
///
/// Thin wrapper around the generic CRUD helpers that provides typed
/// methods used by IPC handlers.
pub struct EntryBmc;

impl EntryBmc {
    const ENTITY: &'static str = "entry";

    /// Create a new entry.
    pub async fn create(ctx: Arc<Ctx>, data: C_Entry) -> Result<Uuid> {
        create(ctx, Self::ENTITY, data).await
    }

    /// Read an entry by id.
    pub async fn read(ctx: Arc<Ctx>, id: Uuid) -> Result<R_Entry> {
        read(ctx, Self::ENTITY, id).await
    }

    /// List entries matching the provided filter.
    pub async fn list(ctx: Arc<Ctx>, filter: EntryFilter) -> Result<Vec<R_Entry>> {
        list(ctx, Self::ENTITY, filter).await
    }

    /// Update an existing entry.
    pub async fn update(ctx: Arc<Ctx>, data: U_Entry) -> Result<Uuid> {
        update(ctx, Self::ENTITY, data).await
    }

    /// Delete (tombstone) an entry.
    pub async fn delete(ctx: Arc<Ctx>, data: D_Entry) -> Result<Uuid> {
        delete(ctx, Self::ENTITY, data).await
    }
}
