use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::ipc::{create_and_read_back_group, create_group_by_name, list_all_groups};

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

            // match create_and_read_back_group(&name).await {
            //     Ok(result) => set_greet_msg.set(result),
            //     Err(e) => set_greet_msg.set(format!("Error {:?}", e)),
            // };
            let _ = create_group_by_name(&name).await;
            match list_all_groups().await {
                Ok(categories) => set_greet_msg.set(
                    categories
                        .into_iter()
                        .map(|c| format!("{:?}", c))
                        .collect::<Vec<_>>()
                        .join("\n"),
                ),
                Err(e) => set_greet_msg.set(format!("Error {:?}", e)),
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
        </main>
    }
}
