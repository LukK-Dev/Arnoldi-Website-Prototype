use leptos::*;
use leptos_meta::{provide_meta_context, Title};

use crate::components::navbar::Navbar;

#[component]
pub fn GradeCalculator() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Arnoldi MVP | Home"/>

        <div class="flex flex-col h-screen">
            <Navbar/>
            <div class="text-white h-screen w-full flex flex-col justify-center items-center">
                <h1 class="font-semibold lg:text-8xl text-6xl mb-4">Notenrechner</h1>
                <h2>Zu erreichende Punktzahl:</h2>
                <ControlledComponent/>
            </div>
        </div>

        <div class="fixed inset-0 z-[-1] overflow-hidden">
            <img src="/school.jpg" class="object-center w-full h-full object-cover pointer-events-none brightness-50 blur-sm scale-105"/>
        </div>
    }
}

#[component]
fn ControlledComponent() -> impl IntoView {
    // create a signal to hold the value
    let (name, set_name) = create_signal("0".to_string());

    view! {
        <input type="number" class="text-slate-600" min="1" max="125" step="1"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }

            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}
