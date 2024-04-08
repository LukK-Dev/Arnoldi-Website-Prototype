use leptos::*;
use leptos_meta::{provide_meta_context, Title};

use crate::components::navbar::Navbar;

#[component]
pub fn HomePage() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Arnoldi MVP | Home"/>

        <div class="flex flex-col h-screen">
            <Navbar/>
            <div class="text-white h-screen w-full flex flex-col justify-center items-center">
                <h1 class="font-semibold lg:text-8xl text-6xl mb-2">Arnoldi Schule</h1>
                <span class="lg:text-5xl text-3xl">Staatliches Gymnasium</span>
            </div>
        </div>

        <div class="fixed inset-0 z-[-1] overflow-hidden">
            <img src="/school.jpg" class="object-center w-full h-full object-cover pointer-events-none brightness-50 blur-sm scale-105"/>
        </div>
    }
}
