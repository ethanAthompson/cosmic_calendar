use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use web_sys::Storage;

// --st 1(violet-900) 2(stone-200) 3(orange-300) 4(rose-200) 5(slate-500) 6(neutral-700)
#[component]
pub fn App() -> impl IntoView {
    view! {
        <header class="shadow-lg p-4 grid grid-cols-2 w-full">
            <nav class="flex justify-start items-center">
                <div class="p-4">
                    <h1 class="text-4xl"> Cosmic Calendar </h1>
                </div>
            </nav>
            <nav class="flex justify-end px-4">
                <div class="flex-nowrap flex space-x-12 items-center justify-end text-xl ">
                    <p class="py-4 rounded-md transition text-2xl hover:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> Tool </p>
                    <p class="py-4 rounded-md transition text-2xl hover:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> About </p>
                    <p class="py-4 rounded-md transition text-2xl hover:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> Install </p>
                    <p class="py-4 rounded-md transition text-2xl focus:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> <ThemeSwitch/></p>
                </div>
            </nav>
        </header>
    }
}

pub enum ZoneTheme {
    Light,
    Dark,
    System,
}

impl ZoneTheme {
    pub fn set_theme(theme: ZoneTheme) {
        match theme {
            ZoneTheme::Light => {
                storage().set_item("theme", "light").unwrap();
                update_dom_el("html-theme", "light");
            }
            ZoneTheme::Dark => {
                storage().set_item("theme", "dark").unwrap();
                update_dom_el("html-theme", "dark");
            }
            ZoneTheme::System => {
                storage().remove_item("theme").unwrap();
                update_dom_el("html-theme", "system");
            }
        }
    }
}

pub fn storage() -> Storage {
    window().local_storage().unwrap().unwrap()
}

pub fn update_dom_el(id: &'static str, class: &'static str) {
    document()
        .get_element_by_id(id)
        .unwrap()
        .set_class_name(class);
}

#[component]
pub fn ThemeSwitch() -> impl IntoView {
    let sun = Icon::from(BsIcon::BsSun);
    let moon = Icon::from(BsIcon::BsMoonStars);
    let system = Icon::from(FiIcon::FiMonitor);

    let (panel, set_panel) = create_signal(false);
    let (class, set_class) = create_signal("hidden");

    let update_panel = move || {
        set_panel.update(|toggle| {
            if *toggle {
                set_class.set("hidden");
                update_dom_el("themes", class.get());
            } else {
                set_class.set("");
                update_dom_el("themes", class.get());
            }

            *toggle = !*toggle
        })
    };

    // Button toggles panel by default
    let toggle_panel = move |_| {
        update_panel();
    };

    // each dropdown-item -> dark, light, system (adjacent to css classes)
    let light_theme = move |_| {
        ZoneTheme::set_theme(ZoneTheme::Light);
    };

    let dark_theme = move |_| {
        ZoneTheme::set_theme(ZoneTheme::Dark);
    };

    let system_theme = move |_| {
        ZoneTheme::set_theme(ZoneTheme::System);
    };

    view! {
        <main class="relative inline-block text-left">
            <button on:click=toggle_panel type="button" class="inline-flex justify-center items-center px-2 py-2">
                <Icon icon=sun class=""/>
            </button>
            <article id="themes" class="hidden">
                <article class="absolute right-0 z-25 mt-8 p-2 w-fit rounded-lg shadow-lg flex flex-col flex-nowrap origin-top-left bg-slate-800 ">
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-orange-700 cursor-pointer" on:click=light_theme>
                         <Icon icon=sun class="cursor-pointer"/>
                        Light
                    </span>
                    <hr class="h-px w-full bg-gray"/>
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-violet-700 cursor-pointer" on:click=dark_theme>
                         <Icon icon=moon class="cursor-pointer"/>
                        Dark
                    </span>
                    <hr class="h-px w-full bg-gray"/>
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-gray-700 cursor-pointer" on:click=system_theme>
                         <Icon icon=system class="cursor-pointer"/>
                        System
                    </span>
                </article>
            </article>
        </main>
    }
}
