use crate::error::Result;
use crate::ipc;
use crate::model::{Entry, Uuid};
use leptos::prelude::*;
use leptos::task::spawn_local;
use model::{self as dto, D_Entry};
use reactive_graph::traits::{Read, Write};
use reactive_stores::StoreFieldIterator;
use reactive_stores::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use web_sys::{HtmlInputElement, MouseEvent, SubmitEvent};

// region:   --- prototype code

#[derive(Store)]
pub struct Entries {
    #[store(key: Uuid = |e| e.id)]
    items: Vec<Entry>,
}

pub struct EntryFmc {
    store: Store<Entries>,
    map: HashMap<Uuid, usize>,
    entries: Vec<Entry>,
}

impl EntryFmc {
    pub async fn init() -> Result<Self> {
        let entries = ipc::list_entry(dto::EntryFilter::new()).await?;
        let items = entries
            .clone()
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<Entry>>();
        let map = items.iter().enumerate().map(|(i, e)| (e.id, i)).collect();
        Ok(Self {
            store: Store::new(Entries {
                items: items.clone(),
            }),
            map,
            entries: items,
        })
    }

    fn find(&self, id: Uuid) -> Option<usize> {
        self.map.get(&id).cloned()
    }

    pub async fn create(&mut self, data: Entry) {
        // optimistically update the store
        // match self.items.writer() {
        //     Some(mut items) => {
        //         items.insert(data.id, data.clone());
        //     }
        //     None => todo!(),
        // }
        // if this fails, we need to roll back the store update
        self.entries.push(data.clone());
        ipc::create_entry(data.into()).await;
        // error variants:
        // - entry with same id already exists
        // - IPC failure
        todo!();
    }

    pub async fn update(&mut self, data: Entry) -> Result<Entry> {
        // get the previous entry from the store
        // create an update DTO
        // send the update via IPC
        // retrieve the updated entry via IPC
        // update the store
        // return the updated entry

        if let Some(index) = &self.find(data.id)
            && let Some(mut prev) = self.entries.get_mut(*index).cloned()
        {
            prev = data.clone();
            let update_dto = data.create_update_dto(&prev);
            ipc::update_entry(update_dto).await?;
        }

        if let Some(index) = &self.find(data.id)
            && let Some(mut prev) = self.store.items().write().get_mut(*index).cloned()
        {
            prev = data.clone();
            let update_dto = data.create_update_dto(&prev);
            ipc::update_entry(update_dto).await?;
        }

        Ok(data)
    }

    pub async fn delete(&mut self, id: Uuid) -> Result<()> {
        // send delete request via IPC
        // remove from store
        // return success

        if let Some(index) = &self.find(id) {
            self.entries.remove(*index);
            ipc::delete_entry(D_Entry {
                id,
                tombstone_reason: "deleted from frontend".to_string(),
            })
            .await?;
        }

        todo!();
    }

    pub fn store(&self) -> Store<Entries> {
        self.store.clone()
    }
}

#[derive(Store, Debug, Clone)]
struct EntryStore {
    #[store(key: Uuid = |(i, e)| *i)]
    entries: HashMap<Uuid, Entry>,
}

impl EntryStore {
    pub async fn init() -> Result<Self> {
        let entries = ipc::list_entry(dto::EntryFilter::new()).await?;
        let entries = entries.into_iter().map(|e| (e.id, e.into())).collect();
        Ok(Self { entries })
    }

    pub fn upsert(store: &Store<Self>, entry: Entry) {
        let prev = store.entries().write().insert(entry.id, entry);
        match prev {
            // invoke update_entry if value used to exist
            Some(prev) => todo!(),
            // otherwise, invoke create_entry
            None => todo!(),
        }
    }
}

#[derive(Store, Debug, Clone, Patch)]
struct VecEntryStore {
    #[store(key: Uuid = |e| e.id)]
    entries: Vec<Entry>,
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(4);

impl VecEntryStore {
    pub async fn init() -> Result<Self> {
        let entries = ipc::list_entry(dto::EntryFilter::new()).await?;
        let entries = entries.into_iter().map(|e| e.into()).collect();
        Ok(Self { entries })
    }

    pub fn add(store: &Store<Self>) {
        let entry = Entry::new(format!(
            "new entry {}",
            NEXT_ID.fetch_add(1, Ordering::Relaxed),
        ));
        store.entries().write().push(entry.clone());
        spawn_local(async move {
            let result = ipc::create_entry(entry.into()).await;
            leptos::logging::log!("ipc::create_entry result: {:?}", result);
        });
    }

    pub fn update(store: &Store<Self>, entry: Entry) {
        let original_entry = AtKeyed::new(store.entries(), entry.id);
        let update_dto = entry.create_update_dto(&original_entry.get());
        original_entry.patch(entry);
        spawn_local(async move {
            let result = ipc::update_entry(update_dto).await;
            leptos::logging::log!("ipc::update_entry result: {:?}", result);
        });
    }

    pub fn delete(store: &Store<Self>, entry: Entry) {
        store.entries().write().retain(|e| e.id != entry.id);
        let dto = entry.into();
        spawn_local(async move {
            let result = ipc::delete_entry(dto).await;
            leptos::logging::log!("ipc::update_entry result: {:?}", result);
        });
    }
}

#[component]
pub fn EntryStoreTest() -> impl IntoView {
    let async_data = LocalResource::new(async move || VecEntryStore::init().await);

    let async_result = move || match async_data.get() {
        None => view! {}.into_any(),
        Some(Err(e)) => view! { <ErrorView error=e /> }.into_any(),
        Some(Ok(v)) => {
            let entry_store = Store::new(v);
            use crate::model::entry::{Entry, EntryStoreFields};

            view! {
                <button on:click=move |_| VecEntryStore::add(&entry_store)>"new entry"</button>
                <For
                    each=move || entry_store.entries()
                    key=|row| row.id().get()
                    let:entry
                >
                    <EntryView store=entry_store entry=entry />
                </For>
            }
            .into_any()
        }
    };

    view! {
        // Catches panics *and* Resource errors thrown via `?`
        <ErrorBoundary
            fallback=|errs| view! {
                <div class="error">
                    <h3>"Something went wrong"</h3>
                    <pre>{move || format!("{:#?}", errs.get())}</pre>
                </div>
            }
        >
        <Suspense fallback=|| view! { <p>"Loading…"</p> }>
            {async_result}
        </Suspense>
        </ErrorBoundary>
    }
}

#[component]
fn ErrorView(error: crate::error::Error) -> impl IntoView {
    view! {
        <div class="error">
            <h3>"Failed to load entries"</h3>
            <pre>{format!("{error:?}")}</pre>
        </div>
    }
}

#[component]
fn EntryView(store: Store<VecEntryStore>, #[prop(into)] entry: Field<Entry>) -> impl IntoView {
    use crate::model::entry::EntryStoreFields;
    let note = RwSignal::new("new note".to_string());
    let input_ref = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        leptos::logging::log!("I see you!");
        let mut entry = entry.get();
        let mut input = input_ref.get().map(|html: HtmlInputElement| html.value());
        input = match input {
            Some(text) => match text.len() {
                0 => None,
                _ => Some(text),
            },
            None => None,
        };
        entry.note = input;
        VecEntryStore::update(&store, entry);
    };
    let on_delete = move |ev: MouseEvent| {
        ev.prevent_default();
        let entry = entry.get();
        VecEntryStore::delete(&store, entry);
    };

    view! {
        <form on:submit=on_submit>
            <label>
                "Change the note"
                <input type="text" node_ref=input_ref/>
            </label>
            <input type="submit"/>
        </form>
        <input type="button" name="delete" value="Delete" on:click=on_delete/>
        <p>
            {move || entry.name()}
            " - " {move || format!("{:?}", entry.start_time().get())}
            " - " {move || entry.note().get().unwrap_or("(no note)".to_string())}
        </p>
        <hr/>
    }
}

// endregion:   --- prototype code
