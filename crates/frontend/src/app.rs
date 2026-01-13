use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use model::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::entry_table::EntryTable;
use crate::grid::EntriesGridDemo;
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

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            // <StoreExample />
            // <EntriesGridDemo />
            // <crate::store::EntryStoreTest />
            <crate::fmc::fmc_example::EntryStoreTest />
        </main>
    }
}
