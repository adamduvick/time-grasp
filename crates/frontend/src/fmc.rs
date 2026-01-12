use std::marker::PhantomData;

use leptos::prelude::*;
use leptos::task::spawn_local;
use model as dto;
use reactive_graph::traits::{Read, Write};
use reactive_stores::{AtKeyed, Patch, PatchField, Store};

use crate::{error::Result, ipc, model::*};

// region:     --- helpers

#[derive(Store, Debug, Clone, Patch)]
struct KeyedVec<T: PatchField + Keyed<I>, I> {
    #[store(key: I = |e| e.id())]
    inner: Vec<T>,
    #[patch(|this, new| this)]
    _marker: PhantomData<I>,
}

impl<T, I> From<Vec<T>> for KeyedVec<T, I>
where
    T: PatchField + Keyed<I>,
{
    fn from(value: Vec<T>) -> Self {
        KeyedVec {
            inner: value,
            _marker: PhantomData::default(),
        }
    }
}

trait Keyed<I> {
    fn id(&self) -> I;
}

impl Keyed<Uuid> for Entry {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Keyed<Uuid> for Category {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Keyed<Uuid> for Group {
    fn id(&self) -> Uuid {
        self.id
    }
}

// endregion:  --- helpers

pub struct Fmc {
    entries: Store<KeyedVec<Entry, Uuid>>,
    categories: Store<KeyedVec<Category, Uuid>>,
    groups: Store<KeyedVec<Group, Uuid>>,
}

impl Fmc {
    pub async fn init() -> Result<Self> {
        let entries = ipc::list_entry(dto::EntryFilter::new()).await?;
        let entries: Vec<Entry> = entries.into_iter().map(|e| e.into()).collect();
        let entries = Store::new(entries.into());

        let categories = ipc::list_category(dto::CategoryFilter::new()).await?;
        let categories: Vec<Category> = categories.into_iter().map(|e| e.into()).collect();
        let categories = Store::new(categories.into());

        let groups = ipc::list_group(dto::CategoryGroupFilter::new()).await?;
        let groups: Vec<Group> = groups.into_iter().map(|e| e.into()).collect();
        let groups = Store::new(groups.into());

        Ok(Self {
            entries,
            categories,
            groups,
        })
    }
}

// region:     --- CRUD operations

trait Crud<T> {
    fn create(&self, item: T);
    fn update(&self, item: T);
    fn delete(&self, item: T);
}

impl Crud<Entry> for Fmc {
    fn create(&self, item: Entry) {
        let store = self.entries.inner();
        store.write().push(item.clone());
        spawn_local(async move {
            let result = ipc::create_entry(item.into()).await;
            leptos::logging::log!("ipc::create_entry result: {:?}", result);
        });
    }

    fn update(&self, item: Entry) {
        let store = self.entries.inner();
        let original_entry = AtKeyed::new(store, item.id);
        let update_dto = item.create_update_dto(&original_entry.get());
        original_entry.patch(item);
        spawn_local(async move {
            let result = ipc::update_entry(update_dto).await;
            leptos::logging::log!("ipc::update_entry result: {:?}", result);
        });
    }

    fn delete(&self, item: Entry) {
        let store = self.entries.inner();
        store.write().retain(|e| e.id != item.id);
        let dto = item.into();
        spawn_local(async move {
            let result = ipc::delete_entry(dto).await;
            leptos::logging::log!("ipc::update_entry result: {:?}", result);
        });
    }
}

impl Crud<Category> for Fmc {
    fn create(&self, item: Category) {
        todo!()
    }

    fn update(&self, item: Category) {
        todo!()
    }

    fn delete(&self, item: Category) {
        todo!()
    }
}

impl Crud<Group> for Fmc {
    fn create(&self, item: Group) {
        todo!()
    }

    fn update(&self, item: Group) {
        todo!()
    }

    fn delete(&self, item: Group) {
        todo!()
    }
}

// endregion:  --- CRUD operations
