use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed w-full flex items-center justify-between flex-wrap p-6 bg-[linear-gradient(180deg,rgba(0,0,0,0.5),rgba(0,0,0,0))]">
            <div class="flex items-center flex-shrink-0 text-white mr-6 group transition duration-300">
                <Icon icon=i::TbSchool class="fill-current h-8 w-8 mr-2"/>
                <A href="/" class="font-semibold text-xl tracking-tight">Arnoldi Website MVP</A>
            </div>
            <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto justify-end">
                <div class="text-md">
                    <NavA href="/vertretungsplan" content="Vertretungsplan"/>
                    <NavA href="/notenrechner" content="Notenrechner"/>
                    <NavA href="/neues" content="Neues"/>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn NavA(href: &'static str, content: &'static str) -> impl IntoView {
    view! {
        <A href=href class="block mt-4 lg:inline-block lg:mt-0 text-white mr-8 group transition duration-300">
            {content}
            <span class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-white rounded"></span>
        </A>
    }
}

fn DropdownNavA(hrefs: &[(&'static str, &'static str)], content: &'static str) -> impl IntoView {}
