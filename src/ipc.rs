use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Debug)]
pub enum InvokeError {
    Js(JsValue),
    Serde(String),
}

pub async fn invoke_typed<Args, Output>(cmd: &str, args: Args) -> Result<Output, InvokeError>
where
    Args: Serialize,
    Output: DeserializeOwned,
{
    let js_args =
        serde_wasm_bindgen::to_value(&args).map_err(|e| InvokeError::Serde(e.to_string()))?;

    let js_resp = invoke(cmd, js_args).await.map_err(InvokeError::Js)?;

    serde_wasm_bindgen::from_value(js_resp).map_err(|e| InvokeError::Serde(e.to_string()))
}

// TODO import model types and create all typed commands in frontend
//
// example:
// ```
// pub async fn create_group(args: C_Group) -> Result<Uuid, InvokeError> {
//     const CMD: &'static str = "create_group";
//     invoke_typed(CMD, args).await?
// }
// ```

// #[async_trait(?Send)]
// pub trait Command {
//     type Args: Serialize;
//     type Output: DeserializeOwned;

//     const COMMAND: &'static str;

//     async fn invoke(args: &Self::Args) -> Result<Self::Output, InvokeError> {
//         let js_args =
//             serde_wasm_bindgen::to_value(args).map_err(|e| InvokeError::Serde(e.to_string()))?;

//         let js_resp = invoke(Self::COMMAND, js_args)
//             .await
//             .map_err(|e| InvokeError::Js(e))?;

//         let rust_resp: Self::Output = serde_wasm_bindgen::from_value(js_resp)
//             .map_err(|e| InvokeError::Serde(e.to_string()))?;

//         Ok(rust_resp)
//     }
// }
