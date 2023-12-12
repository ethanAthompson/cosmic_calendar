use serde::{Deserialize, Serialize};

/// Greet functions Arguments from src-tauri/src/main.rs
#[derive(Serialize, Deserialize)]
pub struct GreetArgs<'a> {
    pub name: &'a str,
}
