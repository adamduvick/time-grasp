use model::model::*;

use serde::Serialize;

use crate::error::Result;
use crate::ipc::support::invoke_typed;

#[derive(Serialize)]
struct CreateCategoryArgs {
    data: C_Category,
}

pub async fn create_category(data: C_Category) -> Result<Uuid> {
    const CMD: &'static str = "create_category";
    let wrapped_args = CreateCategoryArgs { data };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct ReadCategoryArgs {
    id: Uuid,
}

pub async fn read_category(id: Uuid) -> Result<R_Category> {
    const CMD: &'static str = "read_category";
    let wrapped_args = ReadCategoryArgs { id };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct ListCategoryArgs {
    filter: CategoryFilter,
}

pub async fn list_category(filter: CategoryFilter) -> Result<Vec<R_Category>> {
    const CMD: &'static str = "list_category";
    let wrapped_args = ListCategoryArgs { filter };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct UpdateCategoryArgs {
    data: U_Category,
}

pub async fn update_category(data: U_Category) -> Result<Uuid> {
    const CMD: &'static str = "update_category";
    let wrapped_args = UpdateCategoryArgs { data };
    invoke_typed(CMD, wrapped_args).await
}

#[derive(Serialize)]
struct DeleteCategoryArgs {
    data: D_Category,
}

pub async fn delete_category(data: D_Category) -> Result<Uuid> {
    const CMD: &'static str = "delete_category";
    let wrapped_args = DeleteCategoryArgs { data };
    invoke_typed(CMD, wrapped_args).await
}
