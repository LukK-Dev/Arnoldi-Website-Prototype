use http::status::StatusCode;
use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_meta::{provide_meta_context, Title};
use thiserror::Error;

use crate::components::{navbar::Navbar, background::Background};

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn Error(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }

    provide_meta_context();

    view! {
        <Title text="Arnoldi MVP | Error"/>

        <div class="flex flex-col h-screen">
            <Navbar/>
            <div class="text-white h-screen w-full flex justify-center items-center">
                <div class="flex flex-col items-center">
                    <div class="flex mb-4">
                        <Icon icon=i::BiErrorCircleRegular class="fill-current h-16 w-16 mr-6"/>
                        <h1 class="font-semibold text-6xl">{if errors.len() > 1 {"Errors"} else {"Error"}}</h1>
                    </div>
                    <For
                        // a function that returns the items we're iterating over; a signal is fine
                        each= move || {errors.clone().into_iter().enumerate()}
                        // a unique key for each item as a reference
                        key=|(index, _error)| *index
                        // renders each item to a view
                        children=move |error| {
                            let error_string = error.1.to_string();
                            let error_code= error.1.status_code();
                            view! {
                                <h2 class="text-2xl">{error_code.to_string()}</h2>
                                // <p class="mb-1">"Error: " {error_string}</p>
                            }
                        }
                    />
                </div>
            </div>
        </div>

        <Background />
    }
}
