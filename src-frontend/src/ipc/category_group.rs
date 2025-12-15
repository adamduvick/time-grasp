use model::*;

use serde::Serialize;

use crate::error::Result;
use crate::ipc::support::invoke_typed;

pub async fn create_group(data: C_Group) -> Result<Uuid> {
    const CMD: &'static str = "create_group";

    #[derive(Serialize)]
    struct Args {
        data: C_Group,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn read_group(id: Uuid) -> Result<R_Group> {
    const CMD: &'static str = "read_group";

    #[derive(Serialize)]
    struct Args {
        id: Uuid,
    }

    let wrapped_args = Args { id };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn list_group(filter: CategoryGroupFilter) -> Result<Vec<R_Group>> {
    const CMD: &'static str = "list_group";

    #[derive(Serialize)]
    struct Args {
        filter: CategoryGroupFilter,
    }

    let wrapped_args = Args { filter };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn update_group(data: U_Group) -> Result<Uuid> {
    const CMD: &'static str = "update_group";

    #[derive(Serialize)]
    struct Args {
        data: U_Group,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn delete_group(data: D_Group) -> Result<Uuid> {
    const CMD: &'static str = "delete_group";

    #[derive(Serialize)]
    struct Args {
        data: D_Group,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}
