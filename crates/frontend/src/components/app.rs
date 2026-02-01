use std::time::Duration;

use leptos::prelude::*;
use leptos_router::{
    components::{
        A, Form, Outlet, ParentRoute, ProtectedRoute, Redirect, Route, Router, Routes,
        RoutingProgress,
    },
    hooks::{use_navigate, use_params, use_query_map},
    params::Params,
};
use leptos_router_macro::path;

use crate::components::{
    entry_list::{EntryList, EntryView},
    fmc_provider::FmcProvider,
    radix_examples::RadixRoutes,
};

#[component]
pub fn App() -> impl IntoView {
    let (is_routing, set_is_routing) = signal(false);

    view! {
        <FmcProvider>
            <Router set_is_routing>
                <nav>
                    <A href="/">"Entries"</A>
                    <A href="/about">"About"</A>
                    <A href="/settings">"Settings"</A>
                    <A href="/radix">"Radix"</A>
                </nav>
                <main class="container">
                    <Routes transition=true fallback=|| "This page could not be found.">
                        <Route path=path!("about") view=About />
                        <Route path=path!("settings") view=Settings />
                        <RadixRoutes />
                        <ParentRoute path=path!("") view=EntryList>
                            <Route path=path!("/") view=|| "Select a contact." />
                            <Route path=path!("/:id") view=EntryView />
                        </ParentRoute>
                    </Routes>
                </main>
                <div class="routing-progress">
                    <RoutingProgress is_routing max_time=Duration::from_millis(250) />
                </div>
            </Router>
        </FmcProvider>
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
        <p>"No settings yet!"</p>
    }
}
