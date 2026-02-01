#[allow(unused)]
use leptos::prelude::*;

mod app;
mod components;
mod routes;

fn main() {
    leptos::mount::mount_to_body(|| view! { <app::App /> })
}
