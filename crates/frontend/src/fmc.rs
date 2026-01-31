use std::{marker::PhantomData, sync::Arc};

use leptos::task::spawn_local;
use leptos::{leptos_dom::logging, prelude::*};
use model as dto;
use reactive_graph::traits::{Read, Write};
use reactive_stores::{
    ArcField, ArcStore, AtKeyed, Field, Patch, PatchField, Store, StoreField, StoreFieldIter,
    StoreFieldIterator,
};
use time::UtcOffset;

use crate::model::timeframe::TimeFrame;
use crate::{
    error::Result,
    ipc,
    model::{support::get_timezone_offset, timeframe::TimeFrameStoreFields, *},
};

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
    offset: ArcRwSignal<UtcOffset>,
    entries: ArcStore<KeyedVec<Uuid, Entry>>,
    categories: ArcStore<KeyedVec<Uuid, Category>>,
    groups: ArcStore<KeyedVec<Uuid, Group>>,
}

impl Fmc {
    pub async fn init() -> Result<Arc<Self>> {
        let entries = ipc::list_entry(dto::EntryFilter::new()).await?;
        let entries: Vec<Entry> = entries
            .into_iter()
            .map(|e| e.try_into().expect("Backend data is trustworthy"))
            .collect();
        let entries = ArcStore::new(entries.into());

        let categories = ipc::list_category(dto::CategoryFilter::new()).await?;
        let categories: Vec<Category> = categories.into_iter().map(|e| e.into()).collect();
        let categories = ArcStore::new(categories.into());

        let groups = ipc::list_group(dto::CategoryGroupFilter::new()).await?;
        let groups: Vec<Group> = groups.into_iter().map(|e| e.into()).collect();
        let groups = ArcStore::new(groups.into());

        let mut fmc = Self {
            offset: ArcRwSignal::new(get_timezone_offset()),
            entries,
            categories,
            groups,
        };

        Ok(Arc::new(fmc))
    }

    pub fn entries(&self) -> ArcStore<KeyedVec<Uuid, Entry>> {
        self.entries.clone()
    }

    pub fn get_offset(&self) -> ArcRwSignal<UtcOffset> {
        // TODO return a memo here
        self.offset.clone()
    }

    pub fn set_offset(&self, offset: UtcOffset) {
        self.offset.update(|o| {
            if *o != offset {
                *o = offset;
            }
        });
    }

    #[deprecated]
    pub fn get_category(&self, id: Uuid) -> ArcField<Category> {
        AtKeyed::new(self.categories.clone().inner(), id).into()
    }

    pub fn find_category(&self, name: String) -> Option<ArcField<Category>> {
        let result = self
            .categories
            .clone()
            .inner()
            .into_iter()
            .find(|c| c.clone().name().get_untracked() == name);
        result.map(|c| c.into())
    }

    pub fn find_group(&self, name: String) -> Option<ArcField<Group>> {
        let result = self
            .groups
            .clone()
            .inner()
            .into_iter()
            .find(|g| g.clone().name().get_untracked() == name);
        result.map(|g| g.into())
    }
}

// endregion:  --- FMC - Frontend Model Controller

// region:     --- CRUD operations

pub trait Crud<T> {
    /// Add item to the store and persist the creation to disk
    fn create(&self, item: T);
    /// Get item by its id
    /// TODO should return Option<ArcField<T>> or maybe Option<T>
    fn get(&self, id: Uuid) -> ArcField<T>;
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
        let store = self.entries.clone().inner();
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

    fn get(&self, id: Uuid) -> ArcField<Entry> {
        AtKeyed::new(self.entries.clone().inner(), id).into()
    }

    fn update(&self, item: Entry) {
        let field: ArcField<Entry> = self.get(item.id);
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
        let store = self.entries.clone().inner();
        // Check that the store hasn't been dropped
        if let Some(mut store) = store.try_write() {
            // Optimistically update the store
            store.retain(|e| e.id != item.id);
            // Create a patch dto to send to backend
            let dto = item.into();
            // Send dto to backend to persist changes to disk
            spawn_local(async move {
                let result = ipc::delete_entry(dto).await;
                leptos::logging::log!("ipc::delete_entry result: {:?}", result);
                // TODO: implement rollback for failures
            });
        }
    }
}

impl Crud<Category> for Fmc {
    fn create(&self, item: Category) {
        // Get the store
        let store = self.categories.clone().inner();
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

    fn get(&self, id: Uuid) -> ArcField<Category> {
        AtKeyed::new(self.categories.clone().inner(), id).into()
    }

    fn update(&self, item: Category) {
        let field: ArcField<Category> = self.get(item.id);
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
        let store = self.categories.clone().inner();
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
        let store = self.groups.clone().inner();
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

    fn get(&self, id: Uuid) -> ArcField<Group> {
        AtKeyed::new(self.groups.clone().inner(), id).into()
    }

    fn update(&self, item: Group) {
        let field: ArcField<Group> = self.get(item.id);
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
        let store = self.groups.clone().inner();
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

    use leptos::{html, prelude::*};
    use model::category;
    use reactive_stores::{ArcField, Field, Store};
    use time::{Date, Time, UtcOffset, format_description, macros::format_description};
    use web_sys::{HtmlInputElement, MouseEvent, SubmitEvent};

    use crate::{
        fmc::{Crud, Fmc, KeyedVecStoreFields},
        model::{
            Category, CategoryStoreFields, Entry, EntryStoreFields, support::get_timezone_offset,
        },
    };

    #[component]
    pub fn EntryStoreTestNew() -> impl IntoView {
        let async_data = LocalResource::new(async move || Fmc::init().await);
        let async_result = move || match async_data.get() {
            None => view! {}.into_any(),
            Some(Err(e)) => view! { <ErrorView error=e /> }.into_any(),
            Some(Ok(fmc)) => {
                provide_context(Arc::clone(&fmc));
                view! {
                    <For
                        each=move || fmc.entries.clone().inner()
                        key=|row| row.clone().id().get()
                        let:entry
                    >
                        <EntryEditView entry=entry />
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
    fn EntryEditView(#[prop(into)] entry: ArcField<Entry>) -> impl IntoView {
        let fmc = use_context::<Arc<Fmc>>().expect("expected Fmc to be provided");
        let offset = fmc.get_offset();
        let category: ArcField<Category> = fmc.get(entry.clone().category_id().get_untracked());

        let date_format =
            format_description::parse("[year]-[month]-[day]").expect("validated parser");
        let time_format = format_description::parse("[hour]:[minute]").expect("validated parser");

        let name = RwSignal::new(entry.clone().name().get_untracked());
        let note = RwSignal::new(entry.clone().note().get_untracked().unwrap_or_default());
        let category_name = RwSignal::new(category.clone().name().get_untracked());
        let start_datetime = entry
            .clone()
            .time_frame()
            .get_untracked()
            .get_start_datetime(offset.get());
        let start_date = RwSignal::new(start_datetime.date().format(&date_format).unwrap());
        let start_time = RwSignal::new(start_datetime.time().format(&time_format).unwrap());

        let on_submit = {
            let entry = entry.clone();
            let fmc = Arc::clone(&fmc);
            move |ev: SubmitEvent| {
                ev.prevent_default();
                let mut entry = entry.get_untracked().clone();
                entry.name = name.get();
                entry.note = Some(note.get());
                match fmc.find_category(category_name.get()) {
                    Some(category) => entry.category_id = category.id().get_untracked(),
                    None => leptos::logging::log!("category does not exist"),
                }
                leptos::logging::log!("start_time: {:?}", start_time.get_untracked());
                leptos::logging::log!("start_date: {:?}", start_date.get_untracked());
                leptos::logging::log!("Saved Entry");
                fmc.update(entry);
                // entry.time_frame
            }
        };

        view! {
            <form on:submit=on_submit>
                <label>
                    "Name: "
                    <input type="text" bind:value=name />
                </label>
                <label>
                    "Note: "
                    <input type="text" bind:value=note />
                </label>
                <CategoryDropDown category_name=category_name/>
                <label>
                    "Start Date: "
                    <input type="date" bind:value=start_date />
                </label>
                <label>
                    "Start Time: "
                    <input type="time" bind:value=start_time />
                </label>
                <input type="submit" value="Save" />
            </form>
            <p>{move || format!("{:#?}", entry.get())}</p>
        }
    }

    // #[component]
    // fn StartTimeInput(#[prop(into)] entry: ArcField<Entry>) -> impl IntoView {
    //     let fmc = use_context::<Arc<Fmc>>().expect("Fmc provided");
    //     let offset = fmc.get_offset();
    //     let time_format = format_description!("[hour]:[minute]");
    //     let timeframe = entry.clone().time_frame().get_untracked();
    //     let input_signal = RwSignal::new(
    //         timeframe
    //             .get_start_time(offset.get_untracked())
    //             .format(&time_format)
    //             .expect("start time should parse"),
    //     );
    //     let output_signal = RwSignal::new(Option::<Time>::None);
    //     let validator = {
    //         let entry = entry.clone();
    //         move |start_time: &Time| {
    //             if let Some(end_time) = entry.clone().time_frame().get().get_end_time(offset.get())
    //             {
    //                 start_time <= &end_time
    //             } else {
    //                 true
    //             }
    //         }
    //     };
    //     let parser = {
    //         let time_format = format_description!("[hour]:[minute]");
    //         move |input_str: &String| Time::parse(input_str, &time_format).ok()
    //     };

    //     view! {
    //         <ValidatedInputParsed
    //             input_type="time"
    //             input_signal=input_signal
    //             output_signal=output_signal
    //             validator=validator
    //             validation_error_message="Start time must be earlier than end time"
    //             parser=parser
    //             parse_error_message="Invalid input"
    //         />
    //     }
    // }

    // #[component]
    // fn ValidatedInput<V>(
    //     #[prop(into)] input_type: String,
    //     input_signal: RwSignal<String>,
    //     validator: V,
    //     #[prop(into)] validation_error_message: String,
    // ) -> impl IntoView
    // where
    //     V: Fn(&String) -> bool + 'static,
    // {
    //     let input_ref = NodeRef::<html::Input>::new();

    //     let handle_input = move |ev| {
    //         let value_str = event_target_value(&ev);
    //         input_signal.set(value_str.clone());

    //         if let Some(input) = input_ref.get() {
    //             // Now validate the parsed value
    //             if validator(&value_str) {
    //                 input.set_custom_validity("");
    //             } else {
    //                 input.set_custom_validity(&validation_error_message);
    //             }
    //         }
    //     };

    //     view! {
    //         <input
    //             node_ref=input_ref
    //             type=input_type
    //             required
    //             on:input=handle_input
    //             prop:value=move || input_signal.get().to_string()
    //         />
    //     }
    // }

    // #[component]
    // fn ValidatedInputParsed<V, P, T>(
    //     #[prop(into)] input_type: String,
    //     input_signal: RwSignal<String>,
    //     output_signal: RwSignal<Option<T>>,
    //     validator: V,
    //     #[prop(into)] validation_error_message: String,
    //     parser: P,
    //     #[prop(into)] parse_error_message: String,
    // ) -> impl IntoView
    // where
    //     V: Fn(&T) -> bool + 'static,
    //     P: Fn(&String) -> Option<T> + 'static,
    //     T: Clone + Send + Sync + 'static,
    // {
    //     let input_ref = NodeRef::<html::Input>::new();

    //     let handle_input = move |ev| {
    //         let value_str = event_target_value(&ev);
    //         input_signal.set(value_str.clone());

    //         if let Some(input) = input_ref.get() {
    //             // Try to parse the string into T
    //             match parser(&value_str) {
    //                 Some(parsed_value) => {
    //                     output_signal.set(Some(parsed_value.clone()));

    //                     // Now validate the parsed value
    //                     if validator(&parsed_value) {
    //                         input.set_custom_validity("");
    //                     } else {
    //                         input.set_custom_validity(&validation_error_message);
    //                     }
    //                 }
    //                 None => {
    //                     // If parsing fails, mark as invalid
    //                     input.set_custom_validity(&parse_error_message);
    //                 }
    //             }
    //         }
    //     };

    //     view! {
    //         <input
    //             node_ref=input_ref
    //             type=input_type
    //             required
    //             on:input=handle_input
    //             prop:value=move || input_signal.get().to_string()
    //         />
    //     }
    // }

    #[component]
    pub fn EntryStoreTest() -> impl IntoView {
        let async_data = LocalResource::new(async move || Fmc::init().await);

        let async_result = move || match async_data.get() {
            None => view! {}.into_any(),
            Some(Err(e)) => view! { <ErrorView error=e /> }.into_any(),
            Some(Ok(fmc)) => {
                provide_context(Arc::clone(&fmc));

                let fmc_clone = Arc::clone(&fmc);
                let new_entry =
                    move |_| fmc_clone.create(Entry::new("new @ current time".to_string()));

                let fmc_clone = Arc::clone(&fmc);
                let offset = fmc_clone.get_offset();
                let toggle_local = move |_| {
                    offset.update(|val| {
                        if *val == UtcOffset::UTC {
                            *val = get_timezone_offset();
                        } else {
                            *val = UtcOffset::UTC;
                        }
                    })
                };

                let fmc_clone = Arc::clone(&fmc);

                view! {
                    <button on:click=new_entry>"new entry"</button>
                    <button on:click=toggle_local>"toggle UTC time"</button>
                    <div>
                        <p>"Current offset: " {crate::model::support::get_timezone_offset().to_string()}</p>
                    </div>
                    // <CategoryDropDown />
                    <For
                        each=move || fmc_clone.entries.clone().inner()
                        key=|row| row.clone().id().get()
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
    fn EntryView(#[prop(into)] entry: ArcField<Entry>) -> impl IntoView {
        let note = RwSignal::new("new note".to_string());
        let input_ref = NodeRef::new();

        let fmc =
            use_context::<Arc<Fmc>>().expect("Fmc must be provided in higher-level component");

        let fmc1 = Arc::clone(&fmc);
        let fmc2 = Arc::clone(&fmc);

        let entry_clone = entry.clone();
        let on_submit = move |ev: SubmitEvent| {
            ev.prevent_default();
            leptos::logging::log!("I see you!");
            let mut entry = entry_clone.get();
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

        let entry_clone = entry.clone();
        let on_delete = move |ev: MouseEvent| {
            ev.prevent_default();
            let entry = entry_clone.get();
            fmc2.delete(entry);
        };

        let entry1 = entry.clone();
        let entry2 = entry.clone();

        view! {
            <form on:submit=on_submit>
                <label>
                    "Change the note: "
                    <input type="text" node_ref=input_ref/>
                </label>
                <input type="submit"/>
            </form>
            <input type="button" name="delete" value="Delete" on:click=on_delete/>
            <p>{
                let name = entry.clone().name();
                move || name.get()
            }</p>
            <p>{
                let category_id = entry.clone().category_id();
                let clone_fmc = Arc::clone(&fmc);
                // move || clone_fmc.get_category(category_id.get()).name().get()
                move || Crud::<Category>::get(&*clone_fmc, category_id.get()).name().get()
            }</p>
            <p>{
                let note = entry.note().clone();
                move || note.get().unwrap_or("(no note)".to_string())
            }</p>
            <p>{
                let time_frame = entry1.clone().time_frame();
                let offset = fmc.get_offset();
                move || {
                    let time_frame = time_frame.get();
                    format!("{:?} - {:?} - {:?}",
                        time_frame.get_start_datetime(offset.get()),
                        time_frame.get_end_datetime(offset.get()),
                        time_frame.get_duration()
                    )
                }
            }</p>
            <hr/>
        }
    }

    #[component]
    pub fn CategoryDropDown(category_name: RwSignal<String>) -> impl IntoView {
        let fmc =
            use_context::<Arc<Fmc>>().expect("Fmc must be provided in higher-level component");

        view! {
            <label for="category-choice">"Choose a Category:"</label>
            <input list="categories" id="category-choice" name="category-choice" bind:value=category_name />

            <datalist id="categories">
                <For
                    each=move || fmc.categories.clone().inner()
                    key=|row| row.clone().id().get()
                    let:category
                >
                    <option value={
                        let name = category.clone().name();
                        move || name.get()
                    }></option>
                </For>
            </datalist>
        }
    }

    #[component]
    pub fn CategoryDropDownOption(#[prop(into)] category: ArcField<Category>) -> impl IntoView {
        view! {
            <option value={
                let name = category.clone().name();
                move || name.get()
            }></option>
        }
    }
}
