use leptos::prelude::*;

use crate::fmc::Fmc;

#[component]
pub fn FmcProvider(children: ChildrenFn) -> impl IntoView {
    let fmc_resource = LocalResource::new(async move || Fmc::init().await);
    let children = StoredValue::new(children);
    let loaded_fmc = {
        move || {
            let maybe_fmc = fmc_resource
                .try_get()
                .flatten()
                .ok_or_else(|| crate::error::Error::FmcProvider)
                .flatten();

            match maybe_fmc {
                Ok(fmc) => {
                    provide_context(fmc);
                    view! { {children.read_value()()} }.into_any()
                }
                Err(error) => view! { <FmcErrorFallback error=error /> }.into_any(),
            }
        }
    };

    view! {
        <Suspense fallback=move || view! { <FmcLoadingFallback /> }>
            {loaded_fmc}
        </Suspense>
    }
}

#[component]
fn FmcLoadingFallback() -> impl IntoView {
    view! {
        <div class="fmc-loading">
            <div class="spinner"></div>
            <p>"Loading application..."</p>
        </div>
    }
}

#[component]
fn FmcErrorFallback(error: crate::error::Error) -> impl IntoView {
    let retry = move |_| {
        // Trigger a page reload to retry initialization
        window().location().reload().expect("failed to reload");
    };

    view! {
        <div class="fmc-error">
            <h1>"Failed to Initialize Application"</h1>
            <p>"An error occurred while loading the application:"</p>
            <pre>{format!("{error:#?}")}</pre>
            <button on:click=retry>"Retry"</button>
        </div>
    }
}
