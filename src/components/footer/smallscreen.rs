use leptos::*;
use crate::components::links::largescreen::SuperBigLinks;

#[component]
pub fn SmallScreenLinks() -> impl IntoView {
    let with_switch = create_rw_signal(false);
    
    view! {
        <section class="bg-blue-300 desktop:hidden laptop:hidden grid tablet:grid phone:grid watch:grid grid-cols-2 items-center shadow-inner shadow-2xl">
            <nav class="order-2 flex justify-end px-2">
                <div class="flex justify-end space-x-2 tablet:space-x-12 watch:space-x-4 phone:space-x-8 items-center px-4 text-xl">
                    <SuperBigLinks with_switch/>
                </div>
            </nav>

            <nav class="order-1 flex justify-start px-2 text-black dark:text-white phone:text-sm text-xs font-light">
                "© 2023 Zone™. All Rights Reserved."
            </nav>
        </section>
    }
}
