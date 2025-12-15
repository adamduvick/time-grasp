mod category;
// mod category_group;
mod entry;
mod support;

pub use category::*;
// pub use category_group::*;
pub use entry::*;
pub use model::*;

use crate::error::Result;

/// If I have a ton of IPC functions, this macro could help, but it's best to
/// not do much with it at this point until I prove I need it. There may be
/// reasons for having other types of IPC calls that do not mirror so closely the
/// signatures that exist in the backend.
///
/// This proof of concept will remain in case it proves useful in the future.
#[macro_export]
macro_rules! ipc_call {
    (pub async fn $name:ident ( $( $arg:ident : $ty:ty ),* $(,)? ) -> $ret:ty ) => {
        pub async fn $name( $( $arg : $ty ),* ) -> $ret {
            const CMD: &'static str = stringify!($name);

            #[derive(::serde::Serialize)]
            struct Args {
                $( $arg : $ty ),*
            }

            let wrapped_args = Args { $( $arg ),* };
            crate::ipc::support::invoke_typed(CMD, wrapped_args).await
        }
    };
}

ipc_call!(pub async fn create_group(data: C_Group) -> Result<Uuid>);
ipc_call!(pub async fn read_group(id: Uuid) -> Result<R_Group>);
ipc_call!(pub async fn list_group(filter: CategoryGroupFilter) -> Result<Vec<R_Group>>);
ipc_call!(pub async fn update_group(data: U_Group) -> Result<Uuid>);
ipc_call!(pub async fn delete_group(data: D_Group) -> Result<Uuid>);

// TODO remove once tested
#[derive(serde::Serialize)]
struct Args<'a> {
    name: &'a str,
}

pub async fn create_and_read_back_group(name: &str) -> crate::error::Result<String> {
    const CMD: &'static str = "create_and_read_back_group";
    let wrapped_args = Args { name };
    support::invoke_typed(CMD, wrapped_args).await
}

pub async fn create_group_by_name(name: &str) -> crate::error::Result<Uuid> {
    let data = C_Group {
        id: Uuid::new_v4(),
        name: name.to_string(),
        note: None,
    };
    create_group(data).await
}

pub async fn list_all_groups() -> crate::error::Result<Vec<R_Group>> {
    let filter = CategoryGroupFilter { id: None };
    list_group(filter).await
}
