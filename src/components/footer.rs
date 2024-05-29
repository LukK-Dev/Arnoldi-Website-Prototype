use crate::components::navbar::NavA;
use icondata as i;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-screen h-36 flex flex-row flex-grow justify-content align-items gap-x-8 bg-[linear-gradient(180deg,rgba(0,0,0,0),rgba(0,0,0,0.5))] text-white text-lg">
            <div class="flex items-center pl-8">
                <Icon icon=i::LuCopyright class="stroke-white w-12 h-12"/>
                <span class="ml-6 text-lg">2024 | Luk Kaiser und Richard Arai | Alle Rechte vorbehalten.</span>
            </div>
            <div class="flex items-center justify-self-end">
                <NavA href="/impressum">Impressum</NavA>
            </div>
        </footer>
    }
}
