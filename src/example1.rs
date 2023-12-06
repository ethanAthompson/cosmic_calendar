
@layer base {

  .debug {
    @apply desktop:bg-blue-800 laptop:bg-red-800 tablet:bg-black phone:bg-green-800 watch:bg-yellow-800 w-20 h-2;
  }

  /* Light Mode */
  body {
    min-height: 100vh;
    background-color: var(#000000);
    background-image:
      radial-gradient(at 2% 5%, var(#4d1d8b) 0, transparent 68%),
      radial-gradient(at 15% 10%, var(#e7e5e4) 0, transparent 38%),
      radial-gradient(at 62% 99%, var(#fdbb74) 0, transparent 59%),
      radial-gradient(at 73% 92%, var(#fecdd3) 0, transparent 99%),
      radial-gradient(at 86% 13%, var(#64748b) 0, transparent 59%),
      radial-gradient(at 85% 58%, var(#404040) 0, transparent 38%);
  }

  /* Dark Mode change transparency? */
  body,
  .dark {
    min-height: 100vh;
    background-color: var(#000000);
    background-image:
      radial-gradient(at 2% 5%, var(#13917c) 0, transparent 68%),
      radial-gradient(at 15% 10%, var(#260f4a) 0, transparent 38%),
      radial-gradient(at 62% 99%, var(#79716c) 0, transparent 59%),
      radial-gradient(at 73% 92%, var(#b65f03) 0, transparent 99%),
      radial-gradient(at 86% 13%, var(#323a45) 0, transparent 59%),
      radial-gradient(at 85% 58%, var(#202020) 0, transparent 38%);
  }

  /* System Mode: change transparency? */
  @media (prefers-color-scheme:dark) {
    body {
      min-height: 100vh;
      background-color: var(#000000);
      background-image:
        radial-gradient(at 2% 5%, var(#13917c) 0, transparent 68%),
        radial-gradient(at 15% 10%, var(#260f4a) 0, transparent 38%),
        radial-gradient(at 62% 99%, var(#79716c) 0, transparent 59%),
        radial-gradient(at 73% 92%, var(#b65f03) 0, transparent 99%),
        radial-gradient(at 86% 13%, var(#323a45) 0, transparent 59%),
        radial-gradient(at 85% 58%, var(#202020) 0, transparent 38%);
    }
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }
}use charming::{
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
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

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
        <main class="p-4 flex dark:bg-gray-600 dark:hover:bg-gray-800">
            <button class="block p-4 text-white hover:text-gray" on:click=move |_| {action.dispatch(());}> "Show Chart" </button>
            <button class="block p-4 text-white hover:text-gray" on:click=move |_| {teardown_action();}> "Hide Chart" </button>
        </main>
        <main class="p-4 grid grid-cols-2  w-full dark:bg-slate-400 h-full">
            <div id="utc-chart"/>
        </main>
        <main class="flex justify-center text-center items-center bg-slate-200 p-4">
            <form class="flex flex-col p-4" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    class="p-2 border focus:border-blue-200"
                    on:input=update_name
                />
                <button type="submit" class="bg-blue-200 p-4 block">"Greet"</button>
            </form>

            <p class="text-center text-black"><b>{ move || greet_msg.get() }</b></p>
        </main>
    }


    let (theme, set_theme) =
        create_signal(false || storage.get_item("theme").unwrap().unwrap() == "dark");

    let adjust_theme = move |_| set_theme.update(move |theme| *theme = !*theme);

    let icon = Signal::derive(move || {
        if theme.get() {
            console_log(theme.get().to_string().as_str());
            storage.set_item("theme", "light").unwrap();
            Icon::from(BsIcon::BsSun)
        } else {
            console_log(theme.get().to_string().as_str());
            storage.set_item("theme", "dark").unwrap();
            Icon::from(BsIcon::BsMoonStars)
        }
    });

    view! {
        <span on:click=adjust_theme>
            <Icon icon class="cursor-pointer"/>
        </span>
    }
