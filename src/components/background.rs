use leptos::*;

#[component]
pub fn Background() -> impl IntoView{
    view!{
        <div class="fixed inset-0 z-[-1] overflow-hidden">
            <img src="/xArnoldi-2023.jpeg.pagespeed.ic.WVMxwBeclU.webp" class="object-center w-full h-full object-cover pointer-events-none brightness-50 blur-sm scale-105"/>
        </div>   
    }
}