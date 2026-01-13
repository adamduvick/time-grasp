use std::{marker::PhantomData, sync::Arc};

use leptos::prelude::*;
use leptos::task::spawn_local;
use model as dto;
use reactive_graph::traits::{Read, Write};
use reactive_stores::{AtKeyed, Patch, PatchField, Store};

use crate::{error::Result, ipc, model::*};

// region:     --- helpers

#[derive(Store, Debug, Clone, Patch)]
pub struct KeyedVec<K, T: PatchField + Keyed<K>> {
    #[store(key: K = |e| e.key())]
    inner: Vec<T>,
    #[patch(|this, new| this)]
    _marker: PhantomData<K>,
}

impl<K, T> From<Vec<T>> for KeyedVec<K, T>
where
    T: PatchField + Keyed<K>,
{
    fn from(value: Vec<T>) -> Self {
        KeyedVec {
            inner: value,
            _marker: PhantomData::default(),
        }
    }
}

pub trait Keyed<K> {
    fn key(&self) -> K;
}

impl Keyed<Uuid> for Entry {
    fn key(&self) -> Uuid {
        self.id
    }
}

impl Keyed<Uuid> for Category {
    fn key(&self) -> Uuid {
        self.id
    }
}

impl Keyed<Uuid> for Group {
    fn key(&self) -> Uuid {
        self.id
    }
}

// endregion:  --- helpers

// region:     --- FMC - Frontend Model Controller

#[derive(Clone, Debug)]
pub struct Fmc {
    entries: Store<KeyedVec<Uuid, Entry>>,
    categories: Store<KeyedVec<Uuid, Category>>,
    groups: Store<KeyedVec<Uuid, Group>>,
}

impl Fmc {
    pub async fn init() -> Result<Arc<Self>> {
        let entries = ipc::list_entry(dto::EntryFilter::new()).await?;
        let entries: Vec<Entry> = entries.into_iter().map(|e| e.into()).collect();
        let entries = Store::new(entries.into());

        let categories = ipc::list_category(dto::CategoryFilter::new()).await?;
        let categories: Vec<Category> = categories.into_iter().map(|e| e.into()).collect();
        let categories = Store::new(categories.into());

        let groups = ipc::list_group(dto::CategoryGroupFilter::new()).await?;
        let groups: Vec<Group> = groups.into_iter().map(|e| e.into()).collect();
        let groups = Store::new(groups.into());

        Ok(Arc::new(Self {
            entries,
            categories,
            groups,
        }))
    }

    pub fn entries(&self) -> Store<KeyedVec<Uuid, Entry>> {
        self.entries
    }

    pub fn categories(&self) -> Store<KeyedVec<Uuid, Category>> {
        self.categories
    }

    pub fn groups(&self) -> Store<KeyedVec<Uuid, Group>> {
        self.groups
    }
}

// endregion:  --- FMC - Frontend Model Controller

// region:     --- CRUD operations

trait Crud<T> {
    /// Add item to the store and persist the creation to disk
    fn create(&self, item: T);
    /// Update the previous version of item in the store and persist the changes to disk
    ///
    /// Note: The key determines which existing item will be updated
    fn update(&self, item: T);
    /// Remove item from the store and persist the deletion to disk
    ///
    /// Note: Performs a soft-delete operation on disk
    fn delete(&self, item: T);
}

impl Crud<Entry> for Fmc {
    fn create(&self, item: Entry) {
        // Get the store
        let store = self.entries.inner();
        // Check that the store hasn't been dropped
        if let Some(mut store) = store.try_write() {
            // Add item to store optimistically
            store.push(item.clone());
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::create_entry(item.into()).await;
                leptos::logging::log!("ipc::create_entry result: {:?}", result);
                // TODO: implement rollback for failures
            });
        }
    }

    fn update(&self, item: Entry) {
        // Get the store
        let store = self.entries.inner();
        // Get a reference the keyed location in the store
        let field = AtKeyed::new(store, item.id);
        // Check that the store hasn't been dropped
        if let Some(prev) = field.try_get() {
            // Create a patch dto to send to backend
            let dto = prev.new_update_dto(&item);
            // Optimistically update the store
            field.patch(item);
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::update_entry(dto).await;
                leptos::logging::log!("ipc::update_entry result: {:?}", result);
            });
        }
    }

    fn delete(&self, item: Entry) {
        // Get the store
        let store = self.entries.inner();
        // Check that the store hasn't been dropped
        if let Some(mut store) = store.try_write() {
            // Optimistically update the store
            store.retain(|e| e.id != item.id);
            // Create a patch dto to send to backend
            let dto = item.into();
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::delete_entry(dto).await;
                leptos::logging::log!("ipc::update_entry result: {:?}", result);
                // TODO: implement rollback for failures
            });
        }
    }
}

impl Crud<Category> for Fmc {
    fn create(&self, item: Category) {
        // Get the store
        let store = self.categories.inner();
        // Check that the store hasn't been dropped
        if let Some(mut store) = store.try_write() {
            // Add item to store optimistically
            store.push(item.clone());
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::create_category(item.into()).await;
                leptos::logging::log!("ipc::create_category result: {:?}", result);
                // TODO: implement rollback for failures
            });
        }
    }

    fn update(&self, item: Category) {
        // Get the store
        let store = self.categories.inner();
        // Get a reference the keyed location in the store
        let field = AtKeyed::new(store, item.id);
        // Check that the store hasn't been dropped
        if let Some(prev) = field.try_get() {
            // Create a patch dto to send to backend
            let dto = prev.new_update_dto(&item);
            // Optimistically update the store
            field.patch(item);
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::update_category(dto).await;
                leptos::logging::log!("ipc::update_category result: {:?}", result);
            });
        }
    }

    fn delete(&self, item: Category) {
        // Get the store
        let store = self.categories.inner();
        // Check that the store hasn't been dropped
        if let Some(mut store) = store.try_write() {
            // Optimistically update the store
            store.retain(|e| e.id != item.id);
            // Create a patch dto to send to backend
            let dto = item.into();
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::delete_category(dto).await;
                leptos::logging::log!("ipc::delete_category result: {:?}", result);
                // TODO: implement rollback for failures
            });
        }
    }
}

impl Crud<Group> for Fmc {
    fn create(&self, item: Group) {
        // Get the store
        let store = self.groups.inner();
        // Check that the store hasn't been dropped
        if let Some(mut store) = store.try_write() {
            // Add item to store optimistically
            store.push(item.clone());
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::create_group(item.into()).await;
                leptos::logging::log!("ipc::create_group result: {:?}", result);
                // TODO: implement rollback for failures
            });
        }
    }

    fn update(&self, item: Group) {
        // Get the store
        let store = self.groups.inner();
        // Get a reference the keyed location in the store
        let field = AtKeyed::new(store, item.id);
        // Check that the store hasn't been dropped
        if let Some(prev) = field.try_get() {
            // Create a patch dto to send to backend
            let dto = prev.new_update_dto(&item);
            // Optimistically update the store
            field.patch(item);
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::update_group(dto).await;
                leptos::logging::log!("ipc::update_group result: {:?}", result);
            });
        }
    }

    fn delete(&self, item: Group) {
        // Get the store
        let store = self.groups.inner();
        // Check that the store hasn't been dropped
        if let Some(mut store) = store.try_write() {
            // Optimistically update the store
            store.retain(|e| e.id != item.id);
            // Create a patch dto to send to backend
            let dto = item.into();
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::delete_group(dto).await;
                leptos::logging::log!("ipc::delete_group result: {:?}", result);
                // TODO: implement rollback for failures
            });
        }
    }
}

// endregion:  --- CRUD operations

pub mod fmc_example {
    use std::sync::Arc;

    use leptos::prelude::*;
    use reactive_stores::{Field, Store};
    use web_sys::{HtmlInputElement, MouseEvent, SubmitEvent};

    use crate::{
        fmc::{Crud, Fmc, KeyedVecStoreFields},
        model::{Entry, EntryStoreFields},
    };

    #[component]
    pub fn EntryStoreTest() -> impl IntoView {
        let async_data = LocalResource::new(async move || Fmc::init().await);

        let async_result = move || match async_data.get() {
            None => view! {}.into_any(),
            Some(Err(e)) => view! { <ErrorView error=e /> }.into_any(),
            Some(Ok(v)) => {
                let fmc = v;
                provide_context(Arc::clone(&fmc));

                view! {
                    // <button on:click=move |_| VecEntryStore::add(&entry_store)>"new entry"</button>
                    <For
                        each=move || fmc.entries().inner()
                        key=|row| row.id().get()
                        let:entry
                    >
                        <EntryView entry=entry />
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
    fn EntryView(#[prop(into)] entry: Field<Entry>) -> impl IntoView {
        let note = RwSignal::new("new note".to_string());
        let input_ref = NodeRef::new();

        let fmc =
            use_context::<Arc<Fmc>>().expect("Fmc must be provided in higher-level component");

        let fmc1 = Arc::clone(&fmc);
        let fmc2 = Arc::clone(&fmc);

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
            fmc1.update(entry);
        };
        let on_delete = move |ev: MouseEvent| {
            ev.prevent_default();
            let entry = entry.get();
            fmc2.delete(entry);
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
}
