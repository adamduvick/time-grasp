use crate::error::Result;
use model::*;

use wasm_bindgen::prelude::*;

// region:          IPC helpers

#[wasm_bindgen]
extern "C" {
    /// This is needed to invoke a tauri command from the frontend
    ///
    /// Everything must be serializable so as to pass through the IPC
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> std::result::Result<JsValue, JsValue>;
}

/// Since `invoke` only accepts and returns `JsValue` types, this generic wrapper is
/// needed to define type-safe calls in the front end. Not necessary, but it's nicer
/// to work with the real types from the `model` library and it saves a lot of boilerplate
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

/// This macro assumes:
///
/// 1. the signature of the function perfectly mirrors the signature of the `tauri::command`
///    in the backend (except the state automatically injected as an argument)
/// 2. the name of the function is identical to the name of the corresponding `tauri::commmand`
///
/// If these assumptions hold true, this macro helps to easily create a mirroring function
#[macro_export]
macro_rules! ipc_call {
    ($(#[$meta:meta])* $name:ident ( $( $arg:ident : $ty:ty ),* $(,)? ) -> $ret:ty ) => {
        $(#[$meta])*
        #[doc = concat!("Type-safe mirror to `backend::ipc::", stringify!($name), "`")]
        pub async fn $name( $( $arg : $ty ),* ) -> $ret {
            const CMD: &'static str = stringify!($name);

            #[derive(::serde::Serialize, Debug)]
            struct Args {
                $( $arg : $ty ),*
            }

            let wrapped_args = Args { $( $arg ),* };
            ::leptos::logging::debug_log!("frontend::ipc::{} invoked with args:\n{wrapped_args:#?}", stringify!($name));
            invoke_typed(CMD, wrapped_args).await
        }
    };
}

// endregion:       IPC helpers

ipc_call!(create_group(data: C_Group) -> Result<Uuid>);
ipc_call!(read_group(id: Uuid) -> Result<R_Group>);
ipc_call!(list_group(filter: CategoryGroupFilter) -> Result<Vec<R_Group>>);
ipc_call!(update_group(data: U_Group) -> Result<Uuid>);
ipc_call!(delete_group(data: D_Group) -> Result<Uuid>);

ipc_call!(create_category(data: C_Category) -> Result<Uuid>);
ipc_call!(read_category(id: Uuid) -> Result<R_Category>);
ipc_call!(list_category(filter: CategoryFilter) -> Result<Vec<R_Category>>);
ipc_call!(update_category(data: U_Category) -> Result<Uuid>);
ipc_call!(delete_category(data: D_Category) -> Result<Uuid>);

ipc_call!(create_entry(data: C_Entry) -> Result<Uuid>);
ipc_call!(read_entry(id: Uuid) -> Result<R_Entry>);
ipc_call!(list_entry(filter: EntryFilter) -> Result<Vec<R_Entry>>);
ipc_call!(update_entry(data: U_Entry) -> Result<Uuid>);
ipc_call!(delete_entry(data: D_Entry) -> Result<Uuid>);
