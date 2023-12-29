use leptos::*;
use crate::components::links::largescreen::SuperBigLinks;

#[component]
pub fn LargeScreenLinks() -> impl IntoView {
    let with_switch = create_rw_signal(false);
    
    view! {
        <section class="shadow-lg dark:shadow-amber hidden watch:hidden tablet:grid phone:hidden laptop:grid desktop:grid grid-cols-2 items-center py-2">
            <nav class="order-2 flex justify-end px-2">
                <div class="flex justify-end space-x-12 items-center px-4 text-2xl">
                    <SuperBigLinks with_switch/>
                </div>
            </nav>

            <nav class="order-1 flex justify-start px-2 text-black dark:text-white ">
                "© 2023 Zone™. All Rights Reserved."
            </nav>
        </section>
    }
}


