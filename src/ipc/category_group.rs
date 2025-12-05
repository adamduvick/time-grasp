use model::model::*;

use serde::Serialize;

use crate::error::Result;
use crate::ipc::support::invoke_typed;

#[derive(Serialize)]
struct CreateGroupArgs {
    data: C_Group,
}

pub async fn create_group(data: C_Group) -> Result<Uuid> {
    const CMD: &'static str = "create_group";
    let wrapped_args = CreateGroupArgs { data };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct ReadGroupArgs {
    id: Uuid,
}

pub async fn read_group(id: Uuid) -> Result<R_Group> {
    const CMD: &'static str = "read_group";
    let wrapped_args = ReadGroupArgs { id };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct ListGroupArgs {
    filter: CategoryGroupFilter,
}

pub async fn list_group(filter: CategoryGroupFilter) -> Result<Vec<R_Group>> {
    const CMD: &'static str = "list_group";
    let wrapped_args = ListGroupArgs { filter };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct UpdateGroupArgs {
    data: U_Group,
}

pub async fn update_group(data: U_Group) -> Result<Uuid> {
    const CMD: &'static str = "update_group";
    let wrapped_args = UpdateGroupArgs { data };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct DeleteGroupArgs {
    data: D_Group,
}

pub async fn delete_group(data: D_Group) -> Result<Uuid> {
    const CMD: &'static str = "delete_group";
    let wrapped_args = DeleteGroupArgs { data };
    invoke_typed(CMD, wrapped_args).await
}
