use leptos::{
    html::{Input, Select},
    leptos_dom::logging::console_log,
    *,
};
use leptos_icons::*;
use leptos_router::*;
use web_sys::Event;

#[component]
pub fn Card() -> impl IntoView {
    view! {}
}

#[component]
pub fn SelectBar<I>(#[prop(into)] reactive_node: NodeRef<Select>, on_input: I) -> impl IntoView
where
    I: FnMut(Event) + 'static,
{
    view! {
        <select node_ref=reactive_node on:input=on_input id="earth-cal" class="p-2 dark:bg-slate-900 bg-slate-200 cursor-pointer roudned-xl dark:text-white text-black">
             <optgroup label="Islamic">
                <option class="supported_calendar" value="IslamicCivil">Islamic Civil</option>
                <option class="unsupported_calendar" value="IslamicTabular">Islamic Tabular</option>
                <option class="unsupported_calendar" value="IslamicObservational">Islamic Observational</option>
                <option class="unsupported_calendar" value="IslamicUmmAlQura">Islamic Umm AlQura</option>
             </optgroup>
             <optgroup label="Chinese">
                 <option class="supported_calendar" value="Chinese">Chinese</option>
                 <option class="unsupported_calendar" value="RepublicOfChina">Republic of China</option>
            </optgroup>
             <optgroup label="Japanese">
                 <option class="unsupported_calendar" value="Japanese">Japanese</option>
                 <option class="unsupported_calendar" value="JapaneseExtended">Japanese Extended</option>
            </optgroup>
            <optgroup label="Others">
                 <option class="supported_calendar" value="Gregorian">Gregorian</option>
                 <option class="supported_calendar" value="Iso">Iso</option>
                 <option class="supported_calendar" value="Indian">Indian</option>
                 <option class="unsupported_calendar" value="Buddhist">Buddhist</option>
                 <option class="unsupported_calendar" value="Coptic">Coptic</option>
                 <option class="unsupported_calendar" value="Dangi">Dangi</option>
                 <option class="unsupported_calendar" value="Ethiopian">Ethiopian</option>
                 <option class="unsupported_calendar" value="Hebrew">Hebrew</option>
                 <option class="unsupported_calendar" value="Julian">Julian</option>
                 <option class="unsupported_calendar" value="Persian">Persian</option>
            </optgroup>
        </select>
    }
}
