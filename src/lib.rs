// Welcome to lib.rs
//
// Lib.rs contains all the tauri interfaces
//
//

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Greet functions Arguments from src-tauri/src/main.rs
#[derive(Serialize, Deserialize)]
pub struct GreetArgs<'a> {
    pub name: &'a str,
}
