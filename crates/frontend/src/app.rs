use std::time::Duration;

use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use leptos_router::{
    components::{
        A, Form, Outlet, ParentRoute, ProtectedRoute, Redirect, Route, Router, Routes,
        RoutingProgress,
    },
    hooks::{use_navigate, use_params, use_query_map},
    params::Params,
};
use leptos_router_macro::path;
use model::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::ipc::*;

use reactive_stores::Store;

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: String = |row| row.key.clone())]
    rows: Vec<DatabaseEntry>,
    index: i64,
}

#[derive(Store, Debug, Clone)]
pub struct DatabaseEntry {
    key: String,
    value: i64,
}

#[component]
pub fn StoreExample() -> impl IntoView {
    let index = 3;
    let data = Store::new(Data {
        rows: (0..index)
            .map(|i| DatabaseEntry {
                key: format!("key-{i}"),
                value: i,
            })
            .collect(),
        index,
    });

    view! {
        // when we click, update each row,
        // doubling its value
        <button on:click=move |_| {
            // allows iterating over the entries in an iterable store field
            use reactive_stores::StoreFieldIterator;

            // calling rows() gives us access to the rows
            for row in data.rows().iter_unkeyed() {
                *row.value().write() *= 2;
            }
            // log the new value of the signal
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        // when we click, delete one row
        <button on:click=move |_| {
            let _ = data.rows().try_write().expect("rows signal exists").pop();
            let index = *data.index().read();
            data.index().set(index - 1);
            // log the new value of the signal
            leptos::logging::log!("{:?}", data.get());
        }>
            "Remove Row"
        </button>
        // when we click, delete one row
        <button on:click=move |_| {
            let index = *data.index().read();
            data.rows().try_write_untracked().expect("rows signal exists").push(DatabaseEntry {
                key: format!("key-{index}"),
                value: index,
            });
            data.index().set(index + 1);
            // log the new value of the signal
            leptos::logging::log!("{:?}", data.get());
        }>
            "Add Row"
        </button>
        // iterate over the rows and display each value
        <For
            each=move || data.rows()
            key=|row| row.read().key.clone()
            children=|child| {
                let value = child.value();
                view! { <p>{move || value.get()}</p> }
            }
        />
    }
}

/// /                       entries
/// /:id                    entry
/// /:id/category           entry category select
/// /:id/category/new       entry category create
#[component]
pub fn App() -> impl IntoView {
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <Router set_is_routing>
            <nav>
                // ordinary <a> elements can be used for client-side navigation
                // using <A> has two effects:
                // 1) ensuring that relative routing works properly for nested routes
                // 2) setting the `aria-current` attribute on the current link,
                // for a11y and styling purposes
                <A href="/">"Entries"</A>
                <A href="/about">"About"</A>
                <A href="/settings">"Settings"</A>
            </nav>
            <main class="container">
                <Routes transition=true fallback=|| "This page could not be found.">
                    // paths can be created using the path!() macro, or provided as types like
                    // StaticSegment("about")
                    <Route path=path!("about") view=About />
                    <Route path=path!("settings") view=Settings />
                    <Route path=path!("/") view=EntriesView />
                </Routes>
            </main>
            // shows a progress bar while async data are loading
            <div class="routing-progress">
                <RoutingProgress is_routing max_time=Duration::from_millis(250) />
            </div>
        </Router>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h2>"About"</h2>
        <p>"YNAB-inspired time budgetting app"</p>
    }
}

#[component]
fn Settings() -> impl IntoView {
    view! {
        <h2>"Settings"</h2>
        <p>"No settings yet!"</p>
        <A href="/no-exist">"broken internal link"</A>
    }
}

#[component]
fn EntriesView() -> impl IntoView {
    view! {
        <h2>"Entries view"</h2>
        <crate::fmc::fmc_example::EntryStoreTestNew />
    }
}
