use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

pub mod system;

// Welcome to interfaces.rs
//
// interfaces.rs contains the main invoker for tauri
//
// modules connected here are for exchanging data between rust -> js <- rust
//
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
