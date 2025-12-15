use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;

use crate::error::Result;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> std::result::Result<JsValue, JsValue>;
}

pub(in crate::ipc) async fn invoke_typed<Args, Output>(cmd: &str, args: Args) -> Result<Output>
where
    Args: Serialize,
    Output: DeserializeOwned,
{
    let js_args = serde_wasm_bindgen::to_value(&args)?;

    let js_resp = invoke(cmd, js_args).await?;

    let rust_resp = serde_wasm_bindgen::from_value(js_resp)?;

    Ok(rust_resp)
}
