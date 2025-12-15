use model::*;

use serde::Serialize;

use crate::error::Result;
use crate::ipc::support::invoke_typed;

pub async fn create_entry(data: C_Entry) -> Result<Uuid> {
    const CMD: &'static str = "create_entry";

    #[derive(Serialize)]
    struct Args {
        data: C_Entry,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn read_entry(id: Uuid) -> Result<R_Entry> {
    const CMD: &'static str = "read_entry";

    #[derive(Serialize)]
    struct Args {
        id: Uuid,
    }

    let wrapped_args = Args { id };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn list_entry(filter: EntryFilter) -> Result<Vec<R_Entry>> {
    const CMD: &'static str = "list_entry";

    #[derive(Serialize)]
    struct Args {
        filter: EntryFilter,
    }

    let wrapped_args = Args { filter };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn update_entry(data: U_Entry) -> Result<Uuid> {
    const CMD: &'static str = "update_entry";

    #[derive(Serialize)]
    struct Args {
        data: U_Entry,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}

pub async fn delete_entry(data: D_Entry) -> Result<Uuid> {
    const CMD: &'static str = "delete_entry";

    #[derive(Serialize)]
    struct Args {
        data: D_Entry,
    }

    let wrapped_args = Args { data };
    invoke_typed(CMD, wrapped_args).await
}
