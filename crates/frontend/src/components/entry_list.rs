use itertools::Itertools;
use std::sync::Arc;

use leptos::{either::Either, prelude::*};
use leptos_router::{
    components::{
        A, Form, Outlet, ParentRoute, ProtectedRoute, Redirect, Route, Router, Routes,
        RoutingProgress,
    },
    hooks::{use_navigate, use_params, use_query_map},
    params::Params,
};
use leptos_router_macro::path;
use model::Uuid;
use reactive_stores::ArcField;
use time::{Date, macros::format_description};

use crate::{
    fmc::{Crud, Fmc, Keyed, KeyedVecStoreFields},
    model::{Entry, EntryStoreFields},
};

#[component]
pub fn EntryList() -> impl IntoView {
    let fmc = expect_context::<Arc<Fmc>>();

    let entries = {
        let fmc = fmc.clone();
        Memo::new(move |_| {
            let offset = fmc.get_offset().get();
            let grouped: Vec<(Date, Vec<Entry>)> = fmc
                .entries()
                .inner()
                .get()
                .into_iter()
                .sorted_by_key(|e| e.time_frame.get_start_datetime(offset))
                .chunk_by(|e| e.time_frame.get_start_date(offset))
                .into_iter()
                .map(|(date, group)| (date, group.collect()))
                .collect();
            grouped
        })
    };

    view! {
        <div class="entry-containers">
            <div class="entry-list">
                <h2>"Entries"</h2>
                <For
                    each=move || entries.get()
                    key=|row| format!("{:#?}", row.0)
                    let:((date, entries))
                >
                    <div>{format!("{date:#?}")}</div>
                    <For
                        each=move || entries.clone()
                        key=|row| row.key()
                        let:entry
                    >
                        <EntryItemNew entry=entry />
                    </For>
                </For>
            </div>
            <div class="entry-detail">
                <Outlet />
            </div>
        </div>
    }
}

#[component]
fn EntryItemNew(#[prop(into)] entry: Entry) -> impl IntoView {
    let id = {
        let entry = entry.clone();
        move || entry.id.to_string()
    };
    let name = {
        let entry = entry.clone();
        move || entry.name.to_string()
    };

    /// Date
    /// ----------------------------------------------
    /// Name                    Duration
    /// Category Name           Start Time - End Time

    view! {
        <A href={id}>
            <p>{name}</p>
        </A>
    }
}

#[component]
fn EntryItem(#[prop(into)] entry: ArcField<Entry>) -> impl IntoView {
    let id = entry.clone().id();
    let name = entry.clone().name();

    /// Date
    /// ----------------------------------------------
    /// Name                    Duration
    /// Category Name           Start Time - End Time

    view! {
        <A href={move || id.get().to_string()}>
            <p>{move || name.get()}</p>
        </A>
    }
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct EntryParams {
    id: Option<String>,
}

#[component]
pub fn EntryView() -> impl IntoView {
    let fmc = expect_context::<Arc<Fmc>>();
    let offset = fmc.get_offset();

    let params = use_params::<EntryParams>();

    let entry_field = move || {
        let id_str = params
            .try_get()
            .transpose()
            .ok()
            .unwrap_or_default()
            .map(|params| params.id.unwrap_or_default())
            .unwrap_or_default();
        let id = Uuid::parse_str(&id_str).unwrap_or_default();
        let field: ArcField<Entry> = fmc.get(id);
        field
    };

    let time_format = format_description!("[hour]:[minute] [period]");
    let date_format = format_description!("[month]/[day]/[year]");

    let details = move || match entry_field().try_get() {
        None => Either::Left(view! {<p>"Entry does not exist"</p>}),
        Some(entry) => Either::Right(view! {
            <h2>"Entry"</h2>
            <p>{
                match entry.time_frame.get_duration() {
                    Some(duration) => format!("{}", duration),
                    None => "-:--".into(),
                }
            }</p>
            <p>{entry.name}</p>
            <p>"Category!"</p>
            <p>{entry.time_frame.get_start_date(offset.get()).format(&date_format).unwrap()}</p>
            <p>{entry.time_frame.get_start_time(offset.get()).format(&time_format).unwrap()}</p>
            <p>{entry.note}</p>
        }),
    };

    view! {
        {details}
    }
}
