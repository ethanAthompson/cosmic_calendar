use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_router::*;
use web_sys::{Event, InputEvent, KeyboardEvent, MouseEvent};

#[component]
pub fn EnterDateBar() -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXRegular);
    let year_input = create_rw_signal("".to_string());
    let year_input_el: NodeRef<Input> = create_node_ref();

    let month_input = create_rw_signal("".to_string());
    let month_input_el: NodeRef<Input> = create_node_ref();

    let day_input = create_rw_signal("".to_string());
    let day_input_el: NodeRef<Input> = create_node_ref();

    let update_year = move |ev: Event| {
        ev.prevent_default();
        let value = year_input_el.get().expect("<input> exists").value();
        console_log(&value);
    };

    let update_month = move |ev: Event| {
        ev.prevent_default();
    };

    let update_day = move |ev: Event| {
        ev.prevent_default();
    };

    let on_remove_year = move |ev: MouseEvent| {
        year_input.set("".to_string());
        ev.prevent_default();
    };

    let on_remove_month = move |ev: MouseEvent| {
        month_input.set("".to_string());
        ev.prevent_default();
    };

    let on_remove_day = move |ev: MouseEvent| {
        day_input.set("".to_string());
        ev.prevent_default();
    };

    let only_num = move |ev: KeyboardEvent| {
        let key = ev.key_code();

        match key {
            // [A-Z]
            65..=90 => {
                ev.prevent_default();
            }
            // Backspace
            8 => {}
            // Tab
            9 => {}
            // Enter
            13 => {}
            _ => {}
        }
    };

    let (year, set_year) = create_query_signal::<String>("year");
    let (month, set_month) = create_query_signal::<String>("month");
    let (day, set_day) = create_query_signal::<String>("day");
    let (calendar, set_calendar) = create_query_signal::<String>("calendar");

    view! {
         <div class="space-y-2">
            <div class="dark:bg-slate-900 bg-slate-200 rounded-xl py-2 ">
                <section class="relative py-2">
                    <input on:keydown=only_num node_ref=year_input_el on:change=update_year
                            placeholder="Year" type="text" class="p-2 italic w-full dark:bg-slate-900 bg-slate-200
                                desktop:text-2xl laptop:text-2xl tablet:text-2xl text-xl
        "
                            maxlength="4" required="true" name="year" value=year
                            oninput="this.form.requestSubmit()"
                    />
                    <button class="absolute inset-y-0 z-20 end-0 cursor-grab hover:text-red-400 dark:text-white text-black
                        p-2 hover:bg-blend-lighten mix-blend-screen text-start glitch
    
                    " on:click=on_remove_year>
                         <Icon icon=close_icon class="w-9 h-9"/>
                    </button>
                </section>
                <section class="relative py-2">
                    <input on:keydown=only_num node_ref=month_input_el on:change=update_month
                            placeholder="Month" type="text" class="p-2 italic w-full dark:bg-slate-900 bg-slate-200
        desktop:text-2xl laptop:text-2xl tablet:text-2xl text-xl
        "
                            maxlength="2" required="true" name="month" value=month
                            oninput="this.form.requestSubmit()"
                    />
                    <button class="absolute inset-y-0 z-20 end-0 cursor-grab hover:text-red-400 dark:text-white text-black
                        p-2 hover:bg-blend-lighten mix-blend-screen text-start glitch
    
                    " on:click=on_remove_month>
                         <Icon icon=close_icon class="w-9 h-9"/>
                    </button>
                </section>
                <section class="relative py-2">
                    <input on:keydown=only_num node_ref=day_input_el on:change=update_day
                            placeholder="Day" type="text" class="p-2 italic w-full dark:bg-slate-900 bg-slate-200
                desktop:text-2xl laptop:text-2xl tablet:text-2xl text-xl
        "
                            maxlength="2" required="true" name="day" value=day
                            oninput="this.form.requestSubmit()"
                    />

                    <button class="absolute inset-y-0 z-20 end-0 cursor-grab hover:text-red-400 dark:text-white text-black
                        p-2 hover:bg-blend-lighten mix-blend-screen text-start glitch

                    " on:click=on_remove_day>
                         <Icon icon=close_icon class="w-9 h-9"/>
                    </button>
                </section>
            </div>
            <section>
                <input type="submit" required="true" class="
            p-2 hover:bg-blend-lighten mix-blend-screen w-full text-start cursor-pointer
            -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 
            ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-2xl text-xl
                "/>
            </section>
       </div>
    }
}

#[component]
pub fn SelectBar() -> impl IntoView {
    view! {
        <div class="flex items-center space-x-2 p-4 dark:bg-slate-900 bg-slate-200 rounded-xl
            hover:shadow-amber cursor-pointer
        ">
            <DateDisplay />
        </div>
    }
}

#[component]
pub fn DateDisplay() -> impl IntoView {
    let (year, set_year) = create_query_signal::<String>("year");
    let (month, set_month) = create_query_signal::<String>("month");
    let (day, set_day) = create_query_signal::<String>("day");
    let (calendar, set_calendar) = create_query_signal::<String>("calendar");
    
    view! {
        <Show
            when=move || 
                // Shows when its a real month, year, day
                // INFO! default calendar is gregorian
                year.get().unwrap_or("".to_owned()).is_empty() == false &&
                ((year.get().unwrap_or("".to_owned()).len() > 2)) &&
                month.get().unwrap_or("".to_owned()).is_empty() == false &&
                ((month.get().unwrap_or("".to_owned()).parse::<usize>().unwrap() < 13)) && 
                day.get().unwrap_or("".to_owned()).is_empty() == false &&
                ((day.get().unwrap_or("".to_owned()).parse::<usize>().unwrap() < 32))

            fallback=move || view! {<span>"waiting for date..."</span>}
        >
        <section>
            <p class="font-bold desktop:text-2xl laptop:text-2xl tablet:text-2xl text-xl">
                {year}/{month}/{day}
            </p>
        </section>
           <select id="calendars" class="px-2 dark:bg-slate-900 bg-slate-200 cursor-pointer
            hover:bg-blend-lighten mix-blend-screen w-full text-start
            -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 
            ease-in-out duration-300 glitch 
        desktop:text-2xl laptop:text-2xl tablet:text-2xl text-xl
        
        "
            oninput="this.form.requestSubmit()" required="true" name="calendar"
        >
                 <option class="" value="gregorian">Gregorian</option>
                 <option class="" value="indian">Indian</option>
                 <option class="" value="islamic">Islamic</option>
                 <option class="" value="chinese">Chinese</option>
           </select>
        </Show>
    }
}

#[component]
pub fn WaitingButton() -> impl IntoView {
    view! {
        ""
    }
}
