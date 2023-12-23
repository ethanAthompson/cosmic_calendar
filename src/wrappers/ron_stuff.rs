use gloo_net::http::Request;
use leptos::leptos_dom::logging::console_log;
use ron::{
    de::{self, from_reader},
    from_str,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

/// Ron Data Generic Marker
///
/// * Allows the user to decode a ron file whilst return the actual data
///
///
pub struct RonData<T>(T);

impl<T: DeserializeOwned + Debug> RonData<T> {
    pub async fn new(file: &'static str) -> T {
        let pretty = format!("/data/{}", file);

        let req = Request::get(pretty.as_str())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // displays the file requested
        // console_log(req.as_str());

        let data: T = ron::from_str(req.as_str()).unwrap();

        data
    }
}
