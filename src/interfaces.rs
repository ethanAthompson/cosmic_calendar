//! I think the app should be structured like this
/*
    interfaces/
        alloys/ # contain tauri interfaces
            ...
        composites/ # contain basic wrappers
            ...
        polymers/ # contain large data; ctz_traits, etc...
            ...
        ceramics/ # contain type simplifiers, and abstractors
            ...
        microscopy/ # contain tests for alloys, composites, polymers, ceramics
            ...
*/

// //! Small Functions that wraps a certain functionality
// //! that would be redundant to make in production code
// //!


// /// web operations
// pub mod web;

// /// theme operations
// pub mod theme;

// /// date operations
// pub mod date;

// /// string operations
// pub mod strings;

// use wasm_bindgen::prelude::*;

// /// Structures relating to a user
// pub mod user;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
//     pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }


// //! function Arguments from src-tauri/src/main.rs
// //!
// use serde::{Deserialize, Serialize};

// /// Greet Arguments with
// #[derive(Serialize, Deserialize)]
// pub struct GreetArgs<'a> {
//     /// The name of the person being greeted
//     pub name: &'a str,
// }


pub mod ceramics;
pub mod alloys;
pub mod polymers;
pub mod composites;
pub mod microscopy;
