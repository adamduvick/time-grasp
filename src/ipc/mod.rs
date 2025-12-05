mod category;
mod category_group;
mod entry;
mod support;

pub use category::*;
pub use category_group::*;
pub use entry::*;
pub use model::model::*;

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
