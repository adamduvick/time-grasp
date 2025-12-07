//! TODO: document main workspaces here so that building the docs for the ui allows for easy
//! navigation to the rest of the app code
use frontend::app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
