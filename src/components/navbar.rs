use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header>
            <nav class="flex items-center justify-between flex-wrap p-6">
                <div class="flex items-center flex-shrink-0 text-white mr-6 group transition duration-300">
                    <Icon icon=i::TbSchool class="fill-current h-8 w-8 mr-2"/>
                    <A href="/" class="font-semibold text-xl tracking-tight">Arnoldi Website MVP</A>
                </div>
                <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
                    <div class="text-md lg:flex-grow">
                        <a href="/vertretungsplan" class="block mt-4 lg:inline-block lg:mt-0 text-white mr-8 group transition duration-300">
                            Vertretungsplan
                            <span class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-white rounded"></span>
                        </a>
                        <a href="/notenrechner" class="block mt-4 lg:inline-block lg:mt-0 text-white mr-8 group transition duration-300">
                            Notenrechner
                            <span class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-white rounded"></span>
                        </a>
                        <a href="/neues" class="block mt-4 lg:inline-block lg:mt-0 text-white mr-8 group transition duration-300">
                            Neues
                            <span class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-white rounded"></span>
                        </a>
                    </div>
                </div>
            </nav>
        </header>
    }
}
