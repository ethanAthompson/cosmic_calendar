use leptos::*;
use leptos_use::use_media_query;

#[component]
pub fn Card() -> impl IntoView {
    let desktop = use_media_query("(min-width: 1280px)");
    let laptop = use_media_query("(min-width: 1024px)");
    let tablet = use_media_query("(min-width: 640px)");
    let phone = use_media_query("(min-width: 480px)");
    let watch = use_media_query("(min-width: 320px)");
    let screens = vec![
        ("Desktop", desktop),
        ("Laptop", laptop),
        ("Tablet", tablet),
        ("Phone", phone),
        ("Watch", watch),
    ];

    let screen_display = screens
        .into_iter()
        .map(|(name, query)| {

            view! {
                <p class=move || if query.get() {"text-green-400"} else {"text-red-400"}>{name}: {query}</p>
            }
        })
        .collect_view();

    view! {
        <div class="p-2 flex justify-evenly"> 
            <div class="debug">{screen_display}</div>
        </div>         
    }
}
