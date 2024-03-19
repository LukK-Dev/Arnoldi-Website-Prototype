use leptos::*;

use crate::components::navbar::Navbar;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Navbar/>
        <div class="fixed inset-0 z-[-1] overflow-hidden">
            <img src="/school.jpg" class="object-center w-full h-full object-cover pointer-events-none brightness-50 blur-sm scale-105"/>
        </div>
    }
}
