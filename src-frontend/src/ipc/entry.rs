use model::*;

use serde::Serialize;

use crate::error::Result;
use crate::ipc::support::invoke_typed;

#[derive(Serialize)]
struct CreateEntryArgs {
    data: C_Entry,
}

pub async fn create_entry(data: C_Entry) -> Result<Uuid> {
    const CMD: &'static str = "create_entry";
    let wrapped_args = CreateEntryArgs { data };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct ReadEntryArgs {
    id: Uuid,
}

pub async fn read_entry(id: Uuid) -> Result<R_Entry> {
    const CMD: &'static str = "read_entry";
    let wrapped_args = ReadEntryArgs { id };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct ListEntryArgs {
    filter: EntryFilter,
}

pub async fn list_entry(filter: EntryFilter) -> Result<Vec<R_Entry>> {
    const CMD: &'static str = "list_entry";
    let wrapped_args = ListEntryArgs { filter };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct UpdateEntryArgs {
    data: U_Entry,
}

pub async fn update_entry(data: U_Entry) -> Result<Uuid> {
    const CMD: &'static str = "update_entry";
    let wrapped_args = UpdateEntryArgs { data };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct DeleteEntryArgs {
    data: D_Entry,
}

pub async fn delete_entry(data: D_Entry) -> Result<Uuid> {
    const CMD: &'static str = "delete_entry";
    let wrapped_args = DeleteEntryArgs { data };
    invoke_typed(CMD, wrapped_args).await
}
