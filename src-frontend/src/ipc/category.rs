use model::*;

use serde::Serialize;

use crate::error::Result;
use crate::ipc::support::invoke_typed;

pub async fn create_category(data: C_Category) -> Result<Uuid> {
    const CMD: &'static str = "create_category";

    #[derive(Serialize)]
    struct Args {
        data: C_Category,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn read_category(id: Uuid) -> Result<R_Category> {
    const CMD: &'static str = "read_category";

    #[derive(Serialize)]
    struct Args {
        id: Uuid,
    }

    let wrapped_args = Args { id };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn list_category(filter: CategoryFilter) -> Result<Vec<R_Category>> {
    const CMD: &'static str = "list_category";

    #[derive(Serialize)]
    struct Args {
        filter: CategoryFilter,
    }

    let wrapped_args = Args { filter };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn update_category(data: U_Category) -> Result<Uuid> {
    const CMD: &'static str = "update_category";

    #[derive(Serialize)]
    struct Args {
        data: U_Category,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn delete_category(data: D_Category) -> Result<Uuid> {
    const CMD: &'static str = "delete_category";

    #[derive(Serialize)]
    struct Args {
        data: D_Category,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}
