use crate::error::Result;
use model::*;

use wasm_bindgen::prelude::*;

// region:          IPC helpers

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> std::result::Result<JsValue, JsValue>;
}

async fn invoke_typed<Args, Output>(cmd: &str, args: Args) -> Result<Output>
where
    Args: serde::Serialize,
    Output: serde::de::DeserializeOwned,
{
    let js_args = serde_wasm_bindgen::to_value(&args)?;

    let js_resp = invoke(cmd, js_args).await?;

    let rust_resp = serde_wasm_bindgen::from_value(js_resp)?;

    Ok(rust_resp)
}

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
            invoke_typed(CMD, wrapped_args).await
        }
    };
}

// endregion:       IPC helpers

ipc_call!(pub async fn create_group(data: C_Group) -> Result<Uuid>);
ipc_call!(pub async fn read_group(id: Uuid) -> Result<R_Group>);
ipc_call!(pub async fn list_group(filter: CategoryGroupFilter) -> Result<Vec<R_Group>>);
ipc_call!(pub async fn update_group(data: U_Group) -> Result<Uuid>);
ipc_call!(pub async fn delete_group(data: D_Group) -> Result<Uuid>);

ipc_call!(pub async fn create_category(data: C_Category) -> Result<Uuid>);
ipc_call!(pub async fn read_category(id: Uuid) -> Result<R_Category>);
ipc_call!(pub async fn list_category(filter: CategoryFilter) -> Result<Vec<R_Category>>);
ipc_call!(pub async fn update_category(data: U_Category) -> Result<Uuid>);
ipc_call!(pub async fn delete_category(data: D_Category) -> Result<Uuid>);

ipc_call!(pub async fn create_entry(data: C_Entry) -> Result<Uuid>);
ipc_call!(pub async fn read_entry(id: Uuid) -> Result<R_Entry>);
ipc_call!(pub async fn list_entry(filter: EntryFilter) -> Result<Vec<R_Entry>>);
ipc_call!(pub async fn update_entry(data: U_Entry) -> Result<Uuid>);
ipc_call!(pub async fn delete_entry(data: D_Entry) -> Result<Uuid>);
