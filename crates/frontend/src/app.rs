use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
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

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let group = C_Group {
                id: Uuid::new_v4(),
                name,
                note: None,
            };
            match create_group(group).await {
                Ok(id) => match list_group(CategoryGroupFilter { id: None }).await {
                    Ok(categories) => set_greet_msg.set(format!("{categories:#?}")),
                    Err(e) => set_greet_msg.set(format!("Error {e:?}")),
                },
                Err(e) => set_greet_msg.set(format!("Error {e:?}")),
            }
        });
    };

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"New Category"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>
            <StoreExample/>
        </main>
    }
}
