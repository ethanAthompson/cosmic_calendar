use charming::{
    component::{Axis, Title},
    element::AxisType,
    series::Line,
    Chart, WasmRenderer,
};
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    // let (name, set_name) = create_signal(String::new());
    // let (greet_msg, set_greet_msg) = create_signal(String::new());

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         if name.get().is_empty() {
    //             return;
    //         }

    //         let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    let teardown_action = || {
        document()
            .get_element_by_id("utc-chart")
            .unwrap()
            .set_class_name("hidden")
    };

    let action = create_action(|input: &()| async {
        let chart = Chart::new()
            .title(Title::new().text("Time Differences"))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Utc +2:00", "Utc -2:00"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![1, 2]));

        let renderer = WasmRenderer::new(800, 800);
        renderer.render("utc-chart", &chart).unwrap();
        document()
            .get_element_by_id("utc-chart")
            .unwrap()
            .set_class_name("show")
    });

    view! {

        <main class="bg-blue-300 flex p-4 dark:bg-blue-500">
            <button class="bg-green-400 shadow-lg dark:bg-green-500 p-4" on:click=move |_| {action.dispatch(());}> "Show Chart" </button>
            <button class="bg-red-400 shadow-lg dark:bg-red-500 p-4" on:click=move |_| {teardown_action();}> "Hide Chart" </button>
            <div id="utc-chart"/>
        </main>
        // <main class="container">
        //     <h1>
        //     "hi"
        // </h1>
        //     <form class="row" on:submit=greet>
        //         <input
        //             id="greet-input"
        //             placeholder="Enter a name..."
        //             on:input=update_name
        //         />
        //         <button type="submit">"Greet"</button>
        //     </form>

        //     <p><b>{ move || greet_msg.get() }</b></p>
        // </main>
    }
}
