use crate::{
    routes::error::{AppError, Error},
    routes::{homepage::HomePage, grade_calculator::GradeCalculator},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/arnoldi-website-mvp.css"/>

        <Title text="Arnoldi MVP"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <Error outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/notenrechner" view=GradeCalculator/>
                </Routes>
            </main>
        </Router>
    }
}
