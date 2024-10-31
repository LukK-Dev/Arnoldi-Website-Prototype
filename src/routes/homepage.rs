use leptos::*;
use leptos_meta::{provide_meta_context, Title};

use crate::components::{footer::Footer, navbar::Navbar, background::Background};

#[component]
pub fn HomePage() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Arnoldi MVP | Home"/>

        <div class="flex w-screen h-screen flex-col">
            <Navbar/>
            <div class="w-full h-full flex flex-col justify-center items-center">
                <h1 class="font-semibold lg:text-8xl text-6xl mb-2 text-white">Arnoldi Schule</h1>
                <h2 class="lg:text-5xl text-3xl text-white">Staatliches Gymnasium</h2>
            </div>
        </div>

        <Background />

        <main class="flex justify-center items-center">
            <div class="w-full h-screen bg-white" style="">
            </div>
        </main>

        <Footer/>
    }
}
